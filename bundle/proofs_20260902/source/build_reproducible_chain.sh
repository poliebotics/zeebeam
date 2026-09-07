#!/bin/bash
# Fail-closed reproducible build of the whole-session CHAIN guest ELF and its host binary.
# Exits non-zero unless exactly one guest ELF is produced AND its size, SHA-256 and SP1 vkey equal the
# pins of the proved chain relation, which are fixed below and cannot be overridden.
# Requires the frozen tree at the absolute paths described in README.md (rust/ under
# .../joined_build_20260901/BOSUN/zeebeam-science/, deps/BOSUN/ under .../joined_build_20260901/BOSUN/,
# vendor/ under .../joined_build_20260901/), SP1 6.4.0 (cargo-prove) on PATH.
#   usage: build_reproducible_chain.sh
set -euo pipefail
readonly EXPECT_ELF_SHA=4934b9e260d6250dcb119f4eaa9f46a3ff4eb144c02340175b356ef17ade3926
readonly EXPECT_VKEY=0x005402a848fdc787e32c0a110ca2662dd61c580481b4e418d5400a52fbd7df50
readonly EXPECT_ELF_BYTES=460416
DRIVER_SHA=$(sha256sum "$(readlink -f "$0")" | cut -c1-64)
echo "driver_sha256=$DRIVER_SHA"
S=/home/c/Documents/BOSUN/scratch/joined_build_20260901/BOSUN/zeebeam-science/rust/zeebeam_chain_sp1_candidate/script
cd "$S"
# The host build script drives the guest build through sp1-build and reruns when a guest source's
# mtime changes; touching the guest entry point (content unchanged) forces a full guest rebuild from
# source. The previous guest ELF is removed so that it cannot be reused.
touch ../program/src/main.rs
rm -f target/release/zeebeam-chain
find ../program/target/elf-compilation -type f -name "zeebeam-chain-program" -delete 2>/dev/null || true
cargo build --release --locked --bin zeebeam-chain
mapfile -t ELFS < <(find ../program/target/elf-compilation -type f -name "zeebeam-chain-program")
if [ "${#ELFS[@]}" -ne 1 ]; then echo "BUILD_FAILED: expected exactly one guest ELF, found ${#ELFS[@]}" >&2; exit 2; fi
E="${ELFS[0]}"
SHA=$(sha256sum "$E" | cut -c1-64)
SIZE=$(stat -c%s "$E")
HOME_PATHS=$(strings "$E" | grep -c '/home/' || true)
VKEY_LINE=$(./target/release/zeebeam-chain vkey 2>/dev/null | grep -E '^sp1_vkey=' || true)
VKEY="${VKEY_LINE#sp1_vkey=}"
echo "elf=$E"; echo "guest_elf_bytes=$SIZE guest_elf_sha256=$SHA"; echo "home_paths_in_elf=$HOME_PATHS"; echo "sp1_vkey=$VKEY"
if [ "$SIZE" != "$EXPECT_ELF_BYTES" ]; then echo "BUILD_FAILED: ELF size $SIZE != expected $EXPECT_ELF_BYTES" >&2; exit 3; fi
if [ "$SHA" != "$EXPECT_ELF_SHA" ]; then echo "BUILD_FAILED: ELF sha256 $SHA != expected $EXPECT_ELF_SHA" >&2; exit 3; fi
if [ "$VKEY" != "$EXPECT_VKEY" ]; then echo "BUILD_FAILED: vkey $VKEY != expected $EXPECT_VKEY" >&2; exit 4; fi
if [ "$HOME_PATHS" != "0" ]; then echo "BUILD_FAILED: $HOME_PATHS machine paths embedded in the ELF" >&2; exit 5; fi
echo "BUILD_REPRODUCED_OK sha256=$SHA vkey=$VKEY"
