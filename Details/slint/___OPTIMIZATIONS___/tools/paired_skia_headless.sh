#!/usr/bin/env bash
# Paired comparison for perf_test's headless Skia harness (`--skia-bench`), parsing the
# "later frames median" line. Unlike paired_skia_{first,later}_frames.sh this needs no
# xvfb/LIBGL_ALWAYS_SOFTWARE=1 - `--skia-bench` renders through Skia's own software surface
# directly, no window/GL context involved. See perf_test/src/skia_headless.rs's module doc for
# why this mode exists (the winit `--frames` path can never reach the dirty-region clip code at
# all, regardless of renderer flags).
#
# Usage: paired_skia_headless.sh <a_bin> <b_bin> --skia-bench <perf_test args> (no "--" separator)
# Env: RUNS (default 9), TASKSET (core to pin to, e.g. TASKSET=3; unset = unpinned/multi-core).
set -uo pipefail
a_bin="$1"; b_bin="$2"; shift 2
RUNS=${RUNS:-9}
TASKSET=${TASKSET:-}

run() {
    local bin="$1"; shift
    local cmd=()
    if [ -n "$TASKSET" ]; then
        cmd=(taskset -c "$TASKSET")
    fi
    "${cmd[@]}" "$bin" "$@" 2>/dev/null | awk '
        /^later frames median/ { gsub(/ms/, "", $4); print $4 }'
}

a_times=(); b_times=()
for _ in $(seq 1 "$RUNS"); do
    a_times+=("$(run "$a_bin" "$@")")
    b_times+=("$(run "$b_bin" "$@")")
done

printf 'A (%s): %s\n' "$a_bin" "${a_times[*]}"
printf 'B (%s): %s\n' "$b_bin" "${b_times[*]}"

python3 - "${a_times[@]}" -- "${b_times[@]}" <<'PY'
import sys
args = sys.argv[1:]
sep = args.index('--')
a = [float(x) for x in args[:sep]]
b = [float(x) for x in args[sep+1:]]
def median(v):
    v = sorted(v)
    n = len(v)
    return v[n//2] if n % 2 else (v[n//2-1]+v[n//2])/2
ma, mb = median(a), median(b)
wins = sum(1 for x, y in zip(a, b) if y < x)
print(f"A median: {ma:.3f} ms   B median: {mb:.3f} ms   delta: {(mb-ma)/ma*100:+.1f}%   B faster in {wins}/{len(a)} paired runs")
PY
