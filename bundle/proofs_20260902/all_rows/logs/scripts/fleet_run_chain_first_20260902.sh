#!/bin/bash
# Fleet orchestrator variant for the FIRST box: after bootstrap/transfer/build it also pushes the chain
# crate, builds its CUDA host, proves the whole-session chain relation (ZBCHAIN1) and pulls that proof,
# then proves its row shard like every other box, pulls, shreds, terminates.
#   usage: fleet_run_chain_first_20260902.sh <box-ip> <instance-id> <rows-file> <box-label>
set -u
IP="$1"; IID="$2"; ROWS_FILE="$3"; LABEL="$4"; H="ubuntu@$IP"; SSH="ssh -o StrictHostKeyChecking=accept-new -o ServerAliveInterval=30"
J=/home/c/Documents/BOSUN/scratch/joined_build_20260901; ROOT=$J
R=$ROOT/BOSUN/zeebeam-science/rust
S=$R/row_binding_join_membership_sp1_candidate/script
CS=$R/zeebeam_chain_sp1_candidate/script
OUT=$J/proofs_fleet_20260902/$LABEL; mkdir -p "$OUT"
EXPECT_ELF=8bcadf5373742e92eb36535680e4b9f092c00fdcf9a8f7171678c49b170f11db
EXPECT_VKEY=0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490
stamp() { date -u +%H:%M:%S; }
fail() { echo "$(stamp) FLEET_${LABEL}_FAILED: $*"; exit 1; }
echo "$(stamp) STAGE wait_ssh $IP"
for i in $(seq 1 40); do $SSH -o ConnectTimeout=8 "$H" true 2>/dev/null && break; sleep 15; done
$SSH -o ConnectTimeout=8 "$H" true || fail "ssh never came up"
echo "$(stamp) STAGE bootstrap"
scp -q -o StrictHostKeyChecking=accept-new $J/box_bootstrap_20260902.sh $J/box_run_rows_20260902.sh "$ROWS_FILE" "$H:~/"
$SSH "$H" 'bash ~/box_bootstrap_20260902.sh' > $OUT/bootstrap.log 2>&1 || fail "bootstrap"
grep -q BOOTSTRAP_DONE $OUT/bootstrap.log || fail "bootstrap did not finish"
echo "$(stamp) STAGE transfer"
bash $J/transfer_to_box_20260902.sh "$IP" > $OUT/transfer.log 2>&1 || fail "transfer"
grep -q TRANSFER_DONE $OUT/transfer.log || fail "transfer did not finish"
rsync -az -e "$SSH" $J/row_*_membership.json "$H:$ROOT/" >> $OUT/transfer.log 2>&1 || fail "row witnesses"
awk '{printf "frame_%06d.raw\n",$1}' "$ROWS_FILE" > $OUT/frames_list.txt
scp -q $OUT/frames_list.txt "$H:~/frames_list.txt"
# frames: prefer the attached ZeeBeam filesystem on the box (no metered upload from halo), then rsync whatever is still missing
$SSH "$H" 'REC=/data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings; mkdir -p $REC; FSR=$(for d in ~/ZeeBeam/august_dev_712/Recordings $(find ~/ZeeBeam -maxdepth 6 -type d -path "*live_300s_training_001/Recordings" 2>/dev/null); do [ -f "$d/frame_000096.raw" ] && echo "$d" && break; done); if [ -n "$FSR" ]; then while read f; do [ -f "$REC/$f" ] || cp "$FSR/$f" "$REC/$f" 2>/dev/null && chmod 644 "$REC/$f"; done < ~/frames_list.txt; echo "frames_from_fs=$(ls $REC | wc -l) fs=$FSR"; else echo "frames_from_fs=0 (no ZeeBeam FS copy)"; fi' >> $OUT/transfer.log 2>&1
rsync -az --ignore-existing -e "$SSH" --files-from=$OUT/frames_list.txt /data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings/ "$H:/data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings/" >> $OUT/transfer.log 2>&1 || fail "frames"
rsync -az -e "$SSH" --exclude 'target/' $R/zeebeam_chain_sp1_candidate "$H:$R/" >> $OUT/transfer.log 2>&1 || fail "chain crate"
rsync -az -e "$SSH" $J/chain_expect.json /data/zeebeam_evidence_20260822/stage/live_300s_training_001/chain_log.csv "$H:$ROOT/" >> $OUT/transfer.log 2>&1 || fail "chain inputs"
$SSH "$H" "mkdir -p /data/zeebeam_evidence_20260822/stage/live_300s_training_001 && cp $ROOT/chain_log.csv /data/zeebeam_evidence_20260822/stage/live_300s_training_001/chain_log.csv"
scp -q $J/trapdoor_le.hex "$H:$ROOT/trapdoor_le.hex" && $SSH "$H" "chmod 600 $ROOT/trapdoor_le.hex" || fail "trapdoor copy"
echo "$(stamp) STAGE build_cuda (rows host)"
$SSH "$H" "export PATH=\$HOME/.cargo/bin:\$HOME/.sp1/bin:/usr/local/go/bin:\$PATH; cd $S && cargo build --release --features cuda --bin zeebeam-row-binding-membership-ceremony 2>&1 | grep -E 'error\[|^error|Finished' | head -20" > $OUT/build.log 2>&1
grep -q Finished $OUT/build.log || fail "rows host build"
$SSH "$H" "export PATH=\$HOME/.cargo/bin:\$HOME/.sp1/bin:/usr/local/go/bin:\$PATH; cd $S && ./target/release/zeebeam-row-binding-membership-ceremony vkey 2>/dev/null" > $OUT/vkey_check.log 2>&1
grep -q "$EXPECT_VKEY" $OUT/vkey_check.log || fail "vkey mismatch"
grep -q "$EXPECT_ELF" $OUT/vkey_check.log && echo "$(stamp) ELF_REPRODUCED_ON_BOX"
echo "$(stamp) STAGE build_cuda (chain host)"
$SSH "$H" "export PATH=\$HOME/.cargo/bin:\$HOME/.sp1/bin:/usr/local/go/bin:\$PATH; cd $CS && cargo build --release --features cuda 2>&1 | grep -E 'error\[|^error|Finished' | head -20" > $OUT/chain_build.log 2>&1
grep -q Finished $OUT/chain_build.log || fail "chain host build"
echo "$(stamp) STAGE prove_chain"
for attempt in 1 2; do
  $SSH "$H" "export PATH=\$HOME/.cargo/bin:\$HOME/.sp1/bin:/usr/local/go/bin:\$PATH; cd $CS && ./target/release/zeebeam-chain prove --out $ROOT/proofs_chain_20260902 --prover cuda --expect $ROOT/chain_expect.json" > $OUT/chain_prove_$attempt.log 2>&1
  grep -q "verified_proof=true" $OUT/chain_prove_$attempt.log && break
  echo "$(stamp) chain prove attempt $attempt did not verify; retrying"; sleep 15
done
grep -E "guest_elf|sp1_vkey|oracle=|prove_elapsed|proof_bytes|verified_proof" $OUT/chain_prove_*.log | tail -8
rsync -az -e "$SSH" "$H:$ROOT/proofs_chain_20260902/" "$OUT/chain/" >> $OUT/pull.log 2>&1 || echo "$(stamp) WARNING: chain pull incomplete"
echo "$(stamp) STAGE prove_rows ($(wc -l < "$ROWS_FILE") rows)"
$SSH "$H" "bash ~/box_run_rows_20260902.sh ~/$(basename "$ROWS_FILE")" > $OUT/box_rows.log 2>&1
grep -c "ROW .* done" $OUT/box_rows.log | sed 's/^/rows done: /'
echo "$(stamp) STAGE pull"
rsync -az -e "$SSH" "$H:$ROOT/proofs_fleet_20260902/" "$OUT/box/" >> $OUT/pull.log 2>&1 || echo "$(stamp) WARNING: pull incomplete"
echo "$(stamp) STAGE shred_and_terminate"
$SSH "$H" "shred -u $ROOT/trapdoor_le.hex; shred -u /home/c/Documents/BOSUN/scratch/august_leg_20260901/holder_ufvk_extract/holder_ivk_external.bin; echo SHREDDED" >> $OUT/pull.log 2>&1
python3 /home/c/Documents/BOSUN/deploy/lambda/lambda_api.py terminate --id "$IID" --yes --force > $OUT/terminate.log 2>&1; tail -1 $OUT/terminate.log
echo "$(stamp) FLEET_${LABEL}_DONE"
