#!/usr/bin/env python3
"""ZeeBeam: boundary diagram + cost strip, FINAL relation (previous-row leg, both rounds published,
1,101-byte statement). Palette validated with the dataviz skill (light surface). Numbers: final ELF 8bcadf53…,
row 96 execution log 2 Sep 2026; paper/zeebeam.md, Sections 5 to 7 and 9."""
import sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch
from matplotlib.lines import Line2D

STATUS = sys.argv[1] if len(sys.argv) > 1 else "Groth16-proved for all 259 anchored rows under the final key (rows 96 and 72 in ceremony 2; 257 more on an eight-A100 fleet, 3 Sep 2026, all standalone-verified, statements equal to their executions); the whole-session chain relation proved separately (66 beacons, 712 transitions, the tree; 8 min). Rows 260 to 711 await a new anchor"

SURF = "#fcfcfb"; INK = "#0b0b0b"; INK2 = "#52514e"; MUTED = "#898781"; RULE = "#c3c2b7"; GRID = "#e1e0d9"
BLUE = "#2a78d6"; ORANGE = "#eb6834"; AQUA = "#1baf7a"; YELLOW = "#eda100"
FAM = {"beacon": BLUE, "chain": ORANGE, "pose": AQUA, "zcash": YELLOW}
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 9.5, "text.color": INK, "axes.edgecolor": RULE})

fig = plt.figure(figsize=(20, 11.5), facecolor=SURF)
fig.text(0.02, 0.955, "ZeeBeam", fontsize=30, fontweight="semibold", color=INK, ha="left", va="center")
fig.text(0.02, 0.918,
         "Every component that lives inside one zero-knowledge statement. Row 96 of the operator-supplied session dated 22 August 2026. "
         "One SP1 program: 4,149,712,293 instructions, 1,101 public bytes, checked against oracles built without the circuit.",
         fontsize=10.5, color=INK2, ha="left", va="center")
import textwrap
_status_lines = textwrap.wrap(f"Status: {STATUS}.", width=185)[:2]
for _i, _l in enumerate(_status_lines):
    fig.text(0.02, 0.897 - 0.019 * _i, _l, fontsize=10.5, color=INK2, ha="left", va="center")

ax = fig.add_axes([0.02, 0.06, 0.60, 0.83]); ax.set_xlim(0, 100); ax.set_ylim(0, 100); ax.axis("off")
ax.add_patch(FancyBboxPatch((1, 2), 98, 95, boxstyle="round,pad=0,rounding_size=2.5", fc="none", ec=INK, lw=1.6))
ax.text(3, 95.2, "INSIDE THE PROOF   ·   private witnesses in, 1,101 public bytes out   ·   all displayed relations are circuit-checked against pinned constants",
        fontsize=9.5, color=INK, fontweight="semibold", va="center")


def rgba(h, a):
    h = h.lstrip("#"); return tuple(int(h[i:i + 2], 16) / 255 for i in (0, 2, 4)) + (a,)


def node(x0, y0, w, h, fam, title, body, cost, dashed=False, muted=False):
    col = MUTED if muted else FAM[fam]
    ax.add_patch(FancyBboxPatch((x0, y0), w, h, boxstyle="round,pad=0,rounding_size=1.2",
                                fc=rgba(col, 0.06 if muted else 0.10), ec=(col if dashed else "none"),
                                lw=1.2, ls=("--" if dashed else "-")))
    ax.add_patch(FancyBboxPatch((x0, y0 + 0.6), 0.55, h - 1.2, boxstyle="round,pad=0,rounding_size=0.25", fc=col, ec="none"))
    ax.text(x0 + 1.6, y0 + h - 1.6, title, fontsize=9.6, fontweight="semibold", color=INK, va="top")
    ax.text(x0 + 1.6, y0 + h - 4.6, body, fontsize=8.0, color=INK2, va="top", linespacing=1.25)
    if cost:
        ax.text(x0 + w - 1.2, y0 + 0.9, cost, fontsize=8, color=INK2, ha="right", va="bottom")


def arrow(p, q, label=None, col=INK2, rad=0.0, ls="-"):
    ax.add_patch(FancyArrowPatch(p, q, arrowstyle="-|>,head_length=4,head_width=2.4", color=col, lw=1.6,
                                 connectionstyle=f"arc3,rad={rad}", shrinkA=0, shrinkB=0, ls=ls))
    if label:
        mx, my = (p[0] + q[0]) / 2, (p[1] + q[1]) / 2
        ax.text(mx, my + 1.3, label, fontsize=7.6, color=MUTED, ha="center", va="bottom",
                bbox=dict(fc=SURF, ec="none", pad=0.6))


# top band: the liveness sandwich
Y = 70; H = 17
node(3, Y, 21, H, "beacon", "drand quicknet beacon, two rounds",
     "BLS12-381 signatures verified in circuit\nfor the row's round r_t and the previous\nrow's r_t-1; both published (31521620).\nS_t recomputed from row t-1's record:\nr_t-1 is the round bounding THIS pattern.", "4.9M + 6.7M")
node(27, Y, 22, H, "chain", "BLAKE3 feedback chain",
     "S_t -> S_t+1 folds the row's OWN Bayer\nhash, the round and the beacon value.\nThe next pattern needs this capture.\nBLAKE3 of the 24.5 MB frame, in circuit.", "703M")
node(52, Y, 22, H, "chain", "emission derived, in circuit",
     "BLAKE3-XOF expansion -> four-octave\ninteger render, 1920x1080x3. The nominal\nemission is a pure function of the\nsupplied chain state.", "2.14B")
node(77, Y, 21, H, "chain", "coupling score",
     "frozen r32 integer scorer on the (camera,\nemission) pair. Row 96: +56.8M matched.\n72 selection rows: matched > crossed\n72/72 paired; pooled AUROC 0.998.", "339M")
arrow((24, Y + 8.5), (27, Y + 8.5)); arrow((49, Y + 8.5), (52, Y + 8.5)); arrow((74, Y + 8.5), (77, Y + 8.5))

# second band: camera, pose, tree
Y2 = 48; H2 = 15
node(27, Y2, 22, H2, "chain", "the camera frame, once",
     "raw Bayer 4600x5320 -> RGGB planes\n4x2300x2660, packed ONCE and shared by\ntwo consumers. (Same-length swap bug\ncaught by the oracle.)", "421M")
node(52, Y2, 22, H2, "pose", "pose network output",
     "frozen uncr64 integer scorer on the whole\nframe at 64x64; verdict + 11 logit sums\npublished. Row 96: letter_y (cue superman,\nwrong); row 72: neutral (right). Diagnostic.", "345M")
node(77, Y2, 21, H2, "chain", "typed tile root",
     "BLAKE3 tile tree over the primaries,\ncamera even / emission odd, depth 5.\nThe commitment the pose and coupling\nlegs both share; published.", "14.8M")
arrow((38, Y2 + H2), (38, Y + 0.2)); arrow((49, Y2 + 7.5), (52, Y2 + 7.5)); arrow((74, Y2 + 7.5), (77, Y2 + 7.5))
arrow((47, Y2 + H2), (86, Y + 0.2), rad=0.22, label="same frame")

# bottom band: the capture-time Zcash anchor family
Y3 = 8; H3 = 15
node(3, Y3 + 17, 21, H3, "zcash", "anchor tx (mainnet: out-of-band)",
     "8d1672..f206, block 3456294, 22 Aug 2026.\nv6 ZIP-244 txid recomputed from raw bytes\n(2 Ironwood actions); merkle branch\nreplayed; header hashed 000000000052f2b0..\nBoth published.", "0.24M")
node(3, Y3, 21, H3, "zcash", "binding receipt",
     "SHA-256(853 canonical bytes) == the binding\nthe memo leg DECRYPTED, fae21624.. Y and C\nare parsed from the pinned bytes, checked\non-curve and in the subgroup, never free.", "")
node(27, Y3, 22, H3 + 17, "zcash", "chameleon opening + trapdoor",
     "C == m*Base8 + r*Y on BabyJubJub, with\nm = SHA-512(domain || canonical statement)\nmod L, computed in circuit around the\ncontentRoot the prefix leg produced.\nBase8 is a hardcoded constant (audit 2 Sep:\nas a witness it allowed B8 = m^-1 C).\n\nTrapdoor knowledge: Y == td*Base8, td a\nprivate witness (real td at the ceremony).\nFull C published.\n\nWhat it is: an inclusion receipt for the\nreceipt digest. What it is not: a record\nupper bound for anyone, because the\ncustodian held the trapdoor. Said plainly.", "126M, all August legs")
node(52, Y3, 22, H3 + 17, "zcash", "chain-log prefix join",
     "blake3(260-row prefix, 134 KB) == the\nanchored record. contentRoot = domain hash of\nPREFIX.json -> feeds the chameleon leg.\n\nRow 96's S_t, Bayer hash, drand round and\nvalue inside that prefix are compared to the\nrelation's OWN verified outputs above.\nThe receipt covers THIS row.", "")
node(77, Y3 + 17, 21, H3, "zcash", "the on-chain link, closed",
     "the txid leg hands its action 0 to the\nmemo leg; the decrypted binding is the\nONLY digest the receipt may hash to.\nNo host-supplied memo digest remains.", "")
node(77, Y3, 21, H3, "zcash", "memo decryption, in circuit",
     "orchard 0.15.3 runs in the zkVM under the\nholder viewing key -> binding=fae21624..\nUnder A5 (binding of the extracted cmx)\na second key would open cmx twice.\nWrong key rejects.", "15.4M")
arrow((13.5, Y3 + 17), (13.5, Y3 + H3))
arrow((24, Y3 + 11.6), (27, Y3 + 11.6))
arrow((52, Y3 + 22), (49, Y3 + 22))
ax.text(50.5, Y3 + 22 + 1.1, "contentRoot", rotation=90, fontsize=7.2, color=MUTED, ha="center", va="bottom")
ax.text(13.5, 56, "HOW THE BANDS JOIN\n\nThe anchor's commitment covers a\nchain-log prefix that contains row 96.\nIts S_t, Bayer hash, drand round and\nvalue are compared, in circuit, to the\nvalues the top band verified. So the\nreceipt is on THIS row, not merely\nnear it.",
        fontsize=8.0, color=INK2, ha="center", va="center", linespacing=1.3)

handles = [Line2D([0], [0], marker="s", ms=10, mec="none", mfc=c, ls="") for c in (BLUE, ORANGE, AQUA, YELLOW)]
ax.legend(handles, ["drand beacon", "BLAKE3 feedback chain, emission, coupling", "pose network output", "Zcash inclusion"],
          loc="lower left", bbox_to_anchor=(0.0, -0.075), ncol=4, frameon=False, fontsize=9, handletextpad=0.5, columnspacing=1.6)

# cost strip (final ELF, row 96 execution log, 2 Sep 2026)
bx = fig.add_axes([0.775, 0.16, 0.205, 0.64]); bx.set_facecolor(SURF)
legs = [("emission render", 2141120868, "chain"), ("raw-frame BLAKE3", 702715828, "chain"), ("pack + preprocess", 421045347, "chain"),
        ("pose network", 344916129, "pose"), ("coupling score", 338997588, "chain"),
        ("August anchor legs (prefix, receipt,\nchameleon, trapdoor, validation)", 126039564, "zcash"),
        ("untracked glue + I/O", 27847169, "chain"), ("memo decryption", 15394564, "zcash"), ("typed tile root", 14810611, "chain"),
        ("previous-row advance (BLS + BLAKE3)", 6733568, "beacon"), ("drand BLS verify (row t)", 4904590, "beacon"),
        ("XOF expansion", 4829169, "chain"), ("txid + merkle + header", 235697, "zcash"), ("session membership", 121601, "chain")]
legs.sort(key=lambda t: t[1])
names = [l[0] for l in legs]; vals = [l[1] / 1e6 for l in legs]; cols = [FAM[l[2]] for l in legs]
ypos = list(range(len(legs)))
bx.barh(ypos, vals, height=0.55, color=cols, edgecolor="none")
bx.set_yticks(ypos); bx.set_yticklabels(names, fontsize=8.6, color=INK)
bx.set_xlabel("millions of RISC-V instructions inside the proof (linear scale)", fontsize=9, color=MUTED)
bx.set_xlim(0, 2450)
for y, (n, v, f) in zip(ypos, legs):
    lab = f"{v:,}" if v < 1_000_000 else (f"{v / 1e6:,.1f}M" if v < 1e9 else f"{v / 1e9:.2f}B")
    bx.text(v / 1e6 + 18, y, lab, va="center", fontsize=8.3, color=INK2)
for s in ("top", "right"):
    bx.spines[s].set_visible(False)
bx.spines["left"].set_color(RULE); bx.spines["bottom"].set_color(RULE)
bx.tick_params(axis="y", length=0); bx.tick_params(axis="x", colors=MUTED, labelsize=8.5)
bx.xaxis.grid(True, color=GRID, lw=0.8); bx.set_axisbelow(True)
bx.set_title("What each ZK'd thing costs\nthe render is 51.6% of 4.15 billion;\nevery cryptographic leg is nearly free",
             fontsize=10, color=INK, loc="left", pad=10)

fig.text(0.645, 0.105, "OUTSIDE THE PROOF, by design", fontsize=9.5, fontweight="semibold", color=INK)
fig.text(0.645, 0.030,
         "- the verifier looks up the published header hash on canonical mainnet\n"
         "- the verifier checks both published rounds against drand's schedule;\n"
         "  r_t-1's release time bounds the pattern under A4 and premise A8 with the predecessor\n  record fixed beforehand, and the capture only under\n"
         "  the acquisition premises P1/P2 (relay not excluded)\n"
         "- what pose class \"letter_y\" means physically: no hash proves a photon hit a wall",
         fontsize=8.6, color=INK2, va="bottom", linespacing=1.35)
fig.text(0.98, 0.012, "BOSUN, 5 September 2026. Nineteen referee passes; all 259 anchored rows and the session chain proved. Source: paper/zeebeam.md, Sections 5 to 7 and 9",
         fontsize=7.5, color=MUTED, ha="right")

import os
out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "zeebeam")
fig.savefig(out + ".png", dpi=170, facecolor=SURF); fig.savefig(out + ".svg", facecolor=SURF)
print("written", out + ".png", out + ".svg")
