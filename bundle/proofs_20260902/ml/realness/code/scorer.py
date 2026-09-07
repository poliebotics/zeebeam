#!/usr/bin/env python3
"""Frozen ZeeBeam realness discriminator (deliverable 1).

Implements the three declared features and the declared pooling, with weights/config frozen
in config/scorer_config.json and hashed here. Builds the whole-subsequence splits, proves
bank/calib/test disjointness LOUDLY, fits the known-real bank, and scores the genuine
(positive) calibration and test sequences. Attacks live in attack_replay_splice.py and
attack_whitebox.py and reuse this pipeline.

FEATURE INSTANTIATION (also stated in config/scorer_config.json and README.md):
  (1) optical/sensor consistency under the declared emission := per-frame
      margin_t = mean(available wrong-offset eps-MSE) - correct eps-MSE of the frozen
      Phase-G ARM-A step-16000 model. This is the published delta_wrong_mean per row.
      Pooled (the declared headline 'one number per sequence') = median_t(margin_t).
  (2) temporal coherence := median_t(-|margin_t - margin_{t-1}|) over consecutive presented
      rows -- the frame-to-frame stability of feature (1) along the chain.
  (3) known-real bank cross-correlation := median_t(exp(-0.5*((margin_t-mu_B)/sigma_B)^2)),
      a Gaussian-kernel match of the frame's consistency statistic to the bank distribution
      fitted on KNOWN-REAL bank rows (disjoint from calib/test).

Backends: `fixture` (CPU, no GPU, no v1_1 import -- for validation) and `diffusion` (the
frozen model on CUDA -- the full-scale run).
"""
from __future__ import annotations

import sys, os
sys.dont_write_bytecode = True
os.environ.setdefault("PYTHONDONTWRITEBYTECODE", "1")

import argparse, json
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from realness import pipeline as P
from realness import holdout as H
from realness import backends as B
from realness.frozen import load_config, sha256_file


def make_backend(cfg: dict, kind: str, v1_1_dir: str = "") -> B.ArmBackend:
    if kind == "fixture":
        rows = {}
        for sid, blks in cfg["holdout"]["eval_blocks"].items():
            rows[sid] = max(b[1] for b in blks)  # rows_total >= max block end
        return B.FixtureBackend(rows=rows)
    if kind == "diffusion":
        bb = cfg["backbone"]
        vd = v1_1_dir or str(Path(bb["model_source"]).parent)
        return B.DiffusionBackend(v1_1_dir=vd, checkpoint=bb["checkpoint"],
                                  checkpoint_sha256=bb["checkpoint_sha256"],
                                  t=cfg["arm_scoring"]["t"])
    raise SystemExit(f"unknown backend {kind}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", required=True)
    ap.add_argument("--backend", choices=["fixture", "diffusion"], default="diffusion")
    ap.add_argument("--v1_1-dir", default="")
    ap.add_argument("--out-dir", required=True)
    ap.add_argument("--eval-seed", type=int, default=None,
                    help="single eval seed; default = first of config seeds.eval_seeds")
    a = ap.parse_args()

    cfg = load_config(Path(a.config))
    out = Path(a.out_dir); out.mkdir(parents=True, exist_ok=True)
    es = a.eval_seed if a.eval_seed is not None else cfg["seeds"]["eval_seeds"][0]

    # 1. splits + LOUD disjointness proof
    subs = P.build_and_check_splits(cfg)
    disj = H.assert_disjoint(subs)
    (out / "splits.json").write_text(json.dumps(
        [{"name": s.name, "session": s.session, "block": list(s.block),
          "split": s.split, "rows": s.rows} for s in subs], indent=2))
    (out / "disjointness_report.json").write_text(json.dumps(disj, indent=2))
    print(f"[splits] {disj['counts']} rows per split; disjoint OK")

    backend = make_backend(cfg, a.backend, a.v1_1_dir)

    # 2. fit the known-real bank (feature 3 distribution)
    mu_B, sigma_B = P.fit_bank_margins(subs, backend, es)
    bank_scored = P.bank_sequences_scored(subs, backend, es, mu_B, sigma_B)
    print(f"[bank] mu_B={mu_B:.3e} sigma_B={sigma_B:.3e} over "
          f"{sum(len(s.rows) for s in subs if s.split=='bank')} known-real rows")

    # 3. score genuine positives (calib + test)
    weights = cfg["aggregation"]["auxiliary_combined"]["weights"]
    real_calib = P.score_sequences(P.genuine_sequences(subs, "calib"), backend, es,
                                   mu_B, sigma_B, bank_scored, weights)
    real_test = P.score_sequences(P.genuine_sequences(subs, "test"), backend, es,
                                  mu_B, sigma_B, bank_scored, weights)

    report = {
        "label": "ZeeBeam realness discriminator -- genuine (positive) scores",
        "config_sha256": cfg["_config_sha256"],
        "scorer_py_sha256": sha256_file(Path(__file__)),
        "backend": backend.name,
        "eval_seed": es,
        "bank": {"mu_B": mu_B, "sigma_B": sigma_B},
        "feature_instantiation": {
            "f1_consistency": cfg["features"]["f1_consistency"]["per_frame"],
            "f2_temporal": cfg["features"]["f2_temporal"]["per_adjacent"],
            "f3_bank_xcorr": cfg["features"]["f3_bank_xcorr"]["per_frame"],
            "headline_pooling": cfg["aggregation"]["primary_sequence_score"],
        },
        "positives": {"calib": real_calib, "test": real_test},
    }
    (out / "positives_scores.json").write_text(json.dumps(report, indent=2))
    n = len(real_test)
    print(f"[positives] scored {len(real_calib)} calib + {n} test genuine sequences")
    print(f"wrote {out/'positives_scores.json'}")


if __name__ == "__main__":
    main()
