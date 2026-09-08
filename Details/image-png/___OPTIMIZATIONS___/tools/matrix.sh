#!/usr/bin/env bash
# Headless matrix runner: one row per configuration, N repeats each, median reported with the
# spread so a contaminated run is visible rather than averaged in (see MEASUREMENT.md section 4).
#
#   matrix.sh "label|args" "label|args" ...
#
# Env: RUNS (default 3), BIN (default: adapt below).
#
# CAUTION: every number here is a median. For a workload that alternates (something created on
# even frames, destroyed on odd ones), the median reports one side and hides the other entirely -
# a scene allocating 19491 on half its frames and 25 on the rest reads as "25". A wide `spread`
# column is the tell; read the raw run, not this table, whenever the workload is not the same on
# every iteration.
#
# ADAPT: BIN's default path, and the `measure()` function's parsing of that binary's output.
set -uo pipefail

BIN=${BIN:-"./target/release/encoder_bench"}
RUNS=${RUNS:-3}

if [ ! -x "$BIN" ]; then
    echo "no binary at $BIN - set BIN=... or build it first" >&2
    exit 1
fi

# ---- ADAPT: one run -> "time|extra1|extra2|..." ----
# encoder_bench: extra1=allocs, extra2=reallocs, from the LAST `RESULT ...` line printed (pass
# `--only <substring>` in $1's args to pin a run to exactly one configuration).
measure() {
    "$BIN" $1 2>/dev/null | awk '
        /^RESULT / {
            time = "n/a"; e1 = "n/a"; e2 = "n/a"
            for (i = 1; i <= NF; i++) {
                if ($i ~ /^p50=/)      { v = $i; sub(/^p50=/, "", v); time = v }
                if ($i ~ /^allocs=/)   { v = $i; sub(/^allocs=/, "", v); e1 = v }
                if ($i ~ /^reallocs=/) { v = $i; sub(/^reallocs=/, "", v); e2 = v }
            }
        }
        END { printf "%s|%s|%s\n", time, e1, e2 }'
}
# -----------------------------------------------------

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

printf "%-34s %10s %10s %9s %9s\n" "config" "median ms" "spread" "extra1" "extra2"
printf "%-34s %10s %10s %9s %9s\n" "----------------------------------" "----------" "----------" "---------" "---------"

for entry in "$@"; do
    label="${entry%%|*}"
    args="${entry#*|}"
    times=(); e1_seen=""; e2_seen=""
    for _ in $(seq 1 "$RUNS"); do
        IFS='|' read -r t e1 e2 <<< "$(measure "$args")"
        times+=("$(to_ms "$t")")
        # keep every distinct value so non-determinism shows up as "1502/1677" instead of hiding it
        case "|$e1_seen|" in *"|$e1|"*) ;; *) e1_seen="${e1_seen:+$e1_seen|}$e1" ;; esac
        case "|$e2_seen|" in *"|$e2|"*) ;; *) e2_seen="${e2_seen:+$e2_seen|}$e2" ;; esac
    done
    lo=$(printf '%s\n' "${times[@]}" | sort -g | head -1)
    hi=$(printf '%s\n' "${times[@]}" | sort -g | tail -1)
    printf "%-34s %10s %10s %9s %9s\n" \
        "$label" "$(median "${times[@]}")" \
        "$(awk -v a="$lo" -v b="$hi" 'BEGIN { printf "%.2f", b - a }')" \
        "${e1_seen:-n/a}" "${e2_seen:-n/a}"
done
