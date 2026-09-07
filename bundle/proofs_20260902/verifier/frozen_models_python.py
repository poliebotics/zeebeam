#!/usr/bin/env python3
"""Run both frozen integer models in Python (torch) straight from their frozen blobs and manifests,
using the audited zeebeam-science integer kernels, to give an implementation independent of the Rust
guest for the coupling score numerator and the pose head saturation.

Positive controls, all of which must pass before any other row's value is reported:
  - r32: all 72 matched and 72 crossed numerators of the audited parity vector file, and row 96's
    numerator 56,835,791 from the primaries;
  - pose: row 96's frozen logit sums [1890, 1486, 5287, -733, -11950, -3849, -17123, -3722, -6251,
    1282, -17888], verdict 2, head saturation 0.

usage (from the zeebeam-science directory, venv python, PYTHONPATH=src):
  frozen_models_python.py ROW [ROW ...]
"""
import dataclasses, hashlib, io, json, struct, sys
from pathlib import Path
from types import SimpleNamespace

import numpy as np
import torch
from PIL import Image

from zeebeam_science import preprocess_v1_candidate as pp
from zeebeam_science import preprocess_uncropped_v1_candidate as unc
from zeebeam_science.compact_zk_integer_candidate import IntegerInputQuantizer, QuantizedConvLayer
from zeebeam_science.compact_zk_resolution_integer_candidate import score_reduced_r32
from zeebeam_science.compact_zk_resolution_ladder_candidate import reduce_primary_pair
from zeebeam_science.camera_pose_resolution_ladder_candidate import reduce_primary_camera
from zeebeam_science.camera_pose_uncropped_r64_integer_candidate import run_integer_uncropped_pose_r64

STAGE = Path("/data/zeebeam_evidence_20260822/stage/live_300s_training_001")
R32_DIR = Path("/home/c/Documents/BOSUN/scratch/zeebeam_lambda_launch_prep_20260823/zeebeam-trained-r32-ptq-v2-sp1-v1")
R32_BLOB = R32_DIR / "frozen/r32_fit_calibrated_ptq_v2.bin"
R32_MANIFEST = Path("/home/c/Documents/BOSUN/scratch/zeebeam_lambda_artifacts_20260824/experiments/adaptive_compact_integer_forger_v1/source/frozen/r32_fit_calibrated_ptq_v2_manifest.json")
R32_PARITY = R32_DIR / "fixtures/architecture_r32_parity_v2.bin"
POSE_DIR = Path("/home/c/Documents/BOSUN/scratch/joined_build_20260901/BOSUN/scratch/uncr64/uncr64_pose_sp1_20260831")
POSE_BLOB = POSE_DIR / "frozen/uncr64_pose_ptq.bin"
POSE_MANIFEST = POSE_DIR / "frozen/uncr64_pose_ptq_manifest.json"
OUT = Path("/home/c/Documents/BOSUN/scratch/joined_build_20260901")

FROZEN_96_POSE = ([1890, 1486, 5287, -733, -11950, -3849, -17123, -3722, -6251, 1282, -17888], 2, 0)
FROZEN_96_SCORE = 56835791


def sha(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()


def load_layers(blob: bytes, manifest: dict, layer_objects: list) -> list:
    """Rebuild QuantizedConvLayer values from the blob segments and the manifest's canonical objects."""
    assert sha(blob) == manifest["blob"]["sha256"], "blob digest differs from its manifest"
    segs = {(s["layer"], s["kind"]): s for s in manifest["blob"]["segments"]}
    fields = {f.name for f in dataclasses.fields(QuantizedConvLayer)}
    layers = []
    for obj in layer_objects:
        name = obj["name"]
        def tensor(kind, dtype, fallback_shape=None):
            if (name, kind) not in segs and fallback_shape is not None:
                # a head whose output is the raw accumulator carries no requant segment in the
                # blob; the manifest's integrity hash still pins what the tensor must be
                return torch.zeros(fallback_shape, dtype=torch.int64)
            s = segs[(name, kind)]
            raw = blob[s["offset"]:s["offset"] + s["bytes"]]
            assert sha(raw) == s["sha256"], f"{name} {kind} segment digest differs"
            arr = np.frombuffer(raw, dtype=dtype).reshape(s["shape"]).copy()
            return torch.from_numpy(arr)
        kwargs = {k: obj[k] for k in obj if k in fields}
        for k in ("stride", "padding"):
            kwargs[k] = tuple(obj[k])
        for k in ("weight_scale_hex", "accumulator_scale_hex", "accumulator_abs_bounds",
                  "requant_product_abs_bounds", "requantized_abs_bounds"):
            kwargs[k] = tuple(obj[k])
        kwargs["weight_q"] = tensor("weight", np.int8)
        kwargs["bias_q"] = tensor("bias", "<i8")
        kwargs["requant_multipliers"] = tensor("requant_multiplier", "<i8", fallback_shape=(kwargs["bias_q"].shape[0],))
        layer = QuantizedConvLayer(**kwargs)
        layer.validate_integrity()  # recomputes the integrity object; proves the reconstruction is exact
        layers.append(layer)
    return layers


def load_quantizer(obj: dict) -> IntegerInputQuantizer:
    fields = {f.name for f in dataclasses.fields(IntegerInputQuantizer)}
    q = IntegerInputQuantizer(**{k: obj[k] for k in obj if k in fields})
    q.validate()
    return q


def export_like(quantizer, blocks, head, head_attr: str, extra: dict):
    def validate_integrity():
        quantizer.validate()
        for l in (*blocks, head):
            l.validate_integrity()
    ns = SimpleNamespace(input_quantizer=quantizer, blocks=tuple(blocks), validate_integrity=validate_integrity, **extra)
    setattr(ns, head_attr, head)
    return ns


def r32_export():
    m = json.load(open(R32_MANIFEST)); blob = R32_BLOB.read_bytes()
    layers = load_layers(blob, m, [*m["blocks"], m["logit_head"]])
    head = layers[-1]
    logit_scale = float.fromhex(head.accumulator_scale_hex[0])
    return export_like(load_quantizer(m["input_quantizer"]), layers[:-1], head, "logit_head",
                       {"score_abs_bound": m["score_abs_bound"], "logit_scale": logit_scale, "patch_side": m["patch_side"]})


def pose_export():
    m = json.load(open(POSE_MANIFEST)); blob = POSE_BLOB.read_bytes()
    layers = load_layers(blob, m, m["layers"])
    return export_like(load_quantizer(m["input_quantizer"]), layers[:-1], layers[-1], "class_head", {})


def parity_control(exp) -> None:
    b = R32_PARITY.read_bytes()
    assert b[:8] == bytes.fromhex("5a42504152563200"), "parity magic"
    rows = struct.unpack_from("<I", b, 8)[0]; o = 12
    cam = np.frombuffer(b[o:o + rows * 4 * 32 * 32], dtype=np.uint8).reshape(rows, 4, 32, 32); o += rows * 4 * 32 * 32
    emi = np.frombuffer(b[o:o + rows * 3 * 32 * 32], dtype=np.uint8).reshape(rows, 3, 32, 32); o += rows * 3 * 32 * 32
    donors = np.frombuffer(b[o:o + rows * 4], dtype="<u4"); o += rows * 4
    matched = np.frombuffer(b[o:o + rows * 8], dtype="<i8"); o += rows * 8
    crossed = np.frombuffer(b[o:o + rows * 8], dtype="<i8"); o += rows * 8
    pcam = np.frombuffer(b[o:o + 4 * 256 * 256], dtype=np.uint8).reshape(1, 4, 256, 256); o += 4 * 256 * 256
    pemi = np.frombuffer(b[o:o + 3 * 256 * 256], dtype=np.uint8).reshape(1, 3, 256, 256); o += 3 * 256 * 256
    assert o == len(b), (o, len(b))
    ct, et = torch.from_numpy(cam.copy()), torch.from_numpy(emi.copy())
    got_m = score_reduced_r32(exp, ct, et).numerators.numpy()
    got_c = score_reduced_r32(exp, ct, et[donors.astype(np.int64)]).numerators.numpy()
    assert (got_m == matched).all(), "matched parity differs"
    assert (got_c == crossed).all(), "crossed parity differs"
    rc, re_ = reduce_primary_pair(torch.from_numpy(pcam.copy()), torch.from_numpy(pemi.copy()), rung_size=32)
    prim = int(score_reduced_r32(exp, rc, re_).numerators[0])
    assert prim == int(matched[0]) == FROZEN_96_SCORE, (prim, int(matched[0]))
    print(f"r32 parity control: {rows} matched + {rows} crossed numerators reproduced; primary row-96 numerator {prim}")


def row_inputs(row: int):
    raw = (STAGE / "Recordings" / f"frame_{row:06d}.raw").read_bytes()
    png = (STAGE / "derived" / "Emissions" / f"tile_{row:06d}.png").read_bytes()
    img = Image.open(io.BytesIO(png)).convert("RGB")
    emission_rgb = np.asarray(img, dtype=np.uint8).tobytes(order="C")
    camera, emission = pp.preprocess_candidate(raw, emission_rgb, output_size=256)
    camera256 = unc.preprocess_uncropped(raw)
    return camera, emission, camera256


def run_row(row: int, r32, pose) -> dict:
    camera, emission, camera256 = row_inputs(row)
    rc, re_ = reduce_primary_pair(torch.from_numpy(np.ascontiguousarray(camera))[None], torch.from_numpy(np.ascontiguousarray(emission))[None], rung_size=32)
    score = score_reduced_r32(r32, rc, re_)
    cam64 = reduce_primary_camera(torch.from_numpy(np.ascontiguousarray(camera256))[None], rung_size=64)
    out = run_integer_uncropped_pose_r64(pose, cam64)
    return {
        "row": row,
        "coupling_score_numerator": int(score.numerators[0]),
        "coupling_score_denominator": 4,
        "coupling_saturation_counts": score.saturation_counts,
        "pose_logit_sums": [int(v) for v in out.logit_sums_q[0].tolist()],
        "pose_verdict": int(out.verdicts[0]),
        "pose_head_saturation": int(out.saturation_counts[pose.class_head.name]),
        "pose_saturation_counts": out.saturation_counts,
    }


def main() -> int:
    r32, pose = r32_export(), pose_export()
    print("layers reconstructed from blobs; every integrity hash re-validated")
    parity_control(r32)
    ctrl = run_row(96, r32, pose)
    ok = (ctrl["pose_logit_sums"], ctrl["pose_verdict"], ctrl["pose_head_saturation"]) == FROZEN_96_POSE and ctrl["coupling_score_numerator"] == FROZEN_96_SCORE
    print("row 96 control:", "PASS" if ok else "FAIL", json.dumps({k: ctrl[k] for k in ("coupling_score_numerator", "pose_logit_sums", "pose_verdict", "pose_head_saturation")}))
    if not ok:
        return 1
    results = {96: ctrl}
    for a in sys.argv[1:]:
        r = int(a)
        results[r] = ctrl if r == 96 else run_row(r, r32, pose)
        print(f"row {r}:", json.dumps({k: results[r][k] for k in ("coupling_score_numerator", "pose_logit_sums", "pose_verdict", "pose_head_saturation")}))
    (OUT / "frozen_models_python_results.json").write_text(json.dumps(results, indent=1) + "\n")
    print("wrote", OUT / "frozen_models_python_results.json")
    return 0


if __name__ == "__main__":
    sys.exit(main())
