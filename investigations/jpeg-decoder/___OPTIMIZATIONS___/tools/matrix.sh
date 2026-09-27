#!/usr/bin/env bash
# Headless matrix runner: one row per workload, N repeats each, median reported with the
# spread so a contaminated run is visible rather than averaged in (see MEASUREMENT.md section 4).
#
#   matrix.sh "label|image_path [iters]" "label|image_path [iters]" ...
#
# Env: RUNS (default 3), BIN (default ./target/release/bench_decode).
set -uo pipefail

BIN=${BIN:-"./target/release/bench_decode"}
RUNS=${RUNS:-3}

if [ ! -x "$BIN" ]; then
    echo "no binary at $BIN - set BIN=... or build it first" >&2
    exit 1
fi

measure() {
    "$BIN" $1 2>/dev/null | awk '/^min/ { print $3 }'
}

median() { printf '%s\n' "$@" | sort -g | awk '{ a[NR] = $1 } END { printf "%.3f", a[int((NR + 1) / 2)] }'; }

printf "%-34s %10s %10s\n" "workload" "median ms" "spread"
printf "%-34s %10s %10s\n" "----------------------------------" "----------" "----------"

for entry in "$@"; do
    label="${entry%%|*}"
    args="${entry#*|}"
    times=()
    for _ in $(seq 1 "$RUNS"); do
        times+=("$(measure "$args")")
    done
    lo=$(printf '%s\n' "${times[@]}" | sort -g | head -1)
    hi=$(printf '%s\n' "${times[@]}" | sort -g | tail -1)
    printf "%-34s %10s %10s\n" \
        "$label" "$(median "${times[@]}")" \
        "$(awk -v a="$lo" -v b="$hi" 'BEGIN { printf "%.3f", b - a }')"
done
