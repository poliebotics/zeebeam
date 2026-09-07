#!/bin/bash
# Cold-verify a sample of fleet proofs on halo with the ceremony binary against the reproducible ELF.
set -u
J=/home/c/Documents/BOSUN/scratch/joined_build_20260901
CER=$J/BOSUN/zeebeam-science/rust/row_binding_join_membership_sp1_candidate/script/target/release/zeebeam-row-binding-membership-ceremony
ELF=$J/reproducible_20260902/zeebeam_guest_final.elf
B=$J/proofs_20260902/all_rows/proofs
OUT=$J/proofs_20260902/all_rows/logs/cold_verify_halo_20260903; mkdir -p $OUT
export TRAPDOOR_LE_HEX="$(tr -d ' \n' < $J/trapdoor_le.hex)" EXPECT_TRAPDOOR_FLAG=1
for t in 1 95 130 200 259; do
  r=$(printf '%03d' $t)
  s=$(date +%s)
  ROW_JSON=$J/row_${r}_membership.json $CER verify --proof $B/row_${r}_groth16.bin --elf $ELF > $OUT/row_${r}_cold_verify.log 2>&1
  rc=$?
  echo "row $t rc=$rc $(( $(date +%s) - s ))s $(grep -E '^(verified|verified_proof|vkey|sp1_vkey|oracle|tamper)' $OUT/row_${r}_cold_verify.log | tr '\n' ' ' | cut -c1-200)"
done
echo COLD_VERIFY_END
