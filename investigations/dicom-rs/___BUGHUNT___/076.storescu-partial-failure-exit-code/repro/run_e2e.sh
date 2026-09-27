#!/usr/bin/env bash
# Real, two-real-participant, real-loopback-TCP reproduction.
#
# Runs the actual `dicom-storescu` binary against `fake_scp` - a minimal but genuine SCP built
# on `dicom-ul`'s own real, public server association API - communicating over a real loopback
# TCP socket. `fake_scp` performs real negotiation, receives the real C-STORE-RQ, and sends back
# a real, legitimately-encoded C-STORE-RSP with a genuine DICOM failure status (0xA700). Checks
# the real `dicom-storescu` process's own exit code.
#
# Usage: run_e2e.sh <path-to-clone-dir>
set -euo pipefail

CLONE_DIR="$1"
PORT=24900
HERE="$(cd "$(dirname "$0")" && pwd)"
SMALL_FILE="$(mktemp -d)/small.dcm"

cleanup() {
    if [ -n "${SCP_PID:-}" ]; then
        kill "$SCP_PID" 2>/dev/null || true
        wait "$SCP_PID" 2>/dev/null || true
    fi
}
trap cleanup EXIT

cargo build --manifest-path "$CLONE_DIR/Cargo.toml" -p dicom-storescu >/dev/null
cargo build --manifest-path "$HERE/fake_scp/Cargo.toml" >/dev/null
cargo build --manifest-path "$HERE/make_small_file/Cargo.toml" >/dev/null

STORESCU_BIN="$CLONE_DIR/target/debug/dicom-storescu"
FAKE_SCP_BIN="$HERE/fake_scp/target/debug/fake_scp"
MAKE_SMALL_FILE_BIN="$HERE/make_small_file/target/debug/make_small_file"

"$MAKE_SMALL_FILE_BIN" "$SMALL_FILE"

"$FAKE_SCP_BIN" "$PORT" &
SCP_PID=$!
sleep 1

set +e
"$STORESCU_BIN" "127.0.0.1:$PORT" "$SMALL_FILE"
STORESCU_EXIT=$?
set -e

echo "dicom-storescu exit code: $STORESCU_EXIT"

if [ "$STORESCU_EXIT" -eq 0 ]; then
    echo "BUG REPRODUCED: storescu exited 0 (success) despite the C-STORE genuinely failing"
    exit 1
else
    echo "no bug: storescu correctly exited non-zero on the C-STORE failure"
    exit 0
fi
