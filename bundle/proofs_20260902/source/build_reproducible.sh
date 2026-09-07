#!/bin/bash
# Fail-closed reproducible build of the proved guest ELF and the host binaries.
# Exits non-zero unless exactly one guest ELF is produced AND its SHA-256 and SP1 vkey equal the
# release pins of the final relation (ceremony 2), which are fixed below and cannot be overridden.
# Requires the frozen tree at the absolute paths described in README.md, SP1 6.4.0 (cargo-prove) on
# PATH, and the two model blobs at their mirrored paths.
#   usage: build_reproducible.sh
set -euo pipefail
readonly EXPECT_ELF_SHA=8bcadf5373742e92eb36535680e4b9f092c00fdcf9a8f7171678c49b170f11db
readonly EXPECT_VKEY=0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490
readonly EXPECT_ELF_BYTES=1171416
DRIVER_SHA=$(sha256sum "$(readlink -f "$0")" | cut -c1-64)
echo "driver_sha256=$DRIVER_SHA"
S=/home/c/Documents/BOSUN/scratch/joined_build_20260901/BOSUN/zeebeam-science/rust/row_binding_join_membership_sp1_candidate/script
cd "$S"
# The host build script drives the guest build through sp1-build and reruns when a guest source's
# mtime changes; touching the guest entry point (content unchanged) forces a full guest rebuild from
# source. The previous guest ELF is removed so that it cannot be reused.
touch ../program/src/main.rs
rm -f target/release/zeebeam-row-binding-membership-ceremony target/release/zeebeam-row-binding-membership-execute
find ../program/target/elf-compilation -type f -name "zeebeam-row-binding-membership-program" -delete 2>/dev/null || true
cargo build --release --locked --bin zeebeam-row-binding-membership-ceremony --bin zeebeam-row-binding-membership-execute
mapfile -t ELFS < <(find ../program/target/elf-compilation -type f -name "zeebeam-row-binding-membership-program")
if [ "${#ELFS[@]}" -ne 1 ]; then echo "BUILD_FAILED: expected exactly one guest ELF, found ${#ELFS[@]}" >&2; exit 2; fi
E="${ELFS[0]}"
SHA=$(sha256sum "$E" | cut -c1-64)
SIZE=$(stat -c%s "$E")
HOME_PATHS=$(strings "$E" | grep -c '/home/' || true)
VKEY_LINE=$(./target/release/zeebeam-row-binding-membership-ceremony vkey 2>/dev/null | grep -E '^sp1_vkey=' || true)
VKEY="${VKEY_LINE#sp1_vkey=}"
echo "elf=$E"; echo "guest_elf_bytes=$SIZE guest_elf_sha256=$SHA"; echo "home_paths_in_elf=$HOME_PATHS"; echo "sp1_vkey=$VKEY"
if [ "$SIZE" != "$EXPECT_ELF_BYTES" ]; then echo "BUILD_FAILED: ELF size $SIZE != expected $EXPECT_ELF_BYTES" >&2; exit 3; fi
if [ "$SHA" != "$EXPECT_ELF_SHA" ]; then echo "BUILD_FAILED: ELF sha256 $SHA != expected $EXPECT_ELF_SHA" >&2; exit 3; fi
if [ "$VKEY" != "$EXPECT_VKEY" ]; then echo "BUILD_FAILED: vkey $VKEY != expected $EXPECT_VKEY" >&2; exit 4; fi
if [ "$HOME_PATHS" != "0" ]; then echo "BUILD_FAILED: $HOME_PATHS machine paths embedded in the ELF" >&2; exit 5; fi
echo "BUILD_REPRODUCED_OK sha256=$SHA vkey=$VKEY"
