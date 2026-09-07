---
version: 1.21
date: 2026-09-06
status: release-1.0.0
author: BOSUN
---

# Release notes

## What this tree holds

ZeeBeam repository release 1.0.0, assembled on 4 and 5 September 2026 after eleven referee passes on the
manuscript, audits of the companions and of the release tree, a twelfth verification pass on the applied text and seven closing
passes, all by a second model, with every finding adjudicated (the trail and this file record the two that were
declined and why, and the one that was reversed). ZeeBeam names the
capture-and-proof system and a ZeeBeam recording a session proved under it; the work belongs to Dark
Lantern, the wider privacy and zero-knowledge research programme. `SHA256SUMS` at the root lists every
file in the tree but itself, the bundle's own `SHA256SUMS` included; check it with
`sha256sum -c SHA256SUMS`.

| path | what | files |
|------|------|-------|
| `paper/zeebeam.{md,html,pdf}` | the manuscript *ZeeBeam: The Zero-Knowledge Beam*, version 3.20 (the 3.16 text with the title given its paper form and the one look at the verification session recorded in Section 8.1) | 3 |
| `companions/zeebeam_worked_examples.*`, `zeebeam_for_dummies.*` | the worked examples and the plain-language companion, versions 1.16 and 1.18 respectively, each as Markdown, HTML and PDF | 6 |
| `figures/zeebeam.{png,svg,py}` | the boundary diagram and cost strip for one proved row, and the script that draws them | 3 |
| `bundle/proofs_20260902/` | the artifact: 259 row proofs under the final key, two previous-revision row proofs and the whole-session chain proof, statements, pins, decoded statements, prover manifests and logs, the frozen source tree carrying every path-dependency crate's bytes with `stage_for_rebuild.sh` for the layout its manifests expect and fail-closed build drivers, the standalone verifier, the Python oracles, the anchored prefix and receipt; its own `SHA256SUMS` covers 3,069 files (`VERIFY.md` 2.9, `ENVIRONMENT.md` 1.6) | 3,070 |
| `referee_trail/` | twenty-one reports (eleven manuscript passes, the companions audit, the release audit, the verification pass, seven closing passes) and a README describing the mechanical redaction of runner metadata, provider identifiers and machine paths | 22 |
| `README.md`, `LICENSE`, `THIRD_PARTY_NOTICES.md`, `licenses/` (9 texts and notices), `CITATION.cff`, `RELEASE_NOTES.md` | front matter and licence texts | 14 |
| `SHA256SUMS` | the root ledger | 1 |

Start with `README.md`, then `bundle/proofs_20260902/VERIFY.md`.

## The offline kit

The kit is archived separately from this tree because of its size. It holds the raw Zcash block 3456294 and
the raw anchor transaction from the Blockchair API, the 66 drand quicknet rounds the session used, the
toolchain archives the proving machines installed (SP1 6.4.0 `cargo prove`, the `succinct-1.94.0-64bit`
guest toolchain, Go 1.23.4, and the SP1 6.4.0 GPU server asset as a candidate only), and two
`cargo vendor --locked` trees (166 guest crates, 644 host crates). `MANIFEST.md` documents the provenance and
abbreviated digests of the fetched top-level artifacts and describes the vendor trees; `SHA256SUMS` gives
full digests for its 33,754 listed files; `THIRD_PARTY_NOTICES.md` lists every vendored crate with its
licence expression. The crate vendors support an offline rebuild of the guest ELFs given a preinstalled
Rust/Cargo 1.98 host toolchain; the kit does not contain the newer Go toolchain or Go modules that the
native Groth16 wrap needs (the vendored `sp1-recursion-gnark-ffi` declares `go 1.24.0`), and an
end-to-end air-gapped rebuild has not been rehearsed.

```
offline_kit_20260904.tar.zst
683,561,582 bytes
sha256 cec1b091015bc67b8b95ee167eb3c2daf57f655354dfcfff0dd57a043387b57e
```

The archive location is not yet fixed; this file will name it when it is. Two licensing questions about the kit are
still open and recorded in its `THIRD_PARTY_NOTICES.md`: the terms of the SP1 GPU-server archive and Blockchair's
terms for redistributing its API responses. Until they are settled the kit is not to be published; nothing in this
tree depends on it.

## Status

This is release 1.0.0 of the repository; nothing was published before it. The licence is the ZeeBeam
Research and Private Use Licence 1.3 (`LICENSE`): non-commercial research, teaching, verification and
private study are permitted and all other rights are reserved; third-party components keep their own
licences (`THIRD_PARTY_NOTICES.md`, texts in `licenses/`). No persistent identifier has yet been assigned. The author of record
is Cathal Ryan Hynes (PolieBotics); BOSUN, the project's automated research assistant, drafted, built,
proved and verified the work under the principal's direction, as the manuscript's Acknowledgements state.

The coupling-model stride correction was initially declined against the wrong record and then applied in
manuscript v3.6. Two other findings were declined: `Truth Beam` stays in the licence's names clause because it is
the principal's registered mark and the protocol tag `TB-v0.9` refers to it; and the referee's request to reproduce
every embedded font file was met with the licence texts and notices instead, the fonts being embedded as subsets.
No `.zenodo.json` is shipped: Zenodo's legacy importer can assert only one deposit-wide licence value, which would
mislabel this mixed-licence release, so a deposit for a persistent identifier is to be created by hand with the
owner licence and the third-party component licences declared separately.

Rows 260 to 711 of the session lie outside the on-chain receipt and have no proof under the full per-row
relation, although their log transitions are covered by the session-chain proof; covering them without
exercising the disclosed chameleon-equivocation capability would need a further anchor transaction, and none
has been made.

## Log

- 1.21 (2026-09-07, BOSUN) — the citation file's preferred citation and the worked-examples companion (1.16) now name manuscript 3.20 and its title *ZeeBeam: The Zero-Knowledge Beam*; the plain-language companion (1.18) follows; HTML/PDF re-rendered; no other change.
- 1.20 (2026-09-07, BOSUN) — the companions row now names the shipped plain-language companion version, 1.17; no other change.
- 1.19 (2026-09-06, BOSUN) — manuscript 3.20: title *ZeeBeam: The Zero-Knowledge Beam*; plain-language companion 1.17 (companion_to follows); HTML/PDF re-rendered; no other change.
- 1.18 (2026-09-06, BOSUN) — Astra's closing pass on the one-look release: manuscript 3.19 (Section 8 introduction and the one-look guarantee stated exactly), plain-language companion 1.16 (stale unscored-session account replaced); HTML/PDF re-rendered; no other change.
- 1.17 (2026-09-06, BOSUN) — manuscript 3.18: the one look at the 288-row verification session (6 September, PASS) recorded in Section 8.1 and the Section 11 limitation; HTML/PDF re-rendered; no other change.
- 1.16 (2026-09-06, BOSUN) — the manuscript, README and citation titles given their paper form (*ZeeBeam: One Relation to Bind Them*); the three HTML/PDF pairs re-rendered; no other change.
- 1.15 (2026-09-06, BOSUN) — GPT-6 Astra's closing pass 5 applied: the manuscript PDF re-rendered with a long-token fix (Appendix A had clipped the final digit of the Baby Jubjub group order; the Markdown was complete), manuscript v3.16, companions v1.15, the 1.14 entry's section references corrected to Section 7.4 and Appendix F, the referee-trail README's note of the round-12 exception, and the companions' 1.13 history entries restored to v3.14.
- 1.14 (2026-09-06, BOSUN) — the referee trail returns on the principal's ruling of 6 September (a trail stays where the release delivers a positive result): `referee_trail/` (twenty-one reports and its README) is back in the tree, its rows and pointers restored in this file, README, LICENSE, CITATION.cff and the manuscript (v3.15, whose Section 7.4 and Appendix F point at the published trail again); companions 1.14; the 1.12 entry below is history. No other content changed; the paper and companion PDFs and the paper HTML are re-rendered from the edited Markdown.
- 1.13 (2026-09-06, BOSUN) — the source bundle gains the canonical preprocessing configuration that the guest build embeds (a clean staged build from the bundle alone failed without it), its digest line `source/SRC_INPUT_SHA256SUMS`, a staging step, and the logs of the clean build of 6 September that reproduced both pinned ELFs and keys from an empty root; the paper's reproducibility sentence names them. No proof, statement or pin changed; both pinned ELF digests and verification keys were reproduced unchanged. The worked-examples companion advanced to 1.13 (bundle ledger count 3,069) and the plain-language companion to 1.13; the manuscript to 3.14; the table above follows.
- 1.12 (2026-09-06, BOSUN): the referee trail is held back for a later release on the principal's instruction; its rows and pointers are removed from README and this file, LICENSE and CITATION.cff no longer describe it as released, and the manuscript names the reports as retained project records. The paper HTML and PDF were re-rendered from amended paper Markdown; the companion PDFs were re-rendered from unchanged companion Markdown.
- 1.11 (2026-09-05, BOSUN) — after Sol's seventh closing pass, which found no defect, cleared the tree and accepted the stopping rule: its report added, nineteen passes; counts (trail 21 reports and a README, 22 files). No other change; no further pass.
- 1.10 (2026-09-05, BOSUN) — after Sol's sixth closing pass, which cleared the tree again: its two editorial items applied (the round-17 report's fenced quotations restored to the referee's exact text; Appendix F lists every closing report); eighteen passes; counts (trail 20 reports and a README, 21 files).
- 1.9 (2026-09-05, BOSUN) — after Sol's fifth closing pass, which cleared the tree: its four low items applied (trail line links in GitHub's anchor form with links to removed files unlinked; LICENSE 1.3 states the target-linked packages and the embedded font subsets exactly; the publication script's one-shot comment); seventeen passes; counts (trail 19 reports and a README, 20 files).
- 1.8 (2026-09-05, BOSUN) — after Sol's fourth closing pass: the status lines and cross-references of the manuscript and companions brought into agreement (v3.10, v1.9); the figure footer corrected to the pass count and its overprinted text fixed; the Rust library notice provenance settled by running the generator (THIRD_PARTY_NOTICES 1.5); sixteen passes; counts (trail 18 reports and a README, 19 files).
- 1.7 (2026-09-05, BOSUN) — after Sol's third closing pass: the Rust library notice replaced by the byte-exact generated notice of the 1.94.0 release, whose lockfile, COPYRIGHT and compiler-builtins licence are identical to the toolchain's source commit (which ships none); `compiler-builtins` MIT AND Apache-2.0 WITH LLVM-exception with its licence file; `blake2b_simd` copyright; the 166 packages described as the lockfile union; the publication script clears every `GIT_*` variable, pagers, editors and browsers and reads the published commit back; fifteen passes; counts (trail 17 reports and a README, 18 files; licence texts 9).
- 1.6 (2026-09-05, BOSUN) — after Sol's second closing pass: `.zenodo.json` withdrawn (mixed licences cannot be asserted by one value); fourteen passes; the declined and reversed findings restated; counts.
- 1.5 (2026-09-05, BOSUN) — after Sol's closing pass: thirteen passes; the three declined findings recorded with reasons; counts (trail 16 files, licence texts 8, VERIFY 2.9, ENVIRONMENT 1.6).
- 1.4 (2026-09-05, BOSUN) — after Sol's verification pass: status `release-1.0.0`; twelve passes, every finding adjudicated; counts (bundle 3,065 entries, trail 15 files, licence texts); LICENSE 1.2; kit's open licensing questions recorded.
- 1.3 (2026-09-05, BOSUN) — after Sol's release audit and eleventh manuscript pass: root ledger covers the bundle's ledger; counts and versions updated (manuscript 3.5, companions 1.4, VERIFY 2.7, ENVIRONMENT 1.4, bundle 3,064 entries, trail 14 files); kit description made exact and its repacked digest recorded; status rewritten for release 1.0.0; third-party notices added.
- 1.2 (2026-09-05, BOSUN) — ZeeBeam is the name of the release (manuscript v3.4, companions v1.3, zeebeam_* files); Dark Lantern the programme.
- 1.1 (2026-09-05, BOSUN) — renamed to Dark Lantern / ZeeBeam (manuscript v3.3, companions v1.2, new file names); LICENSE added; counts updated.
- 1.0 (2026-09-05, BOSUN) — written with the offline kit's digest once the tarball existed.
