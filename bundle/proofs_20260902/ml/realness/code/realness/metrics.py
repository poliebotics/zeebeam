#!/usr/bin/env python3
"""Sequence-level ROC/AUROC, block-CLUSTERED bootstrap CI, and Youden-J threshold.

Positive = real. All statistics are at the SEQUENCE level (one score per subsequence). The
bootstrap cluster is the contiguous eval BLOCK (declared), so resampling respects the
within-block dependence between adjacent subsequences rather than pretending they are
independent.
"""
from __future__ import annotations

import sys as _sys
_sys.dont_write_bytecode = True

from typing import Dict, List, Tuple
import numpy as np


def auroc(pos: np.ndarray, neg: np.ndarray) -> float:
    """Mann-Whitney AUROC with average-rank ties; positive scored higher = better."""
    pos = np.asarray(pos, float); neg = np.asarray(neg, float)
    pos = pos[np.isfinite(pos)]; neg = neg[np.isfinite(neg)]
    if pos.size == 0 or neg.size == 0:
        return float("nan")
    s = np.concatenate([pos, neg])
    order = np.argsort(s, kind="stable"); ss = s[order]
    ranks_sorted = np.empty(len(s)); i = 0
    while i < len(ss):
        j = i
        while j + 1 < len(ss) and ss[j + 1] == ss[i]:
            j += 1
        ranks_sorted[i:j + 1] = (i + j) / 2.0 + 1.0; i = j + 1
    ranks = np.empty_like(ranks_sorted); ranks[order] = ranks_sorted
    n1 = pos.size; n2 = neg.size
    U = ranks[:n1].sum() - n1 * (n1 + 1) / 2.0
    return float(U / (n1 * n2))


def youden_threshold(pos: np.ndarray, neg: np.ndarray) -> Tuple[float, float]:
    """Threshold maximising tpr - fpr (predict real if score >= thr). Returns (thr, J)."""
    pos = np.asarray(pos, float); pos = pos[np.isfinite(pos)]
    neg = np.asarray(neg, float); neg = neg[np.isfinite(neg)]
    cands = np.unique(np.concatenate([pos, neg]))
    # candidate thresholds midway between sorted unique values, plus the extremes
    mids = (cands[:-1] + cands[1:]) / 2.0 if cands.size > 1 else cands
    grid = np.concatenate([[cands.min() - 1], mids, [cands.max() + 1]])
    best_thr, best_J = grid[0], -1.0
    for thr in grid:
        tpr = float(np.mean(pos >= thr)) if pos.size else 0.0
        fpr = float(np.mean(neg >= thr)) if neg.size else 0.0
        J = tpr - fpr
        if J > best_J:
            best_J, best_thr = J, float(thr)
    return best_thr, best_J


def rates_at(threshold: float, pos: np.ndarray, neg: np.ndarray) -> Dict[str, float]:
    """FNR = real called fake; FPR = fake called real (predict real if score >= thr)."""
    pos = np.asarray(pos, float); pos = pos[np.isfinite(pos)]
    neg = np.asarray(neg, float); neg = neg[np.isfinite(neg)]
    fnr = float(np.mean(pos < threshold)) if pos.size else float("nan")
    fpr = float(np.mean(neg >= threshold)) if neg.size else float("nan")
    return {"threshold": float(threshold), "FNR": fnr, "FPR": fpr,
            "n_pos": int(pos.size), "n_neg": int(neg.size)}


def _cluster_bootstrap(stat_fn, clusters_pos, clusters_neg, n_boot, alpha, seed):
    """Resample whole clusters (with replacement) on each side; recompute stat_fn."""
    rng = np.random.RandomState(seed)
    cp = list(clusters_pos); cn = list(clusters_neg)
    boots = []
    for _ in range(n_boot):
        if cp:
            ip = rng.randint(0, len(cp), size=len(cp))
            pos = np.concatenate([cp[k] for k in ip]) if cp else np.array([])
        else:
            pos = np.array([])
        if cn:
            ino = rng.randint(0, len(cn), size=len(cn))
            neg = np.concatenate([cn[k] for k in ino]) if cn else np.array([])
        else:
            neg = np.array([])
        v = stat_fn(pos, neg)
        if np.isfinite(v):
            boots.append(v)
    boots = np.array(boots)
    if boots.size == 0:
        return (float("nan"), float("nan"), float("nan"))
    return (float(np.mean(boots)),
            float(np.quantile(boots, alpha / 2)),
            float(np.quantile(boots, 1 - alpha / 2)))


def _clusters_by_block(rows: List[Dict], score_key: str) -> List[np.ndarray]:
    """Group sequence scores by eval block -> arrays (the bootstrap cluster unit)."""
    groups: Dict[Tuple, List[float]] = {}
    for r in rows:
        key = (r["session"], tuple(r["block"]))
        groups.setdefault(key, []).append(r[score_key])
    return [np.array(v, float) for v in groups.values()]


def roc_with_ci(pos_rows: List[Dict], neg_rows: List[Dict], score_key: str,
                n_boot=2000, alpha=0.05, seed=0) -> Dict:
    """AUROC point estimate + block-clustered bootstrap CI."""
    pos = np.array([r[score_key] for r in pos_rows], float)
    neg = np.array([r[score_key] for r in neg_rows], float)
    point = auroc(pos, neg)
    cp = _clusters_by_block(pos_rows, score_key)
    cn = _clusters_by_block(neg_rows, score_key)
    mean, lo, hi = _cluster_bootstrap(lambda p, n: auroc(p, n), cp, cn, n_boot, alpha, seed)
    return {"auroc_point": point, "auroc_bootstrap_mean": mean, "auroc_ci95": [lo, hi],
            "n_pos": int(np.isfinite(pos).sum()), "n_neg": int(np.isfinite(neg).sum()),
            "n_pos_clusters": len(cp), "n_neg_clusters": len(cn),
            "CI_WARNING": (None if (len(cp) >= 3 and len(cn) >= 3) else
                           "fewer than 3 clusters on a side: this CI is NOT quotable")}
