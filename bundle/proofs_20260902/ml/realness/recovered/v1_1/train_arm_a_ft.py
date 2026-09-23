#!/usr/bin/env python3
"""ARM-A fine-tune on the Lambda box (single GPU or DDP).

Warm-starts the published Phase G verifier (model_final.pt, sha pinned) and fine-tunes on
D2 + V10 (train rows outside the eval blocks with 60-row guards, half-open) + ALL 712
August development rows (owner ruling 2026-08-23). Phase G locked preprocessing
throughout. bf16 autocast, MSE on eps, AdamW, cosine with warmup, grad accumulation.
Atomic resumable checkpoints (model/optim/step/per-rank RNG) to the ZeeBeam filesystem; periodic
matched-vs-crossed eval at t=150 on held-out D2/V10 rows and an August diagnostic.
Single file, no deps beyond torch/numpy/PIL and the copied phase_g model source.
"""
from __future__ import annotations
import argparse, hashlib, json, math, os, random, sys, time
import torch.distributed as dist
from pathlib import Path
import numpy as np
import torch
import torch.nn.functional as F
from PIL import Image

sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent))  # v1.3: frozen-tree-relative
from phase_g.diffusion_diagnostic_model import DiffusionDiagnosticUNet, build_diffusion_constants, q_sample, ResBlock, Attn  # noqa: E402

ROOT = Path("/lambda/nfs/ZeeBeam")
CKPT_SHA = "b9d93050bfeb1a5cdbf620c38210f3ab6c6fd7af1f61eb45cc17ac269341f055"
CROP = (0, 1704, 155, 2433)
TH, TW = 768, 1024

SESSIONS = {
    "d2": {"dir": ROOT / "truthbeam/sessions/d2", "rows_total": 5992, "raw": "frames/frame_{r:06d}.raw",
           "em": "derived/Emissions/tile_{r:06d}.png",
           "train": [(0, 1238), (1758, 2736), (3256, 4234), (4754, 5992)],
           "eval_blocks": [(1298, 1698), (2796, 3196), (4294, 4694)]},
    "v10": {"dir": ROOT / "truthbeam/sessions/v10", "rows_total": 3743, "raw": "frames/frame_{r:06d}.raw",
            "em": "derived/Emissions/tile_{r:06d}.png",
            "train": [(0, 1050), (1420, 2285), (2655, 3743)],
            "eval_blocks": [(1110, 1360), (2345, 2595)]},
    "august": {"dir": ROOT / "august_dev_712", "rows_total": 712, "raw": "Recordings/frame_{r:06d}.raw",
               "em": "derived/Emissions/tile_{r:06d}.png",
               "train": [(0, 712)], "eval_blocks": []},
}


def sha256(p: Path) -> str:
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for c in iter(lambda: f.read(1 << 22), b""):
            h.update(c)
    return h.hexdigest()


def resolve_raw_layout():
    """The public sessions may store raw frames under a different subdir; detect once."""
    for sid, s in SESSIONS.items():
        d = s["dir"]
        for cand in (s["raw"], "frames/frame_{r:06d}.raw", "Recordings/frame_{r:06d}.raw", "raw/frame_{r:06d}.raw", "frame_{r:06d}.raw"):
            if (d / cand.format(r=0)).is_file():
                s["raw"] = cand
                break
        else:
            raise SystemExit(f"FAIL: no raw layout found for {sid} under {d}")
        if not (d / s["em"].format(r=0)).is_file():
            raise SystemExit(f"FAIL: no emission layout for {sid}")


def load_C(sid, r):
    s = SESSIONS[sid]
    raw = np.frombuffer((s["dir"] / s["raw"].format(r=r)).read_bytes(), dtype=np.uint8)
    if raw.size != 5320 * 4600:
        raise ValueError(f"raw size {sid} {r}")
    x = raw.reshape(4600, 5320)
    cfa = np.stack([x[0::2, 0::2], x[0::2, 1::2], x[1::2, 0::2], x[1::2, 1::2]], 0)
    t = torch.from_numpy(cfa.astype(np.float32) / 255.0)[:, CROP[0]:CROP[1], CROP[2]:CROP[3]]
    return F.interpolate(t.unsqueeze(0), size=(TH, TW), mode="area").squeeze(0)


def load_E(sid, r):
    s = SESSIONS[sid]
    im = Image.open(s["dir"] / s["em"].format(r=r)).convert("RGB")
    if im.size != (1920, 1080):
        raise ValueError(f"emission size {sid} {r}")
    t = torch.from_numpy(np.asarray(im).astype(np.float32) / 255.0).permute(2, 0, 1).contiguous()
    return F.interpolate(t.unsqueeze(0), size=(TH, TW), mode="area").squeeze(0)


def train_row_list():
    rows = []
    for sid, s in SESSIONS.items():
        for a, b in s["train"]:
            rows += [(sid, r) for r in range(a, b)]
    return rows


def eval_rows():
    out = []
    for sid in [s for s in ("d2", "v10") if s in SESSIONS]:
        for a, b in SESSIONS[sid]["eval_blocks"]:
            out += [(sid, r) for r in range(a + 30, b - 30, 40)]
    if "august" in SESSIONS:
        out += [("august", r) for r in range(30, 682, 24)]
    return out


@torch.no_grad()
def quick_eval(model, dc, dev, rows, t_val=150, offset=2):
    model.eval()
    res = {}
    for sid in list(SESSIONS):
        cs, ws = [], []
        for s, r in rows:
            if s != sid:
                continue
            C = load_C(s, r).to(dev, torch.bfloat16)
            # Do not perturb the training RNG: a checkpoint immediately before an
            # eval must resume to the same subsequent training stream.
            gen = torch.Generator(device=dev)
            gen.manual_seed(999 + r)
            noise = torch.randn(1, 4, TH, TW, device=dev, generator=gen)
            tt = torch.full((1,), t_val, device=dev, dtype=torch.long)
            Ct = q_sample(C.float().unsqueeze(0), tt, dc, noise).to(torch.bfloat16)
            with torch.autocast("cuda", dtype=torch.bfloat16):
                e_c = model(Ct, load_E(s, r).to(dev, torch.bfloat16).unsqueeze(0), tt)
                e_w = model(Ct, load_E(s, min(r + offset, SESSIONS[s]["rows_total"] - 1)).to(dev, torch.bfloat16).unsqueeze(0), tt)
            cs.append(float((e_c.float() - noise).pow(2).mean()))
            ws.append(float((e_w.float() - noise).pow(2).mean()))
        c, w = np.array(cs), np.array(ws)
        res[sid] = {"n": len(cs), "correct": float(c.mean()), "wrong": float(w.mean()),
                    "delta": float((w - c).mean()), "paired": float((c < w).mean())}
    model.train()
    return res


def capture_rng_state(dev):
    return {
        "torch": torch.get_rng_state(),
        "cuda": torch.cuda.get_rng_state(dev),
        "numpy": np.random.get_state(),
        "python": random.getstate(),
    }


def save_ckpt(path: Path, model, opt, step, rng_by_rank, args_d):
    tmp = path.with_name(path.name + ".tmp")
    torch.save({"model": model.state_dict(), "optimizer": opt.state_dict(), "step": step,
                "rng_by_rank": rng_by_rank, "rng_world_size": len(rng_by_rank),
                "sampler": "splitmix64_rank_slot_v1", "args": args_d}, tmp)
    os.replace(tmp, path)


def splitmix64(x):
    """Stable integer mixer used to map (seed, rank, sample slot) to a row."""
    mask = (1 << 64) - 1
    x = (x + 0x9E3779B97F4A7C15) & mask
    x = ((x ^ (x >> 30)) * 0xBF58476D1CE4E5B9) & mask
    x = ((x ^ (x >> 27)) * 0x94D049BB133111EB) & mask
    return x ^ (x >> 31)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=str(ROOT / "runs/arm_a_ft_20260823"))
    ap.add_argument("--ckpt", default=str(ROOT / "truthbeam/models/verifier/model_final.pt"))
    ap.add_argument("--max-steps", type=int, default=200000)  # run until stopped
    ap.add_argument("--lr", type=float, default=3e-5)
    ap.add_argument("--warmup", type=int, default=200)
    ap.add_argument("--cosine-horizon", type=int, default=40000)
    ap.add_argument("--accum", type=int, default=8)
    ap.add_argument("--ckpt-every", type=int, default=1000)
    ap.add_argument("--eval-every", type=int, default=2000)
    ap.add_argument("--seed", type=int, default=20260823)
    ap.add_argument("--sessions", default="d2,v10,august")
    ap.add_argument("--from-scratch", action="store_true", help="random init instead of warm start (architecture from the published ckpt args)")
    ap.add_argument("--act-ckpt", action="store_true", help="activation checkpointing (needed on 24GB GPUs)")
    ap.add_argument("--local-root", default="", help="if set, remap session dirs onto this root (same layout: truthbeam/sessions/*, august_dev_712)")
    ap.add_argument("--prefetch", type=int, default=6, help="prefetch worker threads per rank")
    ap.add_argument("--bs", type=int, default=1, help="micro-batch per rank per accum step; effective batch = ranks*accum*bs")
    ap.add_argument("--shuffle-conditions", action="store_true", help="CONTROL: pair each C with a wrong-but-real E via a fixed seeded derangement over training rows; eval stays unshuffled")
    ap.add_argument("--cond-seed", type=int, default=-1, help="REQUIRED with --shuffle-conditions: independent declared seed for the derangement (must not be derived from --seed; halo review point 1)")
    ap.add_argument("--student-base-ch", type=int, default=0, help="PROVABILITY LINE: override base_ch (requires --from-scratch)")
    ap.add_argument("--student-mults", default="", help="PROVABILITY LINE: override channel mults, e.g. 1,2,4 (requires --from-scratch)")
    a = ap.parse_args()
    ddp = "RANK" in os.environ
    rank = int(os.environ.get("RANK", 0)); world = int(os.environ.get("WORLD_SIZE", 1))
    if ddp:
        dist.init_process_group("nccl")
        torch.cuda.set_device(int(os.environ["LOCAL_RANK"]))
    is_main = rank == 0
    out = Path(a.out)
    if is_main:
        out.mkdir(parents=True, exist_ok=True)
    dev = torch.device("cuda")
    for sid in [k for k in list(SESSIONS) if k not in a.sessions.split(",")]:
        del SESSIONS[sid]
    if a.local_root:
        LR = Path(a.local_root)
        remap = {"d2": LR / "sessions/d2", "v10": LR / "sessions/v10", "august": LR / "august_dev_712"}
        for sid in SESSIONS:
            if remap[sid].is_dir():
                SESSIONS[sid]["dir"] = remap[sid]
        if is_main:
            print("data roots:", {k: str(v["dir"]) for k, v in SESSIONS.items()}, flush=True)
    resolve_raw_layout()
    rows = train_row_list()
    if not is_main:
        pass
    print(f"[rank {rank}] train rows: {len(rows)} "
          f"(d2 {sum(1 for s,_ in rows if s=='d2')}, v10 {sum(1 for s,_ in rows if s=='v10')}, august {sum(1 for s,_ in rows if s=='august')})", flush=True)
    ev = eval_rows()

    if sha256(Path(a.ckpt)) != CKPT_SHA:
        raise SystemExit("FAIL: base checkpoint hash")
    ck = torch.load(a.ckpt, map_location="cpu", weights_only=True)
    margs = ck["args"]; mults = tuple(margs.get("mults", (1, 2, 4, 4)))
    base_ch = margs.get("base_ch", 96)
    if a.student_base_ch > 0 or a.student_mults:
        if not a.from_scratch:
            raise SystemExit("FAIL: student architecture requires --from-scratch")
        if a.student_base_ch > 0:
            base_ch = int(a.student_base_ch)
        if a.student_mults:
            mults = tuple(int(x) for x in a.student_mults.split(","))
        if is_main:
            print(f"STUDENT ARCH: base_ch={base_ch} mults={mults} (provability feasibility line)", flush=True)
    model = DiffusionDiagnosticUNet(in_ch=4, base_ch=base_ch, channel_mults=mults,
                                    attn_at=tuple(i == len(mults) - 1 for i in range(len(mults))),
                                    cond_drop_prob=0.2, hint_in_ch=11)
    if not a.from_scratch:
        model.load_state_dict(ck["model"], strict=True)
    model = model.to(dev).train()
    # activation checkpointing: recompute ResBlock/Attn activations in backward (A10 22GB)
    import torch.utils.checkpoint as _ckpt
    def _wrap(mod):
        orig = mod.forward
        def f(*args, **kw):
            if torch.is_grad_enabled():
                return _ckpt.checkpoint(orig, *args, use_reentrant=False, **kw)
            return orig(*args, **kw)
        mod.forward = f
    if a.act_ckpt:
        n_wrapped = 0
        for m in model.modules():
            if isinstance(m, (ResBlock, Attn)):
                _wrap(m); n_wrapped += 1
        print(f"activation checkpointing on {n_wrapped} blocks", flush=True)
    raw_model = model
    if ddp:
        model = torch.nn.parallel.DistributedDataParallel(model, device_ids=[int(os.environ["LOCAL_RANK"])])
    opt = torch.optim.AdamW(model.parameters(), lr=a.lr, weight_decay=0.01)
    dc = build_diffusion_constants(1000, dev, torch.float32)
    step0 = 0
    resume = out / "latest.pt"
    if resume.is_file():
        rk = torch.load(resume, map_location="cpu", weights_only=False)
        raw_model.load_state_dict(rk["model"]); opt.load_state_dict(rk["optimizer"]); step0 = rk["step"]
        per_rank = rk.get("rng_by_rank")
        if isinstance(per_rank, list) and len(per_rank) == world:
            state = per_rank[rank]
            torch.set_rng_state(state["torch"])
            torch.cuda.set_rng_state(state["cuda"], dev)
            np.random.set_state(state["numpy"])
            random.setstate(state["python"])
            resume_mode = "exact_per_rank_v1"
        else:
            # Backward-compatible recovery from pre-fix rank-0-only checkpoints.
            # Rank 0 can restore its actual stream; the other streams were never
            # recorded, so seed them distinctly and say so in the receipt.
            fallback_seed = int((a.seed + 1_000_003 * rank + 7_919 * step0) % (2**31 - 1))
            torch.manual_seed(fallback_seed)
            torch.cuda.manual_seed(fallback_seed)
            np.random.seed(fallback_seed % (2**32))
            random.seed(fallback_seed)
            if rank == 0 and all(k in rk for k in ("rng_torch", "rng_cuda", "rng_np", "rng_py")):
                torch.set_rng_state(rk["rng_torch"])
                old_cuda = rk["rng_cuda"]
                torch.cuda.set_rng_state(old_cuda[0] if isinstance(old_cuda, list) else old_cuda, dev)
                np.random.set_state(rk["rng_np"])
                random.setstate(rk["rng_py"])
            resume_mode = "legacy_rank0_plus_distinct_rank_fallback"
        print(f"resumed at step {step0} mode={resume_mode}", flush=True)
    else:
        torch.manual_seed(a.seed + rank); np.random.seed(a.seed + rank); random.seed(a.seed + rank)
        resume_mode = "fresh_rank_seed"

    hist = open(out / "history.jsonl", "a") if is_main else open(os.devnull, "w")
    args_d = {k: getattr(a, k.replace("-", "_")) if hasattr(a, k.replace("-", "_")) else None for k in vars(a)}
    if a.student_base_ch > 0 or a.student_mults:
        args_d["base_ch"] = int(base_ch)
        args_d["mults"] = tuple(mults)
    args_d = vars(a)
    t0 = time.time(); losses = []
    from concurrent.futures import ThreadPoolExecutor
    pool = ThreadPoolExecutor(max_workers=a.prefetch)
    cond_map = None
    if a.shuffle_conditions:
        if a.cond_seed < 0:
            raise SystemExit("FAIL: --shuffle-conditions requires an explicit --cond-seed (independent declared derangement seed)")
        # Fixed derangement over the row list under its OWN declared seed: every C
        # trains against a wrong-but-real E. Independent of --seed so the shuffle
        # cannot correlate with weight init or batch order (halo review point 1).
        import random as _rnd
        idx = list(range(len(rows)))
        g = _rnd.Random(int(a.cond_seed))
        while True:
            perm = idx[:]
            g.shuffle(perm)
            if all(perm[i] != i for i in idx):
                break
        cond_map = perm
        if is_main:
            print(f"[control] shuffle-conditions ON: derangement over {len(rows)} rows, cond_seed {a.cond_seed} (independent of main seed {a.seed})", flush=True)
    def sample_one(slot):
        # Sampling is a pure function of the completed optimizer step and rank.
        # Prefetched work therefore needs no opaque queue state in a checkpoint.
        key = (int(a.seed) ^ (int(rank) << 32) ^ int(slot)) & ((1 << 64) - 1)
        i = splitmix64(key) % len(rows)
        sid, r = rows[i]
        if cond_map is None:
            return load_C(sid, r), load_E(sid, r)
        es, er = rows[cond_map[i]]
        return load_C(sid, r), load_E(es, er)
    first_slot = step0 * a.accum * a.bs
    pending = [pool.submit(sample_one, first_slot + i) for i in range(a.prefetch)]
    next_slot = first_slot + a.prefetch
    for step in range(step0, a.max_steps):
        lr = a.lr * min(1.0, (step + 1) / a.warmup) * (0.5 * (1 + math.cos(math.pi * min(step, a.cosine_horizon) / a.cosine_horizon)))
        for g in opt.param_groups:
            g["lr"] = lr
        opt.zero_grad(set_to_none=True)
        for _ in range(a.accum):
            Cs, Es = [], []
            for _b in range(a.bs):
                Cb, Eb = pending.pop(0).result()
                pending.append(pool.submit(sample_one, next_slot))
                next_slot += 1
                Cs.append(Cb); Es.append(Eb)
            C = torch.stack(Cs).to(dev, non_blocking=True)
            E = torch.stack(Es).to(dev, non_blocking=True)
            t = torch.randint(0, 1000, (a.bs,), device=dev)
            noise = torch.randn(a.bs, 4, TH, TW, device=dev)
            Ct = q_sample(C, t, dc, noise)
            with torch.autocast("cuda", dtype=torch.bfloat16):
                eps = model(Ct.to(torch.bfloat16), E, t)
                loss = F.mse_loss(eps.float(), noise) / a.accum
            loss.backward()
            losses.append(float(loss) * a.accum)
        torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        opt.step()
        if (step + 1) % 50 == 0 and is_main:
            m = float(np.mean(losses[-50 * a.accum:]))
            if not math.isfinite(m):
                raise SystemExit("FAIL: non-finite loss")
            line = {"step": step + 1, "loss": m, "lr": lr, "elapsed_s": time.time() - t0}
            hist.write(json.dumps(line) + "\n"); hist.flush()
            print(line, flush=True)
        if (step + 1) % a.ckpt_every == 0:
            local_rng = capture_rng_state(dev)
            if ddp:
                rng_by_rank = [None] * world if is_main else None
                dist.gather_object(local_rng, rng_by_rank, dst=0)
            else:
                rng_by_rank = [local_rng]
            if is_main:
                save_ckpt(out / "latest.pt", raw_model, opt, step + 1, rng_by_rank, args_d)
                if (step + 1) % (a.ckpt_every * 5) == 0:
                    save_ckpt(out / f"step_{step+1:08d}.pt", raw_model, opt, step + 1, rng_by_rank, args_d)
            if ddp:
                dist.barrier()
        if (step + 1) % a.eval_every == 0 and is_main:
            r = quick_eval(raw_model, dc, dev, ev)
            line = {"step": step + 1, "eval": r}
            hist.write(json.dumps(line) + "\n"); hist.flush()
            print("EVAL", json.dumps(line), flush=True)


if __name__ == "__main__":
    main()
