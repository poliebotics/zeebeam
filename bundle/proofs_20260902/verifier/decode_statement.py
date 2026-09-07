#!/usr/bin/env python3
"""Decode a ZeeBeam one-proof public statement (1,085, 1,093 or 1,101 bytes) into named fields.

Portable: no absolute paths, writes nothing unless --out is given, exit status 0 on success.
Layout read from the Rust sources (membership/src/lib.rs, join/src/lib.rs, the r32 relation,
august.rs, zcash.rs), not recalled.

usage: decode_statement.py [--out DIR] <public_values.hex or .bin> [more...]
       prints one JSON document per input to stdout; with --out DIR also writes
       DIR/<input-stem>_statement.json (the stem drops a trailing "_public_values" or
       "_groth16_public_values").
"""
import argparse, hashlib, json, struct, sys
from pathlib import Path

R32_DIGEST_NAMES = ["blob_sha256", "blob_manifest_sha256", "model_spec_sha256", "checkpoint_sha256",
                    "final_model_state_sha256", "ptq_source_state_sha256", "integer_artifact_sha256",
                    "parity_report_sha256"]


def hx(b): return b.hex()


def decode(pv: bytes) -> dict:
    if len(pv) not in (1085, 1093, 1101):
        raise ValueError(f"unsupported statement length {len(pv)}; expected 1085, 1093 or 1101")
    d = {"total_bytes": len(pv), "sha256": hashlib.sha256(pv).hexdigest()}
    m = d["membership_prefix"] = {}
    m["magic"] = pv[0:8].decode(); m["abi_version"] = pv[8]; m["protocol_code"] = pv[9]
    m["terminal_committed"] = pv[10]; m["reserved"] = pv[11]
    m["public_bytes_declared"] = struct.unpack_from("<I", pv, 12)[0]
    m["row_index"] = struct.unpack_from("<I", pv, 16)[0]
    m["row_count"] = struct.unpack_from("<I", pv, 20)[0]
    m["tree_depth"] = struct.unpack_from("<H", pv, 24)[0]
    sid_len = struct.unpack_from("<H", pv, 26)[0]
    m["session_id"] = pv[28:28 + sid_len].decode()
    m["s_0"] = hx(pv[156:188]); m["s_n"] = hx(pv[188:220])
    m["authority_manifest_sha256"] = hx(pv[220:252]); m["chain_log_blake3"] = hx(pv[252:284])
    m["context_digest_blake3"] = hx(pv[284:316]); m["ordered_session_root_blake3"] = hx(pv[316:348])
    c = d["coupling"] = {}
    sc = pv[352:696]
    c["header"] = list(sc[0:8])
    c["score_numerator"] = struct.unpack_from("<q", sc, 8)[0]
    c["score_denominator"] = struct.unpack_from("<Q", sc, 16)[0]
    c["typed_context_blake3"] = hx(sc[24:56]); c["typed_root_blake3"] = hx(sc[56:88])
    for i, n in enumerate(R32_DIGEST_NAMES):
        c[n] = hx(sc[88 + 32 * i:120 + 32 * i])
    p = d["pose"] = {}
    ps = pv[696:832]
    p["magic"] = ps[0:8].decode(); p["verdict"] = struct.unpack_from("<I", ps, 8)[0]
    p["head_saturation"] = struct.unpack_from("<I", ps, 12)[0]
    p["logit_sums"] = list(struct.unpack_from("<11q", ps, 16))
    p["uncropped_commitment_sha256"] = hx(ps[104:136])
    b = d["beacon"] = {}
    bs = pv[832:876]
    b["magic"] = bs[0:8].decode(); b["verified"] = struct.unpack_from("<I", bs, 8)[0]
    b["quicknet_chain_hash"] = hx(bs[12:44])
    z = d["zcash_inclusion"] = {}
    zs = pv[876:948]
    z["magic"] = zs[0:8].decode()
    z["txid_internal_le"] = hx(zs[8:40]); z["txid_display"] = hx(zs[8:40][::-1])
    z["header_hash_internal_le"] = hx(zs[40:72]); z["block_hash_display"] = hx(zs[40:72][::-1])
    a = d["august_anchor"] = {}
    au = pv[948:]
    a["magic"] = au[0:8].decode()
    a["content_root_sha256"] = hx(au[8:40]); a["receipt_digest_sha256"] = hx(au[40:72])
    a["commitment_x_le"] = hx(au[72:104]); a["commitment_y_le"] = hx(au[104:136])
    a["commitment_x_decimal"] = str(int.from_bytes(au[72:104], "little"))
    a["commitment_y_decimal"] = str(int.from_bytes(au[104:136], "little"))
    if len(pv) == 1085:      # ZBAUGST2: no rounds in the statement
        a["trapdoor_flag"] = au[136]
    elif len(pv) == 1093:    # ZBAUGST3: the row's own round only
        a["row_drand_round"] = struct.unpack_from(">Q", au, 136)[0]
        a["trapdoor_flag"] = au[144]
    else:                    # ZBAUGST4: previous row's round, then the row's own
        a["previous_drand_round"] = struct.unpack_from(">Q", au, 136)[0]
        a["row_drand_round"] = struct.unpack_from(">Q", au, 144)[0]
        a["trapdoor_flag"] = au[152]
    return d


def load(path: Path) -> bytes:
    raw = path.read_bytes()
    if path.suffix == ".hex" or all(chr(c) in "0123456789abcdefABCDEF \n\r\t" for c in raw[:64]):
        return bytes.fromhex(raw.decode().strip())
    return raw


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", type=Path, default=None, help="directory to write <stem>_statement.json into (optional)")
    ap.add_argument("inputs", nargs="+", type=Path)
    args = ap.parse_args()
    for path in args.inputs:
        d = decode(load(path))
        text = json.dumps(d, indent=1)
        print(text)
        if args.out is not None:
            stem = path.stem
            for suffix in ("_groth16_public_values", "_public_values"):
                if stem.endswith(suffix):
                    stem = stem[: -len(suffix)]
                    break
            args.out.mkdir(parents=True, exist_ok=True)
            (args.out / f"{stem}_statement.json").write_text(text + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
