---
round: 10
date: 2026-09-03
model: gpt-5.6-sol (codex exec, read-only, effort high)
packet: paper.md (v3.0), state.md (v1.20), bundle/ (SHA256SUMS 3,040)
---

Hoy.

## 1. Chain relation

The core relation is correct, but the paper overstates one signature check and the bundle does not supply a reproducible chain build or the claimed independent-verification records.

The first three checksum ledgers pass: `SHA256SUMS`, `SOURCE_TREE_SHA256SUMS`, and `CHAIN_SOURCE_SHA256SUMS`.

The guest in [main.rs](<packet>/bundle/source/chain/program/src/main.rs:120):

- Parses exactly 712 sequential rows.
- Recomputes all 712 `advance_chain` transitions, including the terminal transition to \(S_N\).
- Requires non-decreasing rounds.
- Verifies one quicknet signature for the first occurrence of each of 66 rounds.
- Requires every row’s beacon value to equal the cached verified value for its round.
- Reconstructs the context, row leaves, padding leaves, indexed internal nodes, and wrapped root.
- Commits the claimed 279-byte `ZBCHAIN1` statement.

The domains and encodings match the per-row crate: `ZB:SESSION:CONTEXT:v1`, `ZB:SESSION:ROW:v1`, `ZB:SESSION:NODE:v1`, `ZB:SESSION:ROOT:v1`, big-endian integers, length-prefixed hash parts, and the same row-leaf ordering. See [membership/src/lib.rs](<packet>/bundle/source/rust/row_binding_join_membership_sp1_candidate/membership/src/lib.rs:267).

I independently checked the bundled log:

- 712/712 chain transitions agree.
- The final transition lands on \(S_N\).
- All 66 distinct quicknet signatures pass a batched pairing verification.
- Every logged value is SHA-256 of its row’s bundled signature.
- Each round has exactly one distinct signature and value.
- The reconstructed context and wrapped root agree.

There is nevertheless a precise claim error in §9.5. The guest does not check every repeated row’s `signature` field. It ignores that field after the first occurrence of a round and compares the row value with the cached verified value. The actual log repeats the same signature, so this does not invalidate this proof. The text must say “one signature per distinct round, with every row value equal to the verified value for that round.” If the stronger claim is retained, the guest must hash every repeated signature and the proof must be regenerated.

The decoded manifest matches `chain_expect.json` field by field: row count, depth, distinct-round count, first and last round, \(S_0\), \(S_N\), log digest, authority-manifest digest, context digest, root, and session identifier.

The filesystem was read-only, so a literal in-place rewrite was impossible. I ran the unmodified `chain_expect.py` computation while intercepting only its final write. It produced 842 bytes byte-for-byte identical to the existing JSON, SHA-256 `35bf06e9917404fdaa5196b1de9cb3a51cfe48b8cfdcdd3b0fedf21f1f2291a6`.

The ELF digest and vkey agree record-to-record across [REPRODUCIBLE_CHAIN.json](<packet>/bundle/final_relation/chain/REPRODUCIBLE_CHAIN.json), [chain_manifest.json](<packet>/bundle/final_relation/chain/chain_manifest.json), the box logs, and [§9.5](<packet>/paper.md:788):

| Pin | Value |
|---|---|
| ELF SHA-256 | `4934b9e260d6250dcb119f4eaa9f46a3ff4eb144c02340175b356ef17ade3926` |
| vkey | `0x005402a848fdc787e32c0a110ca2662dd61c580481b4e418d5400a52fbd7df50` |

That agreement is not an independent byte check: the ELF is absent. Worse, [program/Cargo.toml](<packet>/bundle/source/chain/program/Cargo.toml:9) points to missing or wrongly relocated path dependencies, and `CHAIN_SOURCE_SHA256SUMS` covers only the chain wrapper, not those dependencies. A stranger cannot rebuild the chain guest or derive the vkey. This contradicts §12’s “complete frozen source tree.”

The claimed SDK cold-verification and standalone-verification transcripts are also absent. The shipped proving log records in-process verification before the documented destructor panic, but that is not either claimed independent record.

Finally, the operator caveat is understated. The proof establishes internal consistency of an operator-supplied log and its exact tree. It does not establish that the log is an authentic acquisition chronology, that its raw-frame digests name genuine frames, or that its metadata and round schedule were externally precommitted. The operator controls the log, not merely the frames whose digests it records.

## 2. Section 7.6 and fleet

All 712 oracle rows are present, unique, and contiguous.

| Quantity | Recomputed result |
|---|---|
| Numerator range | −1,924,590 at row 309 to 93,030,486 at row 582 |
| Full-session median | 49,650,008.5, not the paper’s 49,650,008 |
| Negative rows | 309 and 320, both outside the anchor |
| Architecture-row median | 49,627,252 |
| Other-row median | 49,650,008.5 |
| Pose agreement | 532/577 |
| Training agreement | 431/461 |
| Evaluation agreement | 101/116 |
| Freeze mapping | cue 12 maps to class 10; 33/34 correct |
| Weakest cues | cue 09: 38/50; cue 02: 46/55 |
| Rounds | 66 |
| Full-session boundaries | 65 |
| Anchored boundaries | **25**, not 65 |
| Executed rows | 259/259, rows 1–259 |
| Instruction range | 4,146,753,922 to 4,154,781,545 |
| Instruction median | 4,150,763,529 |
| OLS slope | 31,134.04 instructions per row |

Thus [§7.6](<packet>/paper.md:514) contains two numerical errors: the exact median loses 0.5, and “65 of the 259 anchored rows” is false. There are 65 boundaries over the whole session but only 25 within rows 1–259.

All 712 membership witnesses agree with their oracle records. All 259 execution statements decode correctly and agree on the checked oracle fields.

Fleet recomputation from the 257 manifests and logs:

| Fleet quantity | Recomputed |
|---|---|
| Proved rows | 257, exactly all anchored rows except 72 and 96 |
| Prove min / median / max | 727,794 / 755,566 / 955,698 ms |
| Setup min / median / max | 20,790 / 22,710 / 24,876 ms |
| RSS min / median / max | 110.9 / 111.3 / 333.5 MiB |
| GPU-hours | 54.6211 |
| Verify median / max | 365 / 609 ms |
| Proof sizes | framed 2,791–2,795 bytes; raw 356 bytes |
| Detached-restart state | 99 rows already complete |
| Per-box output | 33 rows on box 0; 32 on each other box |

Every manifest timing agrees with its box log. Each public statement matches the corresponding executed statement. The standalone transcript contains all 257 rows, including tamper and wrong-vkey rejection. The five cold-verification records cover rows 1, 95, 130, 200, and 259.

Instance-minutes and cost cannot be independently recomputed because exact launch timestamps are not bundled. Assuming all eight machines launched at exactly the paper’s rounded 19:50 time and using the last recorded completion minute yields 4,073 instance-minutes and \(4073/60 \times \$1.99 = \$135.09\). That reproduces the paper’s arithmetic, not its provenance.

I also could not rerun the model oracles from frames, because the frames are absent, or rerun SP1 compilation/execution in this read-only environment. The prescribed figure smoke test silently fails: `all_rows/all_rows_figures.py` does not exist. The actual script is under `all_rows/figures/`, has no `--help` interface, and hard-codes staging paths.

## 3. Inconsistencies introduced elsewhere

- The abstract and contribution 6 say “all 66 beacon signatures.” That is defensible only as “one signature for each of 66 rounds”; it must not imply validation of every repeated signature field.
- §9.5 understates operator control and claims independent verification records that are not bundled.
- §11 is consistent with the new work: rows 260–711 still need an anchor for full per-row coverage. There is no stale “future chain proof” claim there.
- Appendix A’s printed pins agree with the records, but the absent ELF and incomplete, unbuildable source mean those pins cannot be independently reproduced.
- §12’s claim of a complete frozen source tree is false for `source/chain/`.
- §7.4 and Appendix F still say eight manuscript audit rounds and omit [FINDINGS_round9.md](<packet>/FINDINGS_round9.md). Nine prior rounds had closed before this pass.
- The conclusion still describes only two final-relation proofs and omits both the 257 additional row proofs and the whole-session proof.

## 4. Verdicts

(a) **Additions:** Substantively correct but not artifact-closed; required fixes are the §7.6 numbers, narrower signature wording, a stronger operator caveat, portable scripts, the chain ELF and complete dependency tree, and the missing cold/standalone transcripts. No new proof is required unless the authors retain the claim that every repeated signature field is circuit-checked.

(b) **Venue:** The standing judgement remains **weak reject for IEEE S&P and CCS**; these additions improve evidence but do not change the significance case.

— BOSUN ⚓
