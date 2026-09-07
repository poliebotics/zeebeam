#!/bin/bash
# Run on halo: push the proving payload to the box at the SAME absolute paths the crates expect.
# usage: transfer_to_box_20260902.sh <box-ip>
# The viewing key travels only inside this SSH session and is shredded on the box after the runs.
set -eu
IP="$1"; H="ubuntu@$IP"; SSH="ssh -o StrictHostKeyChecking=accept-new"
RS="rsync -az --info=stats1 -e \"$SSH\""
eval $RS --exclude 'target/' /home/c/Documents/BOSUN/scratch/joined_build_20260901/BOSUN "$H:/home/c/Documents/BOSUN/scratch/joined_build_20260901/"
eval $RS /home/c/Documents/BOSUN/scratch/joined_build_20260901/vendor "$H:/home/c/Documents/BOSUN/scratch/joined_build_20260901/"
eval $RS /home/c/Documents/BOSUN/scratch/joined_build_20260901/row_072_membership.json "$H:/home/c/Documents/BOSUN/scratch/joined_build_20260901/"
# explicit files (a filtered rsync here once copied whole spike target dirs; never again)
A=/home/c/Documents/BOSUN/scratch/august_leg_20260901
$SSH "$H" "mkdir -p $A/holder_ufvk_extract"
eval $RS $A/anchor_tx.hex $A/merkle_fixture_3456294.json $A/chain_prefix_260.bin $A/prefix_json.bin $A/binding_core_canonical.bin "$H:$A/"
scp -q $A/holder_ufvk_extract/holder_ivk_external.bin "$H:$A/holder_ufvk_extract/holder_ivk_external.bin"
eval $RS /data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings/frame_000072.raw /data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings/frame_000096.raw "$H:/data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings/"
$SSH "$H" 'chmod 600 /home/c/Documents/BOSUN/scratch/august_leg_20260901/holder_ufvk_extract/holder_ivk_external.bin; sha256sum /data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings/frame_0000{72,96}.raw /home/c/Documents/BOSUN/scratch/august_leg_20260901/anchor_tx.hex; du -sh /home/c/Documents/BOSUN/scratch/joined_build_20260901/BOSUN'
echo TRANSFER_DONE
