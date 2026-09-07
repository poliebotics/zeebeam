#!/usr/bin/env python3
"""Independent Python oracle for one ZeeBeam row, built ONLY from the audited zeebeam-science
modules (not from the circuit): PREPROCESS_V1 output digest, primary-pair digest, typed tile root
(context + root), the uncropped 256 reduction and its commitment. Row 96's frozen constants are the
positive control and must reproduce before any other row's values are trusted.

usage: .venv/bin/python row_oracle.py ROW  (run from the zeebeam-science directory)
"""
import hashlib, json, sys, io
from pathlib import Path
import numpy as np
import blake3
from PIL import Image

sys.path.insert(0, str(Path(__file__).resolve().parents[0]))
from zeebeam_science import preprocess_v1_candidate as pp
from zeebeam_science import preprocess_uncropped_v1_candidate as unc
from zeebeam_science import typed_tile_root_candidate as ttr
from zeebeam_science.row_binding_join_candidate import _primary_pair_sha256

STAGE = Path("/data/zeebeam_evidence_20260822/stage/live_300s_training_001")
FROZEN_96 = {
    "emission_blake3": "5e4a0bc9f4790b843d91185736b916042f5cac794a5d8c31662a2cd601a4bfa8",
    "raw_blake3": "c6535a541172deb06d2f8f208a63c760e8c9340e5ec9dea8d70679d4ec12badd",
    "preprocess_output_sha256": "da6238d581671f9886cd9468a47deb825c3fc5296e1f833d5d8e00b23a37bbb7",
    "primary_pair_sha256": "7d894eedf53567473cb80448826de0016c5869b07bd674bbba1503ff6dc50e37",
    "typed_context": "9d99a1f294eb4e7d1d8b87e21498cf1b598354a9f79b130a227fe6621d794789",
    "typed_root": "8ee3eefa1027cc02d955b65f06840fe624ce07077f8d52f7e9e9f4eb309043b2",
    "uncropped_commitment": "59910376038eab3c9bbee8a31d5bfa4457e3926b93ae9658abfeca1159184db7",
}
FROZEN_52_UNCROPPED = "fa5f5e09d2ef05e5"  # prefix recorded in the state doc


def oracle(row: int) -> dict:
    raw = (STAGE / "Recordings" / f"frame_{row:06d}.raw").read_bytes()
    png = (STAGE / "derived" / "Emissions" / f"tile_{row:06d}.png").read_bytes()
    img = Image.open(io.BytesIO(png)).convert("RGB")
    assert img.size == (1920, 1080), img.size
    emission_rgb = np.asarray(img, dtype=np.uint8).tobytes(order="C")
    camera, emission = pp.preprocess_candidate(raw, emission_rgb, output_size=256)
    out_bytes = pp.output_bytes(camera, emission)
    bindings = ttr.TypedTileBindings()
    cam_t = ttr.TypedPreprocessTensor.from_values(ttr.CAMERA, camera, bindings)
    emi_t = ttr.TypedPreprocessTensor.from_values(ttr.EMISSION, emission, bindings)
    commit = ttr.commit_typed_tile_root(cam_t, emi_t, bindings)
    camera256 = unc.preprocess_uncropped(raw)
    return {
        "row": row,
        "raw_blake3": blake3.blake3(raw).hexdigest(),
        "emission_png_sha256": hashlib.sha256(png).hexdigest(),
        "emission_blake3": blake3.blake3(emission_rgb).hexdigest(),
        "preprocess_spec_sha256": pp.DEFAULT_SPEC.sha256(),
        "preprocess_output_bytes": len(out_bytes),
        "preprocess_output_sha256": hashlib.sha256(out_bytes).hexdigest(),
        "primary_pair_sha256": _primary_pair_sha256(row, camera, emission),
        "typed_context": commit.context_digest.hex(),
        "typed_root": commit.root.hex(),
        "uncropped_spec_sha256": unc.spec_sha256(),
        "uncropped_commitment": unc.commitment(row, camera256),
        "uncropped_camera_sha256": hashlib.sha256(camera256.tobytes(order="C")).hexdigest(),
    }


def main() -> int:
    row = int(sys.argv[1])
    ctrl = oracle(96)
    ok = {k: ctrl[k] == v for k, v in FROZEN_96.items()}
    print("positive control row 96:", ok)
    if not all(ok.values()):
        print("CONTROL FAILED; not emitting anything else")
        return 1
    if row != 96:
        res = oracle(row)
    else:
        res = ctrl
    outp = Path("/home/c/Documents/BOSUN/scratch/joined_build_20260901") / f"row_{row:03d}_python_oracle.json"
    json.dump(res, open(outp, "w"), indent=1)
    print(json.dumps(res, indent=1))
    print("wrote", outp)
    return 0


if __name__ == "__main__":
    sys.exit(main())
