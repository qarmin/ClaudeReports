#!/usr/bin/env bash
# Paired comparison for perf_test's FemtoVG renderer via --frames, parsing the "later frames" median
# line (steady state, after the first-frame conversion cost) - see paired_femtovg_first_frame.sh's
# header for why this exists separately from tools/paired.sh.
#
# Usage: paired_femtovg_later_frames.sh <a_bin> <b_bin> <perf_test args after --mode images ...> (no "--" separator)
# Env: RUNS (default 9), TASKSET (core to pin to, e.g. TASKSET=3; unset = unpinned/multi-core).
set -uo pipefail
a_bin="$1"; b_bin="$2"; shift 2
RUNS=${RUNS:-9}
TASKSET=${TASKSET:-}

run() {
    local bin="$1"; shift
    local cmd=(env LIBGL_ALWAYS_SOFTWARE=1 xvfb-run -a -s "-screen 0 1280x900x24")
    if [ -n "$TASKSET" ]; then
        cmd=(taskset -c "$TASKSET" "${cmd[@]}")
    fi
    "${cmd[@]}" "$bin" --renderer femtovg "$@" 2>/dev/null | awk '
        /^later frames/ {
            v = $3
            if (v ~ /µs$/)      { sub(/µs$/, "", v); print v / 1000 }
            else if (v ~ /ms$/) { sub(/ms$/, "", v); print v + 0 }
            else if (v ~ /s$/)  { sub(/s$/, "", v);  print v * 1000 }
        }'
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
a = sorted(float(x) for x in args[:sep])
b = sorted(float(x) for x in args[sep+1:])
def median(v):
    n = len(v)
    return v[n//2] if n % 2 else (v[n//2-1]+v[n//2])/2
ma, mb = median(a), median(b)
print(f"A median: {ma:.3f} ms   B median: {mb:.3f} ms   delta: {(mb-ma)/ma*100:+.1f}%")
PY
