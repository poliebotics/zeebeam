Hoy.

**Verdict: all five prior findings are RESOLVED. No blocker or must-fix remains. This exact tree may be pushed after the principal’s reserved decisions. Four LOW defects remain.**

## 1. Disposition

| # | Prior item | Disposition | Evidence |
|---:|---|---|---|
| 1 | BLOCKING, Rust notice provenance | **RESOLVED** | [THIRD_PARTY_NOTICES.md:17](../THIRD_PARTY_NOTICES.md?plain=1#L17) now states the prescribed generator result. [Generator log:33](<extras>/generator/rust_generate_copyright.log:33) records 425,661 bytes, SHA-256 `af70…729f`, and `RESULT: IDENTICAL`. Generated and shipped files compare byte-for-byte. |
| 2 | MUST-FIX, version/pass metadata | **RESOLVED** | [RELEASE_NOTES.md:12](../RELEASE_NOTES.md?plain=1#L12), [manuscript:2](../paper/zeebeam.md?plain=1#L2), both companions and all rendered HTML/PDF copies agree on v3.10, v1.9, sixteen passes and four closing passes. |
| 3 | MUST-FIX, figure pass count | **RESOLVED** | [zeebeam.py:143](../figures/zeebeam.py#L143), SVG and PNG say `Sixteen referee passes`. |
| 4 | SHOULD-FIX, figure collision | **RESOLVED** | [zeebeam.py:135](../figures/zeebeam.py#L135) has the joined bullet. Original-resolution inspection found no collision, clipping or overflow. |
| 5 | LOW, trail count | **RESOLVED** | [RELEASE_NOTES.md:83](../RELEASE_NOTES.md?plain=1#L83) now says `17 reports and a README, 18 files`; current totals correctly say eighteen reports plus README. |

Both earlier declines are upheld. Keeping `Truth Beam` in the no-endorsement clause is coherent. Full font files remain unnecessary because the distributed subsets/outlines and applicable notices are present.

## 2. Remaining defects

1. **LOW, seven trail reports contain 96 dead GitHub line links.**

   Example, [FINDINGS_round16_closing.md:11](../referee_trail/FINDINGS_round16_closing.md?plain=1#L11):

   Exact quote:

   ```markdown
   [Notice:17](../THIRD_PARTY_NOTICES.md:17)
   ```

   Exact replacement:

   ```markdown
   [Notice:17](../THIRD_PARTY_NOTICES.md?plain=1#L17)
   ```

   For 66 Markdown targets, apply `../PATH.md:N` → `../PATH.md?plain=1#LN`. For 22 other existing text/code targets, apply `../PATH:N` → `../PATH#LN`. GitHub requires `?plain=1` for Markdown source-line linking. [GitHub documentation](https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/creating-a-permanent-link-to-a-code-snippet).

   Eight further links target files deliberately removed later: five `.zenodo.json`, two `Rust-COPYRIGHT-library.txt`, and one `Rust-COPYRIGHT.txt`. Replace those links with unlinked inline paths, for example:

   ```markdown
   [Rust-COPYRIGHT line 755](../licenses/Rust-COPYRIGHT.txt:755)
   ```

   becomes:

   ```markdown
   `licenses/Rust-COPYRIGHT.txt`, line 755
   ```

2. **LOW, LICENSE falsely says build records incorporate compiled code.**

   [LICENSE:61](../LICENSE#L61)

   Exact quote:

   > `the three guest ELFs and the retained build records incorporate compiled code from the crates named in the frozen lockfiles, whose licences are listed in THIRD_PARTY_NOTICES.md with the licence texts in licenses/;`

   Exact replacement:

   > `the three guest ELFs incorporate target-runtime code from packages named in the frozen guest-workspace Cargo.lock files and Rust standard-library components; build dependencies and procedural macros run during compilation and are not target-linked; the applicable packages and licences are listed in THIRD_PARTY_NOTICES.md, with the licence texts in licenses/;`

3. **LOW, LICENSE’s font summary omits the PDF subsets and Liberation Serif.**

   [LICENSE:65](../LICENSE#L65)

   Exact quote:

   > `the figures embed glyphs of the DejaVu fonts under the Bitstream Vera licence;`

   Exact replacement:

   > `figures/zeebeam.svg embeds DejaVu Sans glyph outlines under the Bitstream Vera licence and figures/zeebeam.png renders them; the three PDFs embed DejaVu subsets under that licence, and paper/zeebeam.pdf also embeds a Liberation Serif subset under the SIL Open Font License 1.1;`

   `THIRD_PARTY_NOTICES.md` already states both licence matters correctly, so defects 2 and 3 do not impair rights allocation. If changed, version the licence and update its active 1.2 references.

4. **LOW, publication-script retry comment contradicts the code.**

   [011_zeebeam_repo.sh:16](<extras>/011_zeebeam_repo.sh:16) says `a second run would try again`; line 47 refuses once `.git` exists.

   Replace lines 16–19 with:

   ```bash
   # ONE-SHOT: once .git is created, every later run refuses locally. Before any manual recovery,
   # inspect local and remote state. Fire only with exclusive administration of the repository. Once the
   # visibility request has been sent, a lost response or a failed read-back may leave the complete verified
   # repository public while this script exits non-zero: inspect it remotely before any retry.
   ```

## 3. Recomputed counts and digests

| Check | Recomputed result |
|---|---|
| Public tree | 3,112 regular files; 18,516,951 bytes; 107 directories; no `.git`, symlinks or special nodes |
| Git modes | 3,099 × `100644`; 13 × `100755` |
| Root ledger | 3,111 unique entries; exact non-self coverage; all pass |
| Root-ledger SHA-256 | `b7e66c74a215f8e194277987adefe6735235d7f6899374418922916292973288` |
| Git tree | `dd796a769de7687563b7826cdd107ada033c2507` |
| Commit message | 1,299 bytes; 23 lines; final LF; `fcf248d98c8075f609cafbada234fe211ec97aa6abea5724a7df71e22583ed61` |
| Bundle | 3,066 files; 14,809,148 bytes |
| Bundle ledger | 3,065/3,065 pass; `fad30aa0cf0339a500f2d450bd718c9856e7df0534ab4f7aff8b9382e661c03e` |
| `SOURCE_TREE` | 38/38; `e45be3c065f31b720418ba78504616dbabcd35275c8f7afdbfd6a94e634437a2` |
| `DEPENDENCY` | 57/57; `e8a586248d520f23f5dffcc84f58ebe557fd5a160bbf004468af32754abe6bc0` |
| `CHAIN_SOURCE` | 7/7; `c35d3230905f006aa529e3561bf3ea70f98e81cff307d443e1b113e9df7bf2ac` |
| Source-list coverage | 102 of 110 source files; the eight documented out-of-scope files remain covered by both higher ledgers |
| Proof inventory | 262 raw, 262 framed, 262 public-values files, 262 manifests |
| Trail | 18 reports plus README; seven reports with tree-relative links |

All packet files are byte-identical to the live publication target and transmit item.

## 4. Other checks

Identifier sweep, excluding trail placeholders and quotations:

| Category | Result |
|---|---:|
| Actual public IPv4 addresses | 0 |
| Chromium metadata `152.0.0.0` | 3, one per PDF |
| Provider identifiers | 0 actual values |
| Operational UUIDs | 0 |
| Former runner path | 0 |
| Retired principal tokens | 0 |
| Internal assistant call sign | 0 |
| Scientific session identifier | 549 occurrences / 547 files |
| `live_300s_training_001` | 29 / 17 |
| `campaign_recording_001` | 3 / 3 |
| `/tmp/go.tgz` | 2 / 1 |

The publication script matches its preview and passes `bash -n`. No script-created prompt path or public partial-tree path remains under exclusive administration. Pre-visibility failures may leave a private empty or partial repository. Post-visibility uncertainty may leave the complete verified commit public, exactly as disclosed.

The 166-row crate table exactly matches the proved guest lock union and vendor metadata. All three ELFs are static RISC-V binaries, report rustc 1.94.0-dev, and contain `core`, `alloc`, `std`, `panic_abort` and `compiler_builtins::mem::memmove`.

`pdffonts` reports 5 + 5 + 4 fonts, all embedded, subsetted and Unicode-mapped, matching the DejaVu and Liberation notices. The generated Rust library notice and shipped file are byte-identical; the whole-tree generator output is 13,854,550 bytes with SHA-256 `339573987987fb12edfd4ad47fe65976af4ccf6a0c54b379eb42fccc6dceb2a5`.

## 5. Final verdicts

| Artifact | Verdict |
|---|---|
| Manuscript v3.10 | **CLEARED. MAY PUBLISH.** |
| *ZeeBeam, by Example* v1.9 | **CLEARED. MAY PUBLISH.** |
| *ZeeBeam, without the maths* v1.9 | **CLEARED. MAY PUBLISH.** |
| Release tree `dd796a…2507` | **MAY PUSH after the principal’s reserved decisions. Four LOW defects remain; none blocks this exact tree.** |

This clearance applies only to the exact frozen tree above. If this fifth report is added, advance to seventeen passes, five closing passes, nineteen reports plus README and eight linked reports, then regenerate the root ledger, Git tree and transmission values. Do not fire item 011 after any public-tree change.

No file was modified.

— BOSUN ⚓