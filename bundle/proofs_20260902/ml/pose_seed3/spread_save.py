#!/usr/bin/env python3
"""Rerun the documented full-arm r64 seed spread, SAVING each trained state_dict.

Positive control: the five eval corrects must equal robustness.json's full entry
[102, 93, 104, 102, 92] exactly (same seeds, same seeded epoch orders, same cache), or this
run refuses to save anything. Pick rule (recorded 2026-08-31 ~07:50 after the spread outcomes existed — a post-outcome representative choice made before PTQ, not preregistration — applied
here): the seed achieving the median accuracy, ties broken by lowest seed value, with
seed None mapped to its actual contract value 20260823 (the module's INITIALIZATION_SEED).
"""
import json, sys, hashlib, statistics
sys.path.insert(0, "/home/c/Documents/BOSUN/zeebeam-science/src")
sys.path.insert(0, "/home/c/Documents/BOSUN/zeebeam-science/tools")
sys.dont_write_bytecode = True
import numpy as np, torch, torch.nn as nn
from zeebeam_science import camera_pose_resolution_ladder_candidate as L
import run_camera_pose_resolution_ladder as R

D = "/home/c/Documents/BOSUN/scratch/zeebeam_camera_pose_ladder_20260823/real_august_seed20260823_e64_v1"
CACHE = "cache"
CFG = "/home/c/Documents/BOSUN/zeebeam-science/configs/camera_pose_resolution_ladder.candidate.json"
EXPECT = [102, 93, 104, 102, 92]
torch.set_num_threads(12)
config = R._validate_config(json.load(open(CFG)))
part = json.load(open(D + "/partition.json"))
rows = np.load(CACHE + "/rows.npy"); idx = {int(r): i for i, r in enumerate(rows)}
tr_rows = [int(e["row_index"]) for e in part["training_rows"]]
ev_rows = [int(e["row_index"]) for e in part["evaluation_rows"]]
tr_lab = torch.tensor([int(e["class_id"]) for e in part["training_rows"]], dtype=torch.int64)
ev_lab = torch.tensor([int(e["class_id"]) for e in part["evaluation_rows"]], dtype=torch.int64)
ev_ex = tuple(L.PoseExample(row_index=int(e["row_index"]), observed_pose=e["source_observed_pose"],
                            cue=e["source_cue"], historical_role=e["historical_role"]) for e in part["evaluation_rows"])
orders = R._epoch_orders(len(tr_rows), config.epochs)
RUNG = 64
spec = L.CameraPoseRungSpec(rung_size=RUNG)
cam = np.load(f"{CACHE}/cam_full.npy", mmap_mode="r")
Xtr = torch.from_numpy(np.stack([cam[idx[r]] for r in tr_rows]))
Xev = torch.from_numpy(np.stack([cam[idx[r]] for r in ev_rows]))
Rtr = L.reduce_primary_camera(Xtr, rung_size=RUNG)
Rev = L.reduce_primary_camera(Xev, rung_size=RUNG)

def state_for(seed):
    m = L.CameraPoseClassifierCandidate(spec)
    if seed is not None:
        torch.manual_seed(seed)
        for mod in m.modules():
            if isinstance(mod, nn.Conv2d): mod.reset_parameters()
    return {k: v.clone() for k, v in m.state_dict().items()}

SEEDS = [None, 1, 2, 3, 4]
results, states = [], []
for s in SEEDS:
    ret = R._train_one_arm(spec=spec, initial_state=state_for(s),
        training_camera=Rtr, training_labels=tr_lab,
        evaluation_camera=Rev, evaluation_labels=ev_lab,
        evaluation_examples=ev_ex, orders=orders, config=config)
    first, metrics = ret[0], ret[1]
    correct = int(metrics["correct"])
    if hasattr(first, "state_dict"): sd = first.state_dict()
    elif isinstance(first, dict): sd = first
    else: raise SystemExit(f"unexpected first return {type(first)}")
    results.append(correct); states.append({k: v.clone() for k, v in sd.items()})
    print(f"seed {s}: correct={correct}", flush=True)
assert results == EXPECT, f"POSITIVE CONTROL FAIL: {results} != {EXPECT}; NOTHING SAVED"
import os
os.makedirs("out/ckpts_full_r64", exist_ok=True)
rec = {"seeds": {}, "expect": EXPECT, "pick_rule": "median accuracy, ties -> lowest seed value, None==20260823",
       "spec_sha_preprocess_uncropped": "a3985f693e1c017037fda13e7b6d15f8896c4d195e02a6298f198552560e8e08"}
for s, c, sd in zip(SEEDS, results, states):
    name = "seed20260823_contract" if s is None else f"seed{s}"
    fp = f"out/ckpts_full_r64/full_r64_{name}.pt"
    torch.save({"state_dict": sd, "seed": (20260823 if s is None else s), "correct": c,
                "arm": "full", "rung": 64, "cache": "cam_full.npy"}, fp)
    rec["seeds"][name] = {"correct": c, "file": fp, "sha256": hashlib.sha256(open(fp, "rb").read()).hexdigest()}
med = statistics.median(results)
cands = [(20260823 if s is None else s, s) for s, c in zip(SEEDS, results) if c == med]
pick = min(cands)[1]
rec["median"] = med
rec["picked_seed"] = 20260823 if pick is None else pick
rec["picked_file"] = rec["seeds"]["seed20260823_contract" if pick is None else f"seed{pick}"]["file"]
json.dump(rec, open("out/ckpts_full_r64/PICK.json", "w"), indent=1, sort_keys=True)
print("PICK:", rec["picked_seed"], rec["picked_file"])
