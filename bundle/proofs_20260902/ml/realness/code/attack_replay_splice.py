#!/usr/bin/env python3
"""Attack class 3 -- replay / splice of real frames under mismatched patterns.

SECURITY DIAGNOSTIC, NOT a realness ROC (see LIMITATIONS.md). Generator-free: it only
rearranges real captures so a frame is PRESENTED at a chain position (with a declared
emission E_t) it does not belong to. The frozen scorer measures consistency of the
presented frame against the DECLARED emission, so a correct scorer should reject the
mismatch. The output is the attack-SUCCESS distribution (forgeries passing as real).

Declared variants (attacker knows the published protocol):
  * reorder   : present the subsequence's own real frames rolled by a seed-chosen shift,
                declared positions unchanged (temporal splice within the block).
  * cross     : present real frames taken from a DIFFERENT subsequence of the SAME split,
                under this subsequence's declared positions (cross-block splice).
  * replay    : hold one real frame and replay it across every declared position.

Forgeries for a split source real frames ONLY from that split, preserving bank/calib/test
disjointness on the negative side too (declared, conservative).

Seeds: forger_seeds and eval_seeds from the frozen config; all reported.
"""
from __future__ import annotations

import sys, os
sys.dont_write_bytecode = True
os.environ.setdefault("PYTHONDONTWRITEBYTECODE", "1")

import argparse, json, random
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from realness import pipeline as P
from realness import holdout as H
from realness import features as Ft
from realness.frozen import load_config, sha256_file
from scorer import make_backend


def make_forgeries(subs, split, forger_seed):
    """Build class-3 forged sequences from real rows of `split` only."""
    rng = random.Random(f"replay_splice|{forger_seed}|{split}")
    pool = P.subseqs_by_split(subs, split)
    out = []
    for s in pool:
        rows = s.rows
        n = len(rows)
        decl = [(s.session, r) for r in rows]

        # reorder (roll by a nonzero shift)
        shift = 1 + rng.randrange(max(1, n - 1))
        pres = [(s.session, rows[(i + shift) % n]) for i in range(n)]
        out.append(Ft.Sequence(
            name=f"{s.name}__reorder{shift}", session=s.session, block=s.block,
            frames=[Ft.PresentedFrame(pres[i], decl[i]) for i in range(n)],
            kind="replay_splice", meta={"variant": "reorder", "shift": shift,
                                        "forger_seed": forger_seed}))

        # cross-block splice from another subsequence in the same split
        others = [o for o in pool if o.name != s.name] or [s]
        donor = rng.choice(others)
        pres2 = [(donor.session, donor.rows[i % len(donor.rows)]) for i in range(n)]
        out.append(Ft.Sequence(
            name=f"{s.name}__cross_{donor.name}", session=s.session, block=s.block,
            frames=[Ft.PresentedFrame(pres2[i], decl[i]) for i in range(n)],
            kind="replay_splice", meta={"variant": "cross", "donor": donor.name,
                                        "forger_seed": forger_seed}))

        # replay one held frame across all declared positions
        held = (s.session, rows[rng.randrange(n)])
        out.append(Ft.Sequence(
            name=f"{s.name}__replay", session=s.session, block=s.block,
            frames=[Ft.PresentedFrame(held, decl[i]) for i in range(n)],
            kind="replay_splice", meta={"variant": "replay", "held_row": held[1],
                                        "forger_seed": forger_seed}))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", required=True)
    ap.add_argument("--backend", choices=["fixture", "diffusion"], default="diffusion")
    ap.add_argument("--v1_1-dir", default="")
    ap.add_argument("--out-dir", required=True)
    a = ap.parse_args()

    cfg = load_config(Path(a.config))
    out = Path(a.out_dir); out.mkdir(parents=True, exist_ok=True)
    subs = P.build_and_check_splits(cfg)
    H.assert_disjoint(subs)
    backend = make_backend(cfg, a.backend, a.v1_1_dir)

    diag = P.evaluate_attack_class(
        attack_name="class3_replay_splice", cfg=cfg, subs=subs, backend=backend,
        make_forgeries=make_forgeries,
        eval_seeds=cfg["seeds"]["eval_seeds"], forger_seeds=cfg["seeds"]["forger_seeds"])
    diag["config_sha256"] = cfg["_config_sha256"]
    diag["attack_py_sha256"] = sha256_file(Path(__file__))
    diag["backend"] = backend.name

    fn = out / "SECURITY_DIAGNOSTIC_class3_replay_splice.json"
    fn.write_text(json.dumps(diag, indent=2))
    asr = diag["attack_success_rate_test"]
    print(f"[class3 replay/splice] SECURITY DIAGNOSTIC -- attack success on test: "
          f"mean={asr['mean']:.3f} range=[{asr['min']:.3f},{asr['max']:.3f}]")
    print(f"wrote {fn}")


if __name__ == "__main__":
    main()
