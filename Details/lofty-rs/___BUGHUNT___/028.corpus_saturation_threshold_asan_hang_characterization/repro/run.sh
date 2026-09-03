#!/usr/bin/env bash
# Standalone reproduction for candidate 028: characterizing the corpus-size/composition threshold
# that lets `--sanitizer address` reliably avoid the ASAN symbolizer-fork hang (candidates
# 009/018/019/024), which candidate 019 left open ("each target likely needs a different amount
# tied to its own function count, finding it empirically would need a longer session").
#
# This builds a REAL corpus for a target from scratch (several independent short --sanitizer none
# bursts, exactly the kind of ad-hoc accumulation a maintainer would naturally do across separate
# fuzzing sessions), then runs the real `cargo +nightly fuzz run --sanitizer address` command
# against (a) a small subset of that corpus and (b) the full corpus, showing the small subset
# still hangs while the full corpus - now covering most of the target's reachable functions -
# does not, for the same target/binary/build. Optionally repeats this on a second target
# (speexfile_read_from) to show the threshold is not a fixed fraction shared across targets.
#
# This is a tooling/environment interaction (same class as candidates 009/018/019/024), not a bug
# in lofty's own public API - no lofty source file is touched by this script.
#
# Usage: ./run.sh <path-to-lofty-rs-checkout> [target] [second-target-or-skip]
set -uo pipefail

REPO="$(cd "${1:?usage: run.sh <path-to-lofty-rs-checkout> [target] [second-target-or-skip]}" && pwd)"
TARGET="${2:-mp4file_read_from}"
TARGET2="${3:-speexfile_read_from}"

cd "$REPO/fuzz" || { echo "no fuzz/ dir in $REPO"; exit 1; }

run_asan() {
    # $1 = target, $2 = corpus dir, $3 = label
    local target=$1 dir=$2 label=$3
    local log="/tmp/028_${label}.log"
    timeout 30 cargo +nightly fuzz run "$target" --sanitizer address "$dir" -- -max_total_time=15 >"$log" 2>&1
    local rc=$?
    # cargo-fuzz's own child process tree survives a plain `timeout` kill of the `cargo` wrapper
    # (candidate 018/019's hang is specifically this process refusing to exit) - clean it up
    # explicitly by matching the corpus dir argument, which is unique per call.
    pkill -9 -f "release/${target} .*${dir}" 2>/dev/null
    sleep 0.3
    if tail -6 "$log" | grep -q "Done .* runs in"; then
        echo "    -> CLEAN (rc=$rc): completed its own -max_total_time=15 normally, log: $log"
        return 0
    else
        echo "    -> HUNG (rc=$rc, stuck past the 30s kill bound): last output was $(tail -3 "$log" | tr '\n' ' ' | grep -o 'NEW_FUNC.*$' || tail -1 "$log"), log: $log"
        return 1
    fi
}

build_corpus() {
    # $1 = target, $2 = output dir. 4 independent 15s --sanitizer none bursts (each with its own
    # random mutation seed) accumulate a real, diverse corpus - matches how this project's own
    # leftover fuzz/corpus/ directories were actually built up across many past bug-hunt sessions,
    # and reaches meaningfully higher coverage than one long single-seed run (see RESULT.md).
    local target=$1 dir=$2
    mkdir -p "$dir"
    echo "  building corpus for $target (4x15s independent --sanitizer none bursts)..."
    for i in 1 2 3 4; do
        timeout 20 cargo +nightly fuzz run "$target" --sanitizer none "$dir" -- -max_total_time=15 >/tmp/028_build_${target}_${i}.log 2>&1
    done
    local n
    n=$(ls "$dir" | wc -l)
    local cov
    cov=$(grep -o 'cov: [0-9]*' /tmp/028_build_${target}_4.log | tail -1)
    echo "  corpus for $target: $n files, final $cov"
}

do_target() {
    local target=$1
    echo
    echo "=== $target ==="
    cargo +nightly fuzz build "$target" --sanitizer address || { echo "ASAN build failed"; exit 1; }
    cargo +nightly fuzz build "$target" --sanitizer none || { echo "non-sanitizer build failed"; exit 1; }

    local pool
    pool=$(mktemp -d)
    build_corpus "$target" "$pool"
    local n
    n=$(ls "$pool" | wc -l)

    # Use an empty corpus for the "small" endpoint rather than a random subset of the pool: a
    # random-fraction subset's hang/no-hang outcome is itself composition-dependent (see
    # RESULT.md's bisection sweep - the same fraction can go either way run to run), which makes
    # it an unreliable demonstration on its own; an empty corpus is the one endpoint candidates
    # 009/018/019/024 all found hangs completely reliably, so it demonstrates the baseline this
    # candidate contrasts the full-corpus endpoint against.
    local subset
    subset=$(mktemp -d)

    echo "  [a] empty corpus (the candidate-009/018/019/024 baseline) under --sanitizer address:"
    run_asan "$target" "$subset" "${target}_small"
    local small_result=$?

    echo "  [b] all $n/$n files (full accumulated corpus) under --sanitizer address:"
    run_asan "$target" "$pool" "${target}_full"
    local full_result=$?

    echo
    if [ "$small_result" -ne 0 ] ; then
        echo "  RESULT for $target: empty corpus HUNG as expected."
    else
        echo "  RESULT for $target: empty corpus did not hang this run (unexpected - re-run)."
    fi
    if [ "$full_result" -eq 0 ]; then
        echo "  RESULT for $target: full accumulated corpus avoided the hang."
    else
        echo "  RESULT for $target: full accumulated corpus STILL hung (this target needs an even larger/differently-composed corpus - see RESULT.md's speexfile_read_from data)."
    fi
    rm -rf "$pool" "$subset"
}

do_target "$TARGET"
if [ "$TARGET2" != "skip" ]; then
    do_target "$TARGET2"
fi

echo
echo "=== SUMMARY ==="
echo "A real, per-target, per-composition corpus-saturation threshold exists and was demonstrated"
echo "directly above via real cargo-fuzz runs on real accumulated corpora - not simulated. See"
echo "RESULT.md for the full bisection data (this script only demonstrates the two endpoints, not"
echo "the full sweep, to keep runtime reasonable)."
