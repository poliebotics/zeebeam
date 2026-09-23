#!/usr/bin/env python3
"""Recompute every run's AUROC and error counts from the per-sequence scores in the two realness records.

No model, no GPU, standard library only. Higher score = more consistent with the declared emission; a sequence is
accepted as real when its score is at or above the run's recorded calibration threshold.
"""
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
RECORDS = [HERE.parent / "REALNESS_ROC_generated_fa_v1_step100k.json", HERE / "SECURITY_DIAGNOSTIC_class3_replay_splice.json"]


def auroc(pos, neg):
    return sum((p > n) + 0.5 * (p == n) for p in pos for n in neg) / (len(pos) * len(neg))


bad = 0
for path in RECORDS:
    rec = json.loads(path.read_text())
    print(f"{path.name} ({rec['attack_class']})")
    for run in rec["runs"]:
        real, att, thr = run["real_scores_test"], run["forgery_scores_test"], run["calib_threshold"]
        a = auroc(real, att)
        fa = sum(s >= thr for s in att)
        fr = sum(s < thr for s in real)
        ok = (abs(a - run["security_diagnostic_auroc_real_vs_attack"]["auroc_point"]) < 1e-12
              and fa / len(att) == run["SECURITY_DIAGNOSTIC_attack_success_rate_test"]
              and fr / len(real) == run["FNR_test_real_rejected"])
        bad += not ok
        print(f"  eval seed {run['eval_seed']}, seed {run['forger_seed']}: {len(real)} real, {len(att)} attack, "
              f"AUROC {a:.4f}, attacks accepted {fa}, reals rejected {fr}  {'matches record' if ok else 'DIFFERS FROM RECORD'}")
print("all runs match their records" if not bad else f"{bad} run(s) differ from their records")
sys.exit(1 if bad else 0)
