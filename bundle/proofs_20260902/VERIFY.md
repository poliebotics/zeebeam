---
version: 2.10
date: 2026-09-09
status: public-release-bundle
author: Cathal Ryan Hynes (author of record); drafted with BOSUN, the project's automated research assistant
---

# ZeeBeam proof bundle: how to verify it yourself

This public release bundle contains 262 Groth16 proofs, produced with SP1 6.4.0 (circuit v6.1.0) on 2 and 3
September 2026: two ceremony-1 row proofs of the relation's previous revision (1,085-byte statement, rows 96 and 72);
259 final-relation row proofs (1,101-byte statement, previous-row leg, both beacon rounds published) for every anchored
row, two of them made in ceremony 2 under a key the proving machine reproduced byte for byte from the frozen tree and
257 on an eight-instance fleet under the same key; and one whole-session chain proof. A third party can verify all of
them with the steps below and nothing from us but these files.

## 0. Check the bundle first

```
sha256sum -c SHA256SUMS
```

Every line must say `OK` before anything else is trusted.

## Files

| file | what |
|------|------|
| `row_096_groth16_proof.bin`, `row_072_groth16_proof.bin` | raw Groth16 proofs, 356 bytes each: 4-byte Groth16 vkey-hash prefix + encoded proof |
| `row_096_groth16_public_values.bin`, `row_072_groth16_public_values.bin` | the 1,085-byte public statements of ceremony 1 (`.hex` copies alongside) |
| `row_096_groth16.bin`, `row_072_groth16.bin` | the same proofs in SP1's `SP1ProofWithPublicValues` bincode framing, for the SP1 SDK |
| `row_096_manifest.json`, `row_072_manifest.json` | prover's record: vkey, ELF digest, timings, oracle result, tamper controls |
| `row_096_statement.json`, `row_072_statement.json` | the ceremony-1 statements decoded into named fields |
| `PINS.json` | every constant a verifier of the ceremony-1 proofs must pin: vkey, ELF, session digests, beacon key, model digests, anchor, receipt, curve |
| `box_guest.elf` | the exact guest ELF that was proved in ceremony 1 (sha256 in PINS.json); needed only to re-derive the vkey |
| `PINS_final.json`, `final_relation/` | the final revision (previous-row leg, both rounds published, 1,101-byte statement): path-independent ELF `zeebeam_guest_final.elf`, its build record, the statements it produced when executed on the development machine (decoded and raw hex), the source-tree digest list, and `ceremony2/`: framed and raw Groth16 proofs, public values, prover manifests, decoded statements and the box, orchestrator and halo verification logs |
| `anchor/` | the 260-row chain-log prefix, `PREFIX.json` and the canonical receipt that the August legs read |
| `ml_splits.json`, `pose_confusion_116.json` | every row's role in both models' data; the pose confusion matrix on the 116 evaluation rows |
| `verifier/` | the standalone verifier source (direct dependencies `sp1-verifier` 6.4.0 and `sha2` 0.10, 209 locked packages), the decoder, the Python oracles and their results |
| `source/` | the frozen source tree (38 hashed files), the vendored `jubjub`, the two embedded model blobs and the fail-closed build driver `build_reproducible.sh` (`set -euo pipefail`, `cargo build --locked`, exactly one ELF, SHA-256 and vkey asserted), with a README on what a fresh build needs; `cd source/rust && sha256sum -c ../../final_relation/SOURCE_TREE_SHA256SUMS`; the driver's transcript is `final_relation/build_transcript_halo_20260902.txt` |
| `final_relation/execution_logs/`, `final_relation/GPU_MEMORY_OBSERVATIONS.md` | the final-ELF execution logs with every cycle-tracker region behind the cost table; the operator's `nvidia-smi` readings behind the GPU-memory figures |
| `ml/realness/` | the generated-fakes ROC record, the realness harness code and configuration, and a README stating which Section 8.1 figures have no archived record |
| `final_relation/boundary_row_095/` | regression fixture for the two-round ABI: row 95 (previous round 31521619, own round 31521620) executed under the final ELF against an independent oracle, 1,101/1,101 bytes, both rounds decoded; an execution, not a proof |
| `verifier/results/halo_timing_20260903.txt` | development-machine wall-clock and memory record for one standalone verification (proof, flipped byte, wrong key) of two row proofs and the chain proof; the verifier's binary size and locked package count |
| `verifier/results/standalone_transcript_*.txt` | complete standalone-verifier transcripts for all four proofs from the bundled source built `--locked`, each ending in both tamper rejections, `VERIFIED` and `exit=0` |
| `final_relation/chain/`, `source/rust/zeebeam_chain_sp1_candidate/` | the whole-session chain relation (paper Section 9.5): framed and raw Groth16 proof, public values, `chain_manifest.json` with the 279-byte ZBCHAIN1 statement decoded, `chain_expect.json` (the statement computed without the circuit) with the portable oracle `chain_expect.py`, `session_tree.py` and the 712-row `chain_log.csv` it reads, the proved guest ELF `zeebeam_chain_guest.elf` (460,416 B, sha256 `4934b9e2…3926`), `REPRODUCIBLE_CHAIN.json` (ELF and vkey reproduced on the proving box), `CHAIN_SOURCE_SHA256SUMS`, the development machine's SDK cold-verification and standalone transcripts and the box logs (`logs/`), and the development-machine build transcript; the frozen chain crate in `source/rust/zeebeam_chain_sp1_candidate/` with its fail-closed driver `source/build_reproducible_chain.sh` |
| `source/deps/`, `source/rust/preprocess_v1_candidate/`, `final_relation/DEPENDENCY_SHA256SUMS` | every crate the proved trees depend on by relative path, at its mirrored path: `zeebeam-b3xof-relation`, `preprocess_v1_candidate`, the trained-r32 model and relation crates and the uncr64 model and relation crates, digested together with `source/vendor/` (`cd source && sha256sum -c ../final_relation/DEPENDENCY_SHA256SUMS`); `source/README.md` gives the layout a rebuild needs and `source/stage_for_rebuild.sh` recreates it from the bundle (the frozen manifests use the development machine's absolute layout); an air-gapped rebuild from the bundle alone has not been rehearsed |
| `all_rows/` | every row of the session through the oracles (`oracle_rows_*.jsonl`, `all_rows_20260902.py`), the executed 1,101-byte statements of rows 1 to 259 (`exec/`), their witnesses (`witnesses/`), the per-row figure, and `proofs/`: the 257 fleet Groth16 proofs (framed, raw, public values, manifests), `STANDALONE_VERIFY.txt`, `FLEET_STATS.json`, and per-box prover logs, orchestration logs and scripts (`logs/`); `PINS_all_rows.json` at the bundle root |
| `ENVIRONMENT.md` | locked environment description |
| `SHA256SUMS` | digests of everything above |

## 1. Verify a ceremony-1 proof (no SP1 SDK, no ELF, no prover; the verifier's cryptographic dependency is `sp1-verifier`, plus `sha2` for printing digests)

```
cd verifier/standalone_verifier && cargo build --release --locked
./target/release/zeebeam-standalone-verifier ../../row_096_groth16_proof.bin ../../row_096_groth16_public_values.bin 0x0092cba2c77fe7fc426bb539add5d0365e4edea340c2f3c040da9bd73cf9a82c
```

Expected: `VERIFIED`, with `tamper_public_byte_87_rejected=true` and `wrong_vkey_rejected=true`.
Repeat with the row 072 files. The Groth16 verifying key for this SP1 version is embedded in the
`sp1-verifier` crate; the SP1 vkey hash above is the hash of the guest program's verification key.

## 1b. Verify a ceremony-2 proof (final relation)

```
./target/release/zeebeam-standalone-verifier ../../final_relation/ceremony2/row_096_groth16_proof.bin ../../final_relation/ceremony2/row_096_groth16_public_values.bin 0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490
```

Expected: `VERIFIED` with the same two tamper rejections. Repeat with the row 072 files. Both
ceremony-2 proving processes panicked in the sp1-cuda client's destructor after writing and verifying
their proofs (`Command terminated by signal 6`; `final_relation/ceremony2/logs/row_*_stderr.log`);
the proofs are valid because they verify independently here. The decoded
statements in `final_relation/ceremony2/` carry `previous_drand_round` and `row_drand_round`; both
are byte-identical to the statements in `final_relation/` that the ELF produced when executed on the
development machine.

## 1c. Verify the chain proof (whole-session relation)

```
./target/release/zeebeam-standalone-verifier ../../final_relation/chain/chain_groth16_proof.bin ../../final_relation/chain/chain_groth16_public_values.bin 0x005402a848fdc787e32c0a110ca2662dd61c580481b4e418d5400a52fbd7df50
```

Expected: `VERIFIED` with the two tamper rejections. The 279-byte statement (`chain_public_values.hex`) is
decoded field by field under `decoded` in `chain_manifest.json`; `chain_expect.json` is the same statement
computed without the circuit from `chain_log.csv` and the frozen constants. `python3
final_relation/chain/chain_expect.py` is portable (it reads `chain_log.csv` and `session_tree.py` beside
itself) and rewrites `chain_expect.json` in place; the file it writes must be identical to the bundled one,
and every field of it must equal the decoded statement. The chain ELF `4934b9e2…3926` (460,416 B) and its
vkey were reproduced on the proving box (`REPRODUCIBLE_CHAIN.json`); the frozen crate is `source/rust/zeebeam_chain_sp1_candidate/`
(`cd source/rust/zeebeam_chain_sp1_candidate && sha256sum -c ../../../final_relation/chain/CHAIN_SOURCE_SHA256SUMS`; the
dependency crate and vendored patch: `cd source && sha256sum -c ../final_relation/DEPENDENCY_SHA256SUMS`). The guest ELF
is `final_relation/chain/zeebeam_chain_guest.elf`; `zeebeam-chain vkey --elf` on it must print the vkey above, and
`source/build_reproducible_chain.sh` rebuilds it fail-closed against these pins. The development machine's
records of both verifications are `logs/halo_sdk_cold_verify_20260903.txt` (SDK, `--elf`, `--expect chain_expect.json`,
ending `CHAIN_OK`) and `logs/halo_standalone_verify_20260903.txt` (ending `VERIFIED`, `exit=0`). What the chain proof
establishes and what it does not is Section 9.5 of the paper: the log is internally consistent and
beacon-seeded at every step and the session tree is its tree; the operator still chose the frames whose
digests the log records.

## 1d. Verify the 257 fleet proofs (all anchored rows)

```
for p in ../../all_rows/proofs/row_*_groth16_proof.bin; do ./target/release/zeebeam-standalone-verifier "$p" "${p%_proof.bin}_public_values.bin" 0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490 | grep -c VERIFIED; done | sort | uniq -c
```

Expected: `257 1`. Rows 96 and 72 are the ceremony-2 proofs of Section 1b, so all 259 anchored rows are
covered under one key. Each proved statement must equal the executed statement of the same row:
`xxd -p -c 4000 all_rows/proofs/row_NNN_groth16_public_values.bin` against `all_rows/exec/row_NNN_public_values.hex`.
`all_rows/proofs/STANDALONE_VERIFY.txt` is the development machine's record of both checks for every row
(257 `VERIFIED`, 257 `statement_vs_executed=equal`); `PINS_all_rows.json` lists every proof's digests,
timings, ELF and vkey (one ELF, one vkey); `all_rows/proofs/FLEET_STATS.json` the timing summary;
`all_rows/logs/boxN/` each prover's stdout and stderr, every stderr ending in the destructor panic described
in 1b; `all_rows/logs/cold_verify_halo_20260903/` the SDK cold verification of rows 1, 95, 130, 200 and
259 against the reproducible ELF and their oracles. `all_rows/figures/all_rows_figures.py` redraws the per-row figure
from the bundled records (`python3 all_rows/figures/all_rows_figures.py [OUT_STEM]`, no arguments needed).

## 2. Check that the vkey hash belongs to this program

Optional, needs the SP1 SDK: `setup(box_guest.elf)` and compare `vk.bytes32()` to the hash above.
`sha256sum box_guest.elf` must equal `guest_elf_sha256` in the manifests. The guest source is the
`row_binding_join_membership_sp1_candidate` tree pinned in PINS.json by its `Cargo.lock` digests. For
the final revision, `final_relation/REPRODUCIBLE_FINAL.json` gives the ELF digest and vkey a
`--locked` build with the recorded `--remap-path-prefix` flags must reproduce.

## 3. Read a statement

```
python3 verifier/decode_statement.py row_096_groth16_public_values.bin
python3 verifier/decode_statement.py final_relation/ceremony2/row_072_groth16_public_values.bin
```

The decoder handles 1,085-, 1,093- and 1,101-byte statements, prints JSON to stdout, writes nothing
unless `--out DIR` is given, and exits 0 on success; `--out` into a scratch directory reproduces the
bundled `*_statement.json` files byte for byte. The fields that matter: `membership_prefix.row_index`, `coupling.score_numerator` (over
4), `pose.verdict` and `pose.logit_sums`, `zcash_inclusion.txid_display` and `block_hash_display`,
`august_anchor.content_root_sha256`, `receipt_digest_sha256`, the commitment `C`, and `trapdoor_flag`
(1 in both proofs). Final-revision statements additionally carry `previous_drand_round` and
`row_drand_round`.

## 4. The two checks the proof cannot do for you

- Look up `block_hash_display` on canonical Zcash mainnet: it must be block 3456294, and the
  transaction `txid_display` must be in it. The circuit proved the txid from raw bytes and the Merkle
  branch to that header; it cannot know which chain is canonical.
- Look up the drand quicknet round on the public schedule. **Which frame it bounds depends on the
  revision.** The ceremony-1 proofs verified only the row's own round r_t (31521620 for row 96,
  31521616 for row 72); that round seeds the *next* state and therefore bounds the *following*
  frame's pattern, not the proved frame's. The final revision also verifies the preceding row's round
  r_{t−1} and publishes both; r_{t−1} is the round whose release time bounds the proved frame's own
  pattern. In both cases the bound is on the pattern's computation; it becomes a bound on the
  capture only under the acquisition assumptions stated in the paper (Section 9.2), which no proof
  supplies.

## 5. What the proof says, and what it does not

Under the pinned verification key, ELF and constants, an accepting proof establishes execution
binding: the published coupling numerator, pose verdict and logit sums are the frozen networks'
outputs on the raw frame whose BLAKE3 digest is bound into the chain state and into the session tree
at the published row index; the beacon signature(s) published verify under the quicknet key; the
published txid and header hash are the ZIP-244 identifier and SHA-256d of the transaction and header
reached by the branch; the memo of action 0 of that transaction, decrypted under the holder's viewing
key, pins the receipt whose digest is published; and `C` opens under the receipt's key `Y` to the
`contentRoot` recomputed from `PREFIX.json`, which joins the anchored chain-log prefix to the row's
own verified values. A trapdoor flag of 1 proves knowledge of `td` with `Y = td·Base8`.

What it does not establish: that the frame came from a camera (raw injection, projector substitution
and optical relay are outside the relation); that the learned scores mean what their names suggest;
or any upper bound on when the record existed. The anchor is a chameleon commitment whose trapdoor
the operator held, so `C` binds nothing against the operator and gives no record upper bound to any
later verifier; it is an inclusion receipt for the receipt digest, with that contingency disclosed.
Publishing `td` later demonstrates equivocation capability, not which record existed at capture time.

Row 96's pose verdict (letter_y) disagrees with its cue annotation (superman); row 72's (neutral)
agrees. Both are reported. The pose model is a diagnostic. No hash proves a photon hit a wall.

## Log

- 2.10 (2026-09-09, BOSUN) — authorship line, 9 September 2026.

- 2.9 (2026-09-05, BOSUN) — title (the bundle holds 262 proofs); the staging script's checker now covers every manifest.

- 2.8 (2026-09-05, BOSUN) — staging script for the frozen layout; provider identifiers and addresses removed from the logs.

- 2.7 (2026-09-05, BOSUN) — public release: status and opening restated for the 262 bundled proofs; verifier dependencies stated exactly; the five further path-dependency crates bundled and digested; realness note replaced by the v2.0 record the manuscript cites; fleet instance identifiers and addresses removed; stale paths and a bytecode file removed.

- 2.6 (2026-09-03, BOSUN) — verifier timing record added (`verifier/results/halo_timing_20260903.txt`).

- 2.5 (2026-09-03, BOSUN) — after Sol's round 10: chain crate relocated under `source/rust/`, dependency crate and digest list, chain ELF, SDK and standalone transcripts, fail-closed chain driver, portable figure script.

- 2.4 (2026-09-03, BOSUN) — chain proof (1c) and the 257 fleet proofs of all anchored rows (1d), their files, and the portable chain oracle.

- 2.3 (2026-09-02, BOSUN) — after Sol's round 6: fail-closed build driver and transcript, execution logs, GPU observations, realness records, oracle-note sidecars.

- 2.2 (2026-09-02, BOSUN) — after Sol's round 5: portable decoder, `source/`, complete verifier transcripts, boundary-row fixture, cleanup-panic disclosure.

- 2.1 (2026-09-02, BOSUN) — ceremony 2 added: final-relation proofs, their verification command and files.

- 2.0 (2026-09-02, BOSUN) — after Sol's round-3 referee report: checksum step first,
  `--locked` builds, the round-indexing correction (ceremony 1 bounds the following frame), the
  contingent upper-bound language replaced by the paper's Section 9.4 statement, the final-revision
  files, `anchor/` and the ML files listed, decoder side effect noted.
- 1.0 (2026-09-02, BOSUN) — written after both proofs were verified standalone.
