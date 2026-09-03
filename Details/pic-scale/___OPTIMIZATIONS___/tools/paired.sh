#!/usr/bin/env bash
# Compare two binaries by alternating runs, and report how many pairs went each way.
#
#   paired.sh <binary A> <binary B> -- <bench args...>
#
# A median over N runs of A followed by N runs of B is not a comparison - some machines land in
# one of several performance modes per process and hold it for the whole run, surviving `taskset`
# pinning. Two unpaired medians then compare two lotteries, not two binaries. See MEASUREMENT.md
# section 2b - this script exists because that rule, written down in prose, kept getting skipped
# anyway.
#
# Alternating A,B,A,B... cancels drift, and the *pair count* is what carries the conclusion: 9/9
# is decisive at effect sizes well under the spread; 5/9 means nothing however the medians land.
#
# ADAPT: the `run()` function below shells out to the bench binary and must print ONE number
# (milliseconds) to stdout per invocation - replace its body with whatever this project's bench
# command and output format actually are.
#
# Env: RUNS (default 9).
set -uo pipefail

if [ "$#" -lt 3 ]; then
    sed -n '2,15p' "$0" >&2
    exit 2
fi

a_bin="$1"; b_bin="$2"; shift 2
[ "${1:-}" = "--" ] && shift
RUNS=${RUNS:-9}

for bin in "$a_bin" "$b_bin"; do
    [ -x "$bin" ] || { echo "not executable: $bin" >&2; exit 1; }
done

# Two binaries that are byte-identical cannot show a difference - and silently reporting 0% for a
# patch that failed to apply is a real failure mode. Check before spending the runs.
if [ "$(md5sum "$a_bin" "$b_bin" | awk '{print $1}' | sort -u | wc -l)" -eq 1 ]; then
    echo "refusing to compare: the two binaries are identical (did the patch apply?)" >&2
    exit 1
fi

# ---- ADAPT: one run -> one number in milliseconds, printed to stdout ----
# pic-scale's criterion benches (harness = false) print one line like:
#   time:   [51.829 ms 53.574 ms 55.652 ms]
# Take the middle (point-estimate) value, unit-normalized to ms. Set PIN=<core> to taskset.
run() {
    local bin="$1"; shift
    local cmd=("$bin" "$@")
    if [ -n "${PIN:-}" ]; then
        cmd=(taskset -c "$PIN" "$bin" "$@")
    fi
    "${cmd[@]}" 2>/dev/null | awk '
        /time:/ {
            gsub(/[][]/, "")
            v = $4  # point estimate is the middle of [low mid high]; $3/$5 are the unit tokens
            unit = $5
            if (unit == "µs")      { print v / 1000 }
            else if (unit == "ms") { print v + 0 }
            else if (unit == "s")  { print v * 1000 }
            else if (unit == "ns") { print v / 1000000 }
        }'
}
# ---------------------------------------------------------------------------

a_times=(); b_times=()
for _ in $(seq 1 "$RUNS"); do
    a_times+=("$(run "$a_bin" "$@")")
    b_times+=("$(run "$b_bin" "$@")")
done

printf 'A: %s\n' "${a_times[*]}"
printf 'B: %s\n' "${b_times[*]}"

atmp="$(mktemp)"; btmp="$(mktemp)"
trap 'rm -f "$atmp" "$btmp"' EXIT
printf '%s\n' "${a_times[@]}" > "$atmp"
printf '%s\n' "${b_times[@]}" > "$btmp"
awk '
    NR == FNR { a[FNR] = $1; n = FNR; next }
    { b[FNR] = $1 }
    END {
        for (i = 1; i <= n; i++) if (b[i] < a[i]) wins++
        asort_a(a, n); asort_a(b, n)
        ma = a[int((n + 1) / 2)]; mb = b[int((n + 1) / 2)]
        printf "median %.3f -> %.3f  (%+.1f%%)   B faster in %d of %d pairs\n",
               ma, mb, 100 * (mb / ma - 1), wins + 0, n
        if (wins >= n - 1 || wins <= 1) print "verdict: consistent"
        else print "verdict: NOT consistent - the medians are not evidence"
    }
    function asort_a(arr, len,   i, j, t) {
        for (i = 2; i <= len; i++) { t = arr[i]; for (j = i - 1; j >= 1 && arr[j] > t; j--) arr[j+1] = arr[j]; arr[j+1] = t }
    }
' "$atmp" "$btmp"
