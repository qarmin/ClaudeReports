#!/usr/bin/env bash
# Compare two binaries' rendering, discounting pixels that are not reproducible in the first place.
#
#   tools/pixelcheck.sh <binary A> <binary B> -- <scene args...>
#
# A single snapshot from each side is not enough. The perf_test scenes contain a status label that
# prints a measured duration, so a 15x10 px block at the bottom of the window differs between two
# runs of the *same* binary. Comparing one snapshot to one snapshot reports that as a rendering
# difference, which is how a real ablation once got credited with a bug it did not have.
#
# So: run each binary N times, take the pixels that vary within a binary as the noise floor, and
# report only differences that fall outside it. Anything reported here is a real rendering change.
#
# The noise floor has to be *sampled*, and undersampling it turns noise into a false "differs":
# at RUNS=3 a scale-factor 1.75 scene reported 259 differing pixels that were entirely inside the
# status label, and RUNS=6 put the floor at 476 px and the verdict back to identical. So the check
# also reports whether the leftover pixels sit inside the region the noise occupies, which is the
# signal to raise RUNS rather than to believe the difference.
#
# Env: RUNS (default 5), FRAMES (default 6).
#
# Raise FRAMES when a divergence is suspected to only accumulate after many edited/scrolled
# frames - candidate 093's scrollbar-thumb regression was pixel-identical at the default FRAMES=6
# (reported "identical" by this script) and only became visible starting around 80 frames, found
# by manually sweeping frame counts outside this script before this env var existed.
set -uo pipefail

if [ "$#" -lt 3 ]; then
    sed -n '2,20p' "$0" >&2
    exit 2
fi

a_bin="$1"; b_bin="$2"; shift 2
[ "${1:-}" = "--" ] && shift
RUNS=${RUNS:-5}
FRAMES=${FRAMES:-6}

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

for side in a b; do
    bin=$([ "$side" = a ] && echo "$a_bin" || echo "$b_bin")
    for run in $(seq 1 "$RUNS"); do
        if ! "$bin" --bench "$@" --frames "$FRAMES" --snapshot "$work/$side$run.ppm" >/dev/null 2>&1; then
            echo "failed to render with $bin" >&2
            exit 1
        fi
    done
done

RUNS=$RUNS python3 - "$work" <<'PY'
import os, sys, pathlib

work = pathlib.Path(sys.argv[1])
runs = int(os.environ["RUNS"])

def load(p):
    data = p.read_bytes().split(b"\n", 3)
    return data[1].decode(), data[3]

width = None
sides = {}
for side in ("a", "b"):
    imgs = []
    for run in range(1, runs + 1):
        dims, body = load(work / f"{side}{run}.ppm")
        width = int(dims.split()[0])
        imgs.append(body)
    sides[side] = imgs

def differing(x, y):
    return {i for i in range(0, min(len(x), len(y)), 3) if x[i:i + 3] != y[i:i + 3]}

# Pixels that a single binary cannot reproduce between its own runs.
noise = set()
for imgs in sides.values():
    for other in imgs[1:]:
        noise |= differing(imgs[0], other)

between = differing(sides["a"][0], sides["b"][0])
real = between - noise

total = min(len(sides["a"][0]), len(sides["b"][0])) // 3
print(f"noise floor (same binary, {runs} runs each): {len(noise)} px")
print(f"raw difference between binaries:            {len(between)} px")
print(f"difference outside the noise floor:         {len(real)} px of {total}")

def bbox(pixels):
    xs = [(i // 3) % width for i in pixels]
    ys = [(i // 3) // width for i in pixels]
    return min(xs), max(xs), min(ys), max(ys)

if not real:
    print("VERDICT: pixel-identical outside the noise floor")
elif noise:
    # The unreproducible part of these scenes is one status label, so anything left over inside its
    # footprint is far more likely to be an undersampled floor than a rendering change.
    nx0, nx1, ny0, ny1 = bbox(noise)
    outside = {i for i in real
               if not (nx0 <= (i // 3) % width <= nx1 and ny0 <= (i // 3) // width <= ny1)}
    if outside:
        x0, x1, y0, y1 = bbox(outside)
        print(f"  {len(outside)} px outside the noise region too, bbox x {x0}..{x1} y {y0}..{y1}")
        print("VERDICT: rendering differs")
    else:
        print(f"  ...but all of it is inside the noise region (x {nx0}..{nx1} y {ny0}..{ny1})")
        print(f"VERDICT: inconclusive - re-run with RUNS={runs * 2}")
else:
    x0, x1, y0, y1 = bbox(real)
    print(f"  bbox: x {x0}..{x1}  y {y0}..{y1}")
    print("VERDICT: rendering differs")
PY
