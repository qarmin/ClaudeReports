#!/usr/bin/env bash
# classify.sh <fuzzer_binary> <crash_dir>: bucket each artifact by ASAN error kind + first libkmod-index.c frame.
set -uo pipefail
ulimit -c 0
export ASAN_OPTIONS=detect_leaks=0
for f in "$2"/*; do
    out=$(timeout 10 "$1" -timeout=3 "$f" 2>&1)
    kind=$(grep -m1 -oE 'ERROR: (AddressSanitizer|libFuzzer): [a-z-]+|runtime error: [^:]+' <<<"$out" | sed 's/ERROR: //')
    frame=$(grep -m1 -oE 'in [a-z_0-9]+ [^ ]*libkmod-index.c:[0-9]+' <<<"$out" | sed 's|[^ ]*/libkmod/||')
    echo "${kind:-none} | ${frame:-?} $(basename "$f")"
done | awk -F' ' '{k=$0; sub(/ [^ ]+$/,"",k); c[k]++; ex[k]=$NF} END {for (k in c) printf "%5d  %s  (e.g. %s)\n", c[k], k, ex[k]}' | sort -rn
