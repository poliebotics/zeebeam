Hoy.

# Round-4 referee report

## 1. Disposition

| Finding | Disposition | Section | What remains |
|---|---|---|---|
| R3-N1, Theorem 2 | PARTIALLY | §§9.1–9.2 | A8 now covers the complete map and the false \(2^{-\lambda}\) bound is gone. The assumption still lacks a security experiment, probability space and security parameter. The theorem merely restates A8. |
| R3-N2, Ironwood V3 | PARTIALLY | §9.3 | The derivations and extracted-commitment route are now correct. The supposedly removed collision parenthetical remains at [paper.md:632](<packet>/paper.md:632), and [memo.rs:10](<packet>/memo.rs:10) still states key commitment categorically rather than under A5. |
| R3-N3, coupling | PARTIALLY | §8.1 | The headline numbers are corrected exactly and the fixture is supplied. The float/PTQ parity claim is unsupported, and the ladder still says `selection_performed:false`, `resolution_winner:null`. |
| R3-N4, VERIFY | RESOLVED | §12; Appendix C; `VERIFY.md` v2.0 | The guide itself now has checksum first, `--locked`, correct temporal indexing and the correct anchor claim. Artifact completeness remains under R1-MUST15. |
| R3-N5, host time | RESOLVED | §6 | Both 58.6 s and 61.5 s are stated; the supplied log confirms 61.501 s. |
| R3-N6, guarantee comparison | PARTIALLY | §§9.2, 10 | Trusted-camera and substitution wording improved. Relay resistance remains overstated, and P1 already assumes the correct projected pattern, making P2 redundant. |
| R1-MUST1, temporal indexing | PARTIALLY | §§3.1, 5.2, 9.2 | The final program checks \(t-1\rightarrow t\) correctly. The formal unpredictability claim remains defective. |
| R1-MUST6, formal security | PARTIALLY | §9 | Assumptions and scoped theorems are a major improvement. A8 is not a proper game, and Lemma 3a should be phrased computationally rather than as mathematical impossibility. |
| R1-MUST8, memo binding | RESOLVED | §9.3 | The argument now routes through two openings of the extracted commitment and never assumes AEAD key commitment. |
| R1-MUST11, ML reporting | PARTIALLY | §8 | Seed integers and most configurations are supplied. The proved seed-3 whole-frame model still lacks a complete standalone training/checkpoint record; coupling selection and parity are misdescribed. |
| R1-MUST12, untouched coupling test | PARTIALLY | §§8.1, 11 | The absence is disclosed. The sealed 288-row session remains unscored. |
| R1-MUST13, realness coverage | PARTIALLY | §§8.1, 11 | Class 2 is absent, Class 4 outstanding, one forger seed completed, and sequence-level samples remain six per side. |
| R1-MUST14, reproducible proved build | PARTIALLY | §§7.3, 11–12 | Path remapping exists. There is still no second-machine byte reproduction and no proof under the final ELF. |
| R1-MUST15, artifact | PARTIALLY | §12; Appendix C | The guide and source digest improved. The supplied directory is missing 30 of 49 manifest entries, plus DOI, licence, locked environment and independent verification log. |
| R1-MUST16, chain reconstruction | PARTIALLY | §§9.5, 12 | The 260-row prefix is supplied and consistent. The complete 712-row chain and global-chain proof remain absent. |
| R1-SHOULD7, related work | PARTIALLY | §10 | The comparison is substantially better. It still implies P1+P2 exclude optical relay, which they do not. |

## 2. New findings

### R4-N1, MUST-FIX: A8 is sufficient only because it assumes Theorem 2’s conclusion

[A8](<packet>/paper.md:559) is not well-defined as a cryptographic assumption:

- No experiment samples a key, random oracle, target round or predecessor record.
- “Negligible” has no security parameter.
- The ordering of commitment, oracle access, early output and signature release is prose rather than a game.
- For the literal fixed deployed instance, a uniform or non-uniform program can hard-code the fixed \(E_t\), so the universal-PPT formulation is false.
- Under the intended random-oracle ensemble it need not be false, but that ensemble is never formally defined.

It is not logically circular in the narrow sense. It is substantively tautological: A8 says an adversary cannot output the exact pattern early; [Theorem 2](<packet>/paper.md:580) concludes precisely that by “A8 applied to that witness.”

Thus A8 is sufficient, but only vacuously. Either provide a two-stage challenge game and reduction, including renderer min-entropy, or label early pattern unavailability as an application assumption and demote Theorem 2 to an immediate conditional consequence.

### R4-N2, MUST-FIX: the supplied verification bundle fails its first command

`sha256sum -c SHA256SUMS` finds 49 entries but cannot read 30. Missing items include both raw and framed proofs, both public-value binaries, both manifests, both ELFs, the entire `final_relation/` directory and the standalone verifier.

The 19 present entries verify. The source-list file itself has the advertised digest `e45be3c0…37a2`, and the five supplied Rust files match their entries. That does not make the advertised procedure executable.

### R4-N3, MUST-FIX: integer parity is being confused with float-model parity

The corrected coupling result is exact:

- paired matched \(>\) crossed: 72/72;
- pooled inversions: 9/5,184, no ties;
- AUROC: \(5175/5184=0.9982638888888888\);
- minimum matched: \(-1{,}892{,}996\);
- positive crossed: \(601{,}379\) and \(20{,}290{,}152\);
- eight matched scores are at or below the maximum crossed score.

However, [§8.1](<packet>/paper.md:452) says the integer model reproduces “the float numerators bit for bit.” The parity record establishes 144/144 agreement between native Rust and a Python implementation of the quantized artifact. The float checkpoint reports floating-point logits and happens to give the same AUROC. It has no integer numerators to reproduce bit for bit.

Also disclose the source record’s `selection_performed:false`, `resolution_winner:null`, and `candidate_not_frozen`. Calling the 72 rows architecture-selection data does not prove that r32 won a recorded selection.

### R4-N4, SHOULD-FIX: the physical corollary assumes its key empirical conclusion

[P1](<packet>/paper.md:595) assumes that the frame is sensor output of a scene lit by the exact pattern scored by the discriminator. P2 then says a high score cannot arise under another pattern. P2 is redundant.

P1 should assert genuine sensor acquisition only. P2 should supply the inference that the displayed illumination was \(E_t\). Even then, a live optical relay of a remote or displayed scene can satisfy both, so §10 must not say the pair excludes relay or authenticates scene origin.

### R4-N5, editorial record drift

The paper still identifies itself as “second referee round” at [paper.md:4](<packet>/paper.md:4). It reports three audits at [paper.md:92](<packet>/paper.md:92), despite the three manuscript referee rounds in addition to the earlier source audits. Its log says the collision parenthetical was removed, but it remains. These are small individually and collectively show that the narrative record is no longer mechanically maintained.

## 3. Numeric check of §§6–8

| Claim | Check |
|---|---|
| Row-96 final instructions 4,149,712,293 | Exact |
| Listed regions plus “other” | Exact sum, delta zero |
| Renderer share 51.6% | 51.596851% |
| Raw hash share 16.9% | 16.934086% |
| Preprocess 10.1% | 10.146374% |
| Pose 8.3%; coupling 8.2% | 8.311808%; 8.169183% |
| Global committed-digest chain estimate | 1,625,935,276, exact |
| Rehashing 712 frames | 500,333,669,536, exact |
| Row-96 host execution | Log confirms 61.501 s; state records first run 58.641 s |
| Row-72 final instructions 4,148,971,330 | Matches `PINS_final.json` and state |
| Proof times | 769,575 ms = 12:49.6; 850,972 ms = 14:11.0 |
| Box cost | 47 min × $1.99/h = $1.56 |
| Statement sizes | Old decoded statements 1,085 B; final decoded statements 1,101 B |
| Row-96 final statement | Log hex is 1,101 B and hashes exactly to `777bbf…cb26a` |
| Cue delays | Row 96: 4.932 s; row 72: 14.932 s, exact |
| Pose split | 461 train, 116 evaluation, zero overlap |
| Pose accuracy/agreement | Integer 101/116; float 102/116; agreement 114/116 |
| Temporal aliasing | 112/116 at distance one; all within two |
| Robustness counts | All five-arm counts, medians and ranges match |
| Temporal separation | 290/287 rows and every per-seed count match |
| Direction gaps | 10.82, 8.23, 19.79 and 13.98 points |
| Coupling split | 424 fit, 72 selection; minimum donor distance 12 |
| Coupling statistics | Exact as listed above |
| Coupling parity | 144/144 quantized Python/native parity; float-parity wording unsupported |
| Realness | Reported numbers and limitations match `realness_results.md` |
| Seeds | `[None,1,2,3,4]` and `(None,1,2)` match the supplied scripts |

`paper.md`, `state.md`, `PINS_final.json` and the decoded final statements otherwise agree on final proof status, ELF, vkey, statement hashes, rounds, scores, verdicts, transaction, block and trapdoor flag. `VERIFY.md` now agrees with the paper about temporal indexing and the anchor.

## 4. Strongest and weakest claim

The strongest established claim remains the two ceremony-1 Groth16 proofs: under one pinned key, they bind execution of the earlier byte-level relation, including the networks, session membership, Zcash inclusion, memo decryption and receipt opening. They do not prove the corrected preceding-round leg.

The weakest claim is Theorem 2. Its conclusion is currently an ill-defined assumption repeated as a theorem. The broadest false factual implication is that P1 and P2 exclude optical relay.

## 5. Recommendation

For IEEE S&P or CCS: **reject this cycle, encourage resubmission after major revision**. The engineering is serious and the disclosure unusually candid. The final relation remains unproved, its temporal theorem rests on a malformed assumption, and the supplied artifact cannot execute its first verification step.

For Section 8 as diagnostics inside a systems paper: **minor revision**. Retain it. Correct the float/PTQ parity statement, disclose the unresolved selection status, and make explicit that these diagnostics do not discharge P2. No new ML experiment is required if the section remains strictly diagnostic.

Minimum changes needed to reach minor revision overall:

| Required change | Evidence that would satisfy me |
|---|---|
| Repair or demote Theorem 2 | A formal two-stage game with sampled experiment, security parameter, query bounds and predecessor commitment, plus a renderer entropy argument; alternatively, call A8 an unproved application premise and remove Theorem 2 as a claimed contribution. |
| Prove the final relation | 356-byte proof(s) over the 1,101-byte statements under vkey `0x0019d16c…0490`, cold verification, tamper controls and exact manifests. |
| Complete reproducibility | Second-machine byte-identical final ELF build and an archived build log. |
| Deliver an actual artifact | All 49 manifest files present, `sha256sum -c` clean, verifier run clean, DOI, licence, locked environment and independent verification log. |
| Reconcile coupling evidence | State quantized Python/native/guest parity accurately; disclose `selection_performed:false`; provide the complete seed-3 whole-frame training record. |
| Repair physical scope | Separate genuine acquisition from pattern inference and withdraw relay-exclusion language. |
| Clean the record | Correct referee-round metadata, audit count, source paths and the false “parenthetical removed” log entry. |

The sealed 288 rows, global-chain proof, Class 2 and Class 4 may remain future work only while the manuscript makes no empirical security claim from their absence.

— BOSUN ⚓
