#!/usr/bin/env bash
# Paired A/B comparison for pic-scale-safe capped-thread candidate (023).
# Usage: paired_pic_scale.sh <bin_a> <bin_b> <bench_id> <runs> [taskset_core]
set -uo pipefail

bin_a="$1"; bin_b="$2"; bench_id="$3"; runs="$4"; core="${5:-}"

run_one() {
    local bin="$1"
    local cmd=("$bin" --bench "$bench_id" --exact)
    if [ -n "$core" ]; then
        cmd=(taskset -c "$core" "${cmd[@]}")
    fi
    "${cmd[@]}" 2>/dev/null | awk '
        /time:/ {
            line=$0
            gsub(/.*time:[ \t]*\[/, "", line)
            gsub(/\].*/, "", line)
            n = split(line, parts, " ")
            mid = parts[3]
            val = parts[3] + 0
            unit = parts[4]
            if (unit == "ns") val = val / 1000
            else if (unit == "ms") val = val * 1000
            else if (unit == "s") val = val * 1000000
            print val
        }'
}

a_times=(); b_times=()
for i in $(seq 1 "$runs"); do
    a=$(run_one "$bin_a")
    b=$(run_one "$bin_b")
    a_times+=("$a")
    b_times+=("$b")
    echo "pair $i: A=${a} us  B=${b} us"
done

atmp=$(mktemp); btmp=$(mktemp)
printf '%s\n' "${a_times[@]}" > "$atmp"
printf '%s\n' "${b_times[@]}" > "$btmp"
awk '
    NR == FNR { a[FNR] = $1; n = FNR; next }
    { b[FNR] = $1 }
    END {
        for (i = 1; i <= n; i++) if (b[i] < a[i]) wins++
        asort_a(a, n); asort_a(b, n)
        ma = a[int((n + 1) / 2)]; mb = b[int((n + 1) / 2)]
        printf "median %.3f -> %.3f us (%+.1f%%)   B faster in %d of %d pairs\n",
               ma, mb, 100 * (mb / ma - 1), wins + 0, n
    }
    function asort_a(arr, len,   i, j, t) {
        for (i = 2; i <= len; i++) { t = arr[i]; for (j = i - 1; j >= 1 && arr[j] > t; j--) arr[j+1] = arr[j]; arr[j+1] = t }
    }
' "$atmp" "$btmp"
rm -f "$atmp" "$btmp"
