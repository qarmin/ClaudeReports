#!/usr/bin/env bash
# Runs one repro cycle for candidate 028 (RemoteDebuggerPeerTCP::has_message()/is_peer_connected()
# missing lock): starts the real server_project/ as a genuine second participant (a real TCP debug
# server, speaking the real var_to_bytes()-encoded wire protocol), then starts client_project/ as
# the real debuggee under `--remote-debug tcp://127.0.0.1:<PORT>` and under ThreadSanitizer, and
# reports whether a data race in RemoteDebuggerPeerTCP (or the List<Array> `in_queue` it guards)
# fired.
#
# Usage: run_repro.sh <path-to-godot-binary> <label> <output-dir>
#
# The binary must be built with:
#   scons platform=linuxbsd target=template_debug use_tsan=yes debug_symbols=yes \
#         disable_path_overrides=no
# (disable_path_overrides=no is required for --remote-debug/export-template binaries to load a
# project from the CWD at all; use_tsan=yes is what makes the race actually observable).

set -uo pipefail

GODOT="$1"
LABEL="$2"
OUTDIR="$3"
PORT=17891

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

rm -rf "$OUTDIR"
mkdir -p "$OUTDIR/xdg_server" "$OUTDIR/xdg_client"

(
	cd "$SCRIPT_DIR/server_project" || exit 1
	XDG_DATA_HOME="$OUTDIR/xdg_server" timeout 30 "$GODOT" --headless \
		>"$OUTDIR/server_stdout.txt" 2>&1
) &
SERVER_BGPID=$!

sleep 2

(
	cd "$SCRIPT_DIR/client_project" || exit 1
	XDG_DATA_HOME="$OUTDIR/xdg_client" \
		TSAN_OPTIONS="log_path=$OUTDIR/tsan_client history_size=7" \
		timeout 30 "$GODOT" --headless --remote-debug "tcp://127.0.0.1:$PORT" \
		>"$OUTDIR/client_stdout.txt" 2>&1
)
CLIENT_EXIT=$?

wait "$SERVER_BGPID" 2>/dev/null

echo "=== $LABEL: client exit $CLIENT_EXIT ==="
if ls "$OUTDIR"/tsan_client.* >/dev/null 2>&1; then
	if grep -lq "remote_debugger_peer.cpp\|List<Array" "$OUTDIR"/tsan_client.*; then
		echo "[$LABEL] RACE DETECTED (RemoteDebuggerPeerTCP / its List<Array> in_queue):"
		grep "SUMMARY: ThreadSanitizer" "$OUTDIR"/tsan_client.* | grep -i "remote_debugger_peer\|list.h"
		exit 10
	else
		echo "[$LABEL] clean: no RemoteDebuggerPeerTCP/List<Array> race in ThreadSanitizer output"
		exit 0
	fi
else
	echo "[$LABEL] no ThreadSanitizer log was produced (unexpected - check stdout logs)"
	exit 1
fi
