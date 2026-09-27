#!/usr/bin/env bash
# Headless matrix runner: one row per configuration, N repeats each, median reported with the
# spread so a contaminated run is visible rather than averaged in (see MEASUREMENT.md section 4).
#
#   tools/matrix.sh "label|args" "label|args" ...
#
# Env: RUNS (default 3), FRAMES (default 10), BIN (default the rdebug perf_test in ../perf_test,
# a sibling of this ___OPTIMIZATIONS___ folder - perf_test itself is untouched by this port).
#
# Prints: label | frame time (median of runs) | allocations | paragraph shapes | first frame | RSS
# Allocations and shapes are exactly reproducible within a run, so a spread there means the
# configuration itself is not deterministic - worth knowing before reading anything else.
#
# CAUTION: every column here is a median across runs, and the frame figures are medians *within*
# a run too. For a workload that alternates (something created on even frames, destroyed on odd
# ones), the median reports one side and hides the other entirely - see MEASUREMENT.md section 4
# and candidate 30's `19491`/`25` alternation. Read the raw run whenever the workload is not the
# same on every frame.
set -uo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# here = .../___OPTIMIZATIONS___/tools ; repo root = two levels up ; harness = repo_root/perf_test
repo_root="$(dirname "$(dirname "$here")")"
harness="$repo_root/perf_test"
BIN=${BIN:-"$harness/target/rdebug/perf_test"}
RUNS=${RUNS:-3}
FRAMES=${FRAMES:-10}

if [ ! -x "$BIN" ]; then
    echo "no binary at $BIN - build it with: (cd $harness && cargo build --profile rdebug --bin perf_test)" >&2
    exit 1
fi

# one run -> "time|allocs|shapes|first|rss"
measure() {
    "$BIN" --bench $1 --frames "$FRAMES" 2>/dev/null | awk '
        /^first frame/     { first = $3 }
        /^resident memory/ { rss = $3 }
        /^later frames/    { gsub("[(,]", "", $7); time = $3; allocs = $7; shapes = $9 }
        END { printf "%s|%s|%s|%s|%s\n", time, allocs, shapes, first, rss }'
}

# "1.23ms" / "987.65µs" / "1.23s" -> milliseconds, so medians and ratios work across units
to_ms() {
    awk -v v="$1" 'BEGIN {
        if (v ~ /µs$/)      { sub(/µs$/, "", v); print v / 1000 }
        else if (v ~ /ms$/) { sub(/ms$/, "", v); print v + 0 }
        else if (v ~ /s$/)  { sub(/s$/, "", v);  print v * 1000 }
        else print "nan"
    }'
}

median() { printf '%s\n' "$@" | sort -g | awk '{ a[NR] = $1 } END { printf "%.2f", a[int((NR + 1) / 2)] }'; }

printf "%-34s %10s %10s %9s %8s %11s %9s\n" \
    "config" "frame ms" "spread" "allocs" "shapes" "first frame" "RSS MB"
printf "%-34s %10s %10s %9s %8s %11s %9s\n" \
    "----------------------------------" "----------" "----------" "---------" "--------" "-----------" "---------"

for entry in "$@"; do
    label="${entry%%|*}"
    args="${entry#*|}"
    times=(); allocs_seen=""; shapes_seen=""; first=""; rss=""
    for _ in $(seq 1 "$RUNS"); do
        IFS='|' read -r t a sh f r <<< "$(measure "$args")"
        times+=("$(to_ms "$t")")
        case "|$allocs_seen|" in *"|$a|"*) ;; *) allocs_seen="${allocs_seen:+$allocs_seen|}$a" ;; esac
        case "|$shapes_seen|" in *"|$sh|"*) ;; *) shapes_seen="${shapes_seen:+$shapes_seen|}$sh" ;; esac
        first="$f"; rss="$r"
    done
    lo=$(printf '%s\n' "${times[@]}" | sort -g | head -1)
    hi=$(printf '%s\n' "${times[@]}" | sort -g | tail -1)
    printf "%-34s %10s %10s %9s %8s %11s %9s\n" \
        "$label" "$(median "${times[@]}")" \
        "$(awk -v a="$lo" -v b="$hi" 'BEGIN { printf "%.2f", b - a }')" \
        "${allocs_seen:-n/a}" "${shapes_seen:-n/a}" "${first:-n/a}" "${rss:-n/a}"
done
