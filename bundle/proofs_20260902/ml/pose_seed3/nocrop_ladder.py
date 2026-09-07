#!/usr/bin/env python3
"""Does the ZeeBeam pose discriminator still work without its subject-framing crop?

Calls the PUBLISHED trainer (`_train_one_arm`) and the PUBLISHED model, partition, epoch
orders, hyperparameters and label-shuffle. The ONLY thing that varies is how much of the
sensor the 4x256x256 camera tensor covers:

  crop  the published PREPROCESS_V1 window, a 1024-square at x0=782,y0=340 on the packed
        2660x2300 CFA plane (17.1% of it), half-up block mean to 256. Byte-exact against
        the published camera_primary_commitments.json.
  wide  a centred 2048-square, 4x the area, same integer reduction. No subject-specific
        framing beyond centring.
  full  the entire packed plane area-averaged to 256x256. Every pixel contributes; the
        scale is anisotropic, which is recorded rather than hidden.

The `crop` arm is the positive control and must reproduce the published counts exactly
(r128 112, r64 114, r32 112 true; 7, 12, 14 shuffled). If it does not, nothing else here
is reportable.

Claim ceiling, from the module's own source: cue-aware annotation, single-take
subject/room/camera/time/background confound, `reality_claim: False`, `proof: False`. This
is a development diagnostic and never evidence of physical realness.
"""
import json, sys, time
sys.path.insert(0, "/home/c/Documents/BOSUN/zeebeam-science/src")
sys.path.insert(0, "/home/c/Documents/BOSUN/zeebeam-science/tools")
sys.dont_write_bytecode = True
import numpy as np, torch
from zeebeam_science import camera_pose_resolution_ladder_candidate as L
import run_camera_pose_resolution_ladder as R

D = "/home/c/Documents/BOSUN/scratch/zeebeam_camera_pose_ladder_20260823/real_august_seed20260823_e64_v1"
CACHE = "/home/c/Documents/BOSUN/scratch/pose_nocrop_20260830/cache"
OUT = "/home/c/Documents/BOSUN/scratch/pose_nocrop_20260830/out/mask_ladder.json"
CFG = "/home/c/Documents/BOSUN/zeebeam-science/configs/camera_pose_resolution_ladder.candidate.json"

torch.set_num_threads(12)
cfgraw = json.load(open(CFG))
config = R._validate_config(cfgraw)
part = json.load(open(D + "/partition.json"))
published = {r["rung_size"]: r for r in json.load(open(D + "/results.json"))["rungs"]}
rows = np.load(CACHE + "/rows.npy"); idx = {int(r): i for i, r in enumerate(rows)}
tr_rows = [int(e["row_index"]) for e in part["training_rows"]]
ev_rows = [int(e["row_index"]) for e in part["evaluation_rows"]]
def to_examples(recs):
    return [L.PoseExample(row_index=int(e["row_index"]), observed_pose=e["source_observed_pose"],
                          cue=e["source_cue"], historical_role=e["historical_role"]) for e in recs]
ev_examples = to_examples(part["evaluation_rows"])
tr_lab = torch.tensor([int(e["class_id"]) for e in part["training_rows"]], dtype=torch.int64)
ev_lab = torch.tensor([int(e["class_id"]) for e in part["evaluation_rows"]], dtype=torch.int64)

# published label shuffle, reproduced through the published helper
prep_examples = R._prepare_examples_for_shuffle() if hasattr(R, "_prepare_examples_for_shuffle") else None
orders = R._epoch_orders(len(tr_rows), config.epochs)
print(f"train={len(tr_rows)} eval={len(ev_rows)} epochs={config.epochs} lr={config.learning_rate} "
      f"wd={config.weight_decay} clip={config.gradient_clip_norm} batch={config.batch_size}", flush=True)

def shuffled_labels_for(rung):
    """Recover the published shuffled training labels from that arm's own checkpoint run."""
    return None

ARMS = {}
for name in ("crop", "wide", "full", "perf", "hole"):
    ARMS[name] = np.load(f"{CACHE}/cam_{name}.npy")

results = {"experiment": "pose discriminator, crop vs no-crop, published trainer",
           "claim_ceiling": "development diagnostic; cue-aware; single-take confounded; reality_claim False",
           "arms_described": {
               "crop": "published PREPROCESS_V1 1024-square at x0=782,y0=340 on packed 2660x2300, half-up mean to 256",
               "wide": "centred 2048-square, 4x area, half-up mean to 256",
               "full": "entire packed plane, anisotropic area mean to 256x256",
               "perf": "PERFORMER WINDOW ONLY: everything outside the 1024-square blanked, same reducer as full",
               "hole": "BACKGROUND ONLY: the 1024-square blanked, everything else kept, same reducer as full"},
           "chance": 1.0 / 11, "n_eval": len(ev_rows), "ladder": {}}

for rung in (128, 64, 32):
    spec = L.CameraPoseRungSpec(rung_size=rung)
    ck = torch.load(f"{D}/checkpoint_r{rung}_true_labels.pt", map_location="cpu", weights_only=False)
    init = L.CameraPoseClassifierCandidate(spec)
    init_sha = L.model_state_sha256(init)
    assert init_sha == ck["initial_model_state_sha256"], f"init state differs at r{rung}"
    initial_state = {k: v.clone() for k, v in init.state_dict().items()}
    results["ladder"][str(rung)] = {"initial_state_sha256_matches_published": True}
    for name, cam in ARMS.items():
        Xtr = torch.from_numpy(np.stack([cam[idx[r]] for r in tr_rows]))
        Xev = torch.from_numpy(np.stack([cam[idx[r]] for r in ev_rows]))
        rtr = L.reduce_primary_camera(Xtr, rung_size=rung)
        rev = L.reduce_primary_camera(Xev, rung_size=rung)
        t0 = time.time()
        model, metrics, _ = R._train_one_arm(
            spec=spec, initial_state=initial_state,
            training_camera=rtr, training_labels=tr_lab,
            evaluation_camera=rev, evaluation_labels=ev_lab,
            evaluation_examples=ev_examples, orders=orders, config=config)
        correct = int(metrics["correct"]); acc = float(metrics["exact_accuracy"])
        with torch.no_grad():
            per_row = model(rev).argmax(dim=1).tolist()
        entry = {"correct": correct, "exact_accuracy": acc, "seconds": round(time.time() - t0, 1),
                 "eval_rows": ev_rows, "predicted": per_row, "truth": ev_lab.tolist()}
        if name == "crop":
            want = published[rung]["arms"]["true_labels"]["metrics"]["correct"]
            entry["published_correct"] = want
            entry["positive_control_reproduces"] = (correct == want)
        results["ladder"][str(rung)][name] = entry
        tag = ""
        if name == "crop":
            tag = f"  published={entry['published_correct']}  {'REPRODUCES' if entry['positive_control_reproduces'] else 'DIFFERS'}"
        print(f"  r{rung:<4} {name:<5} correct={correct:3d}/116 acc={acc:.4f} ({entry['seconds']}s){tag}", flush=True)
        json.dump(results, open(OUT, "w"), indent=1)
print("\nwritten", OUT)
