#!/usr/bin/env bash
# Real, two-real-participant, real-loopback-TCP reproduction.
#
# Runs the actual `dicom-echoscu` binary as a real C-ECHO SCU against `fake_scp` - a minimal but
# genuine DICOM SCP built on `dicom-ul`'s own real, public `ServerAssociationOptions` API (not
# hand-crafted association bytes). `fake_scp` performs real negotiation and receives the real
# C-ECHO-RQ, then sends a real, validly-encoded but empty P-Data-TF PDU as its response, using
# `dicom-ul`'s own real `send()`/wire writer - exactly what a misbehaving/malicious real SCP
# could choose to send. Both sides communicate over a real loopback TCP socket.
#
# Usage: run_e2e.sh <path-to-clone-dir>
set -euo pipefail

CLONE_DIR="$1"
PORT=24500

cleanup() {
    if [ -n "${SCP_PID:-}" ]; then
        kill "$SCP_PID" 2>/dev/null || true
        wait "$SCP_PID" 2>/dev/null || true
    fi
}
trap cleanup EXIT

cargo build --manifest-path "$CLONE_DIR/Cargo.toml" -p dicom-echoscu >/dev/null
cargo build --manifest-path "$(dirname "$0")/fake_scp/Cargo.toml" >/dev/null

ECHOSCU_BIN="$CLONE_DIR/target/debug/dicom-echoscu"
FAKE_SCP_BIN="$(dirname "$0")/fake_scp/target/debug/fake_scp"

"$FAKE_SCP_BIN" "$PORT" &
SCP_PID=$!
sleep 1

set +e
"$ECHOSCU_BIN" "127.0.0.1:$PORT"
ECHOSCU_EXIT=$?
set -e

echo "dicom-echoscu exit code: $ECHOSCU_EXIT"

if [ "$ECHOSCU_EXIT" -eq 101 ]; then
    echo "BUG REPRODUCED: echoscu panicked (Rust panic exit code 101)"
    exit 1
else
    echo "no panic: echoscu exited with a handled error/success code"
    exit 0
fi
