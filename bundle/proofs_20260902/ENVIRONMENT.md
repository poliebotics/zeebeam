---
version: 1.6
date: 2026-09-05
status: locked-environment-description
author: BOSUN
---

# Locked environment for the ZeeBeam proof bundle

## Prover toolchain (both ceremonies)

| component | value | where pinned |
|-----------|-------|--------------|
| SP1 SDK / prover | 6.4.0 (circuit artifacts v6.1.0) | `PINS.json` `sp1`, manifests |
| `cargo prove` | `sp1 f66b4bf 2026-08-12`, installed with `sp1up --version v6.4.0` | `PINS.json`, `box_bootstrap_20260902.sh` |
| guest toolchain | `succinct` channel, rustc 1.94.0-dev (SP1's RISC-V toolchain) | `PINS.json` |
| host toolchain | Rust 1.98.0 (`rust-toolchain` of the host crate); rustup `--profile minimal` | `rust-toolchain`, bootstrap script |
| Groth16 wrap | `sp1-sdk` feature `native-gnark`, no docker. The bootstrap installed Go 1.23.4; the vendored `sp1-recursion-gnark-ffi` 6.4.0 declares `go 1.24.0`, so Go's toolchain rule fetched a newer Go toolchain at build time, whose exact version was not recorded | `Cargo.toml` features, bootstrap script, `go/go.mod` of the crate |
| GPU prover | `sp1-sdk` feature `cuda`, `sp1-gpu-server` auto-downloaded on first use | `Cargo.toml` features |
| guest build | `sp1-build` with `--locked` and `--remap-path-prefix` for `CARGO_HOME` and the tree root | `script/build.rs`, `final_relation/REPRODUCIBLE_FINAL.json` |
| lockfiles | `Cargo.lock` digests of the four workspace roots | `PINS.json` |
| vendored crate | `jubjub` 0.10.0 with four `const` field initialisers made non-const (RedJubjub never executes) | `vendor/`, source-tree digest list |
| patched crate | `bls12_381` fork `sp1-patches` tag `patch-0.8.0-sp1-6.2.0` | `Cargo.toml` `[patch.crates-io]` |
| complete source tree | 38 `.rs`/Cargo/build files, list sha256 `e45be3c065f31b720418ba78504616dbabcd35275c8f7afdbfd6a94e634437a2` | `final_relation/SOURCE_TREE_SHA256SUMS` |

## Proving machines

| ceremony | machine | GPU / driver | CPU / RAM | OS image | result |
|----------|---------|--------------|-----------|----------|--------|
| 1 (2 Sep 2026, ~05:30–06:17 UTC) | Lambda `gpu_1x_a100_sxm4`, us-east-1 | NVIDIA A100-SXM4-40GB | 30 vCPU, 216 GiB | Lambda Ubuntu image | rows 96 and 72, 1,085-byte statements, vkey `0x0092cba2…a82c`, ELF `5b3ebfcd…4ce5` |
| 2 (2 Sep 2026, 15:59–16:36 UTC) | Lambda `gpu_1x_a100_sxm4`, us-east-1 (instance `[provider identifier redacted]`) | NVIDIA A100-SXM4-40GB, driver 570.148.08 | 30 vCPU, 216 GiB, 497 GB disk | Lambda Ubuntu image, Docker 28.3.1 present but unused | final relation proved for rows 96 and 72 (857.4 s and 749.8 s); ELF `8bcadf53…11db` and vkey `0x0019d16c…0490` reproduced byte for byte from the frozen tree before proving; 37 min, about $1.23 |
| fleet (2–3 Sep 2026: launched ~19:50 UTC, dispatched 20:43 UTC, terminated 03:55–04:33 UTC) | 8 × Lambda `gpu_1x_a100_sxm4`, us-east-1 (per-instance records in `all_rows/logs/fleet_instances.json`, with provider identifiers removed before publication) | NVIDIA A100-SXM4-40GB (per-box driver not recorded) | 30 vCPU, 216 GiB each | Lambda Ubuntu image | 257 row proofs under the final key (rows 1–259 except 96 and 72); each box built the CUDA host from the frozen tree (build script rebuilds the guest) and printed the embedded ELF digest and vkey, matching the pins on all eight before proving; one box also built and proved the chain relation (ELF `4934b9e2…3926`, 477.7 s); prove 727.8–955.7 s per row (median 755.6 s), 54.6 GPU-hours; 4,073 instance-minutes, about $135 |

## Development and verification machine

`halo`: HP ZBook Ultra G1a, AMD Ryzen AI MAX+ PRO 395 (16 cores), 128 GB unified LPDDR5X, Ubuntu
26.04 LTS, kernel 7.0.0-27-generic, Python 3.14 with torch and numpy for the oracles. Cold
verification of the ceremony-1 proofs used the SP1 SDK against the exact box ELF; standalone
verification used the bundled verifier, whose cryptographic dependency is `sp1-verifier` 6.4.0 (it also uses `sha2`
0.10 to print digests). Host execution of the final ELF for both rows ran here
(61.5 s for row 96 and 58.6 s for row 72, `final_relation/execution_logs/`).

## Reproduction on the development machine

`source/build_reproducible.sh` (fail-closed) rebuilt the guest from source on halo on 2 September 2026
and reproduced ELF `8bcadf53…11db` and vkey `0x0019d16c…0490` with zero machine paths embedded
(`final_relation/build_transcript_halo_20260902.txt`, `BUILD_REPRODUCED_OK`, exit 0).

## Process cleanup in ceremony 2

Both proving processes panicked after `verified_proof=true` and after the manifests and proofs were
written, in `sp1-cuda-6.4.0/src/pk.rs:63` and `client.rs:221` ("there is no reactor running, must be
called from the context of a Tokio 1.x runtime"), and were terminated by signal 6; GNU time reports
exit status 0 for the wrapper. The proofs verify independently on halo. Logs:
`final_relation/ceremony2/logs/row_96_stderr.log`, `row_72_stderr.log`.

## Fleet incident and restart

At 23:10 UTC on 2 September a network drop on the development machine killed the fleet's proving
processes, which were children of SSH sessions, after 99 proofs had been written and verified in-process;
the orchestrators recorded their boxes as finished without terminating them. At 23:58 UTC the proving was
restarted on each box detached from any session (`setsid nohup`, a lock file against double starts),
skipping rows whose proof existed and had verified; the development machine polled each box's log every
minute, pulled the proofs on completion, shredded the trapdoor and viewing-key files on the box and
terminated it. No row needed its single permitted retry. Orchestration logs and scripts: `all_rows/logs/`.

## Chain host lockfile

The chain crate's host (`source/rust/zeebeam_chain_sp1_candidate/script/Cargo.lock`) pins `sp1-sdk` 6.4.0 like the row host, but
resolves `sp1-cuda` 6.6.0 transitively where the row host resolves 6.4.0. The chain guest was built and
proved with that lockfile on the fleet box that reproduced its ELF and vkey (`REPRODUCIBLE_CHAIN.json`),
and the proof verifies under `sp1-verifier` 6.4.0 standalone. On 3 September the development machine rebuilt the
chain guest from the bundled tree with the fail-closed driver `source/build_reproducible_chain.sh` (`cargo build
--release --locked`, previous ELF deleted first) and reproduced ELF `4934b9e2…3926` (460,416 B, no machine paths)
and vkey `0x005402a8…df50`: `final_relation/chain/build_transcript_halo_20260903.txt`, ending `BUILD_REPRODUCED_OK`.

## Not pinned

The exact `sp1-gpu-server` build fetched on the box (the SDK downloads it at first use; the box log records only `Running sp1-gpu-server 6.4.0 with device 0`), the exact Go toolchain the Groth16 wrap was compiled with (installed 1.23.4, required at least 1.24.0, fetched automatically), and the Lambda base image build number. All three are
recorded as gaps rather than filled in from memory.

## Log

- 1.6 (2026-09-05, BOSUN) — title and fleet-record wording after Sol's closing pass.

- 1.5 (2026-09-05, BOSUN) — provider instance identifier removed; standalone verifier dependencies stated exactly.

- 1.4 (2026-09-05, BOSUN) — Go toolchain stated as installed 1.23.4 with the crate's `go 1.24.0` requirement and automatic fetch; added to the not-pinned list.

- 1.3 (2026-09-03, BOSUN) — chain guest reproduced on the development machine with the fail-closed chain driver; crate path corrected.

- 1.2 (2026-09-03, BOSUN) — fleet row; incident and detached restart; chain-host lockfile note.

- 1.1 (2026-09-02, BOSUN) — host execution times attributed to their rows (round-8 finding); reproduction section; cleanup note.

- 1.0 (2026-09-02, BOSUN) — written for the artifact after Sol's round-4 request for a locked
  environment description.
