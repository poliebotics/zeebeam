#!/usr/bin/env python3
"""Seed robustness and an empirical null for the pose crop/background decomposition.

Sol's audit correctly objected that one initialisation is not evidence of stability and one
deterministic label permutation is not a null distribution. This runs several
initialisations per arm at r64, and several published-mechanism label permutations, to give
an empirical spread instead of a single number.

The rung spec pins initialization_seed by contract, so alternative seeds are applied by
re-initialising the constructed model's conv layers under a fresh manual_seed, which is the
same default scheme the module uses, not a different architecture.
"""
import json, sys, statistics
sys.path.insert(0, "/home/c/Documents/BOSUN/zeebeam-science/src")
sys.path.insert(0, "/home/c/Documents/BOSUN/zeebeam-science/tools")
sys.dont_write_bytecode = True
import numpy as np, torch, torch.nn as nn
from zeebeam_science import camera_pose_resolution_ladder_candidate as L
import run_camera_pose_resolution_ladder as R

D = "/home/c/Documents/BOSUN/scratch/zeebeam_camera_pose_ladder_20260823/real_august_seed20260823_e64_v1"
CACHE = "/home/c/Documents/BOSUN/scratch/pose_nocrop_20260830/cache"
OUT = "/home/c/Documents/BOSUN/scratch/pose_nocrop_20260830/out/robustness.json"
CFG = "/home/c/Documents/BOSUN/zeebeam-science/configs/camera_pose_resolution_ladder.candidate.json"
torch.set_num_threads(12)
config = R._validate_config(json.load(open(CFG)))
part = json.load(open(D + "/partition.json"))
rows = np.load(CACHE + "/rows.npy"); idx = {int(r): i for i, r in enumerate(rows)}
def to_ex(recs):
    return tuple(L.PoseExample(row_index=int(e["row_index"]), observed_pose=e["source_observed_pose"],
                               cue=e["source_cue"], historical_role=e["historical_role"]) for e in recs)
tr_ex, ev_ex = to_ex(part["training_rows"]), to_ex(part["evaluation_rows"])
partition = L.PosePartition(annotation_file_sha256=part["annotation_file_sha256"],
                            split_seed=int(part["split_seed"]), training=tr_ex, evaluation=ev_ex)
tr_rows = [int(e["row_index"]) for e in part["training_rows"]]
ev_rows = [int(e["row_index"]) for e in part["evaluation_rows"]]
tr_lab = torch.tensor([int(e["class_id"]) for e in part["training_rows"]], dtype=torch.int64)
ev_lab = torch.tensor([int(e["class_id"]) for e in part["evaluation_rows"]], dtype=torch.int64)
orders = R._epoch_orders(len(tr_rows), config.epochs)
RUNG = 64
spec = L.CameraPoseRungSpec(rung_size=RUNG)

def state_for(seed):
    m = L.CameraPoseClassifierCandidate(spec)
    if seed is not None:
        torch.manual_seed(seed)
        for mod in m.modules():
            if isinstance(mod, nn.Conv2d): mod.reset_parameters()
    return {k: v.clone() for k, v in m.state_dict().items()}

def run(cam_name, init_state, labels):
    cam = np.load(f"{CACHE}/cam_{cam_name}.npy", mmap_mode="r")
    Xtr = torch.from_numpy(np.stack([cam[idx[r]] for r in tr_rows]))
    Xev = torch.from_numpy(np.stack([cam[idx[r]] for r in ev_rows]))
    _, m, _ = R._train_one_arm(spec=spec, initial_state=init_state,
        training_camera=L.reduce_primary_camera(Xtr, rung_size=RUNG), training_labels=labels,
        evaluation_camera=L.reduce_primary_camera(Xev, rung_size=RUNG), evaluation_labels=ev_lab,
        evaluation_examples=ev_ex, orders=orders, config=config)
    return int(m["correct"])

res = {"rung": RUNG, "n_eval": len(ev_rows), "chance": 116 / 11, "seeds": {}, "null": {}}
SEEDS = [None, 1, 2, 3, 4]
for arm in ("crop", "wide", "full", "perf", "hole"):
    got = [run(arm, state_for(s), tr_lab) for s in SEEDS]
    res["seeds"][arm] = got
    print(f"  {arm:<5} seeds {got}  median={statistics.median(got):.0f} min={min(got)} max={max(got)}", flush=True)
    json.dump(res, open(OUT, "w"), indent=1)

print("\nEmpirical null, published shuffle mechanism with varied shuffle seeds:", flush=True)
for arm in ("crop", "hole"):
    got = []
    for ss in (20260823, 11, 22, 33, 44, 55, 66, 77):
        try: assign = L.shuffled_training_labels(partition, shuffle_seed=ss)
        except Exception: continue
        lab = torch.tensor([assign[r] for r in tr_rows], dtype=torch.int64)
        got.append(run(arm, state_for(None), lab))
    res["null"][arm] = got
    print(f"  {arm:<5} null {got}  median={statistics.median(got):.0f} max={max(got)}", flush=True)
    json.dump(res, open(OUT, "w"), indent=1)
print("written", OUT)
