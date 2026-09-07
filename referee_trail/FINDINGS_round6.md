Hoy.

# 1. Round-5 disposition

## Eight minimum changes

| Item | Disposition | Evidence and residue |
|---|---|---|
| Repair decoder | RESOLVED | All four bundled statements decode byte-identically to their JSON. The 1,093-byte branch also decoded correctly in an in-memory fixture. `--out` could not run because this sandbox forbids `/tmp` writes, but the same serialization path was verified through stdout. [decoder](<packet>/bundle/verifier/decode_statement.py:20) |
| Supply reproducible source | PARTIALLY | All 38 manifest paths pass, plus 39 vendored `jubjub` files, two blobs and the build driver. The driver is fail-open and omits outer `cargo build --locked`; see R6-N1. The seed-3 ML scripts still require absent absolute-path modules, partition files and caches. [source README](<packet>/bundle/source/README.md:25) |
| Complete four verification transcripts | RESOLVED | Four transcripts match the actual proof and statement hashes and end with both rejections, `VERIFIED`, `exit=0`. |
| Reconcile provenance | PARTIALLY | The five named R5 corrections are present. Fresh contradictions remain in the oracle notes, Appendix C and frontmatter. |
| Narrow parity scope | RESOLVED | Section 8.1 now says native/Python 144/144 and guest agreement only on executed rows 96, 72 and 95. All three guest comparisons match. [Section 8.1](<packet>/paper.md:489) |
| Boundary-round regression | RESOLVED | Row 95 decodes to 31,521,619 / 31,521,620. Its 1,101-byte JSON is byte-identical to decoder output; Python model values match; row-94 advance recomputes row-95 \(S_t\). [fixture log](<packet>/bundle/final_relation/boundary_row_095/row_095_execute.log:15) |
| Disclose cleanup aborts | RESOLVED | Section 7.1 and both stderr logs report the destructor panic and signal 6 after proof verification. [Section 7.1](<packet>/paper.md:346) |
| DOI and licence | OPEN | Both remain pending. There is no licence file. DOI may await deposit; the licence cannot await artifact release. [Section 12](<packet>/paper.md:770) |

## R5-N1 through R5-N6

| Finding | Disposition | Evidence and residue |
|---|---|---|
| R5-N1 decoder broken | RESOLVED | Portable, stdout-only by default, three length branches, four exact JSON reproductions. |
| R5-N2 missing source | PARTIALLY | Proof source is supplied and all 38 hashes pass. Build-driver reliability and standalone ML-training closure remain missing. |
| R5-N3 contradictory provenance | PARTIALLY | Original stale statuses, chronology, audit count and artifact list are repaired. R6-N2, N3 and N5 continue the same untidy tradition. |
| R5-N4 unsupported three-way parity | RESOLVED | Claim narrowed exactly to available evidence. |
| R5-N5 cleanup abort and incomplete transcript | RESOLVED | Abort disclosed; replacement transcripts are complete. The old `post_ceremony2` excerpt remains truncated but is no longer presented as complete. |
| R5-N6 equal-round coverage | RESOLVED | Boundary-row execution is sufficient. A third proof was never required. |

# 2. New findings and commands

## R6-N1, MUST-FIX: the reproducible-build driver can report success after failure

[build_reproducible_20260902.sh](<packet>/bundle/source/build_reproducible_20260902.sh:2) uses only `set -u`. Its build pipeline lacks `pipefail`, subsequent hash and vkey failures are unchecked, `find | head` may select stale output, and the final unconditional `echo` can make the script exit 0. The outer Cargo invocation also lacks `--locked`. The guest `build.rs` does set `locked: true`; the defective part is the released driver.

Required repair: `set -euo pipefail`, outer `cargo build --locked`, require exactly one ELF, compare its full SHA-256 and vkey against the pins, and exit nonzero on any mismatch.

## R6-N2, MUST-FIX: independent oracle records call their own values circuit-derived

The final row-72 manifest says `oracle_bytes_circuit_derived: 0` and `oracle_mode: independent_row_72`, then calls its coupling numerator circuit-derived two lines later. [Manifest](<packet>/bundle/final_relation/ceremony2/row_072_manifest.json:6)

Row 95 repeats the contradiction. [Execution log](<packet>/bundle/final_relation/boundary_row_095/row_095_execute.log:17)

The check itself is sound: when `expected_coupling_score_numerator` exists it compares against the Python value and counts all eight bytes as checked. The emitted note is hard-coded stale prose. [witness.rs](<packet>/bundle/source/rust/row_binding_join_membership_sp1_candidate/script/src/witness.rs:209)

Correct the manifests/log sidecars and disclose that the frozen host source prints a stale note. The proof need not change.

## R6-N3, MUST-FIX: Appendix C falsely says ceremony-1 statements publish \(r_t\)

The 1,085-byte statements decode as `ZBAUGST2` and contain no round field. Appendix C says they “carry only \(r_t\).” [Appendix C](<packet>/paper.md:857)

Ceremony 1 verified its private row round, tied through the relation, but did not publish it in the statement. The claimed values come from the accompanying pins and records. Say that. The standalone verifier source also incorrectly documents every public statement as 1,085 bytes. [main.rs](<packet>/bundle/verifier/standalone_verifier/src/main.rs:5)

## R6-N4, SHOULD-FIX: Sections 6-8 contain measurements absent from the bundle

The arithmetic is sound, but the bundle lacks:

- The row-96 cycle-region execution logs underlying Section 6. Only totals appear in `PINS_final.json`.
- Evidence for the 26.5/27.3 GB GPU-memory figures.
- The realness run records supporting the exact six-versus-six, six-versus-eighteen, bootstrap, GPU-exhaustion and 1,000-query claims in Section 8.1. Searches for those terms returned no bundled file.

Exact empirical claims require their records. Appendix F pointing outside the artifact is insufficient for an artifact paper.

## R6-N5, EDITORIAL: record drift and units

The YAML status still says referee round 5 while the prose says sixth round. [paper frontmatter](<packet>/paper.md:1)

The `114 MB` RSS entries derive from 113,760-114,260 KiB. Those are 111.1-111.6 MiB or 116.5-117.0 MB. Use the recorded KiB values or label the approximation honestly.

## Command record

Every shell invocation emitted three sandbox lines:

```text
Failed to create stream fd: Operation not permitted
```

The mandated commands produced:

```text
$ cd bundle && sha256sum -c SHA256SUMS
exit=0
177/177 entries: OK
```

```text
$ cd bundle/source/rust && sha256sum -c ../../final_relation/SOURCE_TREE_SHA256SUMS
exit=0
38/38 entries: OK
```

```text
$ cd bundle && python3 verifier/decode_statement.py final_relation/ceremony2/row_096_groth16_public_values.bin
exit=0
```

It printed exactly the JSON in [row_096_statement.json](<packet>/bundle/final_relation/ceremony2/row_096_statement.json), including:

```text
total_bytes=1101
sha256=777bbf1f5b6197bd82fa9c9c041413efa5c647de7c7a627aa92b3d328adcb26a
previous_drand_round=31521620
row_drand_round=31521620
trapdoor_flag=1
```

Direct stdout-versus-file hashing printed:

```text
row_072 ceremony1 ... 7074bcf7... IDENTICAL
row_096 ceremony1 ... bf2f818a... IDENTICAL
row_072 ceremony2 ... 01f1c2fe... IDENTICAL
row_096 ceremony2 ... e67e7d34... IDENTICAL
```

The synthetic 1,093-byte branch printed:

```text
synthetic_1093 total_bytes=1093
magic=ZBAUGST3
row_drand_round=31521620
trapdoor_flag=1
previous_drand_round_present=False
```

The attempted `--out` test printed:

```text
mkdir: Read-only file system
[runner path redacted]: Read-only file system
```

The standalone build attempt printed:

```text
$ cargo build --release --locked
error: Read-only file system (os error 30) at path ".../standalone_verifier/targetltIbgj"
exit=101
```

`cargo metadata --locked --offline --no-deps` exited 0 and identified `zeebeam-standalone-verifier 0.1.0`; the lockfile pins `sp1-verifier 6.4.0`.

Transcript-to-artifact checking printed:

```text
c1_row072 artifact_fields_and_footer_match True
c1_row096 artifact_fields_and_footer_match True
c2_row072 artifact_fields_and_footer_match True
c2_row096 artifact_fields_and_footer_match True
```

Boundary checking printed:

```text
boundary_decode ... IDENTICAL
bytes 1101
rounds 31521619 31521620 expected 31521619 31521620
coupling 2910142 2910142 2910142
typed_root True
pose 1 1 1
logits_match True
uncropped_match True
```

Independent chain recomputation printed:

```text
row94_advance 2a7ef5dd...9a0c7
row95_S_t     2a7ef5dd...9a0c7
advance_matches True
sig_sha_matches_value True
round_pair 31521619 31521620
```

`bash -n source/build_reproducible_20260902.sh` printed `bash_n_exit=0`; `shellcheck` was unavailable. I also used `sed`, `nl`, `cat`, `rg`, `find`, `wc`, `sha256sum` and read-only Python scripts to inspect and recompute the cited records.

# 3. Numeric check of Sections 6-8

| Claim | Result against bundle |
|---|---|
| Row-96 total | Exact: 4,149,712,293 |
| Region sum | Exact: 4,149,712,293, delta 0 |
| Main shares | 51.596851%, 16.934086%, 10.146374%, 8.311808%, 8.169183% |
| Row-72 total | Exact: 4,148,971,330 |
| Global-chain estimate | Exact: 1,625,935,276 |
| Rehash estimate | Exact: 500,333,669,536 |
| Proof times | Exact: 850.972, 769.575, 857.424 and 749.813 s |
| Costs | 47/60 × $1.99 = $1.55883; 37/60 × $1.99 = $1.22717 |
| Proof and statement sizes | Exact: raw 356 B; framed 2,779/2,780/2,795 B; statements 1,085/1,101 B |
| Coupling splits | Exact: 424 fit, 72 architecture-selection |
| Coupling separation | Exact: paired 72/72; pooled 9/5,184 inversions, zero ties, AUROC 0.9982638889 |
| Coupling extrema | Exact: min matched -1,892,996; max crossed 20,290,152; two crossed positive |
| Parity | Native/Python 144/144; guest/Python exact on rows 96, 72 and 95 |
| Pose split | Exact: 461/116, zero overlap, 577 union |
| Pose accuracy | Integer 101/116; float 102/116; agreement 114/116 |
| Seed spread | Exact: `[102,93,104,102,92]`, median 102, chosen seed 3 |
| Temporal alias | Exact: 112/116 evaluation rows adjacent to same-class training |
| Temporal medians | All ten table values exact |
| Direction gaps | 10.8158, 8.2266, 19.7897 and 13.9829 percentage points |
| Cue delays | Exact: row 72, 14.932 s; row 96, 4.932 s |
| Realness claims | Not checkable from this bundle; supporting run records are absent |
| GPU memory | Not checkable from this bundle |
| RSS | Recorded values are KiB; manuscript unit conversion is sloppy |

Sections 6-8 contain no arithmetic failure. Their remaining weakness is evidence packaging.

# 4. Strongest and weakest claims

The strongest claim is final-relation execution binding for rows 72 and 96: unchanged proofs previously verified independently, exact 1,101-byte statements, one reproduced vkey, complete artifact hashes, and circuit-independent agreement for every public byte.

The weakest scientific claim remains the physical corollary. A8 is an undeveloped application premise, P1 assumes genuine sensor bytes, P2 assigns meaning to a discriminator with no untouched same-session test, and optical relay survives. The weakest factual material is Section 8.1 realness: exact numbers with no bundled records.

# 5. Recommendation

For IEEE S&P or CCS: **weak reject**. Correctness has reached minor-revision territory, but top-tier security significance has not. The central theorem is execution soundness for one large integrated program; the temporal result rests on A8; the anchor deliberately binds nothing against its operator; the physical security conclusion rests on P1 and P2; and only one local transition is proved. The engineering artifact is now respectable. The security result remains premise-heavy.

For Section 8 inside a systems paper: **minor revision, retain as diagnostics**. The coupling and pose accounting is candid and numerically exact. Supply the realness records, describe the seed-3 directory as a checkpoint/provenance package, and forbid any reading of these diagnostics as security validation.

Minimum remaining changes:

| Change | Satisfactory evidence |
|---|---|
| Fail-closed build driver | Clean-machine transcript with `set -euo pipefail`, outer `--locked`, exactly one ELF, asserted SHA-256 and vkey, exit 0 |
| Correct ceremony-1 disclosure | Appendix C and verifier comment say the 1,085-byte statement publishes no round; pins supply the claimed private round |
| Repair oracle provenance | Corrected row-72 and row-95 sidecars, explicitly superseding the stale “circuit-derived” notes |
| Package empirical evidence | Row-96 cycle logs, GPU-memory record and realness run records under `SHA256SUMS`, or remove those exact claims |
| Clean metadata | Round-6 frontmatter and correct RSS units |
| Finish release metadata | Explicit licence before release; DOI upon deposit |

No new proof, ML experiment, global-chain proof or sealed-288 opening is required for minor revision.

— BOSUN ⚓
