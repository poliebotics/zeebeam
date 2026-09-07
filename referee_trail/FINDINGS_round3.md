Hoy.

# Round-3 referee report

## 1. Disposition

### Round-2 N1–N10

| ID | Disposition | Addressed in | What remains |
|---|---|---|---|
| N1 | RESOLVED | Abstract; §§1, 7.1, 7.3, 11, 13; Appendix A | The proof status is stated repeatedly and accurately. Honesty grade: **A**. |
| N2 | PARTIALLY | A4, A8, §9.2 | A8 does not justify the claimed uniformity or \(2^{-\lambda}\) bound. See R3-N1. |
| N3 | RESOLVED | §9.4 | The false third-party upper-bound claim is withdrawn completely. |
| N4 | RESOLVED | §5.1 | The 0..1,101 layout is continuous and byte-complete. |
| N5 | RESOLVED | Appendix A; `PINS_final.json`; final executed statements | Pins, status, layout and both decoded executions agree. |
| N6 | RESOLVED | §9.5 | The recomputed 1.63-billion and 500.33-billion estimates are sound. |
| N7 | RESOLVED | §9.7 | Rounds are correctly described as scheduled releases and conditional lower bounds on emission. |
| N8 | RESOLVED | Opening of §8 | Pose/coupling scope and the separate realness sessions are distinguished. |
| N9 | RESOLVED | §6 | Regions plus the 27,847,169-instruction remainder sum exactly to 4,149,712,293. |
| N10 | RESOLVED | §8.2 | The stated 8–20-point range agrees with 8.23–19.79. |

### Round-1 findings still PARTIALLY or OPEN after round 2

| Round-1 ID | Disposition | Addressed in | What remains |
|---|---|---|---|
| MUST 1, temporal indexing | PARTIALLY | §§3.1, 5.2, 7.3, 9.2 | The final program executes the previous-row check correctly. Theorem 2’s reduction remains incomplete. |
| MUST 2, rounds absent from statement | RESOLVED | §§5.1–5.2; final statement files | Both rounds appear at 1084..1100 in the executed final statements. |
| MUST 5, trapdoor anchor | RESOLVED | §§1, 9.4, 11 | The manuscript now says the anchor gives no record upper bound to anyone. |
| MUST 6, formal security | PARTIALLY | §9 | A1–A8 are useful, but Theorem 2 and Lemma 3a remain technically defective. |
| MUST 8, memo binding | PARTIALLY | §9.3, Lemma 3a | The two-opening route is the right route. Its description of Ironwood V3 derivations and commitment binding is wrong. |
| MUST 11, ML reporting | PARTIALLY | §8 | Configurations, counts and classwise results are much improved. Robustness seed integers and the final whole-frame training record remain absent. The coupling evidence conflicts with §8.1. |
| MUST 12, untouched coupling test | PARTIALLY | §§8.1, 11 | The absence is disclosed. The 288-row sealed session remains unscored, and the claimed 144-value fixture is absent. |
| MUST 13, realness coverage | PARTIALLY | §§8.1, 11 | Class 2 and Class 4 remain absent; the ROC sidecar remains defective. Disclosure is now adequate. Evidence is still inadequate. |
| MUST 14, reproducible proved build | PARTIALLY | §§7.3, 11–12 | Path remapping is implemented. No second-machine byte reproduction and no proof under that ELF exist. |
| MUST 15, artifact | PARTIALLY | §12; Appendix C | `VERIFY.md` contradicts the manuscript and remains stale. No DOI, licence, complete source-tree digest or independent log exists. |
| MUST 16, chain reconstruction | PARTIALLY | §§9.5, 12; `anchor/` | The 260-row prefix is now supplied and internally consistent. The full 712-row chain and global-chain proof remain absent. |
| SHOULD 2, sizes together | RESOLVED | §7.2 | All four quantities are now presented together. |
| SHOULD 7, related work | PARTIALLY | §10 | It is now a comparison, but several guarantee comparisons are technically wrong. |
| SHOULD 8, verifier table | RESOLVED | §5.1 | Byte ranges, encodings and constraints are supplied. |

## 2. New findings

### R3-N1. MUST-FIX: Theorem 2 still has no valid unpredictability reduction

A8 models SHA-256 and BLAKE3 as random oracles. The proof then asserts that the rendered pattern is uniformly random and guesses succeed with probability \(2^{-\lambda}\). Neither follows.

The renderer is a deterministic, potentially many-to-one transformation of the XOF stream. No injectivity or output min-entropy bound is given, and \(\lambda\) is undefined. The adversary also chooses the predecessor state, raw digest and metadata, so the reduction must handle adaptive inputs. “BLS is deterministic” is likewise not a proof of uniqueness; uniqueness should remain an explicit algebraic assumption.

State a game that fixes or commits the predecessor record before release, quantify oracle queries, and prove a min-entropy bound for the rendered output. An assumption directly covering unpredictability of the complete map \((\sigma,\text{predecessor record})\mapsto E_t\) would also suffice. [Theorem 2](<packet>/paper.md:564)

### R3-N2. MUST-FIX: Lemma 3a describes the wrong Ironwood V3 derivation

The actual crate path is:

\[
\rho=\mathrm{Rho::from\_nf\_old}(\text{action.nf}),\quad
g_d=\mathrm{DiversifyHash}(d),\quad
pk_d=[ivk]g_d.
\]

For V3, \(\psi\) and \(esk\) derive from `rseed` and \(\rho\). However, \(rcm\) derives through PRF-expand over `rseed`, \(g_d\), \(pk_d\), \(v\), \(\rho\), and \(\psi\), with domain byte `0x0B`. It is not merely derived from `rseed` and \(\rho\). Acceptance compares the **extracted x-coordinate** of the note commitment to `cmx`, then checks \(epk=[esk]g_d\).

The clean proof is shorter: two accepting scalar `ivk`s give openings of the same extracted Orchard commitment. A5 must explicitly assume message binding of that extracted commitment. Binding gives equal \(g_d\) and \(pk_d\); nonidentity \(g_d\) in a prime-order group gives equal scalar `ivk`s. Fixed `ivk` and `epk` fix the AEAD key, hence deterministic decryption fixes the memo. The DiversifyHash/PRF collision parenthetical is unnecessary and presently inaccurate. [memo.rs](<packet>/memo.rs:75), [Orchard V3 derivation](<cargo-registry>/)

### R3-N3. MUST-FIX: §8.1’s coupling result conflicts with the supplied training record

The exact r32 checkpoint and final-state hashes in `coupling_ladder_results.json` match the pins used by the statement. That record reports:

- AUROC **0.9982638889**, not 1.0.
- One matched example in the 0.3–0.4 bin.
- Two crossed examples above 0.5.
- `resolution_winner: null` and `selection_performed: false`.

The supplied `frozen_models_python_results.json` contains only rows 72 and 96. It contains no 144-value fixture. Consequently, “72/72,” “AUROC 1.0,” and bit-for-bit reproduction on all 144 values are unsupported by this bundle. A later PTQ evaluation might differ, but that evaluation must be supplied and reconciled with the exact source-checkpoint record.

### R3-N4. MUST-FIX: the verification guide materially misstates the artifact

`VERIFY.md`:

- does not run `sha256sum -c SHA256SUMS` first;
- builds without `--locked`;
- tells readers that the ceremony-1 round supplies the proved row’s lower time bound, although those proofs contain only \(r_t\), which bounds the following frame;
- retains the old contingent upper-bound language withdrawn in §9.4;
- calls row 72’s oracle partial in `PINS.json`, while state.md and §6 call it independently complete.

Appendix C and §12 therefore describe a procedure that the supplied guide does not contain. [VERIFY.md](<packet>/VERIFY.md:24)

### R3-N5. SHOULD-FIX: the reported final execution time conflicts with the log

Section 6 and state.md give **58.6 s host execution**. The supplied full row-96 log records `host_elapsed_ms=61501`, or **61.501 s**. If 58.6 s is an SP1 internal timing, label it accordingly. [execution log](<packet>/row_096_execute_final.log:41)

### R3-N6. SHOULD-FIX: §10 overstates competing and proposed guarantees

A trusted camera can still photograph a screen or relayed scene; trusted firmware does not exclude optical relay. A detector over attacker-supplied bytes does not exclude digital substitution without a trusted input path. Finally, “our relation trusts no sensor” is true only for execution binding; the physical corollary explicitly assumes genuine sensor output in P1.

### Artifact consistency that passed

`PINS_final.json`, both final decoded statements, state.md and the row-96 raw public-value hex agree on the ELF, vkey, 1,101-byte length, statement hashes, rounds, scores, verdicts, trapdoor flag, transaction and block. The row-96 hex hashes exactly to `777bbf…cb26a`. Old and final decoded statements differ only in `ZBAUGST2`→`ZBAUGST4` and the two added rounds. The `PREFIX.json` contentRoot recomputes exactly.

## 3. Numeric audit of §§6–8

### Sections 6, 7 and §9.5 costs

| Claim | Result |
|---|---|
| Final row-96 total 4,149,712,293 | Exact |
| Listed §6 regions plus remainder | Exact sum |
| Untracked row glue | 4,001,104,233 minus named nested regions = 27,764,103 |
| Complete “other” row | 27,764,103 + 65,149 + 4,137 + 13,780 = 27,847,169 |
| Renderer share | 51.5969%, correctly 51.6% |
| Raw hash share | 16.9341%, correctly 16.9% |
| Removing renderer roughly halves instructions | Correct |
| Global non-BLS advances | \((6,733,568-4,904,590)\times712=1,302,232,336\) |
| 66 distinct BLS verifications | \(4,904,590\times66=323,702,940\) |
| Global total using committed digests | 1,625,935,276, correctly about 1.6 billion |
| Rehashing every frame | \(702,715,828\times712=500,333,669,536\), correctly about 500 billion |
| Proof times | 769,575 ms = 12:49.6; 850,972 ms = 14:11.0 |
| Box cost | 47 min at $1.99/hour = about $1.56 |
| Host execution | **Mismatch: 58.6 s claimed, 61.501 s logged** |

### Section 8

The pose split recomputes exactly: 461 training, 116 evaluation, zero overlap, 577 total. Integer accuracy is 101/116, float accuracy 102/116, and agreement is 114/116. The supplied confusion matrices are exact recomputations from `ml_splits.json`.

Integer confusion matrix, rows true class and columns prediction:

```text
0:  4 1 0 0 1 1 0 0 0 0 0
1:  0 9 0 0 0 1 0 0 0 0 1
2:  0 0 9 0 0 0 1 0 0 1 0
3:  0 0 0 10 0 0 0 0 0 0 0
4:  0 0 0 0 9 0 0 0 0 1 0
5:  0 0 0 0 0 10 0 0 0 0 0
6:  0 0 0 0 0 0 10 0 0 0 0
7:  0 0 0 0 0 0 0 9 0 1 0
8:  0 0 0 1 0 0 2 0 6 1 0
9:  0 0 1 0 0 0 0 0 0 9 0
10: 1 0 0 0 0 0 0 0 0 0 16
```

Other checks:

| Claim | Result |
|---|---|
| 112/116 at distance one; all within two | Exact |
| Classwise integer results | Exact |
| Five-arm counts and medians | Match `pose_nocrop_result.md` |
| Temporal n=290/287 and whole-frame counts | Exact |
| Direction gaps 10.82, 8.23, 19.79, 13.98 points | Exact |
| Coupling 424/72 split, zero overlap, stated ranges | Exact |
| Coupling AdamW/batch/epoch/seed configuration | Exact |
| Coupling 72/72 and AUROC 1.0 | **Unsupported and conflicts with the ladder record** |
| Realness sample sizes and perfect exploratory results | Match |
| One generator seed, Class 2 absent, Class 4 outstanding | Match |
| Row 72 cue and role | Supported |
| Row 96 4.9-second cue delay | Still absent from the supplied records |
| Sealed 288-row session | Recorded as unopened and unscored |

## 4. Strongest and weakest claims

The strongest established claim remains the two cold-verified ceremony-1 Groth16 proofs: exact byte-level execution binding for the earlier relation under one pinned ELF and key. The final relation’s two 1,101-byte executions provide strong implementation evidence, with proof status disclosed impeccably.

The weakest formal claim is Theorem 2’s \(2^{-\lambda}\) unpredictability conclusion. Its intuition is plausible; its stated reduction does not establish it. The weakest factual line is §8.1’s perfect coupling result, because the supplied exact-checkpoint record says 0.9982638889 and the promised parity fixture is absent.

## 5. Recommendation

For IEEE S&P or CCS: **reject this cycle, encourage resubmission after major revision**. The manuscript is now unusually honest, technically serious and far better scoped. Acceptance still requires a valid Theorem 2 game, a correct Ironwood V3 lemma, reconciliation of the coupling statistics, and a truthful verification guide. The pending final proof is disclosed everywhere and earns no dishonesty penalty; it remains the evidentiary boundary of the full relation.

For the ML diagnostics section as an independent contribution: **reject**. As a candid limitations and diagnostic section inside a systems paper, it is valuable once §8.1 is repaired. One time-confounded take, no untouched coupling test, one forger seed, missing Class 2, unfinished Class 4 and an unscored sealed session do not constitute an ML contribution.

— BOSUN ⚓
