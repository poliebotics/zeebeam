Hoy.

**Verdict: DO NOT PUSH.** Seven prior items are resolved. The Rust notice remains blocking, and this tree has fresh version/count defects.

## 1. Disposition

| # | Prior severity | Item | Disposition | Evidence |
|---:|---|---|---|---|
| 1 | BLOCKING | Git environment and prompts | **RESOLVED** | [Script:22](<extras>/010_zeebeam_repo.sh:22) enumerates every inherited `GIT_*`; lines 23–27 disable TTY forcing, prompts, askpass, pagers, editors and browsers. |
| 2 | BLOCKING | Rust library notice | **PARTIALLY** | The shipped HTML has the claimed `af70…29f` digest and both supplied lockfiles are byte-identical. The equivalence proof omits generator inputs described below. |
| 3 | BLOCKING | `compiler-builtins` | **RESOLVED** | [Notice:17](../THIRD_PARTY_NOTICES.md?plain=1#L17) says `MIT AND Apache-2.0 WITH LLVM-exception`; the complete [licence](../licenses/compiler-builtins-LICENSE.txt#L1) ships. |
| 4 | BLOCKING | `blake2b_simd` copyright | **RESOLVED** | [MIT.txt:87](../licenses/MIT.txt#L87) identifies the source commit and line 91 contains `Copyright (c) 2018 Jack O'Connor`. |
| 5 | MUST-FIX | 166 packages described as target-linked | **RESOLVED** | [Notice:16](../THIRD_PARTY_NOTICES.md?plain=1#L16) now distinguishes target-runtime code from build dependencies and procedural macros; heading line 28 says locked packages. |
| 6 | SHOULD-FIX | Published-state read-back | **RESOLVED** | [Script:76](<extras>/010_zeebeam_repo.sh:76) warns after uncertain visibility; lines 80–90 GraphQL-check URL, `PUBLIC`, `main` and the pushed OID. |
| 7 | LOW | Trail link count | **RESOLVED** | [Trail README:32](../referee_trail/README.md?plain=1#L32) says six; exactly six reports contain tree-relative links. |
| 8 | LOW | Preview size | **RESOLVED** | [Preview:47](<extras>/010_zeebeam_repo.preview.md:47) correctly says 18,505,344 bytes and 3,111 files. |

Both earlier declines are upheld. `Truth Beam` appears in the no-endorsement clause. Shipping the complete font files is unnecessary because the distributed subsets/outlines and applicable notices are present.

## 2. Remaining defects

1. **BLOCKING: the Rust-library equivalence argument is incomplete.**

   [THIRD_PARTY_NOTICES.md:17](../THIRD_PARTY_NOTICES.md?plain=1#L17):

   > `that commit's library/Cargo.lock, COPYRIGHT and library/compiler-builtins/LICENSE.txt are byte-identical to those of the 1.94.0 release, so the notice describes the same library dependency set and the same Rust copyright terms`

   Rust’s generator also consumes the selected library manifests, resolved Cargo metadata and vendor notices, plus the `library` slice of `license-metadata.json`. Lockfile and root `COPYRIGHT` identity do not determine those inputs. [Rust COPYRIGHT](https://github.com/rust-lang/rust/blob/main/COPYRIGHT), [generator source](https://github.com/rust-lang/rust/blob/main/src/tools/generate-copyright/src/main.rs).

   Exact closure: run commit `c7149403…f23b`’s `x.py run generate-copyright` over that checkout and its exact dependency inputs, then ship the unmodified library output. If it reproduces the current bytes, replace the quoted conclusion with:

   > `The Succinct release archive ships no generated notice. Re-running that commit's generate-copyright tool over that commit and its exact vendored inputs produced licenses/Rust-COPYRIGHT-library.html byte-for-byte; sha256 af70aaabed1b73e872f14f9130db37e09f3f4d73a5f7c598b9173697a5d2729f.`

   If it differs, ship that generated output and its actual digest. A complete comparison of every generator input and output is an equivalent cure.

2. **MUST-FIX: current version/pass metadata was not advanced consistently.**

   | File | Exact quote | Exact replacement |
   |---|---|---|
   | [RELEASE_NOTES.md:13](../RELEASE_NOTES.md?plain=1#L13) | `and two closing passes` | `and three closing passes` |
   | [paper/zeebeam.md:4](../paper/zeebeam.md?plain=1#L4) | `status: manuscript-v3.8-second-closing-pass-applied` | `status: manuscript-v3.9-third-closing-pass-applied` |
   | [worked examples:4](../companions/zeebeam_worked_examples.md?plain=1#L4) | `status: companion-second-closing-pass-applied` | `status: companion-third-closing-pass-applied` |
   | [worked examples:6](../companions/zeebeam_worked_examples.md?plain=1#L6) | `ZeeBeam manuscript v3.8` | `ZeeBeam manuscript v3.9` |
   | [worked examples:13](../companions/zeebeam_worked_examples.md?plain=1#L13) | `*ZeeBeam* (v3.8)` | `*ZeeBeam* (v3.9)` |
   | [plain companion:4](../companions/zeebeam_for_dummies.md?plain=1#L4) | `status: companion-second-closing-pass-applied` | `status: companion-third-closing-pass-applied` |
   | [plain companion:6](../companions/zeebeam_for_dummies.md?plain=1#L6) | manuscript `v3.8`; worked examples `v1.7` | manuscript `v3.9`; worked examples `v1.8` |

   Rerender all affected HTML and PDF files. Preserve the historical v3.8/v1.7 log entries.

3. **MUST-FIX: the release figure states the wrong pass count.**

   [zeebeam.py:144](../figures/zeebeam.py#L144):

   > `Fourteen referee passes`

   Replace with:

   > `Fifteen referee passes`

   Regenerate `zeebeam.svg` and `zeebeam.png`; both presently carry the false footer.

4. **SHOULD-FIX: figure text visibly collides.**

   `OUTSIDE THE PROOF, by design` is overprinted by the first bullet. In [zeebeam.py:141](../figures/zeebeam.py#L141), replace:

   ```python
   "- what pose class \"letter_y\" means physically: no hash proves\n"
   "  a photon hit a wall",
   ```

   with:

   ```python
   "- what pose class \"letter_y\" means physically: no hash proves a photon hit a wall",
   ```

   Then regenerate and inspect both image formats.

5. **LOW: the current release log miscounts trail files.**

   [RELEASE_NOTES.md:82](../RELEASE_NOTES.md?plain=1#L82):

   > `counts (trail 17 files, licence texts 9).`

   Replace with:

   > `counts (trail 17 reports and a README, 18 files; licence texts 9).`

If this fourth report is added before publication, advance the live totals atomically to four closing passes, sixteen manuscript passes, eighteen reports plus README, and seven linked reports.

## 3. Recomputed counts and digests

| Check | Recomputed result |
|---|---|
| Public tree | 3,111 regular files; 18,505,344 logical bytes; 107 directories; no symlinks, special nodes or `.git` |
| Git modes | 3,098 × `100644`; 13 × `100755` |
| Root ledger | 3,110 unique entries; exact non-self coverage; all pass |
| Root-ledger SHA-256 | `fa0f716f6a8eee13af3a2d5d8af5be80ea2254084b56e8c716aecef3a58ff9cd` |
| Git tree | `a94a156a7b18089592d3a8be74afcb806b1147e9` |
| Commit message | 1,299 bytes; `b0ff44e1e00a433a95785b6cf7a7db19e39d755c91b2e1341eef3cc86237e719` |
| Bundle | 3,066 files; 14,809,148 bytes |
| Bundle ledger | 3,065/3,065 pass; `fad30aa0cf0339a500f2d450bd718c9856e7df0534ab4f7aff8b9382e661c03e` |
| `SOURCE_TREE` | 38/38; `e45be3c065f31b720418ba78504616dbabcd35275c8f7afdbfd6a94e634437a2` |
| `DEPENDENCY` | 57/57; `e8a586248d520f23f5dffcc84f58ebe557fd5a160bbf004468af32754abe6bc0` |
| `CHAIN_SOURCE` | 7/7; `c35d3230905f006aa529e3561bf3ea70f98e81cff307d443e1b113e9df7bf2ac` |
| Trail | 17 reports plus README; six reports with tree links |
| Proof inventory | 262 raw, 262 framed, 262 public-values files, 262 manifests |

All frozen script values match. The three source lists cover 102 of 110 source files; the eight intentionally out-of-scope files remain covered by the bundle and root ledgers.

## 4. Identifier, publication and licence checks

The identifier sweep, excluding trail placeholders and quotations, found zero public IPv4 addresses, provider IDs, operational UUIDs, former runner paths, retired principal tokens or internal assistant call signs. `152.0.0.0` occurs once in each PDF as Chromium metadata. Intentional scientific identifiers remain unchanged: 549/547 files, 29/17, 3/3 and `/tmp/go.tgz` 2/1.

The script has no remaining identified prompt path. Before visibility change, failures can leave a private empty or partial repository. Once PATCH is sent, a complete verified repository may be public despite a failed response or read-back. External ref mutation remains non-atomic; the preview correctly requires exclusive administration. Under that condition there is no public partial-tree path.

`pdffonts` reports 14 embedded, subsetted, Unicode-mapped fonts matching the DejaVu and Liberation notices. The 166-row crate table matches the lock union. All three static RISC-V ELFs report rustc 1.94.0-dev and contain `core`, `alloc`, `std`, `panic_abort` and `compiler_builtins::mem::memmove`.

## 5. Final verdicts

| Artifact | Verdict |
|---|---|
| Manuscript v3.9 | **SUBSTANCE CLEARED; staged MD/HTML/PDF not release-clean until status metadata agrees.** |
| *ZeeBeam, by Example* v1.8 | **SUBSTANCE CLEARED; staged triple not release-clean until active manuscript references agree.** |
| *ZeeBeam, without the maths* v1.8 | **SUBSTANCE CLEARED; staged triple not release-clean until both companion references agree.** |
| Release tree | **DO NOT PUSH. the principal’s open decisions alone are insufficient; close the Rust notice proof, repair the current metadata and figure, then regenerate the root ledger, Git tree and frozen publication values.** |

No file was modified.

— BOSUN ⚓