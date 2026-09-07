Hoy.

**Verdict: DO NOT PUSH.** Identifier redaction and cryptographic closure now pass. The release does not. The principal’s two decisions alone will not make this tree publishable.

The offline-kit URL/DOI and commit-contact address remain owner decisions, as requested. The kit must remain unpublished until its two licensing questions are settled.

## Disposition of the other 15 findings

| Prior item | Disposition | Evidence or remainder |
|---|---|---|
| 1, infrastructure disclosure | RESOLVED | Zero actual public addresses, provider instance IDs, operational run IDs, disclosed runner paths, crew name or internal call sign remain. |
| 3 and technical parts of 16, publication transaction | PARTIALLY | Tree, message-source hash, recursive `.git` check, parent check and identity check landed. Message TOCTOU, deterministic authentication, hook/config isolation and public-first creation remain defective. |
| 4, frozen source paths | RESOLVED | Default staging produces a layout where all 40 `path =` entries in 19 manifests resolve without modifying frozen bytes. The wrapper’s self-check and documentation have new defects below. |
| 5, coupling stride | RESOLVED | [paper/zeebeam.md:576](../paper/zeebeam.md?plain=1#L576) now says “strides 1, 2, 2, 2 and 2”. |
| 6, Zcash/receipt wording | PARTIALLY | README, CFF, Zenodo, dummies and figure are repaired. The manuscript still says the whole decrypted memo equals SHA-256; the worked companion still asserts chronology as fact. |
| 7, versions/counts/dependencies | PARTIALLY | ENVIRONMENT’s `sha2` dependency is fixed. Manuscript, worked companion and preview still print VERIFY 2.7; worked still prints 3,064 rather than 3,065. |
| 8, witness/threshold/verifier taxonomy | PARTIALLY | Witness and verifier taxonomy are fixed. Worked Proposition 2 still lacks a threshold fixed before application. |
| 9, trail redaction | PARTIALLY | Sensitive tokens are gone. Several redactions changed the logical substance into tautologies or false counts. |
| 10, third-party notices | PARTIALLY | The 166 compiled crates and DejaVu are acknowledged. Mandatory licence texts/notices, PDF fonts and Rust runtime components remain incomplete. |
| 11, licence and Zenodo | PARTIALLY | Classroom, derived-work and patent language is repaired. Zenodo still asserts deposit-wide `other-nc`; incorporated-code/font coverage remains incomplete. |
| 12, audit claims | PARTIALLY | Release status and “adjudicated” language improved. The paper still says “this version applies both”, and the pass taxonomy is inconsistent. |
| 13, preregistration wording | RESOLVED | Current manuscript uses “predeclared”; historical uses in the trail are legitimate evidence. |
| 14, scope overbreadth | RESOLVED | System/recording scope, statement sizes and verifier dependencies are now exact. |
| 15, figure source/mainnet | RESOLVED | The source and out-of-band mainnet label are fixed. The new stale “Eleven referee passes” footer is separate. |
| 17, naming residue | PARTIALLY | Paper and crew-name residue are fixed. `Truth Beam` remains in LICENSE, so that subpart is NOT ADDRESSED. If intentionally reserved, it should have been explicitly DECLINED with a rationale. No other repair warrants a decline. |

## Remaining defects

1. **BLOCKING, publication authentication and public-first failure**, [007_zeebeam_repo.sh:16](<extras>/007_zeebeam_repo.sh:16).

   Current:

   > `GIT_CONFIG_GLOBAL=/dev/null`  
   > `gh repo create ... --public`  
   > `git push origin "$COMMIT:refs/heads/main"`

   This installation uses HTTPS, while the GitHub credential helper exists only in the suppressed global Git configuration. `gh` can create the public repository and the subsequent Git push can fail or prompt, leaving a public empty repository.

   Replace `--public` with `--private`, export:

   ```bash
   export GH_HOST=github.com GH_PROMPT_DISABLED=1 GIT_TERMINAL_PROMPT=0
   ```

   Replace the push with:

   ```bash
   $GIT -c credential.https://github.com.helper= \
     -c 'credential.https://github.com.helper=!/usr/bin/gh auth git-credential' \
     push --no-verify https://github.com/poliebotics/zeebeam.git \
     "$COMMIT:refs/heads/main"

   remote_commit=$(gh api repos/poliebotics/zeebeam/git/ref/heads/main --jq .object.sha)
   if [ "$remote_commit" != "$COMMIT" ]; then
     echo "REFUSED: remote main $remote_commit != $COMMIT" >&2
     exit 1
   fi
   gh api --method PATCH repos/poliebotics/zeebeam -F private=false >/dev/null
   ```

2. **BLOCKING, commit-message TOCTOU**, [007_zeebeam_repo.sh:53](<extras>/007_zeebeam_repo.sh:53).

   Current:

   > `frozen_msg_sha=$(... "$MSG" ...)`  
   > `if [ "$committed_msg_sha" != "$frozen_msg_sha" ]`

   Another process under the owner UID can change the temporary file after its initial hash. Both the commit and later reread can accept the changed value.

   Replace lines 53–55 exactly with:

   ```bash
   committed_msg_sha=$($GIT cat-file commit "$COMMIT" |
     sed '1,/^$/d' | sha256sum | cut -d' ' -f1)
   if [ "$committed_msg_sha" != "$EXPECTED_MSG" ]; then
     echo "REFUSED: committed message hash $committed_msg_sha != frozen $EXPECTED_MSG" >&2
     exit 1
   fi
   ```

3. **MUST-FIX, incomplete Git configuration, hook and repository isolation**, [007_zeebeam_repo.sh:27](<extras>/007_zeebeam_repo.sh:27).

   Preview claim:

   > “inherited Git configuration, hooks and templates disabled”

   `GIT_CONFIG_COUNT`, `GIT_CONFIG_PARAMETERS` and repository-routing variables remain inherited. The final bare `git push` consults `pre-push`. The `.git` check passes in stable execution but has a check/create race.

   Add:

   ```bash
   unset GIT_CONFIG_COUNT GIT_CONFIG_PARAMETERS GIT_DIR GIT_WORK_TREE \
     GIT_COMMON_DIR GIT_INDEX_FILE GIT_OBJECT_DIRECTORY \
     GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_SHALLOW_FILE GIT_NAMESPACE \
     GIT_REPLACE_REF_BASE
   export GIT_ATTR_NOSYSTEM=1
   ```

   Add these options to `GIT`:

   ```text
   -c core.attributesFile=/dev/null -c core.excludesFile=/dev/null
   ```

   Replace initialization with:

   ```bash
   if ! mkdir -m 700 -- .git; then
     echo "REFUSED: .git appeared after verification" >&2
     exit 1
   fi
   $GIT init -q --object-format=sha1 -b main
   ```

   Replace `rev-list --count HEAD` with `rev-list --count "$COMMIT"` and use `$GIT push --no-verify`.

4. **MUST-FIX, manuscript equates the memo with the digest**, [paper/zeebeam.md:32](../paper/zeebeam.md?plain=1#L32).

   Current:

   > “carries an encrypted memo which, decrypted in circuit under the operator's viewing key, equals SHA-256 of a receipt”

   Replace with:

   > carries an encrypted memo whose `binding=` value, parsed after in-circuit decryption under the operator's viewing key, equals SHA-256 of a receipt that opens a chameleon commitment over a chain-log prefix containing the row.

   Regenerate the manuscript HTML and PDF.

5. **MUST-FIX, worked chronology and unfixed P2 threshold**, [worked companion:298](../companions/zeebeam_worked_examples.md?plain=1#L298).

   Current:

   > “the published header hash is that of block 3456294, mined on 22 August 2026 minutes after the rows it covers.”

   Replace with:

   > The published txid is `8d1672…f206`. According to project records, the supplied header is Zcash block 3456294 and its transaction was mined on 22 August 2026, minutes after the rows named by the supplied prefix; neither the proof nor the receipt establishes that chronology. The verifier must confirm, out of band, that the header is canonical Zcash mainnet block 3456294 and that the transaction is in it (Section 10).

   Current P2:

   > “a high coupling score means the recorded illumination was this pattern”

   Replace with:

   > for a threshold τ fixed before applying the corollary, P2 (`q_t ≥ τ` implies, on the relevant acquisition distribution, that the recorded illumination was this pattern rather than another pattern or none)

   Regenerate worked HTML and PDF.

6. **MUST-FIX, stale versions and ledger count.**

   Exact replacements:

   - [paper:1025](../paper/zeebeam.md?plain=1#L1025): `` `VERIFY.md` (v2.7) `` → `` `VERIFY.md` (v2.8) ``.
   - [worked:431](../companions/zeebeam_worked_examples.md?plain=1#L431): `VERIFY.md v2.7` → `VERIFY.md v2.8`.
   - [worked:463](../companions/zeebeam_worked_examples.md?plain=1#L463): `3,064, 38, 7 and 57` → `3,065, 38, 7 and 57`.
   - [worked:504](../companions/zeebeam_worked_examples.md?plain=1#L504) and [preview:60](<extras>/007_zeebeam_repo.preview.md:60): `VERIFY 2.7` → `VERIFY 2.8`.

7. **BLOCKING, incomplete third-party distribution notices**, [THIRD_PARTY_NOTICES.md:16](../THIRD_PARTY_NOTICES.md?plain=1#L16).

   Current:

   > “texts in `licenses/Apache-2.0.txt` and `licenses/MIT.txt`”

   The inventory itself records mandatory BSD-2-Clause (`arrayref`), BSD-3-Clause (`subtle`) and Unicode-3.0 (`unicode-ident`) terms. PDF inspection finds DejaVu subsets in all three PDFs and Liberation Serif in the manuscript PDF. The ELFs also contain Rust `core`/`alloc` runtime code absent from the 166-Cargo-crate inventory.

   Replace the quoted phrase with:

   > applicable licence texts and required copyright/NOTICE material in `licenses/`, including Apache-2.0, MIT, BSD-2-Clause, BSD-3-Clause and Unicode-3.0

   Add the exact upstream BSD-2, BSD-3 and Unicode licence/notices; add the Liberation Fonts OFL-1.1 text and mapping; add a mechanically sourced Rust runtime/component notice. Expand the font row to name all three PDFs and Liberation Serif. Also replace:

   - `final_relation/zeebeam_guest_final.elf` with `bundle/proofs_20260902/final_relation/zeebeam_guest_final.elf`.
   - `final_relation/chain/zeebeam_chain_guest.elf` with `bundle/proofs_20260902/final_relation/chain/zeebeam_chain_guest.elf`.

8. **MUST-FIX, false deposit-wide licence**, .zenodo.json:10 (`.zenodo.json`, line 10).

   Current:

   > “the deposit-wide licence field can name only one licence and names the custom one.”

   Replace with:

   > Licence details for each component are in LICENSE and THIRD_PARTY_NOTICES.md; no deposit-wide single-licence value is asserted.

   Delete .zenodo.json:25 (`.zenodo.json`, line 25):

   ```json
   "license": "other-nc",
   ```

9. **MUST-FIX, trail redaction changed evidentiary substance**, [referee trail README:17](../referee_trail/README.md?plain=1#L17).

   Current claim:

   > “Every report is reproduced with its substance unchanged.”

   Required exact repairs include:

   - [round12:337](../referee_trail/FINDINGS_round12_verification.md?plain=1#L337): replace the first `` `BOSUN` `` with `` `[internal assistant call sign redacted]` ``; retain the second quote as the nonsensical result being criticised.
   - [release audit:242](../referee_trail/FINDINGS_release_ultra.md?plain=1#L242): replace the evidentiary `BOSUN` in the sign-off with `[internal assistant call sign redacted]`; retain `BOSUN` as the proposed replacement.
   - [round12:205](../referee_trail/FINDINGS_round12_verification.md?plain=1#L205): replace the row with `| /tmp/ | 2 / 1 | fleet bootstrap 2 |`; represent the removed exact runner path separately as `[runner path redacted]`.
   - In the role-replacement findings, use `[principal call name redacted]`, `[principal crew name redacted]` and `[internal assistant call sign redacted]` for evidence; retain `the principal`, `operator` and `BOSUN` only as proposed replacements.

10. **MUST-FIX, rebuild README and wrapper overclaim their coverage**, [source README:10](../bundle/proofs_20260902/source/README.md?plain=1#L10).

   Current:

   > “holds every source file the final guest ELF … was built from”

   Replace with:

   > This directory holds every frozen first-party, path-dependency and patched source file, plus the embedded blobs, shipped for the final guest ELF; registry/Git dependencies and Rust/toolchain sources are not in this public tree.

   The wrapper’s default geometry passes, but its checker scans only 13 of 19 manifests and 37 of 40 paths. Replace [stage_for_rebuild.sh:31](../bundle/proofs_20260902/source/stage_for_rebuild.sh#L31) with:

   ```bash
   done < <(find "$ROOT/BOSUN/zeebeam-science/rust" "$ROOT/BOSUN/scratch" "$ROOT/vendor" \
     -name Cargo.toml -not -path '*/target/*')
   ```

   The advertised `ROOT=/other/root` mode cannot satisfy six absolute `jubjub` paths. Delete that usage line and replace line 12 with:

   ```bash
   readonly ROOT="[frozen-layout root]"
   ```

   Replace “An air-gapped rebuild from this bundle alone has not been rehearsed” with:

   > An air-gapped rebuild from this bundle alone is not possible because transitive registry/Git sources and host toolchains are absent; the separate kit supplies crate vendors but lacks a complete host toolchain, and an end-to-end air-gapped rebuild remains unrehearsed.

11. **SHOULD-FIX, pass-count and audit-status inconsistency.**

   - [paper:477](../paper/zeebeam.md?plain=1#L477): delete `; this version applies both`.
   - Replace [RELEASE_NOTES.md:12](../RELEASE_NOTES.md?plain=1#L12) through the first sentence with:

     > ZeeBeam repository release 1.0.0, assembled on 4 and 5 September 2026 after eleven referee passes on the manuscript, audits of the companions and release tree, and a twelfth verification pass on the applied text, all by a second model, with every finding adjudicated.

   - [figures/zeebeam.py:144](../figures/zeebeam.py#L144): `Eleven referee passes` → `Twelve referee passes`; regenerate SVG and PNG.

12. **SHOULD-FIX, false preview/name/environment claims.**

   - [preview:51](<extras>/007_zeebeam_repo.preview.md:51) names an inventory absent from this audit packet. Replace with:

     > The complete file list with digests is the root `SHA256SUMS`; the live transmission item also carries a byte copy named `007_zeebeam_repo.files.txt`.

   - [LICENSE:54](../LICENSE#L54): delete `Truth Beam, `. If retention is intentional, record it as DECLINED with the reason.
   - After that deletion, replace preview’s stale-name claim with:

     > Legacy `Truth Beam` grep: one frozen-source occurrence remains by necessity; three trail occurrences quote the finding.

   - [ENVIRONMENT.md:32](../bundle/proofs_20260902/ENVIRONMENT.md?plain=1#L32): replace “instance ids in `all_rows/logs/fleet_instances.json`” with “per-instance records in `all_rows/logs/fleet_instances.json`, with provider identifiers removed before publication”.

13. **NIT, misleading singular title.**

   In [VERIFY.md:8](../bundle/proofs_20260902/VERIFY.md?plain=1#L8) and [ENVIRONMENT.md:8](../bundle/proofs_20260902/ENVIRONMENT.md?plain=1#L8), replace `one-proof bundle` with `proof bundle`. The bundle contains 262 proofs.

## Identifier sweep

Excluding the trail placeholders and quotations specified by the request:

| Class | Actual occurrences |
|---|---:|
| Public IPv4 address | 0 |
| Provider instance identifier | 0 |
| Operational runner/run identifier | 0 |
| Former exact runner path | 0 |
| Principal’s crew name | 0 |
| Principal’s personal call name | 0 |
| Internal assistant call sign | 0 |

Three PDFs each contain one `152.0.0.0` metadata string: `paper/zeebeam.pdf`, `companions/zeebeam_worked_examples.pdf`, and `companions/zeebeam_for_dummies.pdf`. These are Chromium version strings, not IPv4 addresses.

Two `/tmp/go.tgz` strings remain in `all_rows/logs/scripts/box_bootstrap_20260902.sh`; they are generic bootstrap scratch paths, not the removed runner path.

Scientific evidence labels remain intentionally and are not operational run identifiers:

- `ZEEBEAM_MAINNET_BLOCKING_TRAINING_300S_20260822_001`: 549 occurrences in 547 files outside the trail.
- `live_300s_training_001`: 29 occurrences in 17 files.
- `campaign_recording_001`: 3 occurrences in 3 files.

## Recomputed counts and digests

| Item | Result |
|---|---|
| Public tree | 3,103 regular files; 17,953,820 bytes |
| Root ledger | 3,102 unique entries; exact non-self coverage; all pass |
| Root-ledger SHA-256 | `9d3090c33329a33a4b046ed3dc36a62e34a29e827fa162f45bede2bf0085415e` |
| Bundle | 3,066 files; 14,808,299 bytes |
| Bundle ledger | 3,065 unique entries; exact non-self coverage; all pass |
| Bundle-ledger SHA-256 | `c30a51d825f53a59786b3bcb649b2b64317a83d68c9d94a958f7a554d194a49c` |
| SOURCE_TREE | 38/38 pass; digest `e45be3c065f31b720418ba78504616dbabcd35275c8f7afdbfd6a94e634437a2` |
| DEPENDENCY | 57/57 pass; digest `e8a586248d520f23f5dffcc84f58ebe557fd5a160bbf004468af32754abe6bc0` |
| CHAIN_SOURCE | 7/7 pass; digest `c35d3230905f006aa529e3561bf3ea70f98e81cff307d443e1b113e9df7bf2ac` |
| Git SHA-1 tree, object serialization | `c39670a5134b5e171d386919416c0009adad3a7c` |
| Git modes | 3,090 × `100644`; 13 × `100755`; 0 symlinks |
| Commit-body SHA-256 | `49262f71da6d793c72db0297d3393ba3b43d84c9f57da4590f9cddf360042aef` |
| Commit-body size | 1,299 bytes |
| Proof inventory | 262 raw proofs, 262 framed proofs, 262 public values, 262 manifests |
| Trail | 14 reports plus README, 15 files |

All frozen ledger, count, message and tree values in the script match. Its external `TREE_DIR` is byte-and-mode identical to the supplied `public/`.

The three source lists do **not** literally cover every file under `source/`: collectively they cover 102 of 110. The eight outside their historical component scopes are both blobs, both build drivers, both row-tree READMEs, `source/README.md`, and `stage_for_rebuild.sh`. All eight are covered by the bundle and root ledgers.

## Publication-control result

| Control | Result |
|---|---|
| Recursive `.git` rejection | PASS for a stable tree; check/create race remains |
| Frozen root ledger/count/tree | PASS |
| Message freeze | FAIL |
| Hook/config/attribute isolation | FAIL |
| Parentless assertion | PASS in stable execution; unnecessary `HEAD` race remains |
| Raw author and committer identity | PASS |
| Exact commit-OID push form | PASS |
| Deterministic authentication | FAIL |
| No public partial state | FAIL |
| Preview accurately describes script | FAIL |

## One-line verdicts

- **Manuscript v3.6:** NOT CLEARED, false memo equality, stale VERIFY version, audit residue and incomplete PDF-font notices remain.
- **ZeeBeam, by Example v1.5:** NOT CLEARED, chronology, P2 threshold, VERIFY version, ledger count and PDF notices remain wrong.
- **Plain-language companion v1.5:** Substantive text CLEARED; its PDF is not distribution-clean until font notices are complete.
- **Release tree as staged:** DO NOT PUSH; it may not be pushed merely after the principal makes his two decisions. Repair the blocking and must-fix findings, regenerate derivatives, rebuild both ledgers and the frozen Git values, then rerun this closing pass.

No file was modified.

— BOSUN ⚓
