Hoy.

The cryptographic execution results and reported proof fleet survive this pass. The staged release does not. I found three blocking defects: Proposition 4 exceeds A6, the advertised source snapshot is not rebuild-complete, and the bundled realness note is the wrong version.

No file was modified.

## 1. Substance changes since round 10

Against the post-round-10 corrected manuscript, v3.1, there is no technical or propositional change in v3.4.

The v3.1 to v3.4 diff is confined to the authorized naming and status surface: frontmatter, title, status note, Section 1 Names paragraph, Table 1 caption, logs, and companion titles/intros/`companion_to` fields.

A literal comparison with the round-10 input, v3.0, also shows the corrections demanded by round 10:

- the 66-signature claim was narrowed to distinct-round/per-row values;
- medians and boundary counts were corrected;
- the operator caveat was strengthened;
- source, transcript and ELF paths were corrected;
- the conclusion was expanded from the development rows to the 259-row fleet and chain proof;
- appendix pins and paths were updated.

Those are expected round-10 fixes, not later drift.

One authorized naming change does introduce new ambiguity: the Names paragraph uses “ZeeBeam” first for the system and then for the recording. That is defect 14 below.

## 2. Defects

### 1. Blocking: Proposition 4 does not follow from A6

Section 9.4, [paper](../paper/zeebeam.md?plain=1#L699)

Quoted A6:

> “The verifier’s canonical view includes block 3456294.”

The guest accepts a supplied header and branch and publishes their header hash. Nothing requires that hash to equal canonical block 3456294. Proposition 4 therefore cannot infer canonical inclusion. It also silently uses A5 and says receipt bytes are in the block, when only the ciphertext is there.

Replace A6 with:

> **A6 (Zcash).** The verifier independently checks that the statement’s published header hash equals the hash of canonical Zcash-mainnet block 3456294 and accepts only after that block is beyond the verifier’s chosen reorganisation depth.

Replace Proposition 4’s opening with:

> **Proposition 4.** Under A1–A3, A5 and A6, an accepting proof implies that canonical block 3456294 contains the proved transaction action; its fixed ciphertext has the unique accepting memo binding of Proposition 3, equal to SHA-256 of the proved receipt; and \(C=m\cdot Base8+r\cdot Y\), where \(r\) is the proved opening and \(m\) is the Section 5.2 hash-to-scalar of the canonical `{contentRoot, network, profile, recordScope}` tuple, with `contentRoot` computed from the proved `PREFIX.json`.

Replace:

> “The sound statements are that the receipt bytes are included in block 3456294…”

with:

> The sound statements are that canonical block 3456294 contains the proved transaction ciphertext; under A5, its unique accepting memo binding equals SHA-256 of the proved receipt; and the demonstrated chameleon opening exists.

### 2. Blocking: the claimed complete source snapshot is incomplete

Section 12, Appendix E, `source/README.md`, `VERIFY.md`, `ENVIRONMENT.md`, and the worked companion.

`source/rust/zeebeam_sp1_candidate/join/Cargo.toml` refers to five absent path crates:

- `source/rust/preprocess_v1_candidate`
- `source/deps/BOSUN/scratch/zeebeam_lambda_launch_prep_20260823/zeebeam-trained-r32-ptq-v2-sp1-v1/model`
- the corresponding `relation`
- `source/deps/BOSUN/scratch/uncr64/uncr64_pose_sp1_20260831/model`
- the corresponding `relation`

Both published build drivers consequently fail before compiling. Digest integrity of the supplied subset is not rebuild completeness.

The preferred fix is to add the exact proved source bytes and regenerate every affected ledger. If they are deliberately withheld, replace every complete-source/rebuild claim with:

> The bundled source tree is not self-contained for a rebuild: `join/Cargo.toml` points to five local path crates not included here. The digest lists authenticate only the supplied subset.

### 3. Blocking: the realness provenance file is the wrong version

Section 8.2 and [ml/realness](../bundle/proofs_20260902/ml/realness/README.md?plain=1#L1)

The manuscript cites “realness results v2.0” and the ML README claims that version. The bundled note is v1.0 and contains the obsolete “three of four adversary classes” conclusion.

Replace it with the exact v2.0 note currently outside the bundle at:

`<project-records>/zeebeam_realness_results_20260901.md`

Then regenerate the bundle and release hashes. Do not relabel the v1.0 bytes as v2.0.

### 4. Must-fix: the physical corollary falsely says replay is excluded

Section 10

Quoted:

> “Under P2 and premise A8 it excludes replay.”

Neither the proof nor P2 supplies an upper time bound. A frame acquired after beacon release may be replayed indefinitely.

Replace with:

> Under A8, P1 and a thresholded P2, it rules out only a frame acquired before \(r_{t-1}\)’s scheduled release. It does not exclude replay of a frame acquired after that release, because it proves no upper bound; without P1 it also does not exclude byte substitution.

### 5. Must-fix: the chain proof does not establish when the predecessor was fixed

Section 9.2

Quoted:

> “The chain proof would close this case…”

The operator supplies the authenticated log prefix. The proof verifies transitions, not fixation time.

Replace with:

> A party who chooses \(\rho_{t-1}\) after seeing \(\sigma_{t-1}\) is outside A8. The session-chain proof of Section 9.5 verifies earlier transitions but does not prove when \(\rho_{t-1}\) was fixed, so it does not close that case.

### 6. Must-fix: P2 has no defined threshold

Section 9.2

Quoted:

> “a high coupling numerator”

“High” is not a premise. It supplies no acceptance region.

Replace the physical corollary’s opening with:

> **Physical corollary (conditional).** Fix a stated threshold \(\tau\) before applying the corollary. If (P1) `raw_t` is genuine sensor output rather than injected or synthesised bytes, and (P2) the published numerator \(q_t\) satisfies \(q_t\ge\tau\) and, for the relevant acquisition distribution, that event implies that the illumination recorded in `raw_t` was \(E_t\) rather than another pattern or none, then the recorded light was emitted after \(r_{t-1}\)’s scheduled release.

### 7. Must-fix: “recorded under” repeatedly converts byte pairing into physical fact

Abstract, Sections 1, 3.4 and 8.3

Replace:

> “the pattern row was recorded under”

with:

> the pattern supplied to the coupling computation

Replace:

> “The pattern a proved frame was recorded under”

with:

> The pattern against which the proved frame is checked

Replace:

> “the chain state those frames were recorded under”

with:

> the pattern paired with each frame was derived from a chain state seeded by beacon rounds verified in circuit

Replace Section 3.4’s physical summary with:

> Any physical reading additionally requires A1–A4, the fact-specific early-pattern premise A8, P1 genuine sensor acquisition and P2’s thresholded illumination inference; none is established by the proof.

Section 1’s summary must likewise name A8 explicitly.

### 8. Must-fix: A1 and A2 are written for one ELF/key but used for two

Section 9.1

Replace the openings with:

> **A1.** SP1/Groth16 is knowledge-sound for each pinned ELF invoked below…

> **A2.** Each pinned verification key is that of its corresponding audited sources…

### 9. Must-fix: the witness tuple is incomplete

Section 9.1 and worked companion

The tuple omits the separately read drand signature and does not match the thirteen guest inputs.

Replace with:

> \(w=\)(row header, membership witness, raw frame, \(\sigma_t\), anchor transaction, anchor Merkle branch, anchor block header, incoming viewing key, chain-log prefix, `PREFIX.json`, receipt, opening, trapdoor).

Qualify Theorem 1 with:

> except with the zkVM soundness error and hash-collision advantage

Replace:

> “A3 rules out…”

with:

> A3 makes alternate digest openings infeasible for a polynomial-time prover.

### 10. Must-fix: “all 712 rows through the relation” is false

Sections 1 and 7.6

Only rows 1–259 were executed under the final ELF. All 712 received circuit-independent oracle records.

Replace the Section 1 heading with:

> **Every row through the oracles; every anchored row through the relation.**

Replace the Section 7.6 opening with:

> We ran circuit-independent oracles for all 712 rows and the final ELF for the 259 rows inside the published prefix.

### 11. Must-fix: the standalone verifier does not compare every fixed byte

Sections 1, 5.1, 12, 13 and worked companion

Quoted:

> “The verifier compares every fixed byte against Appendix A.”

The standalone invocation checks Groth16 acceptance and statement bytes. It does not independently decode and compare every pin.

Replace with:

> Fields marked “fixed” are enforced by the guest authenticated by the pinned key; `PINS_final.json` records their expected bytes for an independent decoded-statement comparison.

In Section 13 use:

> whose fixed bytes are enforced by the pinned guest and recorded in `PINS_final.json`

The “three-input verifier” description must say that this is the standalone Groth16 check. Drand and canonical-chain conclusions require the Appendix C out-of-band checks.

### 12. Must-fix: Section 9.7’s disclosure list is materially incomplete

Replace its opening with:

> The statement contains no pixels. Apart from the fixed headers, denominators, flags and model/provenance/beacon constants listed in Section 5.1, its instance-dependent fields reveal the row index, row count, tree depth, session identifier, \(S_0\), \(S_N\), authority and chain-log digests, context and session root, both beacon rounds, coupling numerator, pose verdict, head-saturation count, eleven logit sums, typed context and root, uncropped commitment, anchor txid and block hash, `contentRoot`, receipt digest, \(C\), and the trapdoor flag.

### 13. Must-fix: model replacement is not binding-preserving

Section 8.3

Quoted:

> “retrain or replace the models without touching the binding argument”

Changing a model changes the program, ELF, key and proofs.

Replace the paragraph with:

> A reader may disregard the models’ semantic labels while retaining Theorem 1’s execution claim; replacing either model requires a new ELF, verification key and proofs. The physical inference of Section 9.2 depends on their empirical validity; Theorem 1’s byte-level execution claim does not.

### 14. Should-fix: “ZeeBeam” changes referent inside the Names paragraph

Section 1

Replace the paragraph with:

> ZeeBeam names the capture-and-proof system. A “ZeeBeam recording” is a recording instance with rows proved under that system; the one reported here comprises rows 1–259 of a 712-row session. Dark Lantern names only the wider research programme. TB-v0.9, `ZB…` magic strings and legacy `zkbeam_*` paths are frozen historical identifiers.

Also replace:

> “zkBeam design”

with:

> ZeeBeam design

Apply the same system/recording distinction to both companion introductions and the release README/CITATION metadata.

### 15. Must-fix: the chameleon trapdoor contradicts “new anchor required”

Section 11 and both companions

Quoted:

> “proving rows 260–711 under the full relation requires a new anchor transaction”

The disclosed trapdoor permits an alternate prefix/opening under the same commitment.

Replace with:

> Rows 260 to 711 lie outside the published 260-row prefix. Covering them without exercising the disclosed chameleon-equivocation capability requires a new anchor transaction.

### 16. Should-fix: “honest anchor” and “capture-time anchor” are misleading

Sections 1, 4 and 9.4

Replace:

> “An honest anchor”

with:

> An inclusion receipt with disclosed equivocation

Replace:

> “Capture-time anchors”

with:

> Anchor transactions

Replace the chronology claim with:

> According to the project records, the transaction was mined minutes after the 260 rows named by the supplied `PREFIX.json`; Proposition 4 does not turn that chronology into a bound.

Replace “capture-time one” with “supplied opening.”

### 17. Must-fix: the referee-round count is stale

Sections 1, 7.4, Acknowledgements, status notes and companion logs

The manuscript says both eight and ten rounds. This report is the eleventh.

Replace the audit-history sentence with:

> Sol has refereed this manuscript eleven times: nine rounds before v3.0, a tenth verification pass, and this eleventh ultra-effort publication-byte audit. The eleven reports are included in the public referee trail.

Update the status and acknowledgements consistently and add this report to the trail.

### 18. Must-fix: the fleet idle-time arithmetic is wrong

Section 7.1

Quoted:

> “roughly six instance-hours…”

The stated intervals give \(53\times8+48\times8=808\) minutes, or 13.47 instance-hours.

Replace with:

> roughly 13.5 instance-hours were the pre-dispatch and incident-to-restart idle intervals.

### 19. Must-fix: Section 12 overstates the bundle contents

Replace:

> “oracle values, executed statements and witnesses of every row”

with:

> the oracle values and per-row membership records of all 712 rows, and the executed statements of rows 1 through 259

Replace:

> “Python re-runs both frozen networks”

with:

> the recorded outputs and host-specific scripts used for the Python re-runs of both frozen networks; the raw frames, model manifests and `zeebeam_science` dependency needed to rerun them are not in this bundle

Replace:

> “PINS.json (every constant in Appendix A)”

with:

> `PINS.json` (ceremony-1 pins)

Replace “results.json of two ladder runs” with the actual names:

> `ml/coupling_ladder_results.json` and `ml/pose_ladder_results.json`

### 20. Must-fix: VERIFY metadata, proof count and dependency claims are stale

Appendix C and [VERIFY.md](../bundle/proofs_20260902/VERIFY.md?plain=1#L1)

Appendix C says VERIFY v2.4; the bundled file is v2.6.

VERIFY says “Four Groth16 proofs” and “Nothing here is published.” The bundle has 262 proofs and sits in the staged public tree.

Replace the opening with:

> This public release bundle contains 262 Groth16 proofs produced on 2–3 September 2026: two ceremony-1 row proofs, 259 final-relation row proofs, and one whole-session chain proof. A third party can verify them using the steps below.

Replace:

> “one crate, `sp1-verifier` 6.4.0, and nothing else”

with:

> The standalone verifier has two direct dependencies, `sp1-verifier` 6.4.0 and `sha2` 0.10, resolved to 209 locked packages.

### 21. Must-fix: publication-status text is false in the public tree

Authorship note, Section 12, Appendix F, VERIFY, both companions and release notes

Replace:

> “authorship position to be settled before submission”

with:

> Drafted with BOSUN, the project’s automated research assistant; BOSUN’s role is stated in Acknowledgements.

Replace:

> “The private bundle contains”

with:

> The public release bundle contains

Replace:

> “licence and DOI to be settled”

with:

> The release includes `LICENSE`; no DOI is assigned in this staged tree.

Delete or resolve the literal bibliography prompts:

> “(cite revision/version/commit)”  
> “(verify revision)”  
> “[confirmed …]”

They are editorial notes, not citations.

### 22. Should-fix: Appendix F points readers to private scratch paths

Replace the referee-trail paths with:

> `public/referee_trail/FINDINGS_round1.md` through `public/referee_trail/FINDINGS_round11.md`

Any research document still outside `public/` must either be added or explicitly labelled “not bundled.”

The authority-manifest preimage is absent. Replace its entry with:

> authority-manifest digest … (manifest preimage not bundled)

### 23. Should-fix: the chain reproducibility metadata contains a stale path

`final_relation/chain/REPRODUCIBLE_CHAIN.json`

Replace:

> `source/chain/script/Cargo.lock`

with:

> `source/rust/zeebeam_chain_sp1_candidate/script/Cargo.lock`

### 24. Must-fix: Section 8.1 understates what was executed and misdescribes the selection fixture

Replace:

> “The 72 selection rows are the only rows with reported numerators.”

with:

> The 72 selection rows are the only rows with both matched and crossed ladder numerators in the 144-value fixture.

Replace the guest-evidence sentence with:

> Among the 72 selection rows, the 24 anchored rows 96–119 were executed and proved by the final guest, and every guest numerator equals the fixture’s matched numerator; separate circuit-independent oracle records give coupling numerators for all 712 rows.

### 25. Must-fix: pose “correctness” uses non-blind cue-aware annotations

Section 8.2 and the dummies companion

Replace the split/result description with:

> Within-class interleaved: 461 training and 116 evaluation rows over 577 hold-confirmed, cue-aware block annotations and eleven classes (`ml_splits.json`); the annotator saw each cue, so these are not independent blind pose labels. On that split the integer model agrees with the float model on 114 of 116 evaluation rows and with the cue-aware annotation on 101 (float: 102).

Replace each affected “correct” or “fraction correct” with “agreement with the cue-aware annotation.”

### 26. Should-fix: recorded ML confounds are missing

Section 8.2

Insert:

> Crop and wide use the published block-mean reducer. Whole-frame, performer-window and background-only use a separate anisotropic reducer that warps the rectangular frame to square and gives unequal cells equal weight, so cross-family differences conflate field of view with resampling. The masked performer-window and background-only inputs are out-of-distribution diagnostics. Adjacent rows are temporally dependent, and the initialisation spreads measure optimiser variance, not uncertainty across takes.

Clarify the temporal headings as:

> Early: 287 train, 290 evaluation  
> Late: 290 train, 287 evaluation

Replace:

> “direction asymmetry is real”

with:

> The observed direction asymmetry on this take is descriptive rather than controlled; the directions use different evaluation rows, class composition and half-run difficulty.

### 27. Should-fix: Section 8 overstates provenance for ancillary diagnostics

Replace:

> “Five 3×3 strided convolutions”

with:

> Five 3×3 convolutions with strides 1, 2, 2, 2 and 2

Replace:

> “The proof certifies nothing about either function.”

with:

> The proof certifies the identity and exact evaluation of each pinned function, but nothing about its accuracy or physical meaning.

Replace the 112-second predeclaration claim with:

> The external project audit records that the pre-analysis note was timestamped about 112 s before the result; neither the note nor its timestamp evidence is included in this bundle, and this was not an immutable preregistration.

Replace “were also run” for absent replay/L2 outputs with:

> The project note records that … were also run.

### 28. Must-fix: both companions falsely treat a large hash preimage as hidden

Worked companion and dummies companion

Replace the worked passage with:

> A plain hash supplies computational binding under A3 but no hiding guarantee. Anyone can hash candidate inputs, and byte length alone does not establish that a frame is unguessable. The manuscript’s claim is narrower: the public statement contains no pixels, while the digest binds the relation to particular bytes.

Replace the dummies passage with:

> Byte length alone does not make a frame unguessable, and a digest supplies no hiding guarantee; here it stands in for the frame only as a binding identifier under A3.

### 29. Must-fix: the worked companion inherits the manuscript’s anchor errors

Replace its anchor explanation so it says:

> The transaction contains a ciphertext whose decrypted memo is compared with SHA-256 of the proved receipt. The receipt itself contains the canonical record-scope tuple used, with `contentRoot` and the fixed constants, to derive \(m\). The proved opening demonstrates \(C=m\cdot Base8+r\cdot Y\). Canonical block membership remains an external A6 check.

Also apply defects 4–13, 15–16 and 19–20 to the corresponding worked explanations.

Replace:

> “Inside the proof, the verifier learns only…”

with:

> Under the zkVM’s zero-knowledge assumption, the witness is not disclosed by the proof. The manuscript establishes only the narrower direct fact that no pixel bytes occur in the public statement.

Replace:

> “verification costs nothing worth measuring”

with:

> the recorded standalone invocation took 1.64 s.

### 30. Must-fix: the dummies companion overstates physical origin and relay resistance

Replace its early-pattern passage with:

> The pattern is unpredictable before the scheduled release of the fixed predecessor beacon round only under A8. The proof checks the supplied round and signature; it does not establish when the predecessor state was fixed or when acquisition occurred.

Replace:

> “P1 and P2 rule out replay or relay”

with:

> Synthetic injection violates P1. A live optical relay of a remote or displayed scene lit by the current pattern can satisfy both P1 and P2, so even those premises do not authenticate origin.

Replace its verification summary with:

> The recorded development-machine invocation verified one proof and both negative controls in 1.64 s. Subject to A1–A3 and the trusted key, acceptance binds outputs to frame bytes and beacon-seeded log; the supplied transaction carries only the receipt digest, and canonical block membership remains an external check.

Replace its release-status paragraph with:

> This public release includes a repository-level `LICENSE`; no persistent identifier is recorded. The proof and statement verification artifacts are present, but the source snapshot is not dependency-closed unless the five missing path crates are added.

### 31. Should-fix: companion metadata points to private historical paths

Replace worked `companion_to` with:

> `companion_to: ZeeBeam manuscript v3.4 (public/paper/zeebeam.md)`

Replace dummies `companion_to` with:

> `companion_to: ZeeBeam manuscript v3.4 (public/paper/zeebeam.md); ZeeBeam, by Example v1.3 (public/companions/zeebeam_worked_examples.md)`

### 32. Should-fix: stale hostile-reader phrases remain

Replace “real rows” in the Abstract and Conclusion with:

> concrete session rows

Replace:

> “A recording no longer carries evidence…”

with:

> Pixels alone no longer provide reliable evidence of physical capture.

Replace “seven-component relation” with:

> multi-leg relation

Replace the row-95 boundary sentence with:

> That boundary fixture is an execution record, separate from the row-95 fleet proof reported in Section 7.6.

### 33. Nit: two proof descriptions need exact qualification

In Proposition 3 add:

> `PreparedIncomingViewingKey::new` uses only the scalar half of the 64-byte incoming viewing key; encodings that differ only in `dk` induce the same decryption, so it remains only to consider distinct scalar components.

In Section 9.5 replace:

> “verifies transition \(t-1\to t\)”

with:

> verifies transitions \(t-1\to t\) and \(t\to t+1\)

Replace “anchor of Section 9.3” with “anchor of Section 9.4.”

### 34. Must-fix if the whole public envelope is released

The root release tree has 3,082 files but its SHA ledger has 3,080 entries, omitting itself and the bundle’s own `SHA256SUMS`. Release notes falsely say it omits only itself.

Replace with:

> covers every file except itself and the bundle’s own `SHA256SUMS` (3,080 entries)

The root README and citation metadata also repeat the name ambiguity, “recorded pattern,” physical-lighting, complete-source, and receipt-in-block errors above.

## 3. Recomputation table

| Quantity | Recomputed result | Assessment |
|---|---:|---|
| Bundle regular files | 3,053 | Matches inventory |
| Bundle SHA entries | 3,052 | All listed hashes pass; ledger omits itself |
| Source/dependency/chain ledger entries | 38 / 44 / 7 | All listed hashes pass |
| Groth16 proof, statement, framed-proof counts | 262 / 262 / 262 | VERIFY’s “four” is false |
| Raw proof files | 356 | Matches recorded proof payload size |
| Statement sizes | 1,085; 1,101; chain 279 bytes | Matches |
| Ceremony-1 row 72 | 4,148,971,330 instructions; 269,904 syscalls; 58.564 s | Matches |
| Ceremony-1 row 96 | 4,149,712,293 instructions; 269,904 syscalls; 61.501 s | Matches |
| Row 72 setup/prove/verify | 21,356 / 850,972 / 366 ms | Matches |
| Row 96 setup/prove/verify | 21,401 / 769,575 / 505 ms | Matches |
| Ceremony-2 row 96 | 21,170 / 857,424 / 365 ms | Matches |
| Ceremony-2 row 72 | 21,957 / 749,813 / 363 ms | Matches |
| Ceremony-2 cost | $1.227 | Matches |
| Fleet setup/prove/verify medians | 727,794 / 755,566 / 955,698 ms | Matches |
| Fleet per-proof setup/verify median | 22,710 / 365 ms | Print as 0.365 s consistently |
| Fleet total proving time | 196,635,888 ms = 54.6211 h | Matches |
| Fleet RSS range | 113,520–341,520 KiB = 110.859–333.516 MiB | Matches |
| Fleet framed-size distribution | 2,791×1; 2,792×1; 2,793×16; 2,794×168; 2,795×71 | Matches |
| Fleet cloud cost | \(4073/60\times1.99=\$135.09\) | Matches |
| Recorded idle time | 13.47 instance-hours | Paper’s roughly six fails |
| Executed/oracle/membership rows | 259 / 712 / 712 | Paper conflates these |
| Final 1,101-byte statements | 259 | All decoded values match records |
| Instruction range | 4,146,753,922–4,154,781,545 | Matches |
| Median instructions | 4,150,763,529 | Matches |
| OLS instruction slope | 31,134 per row | Matches |
| All-row coupling | min −1,924,590; median 49,650,008.5; max 93,030,486 | Matches |
| Selection/other coupling medians | 49,627,252 / 49,650,008.5 | Matches |
| Distinct beacon rounds | 66, range 31,521,605–31,521,705 | Matches |
| Round boundaries | 65; every boundary changes by 25 | Matches |
| Row-95 fixture | 1,101 bytes; 4,149,667,741 instructions; rounds 31,521,619/620 | Matches |
| Session root | `38a484b8…6572`, depth 10 | Matches |
| Chain log | 367,429 bytes; BLAKE3 `754e…` | Matches |
| Prefix | 134,188 bytes; 260 rows; BLAKE3 `182a…` | Matches |
| `PREFIX.json` | 603 bytes; SHA-256 `d67…` | Matches |
| Receipt | 853 bytes; SHA-256 `fae…` | Matches |
| Final ELF | 1,171,416 bytes; SHA-256 `8bc…` | Matches |
| Ceremony-1 ELF | 1,160,904 bytes; SHA-256 `5b3…` | Matches |
| Chain ELF | 460,416 bytes; SHA-256 `4934…` | Matches |
| Model blobs | 3,290,552 bytes / `0188…`; 13,312 bytes / `c95…` | Matches |
| Standalone verification | 1.64 s; about 2.3 MiB RSS | Matches |
| Verifier binary/lock graph | 917,512 bytes; 209 locked packages | Matches |
| Coupling fixture | 72 matched + 72 crossed; parity 144/144 | Matches |
| Coupling AUROC | 5,175/5,184 = 0.9982638889 | Matches |
| Coupling inversions/ties | 9 / 0 | Matches |
| Selection overlap with anchored rows | rows 96–119, 24/24 matched | Matches |
| Realness seeds | 20260823, 20260901, 20260902 | Matches outputs |
| Realness pairs/AUROC | 36/36; AUROC 1.0; five clusters | Matches outputs, but provenance note wrong |
| Annotation counts | 577 hold; 98 settling; 37 no-cue = 712 | Matches |
| Pose split | 461 train + 116 evaluation = 577 | Matches |
| Integer/float annotation agreement | 101/116 and 102/116 | Matches |
| Integer/float mutual agreement | 114/116 | Matches |
| Robustness medians | crop 113; wide 105; full 102; performer 109; background 60 | Matches |
| Background seed range | 17–73 | Matches |
| Interleaved seed gap | min 1; median 1; max 4; gap two for one case | Matches |
| Temporal early | 287 train / 290 evaluation | Matches |
| Temporal late | 290 train / 287 evaluation | Matches |
| Temporal seed-gap median | 14 | Matches |
| Direction differences | 10.82 / 8.23 / 19.79 / 13.98 points | Matches |
| Chance pose accuracy | 1/11 | Matches |
| Worked toy SHA/BLAKE3/Merkle/Schnorr/\(2^{-80}\) calculations | All recompute exactly | Pass |
| Raw frames, anchor transaction/header, authority-manifest preimage | Not bundled | Cannot independently remeasure |
| Complete source rebuild | Fails before compilation | Five path crates absent |

## 4. Verdicts

Manuscript v3.4: **not publishable as staged; publishable after all blocking and must-fix items are resolved.**

“ZeeBeam, by Example” v1.3: **after the listed fixes.**

“ZeeBeam, for Everyone Else” v1.3: **after the listed fixes.**

— BOSUN ⚓
