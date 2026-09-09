---
version: 1.1
date: 2026-09-09
status: operator-observations-not-prover-logs
author: Cathal Ryan Hynes (author of record); drafted with BOSUN, the project's automated research assistant
---

# GPU memory figures quoted in the manuscript

The prover does not log GPU memory. The two figures in Section 7.1 are operator readings of
`nvidia-smi --query-gpu=utilization.gpu,memory.used` taken over SSH while a proof was running, and
nothing more:

| ceremony | reading | when | what was running |
|----------|---------|------|------------------|
| 1 (2 Sep 2026, ~05:30–06:17 UTC) | about 26.5 GB used | during the row-96 or row-72 proof | `sp1-gpu-server` 6.4.0 on the A100-SXM4-40GB |
| 2 (2 Sep 2026) | `52 %, 25615 MiB` at ~16:07 UTC; `100 %, 27279 MiB` at ~16:09 UTC | row-96 proof, retry run | `sp1-gpu-server` 6.4.0 on the A100-SXM4-40GB |

The ceremony-2 readings are exact as printed by `nvidia-smi`; the ceremony-1 figure was noted at the
time in the project state document without the exact MiB value and is therefore approximate. Neither
is a peak measurement; the manuscript quotes them as observations of memory in use, not as the
prover's requirement.

## Log

- 1.1 (2026-09-09, BOSUN) — authorship line, 9 September 2026.
- 1.0 (2026-09-02, BOSUN) — written after Sol's round-6 request for the record behind the
  GPU-memory figures.
