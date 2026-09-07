#!/usr/bin/env python3
"""Predeclared temporal-separation test: does the pose signal survive when evaluation rows
are maximally distant in time from training rows of their own class?"""
import json, sys, statistics
sys.path.insert(0,"/home/c/Documents/BOSUN/zeebeam-science/src"); sys.path.insert(0,"/home/c/Documents/BOSUN/zeebeam-science/tools")
sys.dont_write_bytecode=True
import numpy as np, torch, torch.nn as nn
from zeebeam_science import camera_pose_resolution_ladder_candidate as L
import run_camera_pose_resolution_ladder as R

D="/home/c/Documents/BOSUN/scratch/zeebeam_camera_pose_ladder_20260823/real_august_seed20260823_e64_v1"
CFG="/home/c/Documents/BOSUN/zeebeam-science/configs/camera_pose_resolution_ladder.candidate.json"
torch.set_num_threads(12)
config=R._validate_config(json.load(open(CFG)))
part=json.load(open(D+"/partition.json"))
meta={int(e["row_index"]): e for e in part["training_rows"]+part["evaluation_rows"]}
allrows=sorted((int(e["row_index"]), int(e["class_id"])) for e in part["training_rows"]+part["evaluation_rows"])
byc={}
for r,c in allrows: byc.setdefault(c,[]).append(r)
rows_np=np.load("cache/rows.npy"); idx={int(r):i for i,r in enumerate(rows_np)}
RUNG=64; spec=L.CameraPoseRungSpec(rung_size=RUNG)

def split(direction):
    tr,ev=[],[]
    for c,rs in byc.items():
        rs=sorted(rs); h=len(rs)//2
        a,b=(rs[:h],rs[h:]) if direction=="early" else (rs[h:],rs[:h])
        tr+=[(r,c) for r in a]; ev+=[(r,c) for r in b]
    return sorted(tr), sorted(ev)

def ex(rc):
    return tuple(L.PoseExample(row_index=r, observed_pose=meta[r]["source_observed_pose"],
                               cue=meta[r]["source_cue"], historical_role=meta[r]["historical_role"]) for r,_ in rc)

def state(seed):
    m=L.CameraPoseClassifierCandidate(spec)
    if seed is not None:
        torch.manual_seed(seed)
        for mod in m.modules():
            if isinstance(mod,nn.Conv2d): mod.reset_parameters()
    return {k:v.clone() for k,v in m.state_dict().items()}

def run(arm, tr, ev, labels_tr, seed):
    cam=np.load(f"cache/cam_{arm}.npy", mmap_mode="r")
    Xtr=torch.from_numpy(np.stack([cam[idx[r]] for r,_ in tr]))
    Xev=torch.from_numpy(np.stack([cam[idx[r]] for r,_ in ev]))
    ytr=labels_tr; yev=torch.tensor([c for _,c in ev])
    orders=R._epoch_orders(len(tr), config.epochs)
    _,m,_=R._train_one_arm(spec=spec, initial_state=state(seed),
        training_camera=L.reduce_primary_camera(Xtr,rung_size=RUNG), training_labels=ytr,
        evaluation_camera=L.reduce_primary_camera(Xev,rung_size=RUNG), evaluation_labels=yev,
        evaluation_examples=ex(ev), orders=orders, config=config)
    return int(m["correct"]), len(ev)

out={"predeclaration":"TEMPORAL_SPLIT_PREDECLARATION.md","chance_fraction":1/11,"results":{}}
for direction in ("early","late"):
    tr,ev=split(direction)
    gaps=[min(abs(r-t) for t,tc in tr if tc==c) for r,c in ev]
    ytr=torch.tensor([c for _,c in tr])
    g=torch.Generator().manual_seed(20260901)
    yshuf=ytr[torch.randperm(len(ytr),generator=g)]
    d={"n_train":len(tr),"n_eval":len(ev),"median_gap_rows":statistics.median(gaps)}
    for arm in ("crop","full","perf","hole"):
        accs=[run(arm,tr,ev,ytr,s)[0] for s in (None,1,2)]
        d[arm]={"correct_per_seed":accs,"n":len(ev),
                "median_frac":statistics.median(accs)/len(ev)}
        print(f"{direction:5} {arm:5} correct {accs} of {len(ev)}  median {statistics.median(accs)/len(ev):.3f}", flush=True)
    sh=run("full",tr,ev,yshuf,None)[0]
    d["full_shuffled_control"]={"correct":sh,"n":len(ev),"frac":sh/len(ev)}
    print(f"{direction:5} SHUFFLED-label control (full): {sh} of {len(ev)} = {sh/len(ev):.3f}", flush=True)
    out["results"][direction]=d
json.dump(out, open("out/temporal_separation.json","w"), indent=1)
print("\nchance =", round(1/11,4))
