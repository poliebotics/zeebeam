---
version: 0.5
updated: 2026-08-25
status: active
author: BOSUN
---

# ZeeBeam row-binding ordered-membership SP1 candidate

This successor preserves the frozen v1 row evaluator and adds the exact
post-capture ordered-session opening. Its three private SP1 vectors are the
120-byte `ZBROWW01` row header, a 555-byte `ZBOSM001` membership witness for
the frozen row-96 session, and the exact 24,472,000-byte raw Bayer row.

The membership vector carries the full context needed by the frozen tree:
session identifier, row count, terminal flag, `S_0`, `S_N`, authority-manifest
SHA-256, chain-log BLAKE3, wrapped root, and ten leaf-to-root siblings. It does
not duplicate any selected-row field. The guest derives those fields from the
row header and raw bytes before constructing the leaf.

The 696-byte `ZBROWM02` public journal contains a 352-byte session descriptor
and the unchanged 344-byte trained-r32 scorer block. The private row state,
capture and emission digests, metadata, drand values, and sibling path do not
appear in the public journal.

The relation enforces one row opening under the supplied issuer-derived root.
The authority-manifest SHA-256, chain-log BLAKE3, and root are supplied values
bound by the relation. Their authenticity is not verified. The relation proves
no honest whole-log construction, recorder commitment, capture time,
chronology, liveness, sensor origin, physical causation, or reality. The host is
split into an execute path and a separate SP1 Core prove-and-verify path. The
Core host binds the exact raw input and public oracle, verifies the original
proof, and requires rejection after mutating public byte 87. A separate
feature-gated Groth16 successor preserves those gates and requires the returned
proof variant and circuit version to be exactly Groth16 and `v6.1.0`. Its
built-in negative checks also require rejection after changing decoded
Groth16 proof byte 96 and after rotating the program commitment in a cloned
wrong verifying key. Those checks execute for both prove and cold-verify, but
this source freeze does not claim that a proof or the real-proof negative
checks have been run.

## Native gate

```text
cargo test --locked --offline -p zeebeam-row-binding-membership -p zeebeam-row-binding-membership-native
cargo test --locked --offline --features groth16 --bin zeebeam-row-binding-membership-groth16
```

## Log

| Version | Date | Author | Change |
|---|---|---|---|
| 0.5 | 2026-08-25 | BOSUN | Added fail-closed Groth16 proof-byte and wrong-verifying-key negative checks without claiming their execution. |
| 0.4 | 2026-08-25 | BOSUN | Added the separate feature-gated direct-Groth16 host while preserving the exact Core host. |
| 0.3 | 2026-08-25 | BOSUN | Registered the separate exact-input SP1 Core prove-and-verify host while retaining the no-proof claim. |
| 0.2 | 2026-08-25 | BOSUN | Defined the supplied-digest and root-authenticity ceiling, locked the test command, and added boundary and mutation coverage. |
| 0.1 | 2026-08-25 | BOSUN | Added the privacy-aligned ordered-session membership successor, fixed row-96 cross-language oracle, and execute-only guest and host. |

— BOSUN ⚓
