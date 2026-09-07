#!/usr/bin/env python3
"""Small-scale CPU fixture validation (deliverable 4).

Proves, with NO GPU and NO frozen-model dependency:
  A. the whole pipeline runs end to end on synthetic data: splits -> disjointness ->
     bank fit -> genuine scoring -> class-3 attack -> class-4 white-box harness;
  B. the disjointness check ACTUALLY FIRES when a bank/test leak is deliberately injected
     (a leak here would invalidate the study, so the guard must be proven to bite);
  C. the pipeline behaves sensibly: genuine sequences outscore splices; white-box attack
     success rises with perturbation budget.

Run: PYTHONDONTWRITEBYTECODE=1 python3 run_fixture.py --out-dir out/fixture
"""
from __future__ import annotations

import sys, os
sys.dont_write_bytecode = True
os.environ.setdefault("PYTHONDONTWRITEBYTECODE", "1")

import argparse, json, subprocess
from pathlib import Path
import numpy as np

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from realness import holdout as H
from realness.holdout import DisjointnessError


FIXTURE_CONFIG = {
    "schema": "zeebeam-realness-scorer/v1",
    "frozen_utc": "2026-08-30",
    "note": "FIXTURE config: tiny synthetic sessions, CPU only. Not the real study config.",
    "backbone": {"role": "fixture (no model)"},
    "arm_scoring": {"t": 150, "K": 1, "offsets": [-2, 2, -15, 15, 30]},
    "features": {
        "f1_consistency": {"per_frame": "margin_t = mean(wrong eps-MSE) - correct eps-MSE"},
        "f2_temporal": {"per_adjacent": "-|margin_t - margin_{t-1}|"},
        "f3_bank_xcorr": {"per_frame": "exp(-0.5*((margin_t-mu_B)/sigma_B)^2)"},
    },
    "aggregation": {
        "primary_sequence_score": "median_t(margin_t)",
        "auxiliary_combined": {"weights": {"w_f1": 1.0, "w_f2": 0.5, "w_f3": 0.5}},
    },
    "holdout": {
        "unit": "whole contiguous subsequence",
        "subsequence_len": 20,
        "eval_blocks": {"fa": [[0, 60], [100, 160], [200, 260]],
                        "fb": [[0, 60], [100, 160]]},
        "split_seed": 20260830,
    },
    "seeds": {"eval_seeds": [20260823, 20260901, 20260902], "forger_seeds": [11, 22, 33]},
    "metrics": {"cluster_bootstrap": {"unit": "eval block", "n_boot": 500, "alpha": 0.05}},
}


def run(cmd):
    print("+", " ".join(cmd))
    r = subprocess.run(cmd, cwd=str(HERE),
                       env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
                       capture_output=True, text=True)
    print(r.stdout)
    if r.returncode != 0:
        print(r.stderr)
        raise SystemExit(f"FIXTURE FAIL: command exited {r.returncode}")
    return r.stdout


def check_disjointness_fires(cfg):
    """B. Deliberately leak a test row into a bank subsequence; assert the guard raises."""
    subs = H.build_subsequences(cfg["holdout"]["eval_blocks"], cfg["holdout"]["subsequence_len"])
    subs = H.assign_splits(subs, cfg["holdout"]["split_seed"])
    H.assert_disjoint(subs)  # clean baseline must pass
    bank = next(s for s in subs if s.split == "bank")
    test = next(s for s in subs if s.split == "test")
    leaked_row = test.rows[0]
    bank.rows = list(bank.rows) + [leaked_row]     # inject the leak
    fired = False
    try:
        H.assert_disjoint(subs)
    except DisjointnessError as e:
        fired = True
        msg = str(e)
    if not fired:
        raise SystemExit("FIXTURE FAIL: disjointness check did NOT fire on a deliberate leak")
    print(f"[B] disjointness guard fired as required: {msg[:90]}...")
    return {"deliberate_leak_detected": True, "leaked_row": leaked_row,
            "error_message": msg}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", default="out/fixture")
    a = ap.parse_args()
    out = (HERE / a.out_dir) if not os.path.isabs(a.out_dir) else Path(a.out_dir)
    out.mkdir(parents=True, exist_ok=True)
    cfgp = out / "fixture_config.json"
    cfgp.write_text(json.dumps(FIXTURE_CONFIG, indent=2))

    # A. end-to-end via the real CLIs on the fixture backend
    run([sys.executable, "scorer.py", "--config", str(cfgp), "--backend", "fixture",
         "--out-dir", str(out)])
    run([sys.executable, "attack_replay_splice.py", "--config", str(cfgp),
         "--backend", "fixture", "--out-dir", str(out)])
    run([sys.executable, "attack_whitebox.py", "--config", str(cfgp),
         "--backend", "fixture", "--out-dir", str(out)])

    # B. the guard bites on a deliberate leak
    leak = check_disjointness_fires(FIXTURE_CONFIG)
    (out / "leak_test_result.json").write_text(json.dumps(leak, indent=2))

    # C. sanity: genuine outscore splices; white-box success rises with budget
    pos = json.loads((out / "positives_scores.json").read_text())
    c3 = json.loads((out / "SECURITY_DIAGNOSTIC_class3_replay_splice.json").read_text())
    c4 = json.loads((out / "SECURITY_DIAGNOSTIC_class4_whitebox.json").read_text())
    real_test = [r["primary_score"] for r in pos["positives"]["test"]]
    forg = c3["runs"][0]["forgery_scores_test"]
    genuine_beats_splice = float(np.median(real_test)) > float(np.median(forg))
    sweep = c4["attack_success_rate_test_by_budget"]
    lo = sweep[str(0.0)]["mean"]
    hi = sweep[list(sweep.keys())[-1]]["mean"]
    budget_monotone = (hi >= lo)
    checks = {
        "A_pipeline_end_to_end": True,
        "B_disjointness_guard_fires": leak["deliberate_leak_detected"],
        "C_genuine_median_beats_splice_median": genuine_beats_splice,
        "C_whitebox_success_rises_with_budget": budget_monotone,
        "detail": {"genuine_median": float(np.median(real_test)),
                   "splice_median": float(np.median(forg)),
                   "wb_success_budget0": lo, "wb_success_budgetmax": hi},
    }
    (out / "FIXTURE_VALIDATION.json").write_text(json.dumps(checks, indent=2))
    print("\n=== FIXTURE VALIDATION ===")
    print(json.dumps(checks, indent=2))
    if not all(v for k, v in checks.items() if k.startswith(("A_", "B_", "C_"))):
        raise SystemExit("FIXTURE FAIL: one or more checks did not pass")
    print("\nALL FIXTURE CHECKS PASSED")


if __name__ == "__main__":
    main()
