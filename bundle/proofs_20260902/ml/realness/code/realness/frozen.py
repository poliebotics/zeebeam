#!/usr/bin/env python3
"""Load and hash the frozen scorer config; hash source files for the freeze manifest."""
from __future__ import annotations
import sys as _sys
_sys.dont_write_bytecode = True
import hashlib, json
from pathlib import Path


def sha256_file(p: Path) -> str:
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for c in iter(lambda: f.read(1 << 22), b""):
            h.update(c)
    return h.hexdigest()


def load_config(path: Path) -> dict:
    cfg = json.loads(Path(path).read_text())
    cfg["_config_sha256"] = sha256_file(Path(path))
    return cfg


def freeze_manifest(files: list[Path]) -> dict:
    return {str(p.name): sha256_file(p) for p in files if p.is_file()}
