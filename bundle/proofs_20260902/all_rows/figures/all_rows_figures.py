#!/usr/bin/env python3
"""Per-row figures for the whole 712-row session from the independent oracle values
(all_rows_20260902/oracle_rows_*.jsonl) and, where present, the final-ELF executions (exec/summary.txt).
Palette: the ZeeBeam light-surface palette. Outputs PNG + SVG next to this script."""
import json, glob, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

import os, sys
# portable: the bundle root is three levels above this script (bundle/all_rows/figures/); optional argv[1] = output stem
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SURF = "#fcfcfb"; INK = "#0b0b0b"; INK2 = "#52514e"; MUTED = "#898781"; RULE = "#c3c2b7"; GRID = "#e1e0d9"
BLUE = "#2a78d6"; ORANGE = "#eb6834"; AQUA = "#1baf7a"; YELLOW = "#eda100"
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 9.5, "text.color": INK, "axes.edgecolor": RULE})

rows = {}
for f in glob.glob(f'{ROOT}/all_rows/oracle_rows_*.jsonl'):
    for line in open(f):
        d = json.loads(line); rows[d['row']] = d
T = sorted(rows); n = len(T)
num = np.array([rows[t]['coupling_score_numerator'] for t in T]) / 4e6
verdict = np.array([rows[t]['pose_verdict'] for t in T])
rnd = np.array([rows[t]['drand_round'] for t in T]); prev = np.array([rows[t]['previous_drand_round'] or rows[t]['drand_round'] for t in T])
ann = {r['row']: r for r in json.load(open(f'{ROOT}/ml/pose_cue_annotation_712_v2.json'))['rows']}
# the eleven model classes are cues 01..11 in order; the '12_freeze' cue (hold the last pose) carries class 10 in the
# training labels (ml_splits.json: 27 training and 7 evaluation freeze rows labelled 10); pre_cue rows have no label
cue_names = ['01_neutral', '02_superman', '03_letter_y', '04_letter_m', '05_letter_c', '06_letter_a', '07_letter_t', '08_letter_x', '09_hands_up', '10_point_left', '11_point_right/12_freeze']
cue_idx = {'01_neutral': 0, '02_superman': 1, '03_letter_y': 2, '04_letter_m': 3, '05_letter_c': 4, '06_letter_a': 5, '07_letter_t': 6, '08_letter_x': 7, '09_hands_up': 8, '10_point_left': 9, '11_point_right': 10, '12_freeze': 10}
label = np.array([cue_idx.get(ann[t]['cue'], -1) if t in ann else -1 for t in T])
hold = np.array([1 if (t in ann and ann[t]['observed_verdict'] == 'hold_confirmed') else 0 for t in T])
executed = {}
try:
    for line in open(f'{ROOT}/all_rows/exec/summary.txt'):
        p = line.split(); executed[int(p[0])] = ('verified_public_values=true' in line, int([x for x in p if x.startswith('total_instruction_count=')][0].split('=')[1]) if any(x.startswith('total_instruction_count=') for x in p) else None)
except FileNotFoundError:
    pass
proved = {96, 72}

fig = plt.figure(figsize=(20, 12), facecolor=SURF)
fig.text(0.02, 0.965, "ZeeBeam session ZEEBEAM_MAINNET_BLOCKING_TRAINING_300S_20260822_001: every row through the proved relation's oracles",
         fontsize=17, fontweight="semibold", color=INK, ha="left", va="center")
fig.text(0.02, 0.945, f"{n} rows. Values from the circuit-independent Python oracles (both frozen networks re-run from their blobs); rows 1–259 also executed under the final ELF (dots); rows 96 and 72 Groth16-proved (stars).",
         fontsize=9.5, color=INK2, ha="left", va="center")
fig.text(0.02, 0.928, "Rows 260–711 lie outside the August anchor and cannot pass the anchor leg without a new anchor transaction.", fontsize=9.5, color=INK2, ha="left", va="center")

ax1 = fig.add_axes([0.08, 0.635, 0.90, 0.245]); ax1.set_facecolor(SURF)
ax1.plot(T, num, color=ORANGE, lw=1.4)
ax1.axhline(0, color=RULE, lw=0.8)
ax1.axvspan(-0.5, 259.5, color=YELLOW, alpha=0.08, lw=0)
for a, b in ((96, 120), (320, 344), (544, 568)):
    ax1.axvspan(a - 0.5, b - 0.5, color=BLUE, alpha=0.10, lw=0)
ex_t = [t for t in T if t in executed]; ax1.scatter(ex_t, [num[T.index(t)] for t in ex_t], s=6, color=INK2, zorder=3)
ax1.scatter([t for t in proved if t in rows], [num[T.index(t)] for t in proved if t in rows], marker='*', s=160, color=INK, zorder=4)
ax1.set_ylabel("coupling numerator / 4 (millions)", color=INK2); ax1.set_xlim(-2, 714)
ax1.set_title("coupling score (matched pattern) per row; yellow = anchored prefix (rows 0–259); blue = the 72 architecture-selection rows", loc="left", fontsize=10, color=INK)
for s in ("top", "right"): ax1.spines[s].set_visible(False)
ax1.yaxis.grid(True, color=GRID, lw=0.8); ax1.set_axisbelow(True)

ax2 = fig.add_axes([0.08, 0.355, 0.90, 0.225]); ax2.set_facecolor(SURF)
agree = (verdict == label)
ax2.scatter(T, np.where(label < 0, -1, label), s=9, color=MUTED, label="cue (annotation; pre_cue rows at -1)")
ax2.scatter(T, verdict, s=9, color=np.where(agree, AQUA, ORANGE), label="pose verdict (integer model)")
ax2.set_yticks(list(range(-1, len(cue_names)))); ax2.set_yticklabels(['pre_cue (no label)'] + cue_names, fontsize=8)
ax2.set_xlim(-2, 714); ax2.set_ylabel("class");
held = hold == 1
ax2.set_title(f"pose verdict against the cue label per row: agreement {int((agree & held).sum())}/{int(held.sum())} on hold_confirmed rows, {int(agree.sum())}/{n} on all rows (green = agrees, orange = disagrees); "
              "the take is time-confounded, so this is a diagnostic, not accuracy", loc="left", fontsize=10, color=INK)
for s in ("top", "right"): ax2.spines[s].set_visible(False)
ax2.yaxis.grid(True, color=GRID, lw=0.8); ax2.set_axisbelow(True)

ax3 = fig.add_axes([0.08, 0.07, 0.42, 0.225]); ax3.set_facecolor(SURF)
ax3.step(T, rnd - rnd.min(), where='post', color=BLUE, lw=1.4, label="row's own round r_t")
ax3.step(T, prev - rnd.min(), where='post', color=YELLOW, lw=1.0, ls='--', label="previous row's round r_t-1")
ax3.set_xlim(-2, 714); ax3.set_ylabel(f"drand quicknet round − {rnd.min():,}"); ax3.set_xlabel("row")
ax3.set_title(f"beacon rounds per row: {len(set(rnd.tolist()))} distinct rounds over {n} rows (3 s period); rows where r_t-1 ≠ r_t: {int((prev != rnd).sum())}", loc="left", fontsize=10, color=INK)
ax3.legend(frameon=False, fontsize=8, loc="upper left")
for s in ("top", "right"): ax3.spines[s].set_visible(False)
ax3.yaxis.grid(True, color=GRID, lw=0.8); ax3.set_axisbelow(True)

ax4 = fig.add_axes([0.56, 0.07, 0.42, 0.225]); ax4.set_facecolor(SURF)
if executed:
    et = sorted(executed); ei = np.array([executed[t][1] or 0 for t in et]) / 1e9; ok = np.array([executed[t][0] for t in et])
    ax4.scatter(et, ei, s=8, color=np.where(ok, AQUA, ORANGE))
    ax4.set_title(f"final-ELF execution per anchored row: {int(ok.sum())}/{len(et)} match their oracle on all 1,101 bytes;\ninstructions {ei.min():.4f}–{ei.max():.4f} billion (the prefix scan grows linearly with the row index)", loc="left", fontsize=10, color=INK)
    ax4.set_ylabel("RISC-V instructions (billions)"); ax4.set_xlabel("row")
else:
    ax4.text(0.5, 0.5, "executions pending", ha="center", va="center", color=MUTED, transform=ax4.transAxes)
for s in ("top", "right"): ax4.spines[s].set_visible(False)
ax4.yaxis.grid(True, color=GRID, lw=0.8); ax4.set_axisbelow(True)
fig.text(0.98, 0.012, "BOSUN, 2 September 2026. Sources: all_rows_20260902/, pose_cue_annotation_712_v2.json, final ELF 8bcadf53…", fontsize=7.5, color=MUTED, ha="right")
out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(os.path.dirname(os.path.abspath(__file__)), 'all_rows_session')
fig.savefig(out + '.png', dpi=150, facecolor=SURF); fig.savefig(out + '.svg', facecolor=SURF)
print('written', out + '.png', 'rows', n, 'executed', len(executed))
