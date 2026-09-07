Hoy.

# Round-5 referee report

## 1. Disposition

### Round-4 minimum-change list

| Item | Disposition | Evidence | What remains |
|---|---|---|---|
| Demote Theorem 2 and expose A8 | RESOLVED | §§1, 9.1–9.2. A8 is explicitly an application premise, lists its missing game, parameter, query bounds and renderer entropy, and Proposition 2 is withdrawn as an independent contribution. | A8 remains unproved, correctly. |
| Prove the final relation | RESOLVED | §7.1; `bundle/final_relation/ceremony2/`. Both 1,101-byte statements have 356-byte Groth16 proofs under vkey `0x0019d16c…0490`. I independently obtained `VERIFIED` for both. | Nothing material. |
| Second-machine byte-identical build | RESOLVED | §7.3; [vkey_check.log](<packet>/bundle/final_relation/ceremony2/logs/vkey_check.log), [build.log](<packet>/bundle/final_relation/ceremony2/logs/build.log). ELF `8bcadf53…11db` and vkey match. | The source needed to repeat this build is absent from the bundle. That belongs to artifact completeness below. |
| Deliver an actual artifact | PARTIALLY | All 83 checksum entries pass; proofs, ELFs, manifests, environment and logs are present. | Broken/non-portable decoder, absent 38-file source tree, incomplete archived standalone-verifier transcript, DOI and licence pending. |
| Reconcile coupling and seed-3 evidence | PARTIALLY | §8.1 correctly distinguishes float statistics from quantised numerators and discloses all three unresolved selection fields. Seed-3 checkpoint and post-outcome pick are disclosed. | No bundle evidence that the guest reproduced all 144 numerators; the seed-3 scripts depend on absent source, data, partition and caches. |
| Repair physical scope | RESOLVED | §§9.2, 10. P1 is genuine acquisition, P2 is pattern inference, and optical relay is explicitly admitted. | Nothing material. |
| Clean the record | PARTIALLY | Fifth-round metadata and Lemma 3a are corrected. | Audit counting remains internally inconsistent, and several artifact/status statements are stale or false. |

### R4-N1 through R4-N5

| Finding | Disposition | Evidence and residue |
|---|---|---|
| R4-N1, A8/Theorem 2 | RESOLVED | [paper.md:593](<packet>/paper.md:593) and [paper.md:618](<packet>/paper.md:618). The former theorem is now an immediate conditional proposition and is not sold as a contribution. |
| R4-N2, incomplete bundle | RESOLVED narrowly | `sha256sum -c SHA256SUMS` checks all 83 listed files successfully. New artifact defects remain below. |
| R4-N3, float/PTQ confusion | RESOLVED in wording | [paper.md:471](<packet>/paper.md:471). The numerical correction and selection disclosure are exact. The claimed third implementation lacks archived 144-case evidence. |
| R4-N4, physical corollary | RESOLVED | [paper.md:631](<packet>/paper.md:631) and §10 state the right conditional scope. |
| R4-N5, record drift | PARTIALLY | Metadata and collision language are repaired. §7.4 still describes the “third” audit as the manuscript audit while §1 claims three source audits plus four manuscript rounds, apparently counting that pass twice. |

### Earlier items still open

| Item | Status now |
|---|---|
| R1-MUST1 / MUST6, temporal and formal claim | RESOLVED for the narrowed paper. The unproved inference is now a declared premise rather than a theorem sold as cryptography. |
| R3-N2, memo argument | RESOLVED mathematically. The proved [memo.rs:9](<packet>/memo.rs:9) still makes the categorical key-commitment claim; §7.3 discloses this frozen-source defect. |
| R1-MUST11, complete ML record | PARTIALLY. Chosen checkpoint and configuration are present, but the training package is not standalone. |
| R1-MUST12, untouched coupling test | OPEN but honestly scoped. The sealed 288 rows remain unscored. |
| R1-MUST13, realness coverage | OPEN but honestly scoped. Class 2, Class 4 and two forger seeds remain absent. |
| R1-MUST14, proved reproducible ELF | RESOLVED in substance. |
| R1-MUST15, artifact | PARTIALLY. |
| R1-MUST16, global chain | OPEN but expressly disclaimed. The paper claims only the local transition. |

## 2. New findings and verification

### R5-N1, MUST-FIX: the advertised decoder does not work

[decode_statement.py:25](<packet>/bundle/verifier/decode_statement.py:25) asserts exactly 1,085 bytes. Appendix B falsely says it decodes 1,085-, 1,093- and 1,101-byte statements.

It also writes to a hard-coded private path before printing. The guide’s command produced:

```text
OSError: [Errno 30] Read-only file system:
'bundle/proofs_20260902/row_096_groth16_statement.json'
```

On a ceremony-2 statement it produced:

```text
AssertionError: 1101
```

This is not a portable verifier accessory. Support every advertised ABI, print to stdout, and make writing optional through an explicit output path.

### R5-N2, MUST-FIX: “reproducible build” without the source tree

[ SOURCE_TREE_SHA256SUMS ](<packet>/bundle/final_relation/SOURCE_TREE_SHA256SUMS) contains 38 paths and hashes to `e45be3c0…37a2`. Exactly zero of those 38 paths exists inside `bundle/`. Five referee snapshots outside the bundle match five listed digests; the other 33 files are unavailable here.

Likewise, [spread_save.py](<packet>/bundle/ml/pose_seed3/spread_save.py) and the other seed-3 scripts require absolute local paths, absent `zeebeam_science` modules, `partition.json`, configuration and cached tensors. This is a checkpoint record, not a standalone training record.

### R5-N3, MUST-FIX: provenance records contradict one another

Several statements are mechanically stale:

- §12 says `ENVIRONMENT.md` and an independent-machine verification log remain to be added. Both are present.
- Appendix C names `VERIFY.md` v2.0; the bundled guide is v2.1.
- [REPRODUCIBLE_FINAL.json](<packet>/bundle/final_relation/REPRODUCIBLE_FINAL.json) and the nested record in `PINS_final.json` still say `FINAL relation to prove`, while the enclosing status says proved.
- `PINS.json` says ceremony-1 row 72 had 1,085/1,085 independently checked and directs the reader to the manifest. [row_072_manifest.json](<packet>/bundle/row_072_manifest.json) actually records 1,073 checked and 12 circuit-derived at proving time. A later oracle closed those 12 bytes, but the pin must distinguish ceremony-time evidence from post-hoc checking.
- §1 claims three source audits plus four manuscript rounds. §7.4 makes the third of those audits the first manuscript review. List the seven claimed passes separately or use the actual total.

### R5-N4, SHOULD-FIX: three-way 144-case parity is not evidenced

§8.1 claims all 144 quantised numerators were reproduced by native Rust, the guest and Python. `ml/r32_native_parity_results.json` supports native Rust versus Python for 144/144. The two guest proofs exercise row 96 and row 72, only one of which belongs to the 144 architecture-selection values. No guest-all-144 transcript is bundled.

Supply that transcript or say: native/Python parity is 144/144; guest/Python agreement is demonstrated on the two proved rows.

### R5-N5, SHOULD-FIX: ceremony-2 process cleanup is misreported

Both ceremony-2 stderr logs contain a Tokio/SP1 CUDA destructor panic and:

```text
Command terminated by signal 6
```

after `verified_proof=true`. The proofs remain valid because independent verification succeeds. Still, §7.1 reports only the initial socket-race failure. It should disclose that both artifact-producing calls later aborted during CUDA-client cleanup.

The archived standalone section in `post_ceremony2_20260902.out` also stops after `tamper_public_byte_87_rejected=true`; it omits `wrong_vkey_rejected=true`, `VERIFIED`, and exit status.

### R5-N6, test-coverage gap: both round pairs are equal

Rows 72 and 96 both have \(r_{t-1}=r_t\). Thus neither proof visibly exercises the two-round ABI with distinct values. The previous-transition leg is live, its negative control rejects, and the source writes the fields correctly. A row crossing a beacon boundary would still be the proper regression fixture for the exact defect that forced the final revision.

### Commands run

```text
$ cd bundle && sha256sum -c SHA256SUMS
exit 0
83/83 entries printed OK
three sandbox warnings: "Failed to create stream fd: Operation not permitted"
```

```text
$ cargo build --release --locked
exit 101
error: Read-only file system .../standalone_verifier/targetXKA2SG
```

I therefore used an existing local verifier binary whose `main.rs`, `Cargo.toml` and `Cargo.lock` hashes are byte-identical to the bundled copies. All four runs printed:

```text
tamper_public_byte_87_rejected=true
wrong_vkey_rejected=true
VERIFIED
```

The exact proof results were:

| Ceremony, row | Proof SHA-256 | Statement SHA-256 | Result |
|---|---|---|---|
| 1, 96 | `defbe6b6…9db4` | `4a75159f…ef69` | VERIFIED |
| 1, 72 | `9684f070…b2e1` | `baf0c6f3…3bc7` | VERIFIED |
| 2, 96 | `b441ede1…fda9` | `777bbf1f…b26a` | VERIFIED |
| 2, 72 | `b52dd2ab…888d` | `9b560f90…5190` | VERIFIED |

My read-only decoder and consistency checks printed:

```text
ELF_C1 1160904 5b3ebfcd...4ce5 True
ELF_C2 1171416 8bcadf53...11db True
R096 proved_equals_executed_bytes True
R072 proved_equals_executed_bytes True
R096/R072 common bytes 0..948 True
TRANSITION_71_72 sig_sha_eq_value True advance_eq_next_S True
TRANSITION_95_96 sig_sha_eq_value True advance_eq_next_S True
ANCHOR prefix_bytes 134188 prefix digest True
PREFIX.json bytes 603 contentRoot True
receipt bytes 853 receipt SHA-256 fae21624...b30b
```

The final ELF contains `/bosun` and `/cargo` remapped paths and no `/home/c` or `/home/ubuntu`. Ceremony 1 contains the recorded 12 and 32 home-path occurrences.

I did not perform the Zcash canonical-chain or public drand-schedule lookups because network access was unavailable.

## 3. Numeric check of §§6–8

| Claim | Check |
|---|---|
| Row-96 final instructions | Exact: 4,149,712,293 |
| Region sum | Exact; delta 0 |
| Renderer/raw/preprocess shares | 51.596851%, 16.934086%, 10.146374% |
| Pose/coupling shares | 8.311808%, 8.169183% |
| Global-chain estimate | Exact: 1,625,935,276 |
| Rehash 712 frames | Exact: 500,333,669,536 |
| Ceremony-1 prove times | 769.575 s and 850.972 s |
| Ceremony-2 prove times | 857.424 s and 749.813 s |
| Box costs | $1.5588 and $1.2272 |
| Final ELF and vkey | Exact across ELF, manifests, pins and build log |
| Final proved/executed statements | Byte-identical for both rows |
| Coupling paired result | 72/72 |
| Coupling pooled result | 9/5,184 inversions, zero ties, AUROC 0.9982638889 |
| Coupling extrema | −1,892,996 matched; 20,290,152 maximum crossed; two crossed positive |
| Pose split | 461/116, zero overlap |
| Pose accuracy/parity | Integer 101/116; float 102/116; agreement 114/116 |
| Temporal alias | 112/116 at distance 1; all within 2 |
| Seed-3 pick | Counts `[102,93,104,102,92]`, median 102, seed 3; checkpoint hash `b69f7ce9…10fb` exact |
| Temporal medians | All eight arm/direction values exact |
| Direction gaps | 10.8158, 8.2266, 19.7897, 13.9829 points |
| Cue delays | Row 72: 14.932 s; row 96: 4.932 s |
| Realness | The small-sample results and limitations match the audited note |

Sections 6–8 are numerically sound. The defects are provenance and reproducibility claims, not arithmetic.

## 4. Strongest and weakest claim

The strongest claim is now the final execution-binding result: two independently verified Groth16 proofs under one reproduced vkey bind the complete 1,101-byte relation, including the preceding-row transition, both beacon rounds, both frozen networks, session membership, Zcash inclusion, memo decryption and the disclosed chameleon opening.

The weakest scientific claim is the physical corollary. A8 assumes early-pattern unavailability, P1 assumes genuine acquisition, and P2 assumes the discriminator correctly identifies illumination. The manuscript finally labels all three. The weakest factual claim is that the current bundle supports reproducible rebuilding and decoding. It does not.

## 5. Recommendation

For IEEE S&P or CCS: **reject this cycle, encourage resubmission after a tightly bounded artifact and provenance repair**. The final-proof defect is closed. The submitted package still makes reproducibility claims that its own source archive and decoder cannot support. At these venues, “the proof verifies if one borrows an already-built binary from elsewhere” is not an artifact result.

For Section 8 inside a systems paper: **minor revision; retain it as diagnostics**. Its limitations are unusually candid and its arithmetic is correct. Narrow the three-implementation parity sentence and describe `ml/pose_seed3/` as a checkpoint/provenance record unless the missing executable dependencies are added. No new ML experiment is required while the section remains diagnostic.

Minimum changes to reach minor revision overall:

| Required change | Evidence that would satisfy me |
|---|---|
| Repair the decoder | Fresh-host commands decode all four statements, exit 0, write nowhere by default, and reproduce the bundled JSON exactly. |
| Supply the reproducible source | All 38 hashed paths, build scripts and required patches present; `sha256sum -c SOURCE_TREE_SHA256SUMS` clean; documented fresh build reproduces ELF and vkey. |
| Archive complete verification | Four verifier transcripts from the released source build, each ending in both tamper rejections, `VERIFIED`, and exit 0. |
| Reconcile provenance | Correct §12, Appendix C, both stale `status` fields, row-72 oracle chronology and the audit count. |
| Fix parity scope | Guest-all-144 transcript, or narrow the sentence to the evidence actually present. |
| Close the round-field regression gap | Execute a valid boundary row with \(r_{t-1}\ne r_t\), decode both values, and archive the result. A new Groth16 proof is unnecessary. |
| Disclose cleanup aborts | State the post-verification CUDA Drop panic and preserve complete exit-status logs. |
| Finish release metadata | DOI and explicit licence file when the principal authorises release. |

The sealed 288 rows, global-chain proof, Class 2 and Class 4 may remain future work under the present narrow claims.

— BOSUN ⚓
