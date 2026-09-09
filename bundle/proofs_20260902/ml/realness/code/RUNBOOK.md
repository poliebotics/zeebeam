---
version: 1.1
date: 2026-09-09
status: ready-for-gpu
author: Cathal Ryan Hynes (author of record); drafted with BOSUN, the project's automated research assistant (remote)
---

# RUNBOOK — full-scale ZeeBeam realness scoring

Run this when a GPU frees up (~1 Sept). Until then, do NOT run the `diffusion` backend: the
GPUs on [ip redacted] / [ip redacted] / [ip redacted] / [ip redacted] carry training
arms and [ip redacted] / [ip redacted] are held by another job. Pick a box only once its
`nvidia-smi` shows the memory free and no ZeeBeam training process is running.

Everything below reuses the FROZEN, READ-ONLY machinery. Nothing writes outside
`/lambda/nfs/ZeeBeam/experiments/realness_20260830/`.

## 0. Preconditions (verify, do not skip)

```bash
cd /lambda/nfs/ZeeBeam/experiments/realness_20260830
V11=/lambda/nfs/ZeeBeam/experiments/student_scaling_20260828/v1_1

# frozen tree untouched
test "$(sha256sum $V11/FROZEN_SHA256 | cut -d' ' -f1)" \
     = 04670e6633fb2b15494a4979f8e5329f1db497c49f8450148336eb002be01805 && echo FROZEN_OK

# pinned scorer checkpoint present and correct
sha256sum /lambda/nfs/ZeeBeam/pinned/arm_a_eb72_step16000_reference_crossing/step_00016000.pt
#   expect d074775e2d9952d34c16baec134ddeff18df9ebf41170aa4c0f49e50c9c52532

# our own deliverables unchanged since freeze
sha256sum -c HASHES.sha256

# pick a genuinely free GPU (example: it must NOT be a training box)
nvidia-smi --query-gpu=index,utilization.gpu,memory.used,memory.total --format=csv
export GPU=0   # set to the verified-free index
```

## The positive class and the bank (what forms them)

Positives are the held-out REAL time-chained rows: the eval blocks of sessions **d2** and
**v10** (August is training-in-sample by the 2026-08-23 owner ruling and is NEVER a
positive). From `config/scorer_config.json`, byte-identical to the frozen SESSIONS:

- d2 eval blocks: `[1298,1698) [2796,3196) [4294,4694)`  (3 × 400 rows)
- v10 eval blocks: `[1110,1360) [2345,2595)`             (2 × 250 rows)

These 1700 rows are sliced into whole contiguous subsequences of length 100 and assigned by
deterministic round-robin to three disjoint splits:

- **bank**  — supplies feature (3)'s known-real distribution `(mu_B, sigma_B)`;
- **calib** — fixes the Youden-J threshold (real vs each attack), disjoint from test;
- **test**  — the reported positives.

`scorer.py` prints the per-split row counts and writes `disjointness_report.json`; if any
bank/calib/test row overlap exists it RAISES before scoring anything.

## 1. Freeze the scorer identity (once, before any negative is scored)

```bash
PYTHONDONTWRITEBYTECODE=1 python3 make_hashes.py      # writes HASHES.sha256
```

## 2. Score the genuine positives (feature vector + headline pooling)

Runs the frozen ARM-A step-16000 model. Repeat for each declared eval seed
(`20260823 20260901 20260902`); the scorer defaults to the first if `--eval-seed` is omitted.

```bash
for ES in 20260823 20260901 20260902; do
  CUDA_VISIBLE_DEVICES=$GPU PYTHONDONTWRITEBYTECODE=1 \
  python3 scorer.py \
     --config config/scorer_config.json \
     --backend diffusion --v1_1-dir $V11 \
     --eval-seed $ES \
     --out-dir out/full/seed_$ES
done
```

Output per seed: `splits.json`, `disjointness_report.json`, `positives_scores.json`
(per-sequence primary score = `median_t(margin_t)`, the three pooled features, and the
auxiliary combined z-score).

## 3. Score the two generator-free negative classes (SECURITY DIAGNOSTICS)

Both sweep all eval seeds and forger seeds from the config internally.

```bash
# class 3 — replay / splice
CUDA_VISIBLE_DEVICES=$GPU PYTHONDONTWRITEBYTECODE=1 \
python3 attack_replay_splice.py \
   --config config/scorer_config.json \
   --backend diffusion --v1_1-dir $V11 \
   --out-dir out/full

# class 4 — white-box (TRUE projected gradient ascent on pixels through the frozen model)
CUDA_VISIBLE_DEVICES=$GPU PYTHONDONTWRITEBYTECODE=1 \
python3 attack_whitebox.py \
   --config config/scorer_config.json \
   --backend diffusion --v1_1-dir $V11 \
   --out-dir out/full
```

Outputs: `out/full/SECURITY_DIAGNOSTIC_class3_replay_splice.json` and
`…_class4_whitebox.json`. Each carries `"LABEL":"SECURITY_DIAGNOSTIC"`,
`"NOT_A_REALNESS_ROC":true`, the per-run Youden-J threshold, the attack-success rate on
test (= per-class FPR at that threshold), the real-rejection FNR, a real-vs-attack ROC with
a block-clustered CI, and the raw per-sequence scores.

Cost note: class 4 does `PGD_STEPS=40` forward+backward passes per frame per budget
(5 budgets) per forger seed. If wall-clock is tight, cut `--out-dir` to one forger seed by
editing `config seeds.forger_seeds`, or lower `PGD_STEPS` in `attack_whitebox.py` and
re-hash; both are declared changes and must be re-frozen.

## 4. Audit for leakage BEFORE reading any ROC

```bash
for f in out/full/seed_*/disjointness_report.json; do
  python3 -c "import json,sys;d=json.load(open(sys.argv[1]));assert d['ok'] and not any(d['overlaps'].values());print(sys.argv[1],'DISJOINT OK')" "$f"
done
```

Only after this passes should the ROC/AUROC and FPR/FNR be read. Then re-verify the frozen
tree and hashes (step 0) and confirm no `__pycache__`/`.pyc` appeared under `$V11`:

```bash
find $V11 -name '__pycache__' -o -name '*.pyc'   # must print nothing
```

## What this run CANNOT be reported as

A complete four-class realness ROC. Classes 1 and 2 are absent (no generator), and the
present classes are SECURITY DIAGNOSTICS. Report per the labels the JSON already carries and
the boundaries in `LIMITATIONS.md`.

## Log
- 1.1, 2026-09-09, BOSUN. authorship line, 9 September 2026.
- 1.0, 2026-08-30, BOSUN. Commands fixed against the CPU-validated pipeline.
