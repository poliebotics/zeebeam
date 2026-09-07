---
version: 1.0
date: 2026-09-02
status: regression-fixture-execution-not-proof
author: BOSUN
---

# Boundary-row fixture: row 95, previous round 31521619, own round 31521620

`row_095_execute.log` is the frozen host's output for row 95 under the final ELF (`oracle=independent_row_95
checked=1101 circuit_derived=0`, 4,149,667,741 instructions). Its second `oracle_note` line says the coupling
numerator "is circuit-derived"; that wording is stale prose hard-coded in the frozen host
(`script/src/witness.rs`, the branch taken when `expected_coupling_score_numerator` is supplied). The numerator
2,910,142 was in fact compared against the Python re-run of the frozen network in
`row_095_frozen_models_python.json` and counted among the 1,101 checked bytes, as the counts on the same
line record. The frozen source is not edited because any line shift changes the ELF (manuscript 7.3).

Files: `row_095_membership.json` (session-tree witness and every expected value with its source),
`row_095_python_oracle.json` (typed context, typed root, uncropped commitment from the audited modules),
`row_095_frozen_models_python.json` (both networks re-run in Python), `row_095_public_values.hex` and
`row_095_statement_final_executed.json` (the 1,101-byte statement, decoded: previous_drand_round 31521619,
row_drand_round 31521620). This is an execution, not a proof.

## Log

- 1.0 (2026-09-02, BOSUN) — written after Sol's round-6 finding on the stale oracle note.
