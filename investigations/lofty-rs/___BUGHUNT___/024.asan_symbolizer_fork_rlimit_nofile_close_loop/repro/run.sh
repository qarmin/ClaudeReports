#!/usr/bin/env bash
# Standalone reproduction for candidate 024: pins the exact call the default-ASAN cargo-fuzz hang
# (candidates 009/018/019) blocks on in this sandbox, and demonstrates a real fix.
#
# Root cause (see RESULT.md): this sandbox's RLIMIT_NOFILE (soft AND hard) is set to 2147483584.
# ASAN's SymbolizerProcess::StartSymbolizerSubprocess() forks to launch llvm-symbolizer, and in
# the forked child, before execve(), compiler-rt's StartSubprocess() loops calling close() on
# every fd number from 3 up to the process's RLIMIT_NOFILE soft limit. With that limit at ~2.1
# billion, the loop never finishes in practice, so execve() is never reached, so the parent's
# read() on the communication pipe (SymbolizerProcess::ReadFromSymbolizer) blocks forever - which
# is the exact hang candidates 009/018/019 observed behaviorally (always stuck at a NEW_FUNC
# coverage-symbolization event).
#
# Fix under test: lower RLIMIT_NOFILE (ulimit -Sn) to something reasonable BEFORE invoking
# `cargo fuzz run --sanitizer address`. This is a strictly better workaround than the project's
# existing --sanitizer none advice: it keeps ASAN instrumentation instead of disabling it.
#
# This is a tooling/environment interaction, not a bug in lofty's own public API - same
# reasoning as candidates 018/019. Requires: nightly toolchain, cargo-fuzz, ASAN sanitizer
# runtime (all already required to reproduce candidates 009/018/019). gdb is used opportunistically
# to capture a real backtrace of the hang if available, but the script does not depend on it.
#
# Usage: ./run.sh <path-to-lofty-rs-checkout> [target]
set -uo pipefail

REPO="$(cd "${1:?usage: run.sh <path-to-lofty-rs-checkout> [target]}" && pwd)"
TARGET="${2:-mp4file_read_from}"

cd "$REPO/fuzz" || { echo "no fuzz/ dir in $REPO"; exit 1; }

echo "--- environment ---"
echo "RLIMIT_NOFILE (soft): $(ulimit -Sn)"
echo "RLIMIT_NOFILE (hard): $(ulimit -Hn)"
echo

echo "--- building $TARGET with --sanitizer address (untimed) ---"
cargo +nightly fuzz build "$TARGET" --sanitizer address || { echo "ASAN build failed"; exit 1; }

BIN="$REPO/target/x86_64-unknown-linux-gnu/release/$TARGET"
if [ ! -x "$BIN" ]; then
    echo "built binary not found at $BIN"; exit 1
fi

CORPUS_A="$(mktemp -d)"
CORPUS_B="$(mktemp -d)"

echo
echo "=== [1/3] baseline: run $TARGET under ASAN with the sandbox's real (huge) ulimit, empty corpus ==="
echo "    (expected: hangs - stuck at the first NEW_FUNC coverage-symbolization event)"
timeout 30 "$BIN" -max_total_time=15 "$CORPUS_A" > /tmp/024_baseline.log 2>&1
BASELINE_EXIT=$?
echo "exit code: $BASELINE_EXIT (124 = killed by the 30s wall-clock timeout, i.e. hung)"
tail -6 /tmp/024_baseline.log

if command -v gdb >/dev/null 2>&1; then
    echo
    echo "--- capturing a real gdb backtrace of the hang (best-effort, requires ptrace on a direct child - works in most sandboxes since gdb spawns the target itself) ---"
    CORPUS_GDB="$(mktemp -d)"
    nohup gdb -batch \
      -ex "set pagination off" \
      -ex "set follow-fork-mode child" \
      -ex "set detach-on-fork off" \
      -ex "run -max_total_time=40 $CORPUS_GDB" \
      -ex "inferior 2" \
      -ex "bt" \
      -ex "quit" \
      "$BIN" > /tmp/024_gdb.log 2>&1 < /dev/null &
    GDBPID=$!
    disown 2>/dev/null
    sleep 10
    if kill -0 "$GDBPID" 2>/dev/null; then
        kill -INT "$GDBPID"
        sleep 5
    fi
    pkill -9 -f "$BIN" 2>/dev/null
    kill -9 "$GDBPID" 2>/dev/null
    echo "--- gdb backtrace of the hung symbolizer-fork child (inferior 2) ---"
    grep -A 20 "received signal SIGINT" /tmp/024_gdb.log || cat /tmp/024_gdb.log
    rm -rf "$CORPUS_GDB"
else
    echo "(gdb not available - skipping backtrace capture, baseline hang above still demonstrates the bug)"
fi

echo
echo "=== [2/3] fix under test: same target, same empty corpus, ASAN, but with ulimit -Sn lowered first ==="
LOG_FIX=$(mktemp)
(
    ulimit -Sn 65536
    timeout 30 "$BIN" -max_total_time=15 "$CORPUS_B"
) > "$LOG_FIX" 2>&1
FIX_EXIT=$?
echo "exit code: $FIX_EXIT (0 = completed cleanly, no hang)"
tail -8 "$LOG_FIX"

rm -rf "$CORPUS_A" "$CORPUS_B"

echo
echo "=== [3/3] RESULT ==="
if [ "$BASELINE_EXIT" = "124" ] && [ "$FIX_EXIT" = "0" ]; then
    echo "CONFIRMED: $TARGET hangs under --sanitizer address with this sandbox's real RLIMIT_NOFILE (baseline exit 124 = killed while hung), and lowering RLIMIT_NOFILE with 'ulimit -Sn 65536' before the same command makes it run to completion cleanly (exit 0) with full ASAN instrumentation still active."
elif [ "$BASELINE_EXIT" != "124" ]; then
    echo "NOT REPRODUCED this run: the baseline (real ulimit) run did not hang (exit $BASELINE_EXIT) - the sandbox this is running in may have a lower RLIMIT_NOFILE than the one this was found in (2147483584); check 'ulimit -Hn' above."
else
    echo "INCONCLUSIVE: baseline hung as expected but the ulimit-lowered run did not complete cleanly either (exit $FIX_EXIT) - check $LOG_FIX."
fi
