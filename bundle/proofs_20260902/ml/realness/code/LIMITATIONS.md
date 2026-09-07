---
version: 1.0
date: 2026-08-30
status: honest-boundaries
author: BOSUN (remote)
---

# LIMITATIONS — read before any number leaves this tree

This machinery produces a PRELIMINARY, bounded measurement. The boundaries below are part of
the result, not footnotes to it. Several are hard limits the frozen prereg itself states; a
few are places where the prereg cannot be executed as written and the reason is given plainly
rather than substituted silently.

## 1. Only two of four attack classes are built

Classes 3 (replay/splice) and 4 (white-box) are implemented. Classes 1 (naive per-frame
synthesis to the pattern) and 2 (temporally-coherent generative sequence) are NOT built:
both require a photorealistic conditional generator (the pix2pixHD lineage), which is on an
unreachable machine. They are deliberately not fabricated. A discriminator that separates
real from splices and from a white-box pixel attack has NOT been shown to separate real from
a competent photorealistic generator, which is the harder and more important adversary.

## 2. The measured discrimination is bounded by the adversary's strength

An error rate against these two attacks is an upper bound on the discriminator's quality,
not its quality against a real forger. Replay/splice is a weak adversary (it breaks the
S_t↔E_t binding by construction); the white-box attack is stronger but is still a bounded
pixel perturbation of a real frame, not a learned generator. Low attack-success here does
NOT imply low attack-success against classes 1–2. This is why the outputs are labelled
SECURITY DIAGNOSTICS.

## 3. Classes 3 and 4 are NOT a realness ROC, and are never pooled into one

The auditor was explicit: calling the pooled discrimination of generator-free attacks
"realness" would be misleading. The frozen protocol says "reported per attack class AND
pooled". This machinery reports per class, and DELIBERATELY DOES NOT emit a single pooled
"realness" number across classes, because pooling only two of four attack classes — both
generator-free — would misrepresent a security diagnostic as a realness measurement. This is
the one place the frozen prereg's "pooled" instruction is intentionally not executed as
written; the reason is this limitation, and it is surfaced in every output
(`"NOT_A_REALNESS_ROC": true`) rather than hidden.

## 4. One subject, one rig, one occasion (the prereg's own PRELIMINARY caveat)

The positive class is the existing 22 Aug take (sessions d2, v10), held out by whole
contiguous subsequences WITHIN that single occasion. There is no whole-TAKE holdout across
independent occasions and no subject/session generalisation. A discriminator that separates
held-out subsequences of one capture from forgeries of that same capture has not been shown
to generalise to a new subject, rig, or day. The definitive corpus (R1, rig, multiple
independent takes) is required for any non-preliminary claim. This pass is PRELIMINARY.

## 5. The features are conditioning-derived

All three features are built on the same conditional-diffusion quantity (per-frame
correct-vs-wrong-emission eps-MSE) that underlies the existing conditioning result. That is a
validated instrument for optical/sensor consistency, but it means the realness signal and the
conditioning signal share machinery: the discriminator inherits whatever the conditioning
model does and does not capture. Feature (3) in particular is a distributional match on a
low-dimensional scalar statistic, not a full spatial feature-map cross-correlation against a
rich known-real bank; the richer bank needs the R1 corpus.

## 6. Small counts → wide, sometimes unquotable intervals

With ~18 whole subsequences split three ways, each split holds only a handful of sequences
and the bootstrap cluster (the eval block) count per side can fall below 3. The metrics
emit an explicit `CI_WARNING` when a side has fewer than 3 clusters; such an
interval is NOT quotable. Point estimates from so few sequences are indicative only.

## 7. Threshold and split assignment are declared, not tuned — but are choices

Subsequence length (100), the round-robin split assignment, the feature-3 Gaussian kernel,
the auxiliary weights (1.0, 0.5, 0.5), and the eval/forger seeds are all frozen in
`config/scorer_config.json` before scoring and hashed. They are defensible choices, not the
only ones; a different subsequence length or split could shift the small-sample numbers.
There is no per-sequence tuning, and calibration is disjoint from test.

## 8. The white-box full-scale run is compute-bounded, not capability-bounded

The PGD attack is real (gradients through the frozen model) but runs a fixed 40-step,
5-budget sweep. A patient adversary with more steps, a better objective than mean-margin, or
a transfer/EOT strategy could do better. The reported attack-success-vs-budget curve is a
floor on white-box vulnerability, not a ceiling.

## Bottom line

What can be honestly said after the full-scale run: a preliminary, one-occasion,
whole-subsequence security diagnostic showing how the frozen conditioning-derived scorer
resists two generator-free forgery families, with error rates reported per class and the
raw scores published. What CANNOT be said: a definitive realness FPR/FNR, a four-class
result, cross-occasion generalisation, or resistance to a photorealistic generator.

## Log
- 1.0, 2026-08-30, BOSUN.
