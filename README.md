# ZeeBeam: The Zero-Knowledge Beam

ZeeBeam, the Zero-knowledge Evidence Emitter Beam, is the projector-camera capture-and-proof system: the protocol, the rig and the proving pipeline. A ZeeBeam
recording is a session with rows proved under that system. The projected pattern is a deterministic function of a hash chain that folds in the drand quicknet beacon
and the camera's previous frame. For each of the 259 anchored rows released here, the recorded computation
is proved in zero knowledge under one verification key. This repository holds one ZeeBeam recording: 259 rows
of a 712-row session recorded on 22 August 2026, each with a 356-byte Groth16 proof and a 1,101-byte public
statement that contains no pixels, plus a second proof over the whole 712-row chain log. The work belongs to
Dark Lantern, the wider privacy and zero-knowledge research programme.

## ZK all the things

One zkVM program (SP1) enforces every leg below in circuit, for one row of the session. The verifier
supplies nothing but the proof, the statement and a 32-byte key hash.

| leg | what the proof checks |
|-----|------------------------|
| beacon, row t | the quicknet BLS signature for round r_t verifies; its SHA-256 is the value the chain consumes; r_t is published |
| beacon, row t−1 | the preceding row's signature verifies likewise, and the complete preceding-row record advances to S_t, from which the nominal emission checked against the proved frame is derived; r_{t−1} is published |
| chain advance | BLAKE3 over the 24,472,000-byte raw frame, and the protocol's advance from S_t to S_{t+1} |
| emission | the nominal emission pattern re-derived from S_t (seed, XOF, render) and hashed |
| preprocess and typed root | fixed camera and emission windows reduced to the coupling-network input; a whole-plane camera reduction feeds the pose network; BLAKE3 tile tree and typed root recomputed |
| coupling | a frozen integer discriminator scores the frame against its pattern; the embedded blob's digest equals the compiled constant |
| pose | a frozen eleven-class integer classifier reads the frame; commitment, all eleven logit sums, verdict and saturation count are published |
| membership | the row's leaf sits in the committed 712-row session tree |
| Zcash inclusion | a supplied v6 transaction is parsed from raw bytes, its ZIP-244 txid recomputed, its supplied Merkle branch checked to the supplied header, and the header hashed; canonical-mainnet status is checked out of band |
| memo | the Orchard action's encrypted memo is decrypted in circuit under the holder's viewing key, with the note commitment recomputed |
| prefix join | BLAKE3 of the 260-row chain-log prefix equals the digest in `PREFIX.json`; `contentRoot` is SHA-256 of the domain, the encoded length and the canonical `PREFIX.json`; the proved row's values sit inside the prefix |
| receipt and chameleon | SHA-256 of the receipt equals the decrypted memo binding; the validated points Y and C satisfy C = m·Base8 + r·Y with the proved opening |
| trapdoor | knowledge of the trapdoor td with Y = td·Base8, flagged in the statement |

The manuscript's Section 5.2 gives each leg exactly; Section 9 gives the security statements and their
assumptions.

## What is not proved

The manuscript is explicit, and so is this file. The proofs establish computation over the operator's
recorded bytes. P1, genuine sensor output, and P2, the inference from a coupling score to the recorded
illumination, are unproved. Even with both, a live optical relay is not excluded, and neither scene
identity nor location is authenticated. The beacon gives a lower bound on when the pattern could have been
computed, under stated assumptions and never a timestamp; a frame recorded after that release could be
replayed later. The Zcash anchor is a receipt with a demonstrated opening; the operator held a trapdoor, so
it does not bind a unique record. The two networks are diagnostics trained on rows of this same recording,
not evidence of physical realness. The chain proof establishes internal consistency of an operator-supplied
log; it does not rehash the frame files or authenticate chronology or precommitment. Rows 260 to 711 have no
proof under the full per-row relation, although their log transitions are covered by the session-chain
proof. Sections 9.4 to 9.7 and 11 of the manuscript and Section 6 of the plain-language companion carry the
full list.

## Contents

| path | what |
|------|------|
| `paper/` | the manuscript *ZeeBeam: The Zero-Knowledge Beam* (v3.20: the v3.16 text after nineteen referee passes, with the title given its paper form and Section 8.1 recording the one look at the verification session): Markdown, HTML, PDF |
| `companions/` | `zeebeam_worked_examples` (a graded walk from a hash preimage to the proofs in the paper, with runnable checks) and `zeebeam_for_dummies` (the same claims and caveats without formulas) |
| `figures/` | the boundary diagram and cost strip for one proved row, and the script that draws them |
| `bundle/proofs_20260902/` | the artifact: 259 row proofs under the final key, two previous-revision row proofs and the chain proof, statements, pins, decoded statements, prover manifests and logs, the frozen source tree carrying every path-dependency crate's bytes with a staging script for the layout its manifests expect and fail-closed reproducible-build drivers, the standalone verifier, the Python oracles, the 260-row anchored prefix and receipt, and `SHA256SUMS` over its files. Start with `VERIFY.md` |
| `LICENSE`, `THIRD_PARTY_NOTICES.md`, `licenses/` | non-commercial research, teaching, verification and private study; all other rights reserved; the third-party components, including the code compiled into the guest ELFs, with their licence expressions and the licence texts and notices (Apache-2.0, MIT, BSD, Unicode, OFL, DejaVu, the Rust library copyright notice, the compiler-builtins licence) |
| `AUTHORS.md`, `CITATION.cff`, `RELEASE_NOTES.md`, `SHA256SUMS` | authorship, citation, release notes, and the digest of every other file in this tree (no `.zenodo.json`: a deposit for a persistent identifier is to be created by hand with the component licences declared separately) |
| `referee_trail/` | twenty-one reports by a second model (GPT-5.6 through `codex exec`): eleven manuscript referee passes, the companions audit, the release audit, a verification pass on the applied text and seven closing passes, mechanically redacted for runner metadata, provider identifiers and machine paths with substantive findings unchanged (`referee_trail/README.md`). The manuscript states that this is not independent validation |

## Verify it

```
cd bundle/proofs_20260902 && sha256sum -c SHA256SUMS
cd verifier/standalone_verifier && cargo build --release --locked
./target/release/zeebeam-standalone-verifier ../../all_rows/proofs/row_001_groth16_proof.bin ../../all_rows/proofs/row_001_groth16_public_values.bin 0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490
```

Expected: `VERIFIED`, with the flipped-byte and wrong-key controls rejected. `VERIFY.md` gives the same
steps for every row, for the chain proof (key `0x005402a848fdc787e32c0a110ca2662dd61c580481b4e418d5400a52fbd7df50`),
for decoding a statement, and for the two checks no proof can do for you: that block 3456294 with header
hash `000000000052f2b0c7e7929e16086c3d15f09922b050109ec722c8576ffddf0b` is the real Zcash mainnet block
containing transaction `8d1672155e98000498070bd76479e0363b8b9ee0b61f0d32a802d35ef5a4f206`, and that
the 66 distinct quicknet rounds the session used, spanning 31521605 to 31521705, match the public schedule
(no early release is the manuscript's assumption A4). An offline kit with the raw block, the raw transaction,
the 66 beacon rounds, the toolchain tarballs and the vendored crates is archived separately and referenced by
digest in `RELEASE_NOTES.md`.

## Provenance

Principal: Cathal Ryan Hynes (PolieBotics). Drafted, built, proved and verified with BOSUN, the
project's automated research assistant, whose development-machine paths appear in the build records.
The proving ran on rented Lambda A100 machines on 2 and 3 September 2026 for about USD 140 in total;
the exact costs, timings and one network incident are disclosed in the manuscript's Section 7.

## Authorship

All material in this repository is authored by Cathal Ryan Hynes (PolieBotics), who directed, reviewed and released it. BOSUN, an
automated research assistant he operates, drafted the text, built the artefacts and prepared the commits, which therefore carry
BOSUN's identity as the operator up to this commit; from this commit onward, commits are authored as Cathal Ryan Hynes with BOSUN
as committer. `RELEASE_NOTES.md` and `THIRD_PARTY_NOTICES.md` carry the author-of-record line, `author: Cathal Ryan Hynes (author of record); drafted with BOSUN, the project's automated research assistant`. Fourteen documents keep the author line they were released with, because their bytes are fixed inside
pinned artefacts and are read under this section: the manuscript `paper/zeebeam.md` and the companions `companions/zeebeam_for_dummies.md`
and `companions/zeebeam_worked_examples.md`, whose HTML and PDF renderings derive from them, and the eleven documents of the proof bundle
that `bundle/proofs_20260902/SHA256SUMS` lists: `ENVIRONMENT.md`, `VERIFY.md`, `source/README.md`, `source/rust/row_binding_join_membership_sp1_candidate/README.md`, `final_relation/GPU_MEMORY_OBSERVATIONS.md`, `final_relation/boundary_row_095/README.md`, `ml/realness/README.md`, `ml/realness/zeebeam_realness_results_20260901.md`, `ml/realness/code/README.md`, `ml/realness/code/RUNBOOK.md`, `ml/realness/code/LIMITATIONS.md`.
`AUTHORS.md` states the same in two lines; `CITATION.cff` carries the citation metadata.

## Licence

Non-commercial research, teaching, verification and private study are permitted; all other rights are
reserved. The terms are in `LICENSE`. Third-party code and data, including the crates compiled into the guest ELFs,
keep their own licences: `THIRD_PARTY_NOTICES.md` lists them and `licenses/` carries the licence texts.

— BOSUN ⚓
