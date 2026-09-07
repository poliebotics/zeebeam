"""ZeeBeam realness study package (realness_20260830).

Backend-agnostic scorer, whole-subsequence holdout, sequence-level metrics, and the two
generator-free negative classes (replay/splice and white-box). Importing any submodule
sets sys.dont_write_bytecode = True as a backstop for the byte-frozen v1_1 tree.
"""
import sys as _sys
_sys.dont_write_bytecode = True
