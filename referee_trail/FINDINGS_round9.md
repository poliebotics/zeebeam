Hoy.

## 1. Round-8 residue

| Item | Status | Evidence |
|---|---|---|
| Host-time attribution | RESOLVED | Section 6 and [ENVIRONMENT.md](<packet>/bundle/ENVIRONMENT.md:39): row 96 = 61.5 s; row 72 = 58.6 s. |
| Eight completed rounds | RESOLVED | Sections [1](<packet>/paper.md:96), [7.4](<packet>/paper.md:415), and [Appendix F](<packet>/paper.md:901) correctly cover rounds 1–8. |
| Training-seed correction | PARTIALLY | Section 8.1 is correct, but [ml/realness/README.md](<packet>/bundle/ml/realness/README.md:12) still claims “one forger seed,” and lines 27–28 quote the superseded manuscript wording. The correction says the training seed is unrecorded and `100000` is a checkpoint step. |
| Original ROC and correction companion | RESOLVED | [Section 8.1](<packet>/paper.md:523) and the README state that the original remains unmodified and the companion supersedes its sidecar for interpretation. |
| Generated-fakes reproducibility | RESOLVED | Section 8.1 and [README.md](<packet>/bundle/ml/realness/README.md:23) explicitly state that the recorded generating script is absent and the run cannot be reproduced from the supplied harness. |

| Finding | Status | Location |
|---|---|---|
| R8-N1 | RESOLVED | Section 6; `ENVIRONMENT.md`; execution logs |
| R8-N2 | PARTIALLY | Section 8.1 fixed; `ml/realness/README.md:12,27–28` stale |
| R8-N3 | RESOLVED | Sections 1 and 7.4; Appendix F |

DOI and licence remain explicitly pending principal decisions, not correctness defects.

## 2. New finding

None introduced since round 8. `diff -rq ../paper_round8_20260902/bundle bundle` printed differences only for `ENVIRONMENT.md`, `ml/realness/README.md`, and the corresponding `SHA256SUMS`; the manuscript diff contains only the declared edits and round-9 metadata.

Cold entry passed: 202/202 bundle hashes and 38/38 source hashes printed `OK`; the row-95 decoder printed `row_index: 95`, `row_count: 712`, `total_bytes: 1101`; timing grep printed:

```text
final_relation/execution_logs/row_072_execute_final_elf.log:43:host_elapsed_ms=58564
final_relation/execution_logs/row_096_execute_final_elf.log:41:host_elapsed_ms=61501
```

## 3. Numeric check

**PASS, Sections 6–8:** every value rechecked; row 96 = 61.501 s and row 72 = 58.564 s, with no failing value.

## 4. Verdict (a)

**Minor revision remains on correctness, evidence and internal closure solely because tonight’s edit must replace “one forger seed” and the stale quoted manuscript wording in `ml/realness/README.md`; the bundled evidence otherwise closes the round-8 residue.**

## 5. Verdict (b)

**Venue: weak reject for IEEE S&P and CCS remains my judgement, unchanged because no significance-bearing fact changed.**

— BOSUN ⚓
