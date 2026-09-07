#!/usr/bin/env python3
"""Circuit-independent oracle for the whole-session chain relation: recomputes, from chain_log.csv and
the frozen session constants, every public field the chain guest publishes (via session_tree.py, which
reproduced the frozen row-96 tree constants before being trusted). Writes chain_expect.json."""
import json, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import session_tree as st

rows = st.load_rows()
context = st.H(st.CONTEXT_DOMAIN, [b"TB-v0.9", st.SESSION_ID, st.ROWS.to_bytes(4, "big"), b"\x01", bytes.fromhex(st.S_0),
                                   bytes.fromhex(st.S_N), bytes.fromhex(st.MANIFEST), bytes.fromhex(st.CHAIN_LOG)])
assert context.hex() == st.F_CONTEXT
leaves = st.leaves_for(rows, context)
layers, internal = st.build(leaves, context, "padding_domain")
root = st.H(st.ROOT_DOMAIN, [context, st.ROWS.to_bytes(4, "big"), st.DEPTH.to_bytes(2, "big"), internal])
assert root.hex() == st.F_ROOT
rounds = sorted({int(r["drand_round_number"]) for r in rows})
exp = {"row_count": st.ROWS, "tree_depth": st.DEPTH, "distinct_rounds": len(rounds), "first_round": rounds[0], "last_round": rounds[-1],
       "s_0": st.S_0, "s_n": st.S_N, "chain_log_blake3": st.CHAIN_LOG, "authority_manifest_sha256": st.MANIFEST,
       "context_digest_blake3": context.hex(), "ordered_session_root_blake3": root.hex(), "session_id": st.SESSION_ID.decode(),
       "source": "chain_expect.py over session_tree.py (frozen row-96 constants reproduced first); chain_log.csv digest checked"}
json.dump(exp, open(os.path.join(HERE, 'chain_expect.json'), 'w'), indent=1)
print(json.dumps({k: exp[k] for k in ("row_count", "tree_depth", "distinct_rounds", "first_round", "last_round")}))
