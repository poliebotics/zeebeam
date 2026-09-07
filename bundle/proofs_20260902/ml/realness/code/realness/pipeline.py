#!/usr/bin/env python3
"""Orchestration shared by scorer.py and the two attack scripts.

Builds the frozen splits, proves disjointness, fits the known-real bank, scores genuine
sequences, and evaluates a generator-free attack class as a SECURITY DIAGNOSTIC (never a
pooled realness ROC).
"""
from __future__ import annotations

import sys as _sys
_sys.dont_write_bytecode = True

from typing import Dict, List, Callable
import numpy as np

from . import holdout as H
from . import features as Ft
from . import metrics as M


def build_and_check_splits(cfg: dict) -> List[H.SubSeq]:
    ho = cfg["holdout"]
    subs = H.build_subsequences(ho["eval_blocks"], ho["subsequence_len"])
    subs = H.assign_splits(subs, ho["split_seed"])
    H.assert_disjoint(subs)               # raises DisjointnessError on any leak
    return subs


def subseqs_by_split(subs: List[H.SubSeq], split: str) -> List[H.SubSeq]:
    return [s for s in subs if s.split == split]


def genuine_sequences(subs: List[H.SubSeq], split: str) -> List[Ft.Sequence]:
    out = []
    for s in subseqs_by_split(subs, split):
        frames = [Ft.PresentedFrame(present=(s.session, r), declared=(s.session, r))
                  for r in s.rows]
        out.append(Ft.Sequence(name=s.name, session=s.session, block=s.block,
                               frames=frames, kind="genuine"))
    return out


def fit_bank_margins(subs: List[H.SubSeq], backend, eval_seed: int):
    """Pool per-frame margins over ALL bank rows to fit the f3 known-real distribution."""
    margins = []
    for s in subseqs_by_split(subs, "bank"):
        seq = Ft.Sequence(name=s.name, session=s.session, block=s.block,
                          frames=[Ft.PresentedFrame((s.session, r), (s.session, r))
                                  for r in s.rows], kind="genuine")
        margins.append(Ft.per_frame_margin(seq, backend, eval_seed))
    allm = np.concatenate(margins) if margins else np.array([])
    mu_B, sigma_B = Ft.fit_bank(allm)
    return mu_B, sigma_B


def score_sequences(seqs: List[Ft.Sequence], backend, eval_seed, mu_B, sigma_B,
                    bank_rows_scored: List[Dict], weights: dict) -> List[Dict]:
    rows = Ft.sequence_scores(seqs, backend, eval_seed, mu_B, sigma_B)
    Ft.combine_auxiliary(rows, bank_rows_scored, weights)
    return rows


def bank_sequences_scored(subs, backend, eval_seed, mu_B, sigma_B) -> List[Dict]:
    seqs = genuine_sequences(subs, "bank")
    return Ft.sequence_scores(seqs, backend, eval_seed, mu_B, sigma_B)


def evaluate_attack_class(
    attack_name: str,
    cfg: dict,
    subs: List[H.SubSeq],
    backend,
    make_forgeries: Callable[[List[H.SubSeq], str, int], List[Ft.Sequence]],
    eval_seeds: List[int],
    forger_seeds: List[int],
    score_key: str = "primary_score",
) -> Dict:
    """Run one generator-free attack class as a SECURITY DIAGNOSTIC.

    For each (eval_seed, forger_seed): fit bank, score genuine calib+test reals, build calib
    and test forgeries, fix the Youden-J threshold on CALIB (real vs this attack), and report
    on TEST: attack-success rate (= per-class FPR at that threshold), FNR, and a real-vs-this
    attack ROC/AUROC with a block-clustered CI. Explicitly labelled a security diagnostic;
    NOT pooled with any other class into a 'realness' number.
    """
    weights = cfg["aggregation"]["auxiliary_combined"]["weights"]
    mb = cfg["metrics"]["cluster_bootstrap"]
    per_run = []
    succ_test = []   # attack-success rates across runs
    for es in eval_seeds:
        mu_B, sigma_B = fit_bank_margins(subs, backend, es)
        bank_scored = bank_sequences_scored(subs, backend, es, mu_B, sigma_B)
        real_calib = score_sequences(genuine_sequences(subs, "calib"), backend, es,
                                     mu_B, sigma_B, bank_scored, weights)
        real_test = score_sequences(genuine_sequences(subs, "test"), backend, es,
                                    mu_B, sigma_B, bank_scored, weights)
        for fs in forger_seeds:
            forg_calib = make_forgeries(subs, "calib", fs)
            forg_test = make_forgeries(subs, "test", fs)
            fc = score_sequences(forg_calib, backend, es, mu_B, sigma_B, bank_scored, weights)
            ft = score_sequences(forg_test, backend, es, mu_B, sigma_B, bank_scored, weights)

            pos_c = np.array([r[score_key] for r in real_calib], float)
            neg_c = np.array([r[score_key] for r in fc], float)
            thr, J = M.youden_threshold(pos_c, neg_c)

            pos_t = np.array([r[score_key] for r in real_test], float)
            neg_t = np.array([r[score_key] for r in ft], float)
            rates = M.rates_at(thr, pos_t, neg_t)
            roc = M.roc_with_ci(real_test, ft, score_key,
                                n_boot=mb["n_boot"], alpha=mb["alpha"], seed=fs)
            attack_success_rate = rates["FPR"]   # forgeries passing as real on TEST
            succ_test.append(attack_success_rate)
            per_run.append({
                "eval_seed": es, "forger_seed": fs,
                "calib_threshold": thr, "youden_J": J,
                "SECURITY_DIAGNOSTIC_attack_success_rate_test": attack_success_rate,
                "FNR_test_real_rejected": rates["FNR"],
                "security_diagnostic_auroc_real_vs_attack": roc,
                "n_forgeries_test": len(ft), "n_real_test": len(real_test),
                "forgery_scores_test": [r[score_key] for r in ft],
                "real_scores_test": [r[score_key] for r in real_test],
            })
    arr = np.array(succ_test, float)
    return {
        "LABEL": "SECURITY_DIAGNOSTIC",
        "NOT_A_REALNESS_ROC": True,
        "attack_class": attack_name,
        "explanation": ("generator-free attack. Reported as an attack-success distribution, "
                        "NOT a realness ROC and NOT pooled with any other class. See "
                        "LIMITATIONS.md."),
        "score_key": score_key,
        "attack_success_rate_test": {
            "mean": float(np.nanmean(arr)) if arr.size else float("nan"),
            "min": float(np.nanmin(arr)) if arr.size else float("nan"),
            "max": float(np.nanmax(arr)) if arr.size else float("nan"),
            "per_run_values": arr.tolist(),
        },
        "runs": per_run,
    }
