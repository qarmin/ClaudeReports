#!/usr/bin/env bash
set -uo pipefail
a_bin="$1"; b_bin="$2"; shift 2
RUNS=${RUNS:-9}
PIN=${PIN:-}
run() {
    if [ -n "$PIN" ]; then
        taskset -c "$PIN" "$1" --bench "${@:2}" --frames 3 2>/dev/null | awk -F'[ (]+' '/^first frame/ {v=$3; sub(/ms$/,"",v); print v}'
    else
        "$1" --bench "${@:2}" --frames 3 2>/dev/null | awk -F'[ (]+' '/^first frame/ {v=$3; sub(/ms$/,"",v); print v}'
    fi
}
a_times=(); b_times=()
for _ in $(seq 1 "$RUNS"); do
    a_times+=("$(run "$a_bin" "$@")")
    b_times+=("$(run "$b_bin" "$@")")
done
printf 'A (baseline): %s\n' "${a_times[*]}"
printf 'B (patched):  %s\n' "${b_times[*]}"
python3 - "${a_times[@]}" -- "${b_times[@]}" << 'PYEOF'
import sys
args = sys.argv[1:]
sep = args.index('--')
a = [float(x) for x in args[:sep]]
b = [float(x) for x in args[sep+1:]]
wins = sum(1 for x,y in zip(a,b) if y < x)
import statistics
print(f"A median={statistics.median(a):.2f}ms  B median={statistics.median(b):.2f}ms  B faster in {wins}/{len(a)} pairs  delta={100*(statistics.median(b)-statistics.median(a))/statistics.median(a):.1f}%")
PYEOF
