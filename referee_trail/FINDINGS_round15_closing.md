Hoy.

**Verdict: DO NOT PUSH.** Nine of fourteen items are resolved; five remain partial. The ledgers accurately seal the tree, but the tree still contains four publication blockers: incomplete script isolation, the wrong Rust-library notice, false/missing `compiler-builtins` terms, and a missing `blake2b_simd` copyright notice.

## 1. Disposition of the fourteen items

| # | Item | Disposition | Evidence or remainder |
|---:|---|---|---|
| 1 | Git askpass | **RESOLVED** | [Script line 21](<extras>/009_zeebeam_repo.sh:21) sets both askpass programs to `/bin/false`; line 35 disables credential interactivity. |
| 2 | Inherited Git environment | **PARTIALLY** | The named variables were cleared, but [line 19](<extras>/009_zeebeam_repo.sh:19) still omits dates, SSL client-key settings, grafts, protocol controls and trace variables. Pager variables also remain. |
| 3 | Final published-state assertion | **RESOLVED** | Lines 71–72 assert URL, visibility and default branch against `EXPECTED_STATE`. A newly identified branch-tip/read-back weakness remains below. |
| 4 | Zenodo metadata | **RESOLVED** | `.zenodo.json` is absent. [Release notes line 71](../RELEASE_NOTES.md?plain=1#L71) correctly requires a manual mixed-licence deposit. |
| 5 | Liberation notice | **RESOLVED** | [OFL lines 3–5](../licenses/OFL-1.1-Liberation.txt#L3) contain both exact copyright notices; the unrelated Debian stanza is gone. |
| 6 | Rust notice | **PARTIALLY** | Truncation is gone, but Rust notice line 3 (`licenses/Rust-COPYRIGHT-library.txt`, line 3) substitutes Rust 1.98 material for binaries built with Rust 1.94.0-dev. |
| 7 | MIT notices | **PARTIALLY** | Twenty sections exist; nineteen reproduce an upstream copyright. `blake2b_simd` still contains generic terms without its repository copyright. |
| 8 | Rust components and expressions | **PARTIALLY** | The component names are now correct and `nm` confirms them. The `compiler-builtins` expression is false. My preceding report prescribed `OR`; that prescription was wrong. Upstream says **AND**. |
| 9 | Embedded-font inventory | **RESOLVED** | [Notices lines 21–22](../THIRD_PARTY_NOTICES.md?plain=1#L21) exactly match `pdffonts`. |
| 10 | VERIFY version | **RESOLVED** | Manuscript and worked companion now cite VERIFY 2.9; Markdown, HTML and PDF agree. |
| 11 | Round-12 evidence | **RESOLVED** | Required placeholders, historical counts and the criticised quotation are restored. |
| 12 | Trail snapshot/count | **PARTIALLY** | The preceding-tree convention is now accurate, but [trail README line 31](../referee_trail/README.md?plain=1#L31) says four linked reports; round 14 makes five. |
| 13 | Review effort | **RESOLVED** | Manuscript and trail now say high for rounds 1–10 and ultra thereafter. |
| 14 | Reversed/current declines | **RESOLVED** | [Release notes lines 67–70](../RELEASE_NOTES.md?plain=1#L67) distinguish the reversed stride decision from the two current declines. |

Both current declines are sound:

- `Truth Beam` occurs in [LICENSE lines 54–55](../LICENSE#L54) only as part of a no-endorsement clause.

- Reproducing complete font files is unnecessary. All PDF fonts are embedded subsets and the applicable texts/notices ship. The OFL expressly permits full or subset embedding without requiring the complete font package. [Official OFL FAQ §1.12](https://openfontlicense.org/ofl-faq/)

## 2. Remaining defects

1. **BLOCKING: environment and prompt isolation remains false.**

   [Script lines 12–13](<extras>/009_zeebeam_repo.sh:12):

   > `every inherited Git environment variable is cleared; no terminal, GUI or gh prompt can open.`

   `GIT_AUTHOR_DATE`, `GIT_COMMITTER_DATE`, `GIT_SSL_KEY`, `GIT_GRAFT_FILE`, `GIT_ALLOW_PROTOCOL` and `GIT_TRACE*` remain effective. An encrypted inherited client key creates a password-prompt path; trace settings can disclose credentials. `GH_FORCE_TTY` plus `GH_PAGER` can start an arbitrary pager after publication. These variables are documented by the [GitHub CLI environment reference](https://cli.github.com/manual/gh_help_environment).

   Replace script lines 19–23 exactly with:

   ```bash
   while IFS= read -r git_var; do unset "$git_var"; done < <(compgen -A variable GIT_)
   unset GH_FORCE_TTY
   export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_SYSTEM=/dev/null GIT_CONFIG_NOSYSTEM=1 GIT_ATTR_NOSYSTEM=1 GIT_TEMPLATE_DIR=""
   export GIT_TERMINAL_PROMPT=0 GIT_ASKPASS=/bin/false SSH_ASKPASS=/bin/false GH_PROMPT_DISABLED=1 GH_HOST=github.com
   export GIT_PAGER=/bin/cat GH_PAGER=/bin/cat PAGER=/bin/cat
   export GIT_EDITOR=/bin/false GIT_SEQUENCE_EDITOR=/bin/false GH_EDITOR=/bin/false VISUAL=/bin/false EDITOR=/bin/false GH_BROWSER=/bin/false BROWSER=/bin/false
   export GIT_AUTHOR_NAME="BOSUN" GIT_AUTHOR_EMAIL="bosun@cittadel.poliephleet.ie"
   export GIT_COMMITTER_NAME="BOSUN" GIT_COMMITTER_EMAIL="bosun@cittadel.poliephleet.ie"
   ```

2. **BLOCKING: the Rust-library notice is from the wrong release.**

   Current lines 3–4 (`licenses/Rust-COPYRIGHT-library.txt`, line 3):

   > `complete library-only COPYRIGHT notice of a rustup stable toolchain (rustc 1.98.0...)`

   Every ELF’s `.comment` says `rustc version 1.94.0-dev`. The 1.98 notice names dependency versions absent from the actual 1.94 sysroot and omits the exact 1.94 provenance.

   The source anchor is now identifiable: Succinct release `succinct-1.94.0-64bit`, commit `c7149403db5f6f72f410d6dffcee90378235f23b`. The local release asset is 384,963,362 bytes with SHA-256 `12c94435d41bfe4e20131bbcce40b35abd32270ad792befc653af4e3fabc192f`, matching the recorded release asset. [Official Succinct release](https://github.com/succinctlabs/rust/releases/tag/succinct-1.94.0-64bit)

   Exact replacement: replace the current file with the complete, unmodified `COPYRIGHT-library.html` generated from that commit, and change the notice row to:

   > `the complete library-only COPYRIGHT notice generated from succinctlabs/rust release succinct-1.94.0-64bit at commit c7149403db5f6f72f410d6dffcee90378235f23b`

   Rust itself identifies `COPYRIGHT-library.html` as the release-specific standard-library notice. [Rust COPYRIGHT](https://github.com/rust-lang/rust/blob/main/COPYRIGHT)

3. **BLOCKING: `compiler-builtins` is stated under the wrong expression and its complete licence is absent.**

   [THIRD_PARTY_NOTICES line 17](../THIRD_PARTY_NOTICES.md?plain=1#L17):

   > `compiler-builtins: MIT OR Apache-2.0 WITH LLVM-exception`

   Replace with:

   > `compiler-builtins: MIT AND Apache-2.0 WITH LLVM-exception`

   Add `licenses/compiler-builtins-LICENSE.txt` as the byte-exact file from [`library/compiler-builtins/LICENSE.txt` at the exact Succinct commit](https://github.com/succinctlabs/rust/blob/c7149403db5f6f72f410d6dffcee90378235f23b/library/compiler-builtins/LICENSE.txt), including the LLVM-derived notice. Append to the notice row:

   > `; its complete licence and LLVM-derived notices are in licenses/compiler-builtins-LICENSE.txt`

   `nm -C` finds `compiler_builtins::mem::memmove` in all three ELFs. Generic MIT and Apache texts are not a substitute for the component’s combined expression, LLVM exception and notices.

4. **BLOCKING: the `blake2b_simd` MIT copyright is still missing.**

   [MIT line 87](../licenses/MIT.txt#L87):

   > `authors per its manifest and repository`

   No author or copyright follows. The crate’s recorded source commit is `47714dc3c424e82213d7f2eb4a833e3843017be5`; its repository-root licence contains `Copyright (c) 2018 Jack O'Connor`.

   Replace lines 87–100 with this heading followed by the byte-exact upstream file:

   ```text
   === blake2b_simd-1.0.5 (repository-root LICENSE at source commit 47714dc3c424e82213d7f2eb4a833e3843017be5) ===
   ```

   Exact source: [blake2_simd LICENSE at the crate’s recorded commit](https://raw.githubusercontent.com/oconnor663/blake2_simd/47714dc3c424e82213d7f2eb4a833e3843017be5/LICENSE).

5. **MUST-FIX: 166 locked packages are falsely described as 166 target-linked crates.**

   [THIRD_PARTY_NOTICES line 16](../THIRD_PARTY_NOTICES.md?plain=1#L16):

   > `the 166 crates locked by the guest workspaces ... compiled into the ELFs`

   The inventory count and expressions are correct, but it includes host procedural macros such as `serde_derive` and `tracing-attributes`; their machine code is not linked into the RISC-V ELFs.

   Replace the phrase with:

   > `the 166 third-party packages in the union of the shipped guest-workspace Cargo.lock files, listed below; target-runtime crate code is incorporated into the ELFs, while build dependencies and procedural macros execute during compilation and are not target-linked`

   Replace the heading at line 28 with:

   > `## Third-party packages locked by the guest workspaces (166)`

   In [MIT line 3](../licenses/MIT.txt#L3), replace:

   > `compiled into the guest ELFs`

   with:

   > `in the shipped guest-workspace lockfile union`

6. **SHOULD-FIX: final read-back omits the published commit and failure diagnostics are incomplete.**

   [Preview lines 41–43](<extras>/009_zeebeam_repo.preview.md:41) mention only a lost PATCH response. The branch may move between script lines 68 and 71, and failure of the final `gh repo view` exits without warning after the repository may already be public.

   Replace script lines 70–74 with:

   ```bash
   if ! gh api --method PATCH "repos/$REPO" -F private=false >/dev/null; then
     echo "REFUSED: visibility result unknown; the repository may already be public; inspect it remotely before any retry" >&2
     exit 1
   fi
   if ! published_state=$(gh api graphql \
     -f owner="${REPO%%/*}" -f name="${REPO#*/}" \
     -f query='query($owner:String!,$name:String!){repository(owner:$owner,name:$name){url visibility defaultBranchRef{name target{oid}}}}' \
     --jq '.data.repository | .url+" "+.visibility+" branch="+.defaultBranchRef.name+" commit="+.defaultBranchRef.target.oid'); then
     echo "REFUSED: published-state read-back failed; the repository may already be public; inspect it remotely before any retry" >&2
     exit 1
   fi
   expected_published="$EXPECTED_STATE commit=$COMMIT"
   if [ "$published_state" != "$expected_published" ]; then
     echo "REFUSED: unexpected published state: $published_state (inspect the repository remotely before any retry)" >&2
     exit 1
   fi
   echo "$published_state"
   echo "published commit $COMMIT"
   ```

   Replace preview lines 41–43 with:

   > `Any failure before the visibility request leaves the repository private (empty or partial). Once the visibility request is sent, a failed PATCH response or failed final read-back may leave the complete verified repository public; inspect it remotely before any retry. External ref mutation cannot be made atomic with visibility; exclusive administration remains mandatory.`

   Under genuine exclusive administration there is no path to a **public partial tree**. At or after PATCH, the complete verified commit may be public while the script reports failure.

7. **LOW: stale trail link count.**

   Quote:

   > `Links from the four reports that examined this tree...`

   Exact replacement:

   > `Links from the five reports that examined this tree are relative to the tree root (\`../\`).`

8. **LOW: stale preview size.**

   [Preview line 46](<extras>/009_zeebeam_repo.preview.md:46):

   > `The tree (24 MB, 3,109 files including SHA256SUMS)`

   Logical regular-file size is 19,501,267 bytes; allocated `du -sh` size is 25M. Exact replacement:

   > `## The tree (19,501,267 bytes, 3,109 files including SHA256SUMS)`

## 3. Recomputed counts, digests and identifiers

| Item | Recomputed result |
|---|---|
| Public tree | 3,109 regular files; 19,501,267 bytes; 107 directories; no symlinks, special nodes or `.git` |
| Root ledger | 3,108 unique entries; exact non-self coverage; all pass |
| Root-ledger SHA-256 | `5dd3fe4874031950876483c064f4485d3bc083f4384ae4bc403c38ac51ab535c` |
| Bundle | 3,066 files; 14,809,148 bytes |
| Bundle ledger | 3,065 entries; all pass |
| Bundle-ledger SHA-256 | `fad30aa0cf0339a500f2d450bd718c9856e7df0534ab4f7aff8b9382e661c03e` |
| SOURCE_TREE | 38/38 pass; `e45be3c065f31b720418ba78504616dbabcd35275c8f7afdbfd6a94e634437a2` |
| DEPENDENCY | 57/57 pass; `e8a586248d520f23f5dffcc84f58ebe557fd5a160bbf004468af32754abe6bc0` |
| CHAIN_SOURCE | 7/7 pass; `c35d3230905f006aa529e3561bf3ea70f98e81cff307d443e1b113e9df7bf2ac` |
| Git tree | `ba962c1168cc92094fe1f20c0ee07e9e19498227` |
| Git modes | 3,096 × `100644`; 13 × `100755` |
| Frozen message | 1,298 bytes; `c30fb485dff647439db1ca1209cea755a2120b7fa9412f226eab5adb50b8f407` |
| Proof inventory | 262 raw proofs; 262 framed proofs; 262 public-values files; 262 manifests |
| Trail | 16 reports plus README |
| Licences, current | 8 files; 31,699 lines; 1,516,325 bytes |
| Fonts | 14 embedded, subsetted, Unicode-mapped PDF fonts |
| Crate table | 166 rows; all name/version/expression/repository fields match the guest-vendor evidence |

The root ledger digest, file/entry counts, Git tree and message digest exactly match the script’s frozen values. The live target tree and live transmit script/body/preview are byte-identical to the audited copies; the live `.files.txt` equals root `SHA256SUMS`. `bash -n` passes. `shellcheck` was unavailable.

The three source lists jointly cover 102 of 110 source files. The remaining eight are deliberately outside their historical scopes and remain covered by the bundle and root ledgers.

### Identifier sweep

Excluding trail placeholders and quotations:

| Category | Result |
|---|---:|
| Actual public IPv4 addresses | 0 |
| Chromium metadata string `152.0.0.0` | 3 PDF occurrences, not addresses |
| Provider instance identifiers | 0 |
| Operational UUIDs | 0 |
| Former runner path | 0 |
| Retired principal crew/call tokens | 0 |
| Internal assistant call sign | 0 |
| Scientific session identifier | 549 occurrences / 547 files |
| `live_300s_training_001` | 29 / 17 |
| `campaign_recording_001` | 3 / 3 |
| `/tmp/go.tgz` | 2 / 1 |

### Binary and version consistency

All three ELFs are static, unstripped RISC-V executables. `readelf` reports Rust 1.94.0-dev in each. `nm -C` finds `core`, `alloc`, `std`, `panic_abort::zkvm::zkvm_set_abort_message` and `compiler_builtins::mem::memmove` in every ELF.

`pdffonts` reports:

- Manuscript: DejaVu Serif regular/bold/italic, DejaVu Sans Mono, Liberation Serif.

- Worked companion: DejaVu Serif regular/bold/italic, DejaVu Sans Mono and Mono Bold.

- Plain companion: DejaVu Serif regular/bold/italic, DejaVu Sans Mono.

All active versions agree: manuscript 3.8; both companions 1.7; VERIFY 2.9; ENVIRONMENT 1.6; source README 1.4; RELEASE_NOTES 1.6; THIRD_PARTY_NOTICES 1.3; LICENSE 1.2. Counts agree across README, release notes and preview except for the two exactness defects above.

Any repair changes the root ledger and Git tree. Regenerate every affected count, both ledgers and the script’s frozen digest/tree values before another closing pass.

## 4. Final verdicts

- **Manuscript v3.8:** **CLEARED AS A MANUSCRIPT.**

- **ZeeBeam, by Example v1.7:** **CLEARED AS A COMPANION.**

- **Plain-language companion v1.7:** **CLEARED AS A COMPANION.**

- **Release tree:** **DO NOT PUSH.** It may **not** be pushed merely after the principal makes the decisions reserved to him; repair the four blockers, correct the remaining release claims, regenerate the ledgers and frozen values, and rerun closure.

No file was modified.

— BOSUN ⚓