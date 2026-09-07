Hoy.

Verdict: **not yet; do not push**. The ledgers and frozen Git tree are sound. The release remains blocked by residual infrastructure disclosure, the absent offline-kit endpoint, and a mutable-message race in the publication script. No file was modified.

## 1. Prior-report dispositions

### Release ultra report

| # | Disposition | Evidence or remainder |
|---:|---|---|
| 1 | RESOLVED | README now says, “For each of the 259 anchored rows released here…” |
| 2 | RESOLVED | “aboard CittaDel” was removed. |
| 3 | RESOLVED | README now describes the preceding-row record, \(S_t\), and nominal emission. |
| 4 | RESOLVED | All six Table 1 corrections landed at README lines 21–29. |
| 5 | RESOLVED | README lines 37–47 now state P1/P2, relay/location limits, chain scope, and rows 260–711 correctly. |
| 6 | RESOLVED | README now says 66 distinct rounds and identifies A4. |
| 7 | RESOLVED | README names the two previous-revision proofs. |
| 8 | RESOLVED | Root ledger covers the nested bundle ledger: 3,097 non-self entries for 3,098 files, all passing. |
| 9 | NOT ADDRESSED | RELEASE_NOTES still says, “The archive location is not yet fixed.” |
| 10 | PARTIALLY | Paper, companions and VERIFY use public-release language, but RELEASE_NOTES front matter still says `status: release-1.0.0-candidate-nothing-published`. |
| 11 | RESOLVED | Offline manifest/ledger distinction and lack of air-gapped rehearsal are stated correctly. |
| 12 | RESOLVED | LICENSE defines and applies “Non-commercial” to every grant. |
| 13 | RESOLVED | LICENSE now defines the holder-owned corpus and excludes third-party material. |
| 14 | RESOLVED | Published results, modification limits, redistribution and sublicensing are now explicit. |
| 15 | RESOLVED | Attribution formula and statutory-rights saving clause added. |
| 16 | PARTIALLY | Notices exist, but compiled third-party code is mislabeled “not distributed”; DejaVu licence text is absent; GPU-server terms remain undetermined; Blockchair redistribution remains unconfirmed. |
| 17 | RESOLVED | CFF and Zenodo describe software release 1.0.0; manuscript 3.5 is the preferred citation. |
| 18 | RESOLVED | Cathal is consistently sole author/creator; BOSUN is acknowledged. |
| 19 | PARTIALLY | Prose limits the custom licence, but Zenodo still assigns the entire mixed deposit the single machine-readable value `"other-nc"`. |
| 20 | RESOLVED | CFF/Zenodo now say supplied transaction/header and out-of-band canonical-mainnet check. |
| 21 | PARTIALLY | Main naming was fixed, but the former phrase remains in current paper line 18; historical naming remains inadequately marked as superseded. |
| 22 | RESOLVED | Current manuscript says eleven passes and VERIFY 2.7. |
| 23 | PARTIALLY | VERIFY is correct, but ENVIRONMENT says `sp1-verifier 6.4.0 alone`; verifier source says “Nothing else is needed” while importing `sha2`. |
| 24 | PARTIALLY | Figure status and most language were fixed. Source still cites a nonexistent `docs/research/...` path and labels the in-proof node “anchor tx, on mainnet”. |
| 25 | RESOLVED | Script excludes only root `./SHA256SUMS` and catches unlisted nested ledgers. |
| 26 | PARTIALLY | Post-add/post-commit tree checks, message comparison, exact-OID push and `LC_ALL=C` landed. Message TOCTOU, inherited hooks/config and lack of a parentless-root assertion remain. |
| 27 | PARTIALLY | Most false preview claims were removed. It still claims both audits were applied, advertises absent files, and remains publication-incomplete. |
| 28 | RESOLVED | Manifest digest now ends `…f16f2a785`. |
| 29 | RESOLVED | Go 1.24 requirement, missing modules/toolchain and lack of rehearsal are disclosed. |
| 30 | PARTIALLY | Most trail debris was removed, but UUIDs, IPs, instance IDs, a `[runner path redacted]` runner path, broken links and substance-changing self-redaction remain. |
| 31 | PARTIALLY | `fleet_instances.json` was redacted, but sibling logs and the published report reproduce the identifiers and addresses. |
| 32 | RESOLVED | `.pyc`/`__pycache__` and its embedded path are absent. |
| 33 | PARTIALLY | `[principal call name redacted]` and `[internal assistant call sign redacted]` are absent. `[principal crew name redacted]` occurs once, and LICENSE still contains the specifically challenged `Truth Beam`. |
| 34 | RESOLVED | 1.6 seconds, “every other file”, and the Lambda-image caveat are corrected. |

### Manuscript ultra report

| # | Disposition | Evidence or remainder |
|---:|---|---|
| 1 | RESOLVED | A6 and Proposition 4 now use canonical-block verification correctly. |
| 2 | RESOLVED | All five formerly missing crate trees are physically present under `source/rust` and `source/deps`. In-place path resolution is a new remaining defect below. |
| 3 | RESOLVED | Bundled realness report is v2.0 with matching substantive bytes. |
| 4 | RESOLVED | Replay after release is explicitly not excluded. |
| 5 | RESOLVED | Chain proof no longer claims when the predecessor was fixed. |
| 6 | PARTIALLY | Paper and dummies define threshold \(\tau\); worked companion still says only “a high coupling score”. |
| 7 | RESOLVED | Conditional physical-origin language is substantially corrected. |
| 8 | RESOLVED | A1/A2 are scoped to each ELF and key. |
| 9 | PARTIALLY | Paper witness tuple is fixed; worked companion omits \(\sigma_t\). |
| 10 | RESOLVED | 712 oracle rows and 259 final executions/proofs are distinguished. |
| 11 | PARTIALLY | Paper is corrected, but worked line 357 still says the standalone verifier compares fixed fields with Appendix A. |
| 12 | RESOLVED | Disclosure list is complete. |
| 13 | RESOLVED | Model replacement now requires a new ELF, key and proofs. |
| 14 | PARTIALLY | Paper distinguishes system from recording; README/CFF/Zenodo again define ZeeBeam as including “the proved recordings they yield”. |
| 15 | RESOLVED | Chameleon/trapdoor qualification is correct. |
| 16 | RESOLVED | Capture-time/honest-anchor terminology was removed. |
| 17 | PARTIALLY | Paper and dummies say eleven; worked companion says ten. |
| 18 | RESOLVED | Idle time is approximately 13.5 instance-hours. |
| 19 | PARTIALLY | Section 12 is fixed, but paper lines 560–561 call 712 membership records “per-row witnesses”. |
| 20 | PARTIALLY | VERIFY is 2.7 with 262 proofs; worked companion retains v2.6 and obsolete ledger counts. |
| 21 | RESOLVED | Current publication/authorship statements are corrected. Dated historical log entries are not current claims. |
| 22 | RESOLVED | Appendix F identifies unbundled records and missing preimage material. |
| 23 | RESOLVED | Correct chain-host lockfile path named. |
| 24 | RESOLVED | Selection fixture and 24 proved selection rows stated correctly. |
| 25 | RESOLVED | Pose evidence is described as cue-aware, not blind correctness. |
| 26 | RESOLVED | Reducer, OOD, temporal, optimiser and direction confounds are disclosed. |
| 27 | DECLINED | Stride correction declined; the decline is factually wrong. Other subparts landed. |
| 28 | RESOLVED | No hiding guarantee is claimed from byte length or hashing. |
| 29 | PARTIALLY | Worked anchor explanation still says “the receipt’s digest is included in the transaction”; witness, threshold and verification metadata are stale. |
| 30 | PARTIALLY | Plain-language companion still says “whose receipt a Zcash block carries” and narrates chronology too directly. |
| 31 | RESOLVED | Both `companion_to` fields identify manuscript 3.5 and valid release-relative paths. |
| 32 | RESOLVED | Hostile-reader phrases and row-95 wording were corrected. |
| 33 | RESOLVED | Scalar-half qualification, both transitions and Section 9.4 cross-reference landed. |
| 34 | PARTIALLY | Root-ledger count is fixed; metadata still has ZeeBeam-referent and receipt-placement errors. |

## 2. Stride adjudication

**The referee was right.**

The authors are correct only that the explicit arrays in `ml/pose_ladder_results.json` are `[2,2,2,2,2]`. Those arrays describe the pose network, with widths 8, 12, 16, 24, 24.

The disputed sentence describes the coupling network. Its proved source, [model/src/lib.rs](../bundle/proofs_20260902/source/deps/BOSUN/scratch/zeebeam_lambda_launch_prep_20260823/zeebeam-trained-r32-ptq-v2-sp1-v1/model/src/lib.rs#L86), records:

- sides 32→32→16→8→4→2;
- widths 24, 32, 48, 64, 64;
- strides at lines 92, 102, 112, 122 and 132: **1, 2, 2, 2, 2**.

`ml/coupling_ladder_results.json` contains no stride array. Absence there does not override the proved model source. The authors conflated the coupling and pose architectures.

Replace paper line 574 exactly with:

> Five 3×3 integer convolutions with strides 1, 2, 2, 2 and 2 (24, 32, 48, 64, 64 channels) and a 1×1 head…

## 3. Bundle and source closure

All five public ledgers verify. The bundle ledger and root ledger each cover every regular file except themselves, without duplicate, missing or unlisted paths.

The source is **physically complete but not path-resolving in place**:

| Path class | Entries | Resolve inside unpacked bundle |
|---|---:|---:|
| `[lib]` / `[[bin]]` source paths | 10 | 10 |
| Dependency and patch paths | 30 | 17 |
| Total literal `path` keys | 40 | 27 |

The 13 failures are:

- Seven `../../../../scratch/...` dependencies: five in `row_binding_join_sp1_candidate/join/Cargo.toml`, one in membership, one in the chain program. Their bytes exist under `source/deps/BOSUN/scratch/...`, but the manifest paths resolve to nonexistent `proofs_20260902/scratch/...`.
- Six absolute `jubjub` patches pointing to `/home/c/Documents/BOSUN/scratch/joined_build_20260901/vendor/jubjub-0.10.0`. The bundled copy is `source/vendor/jubjub-0.10.0`.

For a portable layout, the seven scratch paths should be `../../../deps/BOSUN/scratch/...`; root-workspace jubjub paths should be `../../vendor/jubjub-0.10.0`, and nested program/script paths `../../../vendor/jubjub-0.10.0`. Because these are proved source bytes, a deterministic staging/symlink wrapper is preferable to silently editing them.

Answer: **No, not as unpacked. Every dependency crate’s bytes are present, but 13 of 30 dependency/patch paths do not resolve against the bundled tree.**

## 4. Recomputed counts and digests

| Check | Recomputed result |
|---|---|
| Public regular files | 3,098 |
| Public logical size | 17,872,517 bytes |
| Root ledger | 3,097 entries; 3,097 pass; exact non-self coverage |
| Root ledger SHA-256 | `c9517012bb99267aaf219c66cec100b1678e4a3c3ec5ae72a9d5a38ec4f19e46` |
| Bundle files / ledger | 3,065 / 3,064; exact non-self coverage |
| Bundle-ledger SHA-256 | `dfd6cc4cc7b26d6e1fa16e24c06c7e18fe8e911a9cfa1014997a765ed200a7f0` |
| SOURCE_TREE | 38/38 pass |
| SOURCE_TREE ledger SHA-256 | `e45be3c065f31b720418ba78504616dbabcd35275c8f7afdbfd6a94e634437a2` |
| DEPENDENCY | 57/57 pass |
| DEPENDENCY ledger SHA-256 | `e8a586248d520f23f5dffcc84f58ebe557fd5a160bbf004468af32754abe6bc0` |
| CHAIN_SOURCE | 7/7 pass |
| CHAIN_SOURCE ledger SHA-256 | `c35d3230905f006aa529e3561bf3ea70f98e81cff307d443e1b113e9df7bf2ac` |
| Raw / framed / public-value proof files | 262 / 262 / 262 |
| Proof allocation | 257 fleet + 2 ceremony-2 + 2 previous revision + 1 chain |
| Trail | 13 reports + README |
| Cargo manifests examined | 18 |
| Files at Git mode 100644 / 100755 | 3,086 / 12 |
| Recomputed Git SHA-1 tree | `e883f79574e803fe89365064af98fcad49767b92` |
| Frozen commit-body SHA-256 | `920aee89f9ffd214c32e1f6cae03355eae582eda983809ae064e320acf7dda1b` |

The sandbox did not permit a writable temporary repository. I recomputed the Git tree directly from Git object serialization; it matches the frozen value exactly.

Offline material supplied:

| Item | Result |
|---|---|
| Offline ledger | 33,754 lines |
| Offline-ledger SHA-256 | `15b69153eb9ebb8897558fc5efeca284a1668add129d42a43b938206471a2055` |
| Supplied MANIFEST SHA-256 | `ab91008603d2e71b4953088e970bebf38f5d3bafde74a92e3dda2907aa3dc820`, matches ledger |
| Supplied notices SHA-256 | `fdae8a5079025dac6921d1f70a8f0abb993b3f14e4b714197909312651d93387`, matches ledger |
| Claimed archive | 683,561,582 bytes, `cec1b091…7b57e`; archive absent, therefore not independently verifiable |

## 5. Identifier and path inventory

### Infrastructure identifiers and addresses

There are **34 genuine IPv4 occurrences, 14 unique addresses**. Three additional PDF metadata hits of `152.0.0.0` are Chromium version strings, not addresses.

| Box | IP | Instance ID | Occurrences |
|---:|---|---|---|
| 0 | `[public address redacted]` | `[provider identifier redacted]` | IP in orchestrator:1, resume:1, trail:251; ID in resume:8 and trail:251 |
| 1 | `[public address redacted]` | `[provider identifier redacted]` | corresponding two logs and trail:252 |
| 2 | `[public address redacted]` | `[provider identifier redacted]` | corresponding two logs and trail:254 |
| 3 | `[public address redacted]` | `[provider identifier redacted]` | corresponding two logs and trail:256 |
| 4 | `[public address redacted]` | `[provider identifier redacted]` | corresponding logs/trail:257, plus realness README twice, scorer config once and trail:265 |
| 5 | `[public address redacted]` | `[provider identifier redacted]` | corresponding two logs and trail:253 |
| 6 | `[public address redacted]` | `[provider identifier redacted]` | corresponding two logs and trail:258 |
| 7 | `[public address redacted]` | `[provider identifier redacted]` | corresponding two logs and trail:255 |

Additional occurrences:

- Ceremony 2 log lines 1 and 43: `[public address redacted]` and `[provider identifier redacted]`.
- The ceremony identifier prefix `[provider identifier redacted]` also occurs in `PINS_final.json` and ENVIRONMENT.
- Trail lines 264–265 contain `[public address redacted]`, `[public address redacted]`, `[public address redacted]`, `[public address redacted]`, `[public address redacted]`, plus the repeated `[public address redacted]`.

Total provider-ID mentions: **19**, comprising 17 full-ID occurrences and two additional abbreviated-prefix occurrences. The logs record the machines as terminated, but the release’s claim that the identifiers were removed is false.

Operational runner UUIDs still present:

- `[run identifier redacted]`
- `[run identifier redacted]`

Both are in `referee_trail/FINDINGS_release_ultra.md:236-237`.

The scientific session identifier `ZEEBEAM_MAINNET_BLOCKING_TRAINING_300S_20260822_001` intentionally occurs **549 times in 547 files**: 514 fleet proof/public-value binaries, 17 final-relation artifacts, two source files and 16 other records. Data-path labels `live_300s_training_001` and `campaign_recording_001` occur 29 times/17 files and three times/three files respectively.

The principal’s crew name, `[principal crew name redacted]`, occurs once, at trail line 155 inside the self-negating sentence “`[principal crew name redacted]` is absent.” `[principal call name redacted]` and `[internal assistant call sign redacted]` have zero occurrences.

### Machine paths

Grouped exhaustive inventory of identifying absolute prefixes:

| Prefix | Occurrences / files | Complete grouping |
|---|---:|---|
| `/home/c/` | 405 / 306 | 257 row stdout logs; `box_guest.elf` 12; root row logs 5; final-relation files 33; ML 47; verifier 5; top-level all_rows scripts 4; source 14; fleet scripts 27; trail 1 |
| `/home/ubuntu/` | 566 / 263 | 521 in 257 row stderr logs; `box_guest.elf` 32; root row stderr 5; final-relation logs 8 |
| `/lambda/nfs/` | 6 / 3 | realness README 1, RUNBOOK 4, scorer config 1 |
| `/tmp/` | 2 / 1 | fleet bootstrap 2 |
| `[runner path redacted]` | 1 / 1 | referee round 6 runner-error line 1 |
| `/data/zeebeam_evidence_20260822` | 25 / 14 | fleet scripts, ceremony/root stderr, pose annotation, chain host source and two verifier scripts |
| `/usr/local/go` | 8 / 4 | bootstrap/row/fleet scripts |

The 257 stderr files contain two `/home/ubuntu/` occurrences each, except box1 row002 through box7 row008, which contain three each. Paper Markdown/HTML and trail prose additionally mention bare `/home/c` and `/home/ubuntu`; the rendered PDF repeats one of each.

Generic OS paths also remain: `/usr/bin/env` 24 times, `/usr/local` once, `/dev/null` 30, `/dev/vda1` once, `/proc/self/status` once, and six compiled standard-library `/sys/...` paths. These are not identifying, but they defeat any literal “no machine paths” claim.

## 6. Publication script

[006_zeebeam_repo.sh](<extras>/006_zeebeam_repo.sh:1) enforces most, but not all, of its [preview](<extras>/006_zeebeam_repo.preview.md:1).

| Claimed control | Result |
|---|---|
| `LC_ALL=C` | PASS |
| Frozen root-ledger digest | PASS |
| 3,097 entries / 3,098 files | PASS |
| Every listed file matches | PASS |
| No unlisted regular file | PASS |
| No symlink/special node | PASS |
| No `.git` anywhere | PARTIAL: only root `.git` is tested; an empty nested `.git` directory passes |
| Frozen message | FAIL under concurrent mutation: hash, commit and comparison reread the mutable file |
| Git tree after `add` | PASS |
| Committed tree | PASS |
| Committed message equals then-current file | PASS, but not necessarily the originally hashed bytes |
| Exact commit pushed | PASS |
| Inherited hooks/config excluded | FAIL |
| Parentless first commit asserted | FAIL |

The message race is real: lines 29–30 hash `$MSG`, line 41 rereads it, and line 44 compares the commit with another reread. Freeze a snapshot and use it for all three operations, or hash the committed payload directly against `EXPECTED_MSG`.

The preview additionally names absent `006_zeebeam_repo.files.txt`. Its “40,069 files” leak-scan count is not a regular-file count: public plus the represented offline kit is 3,098 + 33,755 = 36,853 regular files.

## 7. Numbered new or remaining defects

1. **Blocking | bundle logs, trail, preview | infrastructure disclosure.**

   Quotes:

   > “fleet instance identifiers and IP addresses removed”  
   > “two run identifiers … were removed”

   Both are false. Replace every address/ID with `[public address redacted]`, `[provider identifier redacted]` and `[run identifier redacted]`; preserve per-box pseudonyms and termination facts. Regenerate both ledgers and the Git tree.

2. **Blocking | RELEASE_NOTES.md:51 | missing release artifact.**

   Quote:

   > “The archive location is not yet fixed. This file will name it when it is.”

   Replace with the stable URL or DOI, archive name, exact size and SHA-256. The archive itself was not supplied for this verification.

3. **Blocking | publication script lines 29–44 | mutable-message TOCTOU.**

   Quote:

   > `msg_actual=$(sha256sum "$MSG" …)`  
   > `$GIT commit -q -F "$MSG"`

   Freeze one byte-copy, hash it, commit it, and compare the committed payload with that frozen copy or directly with `EXPECTED_MSG`.

4. **Must-fix | source manifests and release claims | dependency paths do not resolve.**

   Quote:

   > “the frozen source tree with every path dependency”

   Replace with:

   > the frozen source tree carrying every path-dependency crate’s bytes; 13 dependency/patch paths require the mirrored staging layout described in `source/README.md`

   Better: ship and ledger a deterministic staging wrapper that constructs the required geometry.

5. **Must-fix | paper line 574 | wrong coupling architecture.**

   Quote:

   > “Five 3×3 strided integer convolutions”

   Replace with the explicit `[1,2,2,2,2]` wording given above.

6. **Must-fix | paper abstract, CFF, Zenodo, both companions and figure | Zcash/receipt regressions.**

   Quotes include:

   > “a Zcash mainnet transaction”  
   > “carries a memo opening to a receipt”  
   > “the receipt’s digest is included in the transaction”  
   > “whose receipt a Zcash block carries”  
   > “anchor tx, on mainnet”

   Replace with:

   > A supplied Zcash v6 transaction is recomputed from raw bytes and its supplied branch checked to a supplied header; canonical-mainnet status is checked out of band. The action-0 ciphertext decrypts to a memo binding equal to SHA-256 of the proved receipt. The receipt, `contentRoot` and chameleon opening then relate the supplied prefix to the commitment.

   Dummies lines 92 and 126 should say “The operator’s record says…” and “Project records place…; the proof does not establish that chronology.”

7. **Must-fix | worked companion and ENVIRONMENT | stale versions, counts and dependency description.**

   Quotes:

   > “`VERIFY.md` v2.6”  
   > “3,052, 38, 7 and 44 entries”  
   > “referee rounds | ten”  
   > “`sp1-verifier` 6.4.0 alone”

   Replace with `v2.7`; `3,064, 38, 7 and 57`; eleven; and:

   > Standalone cryptographic verification uses `sp1-verifier` 6.4.0; the utility directly uses `sha2` 0.10 for digest output.

   Regenerate companion HTML/PDF.

8. **Must-fix | paper and worked companion | witness, threshold and verifier taxonomy.**

   Replace the worked witness list with:

   > The private witness consists of thirteen guest inputs: row header, membership witness, raw frame, \(\sigma_t\), anchor transaction bytes, anchor Merkle branch, anchor block header, incoming viewing key, chain-log prefix, `PREFIX.json`, receipt, opening and trapdoor.

   Replace “a high coupling score” with the paper’s fixed-\(\tau\) P2 premise.

   Replace worked line 357 with:

   > Fixed fields are enforced by the guest authenticated by the pinned key; an independent verifier may decode the statement and compare them with `PINS_final.json` and Appendix A.

   Replace paper lines 560–561 with:

   > The oracle values and per-row membership records for all 712 rows, the executed statements for rows 1 through 259, and the figure are in the bundle.

9. **Must-fix | referee trail | redaction description is false and substance changed.**

   The two UUIDs, all infrastructure values, `[runner path redacted]`, and `[principal crew name redacted]` remain. Self-redaction changed:

   > “`BOSUN` occurs … replace … with `BOSUN`”

   into the nonsensical:

   > “`BOSUN` occurs … replace … with `BOSUN`”

   Use `[internal assistant call sign redacted]` for the evidentiary token, retain `BOSUN` only as the proposed replacement, change the opener from `Cathal` to `the principal`, and prefix the 25 current-tree links in round11/release with `../`.

10. **Must-fix | THIRD_PARTY_NOTICES.md | incomplete and false distribution mapping.**

    Quote:

    > “every other dependency … not distributed in this tree”

    Their source copies may be absent, but their code is incorporated into the three shipped guest ELFs. Say that explicitly, ship applicable licence/NOTICE texts, include the DejaVu licence, and correct:

    > `final_relation/DEPENDENCY_SHA256SUMS`

    to:

    > `bundle/proofs_20260902/final_relation/DEPENDENCY_SHA256SUMS`

    Do not publish the offline kit until GPU-server licensing and Blockchair-response redistribution are settled.

11. **Must-fix | LICENSE 1.1 and Zenodo | mixed licensing and teaching ambiguity.**

    Core v1.0 contradictions are fixed, but §2(c) permits teaching copies while §3(b) permits redistribution only “within a research group”. Define research group/class/collaborator and permit the intended classroom copying explicitly.

    Change “identified in section 4 and listed” to “identified in section 4 or listed”, and add embedded fonts and compiled third-party code to §4.

    Define whether trained weights, independent reimplementations and derived outputs may be shared. Replace vague “necessary” patent rights with either a defined necessary-claims grant or “No patent licence is granted.”

    Delete the deposit-wide `"license": "other-nc"` if the importer cannot represent the mixed licences; set the custom and third-party licences after import or split the deposit.

12. **Must-fix | paper, RELEASE_NOTES and preview | false audit-status claims.**

    Quotes:

    > “all applied”  
    > “both Sol ultra audits applied”

    One item was declined and several were missed. Replace with:

    > all adjudicated

    only after the remaining accepted repairs are actually applied. Also change release front matter to `status: release-1.0.0`.

13. **Should-fix | paper lines 43, 121, 607 and 615 | “preregistered” contradicts the evidence caveat.**

    Paper lines 681–683 say the note and timestamp evidence are absent and it was not immutable preregistration. Replace “preregistered/preregistration” with “predeclared plan” throughout.

14. **Should-fix | README/CFF/paper | scope overbreadth.**

    Replace:

    > “ZeeBeam is … and the proved recordings they yield”

    with:

    > ZeeBeam is the projector-camera capture-and-proof system. A ZeeBeam recording is a session with rows proved under that system.

    Replace CFF’s universal statement-size claim with:

    > Every raw Groth16 proof is 356 bytes. Final-relation row statements are 1,101 bytes, the two previous-revision statements are 1,085 bytes, and the chain statement is 279 bytes.

    Replace “verify with one crate” with “verify with the bundled verifier crate and its direct `sp1-verifier` and `sha2` dependencies.”

15. **Should-fix | figures/zeebeam.py | stale source and in-proof mainnet label.**

    Replace nonexistent `docs/research/zeebeam_one_proof_architecture_20260901.md` with `paper/zeebeam.md, Sections 5–7 and 9`.

    Replace “anchor tx, on mainnet” with:

    > supplied anchor transaction; canonical-mainnet status checked out of band

    Regenerate PNG/SVG.

16. **Should-fix | publication script/preview | remaining enforcement and inventory claims.**

    Reject `.git` recursively, disable inherited Git hooks/config/attributes/templates, assert the commit has no parent, and post-check author/committer. Include `006_zeebeam_repo.files.txt` or delete its mention. Replace “40,069 files” with the actual regular-file universe and state exactly what was scanned.

17. **Should-fix | paper/LICENSE/trail | naming residue.**

    Remove the current paper’s extra “ZK all the things” explanation if the phrase is authorized only in README/Table 1. Delete `Truth Beam, ` from LICENSE unless intentionally reserved. Replace trail line 155 with “The principal’s crew name is absent,” so it stops publishing the very name it claims is absent.

## 8. LICENSE 1.1 conclusion

The principal commercial-scope, corpus, modification, attribution, statutory-rights and cure contradictions are repaired. It is not yet publication-clean because classroom redistribution, group boundaries, derived models/reimplementations, patent scope and incorporated third-party licensing remain undefined or inconsistent. README’s claim that all third-party licences are “listed” exceeds the public notice actually supplied.

## 9. One-line verdicts

- **Manuscript v3.5:** **Not yet**, coupling stride, abstract transaction language, witness taxonomy and audit-status claims remain wrong.
- **ZeeBeam, by Example v1.4:** **Not yet**, stale VERIFY/count/round metadata plus witness, threshold, verifier and receipt errors remain.
- **Plain-language companion v1.4:** **Not yet**, chronology and “receipt a block carries” still overstate the evidence.
- **Repository release 1.0.0 as staged:** **Do not push**, blocked by identifier disclosure, missing offline endpoint and publication-message TOCTOU; source closure and third-party licensing also require repair.

— BOSUN ⚓
