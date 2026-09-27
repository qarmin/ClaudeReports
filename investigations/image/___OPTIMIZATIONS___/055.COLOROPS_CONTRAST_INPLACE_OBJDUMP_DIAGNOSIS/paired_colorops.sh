#!/usr/bin/env bash
# Paired comparison for two prebuilt imageops criterion bench binaries, for one bench ID.
# Usage: paired_colorops.sh <A_bin> <B_bin> <bench_id> <mode: single|multi> <n_pairs>
set -uo pipefail

a_bin="$1"; b_bin="$2"; bench_id="$3"; mode="$4"; n="${5:-8}"

run() {
    local bin="$1"
    if [ "$mode" = "single" ]; then
        taskset -c 3 "$bin" --bench "$bench_id" --exact --warm-up-time 1 --measurement-time 1.2 2>/dev/null
    else
        "$bin" --bench "$bench_id" --exact --warm-up-time 1 --measurement-time 1.2 2>/dev/null
    fi
}

extract_ms() {
    # parses criterion's "time:   [X Xu Y Yu Z Zu]" line, returns the middle (Y Yu) value in ms
    python3 -c "
import sys, re
for line in sys.stdin:
    m = re.search(r'time:\s+\[\S+ \S+ (\S+) (\S+) \S+ \S+\]', line)
    if m:
        val = float(m.group(1)); unit = m.group(2)
        if unit == 'ms': print(val)
        elif unit in ('us', 'µs'): print(val / 1000)
        elif unit == 'ns': print(val / 1000000)
        elif unit == 's': print(val * 1000)
        else: print('nan')
        break
else:
    print('nan')
"
}

echo "=== $bench_id $mode ($a_bin vs $b_bin) ==="
a_times=(); b_times=()
for i in $(seq 1 "$n"); do
    a_out="$(run "$a_bin")"
    a_val="$(echo "$a_out" | extract_ms)"
    b_out="$(run "$b_bin")"
    b_val="$(echo "$b_out" | extract_ms)"
    echo "A[$i]=$a_val ms  B[$i]=$b_val ms"
    a_times+=("$a_val")
    b_times+=("$b_val")
done

python3 -c "
a = [$(IFS=,; echo "${a_times[*]}")]
b = [$(IFS=,; echo "${b_times[*]}")]
wins = sum(1 for x,y in zip(a,b) if y < x)
import statistics
ma = statistics.median(a); mb = statistics.median(b)
print(f'median A={ma:.4f} B={mb:.4f}  ratio B/A={mb/ma:.3f}  B faster in {wins}/{len(a)} pairs')
"
