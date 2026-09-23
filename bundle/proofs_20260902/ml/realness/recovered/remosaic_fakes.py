#!/usr/bin/env python3
"""Re-mosaic packed CFA fakes (4,2300,2660) back to raw Bayer (4600,5320) — exact inverse
of the RGGB pack, lossless — into shadow session dirs so the frozen ARM-A loader consumes
fakes through its own unchanged code path."""
import json, hashlib, pathlib, sys, time
import numpy as np
sys.dont_write_bytecode = True
BASE = pathlib.Path("/lambda/nfs/ZeeBeam/experiments/fa_fake_table_20260831")
SRC = BASE / "d2v10_fakes"
DST = BASE / "fake_sessions"
man = [json.loads(l) for l in open(SRC / "MANIFEST_d2v10.jsonl")]
t0 = time.time(); n = 0
out_man = open(DST / "REMOSAIC_MANIFEST.jsonl", "a") if (DST.exists() or DST.mkdir(parents=True) or True) else None
for rec in man:
    sid, row = rec["session"], rec["row"]
    dst = DST / sid / "Recordings" / f"frame_{row:06d}.raw"
    dst.parent.mkdir(parents=True, exist_ok=True)
    if dst.exists() and dst.stat().st_size == 4600 * 5320:
        n += 1; continue
    cfa = np.frombuffer((SRC / f"fake_{sid}_row{row:06d}_step_00100000.u8cfa").read_bytes(),
                        dtype=np.uint8).reshape(4, 2300, 2660)
    raw = np.zeros((4600, 5320), dtype=np.uint8)
    raw[0::2, 0::2] = cfa[0]; raw[0::2, 1::2] = cfa[1]
    raw[1::2, 0::2] = cfa[2]; raw[1::2, 1::2] = cfa[3]
    # round-trip check: re-pack must equal the source cfa bit-exactly
    rp = np.stack([raw[0::2, 0::2], raw[0::2, 1::2], raw[1::2, 0::2], raw[1::2, 1::2]], 0)
    assert rp.tobytes() == cfa.tobytes(), f"round-trip fail {sid} {row}"
    tmp = dst.with_suffix(".tmp")
    tmp.write_bytes(raw.tobytes(order="C"))
    tmp.replace(dst)
    out_man.write(json.dumps({"session": sid, "row": row,
        "raw_sha256": hashlib.sha256(raw.tobytes()).hexdigest(),
        "src_fake_sha256": rec["fake_bytes_sha256"]}) + "\n")
    out_man.flush(); n += 1
    if n % 200 == 0: print(f"{n}/{len(man)} {(time.time()-t0)/60:.1f}m", flush=True)
print(f"REMOSAIC COMPLETE {n}/{len(man)}", flush=True)
