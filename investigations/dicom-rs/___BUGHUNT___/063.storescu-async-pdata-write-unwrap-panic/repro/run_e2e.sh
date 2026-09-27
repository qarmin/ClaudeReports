#!/usr/bin/env bash
# Real, two-real-participant, real-loopback-TCP reproduction.
#
# Runs the actual `dicom-storescu` binary (in its async/`-c` concurrency mode, which uses
# store_async.rs's vulnerable code path) sending a genuine ~20MB DICOM file to `fake_scp` - a
# minimal but genuine SCP built on `dicom-ul`'s own real, public server association API. The
# fake SCP performs real negotiation, receives the real C-STORE-RQ command, then abruptly drops
# the real TCP connection before reading the P-Data payload, causing storescu's subsequent real
# socket write to genuinely fail with a real OS-level "connection reset" error.
#
# Usage: run_e2e.sh <path-to-clone-dir>
set -euo pipefail

CLONE_DIR="$1"
PORT=24700
HERE="$(cd "$(dirname "$0")" && pwd)"
BIG_FILE="$(mktemp -d)/big.dcm"

cleanup() {
    if [ -n "${SCP_PID:-}" ]; then
        kill "$SCP_PID" 2>/dev/null || true
        wait "$SCP_PID" 2>/dev/null || true
    fi
}
trap cleanup EXIT

cargo build --manifest-path "$CLONE_DIR/Cargo.toml" -p dicom-storescu --all-features >/dev/null
cargo build --manifest-path "$HERE/fake_scp/Cargo.toml" >/dev/null
cargo build --manifest-path "$HERE/make_big_file/Cargo.toml" >/dev/null

STORESCU_BIN="$CLONE_DIR/target/debug/dicom-storescu"
FAKE_SCP_BIN="$HERE/fake_scp/target/debug/fake_scp"
MAKE_BIG_FILE_BIN="$HERE/make_big_file/target/debug/make_big_file"

"$MAKE_BIG_FILE_BIN" "$BIG_FILE"

"$FAKE_SCP_BIN" "$PORT" &
SCP_PID=$!
sleep 1

set +e
OUTPUT="$("$STORESCU_BIN" -v -c 1 "127.0.0.1:$PORT" "$BIG_FILE" 2>&1)"
set -e
echo "$OUTPUT"

if echo "$OUTPUT" | grep -q "panicked at storescu/src/store_async.rs"; then
    echo "BUG REPRODUCED: storescu's async P-Data write panicked"
    exit 1
else
    echo "no panic: storescu handled the connection failure gracefully"
    exit 0
fi
