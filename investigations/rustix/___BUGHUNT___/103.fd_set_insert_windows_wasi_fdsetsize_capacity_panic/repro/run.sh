#!/usr/bin/env bash
# Runs the real, unmodified `rustix::event::fd_set_insert` public API, cross-interpreted
# under Miri for both affected targets (wasm32-wasip1 and x86_64-pc-windows-gnu), filling an
# FD_SETSIZE-capacity set and then inserting one more, not-yet-present fd.
#
# Expects $1 to be either "before" (bug should reproduce) or "after" (bug should be fixed).
set -euo pipefail
cd "$(dirname "$0")"

mode="${1:?usage: run.sh before|after}"

rustup target add wasm32-wasip1 x86_64-pc-windows-gnu >/dev/null 2>&1 || true
rustup component add miri --toolchain nightly >/dev/null 2>&1 || true

echo "=== wasm32-wasip1 (FD_SETSIZE=1024) ==="
set +e
cargo +nightly miri run --target wasm32-wasip1 >wasi.log 2>&1
wasi_exit=$?
set -e
tail -5 wasi.log
echo "wasi exit code: $wasi_exit"

echo
echo "=== x86_64-pc-windows-gnu (FD_SETSIZE=64) ==="
set +e
cargo +nightly miri run --target x86_64-pc-windows-gnu >win.log 2>&1
win_exit=$?
set -e
tail -5 win.log
echo "windows exit code: $win_exit"

echo
if [ "$mode" = "before" ]; then
    # Bug present: wasi aborts (nonzero, cargo-miri reports "aborting due to
    # previous error"), windows's catch_unwind cleanly reports exit 12.
    if [ "$wasi_exit" -ne 0 ] && [ "$win_exit" -eq 12 ]; then
        echo "RESULT: bug reproduces on both targets, as expected (before fix)"
        exit 0
    else
        echo "RESULT: bug did NOT reproduce as expected (wasi_exit=$wasi_exit, win_exit=$win_exit)"
        exit 1
    fi
else
    if [ "$wasi_exit" -eq 0 ] && [ "$win_exit" -eq 0 ]; then
        echo "RESULT: no panic on either target, fix verified (after fix)"
        exit 0
    else
        echo "RESULT: bug still reproduces after the fix (wasi_exit=$wasi_exit, win_exit=$win_exit)"
        exit 1
    fi
fi
