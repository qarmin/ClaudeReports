#!/usr/bin/env bash
# Compare two binaries by alternating runs, and report how many pairs went each way.
#
#   tools/paired.sh <binary A> <binary B> -- <scene args...>
#
# A median over N runs of A followed by N runs of B is not a comparison on this hardware. The same
# binary, on the same deterministic input, lands in one of two performance modes and holds it for the
# whole process - ~18.3 ms or ~24.5 ms per frame on the machine this was written on, with nothing in
# between, surviving `taskset` pinning. Two unpaired medians therefore compare two lotteries.
#
# Alternating A,B,A,B... cancels drift, and the *pair count* is what carries the conclusion: 9/9 is
# decisive at effect sizes well under the spread, while 5/9 means nothing however the medians land.
#
# This has produced three wrong conclusions in this workspace - a real -12% dismissed as a wash, a
# neutral change reported as -19%, and a real parameter dependency written off as noise.
#
# Env: RUNS (default 9), FRAMES (default 20), BIN args are the two binaries to compare.
set -uo pipefail

if [ "$#" -lt 3 ]; then
    sed -n '2,18p' "$0" >&2
    exit 2
fi

a_bin="$1"; b_bin="$2"; shift 2
[ "${1:-}" = "--" ] && shift
RUNS=${RUNS:-9}
FRAMES=${FRAMES:-20}

for bin in "$a_bin" "$b_bin"; do
    [ -x "$bin" ] || { echo "not executable: $bin" >&2; exit 1; }
done

# Two binaries that are byte-identical cannot show a difference, and silently reporting 0% for a
# patch that failed to apply has happened here. Check before spending the runs.
if [ "$(md5sum "$a_bin" "$b_bin" | awk '{print $1}' | sort -u | wc -l)" -eq 1 ]; then
    echo "refusing to compare: the two binaries are identical (did the patch apply?)" >&2
    exit 1
fi

# one run -> milliseconds
run() {
    "$1" --bench "${@:2}" --frames "$FRAMES" 2>/dev/null | awk '
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

printf 'A: %s\n' "${a_times[*]}"
printf 'B: %s\n' "${b_times[*]}"

printf '%s\n' "${a_times[@]}" > /tmp/paired_a.$$
printf '%s\n' "${b_times[@]}" > /tmp/paired_b.$$
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
' /tmp/paired_a.$$ /tmp/paired_b.$$
rm -f /tmp/paired_a.$$ /tmp/paired_b.$$
