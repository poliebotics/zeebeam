---
version: 1.5
date: 2026-09-06
status: frozen-source-as-proved
author: BOSUN
---

# Frozen source of the proved relation

This directory holds every frozen first-party, path-dependency and patched source file, plus the two embedded
blobs, that the final guest ELF (`final_relation/zeebeam_guest_final.elf`, sha256 `8bcadf53…11db`, vkey
`0x0019d16c…0490`) was built from, byte for byte as proved in ceremony 2; registry and Git dependencies and the
Rust toolchain sources are not in this public tree (the separately archived offline kit vendors the crates).

| path | what |
|------|------|
| `rust/row_binding_join_sp1_candidate/` | the base relation crates: `join` (beacon verification, chain, emission, preprocess, typed root, coupling, pose), `native`, `program`, `script` |
| `rust/row_binding_join_membership_sp1_candidate/` | the membership crate (`membership/src/{lib,zcash,memo,august}.rs`), the proved guest `program/src/main.rs`, the host `script/` (witnesses, execute, ceremony, export, vkey) and `script/build.rs` (the `--locked`, `--remap-path-prefix` guest build) |
| `rust/zeebeam_chain_sp1_candidate/` | the whole-session chain relation (paper Section 9.5): guest `program/`, host `script/` (`zeebeam-chain execute|prove|verify|export|vkey`); separate workspace, depends by relative path on `rust/row_binding_join_sp1_candidate/join` and on the dependency crate below; digest list `final_relation/chain/CHAIN_SOURCE_SHA256SUMS`; proved ELF `final_relation/chain/zeebeam_chain_guest.elf` |
| `deps/BOSUN/scratch/…/b3xof_experiment_snapshot/source/sp1/relation/` | the `zeebeam-b3xof-relation` crate (BLAKE3-XOF emission relation) that `join`, `membership` and the chain guest depend on by relative path; digested with `vendor/` in `final_relation/DEPENDENCY_SHA256SUMS` |
| `build_reproducible_chain.sh` | the fail-closed driver for the chain guest: same discipline as `build_reproducible.sh`, pins ELF sha256 `4934b9e2…3926`, 460,416 bytes, vkey `0x005402a8…df50` |
| `vendor/jubjub-0.10.0/` | the vendored `jubjub` crate with four `const` field initialisers made non-const (RedJubjub never executes); referenced by `[patch.crates-io]` in the workspace roots |
| `blobs/r32_fit_calibrated_ptq_v2.bin` | the frozen coupling network (sha256 `0188b5c0…`, pinned in every statement's `coupling.blob_sha256`) |
| `src/zeebeam_science/preprocess_v1.candidate.json` | the canonical preprocessing configuration (764 bytes, sha256 `6345dc41…bd6d`; its digest is recorded in `PINS.json` and compiled into the guest, but is not an explicit field of the final statements) that `rust/preprocess_v1_candidate/build.rs` reads at build time and compiles into the guest; its digest line is `SRC_INPUT_SHA256SUMS`, and `stage_for_rebuild.sh` places it where the build script looks |
| `blobs/uncr64_pose_ptq.bin` | the frozen pose network (sha256 `c95b0072…`), embedded by `join/src/lib.rs` |
| `clean_build_20260906/` | filtered logs of the clean staged build of 6 September 2026 from an empty root (development tree moved aside, layout recreated from this bundle alone): `final_relation.log` and `chain.log`, each recording `BUILD_REPRODUCED_OK` with the pinned digest and key, followed by `exit=0` |
| `build_reproducible.sh` | the fail-closed build driver: `set -euo pipefail`, `cargo build --locked`, exactly one ELF, size, SHA-256 and vkey asserted against pins fixed in the script, non-zero exit on any mismatch; its development-machine transcript is `../final_relation/build_transcript_halo_20260902.txt` (the ceremony-2 box built with the historical fail-open driver of the day, whose result is recorded in `../final_relation/ceremony2/logs/build.log` and `vkey_check.log`; that driver is not shipped) |

Check the tree against the digest list that was recorded before proving:

```
cd rust && sha256sum -c ../../final_relation/SOURCE_TREE_SHA256SUMS
```

and the build-time configuration input against its own line (it lives outside `rust/`, so the pre-proving list does not cover it; its digest is recorded in `PINS.json` and bound by the pinned guest and verification key, not published as a field in the final statements):

```
sha256sum -c SRC_INPUT_SHA256SUMS
```

## What a fresh build needs

Both trees depend by relative path on `zeebeam-b3xof-relation`
(`<root>/../../../../scratch/zeebeam_lambda_artifacts_20260824/reviews/post_hold_followup_closure_20260825T0446Z/b3xof_experiment_snapshot/source/sp1/relation`,
bundled under `deps/BOSUN/scratch/…`; place `deps/BOSUN/` at `/home/c/Documents/BOSUN/scratch/joined_build_20260901/BOSUN/`
so that the mirrored path resolves) and the chain guest additionally on `rust/row_binding_join_sp1_candidate/join`
(`../../row_binding_join_sp1_candidate/join` from `rust/zeebeam_chain_sp1_candidate/program`, which resolves once
`rust/` is placed as described below). Check them with `cd . && sha256sum -c ../final_relation/DEPENDENCY_SHA256SUMS`
from this directory. The crates embed the two blobs with `include_bytes!` at paths relative to the crate root
(`<root>/../../../../scratch/zeebeam_lambda_launch_prep_20260823/zeebeam-trained-r32-ptq-v2-sp1-v1/frozen/r32_fit_calibrated_ptq_v2.bin`
and `<root>/../../../../scratch/uncr64/uncr64_pose_sp1_20260831/frozen/uncr64_pose_ptq.bin`, where
`<root>` is the crate directory under `rust/`), and the `jubjub` patch is declared by absolute path
`/home/c/Documents/BOSUN/scratch/joined_build_20260901/vendor/jubjub-0.10.0`. To rebuild, place this
directory's `rust/` at `/home/c/Documents/BOSUN/scratch/joined_build_20260901/BOSUN/zeebeam-science/rust/`,
`vendor/` at `/home/c/Documents/BOSUN/scratch/joined_build_20260901/vendor/`, and the two blobs at the
mirrored `scratch/…` paths above (the ceremony-2 bootstrap created exactly these paths on a fresh
machine), then run `build_reproducible.sh`; it exits 0 only if the ELF and key equal the release pins. `build_reproducible_chain.sh`
does the same for the chain guest (its transcript on the development machine is
`final_relation/chain/build_transcript_halo_20260903.txt`; the proving box's reproduction is `REPRODUCIBLE_CHAIN.json`). The `bls12_381` fork is fetched by cargo from
`https://github.com/sp1-patches/bls12_381` at tag `patch-0.8.0-sp1-6.2.0`, pinned by `Cargo.lock`.
Toolchain: SP1 6.4.0 (`sp1up --version v6.4.0`), host Rust 1.98.0; the Groth16 wrap needs Go at least 1.24.0 (the
vendored `sp1-recursion-gnark-ffi` declares `go 1.24.0`; the proving machines installed 1.23.4 and Go fetched a newer
toolchain automatically, unrecorded); see `../ENVIRONMENT.md`. Besides `zeebeam-b3xof-relation`, `join/Cargo.toml`
depends by relative path on `rust/preprocess_v1_candidate` (bundled here) and on the trained-r32 model and relation
crates and the uncr64 model and relation crates under `deps/BOSUN/scratch/…` (bundled, mirrored paths, digested in
`../final_relation/DEPENDENCY_SHA256SUMS`). The frozen manifests name these crates and the `jubjub` patch by the development machine's absolute layout, so they
do not resolve in place; `stage_for_rebuild.sh` (beside this file) recreates that layout by copying from the bundle,
refuses a root that already exists, and checks that every `path =` entry of every frozen manifest resolves before it
reports `STAGED_OK`. An air-gapped rebuild from this bundle alone is not possible, because the transitive registry and Git sources
and the host toolchains are absent; the separate kit supplies the crate vendors but not a complete host toolchain,
and an end-to-end air-gapped rebuild remains unrehearsed.

Because the ELF embeds panic-location line numbers, any edit that shifts a line changes the ELF and
the key (manuscript Section 7.3). The tree here is therefore the proved bytes, defects included:
`membership/src/memo.rs` states the memo leg's key commitment in its header comment without the
manuscript's A5 qualifier. The code is unaffected.

## Log

- 1.5 (2026-09-06, BOSUN): added the canonical preprocessing configuration and `SRC_INPUT_SHA256SUMS`, staged the configuration with `stage_for_rebuild.sh`, and recorded clean staged reproduction of both pinned ELFs and keys in `clean_build_20260906/`.

- 1.4 (2026-09-05, BOSUN) — coverage of this directory stated exactly; air-gapped rebuild stated as not possible from the bundle alone; staging script fixed to a single root and a complete manifest scan.

- 1.3 (2026-09-05, BOSUN) — `stage_for_rebuild.sh` added after Sol's verification pass found that 13 of 30 dependency and patch paths do not resolve in the bundle's own layout.

- 1.2 (2026-09-05, BOSUN) — the five further path-dependency crates bundled and listed; Go requirement stated from the crate's `go.mod`.

- 1.1 (2026-09-02, BOSUN) — fail-closed driver with fixed pins; the historical fail-open driver removed from the bundle.

- 1.0 (2026-09-02, BOSUN) — written after Sol's round-5 finding that the bundle advertised a
  reproducible build without its source.
