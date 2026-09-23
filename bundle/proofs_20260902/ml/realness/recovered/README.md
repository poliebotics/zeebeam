---
version: 1.0
date: 2026-09-23
status: recovered-realness-records
author: Cathal Ryan Hynes (author of record); drafted with BOSUN, the project's automated research assistant
---

# Recovered realness records

These files were written on the rented box on 31 August 2026, copied to the project's backup of its shared filesystem on
1 September 2026, and recovered from that backup on 23 September 2026. Every file is published byte for byte, so the
scripts still name the box's paths (`/lambda/nfs/ZeeBeam/...`).

| file | what | SHA-256 |
|---|---|---|
| `score_fakes_class12.py` | scores the F-A v1 fakes against the real d2/v10 subsequences and writes `../REALNESS_ROC_generated_fa_v1_step100k.json` | `fdb8745de31430b562d62e573b235052c973cb3323abfe1ae20204b42f90fe43` |
| `SECURITY_DIAGNOSTIC_class3_replay_splice.json` | the class-3 record, nine runs, 6 real against 18 attacked subsequences each | `5f702bf75daa556e3e374f8d0694aec278d6da02ed00d37b7f3cdfb2e5d8dd81` |
| `gen_fakes_d2v10.py` | makes the 1,100 fakes, `C_fake = F(C_(r-2), E_(r-2), E_r)`, from the published F-A v1 step-100,000 forger | `d5e4fe614ba6938f9b128105006b69aac86cc8759bba863141c733807d23be91` |
| `MANIFEST_d2v10.jsonl` | one line per fake: session, row, source row, forger digest, SHA-256 of the fake | `3bba45a184c1eaf5783b530d61cb31a0d069817c240fd10d9365e3d3cda73f52` |
| `remosaic_fakes.py` | packs each fake back into a raw Bayer frame so the scorer reads it through its own loader | `4f4b3510d280d92024ef8fd510814b0f40b053d68bb2877805229fc06f83d3ad` |
| `REMOSAIC_MANIFEST.jsonl` | one line per packed frame: session, row, SHA-256 of the raw, SHA-256 of its source fake | `9ff188b538f8502883247ec75c8430425f5dea74819b60480c2d0cd8df4ad42d` |
| `v1_1/train_arm_a_ft.py`, `v1_1/phase_g/__init__.py`, `v1_1/phase_g/diffusion_diagnostic_model.py` | the three files of the frozen scorer tree that the harness imports; the model source is byte-identical to the Phase G model source the Truth Beam release publishes | `d3f650ae16dfc6710cfb4e74cc5152b042587761af130313c5eca825cea5fc06`, `01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b`, `f1241c1e4b7d042397d314207a45becd24066d9ed672ec759fbc14fd9e0c3beb` |
| `out_full/seed_20260823/`, `seed_20260901/`, `seed_20260902/` | the full run's per-seed outputs: `splits.json` (the bank, calibration and test subsequences, identical across seeds), `positives_scores.json` (the genuine scores) and `disjointness_report.json` | listed in the bundle's `SHA256SUMS` |
| `recompute_auroc.py` | recomputes every run's AUROC and errors from the two records' scores; standard library only | written 23 September 2026 |

## Recompute the AUROCs

```
python3 recompute_auroc.py
```

Twelve runs: three generated-fakes runs (6 real against 6 forged) and nine class-3 runs (6 real against 18 attacked).
All give AUROC 1.0, no attack accepted and no real rejected at the recorded thresholds, matching the records.

## Re-run the scoring

1. Fetch the scorer checkpoint, https://data.truthbeam.com/models/zeebeam_arm_a/step_00016000.pt, and check its
   SHA-256 against `d074775e2d9952d34c16baec134ddeff18df9ebf41170aa4c0f49e50c9c52532`.
2. Fetch the d2 and v10 frames and emission tiles the splits use (https://data.truthbeam.com/sessions/) and the
   forger, https://data.truthbeam.com/models/fa_v1_forger/f_a_v1_step_00100000.pt.
3. Point the paths in `../code/scorer_config.json`, `../code/RUNBOOK.md` and these scripts (`RD`, `SHADOW`, `OUT`,
   `SNAP`, `BASE`) at the local copies, and pass `v1_1/` as the scorer tree (`--v1_1-dir`).
4. Class 3: follow `../code/RUNBOOK.md`; it needs no fakes.
5. Generated fakes: `gen_fakes_d2v10.py`, then `remosaic_fakes.py`, then `score_fakes_class12.py`.

The fakes were made on a CUDA GPU under bf16 autocast, and byte equality across GPUs and precision paths is not
claimed. Compare regenerated fakes with `MANIFEST_d2v10.jsonl`. Where a digest differs, expect scores within bf16
noise of the record rather than bit-identical ones. The positive control in `gen_fakes_d2v10.py` regenerates an
August fake from an unpublished development frame; skip it. The 1,100 fakes and their packed frames, about 25 GiB,
are not published.

## Log

- 1.0 (2026-09-23, BOSUN) — the recovered files described, with the recompute and re-run paths.
