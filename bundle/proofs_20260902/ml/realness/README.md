---
version: 1.3
date: 2026-09-09
status: realness-records-what-exists
author: Cathal Ryan Hynes (author of record); drafted with BOSUN, the project's automated research assistant
---

# Realness diagnostics: which records exist

| claim in Section 8.1 | record here | status |
|---|---|---|
| generated-fakes class (F-A v1 forger, step-100,000 checkpoint): 0 fakes passing, 0 reals rejected, AUROC 1.0, three evaluation seeds, one checkpoint run (its training seed is unrecorded; the configured seeds 11, 22, 33 were never trained), six real against six forged per seed | `REALNESS_ROC_generated_fa_v1_step100k.json` (runs, thresholds, checkpoint/config/script digests) | archived; its sidecar text is self-contradictory (LABEL says generated-realness while an inherited `explanation` field says generator-free), as the note records |
| replay-and-splice diagnostic (class 3), six against eighteen, nine runs | none | NOT archived: the run outputs (`out/full/SECURITY_DIAGNOSTIC_class3_replay_splice.json`) stayed on the rented box, which was terminated |
| adaptive white-box attack (class 4) | none | NOT run to completion (exhausted a 40 GB GPU twice); no result is claimed |
| black-box l2 diagnostic, no margin at eps = 16 after 1,000 queries | none | NOT archived: same box; reported only in the project note |
| audited note | `zeebeam_realness_results_20260901.md` v2.0 | the record the manuscript cites |
| code and configuration | `code/` (`attack_replay_splice.py`, `attack_whitebox.py`, `run_fixture.py`, `scorer.py`, `make_hashes.py`, the `realness/` package, `scorer_config.json`, `LIMITATIONS.md`, `README.md`, `RUNBOOK.md`) | the staged harness; `python3 run_fixture.py --help` runs from `code/` |

## Provenance notes on the generated-fakes record

- `script_sha256` in the record (`fdb8745d…`) matches no file on the development machine: the generating
  script's exact version stayed on the rented box. `config_sha256` matches `code/scorer_config.json`.
  **Consequence: the generated-fakes run cannot be reproduced from this bundle.** The harness here
  imports and runs, but it is not byte-identical to the script that produced the record.
- The record's `forger_seed` field holds `100000`, which is the checkpoint step of the one published
  F-A v1 forger that was run. `scorer_config.json` lists three planned forger seeds (11, 22, 33); none of
  those forgers was trained. The manuscript states this as: one checkpoint run was completed where the preregistration asked for
  three forger seeds; its training seed is unrecorded, and the configured seeds were never trained.
- The record's sidecar text is self-contradictory (`LABEL` says generated-realness, an inherited
  `explanation` field says generator-free). The original is kept unmodified;
  `REALNESS_ROC_generated_fa_v1_step100k.CORRECTION.json` states the correct reading. The numbers are
  unaffected: three evaluation seeds, six real against six forged each, zero errors at the recorded
  thresholds, all 36 pairwise comparisons per seed correctly ordered.

The manuscript labels the unarchived figures as such and rests no claim on them.

## Log

- 1.3 (2026-09-09, BOSUN) — authorship line, 9 September 2026.

- 1.2 (2026-09-02, BOSUN) — seed wording aligned with the correction companion (round-9 residue).

- 1.1 (2026-09-02, BOSUN) — harness package bundled; provenance notes and a correction companion for the record's sidecar.

- 1.0 (2026-09-02, BOSUN) — written after Sol's round-6 request for the records behind Section 8.1.
