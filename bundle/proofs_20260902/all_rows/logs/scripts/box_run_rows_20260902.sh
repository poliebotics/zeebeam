#!/bin/bash
# Run ON a fleet box: prove every row listed in a rows file (one row index per line) with the CUDA
# ceremony binary, each with its independent-oracle ROW_JSON, retrying a row once if the first attempt
# does not verify (the first CUDA call can race the gpu-server socket). Rows whose proof already exists
# and verified are skipped, so the same rows file can be re-run to resume. Leaves proofs + manifests in OUT.
# Run it DETACHED on the box (nohup ... &) so that a dropped SSH session from halo cannot kill the prover.
#   usage: box_run_rows_20260902.sh <rows-file>
set -u
ROWS_FILE="$1"
export PATH="$HOME/.cargo/bin:$HOME/.sp1/bin:/usr/local/go/bin:$PATH"
ROOT=/home/c/Documents/BOSUN/scratch/joined_build_20260901
S=$ROOT/BOSUN/zeebeam-science/rust/row_binding_join_membership_sp1_candidate/script
OUT=$ROOT/proofs_fleet_20260902; mkdir -p "$OUT"
FR=/data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings
B=$S/target/release/zeebeam-row-binding-membership-ceremony
[ -x "$B" ] || { echo "CEREMONY BINARY ABSENT"; exit 1; }
export TRAPDOOR_LE_HEX="$(tr -d ' \n' < $ROOT/trapdoor_le.hex)" EXPECT_TRAPDOOR_FLAG=1
prove_one() {
  t=$1; r=$(printf '%03d' "$t")
  ROW_JSON=$ROOT/row_${r}_membership.json "$B" prove --raw "$FR/frame_000${r}.raw" --out "$OUT" --prover cuda > "$OUT/row_${r}_stdout.log" 2> "$OUT/row_${r}_stderr.log"
  grep -q '^verified_proof=true' "$OUT/row_${r}_stdout.log"
}
while read -r t; do
  [ -z "$t" ] && continue
  r=$(printf '%03d' "$t")
  if [ -f "$OUT/row_${r}_groth16.bin" ] && grep -q '^verified_proof=true' "$OUT/row_${r}_stdout.log" 2>/dev/null; then
    echo "$(date -u +%H:%M:%S) ROW $t already proved"; continue
  fi
  echo "$(date -u +%H:%M:%S) ROW $t start"
  if ! prove_one "$t"; then
    echo "$(date -u +%H:%M:%S) ROW $t first attempt did not verify; retrying once"; sleep 15
    prove_one "$t" || { echo "$(date -u +%H:%M:%S) ROW $t FAILED twice"; echo "$t" >> "$OUT/FAILED_ROWS.txt"; continue; }
  fi
  echo "$(date -u +%H:%M:%S) ROW $t done $(grep -E '^prove_elapsed_ms=' $OUT/row_${r}_stdout.log)"
done < "$ROWS_FILE"
echo BOX_ROWS_END
