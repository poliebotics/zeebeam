Hoy.

# 1. Round-7 disposition

| Item | Status | Evidence and residue |
|---|---|---|
| Build workflow | RESOLVED | Fixed read-only pins, size/SHA-256/vkey checks and driver digest are in [build_reproducible.sh](<packet>/bundle/source/build_reproducible.sh:8); the transcript begins with the matching digest and ends `BUILD_REPRODUCED_OK`, `exit=0` [transcript](<packet>/bundle/final_relation/build_transcript_halo_20260902.txt:1); the old driver is absent and [source/README.md](<packet>/bundle/source/README.md:20) names only the new driver; Section 12 correctly distinguishes development-machine transcript from ceremony-2 fresh-machine evidence. |
| Ceremony-1 row-72 RSS | RESOLVED | Section 7.1 now gives 114,160 KiB = 111.5 MiB, matching the manifest. |
| Realness evidence | PARTIALLY | The `realness/` modules are present and `run_fixture.py --help` succeeds; the configuration digest matches and the correction companion explains the bad fields, but no bundled file matches the record’s `script_sha256=fdb8745d…`, so the actual generated-fakes run remains irreproducible from this bundle. |
| Unarchived replay/splice and ℓ2 figures | RESOLVED | Exact figures are gone from Section 8.1; it says the runs are unarchived and makes no quantitative claim from them [Section 8.1](<packet>/paper.md:512). |
| Bootstrap wording | RESOLVED | The interval is correctly described as degenerate and non-informative [Section 8.1](<packet>/paper.md:509). |
| Audit metadata | PARTIALLY | The requested five-to-six edit was made, but v2.8 has harvested round 7, so seven reports are now complete; Sections 1 and 7.4 still say six, while Appendix F names only rounds 1–5. |
| DOI and licence | OPEN | Both remain absent and are explicitly pending the principal’s decisions [Section 12](<packet>/paper.md:783). |

| Finding | Status | Residue |
|---|---|---|
| R7-N1 build driver | RESOLVED | Fixed pins, correct README, historical driver removed, provenance labels corrected. |
| R7-N2 RSS | RESOLVED | 111.5 MiB, 114,160 KiB. |
| R7-N3 realness harness | PARTIALLY | Imports repaired; exact generating script still absent; seed wording remains wrong. |
| R7-N4 referee count | PARTIALLY | Six was inserted, but the current completed count is seven and Appendix F is incomplete. |
| R7-N5 bootstrap | RESOLVED | Degeneracy and lack of population uncertainty are stated. |

# 2. New findings and command record

Cold-entry commands:

```text
$ cd bundle && sha256sum -c SHA256SUMS
202/202 listed paths printed: OK
exit=0

$ cd source/rust && sha256sum -c ../../final_relation/SOURCE_TREE_SHA256SUMS
38/38 listed paths printed: OK
exit=0
```

The decoder printed a full JSON object whose decisive fields were:

```text
total_bytes: 1101
sha256: 777bbf1f5b6197bd82fa9c9c041413efa5c647de7c7a627aa92b3d328adcb26a
row_index: 96
row_count: 712
score_numerator: 56835791
pose.verdict: 2
beacon.verified: 1
previous_drand_round: 31521620
row_drand_round: 31521620
trapdoor_flag: 1
```

The harness command printed:

```text
$ cd ml/realness/code && python3 run_fixture.py --help
usage: run_fixture.py [-h] [--out-dir OUT_DIR]

options:
  -h, --help         show this help message and exit
  --out-dir OUT_DIR
```

## R8-N1: Section 6 assigns row 72’s time to row 96

Exact command:

```text
$ rg -n 'host_elapsed_ms' bundle/final_relation/execution_logs
bundle/final_relation/execution_logs/row_072_execute_final_elf.log:43:host_elapsed_ms=58564
bundle/final_relation/execution_logs/row_096_execute_final_elf.log:41:host_elapsed_ms=61501
```

Yet [Section 6](<packet>/paper.md:303) and [ENVIRONMENT.md](<packet>/bundle/ENVIRONMENT.md:39) call 58.6 s and 61.5 s two row-96 runs. There is one archived run per row: row 96 is 61.501 s; row 72 is 58.564 s.

## R8-N2: the forger-seed statement contradicts the correction

Exact output:

```text
paper.md:511: one forger training seed was completed...
CORRECTION.json:7: 100000 is the checkpoint step ... not a training seed;
                   the configured seeds 11, 22, 33 were never trained
scorer_config.json:72: "forger_seeds": [11, 22, 33]
script_digest_matches= []
```

Replace “one forger training seed was completed” with “one checkpoint run was completed; its training seed is unrecorded, and configured seeds 11, 22, and 33 were never trained.” The [correction companion](<packet>/bundle/ml/realness/REALNESS_ROC_generated_fa_v1_step100k.CORRECTION.json:5) is clear; the manuscript is not.

Section 8.1 also says the record “is not cited until its sidecar is corrected” although the correction companion already exists and the paragraph uses the record’s numbers [paper.md](<packet>/paper.md:520). State that the original is retained and superseded for interpretation by the companion.

## R8-N3: audit metadata is stale again

Exact command output:

```text
$ find . -maxdepth 1 -name 'FINDINGS_round*.md'
FINDINGS_round1.md
...
FINDINGS_round7.md

paper.md:96: six manuscript referee rounds
paper.md:415: has refereed this manuscript six times
paper.md:421: the six referee reports
```

Seven rounds precede round 8. Appendix F lists only rounds 1–5 [paper.md](<packet>/paper.md:899); add rounds 6 and 7.

# 3. Numeric check of Sections 6–8

| Claim | Independent result |
|---|---|
| Row-96 execution | 4,149,712,293 instructions; 269,904 syscalls; 61.501 s |
| Row-72 execution | 4,148,971,330 instructions; 269,904 syscalls; 58.564 s |
| Row-96 five principal shares | 51.596851%, 16.934086%, 10.146374%, 8.311808%, 8.169183% |
| “Other” arithmetic | 27,764,103 + 65,149 + 4,137 + 13,780 = 27,847,169 |
| Section-6 table | Sum 4,149,712,293; delta 0 |
| Ceremony manifests | Setup 21.356/21.401/21.170/21.957 s; prove 850.972/769.575/857.424/749.813 s; verification 0.366/0.505/0.365/0.363 s |
| RSS | 114,160; 114,260; 113,760; 114,240 KiB, all manuscript values correct |
| Proof sizes | Ceremony 1 framed 2,780/2,779 B; ceremony 2 both 2,795 B; all raw proofs 356 B; statements 1,085/1,101 B |
| Box costs | 47/60 × $1.99 = $1.55883; 37/60 × $1.99 = $1.22717 |
| Coupling | Paired 72/72; pooled 5,175 wins, 9 losses, 0 ties; AUROC 0.9982638889 |
| Coupling extrema | Minimum matched −1,892,996; maximum crossed 20,290,152; two crossed positive |
| Splits | Pose 461/116, overlap 0, union 577; coupling 424/72 |
| Pose | Integer 101/116; float 102/116; agreement 114/116 |
| Pose seed spread | `[102, 93, 104, 102, 92]`; median 102 |
| Temporal medians | 0.986207/0.878049; 0.796552/0.714286; 0.734483/0.536585; 0.289655/0.149826 |
| Direction gaps | 10.8158, 8.2266, 19.7897 and 13.9829 percentage points |
| Cue delays | Row 72: 14.932 s; row 96: 4.932 s |
| Generated-fakes record | Each evaluation seed has 6 real and 6 fake scores, zero threshold errors, 36/36 correctly ordered pairs, recorded CI `[1.0, 1.0]` |
| Replay/splice and ℓ2 | No bundled run records; Section 8 correctly reports no figures |
| White-box | Outstanding, with no result claimed |

The sole numerical error in Sections 6–8 is the swapped/misattributed host execution time.

# 4. Strongest and weakest claim

The strongest claim is Theorem 1’s execution binding: two independently verifiable final-relation proofs, byte-identical 1,101-byte proved and executed statements, a pinned reproducible ELF/key, complete source hashes, independent oracles and tamper rejection.

The weakest claim is the physical corollary: it assumes genuine sensor bytes, assumes the learned numerator identifies physical illumination, inherits A8’s undeveloped early-pattern premise, permits optical relay, proves one local transition, accepts an operator-manufactured session tree, and uses an anchor that binds nothing against its operator.

# 5. Verdicts

(a) **Correctness, evidence and internal closure: minor revision remains, because tonight’s fixable residue is to correct the row-72/row-96 execution times, say seven completed rounds and cite rounds 6–7, replace the false training-seed wording, describe the correction companion consistently, and either bundle the exact `fdb8745d…` generating script or state explicitly that the generated-fakes run cannot be reproduced from the supplied harness; DOI and licence remain separate principal-controlled release metadata.**

(b) **Venue: weak reject for IEEE S&P and CCS because, in my judgement, the significance remains below their bar, while Section 8 should be retained after those minor corrections as candid diagnostics inside a systems paper and must carry no security-guarantee weight.**

— BOSUN ⚓
