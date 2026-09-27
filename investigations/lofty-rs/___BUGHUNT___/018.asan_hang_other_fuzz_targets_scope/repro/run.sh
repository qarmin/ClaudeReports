#!/usr/bin/env bash
# Standalone reproduction for candidate 018: does the default-ASAN cargo-fuzz hang seen for
# candidate 009 (picture_from_ape_bytes/picture_from_flac_bytes) also affect the project's
# larger *file_read_from fuzz targets?
#
# This is a tooling/environment interaction, not a bug in lofty's own public API, so the
# "repro" here is the real command a maintainer would actually run (cargo +nightly fuzz run
# against the project's own existing fuzz/fuzz_targets/*.rs harnesses), not a hand-rolled
# Cargo crate standing in for it - there is no library API call that would demonstrate this on
# its own. Requires: nightly toolchain, cargo-fuzz, and (only for the --sanitizer address leg)
# the ASAN sanitizer runtime, all already required to reproduce candidate 009.
#
# Usage: ./run.sh <path-to-lofty-rs-checkout>
set -uo pipefail

REPO="${1:?usage: run.sh <path-to-lofty-rs-checkout>}"
TARGET="${2:-mp4file_read_from}"
cd "$REPO/fuzz" || { echo "no fuzz/ dir in $REPO"; exit 1; }

# Build each variant before timing the run - on a fresh clone, compiling lofty + the fuzz harness
# itself can take well over the 30s wait window below, which would otherwise be misread as a hang.
echo "--- building (untimed) ---"
cargo +nightly fuzz build "$TARGET" --sanitizer address || { echo "ASAN build failed"; exit 1; }
cargo +nightly fuzz build "$TARGET" --sanitizer none || { echo "non-sanitizer build failed"; exit 1; }

echo "=== [1/2] cargo +nightly fuzz run $TARGET --sanitizer address (expect: hang, no libFuzzer progress) ==="
LOG_ASAN=$(mktemp)
cargo +nightly fuzz run "$TARGET" --sanitizer address -- -max_total_time=20 >"$LOG_ASAN" 2>&1 &
PID=$!
sleep 30
if kill -0 "$PID" 2>/dev/null; then
    echo "--> still running after 30s wall-clock (max_total_time was only 20s) - HUNG"
    HUNG=1
    pkill -9 -f "target/x86_64-unknown-linux-gnu/release/${TARGET} " 2>/dev/null
    pkill -9 -f "cargo-fuzz fuzz run ${TARGET} " 2>/dev/null
    wait "$PID" 2>/dev/null
else
    echo "--> process exited on its own within 30s"
    HUNG=0
fi
echo "----- captured ASAN-build log -----"
cat "$LOG_ASAN"
echo "------------------------------------"

echo
echo "=== [2/2] cargo +nightly fuzz run $TARGET --sanitizer none (expect: runs cleanly, no hang) ==="
LOG_NONE=$(mktemp)
timeout 40 cargo +nightly fuzz run "$TARGET" --sanitizer none -- -max_total_time=15 >"$LOG_NONE" 2>&1
NONE_EXIT=$?
echo "----- captured non-sanitizer log (tail) -----"
tail -5 "$LOG_NONE"
echo "------------------------------------"

echo
echo "=== RESULT ==="
if [ "${HUNG:-0}" = "1" ] && [ "$NONE_EXIT" = "0" ]; then
    echo "CONFIRMED: $TARGET hangs under default --sanitizer address but runs cleanly under --sanitizer none - same tooling gap as candidate 009, now shown on a *file_read_from target."
elif [ "${HUNG:-0}" = "0" ]; then
    echo "NOT REPRODUCED on $TARGET: ASAN build did not hang within the wait window."
else
    echo "INCONCLUSIVE: ASAN build hung but the --sanitizer none control run did not complete cleanly either (exit $NONE_EXIT) - check $LOG_NONE."
fi
