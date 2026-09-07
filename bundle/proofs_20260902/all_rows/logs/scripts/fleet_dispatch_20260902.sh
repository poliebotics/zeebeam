#!/bin/bash
# Fleet dispatcher, ON HALO: for every instance in fleet_20260902.json that is ACTIVE according to the
# Lambda API (not the possibly stale record) and has no orchestrator yet, start an orchestrator in the
# background with the next unassigned row shard. The first dispatched box also proves the chain relation
# (fleet_run_chain_first_20260902.sh); the others run fleet_run_20260902.sh. Idempotent.
#   usage: fleet_dispatch_20260902.sh
set -u
J=/home/c/Documents/BOSUN/scratch/joined_build_20260901
F=$J/fleet_20260902.json; D=$J/fleet_dispatch_state.json
[ -f "$F" ] || { echo "no fleet record"; exit 0; }
python3 - "$F" "$D" <<'EOF'
import json, os, subprocess, sys, glob
sys.path.insert(0, "/home/c/Documents/BOSUN/deploy/lambda")
import lambda_api as L
F, D = sys.argv[1], sys.argv[2]
J = '/home/c/Documents/BOSUN/scratch/joined_build_20260901'
fleet = json.load(open(F)); state = json.load(open(D)) if os.path.exists(D) else {"dispatched": {}}
api = L.LambdaAPI(L.load_key())
live = {i['id']: i for i in (api.request("GET", "/instances").get("data") or [])}
shards = sorted(glob.glob(f'{J}/fleet_rows/rows_shard_*_of_*.txt'))
used = set(v['shard'] for v in state['dispatched'].values())
free = [s for s in shards if s not in used]
for inst in fleet['instances']:
    iid = inst['id']
    if iid in state['dispatched']:
        continue
    cur = live.get(iid, {})
    if cur.get('status') != 'active' or not cur.get('ip'):
        print('not active yet:', iid[:8], cur.get('status')); continue
    if not free:
        print('no free shard for', iid[:8]); continue
    shard = free.pop(0)
    n = len(state['dispatched'])
    label = f"box{n}"
    script = 'fleet_run_chain_first_20260902.sh' if n == 0 else 'fleet_run_20260902.sh'
    log = f'{J}/proofs_fleet_20260902/{label}.orchestrator.log'
    os.makedirs(f'{J}/proofs_fleet_20260902', exist_ok=True)
    subprocess.run(f'nohup bash {J}/{script} {cur["ip"]} {iid} {shard} {label} > {log} 2>&1 &', shell=True, check=True)
    state['dispatched'][iid] = {"ip": cur['ip'], "shard": shard, "label": label, "log": log, "script": script}
    print('dispatched', label, cur['ip'], os.path.basename(shard), script)
json.dump(state, open(D, 'w'), indent=1)
print('dispatched total:', len(state['dispatched']), 'of', len(fleet['instances']), 'recorded instances')
EOF
