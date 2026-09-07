#!/bin/bash
# After the fleet: gather every pulled row proof into the bundle, export raw proofs, standalone-verify
# each, check proved statement == executed statement (all_rows_20260902/exec), write PINS_all_rows.json.
#   usage: fleet_collect_20260902.sh
set -u
J=/home/c/Documents/BOSUN/scratch/joined_build_20260901
S=$J/BOSUN/zeebeam-science/rust/row_binding_join_membership_sp1_candidate/script
CER=$S/target/release/zeebeam-row-binding-membership-ceremony
SV=$J/standalone_verifier/target/release/zeebeam-standalone-verifier
VK=0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490
B=$J/proofs_20260902/all_rows/proofs; mkdir -p $B $J/proofs_20260902/all_rows/logs
for d in $J/proofs_fleet_20260902/box*/box; do
  [ -d "$d" ] || continue
  cp -n $d/row_*_groth16.bin $d/row_*_manifest.json $d/row_*_public_values.hex $B/ 2>/dev/null
  b=$(basename $(dirname $d)); mkdir -p $J/proofs_20260902/all_rows/logs/$b
  cp -n $d/row_*_stdout.log $d/row_*_stderr.log $d/FAILED_ROWS.txt $J/proofs_20260902/all_rows/logs/$b/ 2>/dev/null
done
echo "framed proofs gathered: $(ls $B/row_*_groth16.bin | wc -l)"
: > $B/STANDALONE_VERIFY.txt
for p in $B/row_*_groth16.bin; do
  r=$(basename $p _groth16.bin)
  [ -f $B/${r}_groth16_proof.bin ] || $CER export --proof $p --out $B > /dev/null 2>&1
  # export writes row_NNN_groth16_proof.bin + row_NNN_groth16_public_values.bin named by the proof's row
  out=$($SV $B/${r}_groth16_proof.bin $B/${r}_groth16_public_values.bin $VK 2>&1 | grep -E "VERIFIED|rejected" | tr '\n' ' ')
  n=${r#row_}; n=$((10#$n))
  ex=$J/all_rows_20260902/exec/row_$(printf '%03d' $n)_public_values.hex
  same="no_exec"; [ -f "$ex" ] && { [ "$(xxd -p -c 4000 $B/${r}_groth16_public_values.bin | tr -d '\n')" = "$(tr -d '\n' < $ex)" ] && same=equal || same=DIFFERENT; }
  echo "$r $out statement_vs_executed=$same" >> $B/STANDALONE_VERIFY.txt
done
echo "verified: $(grep -c VERIFIED $B/STANDALONE_VERIFY.txt) / $(wc -l < $B/STANDALONE_VERIFY.txt); equal statements: $(grep -c statement_vs_executed=equal $B/STANDALONE_VERIFY.txt); different: $(grep -c DIFFERENT $B/STANDALONE_VERIFY.txt)"
python3 - <<'EOF'
import json, glob, hashlib, os
J='/home/c/Documents/BOSUN/scratch/joined_build_20260901'; B=f'{J}/proofs_20260902/all_rows/proofs'
rows={}
for m in sorted(glob.glob(f'{B}/row_*_manifest.json')):
    d=json.load(open(m)); r=d['row']
    raw=f'{B}/row_{r:03d}_groth16_proof.bin'
    rows[r]={k:d.get(k) for k in ('sp1_vkey','guest_elf_sha256','prove_elapsed_ms','setup_elapsed_ms','verify_and_tamper_elapsed_ms','proof_bytes','proof_sha256','public_values_bytes','public_values_sha256','oracle_mode','oracle_bytes_checked','oracle_bytes_circuit_derived','trapdoor_flag','prover')}
    if os.path.exists(raw):
        b=open(raw,'rb').read(); rows[r]['raw_groth16_proof_bytes']=len(b); rows[r]['raw_groth16_proof_sha256']=hashlib.sha256(b).hexdigest()
sv={l.split()[0]:l.strip() for l in open(f'{B}/STANDALONE_VERIFY.txt')} if os.path.exists(f'{B}/STANDALONE_VERIFY.txt') else {}
for r in rows: rows[r]['standalone']=sv.get(f'row_{r:03d}','')
vk={v['sp1_vkey'] for v in rows.values()}; elf={v['guest_elf_sha256'] for v in rows.values()}
pins={"schema":"zeebeam-pins/all-rows-v1","rows_proved":sorted(rows),"count":len(rows),"vkeys":sorted(vk),"elfs":sorted(elf),
      "all_same_vkey_and_elf": len(vk)==1 and len(elf)==1, "prove_ms_total": sum(v['prove_elapsed_ms'] or 0 for v in rows.values()),
      "prove_ms_min": min((v['prove_elapsed_ms'] for v in rows.values() if v['prove_elapsed_ms']), default=None),
      "prove_ms_max": max((v['prove_elapsed_ms'] for v in rows.values() if v['prove_elapsed_ms']), default=None),
      "oracle_all_full": all(v['oracle_bytes_circuit_derived']==0 and v['oracle_bytes_checked']==1101 for v in rows.values()),
      "rows": {str(r): rows[r] for r in sorted(rows)}}
json.dump(pins, open(f'{J}/proofs_20260902/PINS_all_rows.json','w'), indent=1)
print('PINS_all_rows:', pins['count'], 'rows; same vkey/elf:', pins['all_same_vkey_and_elf'], '; oracle full:', pins['oracle_all_full'], '; prove ms min/max', pins['prove_ms_min'], pins['prove_ms_max'])
EOF
echo FLEET_COLLECT_END
