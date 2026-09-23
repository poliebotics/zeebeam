#!/usr/bin/env python3
"""Real-vs-GENERATED realness scoring: F-A v1 (step-100k) fakes on held-out d2/v10.

Dated extension beside the frozen realness deliverables (nothing frozen is edited). The
negatives are GENERATOR-BASED: for every calib/test row r, the presented frame is the F-A
fake C_fake = F(C_(r-2), E_(r-2), E_r) — the forged chain presented under its OWN declared
emissions (presented key (sid_fake, r), declared key (sid, r)). Fakes reach the frozen
ARM-A loader through re-mosaiced shadow raws (exact permutation inverse, round-trip
asserted at write time), so the presented path is byte-faithful to the loader's own code.
Unlike classes 3/4 (generator-free SECURITY DIAGNOSTICS), this run IS a real-vs-generated
ROC on the held-out sessions, with declared limits: ONE published forger checkpoint
(step-100k, sha 2bf156d0...), one occasion, zero-shot April->August forger applied to
d2/v10, noise draws keyed by presented identity per the frozen convention.
"""
import sys, os, json
sys.dont_write_bytecode = True
os.environ.setdefault("PYTHONDONTWRITEBYTECODE", "1")
from pathlib import Path
RD = Path("/lambda/nfs/ZeeBeam/experiments/realness_20260830")
sys.path.insert(0, str(RD))
from realness import pipeline as P
from realness import holdout as H
from realness import features as Ft
from realness.frozen import load_config, sha256_file
from scorer import make_backend

SHADOW = Path("/lambda/nfs/ZeeBeam/experiments/fa_fake_table_20260831/fake_sessions")
OUT = Path("/lambda/nfs/ZeeBeam/experiments/fa_fake_table_20260831/realness_roc")
CKPT_SHA = "2bf156d07b1ddf72ec53dab500cc0df2344596edd40890f904a60901217ad92e"


def make_forgeries(subs, split, forger_seed):
    out = []
    for s in P.subseqs_by_split(subs, split):
        out.append(Ft.Sequence(
            name=f"{s.name}__fa_v1_100k", session=s.session, block=s.block,
            frames=[Ft.PresentedFrame((f"{s.session}_fake", r), (s.session, r)) for r in s.rows],
            kind="generated_fa_v1",
            meta={"forger_checkpoint_sha256": CKPT_SHA, "recipe": "C_fake=F(C_(r-2),E_(r-2),E_r)",
                  "forger_seed_tag": forger_seed}))
    return out


def main():
    cfg = load_config(RD / "config/scorer_config.json")
    OUT.mkdir(parents=True, exist_ok=True)
    subs = P.build_and_check_splits(cfg)
    H.assert_disjoint(subs)
    backend = make_backend(cfg, "diffusion", "/lambda/nfs/ZeeBeam/experiments/student_scaling_20260828/v1_1")
    for sid in ("d2", "v10"):
        entry = dict(backend.T.SESSIONS[sid])
        entry["dir"] = SHADOW / sid
        backend.T.SESSIONS[f"{sid}_fake"] = entry
    diag = P.evaluate_attack_class(
        attack_name="generated_fa_v1_step100k", cfg=cfg, subs=subs, backend=backend,
        make_forgeries=make_forgeries,
        eval_seeds=cfg["seeds"]["eval_seeds"], forger_seeds=[100000])
    diag["LABEL"] = "REALNESS_NEGATIVES_GENERATED"
    diag["NOT_A_REALNESS_ROC"] = False
    diag["realness_roc_note"] = (
        "Generator-based negatives on the held-out sessions: this IS a real-vs-generated "
        "ROC, with limits: one published F-A v1 checkpoint (step-100k), zero-shot forger, "
        "one occasion, presented==declared chain semantics, shadow-raw loader path "
        "(round-trip asserted), noise keyed by presented identity per the frozen convention.")
    diag["forger_checkpoint_sha256"] = CKPT_SHA
    diag["config_sha256"] = cfg["_config_sha256"]
    diag["script_sha256"] = sha256_file(Path(__file__))
    fn = OUT / "REALNESS_ROC_generated_fa_v1_step100k.json"
    fn.write_text(json.dumps(diag, indent=2))
    asr = diag["attack_success_rate_test"]
    print(f"[generated fa_v1 100k] fake-pass rate on test: mean={asr['mean']:.3f} "
          f"range=[{asr['min']:.3f},{asr['max']:.3f}]")
    print(f"wrote {fn}")


if __name__ == "__main__":
    main()
