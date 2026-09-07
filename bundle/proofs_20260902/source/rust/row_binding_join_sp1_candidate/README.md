# ZeeBeam row binding join SP1 candidate

This source-only candidate joins one mandatory `TB-v0.9` row through:

1. exact 24,472,000-byte raw Bayer BLAKE3;
2. the exact v9 transition from `S_t` to `S_next`;
3. same-row `S_t` BLAKE3-XOF expansion and the exact 1920 by 1080 RGB renderer;
4. `PREPROCESS_V1_CANDIDATE_20260823` at primary size 256;
5. the unchanged typed-root context and frozen trained-r32 PTQ-v2 score.

Classification is always `CANDIDATE_ONLY`. Row membership is always false. The
private and public ABIs contain no session identifier, chain-log commitment,
membership path, capture-time anchor, recorded PNG, proof envelope, or prior
guest ELF. This relation proves no chronology, liveness, sensor origin, physical
causation, or reality.

The implementation consumes two SP1 vectors. The first is a closed 120-byte
header (`ZBROWW01`, ABI 1, protocol 9, row index LE, `S_t`, exact 28-byte
metadata, drand round LE, drand value, zero reserved bytes). The second must be
exactly 24,472,000 raw bytes.

The public journal is exactly 672 bytes. Its 328-byte row prefix is followed by
the existing unchanged 344-byte trained-r32 public block. Framing integers are
little-endian, while the public drand round is emitted in the big-endian protocol
encoding. The public prefix contains no session or membership field. The frozen
row-96 public journal SHA-256 is
`9d1380cd2a691fbc2cafee1cf0c02c8e9ab7a160dfdeb43b1c4e251a96cab513`.

Only offline native tests are authorized at this stage:

```text
cargo test --offline -p zeebeam-row-binding-join --lib
cargo test --offline -p zeebeam-row-binding-join-native
```

The SP1 host is execute-only by construction. No proof-generation mode exists in
`script/src/main.rs`. Building or executing the SP1 guest is a later explicit
gate after native parity and source review.
