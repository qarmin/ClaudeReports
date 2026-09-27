#!/usr/bin/env python3
"""Characterize a difference between two decoded-output dumps, for a candidate whose patch
intentionally changes output (e.g. a faster approximate IDCT) rather than staying bit-exact.

bench_decode itself only hashes output and aborts on any mismatch - it can't tell you HOW
different two outputs are. Use this instead when a candidate is expected to differ:

    # dump raw decoded bytes from each binary once
    ./bin_baseline <image> 1 --dump baseline.raw
    ./bin_patched  <image> 1 --dump patched.raw
    tools/diffbytes.py baseline.raw patched.raw

NOTE: bench_decode does not currently support --dump. Add it temporarily (write `data` to a
file with std::fs::write instead of / in addition to hashing it) only when a candidate actually
needs a characterized-difference check - most candidates are bit-exact and never need this.
"""
import sys

def main() -> int:
    if len(sys.argv) != 3:
        print(__doc__)
        return 2
    a = open(sys.argv[1], "rb").read()
    b = open(sys.argv[2], "rb").read()
    if len(a) != len(b):
        print(f"VERDICT: length differs ({len(a)} vs {len(b)} bytes) - not just a rounding change")
        return 1
    diffs = [(i, a[i], b[i]) for i in range(len(a)) if a[i] != b[i]]
    if not diffs:
        print("VERDICT: byte-identical")
        return 0
    max_delta = max(abs(x - y) for _, x, y in diffs)
    frac = len(diffs) / len(a)
    print(f"bytes differing: {len(diffs)} of {len(a)} ({frac:.4%})")
    print(f"max absolute delta: {max_delta}")
    print(f"first few diffs (offset, baseline, patched): {diffs[:10]}")
    print("VERDICT: output differs - characterize whether this magnitude is acceptable "
          "(see SKILL.md 'Correctness' - report this, don't decide for the reader)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
