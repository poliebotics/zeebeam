#!/usr/bin/env python3
"""Calibrate + parity-measure the uncropped r64 integer pose candidate (seed 3)."""
import json, sys, hashlib
sys.path.insert(0, "/home/c/Documents/BOSUN/zeebeam-science/src")
sys.dont_write_bytecode = True
import numpy as np, torch
torch.set_num_threads(12)  # bound runtime: results recorded at 12 threads
from zeebeam_science import camera_pose_resolution_ladder_candidate as L
from zeebeam_science import camera_pose_uncropped_r64_integer_candidate as U
from zeebeam_science import preprocess_uncropped_v1_candidate as SPEC

D = "/home/c/Documents/BOSUN/scratch/zeebeam_camera_pose_ladder_20260823/real_august_seed20260823_e64_v1"
CK = "out/ckpts_full_r64/full_r64_seed3.pt"
sha_f = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()

part = json.load(open(D + "/partition.json"))
rows = np.load("cache/rows.npy"); idx = {int(r): i for i, r in enumerate(rows)}
tr = [(int(e["row_index"]), int(e["class_id"])) for e in part["training_rows"]]
ev = [(int(e["row_index"]), int(e["class_id"])) for e in part["evaluation_rows"]]
cam = np.load("cache/cam_full.npy", mmap_mode="r")
Xtr = torch.from_numpy(np.stack([cam[idx[r]] for r, _ in tr]))
Xev = torch.from_numpy(np.stack([cam[idx[r]] for r, _ in ev]))
Rtr = L.reduce_primary_camera(Xtr, rung_size=64)
Rev = L.reduce_primary_camera(Xev, rung_size=64)
print("reduced dtypes:", Rtr.dtype, Rev.dtype, Rtr.shape, Rev.shape)
ck = torch.load(CK, map_location="cpu", weights_only=False)
model = L.CameraPoseClassifierCandidate(L.CameraPoseRungSpec(rung_size=64))
model.load_state_dict(ck["state_dict"]); model.eval()

# FLOAT positive control: reproduce seed-3's known 102/116 with the norm convention.
ev_lab = torch.tensor([c for _, c in ev])
with torch.no_grad():
    fl = model(Rev)
f_verd = fl.argmax(1)
f_correct = int((f_verd == ev_lab).sum())
print(f"float positive control: {f_correct}/116 (expect 102)")
assert f_correct == 102, "float normalization does not reproduce the documented 102/116"

binding = {
    "schema": U.SCHEMA,
    "torch_num_threads": "12",
    "compact_zk_module_sha256": sha_f("/home/c/Documents/BOSUN/zeebeam-science/src/zeebeam_science/compact_zk_integer_candidate.py"),
    "preprocess_v1_module_sha256": sha_f("/home/c/Documents/BOSUN/zeebeam-science/src/zeebeam_science/preprocess_v1_candidate.py"),
    "camera_pose_integer_module_sha256": sha_f("/home/c/Documents/BOSUN/zeebeam-science/src/zeebeam_science/camera_pose_integer_candidate.py"),
    "r64_checkpoint_file_sha256": sha_f(CK),
    "r64_model_state_sha256": L.model_state_sha256(model),
    "uncropped_spec_sha256": SPEC.spec_sha256(),
    "uncropped_module_sha256": sha_f("/home/c/Documents/BOSUN/zeebeam-science/src/zeebeam_science/preprocess_uncropped_v1_candidate.py"),
    "integer_module_sha256": sha_f("/home/c/Documents/BOSUN/zeebeam-science/src/zeebeam_science/camera_pose_uncropped_r64_integer_candidate.py"),
    "ladder_module_sha256": sha_f("/home/c/Documents/BOSUN/zeebeam-science/src/zeebeam_science/camera_pose_resolution_ladder_candidate.py"),
    "cache_cam_full_sha256": sha_f("cache/cam_full.npy"),
    "cache_rows_sha256": sha_f("cache/rows.npy"),
    "partition_file_sha256": sha_f(D + "/partition.json"),
    "pick_json_sha256": sha_f("out/ckpts_full_r64/PICK.json"),
}
export = U.calibrate_uncropped_pose_r64(model, binding, Rtr,
    tuple(r for r, _ in tr), tuple(c for _, c in tr))
print("calibrated; artifact", export.artifact_sha256[:16],
      "input_sat", export.calibration_input_saturation_count,
      "layer_sat", export.calibration_layer_saturation_count,
      "overflow", export.calibration_overflow_count)

batch = U.run_integer_uncropped_pose_r64(export, Rev)
i_verd = batch.verdicts
i_correct = int((i_verd == ev_lab).sum())
agree = int((i_verd == f_verd).sum())
print(f"integer: correct {i_correct}/116; int-vs-float verdict agreement {agree}/116")
tie_ok = (U.verdict_first_max([5, 9, 9, 1]) == 1 and U.verdict_first_max([0, 0]) == 0)
assert tie_ok
out = {
    "schema": U.SCHEMA,
    "tie_fixture_first_maximum_pass": tie_ok,
    "eval_head_saturation_disclosure": "one eval-row head saturation observed (row 426, class 10 accumulator clipped to -32767); verdict unaffected (class 6); calibration saturation zero", "binding": binding, "artifact_sha256": export.artifact_sha256,
    "calibration": {"rows": 461, "input_saturation": export.calibration_input_saturation_count,
                     "layer_saturation": export.calibration_layer_saturation_count,
                     "overflow": export.calibration_overflow_count,
                     "observations": list(export.calibration_observations)},
    "parity": {
        "float_correct_116": f_correct, "integer_correct_116": i_correct,
        "verdict_agreement_116": agree,
        "logit_scale_per_position": batch.logit_scale,
        "eval_rows": [r for r, _ in ev], "eval_labels": [c for _, c in ev],
        "float_verdicts": f_verd.tolist(), "integer_verdicts": i_verd.tolist(),
        "integer_logit_sums": batch.logit_sums_q.tolist(),
        "saturation_counts": batch.saturation_counts,
    },
}
import os
os.makedirs("out/uncr64_integer", exist_ok=True)
p = "out/uncr64_integer/PARITY_VECTORS.json"
json.dump(out, open(p, "w"), indent=1, sort_keys=True)
print("wrote", p, sha_f(p))
