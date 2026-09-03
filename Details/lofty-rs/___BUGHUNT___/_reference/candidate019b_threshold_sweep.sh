#!/usr/bin/env bash
# ADAPT: one-off sweep for the current candidate (coverage-saturation threshold
# investigation). Not a reusable tool - kept in _reference/ per SKILL.md.
#
# For a given target, build progressively larger random subsets of the
# leftover fuzz/corpus/<target>/ pool and run --sanitizer address (no ulimit
# workaround) against each, recording hang/no-hang, to find where the hang
# reliably stops appearing.
set -uo pipefail

TARGET="${1:-mp4file_read_from}"
POOL="fuzz/corpus/${TARGET}"
SCRATCH=$(mktemp -d)
TIMEOUT_S=30
LIBFUZZER_TIME=15
SIZES=(0 1 3 5 10 20 35 51)
TRIALS_PER_SIZE=2

pool_count=$(ls "$POOL" | wc -l)
echo "== target=$TARGET pool_size=$pool_count scratch=$SCRATCH =="

run_one() {
    local size=$1 trial=$2
    local dir="$SCRATCH/size_${size}_trial_${trial}"
    mkdir -p "$dir"
    if [ "$size" -gt 0 ]; then
        ls "$POOL" | shuf -n "$size" --random-source=<(yes "$size$trial") | while read -r f; do
            cp "$POOL/$f" "$dir/"
        done
    fi
    local actual
    actual=$(ls "$dir" | wc -l)
    local start end rc
    start=$(date +%s)
    timeout -s KILL "$TIMEOUT_S" cargo +nightly fuzz run "$TARGET" --sanitizer address "$dir" \
        -- -max_total_time="$LIBFUZZER_TIME" > "$SCRATCH/log_${size}_${trial}.txt" 2>&1
    rc=$?
    end=$(date +%s)
    local elapsed=$((end - start))
    local last_line
    last_line=$(tail -3 "$SCRATCH/log_${size}_${trial}.txt" | tr '\n' ' ')
    if [ "$rc" -eq 137 ]; then
        echo "size=$size(actual=$actual) trial=$trial HUNG (killed after ${elapsed}s) last=[${last_line}]"
    else
        echo "size=$size(actual=$actual) trial=$trial ok rc=$rc elapsed=${elapsed}s last=[${last_line}]"
    fi
}

for size in "${SIZES[@]}"; do
    if [ "$size" -eq 0 ]; then
        run_one 0 1
    else
        for t in $(seq 1 "$TRIALS_PER_SIZE"); do
            run_one "$size" "$t"
        done
    fi
done

echo "== done, logs in $SCRATCH =="
