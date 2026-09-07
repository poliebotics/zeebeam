#!/bin/bash
# Deterministic staging of the frozen source layout. The proved crates' manifests name their path
# dependencies and the jubjub patch by the development machine's absolute layout
# (/home/c/Documents/BOSUN/scratch/joined_build_20260901/...), so the bundled source does not resolve in
# place. This script recreates exactly that layout from the bundle, by copying (never by editing the frozen
# bytes), and refuses to touch a root that already exists. Run it before build_reproducible.sh or
# build_reproducible_chain.sh; the two model blobs are placed at the paths the crates embed.
#   usage: stage_for_rebuild.sh   (the root is fixed: the six absolute jubjub patch paths resolve nowhere else)
set -euo pipefail
HERE="$(cd "$(dirname "$(readlink -f "$0")")" && pwd)"
readonly ROOT="/home/c/Documents/BOSUN/scratch/joined_build_20260901"
if [ -e "$ROOT" ]; then echo "REFUSED: $ROOT already exists; stage into an empty root" >&2; exit 2; fi
for d in rust src deps/BOSUN/scratch vendor blobs; do [ -d "$HERE/$d" ] || { echo "REFUSED: $HERE/$d missing" >&2; exit 2; }; done
mkdir -p "$ROOT/BOSUN/zeebeam-science" "$ROOT/BOSUN/scratch"
cp -a "$HERE/rust" "$ROOT/BOSUN/zeebeam-science/rust"
# the canonical preprocessing configuration that rust/preprocess_v1_candidate/build.rs embeds at build time
cp -a "$HERE/src" "$ROOT/BOSUN/zeebeam-science/src"
cp -a "$HERE/deps/BOSUN/scratch/." "$ROOT/BOSUN/scratch/"
cp -a "$HERE/vendor" "$ROOT/vendor"
mkdir -p "$ROOT/BOSUN/scratch/zeebeam_lambda_launch_prep_20260823/zeebeam-trained-r32-ptq-v2-sp1-v1/frozen" \
         "$ROOT/BOSUN/scratch/uncr64/uncr64_pose_sp1_20260831/frozen"
cp "$HERE/blobs/r32_fit_calibrated_ptq_v2.bin" "$ROOT/BOSUN/scratch/zeebeam_lambda_launch_prep_20260823/zeebeam-trained-r32-ptq-v2-sp1-v1/frozen/"
cp "$HERE/blobs/uncr64_pose_ptq.bin" "$ROOT/BOSUN/scratch/uncr64/uncr64_pose_sp1_20260831/frozen/"
[ -f "$ROOT/BOSUN/zeebeam-science/src/zeebeam_science/preprocess_v1.candidate.json" ] || { echo "STAGING_FAILED: preprocessing configuration missing" >&2; exit 3; }
# every `path =` entry of every frozen manifest must now resolve
fail=0
while IFS= read -r m; do
  d=$(dirname "$m")
  while IFS= read -r rel; do
    case "$rel" in /*) t="$rel";; *) t="$d/$rel";; esac
    [ -e "$t" ] || { echo "UNRESOLVED: $m -> $rel" >&2; fail=1; }
  done < <(grep -oE 'path *= *"[^"]+"' "$m" | sed -E 's/path *= *"([^"]+)"/\1/')
done < <(find "$ROOT/BOSUN/zeebeam-science/rust" "$ROOT/BOSUN/scratch" "$ROOT/vendor" -name Cargo.toml -not -path '*/target/*')
[ "$fail" = 0 ] || { echo "STAGING_FAILED: some path dependencies do not resolve" >&2; exit 3; }
echo "STAGED_OK root=$ROOT"
