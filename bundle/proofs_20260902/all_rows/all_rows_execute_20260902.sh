#!/bin/bash
# Stage B of the all-rows run, ON HALO: execute every anchored row (1..259) under the FINAL ELF with its
# independent-oracle ROW_JSON, eight at a time. Row 0 has no previous row (the previous-row leg refuses
# it by design) and rows >= 260 lie outside the anchored prefix (the August join refuses them), so they
# are not executed here; their oracle values are in all_rows_20260902/oracle_rows_*.jsonl.
#   usage: all_rows_execute_20260902.sh [parallelism]
set -u
P="${1:-8}"
J=/home/c/Documents/BOSUN/scratch/joined_build_20260901
S=$J/BOSUN/zeebeam-science/rust/row_binding_join_membership_sp1_candidate/script
B=$S/target/release/zeebeam-row-binding-membership-execute
FR=/data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings
OUT=$J/all_rows_20260902/exec; mkdir -p "$OUT"
export TRAPDOOR_LE_HEX="$(tr -d ' \n' < $J/trapdoor_le.hex)" EXPECT_TRAPDOOR_FLAG=1
one() {
  t=$1; r=$(printf '%03d' "$t"); log=$OUT/row_${r}_execute.log
  ROW_JSON=$J/row_${r}_membership.json "$B" --raw "$FR/frame_000${r}.raw" > "$log" 2>&1
  grep -o 'public_values_hex=[0-9a-f]*' "$log" | cut -d= -f2 > $OUT/row_${r}_public_values.hex
  o=$(grep -E '^oracle=' "$log" | head -1); v=$(grep -E '^verified_public_values=' "$log" | head -1); n=$(grep -E '^total_instruction_count=' "$log" | head -1)
  echo "$t $o $v $n $(grep -c panicked "$log")" >> $OUT/summary.txt
}
export -f one; export J S B FR OUT TRAPDOOR_LE_HEX EXPECT_TRAPDOOR_FLAG
: > $OUT/summary.txt
seq 1 259 | xargs -P "$P" -I{} bash -c 'one {}'
sort -n $OUT/summary.txt -o $OUT/summary.txt
echo "rows executed: $(wc -l < $OUT/summary.txt)  verified: $(grep -c 'verified_public_values=true' $OUT/summary.txt)  panics: $(awk '$NF!=0' $OUT/summary.txt | wc -l)"
echo ALL_ROWS_EXECUTE_END
