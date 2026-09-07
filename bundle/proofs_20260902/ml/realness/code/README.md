---
version: 1.0
date: 2026-08-30
status: drafted-cpu-validated
author: BOSUN (remote, box [public address redacted])
---

# ZeeBeam realness study — machinery

Implements the frozen preregistration
`docs/research/zeebeam_realness_study_prereg_20260828.md` (v1.0). It measures a DIFFERENT
quantity from the existing AUROC~1.0: that number is CONDITIONING discrimination
(right-vs-wrong emission on a real frame) and supplies no realness FPR/FNR. This study asks
whether a frozen neural-physical discriminator separates genuine time-chained captures from
adaptive forgeries, and reports the error rate as the strength of the evidence.

This tree is DRAFTING plus CPU/fixture validation. The full-scale scoring runs later on a
freed GPU (see `RUNBOOK.md`). No large GPU job is launched from here.

## What is built, and what is deliberately not

Built and validated: the frozen scorer (`scorer.py`), the whole-subsequence holdout with a
loud disjointness proof (`realness/holdout.py`), and the two generator-free negative
classes — class 3 replay/splice (`attack_replay_splice.py`) and class 4 white-box
(`attack_whitebox.py`).

NOT built, by the auditor's instruction: classes 1 (naive per-frame synthesis) and 2
(temporally-coherent generative sequence). Both need a photorealistic conditional generator
(the pix2pixHD lineage) that lives on an unreachable machine. They are not fabricated. See
`LIMITATIONS.md`.

## Hard labelling rule (read before quoting any number)

Classes 3 and 4 are reported as **SECURITY DIAGNOSTICS / attack-success distributions**,
never as a "realness ROC" and never pooled into one number. The label is baked into the
output filenames (`SECURITY_DIAGNOSTIC_*.json`), the JSON (`"LABEL": "SECURITY_DIAGNOSTIC"`,
`"NOT_A_REALNESS_ROC": true`), and this README. Pooling only two of four attack classes and
calling it "realness" would be misleading; the machinery refuses to do it.

## How each declared feature is instantiated

The frozen feature vector maps onto the validated Phase-G conditional-diffusion machinery
(`published_protocol_eval.py` + `phase_g/diffusion_diagnostic_model.py`, both READ-ONLY).
The single per-frame primitive is the conditional eps-MSE of the pinned ARM-A step-16000
model (sha `d074775e…`): `mse = mean((model(q_sample(C,t,noise), E, t) - noise)^2)` at
`t=150`, `K=1`, bf16, offsets `[-2,+2,-15,+15,+30]`, boundary-absent never imputed —
byte-identical scoring to the published protocol, reused not reinvented.

Per-frame consistency signal: `margin_t = mean(available wrong-offset mse) - correct_mse`
(the published `delta_wrong_mean` per row; higher = the frame is more consistent with its
own declared emission than with wrong ones).

- **Feature (1) optical/sensor consistency under the declared emission** := `margin_t`.
  Pooled to the sequence as `median_t(margin_t)`. This IS the declared headline "one number
  per sequence"; the ROC, Youden-J threshold, FPR and FNR are computed on it.
  *Why defensible:* feature (1) asks whether the frame is optically consistent with
  `E_t = render(XOF(S_t))`; the frozen conditional scorer measures exactly that, and its
  correct-vs-wrong margin is the validated instrument for it.

- **Feature (2) temporal coherence** := `median_t(-|margin_t - margin_{t-1}|)` over
  consecutive presented rows.
  *Why defensible:* temporal coherence of feature (1) is literally the frame-to-frame
  stability of the consistency signal along the chain; a spliced/replayed sequence injects
  a discontinuity in that signal, which this penalises. Length-1 sequences yield 0.

- **Feature (3) known-real bank cross-correlation** :=
  `median_t(exp(-0.5*((margin_t - mu_B)/sigma_B)^2))`, with `(mu_B, sigma_B)` fitted over
  KNOWN-REAL bank rows disjoint from calibration and test.
  *Why defensible:* for a scalar neural-physical statistic, "cross-correlation against a
  known-real bank" is operationally a distributional match; the bank-standardised Gaussian
  kernel is a bounded, monotone similarity that is high inside the known-real cloud and low
  outside it. Limitation: it is a match on a low-dimensional statistic, not a full spatial
  feature-map cross-correlation (that needs the richer R1 corpus). Stated in `LIMITATIONS.md`.

Aggregation. The headline sequence score is the declared pooling of feature (1). The full
three-feature vector is also combined into a declared AUXILIARY z-score
(`combined_aux`, frozen weights `w=(1.0,0.5,0.5)`), standardised by BANK statistics only so
test data never informs the combination. The auxiliary is reported, not the headline.

All weights and constants live in `config/scorer_config.json`, hashed into
`HASHES.sha256`. Editing any value breaks the freeze.

## Layout

```
realness_20260830/
  scorer.py                     # deliverable 1: frozen discriminator + genuine scoring
  attack_replay_splice.py       # deliverable 2a: class 3 (SECURITY DIAGNOSTIC)
  attack_whitebox.py            # deliverable 2b: class 4 (SECURITY DIAGNOSTIC)
  run_fixture.py                # deliverable 4: CPU end-to-end + leak-fires proof
  make_hashes.py                # writes HASHES.sha256 over all deliverables
  config/scorer_config.json     # frozen weights/constants (hashed)
  realness/                     # backend-agnostic library
    backends.py                 #   FixtureBackend (CPU) + DiffusionBackend (frozen model)
    features.py                 #   the three features + pooling
    holdout.py                  # deliverable 3: whole-subsequence blocking + disjointness
    metrics.py                  #   sequence AUROC, block-clustered bootstrap, Youden-J
    pipeline.py                 #   orchestration
    frozen.py                   #   config load + hashing
  out/fixture/                  # fixture validation artifacts (committed as evidence)
  RUNBOOK.md  LIMITATIONS.md  README.md  HASHES.sha256
```

## Fixture validation (already run, CPU only)

`out/fixture/FIXTURE_VALIDATION.json` records: pipeline runs end to end; the disjointness
guard FIRES on a deliberately-leaked fixture; genuine sequences outscore splices; white-box
success rises with budget. Reproduce:

```
cd /lambda/nfs/ZeeBeam/experiments/realness_20260830
CUDA_VISIBLE_DEVICES="" PYTHONDONTWRITEBYTECODE=1 python3 run_fixture.py --out-dir out/fixture
```

## Log
- 1.0, 2026-08-30, BOSUN. Built and CPU-validated on box [public address redacted]. Classes 1,2
  deliberately not built (no generator). Full-scale GPU scoring deferred to RUNBOOK.md.
