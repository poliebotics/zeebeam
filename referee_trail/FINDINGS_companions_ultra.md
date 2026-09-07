---
audit: companions ultra
date: 2026-09-03
model: gpt-5.6-sol (codex exec, read-only, effort ultra)
---

Hoy.

Primary references: [paper.md](<packet>/paper.md:1), [FINDINGS_round10.md](<packet>/FINDINGS_round10.md:1), [VERIFY.md v2.5](<packet>/bundle/VERIFY.md:1), [FLEET_STATS.json](<packet>/bundle/all_rows/proofs/FLEET_STATS.json:1), [PINS_final.json](<packet>/bundle/PINS_final.json:1), [chain_manifest.json](<packet>/bundle/final_relation/chain/chain_manifest.json:1), and verifier [Cargo.toml](<packet>/bundle/verifier/standalone_verifier/Cargo.toml:1).

## 1. Defects in worked_examples.md

1. **Nit, line 6.** Quote: “`companion_to: zk_all_the_things_paper_draft.md v3.1`”. Replace with: “`companion_to: paper.md v3.1`”.

2. **Blocking, lines 34–37 and 60–66.** Quotes: “the verifier learns nothing about w beyond the fact that it exists”, “in the ZeeBeam system it is what keeps a 24.5-megabyte camera frame private”, and “A zero-knowledge proof proves it and reveals nothing else.” The manuscript assumes knowledge soundness, not an instantiated privacy theorem. Section 9.7 establishes only that the statement contains no pixels. Replace with:

   > Zero knowledge is the property that a proof transcript reveals no more than its public statement. This manuscript establishes execution binding under A1–A3 and separately records that the 1,101-byte statement contains no pixels. It does not state or prove a privacy theorem for this SP1 6.4.0 artifact. Do not claim that the proof keeps the frame private or reveals nothing else unless that theorem and its assumptions are added to the manuscript.

   Cite paper §§9.1 and 9.7.

3. **Must-fix, lines 39–41 and 54–58.** Quotes: “A hash is the workhorse commitment” and “publishing `be8b37e7…1174` commits me to the string ‘CittaDel’ without revealing it.” A bare hash is not hiding for a guessable value. Replace with:

   > A plain hash can supply binding under collision resistance, but it is not a sealed envelope and does not hide a guessable message: anyone can hash candidate strings. Publishing the full digest binds the later preimage claim under A3; it does not by itself conceal `CittaDel`.

4. **Must-fix, lines 63–66.** Quote: “BLAKE3 hash (a faster cousin of SHA-256 with the same properties)” and “the proof shows that the frame behind that hash has certain properties”. Replace with:

   > its BLAKE3 digest, used here under collision-resistance assumption A3, is folded into the session chain; Theorem 1 binds the published outputs of the pinned functions to those frame bytes without placing pixels in the statement.

5. **Must-fix, line 83.** Quote: “with a 17-element group it is trivial”. Replace with: “with this 16-element multiplicative group modulo 17 it is trivial.”

6. **Blocking, lines 94–98.** Quote: “With sixteen possible challenges a cheater passes with probability 1/16 per run; repeat the run twenty times and the probability is below one in 10^24.” The arithmetic is correct, but the protocol claim is false because the group order is composite. Replace the paragraph with:

   > Here take `c` from `{0,1}`. The two displayed honest transcripts accept. A prover prepared for only one challenge succeeds with probability 1/2 per independent run; after 80 runs that guessing probability is `2^-80 ≈ 8.27 × 10^-25`, below `10^-24`. Answering both challenges for the same commitment permits extraction of the witness. Do not claim a 16-way Schnorr error from this composite-order group.

   Explicit counterexample to the current text: choose `t=1`; `(c,s)=(0,0)` and `(8,8)` both accept because `3^8 = 7^8 = 16 mod 17`. Delete “Real systems use a challenge space of about 2^254, so one run suffices.”

7. **Should-fix, lines 100–102.** Quote: “Why does the verifier learn nothing?” Replace with:

   > Why does an honest verifier’s toy transcript reveal no additional information beyond `y`? Here `y^(−c)` means the modular inverse. This is an honest-verifier simulation illustration, not a security result for a tiny group.

8. **Must-fix, lines 104–108.** Quote: “The prover cannot choose t after seeing c, because c depends on t” and “one of the two places the whole construction’s security is spent.” Replace with:

   > Fiat–Shamir derives the challenge by hashing the preceding transcript and mapping the digest into the challenge space. A prover can try different commitments, so circular dependence alone is not a soundness proof. A1 includes the Fiat–Shamir soundness error and Groth16 CRS assumption; Theorem 1 also needs A2 and A3, and later claims need further assumptions.

9. **Must-fix, lines 110–113 and 204–205.** Quotes: “Two of the relation’s legs are proofs of this family” and “the Schnorr idea ... in its pairing-based form”. Neither leg runs Schnorr. Replace both with:

   > ZeeBeam does not run Schnorr in these legs. The beacon leg verifies a BLS pairing equation; the chameleon and trapdoor legs verify Baby Jubjub group equations. The limited analogy is only that they contain public group elements, private scalars and algebraic checks.

   Cite paper §5.2.

10. **Should-fix, lines 121–138.** The node hashes are reproducible only if `||` means raw digest bytes, and “the proof is two hashes long” omits the leaf/index or branch directions. Insert:

   > Here `||` concatenates the raw 32-byte digests, not their hexadecimal text. The authentication path contains two sibling hashes; membership also requires the leaf or its agreed digest and its position.

11. **Must-fix, lines 140–147.** Quote: “the 312 empty slots are filled with domain-separated padding leaves so that an empty slot can never be mistaken for a row. Each leaf hashes...” Replace with:

   > The remaining 312 slots use domain-separated padding leaves; confusing one with a row would require a BLAKE3 collision under A3. Each row leaf hashes the context, row index, `S_t`, raw-frame digest, metadata, `r_t`, `v_t`, `S_{t+1}` and emission digest. The session context includes the row count, depth, identifier, `S_0`, `S_N`, authority-manifest digest and chain-log digest.

   The current list omits context and row index. Cite paper §4.

12. **Should-fix, line 154.** Quote: “Hand-building a circuit ... is how zero-knowledge proofs were made for a decade.” Replace with: “One route is to hand-build a circuit for the relation. ZeeBeam instead uses SP1 6.4.0.” The historical universal is unsupported and false.

13. **Must-fix, lines 161–164.** Quote: “The verification key is a 32-byte hash of the compiled program” and “pinning the exact checks”. Replace with:

   > The public program identifier is SP1’s 32-byte verification-key hash, which is a function of the compiled ELF. Under A2, the verifier must obtain it through a trusted channel and trust its connection to the audited source, pinned toolchain and lockfile.

   Replace “Change one instruction and the key changes” with “A change that alters the ELF normally changes the identifier; §7.3 records one concrete comment-only edit that did.”

14. **Must-fix, lines 171–173.** Quote: “12 min 30 s to 15 min 56 s ... across the 257 fleet rows”. Replace `12 min 30 s` with `12 min 08 s`. Exact fleet values are 727.794 / 755.566 / 955.698 seconds. The document’s own line 400 already has the correct minimum.

15. **Must-fix, lines 180–184, 372, 375 and 404.** Quotes: “built from the `sp1-verifier` 6.4.0 crate alone”, “918-kilobyte binary”, “1.64 s and 2.2 MB”, and “About half a second per proof.” None of those measurements is in the manuscript or bundle. The verifier also directly depends on `sha2`. Replace with:

   > The bundle contains standalone-verifier source. Cryptographic verification uses `sp1-verifier` 6.4.0; the utility also directly uses `sha2` 0.10 to display digests. Archived transcripts record acceptance of a valid proof, rejection after one statement byte is flipped and rejection under a wrong key. No standalone binary-size, elapsed-time or RSS measurement is bundled.

   Replace table line 404 accordingly. The current wording also miscounts the two negative checks by listing “two tamper checks and a wrong key”; the two checks are the flipped byte and wrong key.

16. **Should-fix, lines 174–175.** Quote: “checkable forever by anyone.” Replace with: “independently checkable by a verifier holding the artifact and trusted key hash, subject to the stated assumptions and out-of-band checks.”

17. **Must-fix, lines 188–190.** Quote: “Only a zkVM makes” and “only a zkVM lets”. Replace with:

   > Using a zkVM makes this relation a Rust program and lets an auditor inspect its source; other proof systems could encode and audit the relation differently.

18. **Must-fix, lines 196–199.** The witness list omits `PREFIX.json`, the receipt and headers. Replace it with the paper’s exact list:

   > The private witness comprises the raw frame, headers, chain-log prefix, `PREFIX.json`, receipt, transaction bytes, Merkle branch, block header, incoming viewing key, chameleon opening, trapdoor and session-tree siblings.

19. **Must-fix, line 207.** Quote: “SP1 6.4.0 has no pairing instruction”. Replace with: “SP1 6.4.0 has no pairing syscall, so the patched crate performs the pairing in software over accelerated field arithmetic.”

20. **Must-fix, lines 217–218.** Quote: “The pattern the camera saw at row t”. Replace with: “The nominal emission against which row t is checked”. What the camera physically saw is P1/P2 territory.

21. **Should-fix, line 224.** Quote: “Leg D. The emission (2.14 billion, 51.6%).” Replace with: “Leg D. The emission render (2,141,120,868 instructions, 51.6%); XOF expansion is a separate 4,829,169-instruction region.” Cite paper §6.

22. **Must-fix, line 228.** Quote: “a beacon value nobody knew in advance.” Replace with:

   > the verified predecessor-round value. Its timing interpretation is conditional on A4 and unproved premise A8; the signature is computationally unavailable before release, not nonexistent.

23. **Must-fix, lines 230–233.** Quote: “a typed context and root that both networks are bound to.” Replace with:

   > a typed context and root for the preprocessed camera-pattern pair used by the coupling network. The pose network separately consumes a whole-frame reduction and publishes an uncropped commitment.

24. **Should-fix, lines 243–245.** Quote: “reduces the whole frame to 4 × 256 × 256, then to 4 × 4 blocks”. Replace with: “reduces the whole frame to four 256 × 256 planes, then averages non-overlapping 4 × 4 blocks to obtain four 64 × 64 planes”.

25. **Must-fix, lines 255–256.** Quote: “The verifier’s one out-of-band duty is to look that block up”. Replace with:

   > For this leg, the verifier must confirm the header as canonical Zcash mainnet block 3456294 and confirm the transaction’s inclusion. Separately, the verifier must check both published quicknet rounds against the public schedule.

   VERIFY §4 specifies both classes of check.

26. **Must-fix, lines 262–263.** Quote: “the extracted note commitment is binding, so no second key opens the same action to a different memo.” Replace with:

   > Under A3 and A5, for the pinned transaction and action index, every incoming viewing key that makes the memo leg accept yields the same `binding=` value except with negligible probability. The argument routes through binding of `ExtractP ∘ NoteCommit` and deterministic decryption under a fixed key. The AEAD is expressly not assumed key-committing.

27. **Blocking, lines 265–273.** Quote: “The anchor commits to a `PREFIX.json`”. That is precisely what Proposition 4 refuses to claim. Replace the opening and interpretation with:

   > The memo includes the digest of an 853-byte receipt. The proof shows that the receipt’s `C` currently opens under `Y` to the content root computed from the 603-byte `PREFIX.json`; that file names the BLAKE3 of the 134,188-byte serialized prefix covering rows 0–259. Because the operator knew `td`, `C` binds nothing against the operator and gives no upper bound on when this prefix existed. The sound claims are inclusion of the receipt digest and existence of the demonstrated opening.

   Also define `m`, `Base8`, `Y`, `r` and `td` before presenting the equation.

28. **Should-fix, line 271.** Quote: “The trapdoor flag ... is 1 in every proof”. Replace with: “The trapdoor flag is 1 in every per-row proof in this bundle.” The chain statement has no such field.

29. **Must-fix, lines 276–280.** Quote: “Break any link and the program does not write the statement, and there is no proof.” Replace with:

   > Under A1–A3, an altered in-circuit value must still satisfy every checked equality; otherwise an accepting proof should not exist except with the proof system’s soundness error. Canonical-chain membership is not one of those in-circuit links and remains an external check.

30. **Must-fix, lines 298–303.** Quote: “The 1,101 bytes divide into fields that are fixed ... recomputed in circuit ... and outputs”. That taxonomy omits supplied session/context fields and mislabels both rounds as outputs. Replace with:

   > The statement contains fixed constants; supplied session and context fields consumed by checks; values recomputed or derived by the relation; and learned outputs. The rounds are supplied values whose signatures are verified, while the trapdoor flag is derived from the witness. Section 5.1’s byte table is authoritative, and only fields marked fixed are compared against Appendix A.

31. **Must-fix, lines 309–316.** Quote: “The per-row proof verifies one transition of the chain.” Replace with:

   > The final per-row relation verifies both the predecessor transition into `S_t` and the current row transition into `S_{t+1}`; it does not verify the other session transitions.

   Also expand “one quicknet signature for each of the 66 distinct rounds” to:

   > one signature from the first occurrence of each distinct round; later repeated signature fields are not re-verified, while every row’s value is checked against the cached verified value.

   This exact distinction was the central chain wording correction in round 10.

32. **Blocking, lines 342–349.** The Proposition 2 paragraph omits A8’s fixed-predecessor scope. Replace it with:

   > Under A1–A4 and unproved premise A8, and only when the predecessor record was fixed before round `r_{t−1}`’s scheduled release, the pattern checked against the proved frame is the value A8 says was unavailable before that release. A party choosing the predecessor after seeing the signature lies outside A8. Turning this into a claim about recorded light additionally requires P1 and P2, neither of which the proof establishes. A live optical relay can satisfy both, so relay and scene location remain unauthenticated.

33. **Blocking, lines 365–385.** The verification instructions are not runnable: directory state drifts, file placeholders are unresolved, keys are ellipsized, decoder paths become wrong, and the fleet loop is omitted. Replace the commands with bundle-root-relative commands:

   ```sh
   sha256sum -c SHA256SUMS
   (cd source/rust && sha256sum -c ../../final_relation/SOURCE_TREE_SHA256SUMS)
   (cd source/rust/zeebeam_chain_sp1_candidate && \
     sha256sum -c ../../../final_relation/chain/CHAIN_SOURCE_SHA256SUMS)
   (cd source && sha256sum -c ../final_relation/DEPENDENCY_SHA256SUMS)

   (cd verifier/standalone_verifier && cargo build --release --locked)
   ZB_VERIFY_BIN=verifier/standalone_verifier/target/release/zeebeam-standalone-verifier

   "$ZB_VERIFY_BIN" \
     final_relation/ceremony2/row_096_groth16_proof.bin \
     final_relation/ceremony2/row_096_groth16_public_values.bin \
     0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490

   "$ZB_VERIFY_BIN" \
     final_relation/chain/chain_groth16_proof.bin \
     final_relation/chain/chain_groth16_public_values.bin \
     0x005402a848fdc787e32c0a110ca2662dd61c580481b4e418d5400a52fbd7df50

   for p in all_rows/proofs/row_*_groth16_proof.bin; do
     "$ZB_VERIFY_BIN" "$p" "${p%_proof.bin}_public_values.bin" \
       0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490 |
       grep -c VERIFIED
   done | sort | uniq -c

   python3 verifier/decode_statement.py \
     final_relation/ceremony2/row_096_groth16_public_values.bin
   ```

   Expected fleet result: `257 1`. Say “3,051 checksum entries”, not “3,051 files”: the bundle has 3,052 regular files because the ledger cannot contain its current own digest. The four ledgers have 3,051, 38, 7 and 44 entries, and all pass.

34. **Must-fix, lines 380–382.** Quote: “their scheduled release times, which are the lower bound of Section 9.” Replace with:

   > Under A8, the scheduled release of `r_{t−1}` is the claimed lower bound on pattern computation. It becomes a lower bound on recorded light only under P1 and P2. The row’s own round `r_t` instead concerns the following frame.

35. **Nit, line 412.** Replace “`) — written`” with “`): written`”. This is the only em dash. No literal “impossible” occurs.

## 2. Defects in for_dummies.md

1. **Nit, line 6.** Replace “`zk_all_the_things_paper_draft.md ... zk_all_the_things_worked_examples.md`” with “`paper.md v3.1; worked_examples.md v1.0`”.

2. **Must-fix, line 11 and throughout.** Quote: “What the barge proved about 259 photographs”. P1 is unestablished, so calling the witnesses photographs outruns the result. Replace with:

   > What the relation proved for 259 rows of supplied frame bytes, and what it did not

   Use “row”, “frame” or “raw frame bytes” throughout; reserve “photograph” for explicitly conditional physical readings. Introduce the manuscript terms `leg` and `round` before using the analogies.

3. **Blocking, lines 22–24 and 32–34.** Quotes: “comes out right only if the grid really is solved” and “Witness ... stays hidden.” Replace with:

   > A zero-knowledge proof aims to establish a precisely written statement without disclosing its witness. For this artifact, Theorem 1 says that, under A1–A3 and a trusted pinned key, acceptance implies that some private inputs satisfy the program’s exact checks. The manuscript separately states that the public statement contains no pixels; it gives no separate privacy theorem for this SP1 artifact.

4. **Should-fix, lines 26–30.** Quote: “the first versions in the 1980s” and “winning the lottery twice a day for a year.” The date is not grounded here and the lottery comparison is undefined. Replace with:

   > In an interactive proof, the verifier supplies a random challenge. Independent repetition can reduce the chance of guessing every challenge. A non-interactive proof is checked together with its public statement and pinned key; A1 still retains the Fiat–Shamir soundness error.

5. **Must-fix, lines 38–46.** Quotes: “Change one letter ... the whole fingerprint changes”, “nobody can work backwards”, and “a fingerprint is a sealed envelope.” Replace with:

   > This construction uses 32-byte BLAKE3, BLAKE2b-256 and SHA-256 digests. Under A3, finding two different inputs with the same digest is believed infeasible. A hash is not encryption and does not hide a guessable input, so it is closer to a tamper-evident claim ticket than a sealed envelope. The guest binds a frame digest into the chain and row leaf; the public statement contains no pixels.

6. **Must-fix, lines 52–54.** Quote: “The program is identified by ... its verification key. Pin the key and you have pinned the checks.” Replace with:

   > SP1 supplies a 32-byte verification-key hash derived from the compiled guest ELF. A2 additionally requires the verifier to trust its connection to the audited source, toolchain and lockfile and to obtain it through a trusted channel.

7. **Should-fix, lines 56–59.** Replace the generic comment claim with:

   > In the reported test, replacing a three-line header comment with five lines shifted compiled panic-location records, changed the ELF and key, and reverting the comment restored both.

8. **Must-fix, lines 60–62.** Quote: “Checking the proof takes about half a second on a laptop.” No laptop timing is recorded. Replace with:

   > Across the 257 fleet proofs, median proving time was 755.566 seconds. At $1.99 per instance-hour that is about $0.418 for the median prove step alone, excluding setup and idle time. The bundle records successful standalone laptop verification but no laptop wall-clock time.

9. **Must-fix, lines 66–68.** Quote: “photographed it about two and a half times a second.” Replace with:

   > The operator’s record says the session ran for 301 seconds on 22 August 2026 and produced 712 rows, an average of 2.365 rows per second. The following is the intended protocol and operator-supplied record; the proofs do not authenticate acquisition chronology.

10. **Blocking, lines 70–72.** Quote: “nobody, not even the operators, can know a value before its scheduled time. The rig read the newest one at each row.” Replace with:

   > Quicknet publishes a BLS signature for a numbered round every three seconds, and its random value is the SHA-256 of that signature. A4 assumes that the threshold group releases no signature or share early; the signature is computationally unavailable before release, not nonexistent. The supplied log records a sampled round at each row. The proofs do not establish that it was the newest round.

   The log’s staleness is 1,363–10,799 ms; 669 of 712 rows exceed the three-second period. “Newest” is contradicted by the record.

11. **Must-fix, lines 74–77.** Quote: “Changing one photograph would change every state after it.” Replace with:

   > Under collision resistance, changing the frame bytes without finding a collision changes their digest and requires recomputation of later states. The chain proof establishes consistency of the operator-supplied log, not capture chronology.

12. **Blocking, lines 79–81.** Quote: “a lottery number that did not exist until moments before. Nobody could have painted the scene with that pattern in advance.” Replace with:

   > The checked pattern for a row derives from the preceding row’s recorded beacon value. If the predecessor record was fixed before release, then under A4 and unproved premise A8 the exact computed pattern was unavailable before that preceding round’s scheduled release. The proof does not establish A8, projector display or physical illumination.

13. **Must-fix, lines 83–84.** Quote: “The camera records the scene under that pattern.” Replace with:

   > In the intended protocol, the camera records while that pattern is meant to be displayed. The proof does not establish that the projector displayed it or that the sensor saw it; those are parts of unestablished P1 and P2.

14. **Blocking, lines 86–88.** Quote: “frozen before any of this” and “guesses which of eleven poses the person ... was holding.” The models were trained after the 22 August take. Replace with:

   > Two small neural networks were trained using rows from this same take and frozen before proving. One emits the fixed coupling numerator; its diagnostics do not establish P2. The other emits one of eleven cue-class labels. The manuscript treats both as fixed diagnostic functions, not judges of physical truth.

15. **Must-fix, lines 90–94.** Quote: “Its encrypted memo carries a receipt naming those 260 rows.” Replace with:

   > The operator placed all 712 logged rows in a Merkle tree. A Zcash mainnet transaction was mined minutes after the logged first 260 rows. Its encrypted memo contains the digest of a receipt; the demonstrated receipt and chameleon opening connect that digest to the 260-row prefix.

16. **Must-fix, lines 98–104.** Quote: “For each of the 259 rows that the receipt covers” and “without revealing the photograph”. Replace with:

   > Rows 1–259 within the receipt’s 260-row prefix each have a proof. Under A1–A3 and the trusted pinned key, acceptance establishes Theorem 1’s exact execution relation. The statement contains no pixels; do not make a broader witness-privacy claim without a manuscript theorem.

17. **Blocking, lines 105–107.** Quote: “the Zcash transaction is real, sits in a real block”. Replace with:

   > The guest recomputes the transaction identifier, Merkle branch and supplied header hash and decrypts action 0’s memo. The verifier must separately confirm that the header is canonical Zcash mainnet block 3456294 and that the transaction is in it. The memo contains the receipt digest, not the receipt itself.

18. **Must-fix omission, after line 107.** Insert:

   > The memo argument does not assume that its encryption is key-committing. Acceptance also requires reproduction of the on-chain Orchard note commitment and the transaction’s ephemeral public key. The accepting-key conclusion is conditional on A3 and A5.

19. **Blocking, lines 109–111.** Quote: “A second proof covers the session as a whole”. Replace with:

   > A second proof covers the operator-supplied chain log: all 712 recorded transitions, one signature verification from the first occurrence of each of 66 distinct rounds, every row value checked against the cached verified value, non-decreasing rounds and reconstruction of the same session tree. It does not check later repeated signature fields, re-hash the frames, authenticate acquisition chronology or show that metadata and round schedule were precommitted.

20. **Must-fix, lines 113–116.** Quote: “an independent model before the manuscript settled” and “checked twice”. Replace with:

   > A second model reviewed the manuscript through ten rounds. The manuscript explicitly says this is not independent validation, and the document remains a draft. Each proof was standalone-verified; each proved statement matched its executed statement; and each execution matched its circuit-independent 1,101-byte oracle.

21. **Must-fix, lines 122–126.** Quote: “Someone who fed synthetic bytes ... or pointed the camera at a screen ... would get a valid proof.” Replace with:

   > The relation can accept synthetic bytes if those exact bytes participate in a satisfying chain and prefix; it does not permit later substitution under an existing digest. A live optical relay of a remote or displayed scene lit by the current pattern can satisfy both P1 and P2, so relay and scene origin remain unauthenticated.

22. **Blocking, lines 128–132.** Quote: “the pattern could not have been made before that number’s release.” Replace the paragraph with:

   > Under A4 and unproved premise A8, and only if the predecessor record was fixed before release, Proposition 2 gives a lower bound on availability of the computed pattern. The signature is computationally unavailable before release, not nonexistent. Applying that bound to recorded light additionally requires P1 and P2. It is not a capture timestamp, upper bound or scene-origin claim.

23. **Blocking, lines 134–137.** Quote: “the rows were included in something the operator committed to.” Replace with:

   > After the canonical-chain check, the anchor shows that the receipt digest was included in block 3456294 and that the proof currently demonstrates an opening from the receipt to this prefix. Because the operator held the trapdoor, the commitment binds nothing against the operator and gives no record upper bound to a later verifier. It does not show that this prefix was held when the block was mined.

24. **Must-fix, lines 139–142.** Quote: “human labels on 101 of 116 test frames” and “every pose was held at a different time”. Replace with:

   > The pose network agrees with cue annotations on 101 of 116 evaluation rows. Each class occupies one contiguous time interval, so class and time are perfectly confounded; the temporal-separation test narrowed but did not remove that confound. The coupling network has no untouched test set on this session, and the sealed 288-row session has not been scored. Both models are diagnostics, not a security argument.

25. **Must-fix, lines 144–145.** Quote: “Only 259 of 712 rows are anchored ... The rest can be proved”. Replace with:

   > The receipt prefix contains rows 0–259. The current final relation proves rows 1–259 because row 0 has no predecessor. Rows 260–711 lie outside the August prefix and require a new anchor for full-relation proofs; row 0 remains refused by this relation.

26. **Blocking, lines 149–152.** Quote: “confirm in a second” and “a real blockchain transaction received.” Replace with:

   > Given the trusted key hash, the standalone verifier checks a 356-byte proof against its 1,101-byte statement and establishes Theorem 1’s execution relation. The bundle records successful verification but no laptop elapsed time. Canonical-chain and quicknet-schedule checks remain separate, and the proof does not establish capture authenticity, chronology, model meaning or that the demonstrated prefix existed when the block was mined.

27. **Must-fix, lines 152–154.** Quote: “exactly the questions the next recordings are designed to answer”. Replace with:

   > The manuscript lists separate future work: a trusted sensor path or empirical relay and injection study for P1/P2; shuffled cues across sessions and subjects for pose; one-look scoring of the sealed coupling session; completion of the adaptive attack; and a plain hash commitment for an upper bound. None is yet established.

28. **Must-fix, lines 158–162.** Quote: “The bundle contains everything” and “a verifier that depends on one library”. Replace with:

   > The private, not-yet-published bundle contains the listed proof-verification artifacts and frozen source, but not raw frames; DOI and licence remain pending. The standalone verifier directly depends on pinned `sp1-verifier` 6.4.0 and `sha2` 0.10, with 209 packages in the locked graph. `SHA256SUMS` covers 3,051 listed files, every regular bundle file except itself. Follow `VERIFY.md` v2.5 for row, chain and fleet verification and both out-of-band checks.

29. **Nit, line 168.** Replace the em dash with a colon. No literal “impossible” occurs, but the equivalent absolutes in defects 5, 10, 12 and 22 are equally unacceptable.

## 3. Toy recomputation

All strings were UTF-8/ASCII without newline. Internal Merkle nodes hash concatenated raw 32-byte digests.

| Quantity | Document | Recomputed value | Result |
|---|---|---|---|
| SHA-256(`CittaDel`) | `be8b37e7…1174` | `be8b37e7031838cd072205ce6ebba28b6275e13dc5054e9610be2fbab49f1174` | Match; all 64 positions differ from the second digest |
| SHA-256(`cittaDel`) | `a3758122…edcd` | `a3758122e0a4f76121d3a31d7bac99a633ffae69966e39114a5955302ecbedcd` | Match |
| BLAKE3(`hello`) | `ea8f163d…200f` | `ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f` | Match |
| Powers `3^1` through `3^16 mod 17` | `3, 9, 10, 13, 5, 15, 11, 16, 14, 8, 7, 4, 12, 2, 6, 1` | Same | Match |
| Group cardinality | “17-element group” | 16 nonzero residues | **Fail** |
| `x=11`, `y` | `3^11 mod 17 = 7` | 7 | Match |
| `k=5`, `t` | `3^5 mod 17 = 5` | 5 | Match |
| `c=1` transcript | `s=0`; both sides 1 | `s=0`; left 1, right `5×7 mod 17 = 1` | Match |
| `c=0` transcript | `s=5`; both sides 5 | `s=5`; both sides 5 | Match |
| Claimed 16-way soundness | `1/16` | At least `1/8`; `t=1` answers challenges 0 and 8 without the full discrete log | **Fail** |
| Conditional arithmetic | `(1/16)^20 < 10^-24` | `2^-80 = 1/1,208,925,819,614,629,174,706,176 = 8.271806125530277×10^-25` | Arithmetic matches, protocol premise does not |
| `leaf0` | `1cfebe87c97b79c0…` | `1cfebe87c97b79c0a5f3127b9c675beea39145ac2b0ab95f70bd8cfb9ac8188b` | Match |
| `leaf1` | `449a41e3abc65628…` | `449a41e3abc65628bad6a9d7648fb06fe41850b80ab9b2ce936178cc62b997b2` | Match |
| `leaf2` | `8ae21941fb7905e1…` | `8ae21941fb7905e10882f33ef06f0845719bbdcdfb120b519c6f73b338c5950a` | Match |
| `leaf3` | `892f356afec72b4d…` | `892f356afec72b4daa27403fe2160d9d4f57b758dd45bfbd5b897d088ead8061` | Match |
| `node01` | `ae8c4625d4b859ec…` | `ae8c4625d4b859ecba5fc2ad22d7563e1b0ae82ac18c2cca896de0679a042b03` | Match |
| `node23` | `334419e44623e804…` | `334419e44623e804c6441bc2dbfec95e83e784aa046619f74268dc66286937bf` | Match |
| Merkle root | `627972237f0bc016…` | `627972237f0bc016072f7464e2e1ce66bf1398e5da649f204c62036896991852` | Match |

All unlisted real-system numbers checked clean, including 712 rows, 301 seconds, 66 rounds, frame and pattern dimensions, 1,101-byte statements, 356-byte raw proofs, 259 proved rows, instruction counts, both vkeys, ELF hashes and sizes, chain-proof figures, txid prefix and block height. The ten-round count is correct from the v3.1 frontmatter, §7.4, log and `FINDINGS_round10.md`; paper line 14’s “ninth” is stale manuscript text.

All 3,051 `SHA256SUMS` entries pass.

## 4. Contradictions between the companions

1. Worked says canonical block membership requires an external verifier lookup; dummies says the proof establishes that the transaction “is real” and “sits in a real block”.

2. Worked says the memo yields a receipt digest; dummies says the memo carries the receipt.

3. Worked correctly says the trapdoor makes the anchor a receipt with no record upper bound; dummies initially says the rows were included in something the operator committed to.

4. Worked identifies rows 0–259 as the prefix, row 0 as refused and rows 260–711 as awaiting another anchor; dummies says “only 259” are anchored and that all “the rest” become provable.

5. Worked labels Proposition 2 conditional on A8 and P1/P2; dummies’ main narrative says the value “did not exist” and the pattern “could not” have been made earlier.

6. Worked treats pose as a fixed diagnostic output confounded with time; dummies calls the network a judge that guesses the physical pose and falsely says it was frozen before capture.

7. Worked discloses that the chain proof receives an operator-supplied log and proves no authentic chronology; dummies describes it as covering “the session as a whole” without that qualification.

8. Worked reports 1.64 seconds for a complete verifier invocation; dummies says half a second. Neither figure is preserved in the ground truth.

## 5. Verdicts

`worked_examples.md`: **publishable-alongside-the-manuscript after the listed fixes**.

`for_dummies.md`: **publishable-alongside-the-manuscript after the listed fixes**.

— BOSUN ⚓
