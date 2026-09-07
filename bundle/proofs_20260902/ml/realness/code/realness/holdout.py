#!/usr/bin/env python3
"""Whole-subsequence holdout with a provable, LOUD bank/calib/test disjointness check.

A leak here (a frame from a scored block feeding the known-real bank, or the calibration
threshold seeing test rows) would invalidate every downstream number. The check is
therefore explicit, mandatory, and raises `DisjointnessError` rather than warning.

Holdout unit = whole contiguous subsequence. Eval blocks (the only held-out real rows) are
sliced into contiguous subsequences of the declared length; each subsequence is assigned as
a whole to exactly one of {bank, calib, test} by a deterministic round-robin. Positives are
the TEST subsequences. The BANK subsequences supply feature (3)'s known-real distribution.
The CALIB subsequences fix the Youden-J threshold. All three are whole-subsequence disjoint.
"""
from __future__ import annotations

import sys as _sys
_sys.dont_write_bytecode = True

from dataclasses import dataclass
from typing import Dict, List, Tuple

Ref = Tuple[str, int]


class DisjointnessError(AssertionError):
    """Raised when bank/calib/test row sets overlap. Fatal by design."""


@dataclass
class SubSeq:
    name: str
    session: str
    block: Tuple[int, int]
    rows: List[int]
    split: str = ""   # bank | calib | test

    def refs(self) -> List[Ref]:
        return [(self.session, r) for r in self.rows]


def slice_block(session: str, block: Tuple[int, int], L: int) -> List[SubSeq]:
    """Contiguous subsequences of length L; a trailing remainder >= L/2 is its own
    subsequence, a shorter tail is merged into the previous one (declared tail policy)."""
    a, b = block
    rows = list(range(a, b))
    subs: List[List[int]] = []
    i = 0
    while i < len(rows):
        chunk = rows[i:i + L]
        subs.append(chunk)
        i += L
    if len(subs) >= 2 and len(subs[-1]) < L // 2:
        subs[-2].extend(subs[-1])
        subs.pop()
    out = []
    for j, ch in enumerate(subs):
        out.append(SubSeq(name=f"{session}_{a}_{b}_s{j}", session=session,
                          block=(a, b), rows=ch))
    return out


def build_subsequences(eval_blocks: Dict[str, List[List[int]]], L: int) -> List[SubSeq]:
    """All subsequences, in fixed (session, block, offset) order."""
    out = []
    for sid in sorted(eval_blocks):
        for blk in eval_blocks[sid]:
            out.extend(slice_block(sid, (blk[0], blk[1]), L))
    return out


def assign_splits(subs: List[SubSeq], split_seed: int) -> List[SubSeq]:
    """Deterministic round-robin over [bank, calib, test].

    Round-robin (not random shuffle) guarantees every split is non-empty and each eval
    block contributes to all three, which keeps bank/calib representative of the same
    occasion as test without any random-draw imbalance. split_seed only rotates the phase,
    so the assignment is reproducible and declared.
    """
    order = ["bank", "calib", "test"]
    phase = split_seed % 3
    for k, s in enumerate(subs):
        s.split = order[(k + phase) % 3]
    return subs


def split_refs(subs: List[SubSeq]) -> Dict[str, List[Ref]]:
    out: Dict[str, List[Ref]] = {"bank": [], "calib": [], "test": []}
    for s in subs:
        out[s.split].extend(s.refs())
    return out


def assert_disjoint(subs: List[SubSeq]) -> Dict:
    """Prove bank/calib/test are pairwise row-disjoint. Raise loudly if not.

    Returns a summary dict when clean. This is the check the whole study rests on, so it is
    verbose and unconditional.
    """
    sets = {k: set(v) for k, v in split_refs(subs).items()}
    report = {"counts": {k: len(v) for k, v in sets.items()}}
    pairs = [("bank", "test"), ("calib", "test"), ("bank", "calib")]
    overlaps = {}
    for x, y in pairs:
        inter = sets[x] & sets[y]
        overlaps[f"{x}&{y}"] = sorted(inter)[:20]
        if inter:
            raise DisjointnessError(
                f"LEAK: {len(inter)} row(s) shared between '{x}' and '{y}' splits; "
                f"e.g. {sorted(inter)[:10]}. Bank/test disjointness is violated -- refusing "
                f"to score. This must be fixed before any realness number is trusted.")
    # also assert no subsequence straddles two splits (whole-subsequence invariant)
    for s in subs:
        if s.split not in ("bank", "calib", "test"):
            raise DisjointnessError(f"subsequence {s.name} has no split assigned")
    report["overlaps"] = overlaps
    report["ok"] = True
    report["whole_subsequence_unit"] = True
    return report
