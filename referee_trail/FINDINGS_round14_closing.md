Hoy.

**Verdict: DO NOT PUSH.** Seven of the thirteen findings are resolved, five are partial, and the Zenodo decline is unsound. The principal’s open decisions do not cure the remaining publication and licence blockers.

## 1. Disposition of the 13 findings

| # | Disposition | Evidence or remainder |
|---:|---|---|
| 1 | **PARTIALLY** | [Script lines 60–65](<extras>/008_zeebeam_repo.sh:60) create private, use an explicit `gh` credential helper, read back `main`, then change visibility. Git GUI askpass remains possible. |
| 2 | **RESOLVED** | [Lines 57–58](<extras>/008_zeebeam_repo.sh:57) hash the message bytes read from the commit object against fixed `EXPECTED_MSG`. |
| 3 | **PARTIALLY** | Atomic `.git`, hooks, attributes, excludes, parentless and identity checks landed. `GIT_EXEC_PATH`, `GIT_SSL_NO_VERIFY` and other inherited Git variables remain active. |
| 4 | **RESOLVED** | [Manuscript line 34](../paper/zeebeam.md?plain=1#L34): “whose `binding=` value ... equals SHA-256 of a receipt”. |
| 5 | **RESOLVED** | [Worked lines 298–302](../companions/zeebeam_worked_examples.md?plain=1#L298) qualify chronology; [lines 411–413](../companions/zeebeam_worked_examples.md?plain=1#L411) fix τ before applying Proposition 2. |
| 6 | **PARTIALLY** | Ledger count is now 3,065. Manuscript and worked companion still cite VERIFY 2.8; actual guide is 2.9. |
| 7 | **PARTIALLY** | The crate inventory and three newly added BSD/Unicode files are exact. Font, Rust-runtime, OFL and MIT notices remain incomplete. |
| 8 | **DECLINED, UNSOUND** | Legacy `.zenodo.json` has one scalar, but Zenodo states that selected licence applies to all deposited files. `other-nc` therefore mislabels permissively licensed components; prose does not override it. |
| 9 | **PARTIALLY** | Sensitive values are absent, but round 12 still contains altered evidence and false counts. |
| 10 | **RESOLVED** | [Source README 1.4](../bundle/proofs_20260902/source/README.md?plain=1#L10) states exact coverage and bundle-only rebuild limits. The fixed-root script scans 19 manifests and all 40 paths. |
| 11 | **RESOLVED** | Manuscript, release notes and regenerated figure consistently record thirteen passes; the previous audit-status clause is gone. |
| 12 | **RESOLVED** | Inventory and environment fixes landed. The `Truth Beam` subfinding is **DECLINED, sound**. |
| 13 | **RESOLVED** | VERIFY and ENVIRONMENT now use “proof bundle”. |

The three stated declines:

- **Truth Beam:** sound. It is a registered mark of the principal and appears only in a no-endorsement clause. The official register confirms the mark and status. [Irish trademark register](https://eregister.ipoi.gov.ie/register/TMRegister.aspx?idappli=264324)
- **Zenodo:** unsound. The importer’s scalar limitation is real, but the selected scalar applies to every file, while current Zenodo supports custom and mixed licences. [Legacy `.zenodo.json`](https://help.zenodo.org/docs/github/describe-software/zenodo-json/), [deposit API semantics](https://developers.zenodo.org/), [mixed-licence guidance](https://help.zenodo.org/docs/deposit/describe-records/licenses/)
- **Stride:** the initial decline was wrong. The correction is now applied at [paper line 576](../paper/zeebeam.md?plain=1#L576), so its final disposition is **RESOLVED**, not currently declined.

## 2. Remaining defects

1. **BLOCKING: Git askpass remains enabled.**  
   [Script line 17](<extras>/008_zeebeam_repo.sh:17):

   > `export GIT_TERMINAL_PROMPT=0 GH_PROMPT_DISABLED=1 GH_HOST=github.com`

   Replace with:

   ```bash
   export GIT_TERMINAL_PROMPT=0 GIT_ASKPASS=/bin/false SSH_ASKPASS=/bin/false GH_PROMPT_DISABLED=1 GH_HOST=github.com
   ```

   Add to the `GIT=` options:

   ```text
   -c credential.interactive=false -c core.askPass=/bin/false
   ```

2. **BLOCKING: incomplete inherited-environment isolation.**  
   Replace [line 16](<extras>/008_zeebeam_repo.sh:16) with:

   ```bash
   unset GIT_CONFIG GIT_CONFIG_COUNT GIT_CONFIG_PARAMETERS GIT_DIR GIT_WORK_TREE GIT_COMMON_DIR GIT_INDEX_FILE GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_SHALLOW_FILE GIT_NAMESPACE GIT_REPLACE_REF_BASE GIT_EXEC_PATH GIT_ATTR_SOURCE GIT_DEFAULT_REF_FORMAT GIT_SSL_NO_VERIFY
   ```

   `GIT_EXEC_PATH` can redirect `git-remote-https`; `GIT_SSL_NO_VERIFY` can disable TLS verification.

3. **SHOULD-FIX: final public state is printed, not asserted.**  
   Replace [line 66](<extras>/008_zeebeam_repo.sh:66) with:

   ```bash
   published_state=$(gh repo view "$REPO" --json url,visibility,defaultBranchRef --jq '.url+" "+.visibility+" branch="+.defaultBranchRef.name')
   if [ "$published_state" != "https://github.com/poliebotics/zeebeam PUBLIC branch=main" ]; then
     echo "REFUSED: unexpected published state: $published_state" >&2
     exit 1
   fi
   echo "$published_state"
   ```

   The read-back and visibility APIs cannot be atomic. Fire only with exclusive repository administration. A lost PATCH response can leave the complete repository public while the script exits nonzero; inspect remotely before retrying. Normal failures through line 64 leave no public partial state.

4. **BLOCKING: false Zenodo licence metadata.**  
   .zenodo.json lines 10 and 25 (`.zenodo.json`, line 10) assert `other-nc` deposit-wide.

   There is no truthful replacement scalar. Remove `.zenodo.json` from automatic archiving and create or correct the Zenodo record manually with multiple component licences before publication. Replace the release-note rationale with:

   > Zenodo’s legacy `.zenodo.json` importer cannot represent this mixed-licence release, so no final deposit-wide scalar is asserted; the record must declare the custom owner licence and the third-party component licences separately before publication.

5. **BLOCKING: Liberation copyright notice missing.**  
   [OFL file lines 1–2](../licenses/OFL-1.1-Liberation.txt#L1) names the font but omits the copyright required by OFL condition 2.

   Insert:

   ```text
   Copyright:
    Digitized data copyright (c) 2010 Google Corporation with Reserved Font Arimo, Tinos and Cousine.
    Copyright (c) 2012 Red Hat, Inc. with Reserved Font Name Liberation.
   ```

   Lines 96–118 are an unrelated Debian packaging GPL stanza and should be removed.

6. **BLOCKING: Rust notice is truncated and provenance-mismatched.**  
   Rust-COPYRIGHT line 755 (`licenses/Rust-COPYRIGHT.txt`, line 755):

   > `[truncated]`

   Replace the entire file with the complete, lossless `COPYRIGHT-library.html` generated from the exact `succinct-1.94.0-64bit` source, plus its exact `compiler-builtins` licence/notice. The current file is an unspecified host-rustup toolchain dump and cannot be repaired by deleting the marker. Rust specifically produces the library-only notice for this purpose. [Rust COPYRIGHT description](https://github.com/rust-lang/rust/blob/main/COPYRIGHT)

7. **BLOCKING: MIT copyright notices absent.**  
   [MIT.txt lines 1–3](../licenses/MIT.txt#L1) says authors are listed in THIRD_PARTY_NOTICES; they are not.

   Replace the header with:

   > MIT licence and notices. The following component-labelled sections reproduce the complete upstream MIT licence files, including their copyright notices, for every component for which this distribution relies on MIT.

   Append the exact upstream files for the 18 MIT-only crates: `bech32`, `bincode`, `bitvec`, `blake2b_simd`, `const-default`, `funty`, `generic-array`, `getset`, `libm`, `nonempty`, `radium`, `slab`, `spin`, `tap`, `tracing`, `tracing-attributes`, `tracing-core`, `wyz`. For `byteorder` and `memchr`, either append their MIT notices or explicitly elect and include the Unlicense.

8. **MUST-FIX: Rust component and expression mapping is false.**  
   `nm -C` finds `std` and `panic_abort` in all three ELFs. Replace [THIRD_PARTY_NOTICES line 17](../THIRD_PARTY_NOTICES.md?plain=1#L17) with a row naming:

   > Rust `core`, `alloc`, `std`, `panic_abort`, `compiler-builtins` and linked standard-library dependencies compiled into the three ELFs.

   The licence cell must state:

   > `core`, `alloc`, `std` and `panic_abort`: MIT OR Apache-2.0; `compiler-builtins`: MIT OR Apache-2.0 WITH LLVM-exception; other linked standard-library component terms are reproduced in the exact complete `COPYRIGHT-library.html`.

   Plain Apache-2.0 is not an alternative for `compiler-builtins`. [Upstream licence](https://github.com/rust-lang/compiler-builtins)

9. **MUST-FIX: embedded font inventory incomplete and wording false.**  
   [THIRD_PARTY_NOTICES line 21](../THIRD_PARTY_NOTICES.md?plain=1#L21) omits DejaVu Sans Mono Bold and says “the fonts themselves are not distributed”.

   Replace the PDF portion with:

   > DejaVu Serif, DejaVu Serif Bold, DejaVu Serif Italic and DejaVu Sans Mono subsets in all three PDFs, plus DejaVu Sans Mono Bold in `companions/zeebeam_worked_examples.pdf`.

   Replace the final claim with:

   > the full upstream font files are not separately distributed; the SVG and PDFs distribute embedded outlines or subsets.

10. **MUST-FIX: stale VERIFY version.**  
    Replace `2.8` with `2.9` at:

    - [paper line 1025](../paper/zeebeam.md?plain=1#L1025)
    - [paper line 1073](../paper/zeebeam.md?plain=1#L1073)
    - [worked line 433](../companions/zeebeam_worked_examples.md?plain=1#L433)
    - [worked line 506](../companions/zeebeam_worked_examples.md?plain=1#L506)

    Regenerate both HTML and PDF pairs.

11. **MUST-FIX: trail evidence remains altered.**  
    In [round 12](../referee_trail/FINDINGS_round12_verification.md?plain=1#L43):

    - Line 43: `the principal's crew name` → `` `[principal crew name redacted]` ``.
    - Line 194: replace with:  
      `The principal’s crew name, \`[principal crew name redacted]\`, occurs once, at trail line 155 inside the self-negating sentence “\`[principal crew name redacted]\` is absent.” \`[principal call name redacted]\` and \`[internal assistant call sign redacted]\` have zero occurrences.`
    - Line 205: replace with `| /tmp/ | 2 / 1 | fleet bootstrap 2 |`; record `[runner path redacted]` separately as `1 / 1`.
    - Line 335: replace `one mention of the principal's crew name` with `` `[principal crew name redacted]` ``.
    - Line 341: replace the first placeholder with `BOSUN`, preserving the criticised nonsensical result:  
      `> “\`BOSUN\` occurs … replace … with \`BOSUN\`”`

12. **MUST-FIX: trail README falsely describes a snapshot.**  
    [README lines 19–21](../referee_trail/README.md?plain=1#L19) says “nothing else differs”. Many adjudicated files differ.

    Replace with:

    > The last report examined the preceding staged tree. The present tree includes that report and the adjudicated changes made in response to it; its counts, digests, versions and cited line numbers describe the pre-closing tree.

    At line 29, replace `three reports` with `four reports`.

13. **SHOULD-FIX: stale effort description.**  
    [Paper line 473](../paper/zeebeam.md?plain=1#L473):

    > `reasoning effort high`

    Replace with:

    > `reasoning effort high for rounds 1 to 10 and ultra thereafter`

14. **SHOULD-FIX: reversed stride decision is presented as currently declined.**  
    Replace [RELEASE_NOTES line 67](../RELEASE_NOTES.md?plain=1#L67) opening with:

    > The coupling-model stride correction was initially declined against the wrong record and then applied in manuscript v3.6. Two other findings were declined:

    In preview line 87, replace `the three declined findings` with `the two current declines and the reversed stride decision`.

## 3. Sweeps, counts and digests

### Identifier sweep

| Category | Actual matches |
|---|---:|
| Public IPv4 addresses | 0 |
| Provider instance identifiers | 0 |
| Operational run UUIDs | 0 |
| Audit-runner path | 0 |
| Principal’s crew name | 0 |
| Principal’s personal call name | 0 |
| Internal assistant call sign | 0 |

The IPv4 regex finds `152.0.0.0` once in each PDF, but each is Chromium version metadata, not an address. The former UUIDs, provider values and runner temporary path are absent.

Intentional scientific identifiers remain:

- `ZEEBEAM_MAINNET_BLOCKING_TRAINING_300S_20260822_001`: 549 occurrences in 547 files.
- `live_300s_training_001`: 29 in 17 files.
- `campaign_recording_001`: 3 in 3 files.
- Generic `/tmp/go.tgz`: 2 in one bootstrap script.

### Recomputed mechanical results

| Item | Recomputed result |
|---|---|
| Public tree | 3,109 regular files; 18,013,028 bytes; no symlinks/special nodes |
| Root ledger | 3,108 unique entries; exact non-self coverage; all pass |
| Root-ledger SHA-256 | `ba67ae382a193c3de7f2e463724890ca2900ac74199922c596c043f34f1b4fba` |
| Git modes | 3,096 × `100644`; 13 × `100755` |
| Git SHA-1 tree | `c4aec526e7186f17c1b576a5a28c46876a0cbb64` |
| Bundle | 3,066 files; 14,809,148 bytes |
| Bundle ledger | 3,065 entries; all pass |
| Bundle-ledger SHA-256 | `fad30aa0cf0339a500f2d450bd718c9856e7df0534ab4f7aff8b9382e661c03e` |
| SOURCE_TREE | 38/38 pass; `e45be3c065f31b720418ba78504616dbabcd35275c8f7afdbfd6a94e634437a2` |
| DEPENDENCY | 57/57 pass; `e8a586248d520f23f5dffcc84f58ebe557fd5a160bbf004468af32754abe6bc0` |
| CHAIN_SOURCE | 7/7 pass; `c35d3230905f006aa529e3561bf3ea70f98e81cff307d443e1b113e9df7bf2ac` |
| Commit message | 1,312 bytes; `a13525315dc7ea3ec2c76bfced9b27dac24b92ed79244cadd07edc003d7ca2e1` |
| Proof inventory | 262 raw, 262 framed, 262 public-values, 262 manifests |
| Trail | 15 reports plus README, 16 files |
| Licence files | 8, but incomplete as detailed above |

The root ledger digest, Git tree and message digest exactly match the script’s frozen values. The live transmit script/body/preview equal the supplied `extras/` files, and `008_zeebeam_repo.files.txt` is byte-identical to root `SHA256SUMS`.

The three source lists jointly cover 102 of 110 source files. The eight outside their historical scopes are both blobs, both build drivers, both row-tree READMEs, source README and staging script. Bundle and root ledgers cover all eight.

### Fonts and crates

`pdffonts` reports:

- Manuscript: DejaVu Serif regular/bold/italic, DejaVu Sans Mono, Liberation Serif.
- Worked: DejaVu Serif regular/bold/italic, DejaVu Sans Mono and Mono Bold.
- Plain companion: DejaVu Serif regular/bold/italic, DejaVu Sans Mono.

The guest lockfile union contains 177 packages: ten first-party, separately noticed `jubjub`, and exactly 166 third-party name/version pairs. All 166 rows and all Cargo licence expressions match the offline vendor manifests. No crate row is missing. The missing material is notices and the Rust standard-library/runtime inventory.

### Version consistency

- Manuscript Markdown/HTML/PDF: 3.7.
- Worked Markdown/HTML/PDF: 1.6.
- Plain companion Markdown/HTML/PDF: 1.6.
- VERIFY: 2.9.
- ENVIRONMENT: 1.6.
- Source README: 1.4.
- RELEASE_NOTES: 1.5.

Headline versions and file counts otherwise agree across README, release notes, trail and preview. The exceptions are the four stale VERIFY 2.8 references, the stale effort phrase, and the current/historical decline ambiguity.

## 4. Final verdicts

- **Manuscript v3.7:** **NOT CLEARED**. VERIFY and effort metadata are stale; its PDF’s Liberation notice is incomplete.
- **ZeeBeam, by Example v1.6:** **NOT CLEARED**. VERIFY is stale and its bold monospace font is omitted from the notice.
- **Plain-language companion v1.6:** **CLEARED AS A DOCUMENT**, but cannot be distributed inside this release until the release-wide licence defects are fixed.
- **Release tree:** **DO NOT PUSH.** It may not be pushed merely after the principal makes his open decisions. Repair the publication isolation, Zenodo metadata, licence notices and trail/version defects, rebuild both ledgers and frozen Git values, then rerun closing verification.

No file was modified.

— BOSUN ⚓
