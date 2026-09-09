---
version: 2.1
date: 2026-09-09
status: settled-two-of-four-classes-exploratory
author: Cathal Ryan Hynes (author of record); drafted with BOSUN, the project's automated research assistant
---

# Realness: does the frozen scorer reject forged chains?

> **Corrected 1 Sep after audit.** The first issue claimed three of four adversary classes.
> It is **two at best, and exploratory**. The preregistration defines class 2 as generation
> conditioned on the *previous forged* frame; this lane generates from the *real* frame two
> rows back, so it is class 1 only. The preregistration also requires at least three forger
> seeds; this used one. Both corrections are load-bearing and are carried below.

The 28 August completeness audit named one measurement as owed above all others: our
headline AUROC of 1.0 was **conditioning** discrimination (right pattern versus wrong
pattern on a real frame), not **realness** (real versus forged). This note reports the owed
measurement for **one** generator-based class (class 1) plus one generator-free diagnostic
(class 3). Class 2 (autoregressive forgery, each fake conditioned on the previous fake) has
not been run. Class 4 is outstanding. The class-1 result rests on a single forger seed where
the preregistration requires three or more, so it is **exploratory, not confirmatory**.

## The generator-based result (class 1 only, exploratory)

Negatives are frames produced by the published F-A v1 forger at its step-100,000 checkpoint:
each fake is generated from the **real** capture two rows earlier plus that row's and the
target row's emissions. Because the source frame is real rather than a previous forgery, this
is the preregistration's **class 1**, not class 2; no autoregressive forgery chain was built.
One forger checkpoint was used where the preregistration requires at least three seeds, which
makes this exploratory. The forged frames
were made for the held-out sessions themselves and re-mosaiced back to raw so the frozen
scorer consumed them through its own unchanged loader; the re-mosaic is an exact inverse of
the packing and was round-trip asserted on every file.

Per evaluation seed (three seeds), on the test split:

| quantity | value |
|---|---|
| fake sequences passing as real | 0.000 (all three seeds) |
| real sequences wrongly rejected | 0.000 (all three seeds) |
| real-vs-generated AUROC | 1.0, bootstrap CI [1.0, 1.0] |
| sample size | 6 real vs 6 forged test **subsequences** per seed, 5 clusters each |

**The small sample travels with every quote of these numbers.** Six against six, at sequence
level, is a clean separation but a narrow one; a perfect score on twelve sequences is not the
same evidence as a perfect score on hundreds. It is stated here as it stands.

## The generator-free diagnostics

Class 3 (replay and splice: real frames rearranged so a frame is presented at a chain
position it does not belong to) ran nine times, three evaluation seeds by three forger seeds.
Every run: attack success 0.000, real rejection 0.000, AUROC 1.0, on 6 real against 18 forged
sequences per run.

This class carries the label the runbook requires: **SECURITY DIAGNOSTIC, not a realness
ROC**. It is generator-free, so it measures resistance to rearrangement, not to synthesis,
and it must never be pooled with the generated-fakes result into a single "realness" number.

Class 4 (white-box projected gradient ascent on pixels through the frozen model) is
**OUTSTANDING**. It exhausted memory on a 40 GB card twice; an 80 GB retry is planned and
queued behind a training arm. Nothing in this note should be read as covering an adaptive
white-box attacker; the separately sealed L2-6 attack floor already showed the frozen
compact verifier has **no margin** against such an attacker at eps16 with 1,000 queries, and
that citation travels with any use of these results.

## Split hygiene

Before any score was read, the bank/calibration/test split was checked independently of the
scorer's own report: 600 bank, 600 calibration and 500 test **rows** over the 1,700 held-out
rows — six whole subsequences per split, which is the unit actually scored and the reason the
sample size above is six and not six hundred. Zero overlap between any pair, and identical
splits across evaluation seeds by design.
The scorer's own disjointness report agrees. My first check of this was itself buggy and read
the row structure wrongly; the corrected check is the one reported.

## Limits

Preliminary: one subject, one rig, one occasion, with a within-occasion subsequence holdout.
No whole-take, subject, rig or day generalisation is claimed or tested. One published forger
checkpoint (step-100,000), applied zero-shot from its April training to the August-era
sessions, where the preregistration asks for three or more seeds. Held-out sessions d2 and v10
only. Sequence-level scoring on six subsequences per split. A real first data point on
real-versus-generated, and not a general anti-forgery claim.

## Log

- 2.1 (2026-09-09, BOSUN) — authorship line, 9 September 2026.
- 2.0 (2026-09-01, BOSUN) — corrected after Sol's ultra audit returned BLOCK. Coverage
  restated from three classes to one generator-based class plus one diagnostic; exploratory
  status added (one forger seed against a preregistered three); "sequences" corrected to rows
  versus subsequences; the L2-6 citation's own qualifications carried; class-4 wording fixed.
  A separate defect is recorded rather than hidden: the produced ROC artifact carries a
  self-contradiction, its LABEL saying generated-realness while an inherited explanation
  field still says generator-free and not-a-realness-ROC, because the scoring script reused
  the attack harness's text. The numbers are unaffected; the artifact needs a corrected
  sidecar before it is cited anywhere.
- 1.0 (2026-09-01, BOSUN) — first issue. Generated-fakes run completed 31 August 14:00
  UTC; class 3 completed 16:34 UTC; class 4 outstanding. The generated-fakes lane exists
  because the original runbook recorded classes 1 and 2 as absent for want of a
  photorealistic generator; the F-A v1 forger and its code snapshot supplied exactly that.
