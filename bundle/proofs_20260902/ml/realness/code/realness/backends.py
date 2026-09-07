#!/usr/bin/env python3
"""Feature backends for the ZeeBeam realness scorer.

The expensive primitive of feature (1) is, for a PRESENTED frame C (a real capture)
scored against a DECLARED emission E and its offset arms:

    arm_scores(present=(sid, row), declared=(sid, row), eval_seed)
        -> {"correct": mse, "wrong_-2": mse, "wrong_+2": mse, ...}

where mse is the frozen conditional eps-MSE of the Phase-G ARM-A step-16000 model.

Two backends implement the SAME interface:

  * DiffusionBackend  -- the real, frozen neural-physical scorer. Loads the pinned
    checkpoint and reuses the frozen v1_1 loaders/preprocessing byte-for-byte. Runs on
    CUDA. USED ONLY IN THE FULL-SCALE RUN (later, when GPUs free up). Importing the v1_1
    tree is done lazily and with bytecode writing disabled so no __pycache__/.pyc is ever
    created under the byte-frozen v1_1 directory.

  * FixtureBackend    -- a deterministic CPU synthetic. No model, no GPU, no v1_1 import.
    Produces per-frame arm scores with the right STRUCTURE (correct arm lower than wrong
    arms for genuine frames; a controllable anomaly for injected foreign frames) so the
    entire statistical pipeline -- holdout, disjointness, pooling, ROC, threshold, attacks
    -- can be validated end to end on CPU without touching a GPU or the frozen model.

The scorer, holdout, metrics and attack code are backend-agnostic: they consume arm
scores, never the model.
"""
from __future__ import annotations

# Hard guard: never write bytecode anywhere (esp. under the byte-frozen v1_1 tree).
import sys as _sys
_sys.dont_write_bytecode = True
import os as _os
_os.environ.setdefault("PYTHONDONTWRITEBYTECODE", "1")

import hashlib
from pathlib import Path
from typing import Dict, Tuple

import numpy as np

Ref = Tuple[str, int]  # (session_id, row)

OFFSETS = [-2, 2, -15, 15, 30]


def _mix_seed(*parts) -> int:
    """Deterministic 63-bit seed from arbitrary parts (stable across processes)."""
    h = hashlib.sha256("|".join(str(p) for p in parts).encode()).digest()
    return int.from_bytes(h[:8], "big") & ((1 << 63) - 1)


class ArmBackend:
    """Interface. Subclasses implement `arm_scores` and expose `rows_total(sid)`."""

    name = "abstract"

    def rows_total(self, sid: str) -> int:
        raise NotImplementedError

    def arm_scores(self, present: Ref, declared: Ref, eval_seed: int) -> Dict[str, float]:
        raise NotImplementedError

    # ---- optional white-box hook (differentiable). Real backend overrides. ----
    def supports_whitebox(self) -> bool:
        return False


# --------------------------------------------------------------------------------------
# Fixture backend: deterministic CPU synthetic, no model, no GPU, no v1_1 import.
# --------------------------------------------------------------------------------------
class FixtureBackend(ArmBackend):
    """Synthetic per-frame arm scores.

    Model of a genuine frame: correct-arm eps-MSE is drawn around a low base; each wrong
    arm is drawn around a higher base whose gap grows with |offset| (the real model's
    behaviour). A per-frame `anomaly` in [0,1] linearly collapses the correct/wrong gap and
    lifts the correct-arm MSE, which is how we synthesise a foreign/spliced frame: its
    consistency statistic is pushed OUTSIDE the known-real cloud. Everything is a pure
    function of (sid, row, declared_row, eval_seed, anomaly) so results are reproducible.
    """

    name = "fixture"

    def __init__(self, rows: Dict[str, int], base_correct=1.00e-3, base_gap=4.0e-3,
                 noise_sd=1.5e-4, anomaly: Dict[Ref, float] | None = None):
        self._rows = dict(rows)
        self.base_correct = base_correct
        self.base_gap = base_gap
        self.noise_sd = noise_sd
        self.anomaly = dict(anomaly or {})
        # white-box HARNESS-VALIDATION knob: maps (present, declared) -> relief in [0,1]
        # that scales the auto splice-anomaly DOWN, simulating a successful pixel
        # optimisation against the scorer. The real pixel PGD runs on the diffusion backend.
        self.relief: Dict[Tuple[Ref, Ref], float] = {}

    def rows_total(self, sid: str) -> int:
        return self._rows[sid]

    def set_anomaly(self, ref: Ref, value: float):
        self.anomaly[ref] = float(value)

    def set_relief(self, present: Ref, declared: Ref, value: float):
        self.relief[(present, declared)] = float(max(0.0, min(1.0, value)))

    def clear_relief(self):
        self.relief = {}

    def arm_scores(self, present: Ref, declared: Ref, eval_seed: int) -> Dict[str, float]:
        sid_p, r_p = present
        sid_d, r_d = declared
        total = self._rows[sid_d]
        rng = np.random.RandomState(_mix_seed(eval_seed, "fix", sid_p, r_p, sid_d, r_d) % (2**32))
        # Anomaly rises if the presented frame is flagged, OR if the presented frame does
        # not match its declared position (a splice: present row != declared row, or a
        # cross-session presentation).
        a = self.anomaly.get(present, 0.0)
        if (sid_p != sid_d) or (r_p != r_d):
            a = max(a, min(1.0, 0.15 + 0.02 * abs(r_p - r_d)))
        relief = self.relief.get((present, declared), 0.0)
        a = a * (1.0 - relief)   # white-box harness knob: optimisation reduces the anomaly
        out = {}
        # correct arm: base, lifted toward the wrong-arm level as anomaly -> 1
        correct = self.base_correct + a * self.base_gap + rng.normal(0, self.noise_sd)
        out["correct"] = float(max(1e-6, correct))
        for o in OFFSETS:
            rr = r_d + o
            if rr < 0 or rr >= total:
                continue  # arm absent, never imputed (matches frozen boundary policy)
            gap = self.base_gap * (0.6 + 0.4 * min(1.0, abs(o) / 30.0)) * (1.0 - a)
            wr = self.base_correct + gap + rng.normal(0, self.noise_sd)
            out[f"wrong_{o:+d}"] = float(max(1e-6, wr))
        return out


# --------------------------------------------------------------------------------------
# Real backend: the frozen Phase-G ARM-A step-16000 scorer. GPU-only (full-scale run).
# --------------------------------------------------------------------------------------
class DiffusionBackend(ArmBackend):
    """Frozen neural-physical scorer. Reuses the frozen v1_1 loaders + model source.

    The v1_1 import is lazy and bytecode-free. This class is exercised only by the
    full-scale GPU run in RUNBOOK.md; the CPU fixture never constructs it.
    """

    name = "diffusion_arm_a_step16000"

    def __init__(self, v1_1_dir: str, checkpoint: str, checkpoint_sha256: str,
                 device: str = "cuda", t: int = 150):
        _sys.dont_write_bytecode = True
        import torch

        self.t = int(t)
        self.device = device
        v = Path(v1_1_dir)
        if str(v) not in _sys.path:
            _sys.path.insert(0, str(v))
        # Lazy imports of the frozen, READ-ONLY tree. dont_write_bytecode guards __pycache__.
        import train_arm_a_ft as T  # noqa: E402
        from phase_g.diffusion_diagnostic_model import (  # noqa: E402
            DiffusionDiagnosticUNet, build_diffusion_constants, q_sample)
        self.T = T
        self.q_sample = q_sample

        # verify pinned checkpoint identity before trusting it
        got = _sha256(Path(checkpoint))
        if got != checkpoint_sha256:
            raise SystemExit(f"FAIL: checkpoint sha {got} != frozen {checkpoint_sha256}")
        T.resolve_raw_layout()

        ck = torch.load(checkpoint, map_location="cpu", weights_only=False)
        margs = dict(ck["args"])
        # student-width honouring, identical to published_protocol_eval
        _sb = margs.get("student_base_ch") or 0
        _sm = margs.get("student_mults") or ""
        if margs.get("mults"):
            mults = tuple(margs["mults"])
        elif _sm:
            mults = tuple(int(x) for x in str(_sm).split(","))
        else:
            mults = (1, 2, 4, 4)
        if not margs.get("base_ch") and _sb:
            margs["base_ch"] = int(_sb)
        model = DiffusionDiagnosticUNet(
            in_ch=4, base_ch=margs.get("base_ch", 96), channel_mults=mults,
            attn_at=tuple(i == len(mults) - 1 for i in range(len(mults))),
            cond_drop_prob=0.2, hint_in_ch=11)
        model.load_state_dict(ck["model"], strict=True)
        self.model = model.to(device).eval()
        self.dc = build_diffusion_constants(1000, device, torch.float32)
        self._torch = torch

    def rows_total(self, sid: str) -> int:
        return int(self.T.SESSIONS[sid]["rows_total"])

    def _noise_for(self, sid: str, row: int, eval_seed: int):
        torch = self._torch
        g = torch.Generator(device=self.device)
        g.manual_seed(_mix_seed(eval_seed, "noise", sid, row))
        return torch.randn(1, 4, self.T.TH, self.T.TW, device=self.device, generator=g)

    def supports_whitebox(self) -> bool:
        return True

    def _mse(self, C, E, noise):
        """Differentiable eps-MSE for presented C (float tensor 4xHxW), declared E (3xHxW)."""
        torch = self._torch
        tt = torch.full((1,), self.t, device=self.device, dtype=torch.long)
        Ct = self.q_sample(C.float().unsqueeze(0), tt, self.dc, noise)
        with torch.autocast("cuda", dtype=torch.bfloat16):
            e = self.model(Ct.to(torch.bfloat16), E.unsqueeze(0).to(torch.bfloat16), tt)
        return (e.float() - noise).pow(2).mean()

    def arm_scores(self, present: Ref, declared: Ref, eval_seed: int) -> Dict[str, float]:
        torch = self._torch
        sid_p, r_p = present
        sid_d, r_d = declared
        total = self.rows_total(sid_d)
        with torch.no_grad():
            C = self.T.load_C(sid_p, r_p).to(self.device)
            noise = self._noise_for(sid_p, r_p, eval_seed)
            out = {}
            E = self.T.load_E(sid_d, r_d).to(self.device)
            out["correct"] = float(self._mse(C, E, noise))
            for o in OFFSETS:
                rr = r_d + o
                if rr < 0 or rr >= total:
                    continue
                Eo = self.T.load_E(sid_d, rr).to(self.device)
                out[f"wrong_{o:+d}"] = float(self._mse(C, Eo, noise))
        return out


def _sha256(p: Path) -> str:
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for c in iter(lambda: f.read(1 << 22), b""):
            h.update(c)
    return h.hexdigest()
