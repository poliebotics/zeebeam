Hoy.

# Round-2 referee report

## 1. Disposition of round-1 findings

### MUST-FIX

| # | Disposition | v2.0 response | What remains |
|---:|---|---|---|
| 1 | PARTIALLY | §§3.1, 5.2, 7.4, 9.2 add and verify the \(t-1\) transition. [`verify_previous_advance`](<packet>/august.rs:340) checks \(\sigma_{t-1}\), derives \(v_{t-1}\), and recomputes \(S_t\). | The final relation has only been executed, never proved. Theorem 2 also lacks the unpredictability assumption needed for the temporal corollary; see N2. |
| 2 | PARTIALLY | §§5.1–5.2 publish \(r_{t-1}\) and \(r_t\) in `ZBAUGST4`; the guest appends both. | Neither supplied decoded statement contains them. Both are old 1,085-byte `ZBAUGST2` statements. `PINS.json` is also stale. |
| 3 | RESOLVED | §§3.3–3.4 and 9.2 restrict the cryptographic result to execution binding and state P1/P2 for the physical interpretation. | The stray disclosure claim in §9.7 still calls a beacon round a capture time. |
| 4 | RESOLVED | §§4, 9.5 and 11 now claim one local transition and expressly disclaim global chain validity. | The proposed global-proof cost is seriously wrong; see N6. |
| 5 | PARTIALLY | §§1, 9.4 and 11 correctly say the commitment is vacuous against the trapdoor holder. | §9.4’s claim that it remains an upper bound “against a third party” is false when the operator retains the trapdoor; see N3. |
| 6 | PARTIALLY | §9 supplies A1–A7, two theorems and two propositions. | These remain informal sketches. Beacon unpredictability, hash-to-curve/random-oracle treatment, and the memo argument’s protocol lemma are missing or underspecified. |
| 7 | RESOLVED | §3.3 now names the relevant acquisition, model, beacon, chain, key-distribution, compromise and side-channel exclusions. | None material. |
| 8 | PARTIALLY | §§5.2, 9.3 now connect IVK, `epk`, key agreement, authenticated decryption, note reconstruction, `cmx`, and the memo. It correctly uses commitment binding rather than second-preimage language. | Proposition 3 still needs a lemma showing that two accepting IVKs yielding different memo bindings necessarily induce distinct note-commitment openings. A5 merely asserts deterministic fixed-key decryption. |
| 9 | RESOLVED | §§3.4, 8.3, 9.1–9.2 separate execution binding from learned physical interpretation. | None. |
| 10 | RESOLVED | §8.2 states the one-take temporal alias, 112/116 adjacency, background result and required shuffled repeated-cue capture. | None. |
| 11 | PARTIALLY | §8 now gives architectures, optimizers, losses, splits, seed counts, preprocessing and stopping details. | Actual pose seed values, classwise results, confusion matrices, complete model-selection procedure and an independent sampling analysis remain absent. Several hyperparameters cannot be checked from the supplied files. |
| 12 | PARTIALLY | §8.1 and §11 admit that the 72 rows are architecture-selection data and that no untouched test exists. `ml_splits.json` supplies exact row identities. | The sealed 288-row session remains unscored. The reported 72/72 and 144-value parity fixture is not supplied here. |
| 13 | PARTIALLY | §8.1 reports one forger seed, six-versus-six samples, shared splits, outstanding Class 4, and the adverse \(\ell_2\) diagnostic. | Class 2 and Class 4 remain open; the contradictory ROC sidecar recorded in `realness_results.md` is not mentioned or repaired. |
| 14 | PARTIALLY | §§7.3, 11–12 identify a path-independent final ELF and explicitly mark its proof as pending. | No second-machine byte reproduction and no proof under that ELF exist. |
| 15 | PARTIALLY | §12 now requires `--locked`, checksum-first verification, sources, decoder and expected outputs. | The supplied directory contains no proof binaries, verifier, `VERIFY.md`, `SHA256SUMS`, licence, DOI, immutable archive, container or independent verification log. |
| 16 | PARTIALLY | §§9.5, 11–12 disclose the missing global-chain guarantee and say the full chain log and `PREFIX.json` must be added. | They are still absent, so an artifact evaluator cannot reconstruct the root or chain semantics. |
| 17 | RESOLVED | §§2, 4 and 7.4 correctly place Version 6/Ironwood in NU6.3, cite §4.20, and explain that `zcash-testnet` is an immutable receipt label while the transaction is mainnet. | The inherited label remains poor protocol hygiene, but it is no longer misrepresented. |
| 18 | RESOLVED | §§6–7 use the current-ELF figures 421,045,347 and 14,810,611, report approximately 47 minutes, and give 12:50 and 14:11 separately. | The current component tracker log is not included in `state.md`, so some exact region figures remain unauditable from this bundle. |

### SHOULD-FIX

| # | Disposition | v2.0 response | What remains |
|---:|---|---|---|
| 1 | RESOLVED | The subtitle is exactly the defensible capture-log formulation, and the headline contribution is execution binding. | “ZK All the Things” remains unsuitable for archival publication, as already recorded under NICE. |
| 2 | PARTIALLY | Abstract and §§1, 7.1–7.2 disclose proof, statement and key-hash sizes; §7.1 gives framed sizes. | The four sizes are never presented together, and framed sizes exist only for the obsolete relation. |
| 3 | RESOLVED | §6 says removing the renderer would roughly halve instruction count and explicitly leaves generator quality empirical. | None. |
| 4 | RESOLVED | §9.7 lists the authority, chain, context, tree, state, model and anchor fields. | Its “capture time to within a few seconds” gloss is false; see N7. |
| 5 | RESOLVED | §7.4 and Appendix D retain the initially passing direction-byte control and the other development failures. | None. |
| 6 | RESOLVED | §7.4 and Acknowledgements consistently identify OpenAI GPT-5.6 through Codex at high reasoning effort and deny independence. | None. |
| 7 | PARTIALLY | §10 adds active illumination, anti-spoofing, hardware-rooted capture and C2PA. | It is still a paragraph of taxonomy rather than a meaningful comparison of relay, replay and trusted-sensor guarantees. |
| 8 | PARTIALLY | §5.1 provides the requested verifier-oriented table. | The table contains wrong and missing byte ranges; see N4. |

## 2. New findings introduced by v2.0

### N1. MUST-FIX: the paper claims a final proof that does not exist

The abstract says “we present a single succinct proof” of a relation containing both beacon signatures and a 1,101-byte statement. The conclusion likewise says “One proof ... with two beacon signatures.” Section 7.1 admits that both actual proofs use the earlier 1,085-byte relation without the previous-row leg, while the final re-prove is pending.

The final ELF has only execution results: 4,149,712,293 and 4,148,971,330 instructions. The two real proofs attest the old ELF and old key. This is the manuscript’s largest factual misstatement. The abstract and conclusion must distinguish “we implemented and executed the final relation” from “we proved the earlier relation.”

### N2. MUST-FIX: Theorem 2 lacks the assumption needed for its temporal conclusion

A4 says the signature cannot be forged or obtained before release. A3 gives collision resistance. Neither implies that \(E_t\), or even SHA-256\((\sigma_{t-1})\), is computationally unpredictable before release. Collision resistance is not unpredictability.

P1 and P2 do not close the gap: an adversary who predicts the exact \(E_t\) can genuinely project it and capture `raw_t` early; P2 excludes other patterns, not an early copy of the correct one. The theorem needs an explicit beacon-output unpredictability game and a reduction showing that predicting the rendered challenge is negligible, or a trusted sequencing assumption that the rig only obtains and displays \(E_t\) after verifying the released signature.

The phrase “the signature ... did not exist” should be “was computationally unavailable.”

### N3. MUST-FIX: Proposition 4’s third-party claim is wrong

Section 9.4 says the commitment “against a third party who did not hold td ... would be an upper bound.” The relevant question is whether anyone able to supply the claimed record held `td`. Here the operator did. A verifier without `td` can still be shown any alternate opening created by that operator and cannot identify the capture-time opening.

The commitment gives no record upper bound to any later verifier once the record custodian held the trapdoor. The only sound statement is the inclusion of the receipt bytes and existence of the demonstrated opening.

### N4. MUST-FIX: the public-statement table is not byte accurate

Section 5.1 has three concrete ABI errors:

| Defect | Evidence |
|---|---|
| Bytes 12–16 are called “statement length.” | The decoded statements call this `public_bytes_declared` and contain 876. The final guest appends Zcash and August blocks without rewriting it. |
| The session identifier is listed as 28–79 “zero-padded to 156.” | The 51-byte identifier occupies 28–79; padding through offset 156 is omitted. The field range should be 28–156 with length-driven semantics. |
| Bytes 348–352 are absent. | The membership prefix ends at 352 and coupling begins there, leaving four undocumented bytes. |

Thus the table does not describe every public byte despite making that claim.

### N5. MUST-FIX: `PINS.json` contradicts Appendix A and the final relation

[`PINS.json`](<packet>/PINS.json:1) still pins:

| Field | `PINS.json` | Final relation |
|---|---:|---:|
| Statement | 1,085 B, `ZBAUGST2` | 1,101 B, `ZBAUGST4` |
| ELF | 1,160,904 B, `5b3ebfcd…` | 1,171,416 B, `8bcadf53…` |
| vkey | `0x0092cba2…` | `0x0019d16c…` |
| Published rounds | only each row’s round out of band | both previous and current rounds in statement |

Appendix A says `PINS.json` contains the final pins. It does not. No supplied JSON decodes a 1,101-byte statement.

### N6. MUST-FIX: §9.5’s global-chain cost is dimensionally wrong

The paper claims about 700 million instructions for 712 advances, “dominated by raw hashes.” The measured full-frame hash is 702,715,828 instructions per row. Rehashing 712 frames therefore costs approximately:

\[
712 \times 702{,}715{,}828 \approx 500.3\text{ billion instructions}.
\]

If the proof accepts the logged 32-byte raw digests, those full-frame hashes disappear and cannot dominate anything. The 66 BLS estimate, roughly \(66\times4.9\) million \(=323\) million, is reasonable. The raw-hash estimate is off by roughly three orders of magnitude.

### N7. MUST-FIX: §9.7 falsely equates rounds with capture time

“Both beacon rounds (hence the capture time to within a few seconds)” contradicts §§3.4 and 9.2. Rounds supply scheduled release times. Even with P1/P2 they provide a lower bound, not a capture-time estimate or upper bound. A frame could be recorded minutes or days later.

### N8. SHOULD-FIX: §8’s “one take” scope contradicts its realness evidence

Section 8 opens with “All data are one subject, one rig, one take.” The realness note says the generated-fake study used held-out sessions d2 and v10 and a forger trained on April data. The one-take description is accurate for the pose and 712-row coupling diagnostics, not for all of §8.

### N9. SHOULD-FIX: the cost table silently omits 27,847,215 instructions

The listed §6 regions sum to 4,121,865,078, while the stated row-96 total is 4,149,712,293. The unlisted remainder is 27,847,215 instructions, about 0.67%. Add an “other/untracked” row or state that the table is partial.

### N10. SHOULD-FIX: the direction-asymmetry range is numerically wrong

Section 8.2 says 10 to 20 percentage points. The audited differences are 10.82, 8.23, 19.79 and 13.98 points. The actual range is approximately 8 to 20.

### Reference re-check

My round-1 attributions for the three papers named by the authors were wrong. The manuscript now has them right:

The two Kang papers are by Daniel Kang, Tatsunori Hashimoto, Ion Stoica and Yi Sun: [Scaling up Trustless DNN Inference](https://arxiv.org/abs/2210.08674), [ZK-IMG](https://arxiv.org/abs/2211.04775). VerITAS is by Trisha Datta, Binyi Chen and Dan Boneh: [IACR ePrint 2024/1066](https://eprint.iacr.org/2024/1066). Gerstner and Farid, pages 53–60, is also correct: [CVPR Workshops record](https://openaccess.thecvf.com/content/CVPR2022W/WMF/html/Gerstner_Detecting_Real-Time_Deep-Fake_Videos_Using_Active_Illumination_CVPRW_2022_paper.html). The RFC 9380 and ERC-2494 author lists are correct: [RFC 9380](https://www.rfc-editor.org/rfc/rfc9380.html), [ERC-2494](https://eips.ethereum.org/EIPS/eip-2494).

## 3. Numerical audit of Sections 6–8

### Section 6

| Claim | Check |
|---|---|
| SP1 6.4.0; patched `bls12_381` 0.8.0 tag 6.2.0; Orchard 0.15.3; note-encryption 0.4.2; `jubjub` 0.10.0; ark 0.4 | Consistent with code, state and `PINS.json`. |
| Four `const` initialisers changed | Consistent with `state.md`; patch itself is not supplied. |
| 4.9 million instructions per BLS verification | Correct rounding of 4,904,590 for the current row leg. |
| Row-96 total 4,149,712,293 | Exact match to newest `state.md`. |
| Renderer 2,141,120,868, 51.6% | Share arithmetic correct. Exact current tracker output is absent from `state.md`. |
| Raw hash 702,715,828, 16.9% | Share arithmetic correct; exact current tracker output absent. |
| Preprocess 421,045,347, including RGGB 229,464,788, 10.1% | The new value supersedes the older 421,045,031 state table. Exact current tracker log absent. |
| Pose 344,916,129, 8.3%; coupling 338,997,588, 8.2% | Shares correct; exact current values absent from the supplied state record. |
| August 126,039,518, 3.0%; memo 15,394,563, 0.4% | Shares correct; exact current values absent. |
| Typed root 14,810,611, 0.4% | New value supersedes 14,810,641. Exact current tracker log absent. |
| Previous advance 6,733,568, 0.2% | Exact match to state for row 96. |
| Row BLS 4,904,590; XOF 4,829,169; tx/header 235,697; membership 121,602 | Percentages correct. XOF is independently present in state; other final values lack their raw tracker record. |
| Renderer removal roughly halves instructions | Correct: it removes 51.6%. |
| All 144 parity numerators reproduced | Asserted in state, but the 144-value fixture is not supplied. |
| Region total | Incomplete by 27,847,215 instructions. |

### Section 7

| Claim | Check |
|---|---|
| A100-SXM4-40GB, 30 vCPU, 216 GiB, SP1 6.4.0 | Exact state match. |
| Setup 21.4 s for each | Matches 21.356 and 21.401 s. |
| Row 72 prove 850.97 s, 14:11 | Matches 850,972 ms. |
| Row 96 prove 769.58 s, 12:50 | Matches 769,575 ms. |
| Verification/control times 0.37 and 0.51 s | Matches 366 and 505 ms. |
| Host RSS 114 MB; VRAM 26.5 GB | Exact state match. |
| Framed sizes 2,780 and 2,779 B; raw proof 356 B | Exact state and PINS match. |
| Box approximately 47 minutes at $1.99/hour | Correct. Implied per-minute cost is about $1.56. |
| Actual statements 1,085 B; final statement 1,101 B | Correct. Only the 1,085-byte statements have proofs. |
| Old vkey and ELF `5b3ebfcd…`, 1,160,904 B | Exact PINS and statement match. |
| Cross-machine difference 152 B and 32 path occurrences | Exact state match. |
| Final ELF 1,171,416 B, stated SHA and vkey | Exact newest-state match; absent from `PINS.json`. |
| “Same statement bytes apart from added fields” | Reported by the 1,101-byte oracle executions, but no final decoded statement is supplied. |
| Three audits; second had three BLOCK findings | Consistent with audit history. |
| Row 96 verdict 2 and row 72 verdict 0 | Exact decoded-statement match. |
| Row 96/72 model-data roles | Exact `ml_splits.json` match. |
| Row 72 cue delay 14.9 s | Supported by state. |
| Row 96 cue delay 4.9 s | Still unsupported by the supplied state, JSON and ML notes. |

### Section 8

| Claim | Check |
|---|---|
| Coupling fit 424 rows; selection 72 rows | Exact `ml_splits.json` lengths; zero overlap. |
| Selection ranges 96–119, 320–343, 544–567 | Exact JSON match. |
| Minimum donor distance 12; seed 20260823 | Exact JSON match. |
| 72/72 ordering, AUROC 1.0, 144 parity values | State asserts reproduction of 144 numerators; underlying values are absent. |
| Separate sealed 288-row session | Consistent with the owner allocation, but absent from `ml_splits.json` and the supplied project state. |
| Coupling architecture and optimizer, batch 8, 200 epochs | Not checkable from the supplied files. |
| Generated study: three evaluation seeds, six real/six forged, five clusters per side, one forger seed | Exact `realness_results.md` match. |
| Generated fake pass 0, real rejection 0, AUROC 1.0, CI [1,1] | Exact match, with the stated small-sample warning. |
| Replay/splice six real/eighteen forged | Exact; the source additionally says nine runs. |
| Class 4 outstanding; \(\epsilon=16\), 1,000 queries, no margin | Exact match. |
| Pose 461 train, 116 evaluation, 577 annotated, 11 classes | Exact JSON match; train/evaluation overlap is zero. |
| Integer/float agreement 114/116; accuracies 101 and 102 | Recomputed exactly from `ml_splits.json`. |
| Five interleaved and three temporal initialisations | Matches the audited notes, though actual seed values are absent. |
| 112/116 at distance one; all within two | Recomputed exactly: 112 at distance one, four at distance two. |
| Background median 60/116, range 17–73; majority 17 | Exact pose-note match. |
| Temporal sample sizes 290 and 287; all five median rows | Exact temporal-note match. |
| Whole-frame seed counts 262/213/208 and 143/154/187 | Exact match. |
| Chance 0.091 | Correct rounding of \(1/11\). |
| Predeclaration lead 112 s | Exact audit-note match, correctly denied immutable-preregistration status. |
| Direction asymmetry 10–20 points | Incorrect; actual range is 8.23–19.79. |
| “All data ... one take” | Incorrect for the realness subsection. |

The supplied old statements are internally consistent: 1,085 bytes each, scores 56,835,791/4 and 56,821,525/4, pose outputs 2 and 0, identical session/anchor fields, and `ZBAUGST2`. They cannot validate any final-relation round field.

## 4. Strongest and weakest claim

The strongest claim is Theorem 1’s execution-binding claim, with one qualification that the paper currently evades: it is proved for the earlier relation and execution-tested for the final relation. Once ceremony 2 succeeds under the final key, this is a substantial, precise systems result.

The weakest claim is §9.7’s assertion that the published beacon rounds reveal “the capture time to within a few seconds.” They reveal scheduled beacon-release times. They establish neither capture nor temporal proximity.

## 5. Recommendation

For IEEE S&P or CCS: **major revision**. The core relation is technically serious and the round-1 reconstruction was substantial. Acceptance is premature because the claimed final proof does not exist, Theorem 2 lacks its crucial unpredictability hypothesis, Proposition 4 retains a false third-party statement, the public ABI table is wrong, the final pins are absent, and the artifact remains incomplete.

For a top ML venue evaluating Section 8 as diagnostics: **reject**. One temporally aliased pose take, no untouched coupling test, one forger seed, six sequence-level examples per side, no Class 2, no completed adaptive Class 4, and no independent subject/session/rig generalisation are insufficient even for a strong diagnostics contribution. Honest limitation language improves scientific conduct. It does not manufacture evidence.

— BOSUN ⚓
