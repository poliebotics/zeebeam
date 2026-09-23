#!/usr/bin/env python3
"""F-A v1 fakes for the realness held-out sessions d2/v10 (calib+test rows only).

Purpose: domain-matched generator-based NEGATIVES for the realness real-vs-generated ROC.
The august fake table cannot serve: positives are d2/v10 sequences, so negatives must be
d2/v10 rows or the ROC separates sessions, not fakeness. Recipe unchanged:
C_fake = F(C_(r-2), E_(r-2), E_r), SOURCE_LAG=2, uint8 commit round(clamp(f32,0,1)*255),
CUDA bf16 autocast, step-100k checkpoint only (the strongest published forger).
POSITIVE CONTROL first: regenerate august row 96 step-100k and require BIT-EXACT equality
with the existing table fake (same box, same precision path) before touching d2/v10.
Rows: the union of calib+test rows from the frozen splits (identical across eval seeds,
disjointness independently confirmed 31 Aug); bank rows stay genuine-only.
"""
import hashlib, json, os, pathlib, sys, time
sys.dont_write_bytecode = True
SNAP = "/lambda/nfs/ZeeBeam/truthbeam/models/fa_v1_forger/code_snapshot_20260831"
sys.path.insert(0, SNAP + "/pkgs"); sys.path.insert(0, SNAP)
import numpy as np, torch
from PIL import Image
from phase_f.editor_controlnet import EditorControlNet

BASE = pathlib.Path("/lambda/nfs/ZeeBeam")
OUT = BASE / "experiments/fa_fake_table_20260831/d2v10_fakes"
TABLE = BASE / "experiments/fa_fake_table_20260831"
SPLITS = BASE / "experiments/realness_20260830/out/full/seed_20260823/splits.json"
SESS = {
    "d2": {"dir": BASE / "truthbeam/sessions/d2", "raw": "Recordings/frame_{r:06d}.raw"},
    "v10": {"dir": BASE / "truthbeam/sessions/v10", "raw": "Recordings/frame_{r:06d}.raw"},
    "august": {"dir": BASE / "august_dev_712", "raw": "Recordings/frame_{r:06d}.raw"},
}
W, H, LAG = 5320, 4600, 2
CKPT = "step_00100000"
CKPT_SHA = "2bf156d07b1ddf72ec53dab500cc0df2344596edd40890f904a60901217ad92e"
sha = lambda b: hashlib.sha256(b).hexdigest()

def load_C(sid, row):
    p = SESS[sid]["dir"] / SESS[sid]["raw"].format(r=row)
    raw = np.frombuffer(p.read_bytes(), dtype=np.uint8)
    assert raw.size == W * H, f"{sid} row {row} raw size {raw.size}"
    r2 = raw.reshape(H, W)
    cfa = np.stack([r2[0::2, 0::2], r2[0::2, 1::2], r2[1::2, 0::2], r2[1::2, 1::2]], 0)
    return torch.from_numpy(cfa.astype(np.float32) / 255.0)

def load_E(sid, row):
    im = Image.open(SESS[sid]["dir"] / "derived" / "Emissions" / f"tile_{row:06d}.png").convert("RGB")
    assert im.size == (1920, 1080), f"{sid} row {row} emission {im.size}"
    return torch.from_numpy(np.asarray(im).astype(np.float32) / 255.0).permute(2, 0, 1).contiguous()

def main():
    assert torch.cuda.is_available()
    dev = "cuda"
    OUT.mkdir(parents=True, exist_ok=True)
    ck_path = f"/lambda/nfs/ZeeBeam/truthbeam/models/fa_v1_forger/f_a_v1_{CKPT}.pt"
    got = sha(open(ck_path, "rb").read())
    assert got == CKPT_SHA, got
    with torch.serialization.safe_globals([pathlib.PosixPath]):
        ck = torch.load(ck_path, map_location="cpu", weights_only=True)
    args = ck.get("args", {})
    m = EditorControlNet(init_mode="scratch",
                         hint_use_source=(args.get("hint_mode", "v1_5_treatment") == "v1_5_treatment"))
    m.load_state_dict(ck["editor"], strict=True); m.eval(); m.to(dev)

    def gen(sid, row):
        C_src = load_C(sid, row - LAG).unsqueeze(0).to(dev)
        E_src = load_E(sid, row - LAG).unsqueeze(0).to(dev)
        E_tgt = load_E(sid, row).unsqueeze(0).to(dev)
        with torch.no_grad(), torch.autocast("cuda", dtype=torch.bfloat16):
            fake = m(C_src, E_src, E_tgt)
        u8 = (fake.float().squeeze(0).clamp(0, 1) * 255).round().to(torch.uint8).cpu().numpy()
        return np.ascontiguousarray(u8).tobytes(order="C")

    # POSITIVE CONTROL: august row 96 must be BIT-EXACT vs the existing table fake.
    table_man = [json.loads(l) for l in open(TABLE / "MANIFEST.jsonl")]
    want = next(r["fake_bytes_sha256"] for r in table_man if r["row"] == 96 and r["ckpt"] == CKPT)
    pc = sha(gen("august", 96))
    assert pc == want, f"PC FAIL: regen {pc} != table {want}"
    print(f"PC PASS: august row 96 {CKPT} regenerated bit-exact ({pc[:16]})", flush=True)

    splits = json.load(open(SPLITS))
    todo = {}
    for s in splits:
        if s["split"] in ("calib", "test"):
            todo.setdefault(s["session"], set()).update(s["rows"])
    man = open(OUT / "MANIFEST_d2v10.jsonl", "a")
    total = sum(len(v) for v in todo.values()); done = 0; t0 = time.time()
    for sid in sorted(todo):
        for row in sorted(todo[sid]):
            fp = OUT / f"fake_{sid}_row{row:06d}_{CKPT}.u8cfa"
            if fp.exists() and fp.stat().st_size == 4 * 2300 * 2660:
                done += 1; continue
            bts = gen(sid, row)
            tmp = fp.with_suffix(".tmp")
            with open(tmp, "wb") as f: f.write(bts)
            os.replace(tmp, fp)
            man.write(json.dumps({"session": sid, "row": row, "source_row": row - LAG,
                                   "ckpt": CKPT, "checkpoint_sha256": CKPT_SHA,
                                   "fake_bytes_sha256": sha(bts), "bytes": len(bts)}) + "\n")
            man.flush(); done += 1
            if done % 50 == 0:
                print(f"{done}/{total} elapsed={(time.time()-t0)/60:.1f}m", flush=True)
    man.close()
    print(f"D2V10 FAKES COMPLETE {done}/{total}", flush=True)

if __name__ == "__main__":
    main()
