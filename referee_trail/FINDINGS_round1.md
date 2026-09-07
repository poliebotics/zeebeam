Hoy.

# REFEREE REPORT

## MUST-FIX

1. **The central temporal lower-bound claim has an off-by-one hole.** Sections 4.3 and 4.6 define \(E_t=\mathrm{render}(S_t)\), expose frame \(t+1\) while showing \(E_t\), and only incorporate drand round \(r_t\) into \(S_{t+1}\). Yet the proof scores raw row \(t\) against \(E_t\). An operator can acquire or synthesize raw row \(t\) before learning \(r_t\), then obtain \(r_t\), compute \(S_{t+1}\), and prove the row. Thus Sections 1, 3, 9.1, and 13 do not establish that the proved frame was captured after the named round. This is the first attack a skeptical reader will make.

2. **The public statement does not disclose or directly bind the claimed drand round or value.** Section 7.3 reports rounds 31,521,620 and 31,521,616, but bytes 832–876 contain only a verification flag and Quicknet chain hash. The rounds occur in `PINS.json`, not in either decoded statement. A verifier cannot derive the claimed schedule check from the proof statement alone. Sections 4.6, 5.1, 7.3, 9.1, and 12 must either expose round, value, and signature or provide an authenticated chain-log opening sufficient to recompute the committed root.

3. **The proof establishes computation over supplied bytes, not a physical camera response.** A malicious operator controlling the rig and software can inject synthetic Bayer bytes, substitute the projector, relay another optical scene, or compromise the camera driver. No trusted sensor path or empirical argument excludes these attacks. Therefore “a recording that scores well must have been made after the value became public” in Sections 1 and 9.1 is unsupported. Define the result as byte-level execution binding unless additional hardware or acquisition assumptions are explicit and defended.

4. **Merkle membership does not prove global session chronology.** Sections 3, 4.6, 5.2, and 9.4 verify one local transition and one leaf’s membership in an operator-generated root. They do not verify all 712 transitions, consistency between every row, or that \(S_0\), \(S_N\), the prefix receipt, and the committed leaves form one valid chain. A prover can commit a tree containing independently manufactured or spliced leaves. The claimed resistance to reordering and splicing needs a global-chain proof or substantially narrower wording.

5. **The chameleon-hash upper bound is operationally vacuous against the party holding the trapdoor.** Sections 5.4 and 9.2 honestly encode the disjunction, but then overstate its meaning. Proving knowledge of the trapdoor merely proves that the anchor does not bind a unique record. A normal collision-resistant timestamp would bind an opening; the trapdoor holder can generate an alternate post-block opening. Later trapdoor publication lets anyone equivocate and further weakens custody attribution. Present this as a receipt or inclusion observation, not an adversarial upper-bound guarantee.

6. **There is no formal security theorem, game, or reduction.** Section 9 gives prose intuitions where a top cryptography venue requires explicit statements. At minimum, enumerate and use consistently:

   - SP1 execution or knowledge soundness, recursion, Groth16 soundness, Fiat-Shamir, and CRS assumptions.
   - BLS unforgeability, threshold non-early-release assumptions, and beacon timing.
   - Collision or second-preimage assumptions for each hash use.
   - Sinsemilla binding.
   - Baby Jubjub discrete-log and subgroup assumptions.
   - ChaCha20-Poly1305, KDF, and multi-key behavior.
   - Zcash consensus, finality, and reorganization assumptions.
   - Correctness of the guest, compiler, and pinned verification key.

7. **The threat model is materially incomplete and internally inconsistent.** Sections 3 and 9 say the operator may control everything, but the conclusions require trustworthy physical acquisition. Explicitly name exclusions covering sensor/driver compromise, digital raw injection, projector substitution, optical relays, adaptive model attacks, model or training-data poisoning, beacon threshold corruption or early share leakage, network-delay manipulation, Zcash reorganization or consensus attacks, verification-key substitution, compromised verifier distribution, key compromise, and side-channel leakage.

8. **The memo key-commitment argument is incomplete.** Section 5.3 treats `cmx` as though it directly commits the 512-byte memo and reduces alternate-key acceptance immediately to a Sinsemilla second preimage. `cmx` commits note fields, not the plaintext memo, and ChaCha20-Poly1305 is not generally key committing. A valid proof must connect the recipient key, diversified address, `epk`, shared-secret derivation, note commitment, and authenticated decryption. Also distinguish collision resistance from second-preimage resistance when the prover chooses the original transaction.

9. **The ML evidence is diagnostic, not security evidence.** Sections 8 and 9.1 rely on learned separation for the physical inference, while Section 8.3 says proof value does not depend on the models. Only byte-level execution binding is model-independent. The liveness or physical-response conclusion depends entirely on model validity and must be stated separately.

10. **The pose split is severely time aliased.** Section 8.2 calls the evaluation held out, but the audit shows one subject, one take, and one rig, with 112 of 116 evaluation rows exactly one frame from a same-class training row and all 116 within two frames. Class and time are nearly synonymous. Background-only performance, median 60/116 with range 17–73, confirms the confound. Report this as an interleaved same-take diagnostic, not generalization.

11. **Standard ML reporting is absent.** Section 8 must provide exact split construction and identities, class counts, training seeds, optimizer, loss, learning-rate schedule, stopping rule, preprocessing, model-selection procedure, seed dispersion, uncertainty intervals, and the independent sampling unit. With one take and one subject, conventional frame-level confidence intervals would be pseudo-replication.

12. **The coupling model appears to lack an untouched test set.** Section 8.1 says 712 development rows were used for training and a 72-row set for architecture selection, then reports that same set’s performance. Architecture-selection data cannot serve as confirmatory test data. The supplied review materials also do not substantiate the reported 72/72, AUROC 1.000, or 144-bit parity results. Publish split identities and evaluate once on an untouched session or take.

13. **The realness study is substantially underpowered and incomplete.** Section 8.2 does not disclose that only one forger-training seed was completed, versus three preregistered; each evaluation seed uses only six real and six forged subsequences with five clusters per side; and the three runs reuse the same underlying splits. The adaptive Class 4 attack remains outstanding, while the recorded \(\ell_2\) black-box diagnostic found no margin at \(\epsilon=16\) after 1,000 queries. The manuscript must report these limitations and reconcile the artifact-label contradiction.

14. **The published proof is not tied to a reproducible build.** Sections 9.5 and 12 say the guest sources and build recipe are pinned, but `PINS.json` lacks a digest of the complete guest source tree and build recipe. The second build was 152 bytes longer because of 32 embedded paths. The remapped-path build has a different, unproved verification key. A reproducible build is not achieved until the reproducible ELF itself is proved.

15. **The verification package does not meet artifact-evaluation expectations.** `VERIFY.md` uses `cargo build --release`, not `--locked`, and does not begin by checking a signed manifest or `sha256sum -c`. The reviewed directory does not contain the two proof artifacts or verifier needed to execute the procedure. A credible artifact needs an immutable archive and DOI, license, complete sources, locked environment or container, exact CPU/GPU/toolchain metadata, automated statement decoding and verification, expected outputs, and an independent-machine verification log.

16. **The release manifest is insufficient to validate the claimed chain semantics.** Sections 5.2, 9.4, and 12 require the full chain log or authenticated openings, prefix receipt, witness-generation specification, and enough source data to reconstruct the claimed root. Privacy-sensitive omissions must be listed explicitly together with the resulting unverifiable claims.

17. **The Zcash protocol citations and network semantics are wrong or ambiguous.** Section 5.5 calls Ironwood an “NU7-era successor,” but current ZIP-229 assigns Version 6/Ironwood to NU6.3. Section 5.3 attributes Sapling/Orchard note decryption to protocol §4.19, while the current specification places it in §4.20. `PINS.json` also commits the receipt/prefix network string as `zcash-testnet` while the anchor is described as Zcash mainnet. Explain or correct this cross-network semantic mismatch. [ZIP-229](https://zips.z.cash/zip-0229), [Zcash protocol specification](https://zips.z.cash/protocol/protocol.pdf).

18. **Several exact numerical claims disagree with the state of record.** Most importantly, Section 7.1 gives 421,045,347 preprocessing instructions rather than 421,045,031; gives 14,811,293 typed-tile-root instructions rather than the recorded 14,810,641; and Section 7.2 reports approximately 49 minutes of box time where the state records approximately 47 minutes. Section 13’s “thirteen minutes each” hides the measured 12:50 and 14:11 spread.

## SHOULD-FIX

1. **Reframe the title and headline contribution.** The title oversells “a camera’s response” and implies that the physical event itself is proved. It also obscures that there is one proof per selected row.

   Strongest defensible claim: “Under one pinned SP1 verification key, two concrete proofs attest execution of the implemented byte-level relation, including raw-frame hashing, integer model inference, Zcash transaction and header computations, memo decryption, and the declared chameleon-opening checks.”

   Weakest claim: “The proved drand round lower-bounds the capture time of the proved camera frame.”

   A defensible title would be: “A zkVM Proof of a Beacon-Seeded Capture-Log Relation with Learned Scores and a Zcash Receipt.”

2. **Separate proof size from total verifier input.** Sections 7.3 and 13 emphasize a 356-byte proof, but verification also consumes a 1,085-byte public statement and a 32-byte verification-key hash; the framed artifacts are 2,779 and 2,780 bytes. Report all four quantities together.

3. **Do not claim that replacing the renderer would “halve the proof” without qualification.** Section 11 shows the current renderer consumes roughly 52% of guest instructions. Removing that cost could roughly halve instruction count; replacing it with an unspecified cheaper generator does not imply the same reduction.

4. **Correct the privacy inventory.** Section 9.6 says no chain digests other than the session root and beacon chain hash are public, but the statement also exposes the authority-manifest SHA-256, chain-log BLAKE3 digest, context digest, ordered-tree root, \(S_0\), and \(S_N\).

5. **Disclose all negative-control history compactly.** Section 7.4 correctly reports the final controls, but the control that initially passed before a fix should remain prominently described as part of the development history.

6. **Clarify the AI-audit attribution.** Section 7.4 describes “GPT-5.6 Sol at high effort,” while the acknowledgements refer to “GPT-5.6 Ultra.” Use the actual model and effort labels consistently and do not treat model review as independent validation.

7. **Expand related work on trusted capture and active illumination.** Sections 2 and 8 need comparison against hardware-rooted capture, active illumination, replay/relay resistance, face anti-spoofing, and sensor attestation, not only generic provenance and zkML.

8. **Report the public inputs in a verifier-oriented table.** Section 5 currently describes byte ranges, but readers need field names, encodings, endianness, semantics, and whether each field is cryptographically constrained or merely metadata.

## NICE

1. Replace the jocular “ZK All the Things” title for archival publication.

2. Move the byte layout and instruction accounting to tables; Sections 5 and 7 are unnecessarily difficult to audit in prose.

3. Define “row,” “frame,” “emission,” “capture,” and “response” once and use them consistently. Their present interchangeability helps conceal the temporal indexing problem.

4. Avoid “exact,” “complete,” and “end-to-end” unless the scope is immediately qualified as execution of the implemented relation.

# Numerical audit

Sources: [state.md](<packet>/state.md), [PINS.json](<packet>/PINS.json), [row 096](<packet>/row_096_statement.json), [row 072](<packet>/row_072_statement.json), [pose audit](<packet>/pose_nocrop_result.md), and [realness audit](<packet>/realness_results.md).

| Paper value | Source value | Result |
|---|---:|---|
| Statement 1,085 B | 1,085 B for both rows | OK |
| Raw proof 356 B | 356 B for both rows | OK |
| Verification-key hash 32 B | 32 B | OK |
| Framed proof 2,779/2,780 B | 2,779/2,780 B | OK |
| 712 session rows | 712 | OK |
| Merkle depth 10 | 10 | OK |
| Prefix through row 260 | 260 | OK |
| Raw frame 24.5 MB | 24,472,000 B | OK, rounded |
| Sensor 5,320 × 4,600 | 5,320 × 4,600 | OK |
| Projector 1,920 × 1,080 | 1,920 × 1,080 | OK |
| Session 301 s | Recorded as 301 s | OK |
| 66 beacon rounds | 66 | OK |
| 10.79 rows/round | 712/66 = 10.79 | OK |
| Block 3,456,294 | 3,456,294 | OK |
| Row 096 round 31,521,620 | PINS: 31,521,620 | OK externally; absent from statement |
| Row 072 round 31,521,616 | PINS: 31,521,616 | OK externally; absent from statement |
| Membership bytes 0–352 | 0–352 | OK |
| Coupling bytes 352–696 | 352–696 | OK |
| Pose bytes 696–832 | 696–832 | OK |
| Beacon bytes 832–876 | 832–876 | OK |
| Zcash bytes 876–948 | 876–948 | OK |
| Anchor bytes 948–1,085 | 948–1,085 | OK |
| Row 096 instructions 4,117,367,985 | 4,117,367,985 | OK |
| Renderer 2,141,117,563 | 2,141,117,563 | OK |
| Raw unpack 697,810,890 | 697,810,890 | OK |
| Preprocessing 421,045,347 | 421,045,031 | **Mismatch: +316** |
| Pose 344,916,171 | 344,916,171 | OK |
| Coupling 338,997,663 | 338,997,663 | OK |
| Anchor 100,431,532 | 100,431,532 | OK |
| Memo 15,394,521 | 15,394,521 | OK |
| Typed tile root 14,811,293 | 14,810,641 recorded | **Mismatch: +652** |
| Drand 4,904,593 | 4,904,593 | OK |
| Transaction ID 235,693 | 235,693 | OK |
| Row 096 proving 12:50 | 769,575 ms = 12:49.6 | OK |
| Row 072 proving 14:11 | 850,972 ms = 14:11.0 | OK |
| “13–14 min” | 12:50 and 14:11 | Approximate; widen or give exact values |
| “Thirteen minutes each” | 12:50 and 14:11 | **Mismatch/over-rounding** |
| Setup about 21 s | 21.401/21.356 s | OK |
| Verification below 1 s | 505/366 ms | OK |
| Box time about 49 min | About 47 min, 05:30–06:17 | **Mismatch** |
| A100 40 GB | A100 40 GB | OK |
| Peak VRAM 26.5 GB | 26.5 GB | OK |
| Host RAM 114 MB | 114 MB | OK |
| 30 vCPU, 216 GiB | 30 vCPU, 216 GiB | OK |
| Cost $1.99/hour | $1.99/hour | OK |
| Proved ELF 1,160,904 B | 1,160,904 B | OK |
| Second build +152 B | +152 B | OK |
| 32 path occurrences | 32 | OK |
| Remapped ELF 1,159,872 B | 1,159,872 B | OK |
| 0 home-path strings | 0 | OK |
| Remapped registry/tree paths 31/12 | 31/12 | OK |
| 30 negative controls | 30 enumerated | OK |
| Row 072 cue 14.9, verdict 0 | 14.9, verdict 0 | OK |
| Row 096 cue 4.9, verdict 2 | Verdict 2 confirmed; cue 4.9 not independently present in supplied state | Partly unverifiable |
| Row 096 score 56,835,791/4 | 56,835,791/4 | OK |
| Row 072 score 56,821,525/4 | 56,821,525/4 | OK |
| Pose parity 577/577 | 577/577 | OK |
| Six arms, 116 examples, zero logit deviation | Six, 116, zero | OK |
| Integer/float 114/116; accuracy 101/102 | Not recorded in supplied supporting sources | Unverifiable |
| Calibration set 461 | Not recorded in supplied supporting sources | Unverifiable |
| Pose crop 113/116, perfect 109, wide 105, full 102 | 113, 109, 105, 102 | OK |
| Background-only median 60, range 17–73 | 60, 17–73 | OK |
| Chance 10.5%, majority 17/116 | 10.5%, 17/116 | OK |
| Permutation median 11, max 16 | 11, 16 | OK |
| 112/116 adjacent training frames | 112/116 | OK |
| All 116 within two frames | 116/116 | OK |
| Coupling 72/72, AUROC 1.000, 144 parity cases | Not present in supplied supporting sources | Unverifiable |
| Realness three evaluation seeds | Three seeds | OK, but same splits reused |
| Six real/six forged subsequences per seed | 6/6 | OK |
| Five clusters per side | 5/5 | OK |
| Fake pass 0%, reject 0%, AUROC 1.000 | Recorded as stated | OK, very small sample |
| Bootstrap interval [1.000, 1.000] | [1.000, 1.000] | OK but misleadingly narrow |
| Class 3: nine runs, 6 real/18 forged | 9, 6/18 | OK |
| One forger seed | One completed vs three preregistered | Omission in paper |
| Class 4 \(\epsilon=16\), 1,000 queries | 16, 1,000 | Omitted adverse diagnostic |
| Baby Jubjub \(a=168700,d=168696\) | Same | OK |
| Receipt 853 B | 853 B | OK |
| Prefix JSON 603 B | 603 B | OK |
| Prefix bytes 134,188 B | 134,188 B | OK |
| Renderer about 52% | 2,141,117,563 / 4,117,367,985 = 52.0% | OK |
| “Halve the proof” with cheaper generator | Only zero renderer cost would approximately halve instructions | Overstated |

# Corrected references and `[verify]` audit

1. **C2PA.** *C2PA Technical Specification*, cite the exact version and access date. **Real; attribution broadly correct.** The standard authenticates provenance assertions but does not establish that depicted content is true. [Official specifications](https://spec.c2pa.org/specifications/).

2. **Assa Naveh and Eran Tromer.** “PhotoProof: Cryptographic Image Authentication for Any Set of Permissible Transformations.” *IEEE Symposium on Security and Privacy*, 2016, pp. 255–271. **Real; correct after adding venue and pages.** [Project page](https://cs-people.bu.edu/tromer/photoproof/).

3. **Dan Boneh, Ben Lynn, and Hovav Shacham.** “Short Signatures from the Weil Pairing.” *ASIACRYPT 2001*, LNCS 2248, pp. 514–532. **Real; correctly attributed.**

4. **Ewa Syta et al.** “Scalable Bias-Resistant Distributed Randomness.” *IEEE Symposium on Security and Privacy*, 2017. **Real; correctly attributed.**

5. **drand.** *Protocol Specification* and Quicknet deployment documentation, cite version/access date. **Real; Quicknet chain attribution is correct.** [Protocol](https://docs.drand.love/docs/specification/), [Quicknet deployment](https://docs.drand.love/blog/2023/10/16/quicknet-is-live/).

6. **A. Faz-Hernandez, S. Scott, N. Sullivan, R. S. Wahby, and C. A. Wood.** *Hashing to Elliptic Curves*. RFC 9380, August 2023. **Real; correct.** [RFC 9380](https://www.rfc-editor.org/rfc/rfc9380.html).

7. **Jean-Philippe Aumasson et al.** “BLAKE3: One Function, Fast Everywhere,” 2020 specification. **Real; not an archival venue publication.** Cite the exact specification revision. [Specification](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.tex).

8. **Zcash Protocol Specification.** Cite the exact revision. Note-encryption material is in current §4.20, not §4.19. **Real; section attribution in the manuscript is incorrect.** [Specification](https://zips.z.cash/protocol/protocol.pdf).

9. **ZIP 224.** “Orchard Shielded Protocol.” **Real; Orchard-specific.** [ZIP 224](https://zips.z.cash/zip-0224).

10. **ZIP 244.** “Transaction Identifier Non-Malleability.” **Real; relevant to pre-Version-6 transaction identifiers but insufficient alone for Ironwood.** [ZIP 244](https://zips.z.cash/zip-0244).

11. **ZIP 229.** “Version 6 Transaction Format.” Draft, 2026. **Required correction for Ironwood; identifies NU6.3, not NU7.** [ZIP 229](https://zips.z.cash/zip-0229).

12. **Barry WhiteHat, Marta Bellés, and Jordi Baylina.** “ERC-2494: Baby Jubjub Elliptic Curve,” January 2020, status Stagnant. **Real; “iden3” alone is not the correct bibliographic attribution.** [ERC-2494](https://eips.ethereum.org/EIPS/eip-2494).

13. **Jens Groth.** “On the Size of Pairing-Based Non-Interactive Arguments.” *EUROCRYPT 2016*, Part II, LNCS 9666, pp. 305–326. **Real; correctly attributed.**

14. **Shai Haber and W. Scott Stornetta.** “How to Time-Stamp a Digital Document.” *Journal of Cryptology* 3(2), 1991, pp. 99–111. **Real; correctly attributed.**

15. **Hugo Krawczyk and Tal Rabin.** “Chameleon Signatures.” *NDSS 2000*. **Real; correctly attributed.** [NDSS entry](https://www.ndss-symposium.org/ndss2000/chameleon-signatures/).

16. **Tianyi Liu, Xiang Xie, and Yupeng Zhang.** “zkCNN: Zero Knowledge Proofs for Convolutional Neural Network Predictions and Accuracy.” *ACM CCS 2021*. **Real; manuscript should include the full title and venue.** [IACR ePrint 2021/673](https://eprint.iacr.org/2021/673).

17. **Nils Fleischhacker et al.** “Scaling Up Trustless DNN Inference with Zero-Knowledge Proofs.” arXiv:2210.08674, 2022; presented at the RegML workshop at NeurIPS 2023. **Real; not a main-track NeurIPS paper.** [arXiv](https://arxiv.org/abs/2210.08674), [OpenReview](https://openreview.net/pdf?id=GjNRF5VTfn).

18. **Minsoo Kang, Tatsunori Hashimoto, Ion Stoica, and Yi Sun.** “ZK-IMG: Attested Images via Zero-Knowledge Proofs to Fight Disinformation.” arXiv:2211.04775, 2022. **Real; no verified archival venue should be implied.** [arXiv](https://arxiv.org/abs/2211.04775).

19. **Franziska Datta et al.** “VerITAS: Plaintext Encoders for Zero-Knowledge Proofs of Neural Network Inference.” *IEEE Symposium on Security and Privacy*, 2025, pp. 4606–4623. **Real; a 2024-only attribution misses the final venue/year.** [IEEE S&P 2025 accepted papers](https://sp2025.ieee-security.org/accepted-papers.html).

20. **Succinct Labs.** *SP1 v6.4.0*, release and pinned commit, 2026. **Real; cite the exact release/commit as software, not as a research paper.** [SP1 v6.4.0](https://github.com/succinctlabs/sp1/releases/tag/v6.4.0).

21. **EZKL.** Software documentation and exact pinned release/commit. **Real software; no paper or venue should be invented.** [Documentation](https://docs.ezkl.xyz/).

22. **OpenTimestamps.** Protocol/software project, cite exact client version and access date. **Real; not a peer-reviewed paper.** [Project site](https://opentimestamps.org/).

23. **Candice R. Gerstner and Hany Farid.** “Detecting Real-Time Deep-Fake Videos Using Active Illumination.” *CVPR Workshops*, 2022, pp. 53–60. **Real; suitable replacement for the manuscript’s vague active-illumination placeholder.** [CVPR paper](https://openaccess.thecvf.com/content/CVPR2022W/WMF/html/Gerstner_Detecting_Real-Time_Deep-Fake_Videos_Using_Active_Illumination_CVPRW_2022_paper.html).

24. **Zitong Yu et al.** “Deep Learning for Face Anti-Spoofing: A Survey.” arXiv:2106.14948, 2021. **Real; useful background, but not evidence for this system’s security.** [arXiv](https://arxiv.org/abs/2106.14948).

## Overall verdict

**Recommendation: reject in present form, with encouragement to resubmit after major reconstruction.** The implementation effort and two concrete zkVM proofs are substantial, and the byte-level relation appears unusually well audited. However, the paper’s headline security claim fails because the proved frame is not temporally downstream of the proved beacon round, the public statement does not expose that round, and neither Merkle membership nor learned classifiers establish physical capture chronology. The security section lacks formal games and reductions, the ML evaluations are time-confounded and underreported, and the supplied artifact cannot reproduce the proved ELF or independently run verification. A viable revision should narrow the claim to execution binding, repair the frame/round indexing, authenticate the session chain globally, reproduce and re-prove the guest, and replace the current ML claims with independently split multi-session evidence.

— BOSUN ⚓
