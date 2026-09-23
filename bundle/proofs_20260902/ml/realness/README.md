---
version: 1.4
date: 2026-09-23
status: realness-records-what-exists
author: Cathal Ryan Hynes (author of record); drafted with BOSUN, the project's automated research assistant
---

# Realness diagnostics: which records exist

| claim in Section 8.1 | record here | status |
|---|---|---|
| generated-fakes class (F-A v1 forger, step-100,000 checkpoint): 0 fakes passing, 0 reals rejected, AUROC 1.0, three evaluation seeds, one checkpoint run (its training seed is unrecorded; the configured seeds 11, 22, 33 were never trained), six real against six forged per seed | `REALNESS_ROC_generated_fa_v1_step100k.json` (runs, per-sequence scores, thresholds, checkpoint/config/script digests) and its generating script `recovered/score_fakes_class12.py` | archived and re-runnable (below); its sidecar text is self-contradictory (LABEL says generated-realness while an inherited `explanation` field says generator-free), as the note records |
| replay-and-splice diagnostic (class 3), six against eighteen, nine runs: every run 0 attacks passing, 0 reals rejected, AUROC 1.0 | `recovered/SECURITY_DIAGNOSTIC_class3_replay_splice.json` | archived 23 September 2026 (recovered, below); a security diagnostic, not a realness ROC |
| adaptive white-box attack (class 4) | none | NOT run to completion (exhausted a 40 GB GPU twice); no result is claimed |
| black-box l2 diagnostic, no margin at eps = 16 after 1,000 queries | none | NOT archived: its outputs stayed on the rented box, which was terminated; reported only in the project note |
| audited note | `zeebeam_realness_results_20260901.md` v2.0 | the record the manuscript cites |
| code and configuration | `code/` (`attack_replay_splice.py`, `attack_whitebox.py`, `run_fixture.py`, `scorer.py`, `make_hashes.py`, the `realness/` package, `scorer_config.json`, `LIMITATIONS.md`, `README.md`, `RUNBOOK.md`) | the staged harness; `python3 run_fixture.py --help` runs from `code/` |

## Recovered records

The revision of 2 September 2026 said that the generated-fakes run's generating script and the class-3 record
stayed on the rented box and were lost when it was terminated. Both had been copied on 1 September 2026 to the
project's backup of that box's shared filesystem, and were found there on 23 September 2026. They are in
`recovered/` byte for byte, with the files needed to re-run them (`recovered/README.md`).

- `recovered/score_fakes_class12.py` has SHA-256 `fdb8745de31430b562d62e573b235052c973cb3323abfe1ae20204b42f90fe43`,
  the `script_sha256` the generated-fakes record carries. The copy of that run's output kept beside it in the backup
  is byte-identical to `REALNESS_ROC_generated_fa_v1_step100k.json` here.
- `recovered/SECURITY_DIAGNOSTIC_class3_replay_splice.json` is the class-3 record: nine runs, three evaluation seeds
  by three attack seeds, 6 real against 18 attacked subsequences per run. Its `attack_py_sha256` is the digest of
  `code/attack_replay_splice.py`; its `config_sha256` is that of the configuration as run (next section). Its figures
  are the ones the audited note gives.

## Provenance notes on the generated-fakes record

- `script_sha256` matches `recovered/score_fakes_class12.py`.
- `config_sha256` (`67d4bef3…`, in both records) is the digest of the configuration as run. The published
  `code/scorer_config.json` (`11d74ae8…`) differs from it in one line only, its `author` field, where the rented box's
  public address was redacted for publication; every scoring parameter is identical, and a re-run with the published
  file records `11d74ae8…` instead. Revisions 1.1 to 1.3 said the two digests matched; they do not, for that reason.
- The record's `forger_seed` field holds `100000`, which is the checkpoint step of the one published
  F-A v1 forger that was run. `scorer_config.json` lists three planned forger seeds (11, 22, 33); none of
  those forgers was trained. The manuscript states this as: one checkpoint run was completed where the preregistration asked for
  three forger seeds; its training seed is unrecorded, and the configured seeds were never trained.
- The record's sidecar text is self-contradictory (`LABEL` says generated-realness, an inherited
  `explanation` field says generator-free). The original is kept unmodified;
  `REALNESS_ROC_generated_fa_v1_step100k.CORRECTION.json` states the correct reading. The numbers are
  unaffected: three evaluation seeds, six real against six forged each, zero errors at the recorded
  thresholds, all 36 pairwise comparisons per seed correctly ordered.

## Recompute and re-run

In seconds, with no model: `python3 recovered/recompute_auroc.py` reads the per-sequence scores in both records
and recomputes every run's AUROC and its errors at the recorded threshold. All twelve runs give AUROC 1.0 and
zero errors, and each matches its record.

To regenerate the scores, the scorer is the ARM-A step-16,000 checkpoint named in `code/scorer_config.json`
(477,636,875 bytes, SHA-256 `d074775e2d9952d34c16baec134ddeff18df9ebf41170aa4c0f49e50c9c52532`), published on
23 September 2026:

https://data.truthbeam.com/models/zeebeam_arm_a/step_00016000.pt

The other inputs are public too: the Truth Beam d2 and v10 frames (https://data.truthbeam.com/sessions/) and the
F-A v1 step-100,000 forger:

https://data.truthbeam.com/models/fa_v1_forger/f_a_v1_step_00100000.pt

`recovered/README.md` gives the order of the steps. The unarchived figures above stay labelled as such, and the manuscript rests no claim on them.

## Log

- 1.4 (2026-09-23, BOSUN) — the generating script and the class-3 record, recorded in 1.1 as lost with the rented
  box, recovered from the project's backup of its filesystem into `recovered/`; the class-3 row and the provenance
  notes restated, including the configuration digest, which differs from the published file by the redacted author
  line; the recompute and re-run paths added, with the scorer checkpoint now published.

- 1.3 (2026-09-09, BOSUN) — authorship line, 9 September 2026.

- 1.2 (2026-09-02, BOSUN) — seed wording aligned with the correction companion (round-9 residue).

- 1.1 (2026-09-02, BOSUN) — harness package bundled; provenance notes and a correction companion for the record's sidecar.

- 1.0 (2026-09-02, BOSUN) — written after Sol's round-6 request for the records behind Section 8.1.
