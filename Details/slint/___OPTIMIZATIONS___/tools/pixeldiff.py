#!/usr/bin/env python3
"""Row/column breakdown for a real (outside-noise-floor) pixel difference between two binaries.

    tools/pixeldiff.py --a-repeats <a1.ppm> <a2.ppm> ... --b-repeats <b1.ppm> <b2.ppm> ...

Complements tools/pixelcheck.sh rather than replacing it: pixelcheck.sh's own "inside/outside the
noise region" verdict uses one bounding box per side, which reports "inconclusive" whenever the
noise floor is scattered (e.g. a status label in one corner *and* a scrollbar row elsewhere) even
when the real difference is cleanly separable from noise at the per-pixel level. This script does
the same repeats-establish-a-noise-set idea, but subtracts noise as an exact set of pixel
positions (not a bounding box) and prints which rows/columns the leftover difference sits in - use
it when pixelcheck.sh reports "inconclusive" and you need to tell a real, spatially separate
divergence apart from a widely-scattered noise floor.

Needs >= 2 repeats per side to have any noise floor to subtract; more repeats narrows it. All
inputs must be same-size PPM (P6) snapshots, e.g. from `<binary> --bench ... --snapshot foo.ppm`.
"""

import argparse
import pathlib


def load(p):
    data = pathlib.Path(p).read_bytes().split(b"\n", 3)
    dims = data[1].decode()
    return dims, data[3]


def differing(x, y):
    n = min(len(x), len(y))
    return {i for i in range(0, n, 3) if x[i:i + 3] != y[i:i + 3]}


def noise_set(files):
    imgs = [load(f)[1] for f in files]
    noise = set()
    for other in imgs[1:]:
        noise |= differing(imgs[0], other)
    return noise, imgs[0]


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--a-repeats", nargs="+", required=True, help="Repeated snapshots from binary A (same scene/args)")
    ap.add_argument("--b-repeats", nargs="+", required=True, help="Repeated snapshots from binary B (same scene/args)")
    args = ap.parse_args()

    a_noise, a_ref = noise_set(args.a_repeats)
    b_noise, b_ref = noise_set(args.b_repeats)

    between = differing(a_ref, b_ref)
    real = between - a_noise - b_noise
    width = int(load(args.a_repeats[0])[0].split()[0])

    total = min(len(a_ref), len(b_ref)) // 3
    print(f"A noise floor: {len(a_noise)} px")
    print(f"B noise floor: {len(b_noise)} px")
    print(f"raw diff A[0] vs B[0]: {len(between)} px")
    print(f"real diff (outside both noise floors): {len(real)} px of {total}")
    if real:
        cols_by_row = {}
        for i in real:
            px = i // 3
            r, c = px // width, px % width
            cols_by_row.setdefault(r, []).append(c)
        rows = sorted(cols_by_row)
        print(f"rows affected: min={rows[0]} max={rows[-1]} count={len(rows)}")
        for r in rows[:10]:
            cs = cols_by_row[r]
            print(f"  row {r}: cols {min(cs)}-{max(cs)} ({len(cs)} px)")


if __name__ == "__main__":
    main()
