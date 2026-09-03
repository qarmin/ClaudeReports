#!/usr/bin/env bash
# Standalone reproduction for candidate 019: does seeding fuzz/corpus/<target>/ with a single
# file reliably avoid the default-ASAN cargo-fuzz hang documented in candidates 009/018?
#
# Candidate 018 observed the hang go away in ONE control run on ONE target
# (mp4file_read_from) using a leftover 23-29 file corpus, and flagged "sweep this across every
# target with a real single-file seed" as an open question. This script does that sweep's core
# check on one representative target (mp4file_read_from by default): seed the corpus with
# exactly one file (a real, non-trivial file, not an empty/degenerate one) and show the ASAN run
# still hangs, refuting the "even one file" workaround as reliable.
#
# This is a tooling/environment interaction, not a bug in lofty's own public API - same
# reasoning as candidate 018's repro. Requires: nightly toolchain, cargo-fuzz, ASAN sanitizer
# runtime (all already required to reproduce candidates 009/018).
#
# Usage: ./run.sh <path-to-lofty-rs-checkout> [target] [seed-file]
set -uo pipefail

REPO="$(cd "${1:?usage: run.sh <path-to-lofty-rs-checkout> [target] [seed-file]}" && pwd)"
TARGET="${2:-mp4file_read_from}"
SEED_FILE="${3:-$REPO/lofty/tests/files/assets/ilst_trailing_padding.m4a}"

cd "$REPO/fuzz" || { echo "no fuzz/ dir in $REPO"; exit 1; }

echo "--- building (untimed) ---"
cargo +nightly fuzz build "$TARGET" --sanitizer address || { echo "ASAN build failed"; exit 1; }
cargo +nightly fuzz build "$TARGET" --sanitizer none || { echo "non-sanitizer build failed"; exit 1; }

CORPUS="corpus/$TARGET"
rm -rf "$CORPUS"
mkdir -p "$CORPUS"
if [ ! -f "$SEED_FILE" ]; then
    echo "seed file $SEED_FILE not found"; exit 1
fi
cp "$SEED_FILE" "$CORPUS/seed1"
echo "seeded $CORPUS/ with exactly 1 file ($(wc -c < "$CORPUS/seed1") bytes): $SEED_FILE"

echo
echo "=== [1/2] cargo +nightly fuzz run $TARGET --sanitizer address, corpus pre-seeded with 1 file ==="
echo "    (workaround under test: does 1 seed file avoid the hang?)"
LOG_ASAN=$(mktemp)
cargo +nightly fuzz run "$TARGET" --sanitizer address -- -max_total_time=15 -rss_limit_mb=4096 >"$LOG_ASAN" 2>&1 &
PID=$!
sleep 45
if kill -0 "$PID" 2>/dev/null; then
    echo "--> still running after 45s wall-clock (max_total_time was only 15s) - HUNG despite the 1-file seed"
    HUNG=1
    pkill -9 -f "target/x86_64-unknown-linux-gnu/release/${TARGET} " 2>/dev/null
    pkill -9 -f "cargo-fuzz fuzz run ${TARGET} " 2>/dev/null
    wait "$PID" 2>/dev/null
else
    echo "--> process exited on its own within 45s - workaround held for this target"
    HUNG=0
fi
echo "----- captured ASAN-build log -----"
cat "$LOG_ASAN"
echo "------------------------------------"

echo
echo "=== [2/2] control: cargo +nightly fuzz run $TARGET --sanitizer none, same 1-file corpus (expect: runs cleanly) ==="
LOG_NONE=$(mktemp)
timeout 40 cargo +nightly fuzz run "$TARGET" --sanitizer none -- -max_total_time=15 -rss_limit_mb=4096 >"$LOG_NONE" 2>&1
NONE_EXIT=$?
echo "----- captured non-sanitizer log (tail) -----"
tail -5 "$LOG_NONE"
echo "------------------------------------"

echo
echo "=== RESULT ==="
if [ "${HUNG:-0}" = "1" ] && [ "$NONE_EXIT" = "0" ]; then
    echo "CONFIRMED: $TARGET still hangs under --sanitizer address despite a 1-file pre-seeded corpus (--sanitizer none control ran cleanly on the identical corpus) - the 'seed with even one file' workaround from candidate 018 does NOT reliably avoid the hang."
elif [ "${HUNG:-0}" = "0" ]; then
    echo "NOT REPRODUCED on $TARGET this run: the 1-file seed avoided the hang here (this can happen - see RESULT.md, e.g. speexfile_read_from) - re-run or try another target/seed to see the full sweep picture."
else
    echo "INCONCLUSIVE: ASAN run hung but the --sanitizer none control did not complete cleanly either (exit $NONE_EXIT) - check $LOG_NONE."
fi
