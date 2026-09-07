#!/usr/bin/env python3
"""Rebuild the ZeeBeam ordered-session BLAKE3 tree from chain_log.csv, independently of the Rust
membership crate, and prove it against the frozen row-96 constants before trusting it for any
other row. Emits the Merkle siblings and header fields for a requested row as JSON.

Construction (read from membership/src/lib.rs, not recalled):
  domain_hash(D, parts) = BLAKE3( D || for each part: u32be(len) || part )
  context = H(CONTEXT, [b"TB-v0.9", session_id, u32be(rows), [1], s_0, s_n, manifest_sha256, chain_log_blake3])
  leaf_t  = H(ROW, [context, u32be(t), s_t, raw_blake3, meta28, u64be(round), value, s_next, emission_blake3])
  node    = H(NODE, [context, u16be(level from 1), u32be(parent_index), left, right])
  root    = H(ROOT, [context, u32be(rows), u16be(depth), internal_root])
Padding of the 712 leaves to depth 10 follows the Python golden
(zeebeam-science/src/zeebeam_science/ordered_session_opening_candidate.py): PADDING domain over
[context, u32be(row_count), u32be(index)].
"""
import os
import csv, json, sys, blake3

CSV = os.path.join(os.path.dirname(os.path.abspath(__file__)), "chain_log.csv")
SESSION_ID = b"ZEEBEAM_MAINNET_BLOCKING_TRAINING_300S_20260822_001"
ROWS, DEPTH = 712, 10
S_0 = "74e3a131e1aaadd98e18c5f2a5f28ffaf59b8cd738e10216586efa2a893f384c"
S_N = "aeea9f4d6a55ebecd900eae187ea70dce05696551f96ae976c9a5243b3a4398e"
MANIFEST = "740d752d27b70cb63c9501a470562a52616f4a445a7a9c0fb300844f61c7d783"
CHAIN_LOG = "754e5716e65a065e5ad3146131609a1fd12e8310d966fb6bf2d3de6c568e7f8b"
CONTEXT_DOMAIN = b"ZEEBEAM_ORDERED_SESSION_CONTEXT_V1\0"
ROW_DOMAIN = b"ZEEBEAM_ORDERED_SESSION_ROW_V1\0"
NODE_DOMAIN = b"ZEEBEAM_ORDERED_SESSION_NODE_V1\0"
ROOT_DOMAIN = b"ZEEBEAM_ORDERED_SESSION_ROOT_V1\0"
PADDING_DOMAIN = b"ZEEBEAM_ORDERED_SESSION_PADDING_V1\0"
# frozen row-96 constants from native/src/lib.rs (the positive control)
F_CONTEXT = "f5eba65f4a3604bee08207b4af571b97813cbd4dd3f919c110f6c5125ab21ee0"
F_LEAF96 = "2e4e76aa9bfece295ce728ad12e37d1cc40fddfd190fa6012116cb88969f4f8b"
F_INTERNAL = "1d43309ef9a0e2fc6dba98846134280b98cda01840d2204dc41100d5038d3b37"
F_ROOT = "38a484b8793f3afadaf8fc5ba1c04d47e08cdbb4c07a39da09fb4149499a6572"
F_SIBLINGS96 = [
    "5044e45b95bb454f818d7c6053cd017c88eb63167e00553ffca90fc4db105ea7",
    "dbf38b9f009639960bb19bb34a49723b86f8dab1124e9957838c53ded27e5f14",
    "e8b97d9ae3e24d60685b5c48137e35173b22951e8338cc785470555bc110025f",
    "25d45e7302f4b9c1785ca604a4ac9d819a920eb76de51e947d2431dd8ee65913",
    "62be39eb75e5d08eb05003a92da6c99949ada732bee04fc1d23963562320747c",
    "881655aed30915bf6e5779e36c284b96ab57e679c935b067d441b6080a4b9cf8",
    "9cf816f0e5cec6347a00959af7a661b7ed783fea126aa75479d5168826683f86",
    "22e5c272ed13d3b3dede9638ea3f8519fbd9a3e90c22014c9fe0c2089e64d5d0",
    "583b3de49e15a665d6aeeaccc7af17165a58fc75a42dea3f6da873f321dadf59",
    "626eacc9816d13823066712b6b8953580e0383460da0ffa0b4a5775501872240",
]


def H(domain, parts):
    h = blake3.blake3()
    h.update(domain)
    for p in parts:
        h.update(len(p).to_bytes(4, "big"))
        h.update(p)
    return h.digest()


def load_rows():
    raw = open(CSV, "rb").read()
    assert blake3.blake3(raw).hexdigest() == CHAIN_LOG, "chain_log.csv digest differs from the frozen constant"
    lines = raw.decode().splitlines()
    assert lines[0].startswith("#")
    rows = list(csv.DictReader(lines[1:]))
    assert len(rows) == ROWS
    for i, r in enumerate(rows):
        assert int(r["t"]) == i
    assert rows[0]["S_t_hex"] == S_0
    return rows


def leaves_for(rows, context):
    out = []
    for i, r in enumerate(rows):
        s_next = bytes.fromhex(rows[i + 1]["S_t_hex"]) if i + 1 < ROWS else bytes.fromhex(S_N)
        out.append(H(ROW_DOMAIN, [
            context, i.to_bytes(4, "big"), bytes.fromhex(r["S_t_hex"]), bytes.fromhex(r["bayer_blake3_hex"]),
            bytes.fromhex(r["meta_hex"]), int(r["drand_round_number"]).to_bytes(8, "big"),
            bytes.fromhex(r["drand_round_value_hex"]), s_next, bytes.fromhex(r["emission_live_pixel_blake3_hex"]),
        ]))
    return out


def build(leaves, context, padding):
    """Returns (layers, internal_root). layers[0] = leaves padded to 2^DEPTH."""
    width = 1 << DEPTH
    if padding == "padding_domain":
        # golden: padding_leaf_hash = H(PADDING, [context, u32be(row_count), u32be(index)])
        layer = leaves + [H(PADDING_DOMAIN, [context, ROWS.to_bytes(4, "big"), i.to_bytes(4, "big")]) for i in range(len(leaves), width)]
    elif padding == "zero_leaves":
        layer = leaves + [bytes(32)] * (width - len(leaves))
    elif padding == "zero_nodes":
        layer = list(leaves)
    else:
        raise ValueError(padding)
    layers = [layer]
    for level in range(1, DEPTH + 1):
        nxt = []
        for parent in range((len(layer) + 1) // 2 if padding == "zero_nodes" else len(layer) // 2):
            left = layer[2 * parent]
            right = layer[2 * parent + 1] if 2 * parent + 1 < len(layer) else bytes(32)
            nxt.append(H(NODE_DOMAIN, [context, level.to_bytes(2, "big"), parent.to_bytes(4, "big"), left, right]))
        layers.append(nxt)
        layer = nxt
    assert len(layer) == 1
    return layers, layer[0]


def siblings(layers, index, padding):
    out = []
    pos = index
    for level in range(DEPTH):
        sib = pos ^ 1
        layer = layers[level]
        out.append(layer[sib] if sib < len(layer) else bytes(32))
        pos //= 2
    return out


def main():
    rows = load_rows()
    context = H(CONTEXT_DOMAIN, [b"TB-v0.9", SESSION_ID, ROWS.to_bytes(4, "big"), b"\x01", bytes.fromhex(S_0),
                                 bytes.fromhex(S_N), bytes.fromhex(MANIFEST), bytes.fromhex(CHAIN_LOG)])
    print("context_ok", context.hex() == F_CONTEXT)
    leaves = leaves_for(rows, context)
    print("leaf96_ok", leaves[96].hex() == F_LEAF96)
    chosen = None
    for padding in ("padding_domain",):
        layers, internal = build(leaves, context, padding)
        sib = siblings(layers, 96, padding)
        ok_sib = [s.hex() for s in sib] == F_SIBLINGS96
        root = H(ROOT_DOMAIN, [context, ROWS.to_bytes(4, "big"), DEPTH.to_bytes(2, "big"), internal])
        print(f"padding={padding} siblings96_ok={ok_sib} internal_ok={internal.hex() == F_INTERNAL} root_ok={root.hex() == F_ROOT}")
        if ok_sib and internal.hex() == F_INTERNAL and root.hex() == F_ROOT:
            chosen = (padding, layers)
    if chosen is None:
        print("NO PADDING CANDIDATE REPRODUCES THE FROZEN TREE")
        return 1
    padding, layers = chosen
    if len(sys.argv) > 1:
        t = int(sys.argv[1])
        r = rows[t]
        out = {
            "row": t, "padding": padding,
            "s_t": r["S_t_hex"], "raw_blake3": r["bayer_blake3_hex"], "emission_blake3": r["emission_live_pixel_blake3_hex"],
            "meta": r["meta_hex"], "drand_round": int(r["drand_round_number"]), "drand_value": r["drand_round_value_hex"],
            "drand_signature": r["drand_signature_hex"],
            "s_next": rows[t + 1]["S_t_hex"] if t + 1 < ROWS else S_N,
            "expected_previous_drand_round": int(rows[t - 1]["drand_round_number"]) if t > 0 else None,
            "leaf": leaves[t].hex(), "siblings": [s.hex() for s in siblings(layers, t, padding)],
            "context": context.hex(), "ordered_session_root": F_ROOT,
        }
        json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), f"row_{t:03d}_membership.json"), "w"), indent=1)
        print(f"wrote row_{t:03d}_membership.json")
    return 0


if __name__ == "__main__":
    sys.exit(main())
