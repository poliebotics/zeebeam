#!/usr/bin/env python3
"""Stage A of the all-rows run: independent oracle values for every row of the 712-row session, and a
complete ROW_JSON witness (session tree + expectations) for each row, written next to the row-72/95 ones.

For each row t in the shard: (1) session_tree.py t -> row_{t:03d}_membership.json (tree path, chain fields,
expected_previous_drand_round); (2) row_oracle.py t -> row_{t:03d}_python_oracle.json (typed context, typed
root, uncropped commitment from the audited zeebeam-science modules); (3) frozen_models_python.run_row(t)
(torch re-run of both frozen networks from their blobs); (4) merge into the membership JSON; (5) append one
line to all_rows_20260902/oracle_rows_<shard>.jsonl. Rows outside the anchored 260-row prefix still get
oracle values (for the graphs) but cannot execute under the final relation, whose August leg requires the
row to lie inside the anchored prefix; that is recorded per row.

usage (from the zeebeam-science venv): all_rows_20260902.py <shard_index> <shard_count>
"""
import json, subprocess, sys, time
from pathlib import Path

J = Path('/home/c/Documents/BOSUN/scratch/joined_build_20260901')
OUT = J / 'all_rows_20260902'; OUT.mkdir(exist_ok=True)
PY = '/home/c/Documents/BOSUN/zeebeam-science/.venv/bin/python'
ROWS, PREFIX_ROWS = 712, 260
sys.path.insert(0, str(J))
import frozen_models_python as F  # noqa: E402

shard, nshards = int(sys.argv[1]), int(sys.argv[2])
rows = [t for t in range(ROWS) if t % nshards == shard]
r32, pose = F.r32_export(), F.pose_export()
F.parity_control(r32)
ctrl = F.run_row(96, r32, pose)
assert ctrl['coupling_score_numerator'] == F.FROZEN_96_SCORE, 'row-96 positive control failed'
log = open(OUT / f'oracle_rows_{shard}.jsonl', 'a')
for t in rows:
    t0 = time.time()
    mp = J / f'row_{t:03d}_membership.json'
    if t in (72, 95, 96) and mp.exists() and 'expected_pose_verdict' in mp.read_text():
        pass  # already complete from the ceremony work; do not overwrite
    else:
        subprocess.run([sys.executable, str(J / 'session_tree.py'), str(t)], check=True, capture_output=True, cwd=str(J))
    subprocess.run([PY, str(J / 'row_oracle.py'), str(t)], check=True, capture_output=True, cwd='/home/c/Documents/BOSUN/zeebeam-science')
    o = json.load(open(J / f'row_{t:03d}_python_oracle.json'))
    res = F.run_row(t, r32, pose)
    m = json.load(open(mp))
    if 'expected_pose_verdict' not in m:
        m.update({'expected_pose_verdict': res['pose_verdict'], 'expected_pose_logit_sums': res['pose_logit_sums'],
                  'expected_pose_head_saturation': res['pose_head_saturation'],
                  'expected_coupling_score_numerator': res['coupling_score_numerator'],
                  'expected_typed_context': o['typed_context'], 'expected_typed_root': o['typed_root'],
                  'expected_uncropped_commitment': o['uncropped_commitment'],
                  'expected_pose_source': 'frozen_models_python.py (torch re-run of the frozen uncr64 blob; 144-value r32 parity control and row-96 control passed first)',
                  'expected_scorer_source': 'row_oracle.py (audited zeebeam-science modules) + frozen_models_python.py (torch re-run of the frozen r32 blob)'})
        json.dump(m, open(mp, 'w'), indent=1)
    rec = {'row': t, 'in_anchored_prefix': t < PREFIX_ROWS, 'executable_under_final_relation': 0 < t < PREFIX_ROWS,
           'drand_round': int(m['drand_round']), 'previous_drand_round': m.get('expected_previous_drand_round'),
           'coupling_score_numerator': res['coupling_score_numerator'], 'pose_verdict': res['pose_verdict'],
           'pose_logit_sums': res['pose_logit_sums'], 'pose_head_saturation': res['pose_head_saturation'],
           'typed_root': o['typed_root'], 'uncropped_commitment': o['uncropped_commitment'], 'seconds': round(time.time() - t0, 1)}
    log.write(json.dumps(rec) + '\n'); log.flush()
    print(f"row {t}: verdict {res['pose_verdict']} numerator {res['coupling_score_numerator']} ({rec['seconds']} s)", flush=True)
print('SHARD_DONE', shard)
