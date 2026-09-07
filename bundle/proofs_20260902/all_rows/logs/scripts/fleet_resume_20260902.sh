#!/bin/bash
# Resume orchestrator, ON HALO, for a box that is already bootstrapped, loaded and built: push the
# (updated) box_run_rows script and the rows file, start the proving DETACHED on the box (nohup, so a
# dropped SSH session from halo cannot kill it), poll the box until BOX_ROWS_END (tolerating network
# loss), then pull the proofs, shred the secrets and terminate the instance.
#   usage: fleet_resume_20260902.sh <box-ip> <instance-id> <rows-file> <box-label>
set -u
IP="$1"; IID="$2"; ROWS_FILE="$3"; LABEL="$4"; H="ubuntu@$IP"
SSH="ssh -o StrictHostKeyChecking=accept-new -o ConnectTimeout=15 -o ServerAliveInterval=20 -o ServerAliveCountMax=3"
J=/home/c/Documents/BOSUN/scratch/joined_build_20260901; ROOT=$J
OUT=$J/proofs_fleet_20260902/$LABEL; mkdir -p "$OUT"
stamp() { date -u +%H:%M:%S; }
retry() { for i in $(seq 1 20); do "$@" && return 0; sleep 30; done; return 1; }
echo "$(stamp) RESUME $LABEL $IP rows=$(wc -l < "$ROWS_FILE")"
retry scp -q -o StrictHostKeyChecking=accept-new -o ConnectTimeout=15 $J/box_run_rows_20260902.sh "$ROWS_FILE" "$H:~/" || { echo "$(stamp) RESUME_${LABEL}_FAILED: cannot reach box"; exit 1; }
RF=$(basename "$ROWS_FILE")
# start detached with setsid so the prover outlives this SSH session; a lock file prevents double starts
retry $SSH "$H" "if [ -f ~/box_rows.lock ] && kill -0 \$(cat ~/box_rows.lock) 2>/dev/null; then echo already_running; else setsid nohup bash ~/box_run_rows_20260902.sh ~/$RF > ~/box_rows_detached.log 2>&1 < /dev/null & echo \$! > ~/box_rows.lock; echo started; fi" || { echo "$(stamp) RESUME_${LABEL}_FAILED: cannot start prover"; exit 1; }
echo "$(stamp) STAGE prove_rows_detached"
# poll: copy the box's log to halo each minute; finish when BOX_ROWS_END appears; give up after 12 h
for i in $(seq 1 720); do
  sleep 60
  $SSH "$H" "cat ~/box_rows_detached.log" > $OUT/box_rows.log.tmp 2>/dev/null && mv $OUT/box_rows.log.tmp $OUT/box_rows.log
  grep -q BOX_ROWS_END $OUT/box_rows.log 2>/dev/null && break
done
grep -q BOX_ROWS_END $OUT/box_rows.log 2>/dev/null || echo "$(stamp) WARNING: prover did not report BOX_ROWS_END within the poll window"
echo "$(stamp) rows done: $(grep -c 'ROW .* done' $OUT/box_rows.log) already: $(grep -c 'already proved' $OUT/box_rows.log) failed: $(grep -c FAILED $OUT/box_rows.log)"
echo "$(stamp) STAGE pull"
retry rsync -az -e "$SSH" "$H:$ROOT/proofs_fleet_20260902/" "$OUT/box/" || echo "$(stamp) WARNING: pull incomplete"
echo "$(stamp) pulled proofs: $(ls $OUT/box/row_*_groth16.bin 2>/dev/null | wc -l)"
echo "$(stamp) STAGE shred_and_terminate"
retry $SSH "$H" "shred -u $ROOT/trapdoor_le.hex 2>/dev/null; shred -u /home/c/Documents/BOSUN/scratch/august_leg_20260901/holder_ufvk_extract/holder_ivk_external.bin 2>/dev/null; echo SHREDDED" >> $OUT/pull.log 2>&1 || echo "$(stamp) WARNING: shred not confirmed"
for i in $(seq 1 30); do python3 /home/c/Documents/BOSUN/deploy/lambda/lambda_api.py terminate --id "$IID" --yes --force > $OUT/terminate.log 2>&1 && grep -qi "terminat" $OUT/terminate.log && break; sleep 60; done
tail -1 $OUT/terminate.log
echo "$(stamp) RESUME_${LABEL}_DONE"
