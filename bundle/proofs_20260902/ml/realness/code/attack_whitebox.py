#!/usr/bin/env python3
"""Attack class 4 -- white-box optimisation against the (public) scorer.

SECURITY DIAGNOSTIC, NOT a realness ROC (see LIMITATIONS.md). The scorer is public in the
release, so a white-box adversary can optimise a forgery to raise its score toward the
decision threshold under a bounded perturbation budget. This measures how easily the
PUBLISHED scorer is fooled, as a function of budget -- a security property, not realness.

Two execution modes, one interface:

  * diffusion backend (GPU, full-scale run): TRUE projected gradient ascent on the presented
    pixels through the frozen model. Objective = mean per-frame margin (a smooth surrogate
    for the median headline), maximised under an L-inf budget on the normalised [0,1] frame,
    started from a class-3 splice. Gradients flow through q_sample and the UNet.

  * fixture backend (CPU, no model): HARNESS VALIDATION. There is no differentiable
    pixel->score map without the model, so the budget sweep is simulated with the fixture's
    `relief` knob (a successful optimisation reduces the splice anomaly). This validates the
    budget sweep, threshold-crossing bookkeeping, seeds and labelling; it is NOT a real
    perturbation and is marked as such in the output.

Threshold: Youden-J on the CALIBRATION split (real vs white-box forgeries at the largest
budget), disjoint from test. Attack success on TEST reported per budget. Seeds reported.
"""
from __future__ import annotations

import sys, os
sys.dont_write_bytecode = True
os.environ.setdefault("PYTHONDONTWRITEBYTECODE", "1")

import argparse, json, random
from pathlib import Path
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from realness import pipeline as P
from realness import holdout as H
from realness import features as Ft
from realness import metrics as M
from realness.frozen import load_config, sha256_file
from scorer import make_backend

BUDGETS = [0.0, 2/255, 8/255, 16/255, 32/255]   # L-inf on normalised [0,1] frame (declared)
PGD_STEPS = 40
PGD_LR = 1/255


def base_splices(subs, split, forger_seed):
    """Start the white-box from class-3 reorder splices of `split` real frames only."""
    rng = random.Random(f"whitebox_base|{forger_seed}|{split}")
    out = []
    for s in P.subseqs_by_split(subs, split):
        rows = s.rows; n = len(rows)
        shift = 1 + rng.randrange(max(1, n - 1))
        decl = [(s.session, r) for r in rows]
        pres = [(s.session, rows[(i + shift) % n]) for i in range(n)]
        out.append(Ft.Sequence(name=f"{s.name}__wb", session=s.session, block=s.block,
                               frames=[Ft.PresentedFrame(pres[i], decl[i]) for i in range(n)],
                               kind="whitebox", meta={"shift": shift, "start": "class3_reorder"}))
    return out


# ---- fixture harness-validation: simulate optimisation via the relief knob ----
def apply_fixture_relief(backend, seqs, budget):
    backend.clear_relief()
    frac = min(1.0, budget / BUDGETS[-1]) if BUDGETS[-1] > 0 else 0.0
    for seq in seqs:
        for fr in seq.frames:
            backend.set_relief(fr.present, fr.declared, frac)


# ---- diffusion true PGD (GPU; executes only under --backend diffusion) ----
def pgd_optimise(backend, seq, budget, eval_seed, steps=PGD_STEPS, lr=PGD_LR):
    """Return a per-frame additive perturbation dict maximising mean margin under L-inf<=budget."""
    torch = backend._torch
    dev = backend.device
    T = backend.T
    perts = {}
    for fr in seq.frames:
        C0 = T.load_C(*fr.present).to(dev)
        delta = torch.zeros_like(C0, requires_grad=True)
        noise = backend._noise_for(fr.present[0], fr.present[1], eval_seed)
        Edecl = T.load_E(*fr.declared).to(dev)
        total = backend.rows_total(fr.declared[0])
        wrong_Es = []
        for o in B_OFFSETS:
            rr = fr.declared[1] + o
            if 0 <= rr < total:
                wrong_Es.append(T.load_E(fr.declared[0], rr).to(dev))
        for _ in range(steps):
            Cadv = (C0 + delta).clamp(0, 1)
            correct = backend._mse(Cadv, Edecl, noise)
            wrongs = [backend._mse(Cadv, We, noise) for We in wrong_Es]
            margin = (torch.stack(wrongs).mean() - correct) if wrongs else -correct
            loss = -margin                      # maximise margin
            g = torch.autograd.grad(loss, delta)[0]
            with torch.no_grad():
                delta -= lr * g.sign()
                delta.clamp_(-budget, budget)
        perts[fr.present] = delta.detach()
    return perts


B_OFFSETS = [-2, 2, -15, 15, 30]


def score_whitebox_at_budget(backend, subs, split, seqs, budget, eval_seed,
                             mu_B, sigma_B, bank_scored, weights):
    """Score white-box forgeries for `split` at a given budget under the current backend."""
    if backend.name == "fixture":
        apply_fixture_relief(backend, seqs, budget)
        rows = P.score_sequences(seqs, backend, eval_seed, mu_B, sigma_B, bank_scored, weights)
        backend.clear_relief()
        return rows
    # diffusion: true PGD, then score the perturbed frames via a patched loader
    torch = backend._torch
    rows = []
    for seq in seqs:
        perts = pgd_optimise(backend, seq, budget, eval_seed)
        orig_load = backend.T.load_C
        def patched(sid, r, _p=perts, _o=orig_load, _dev=backend.device):
            base = _o(sid, r)
            d = _p.get((sid, r))
            return base if d is None else (base.to(_dev) + d).clamp(0, 1).cpu()
        backend.T.load_C = patched
        try:
            rows += Ft.sequence_scores([seq], backend, eval_seed, mu_B, sigma_B)
        finally:
            backend.T.load_C = orig_load
    Ft.combine_auxiliary(rows, bank_scored, weights)
    return rows


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
    weights = cfg["aggregation"]["auxiliary_combined"]["weights"]
    mb = cfg["metrics"]["cluster_bootstrap"]
    sk = "primary_score"

    runs = []
    succ_by_budget = {b: [] for b in BUDGETS}
    for es in cfg["seeds"]["eval_seeds"]:
        mu_B, sigma_B = P.fit_bank_margins(subs, backend, es)
        bank_scored = P.bank_sequences_scored(subs, backend, es, mu_B, sigma_B)
        real_calib = P.score_sequences(P.genuine_sequences(subs, "calib"), backend, es,
                                       mu_B, sigma_B, bank_scored, weights)
        real_test = P.score_sequences(P.genuine_sequences(subs, "test"), backend, es,
                                      mu_B, sigma_B, bank_scored, weights)
        for fs in cfg["seeds"]["forger_seeds"]:
            calib_seqs = base_splices(subs, "calib", fs)
            test_seqs = base_splices(subs, "test", fs)
            # threshold: Youden-J on calib real vs white-box at the LARGEST budget
            fc_max = score_whitebox_at_budget(backend, subs, "calib", calib_seqs,
                                              BUDGETS[-1], es, mu_B, sigma_B, bank_scored, weights)
            thr, J = M.youden_threshold(
                np.array([r[sk] for r in real_calib], float),
                np.array([r[sk] for r in fc_max], float))
            budget_results = []
            for b in BUDGETS:
                ft = score_whitebox_at_budget(backend, subs, "test", test_seqs,
                                              b, es, mu_B, sigma_B, bank_scored, weights)
                rates = M.rates_at(thr, np.array([r[sk] for r in real_test], float),
                                   np.array([r[sk] for r in ft], float))
                roc = M.roc_with_ci(real_test, ft, sk, n_boot=mb["n_boot"],
                                    alpha=mb["alpha"], seed=fs)
                succ_by_budget[b].append(rates["FPR"])
                budget_results.append({
                    "budget_Linf": b,
                    "SECURITY_DIAGNOSTIC_attack_success_rate_test": rates["FPR"],
                    "FNR_test_real_rejected": rates["FNR"],
                    "security_diagnostic_auroc_real_vs_attack": roc,
                    "forgery_scores_test": [r[sk] for r in ft],
                })
            runs.append({"eval_seed": es, "forger_seed": fs, "calib_threshold": thr,
                         "youden_J": J, "real_scores_test": [r[sk] for r in real_test],
                         "budget_sweep": budget_results})

    diag = {
        "LABEL": "SECURITY_DIAGNOSTIC",
        "NOT_A_REALNESS_ROC": True,
        "attack_class": "class4_whitebox",
        "mode": ("harness_validation_fixture_relief" if backend.name == "fixture"
                 else "true_pgd_on_pixels"),
        "explanation": ("white-box optimisation against the public scorer, reported as an "
                        "attack-success-vs-budget curve. NOT a realness ROC and NOT pooled "
                        "with any other class. On the fixture backend this validates the "
                        "harness only (relief knob), not a real perturbation."),
        "budgets_Linf": BUDGETS, "pgd_steps": PGD_STEPS, "pgd_lr": PGD_LR,
        "attack_success_rate_test_by_budget": {
            str(b): {"mean": float(np.nanmean(v)) if v else float("nan"),
                     "values": v} for b, v in succ_by_budget.items()},
        "config_sha256": cfg["_config_sha256"],
        "attack_py_sha256": sha256_file(Path(__file__)),
        "backend": backend.name,
        "runs": runs,
    }
    fn = out / "SECURITY_DIAGNOSTIC_class4_whitebox.json"
    fn.write_text(json.dumps(diag, indent=2))
    print(f"[class4 whitebox] SECURITY DIAGNOSTIC ({diag['mode']}) attack success by budget:")
    for b in BUDGETS:
        vals = succ_by_budget[b]
        print(f"    Linf={b:.4f}: mean={np.nanmean(vals):.3f}" if vals else f"    Linf={b}: --")
    print(f"wrote {fn}")


if __name__ == "__main__":
    main()
