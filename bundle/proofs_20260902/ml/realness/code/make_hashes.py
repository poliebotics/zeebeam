#!/usr/bin/env python3
"""Write HASHES.sha256 over every deliverable (scorer, attacks, library, config, docs).

Freezes the discriminator + attack identity. Run after any edit; the values in here are
what a hostile reviewer recomputes.
"""
from __future__ import annotations
import sys, os, hashlib
sys.dont_write_bytecode = True
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha256(p: Path) -> str:
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for c in iter(lambda: f.read(1 << 22), b""):
            h.update(c)
    return h.hexdigest()


def main():
    targets = []
    for pat in ["scorer.py", "attack_replay_splice.py", "attack_whitebox.py",
                "run_fixture.py", "make_hashes.py",
                "config/scorer_config.json",
                "RUNBOOK.md", "LIMITATIONS.md", "README.md"]:
        p = HERE / pat
        if p.is_file():
            targets.append(p)
    for p in sorted((HERE / "realness").glob("*.py")):
        targets.append(p)
    lines = []
    for p in sorted(targets):
        lines.append(f"{sha256(p)}  {p.relative_to(HERE)}")
    (HERE / "HASHES.sha256").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))
    print(f"\nwrote {HERE/'HASHES.sha256'} ({len(lines)} files)")


if __name__ == "__main__":
    main()
