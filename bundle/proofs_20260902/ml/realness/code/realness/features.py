#!/usr/bin/env python3
"""The three declared realness features and the declared pooling.

Given a sequence of PRESENTED frames (each with a DECLARED emission position) and a
backend that yields per-frame arm scores, compute:

  margin_t  = mean_over_available_offsets(wrong_mse) - correct_mse
              (feature 1 per-frame consistency signal; == published delta_wrong_mean)

  f1 (consistency), pooled  = median_t(margin_t)             <- the declared headline score
  f2 (temporal coherence)   = median_t(-|margin_t - margin_{t-1}|)   over consecutive rows
  f3 (bank cross-correlation) = median_t( exp(-0.5*((margin_t-mu_B)/sigma_B)^2) )

The bank stats (mu_B, sigma_B) come from KNOWN-REAL bank rows only, supplied by the caller
(the holdout module guarantees they are disjoint from calibration and test).
"""
from __future__ import annotations

import sys as _sys
_sys.dont_write_bytecode = True

from dataclasses import dataclass, field
from typing import List, Tuple, Dict, Optional

import numpy as np

Ref = Tuple[str, int]


@dataclass
class PresentedFrame:
    """One frame in a (genuine or forged) sequence."""
    present: Ref      # which real capture is actually shown
    declared: Ref     # which chain position/emission it is presented AS


@dataclass
class Sequence:
    """A contiguous scored unit (whole subsequence)."""
    name: str
    session: str
    block: Tuple[int, int]          # the eval block this came from (the bootstrap cluster)
    frames: List[PresentedFrame]
    kind: str = "genuine"           # genuine | replay_splice | whitebox
    meta: Dict = field(default_factory=dict)


def per_frame_margin(seq: Sequence, backend, eval_seed: int) -> np.ndarray:
    """margin_t for each presented frame (nan if no wrong arm was available)."""
    out = np.full(len(seq.frames), np.nan)
    for i, fr in enumerate(seq.frames):
        a = backend.arm_scores(fr.present, fr.declared, eval_seed)
        wrong = [v for k, v in a.items() if k.startswith("wrong_")]
        if not wrong or "correct" not in a:
            continue
        out[i] = float(np.mean(wrong) - a["correct"])
    return out


def fit_bank(bank_margins: np.ndarray) -> Tuple[float, float]:
    """mu_B, sigma_B over pooled known-real per-frame margins (ddof=1, floored)."""
    m = bank_margins[np.isfinite(bank_margins)]
    if m.size < 2:
        raise ValueError("bank has < 2 finite margins; cannot fit f3 known-real distribution")
    return float(m.mean()), float(max(m.std(ddof=1), 1e-9))


def pooled_features(margins: np.ndarray, mu_B: float, sigma_B: float) -> Dict[str, float]:
    """The three pooled features from a sequence's per-frame margins."""
    m = margins
    finite = m[np.isfinite(m)]
    if finite.size == 0:
        return {"f1_consistency": float("nan"), "f2_temporal": float("nan"),
                "f3_bank_xcorr": float("nan")}
    f1 = float(np.median(finite))
    # temporal: consecutive PRESENTED rows; skip pairs with a nan
    diffs = []
    for i in range(1, len(m)):
        if np.isfinite(m[i]) and np.isfinite(m[i - 1]):
            diffs.append(-abs(m[i] - m[i - 1]))
    f2 = float(np.median(diffs)) if diffs else 0.0
    z = (finite - mu_B) / sigma_B
    f3 = float(np.median(np.exp(-0.5 * z * z)))
    return {"f1_consistency": f1, "f2_temporal": f2, "f3_bank_xcorr": f3}


def sequence_scores(sequences: List[Sequence], backend, eval_seed: int,
                    mu_B: float, sigma_B: float) -> List[Dict]:
    """Primary score (= f1 pooled) plus the pooled feature vector for each sequence."""
    rows = []
    for seq in sequences:
        margins = per_frame_margin(seq, backend, eval_seed)
        pf = pooled_features(margins, mu_B, sigma_B)
        rows.append({
            "name": seq.name, "session": seq.session, "block": list(seq.block),
            "kind": seq.kind, "n_frames": len(seq.frames),
            "n_finite": int(np.isfinite(margins).sum()),
            "primary_score": pf["f1_consistency"],   # THE declared sequence score
            "features": pf,
            "margins": margins.tolist(),
            "meta": seq.meta,
        })
    return rows


def combine_auxiliary(scored: List[Dict], bank_scored: List[Dict],
                      weights: Dict[str, float]) -> None:
    """Attach the declared AUXILIARY combined z-score (not the headline).

    Standardised by BANK-subsequence statistics only, so test data never informs the
    combination. Mutates each row in `scored` in place, adding 'combined_aux'.
    """
    def col(rows, k):
        return np.array([r["features"][k] for r in rows], float)
    stats = {}
    for k in ("f1_consistency", "f2_temporal", "f3_bank_xcorr"):
        v = col(bank_scored, k)
        v = v[np.isfinite(v)]
        stats[k] = (float(v.mean()) if v.size else 0.0,
                    float(max(v.std(ddof=1), 1e-9)) if v.size > 1 else 1.0)
    wmap = {"f1_consistency": weights["w_f1"], "f2_temporal": weights["w_f2"],
            "f3_bank_xcorr": weights["w_f3"]}
    for r in scored:
        c = 0.0
        for k, w in wmap.items():
            mu, sd = stats[k]
            val = r["features"][k]
            z = (val - mu) / sd if np.isfinite(val) else 0.0
            c += w * z
        r["combined_aux"] = float(c)
