#!/usr/bin/env bash
# Standalone repro: reproduce test_logger.cpp's SIGSEGV against a real Godot build.
# Usage: run.sh <path-to-godot-tests-binary>
# The binary must be built with `tests=yes` (a --test-capable build).
set -euo pipefail

BIN="${1:?usage: run.sh <path-to-godot-binary-built-with-tests=yes>}"

FAKE_HOME="$(mktemp -d)"
# Only $HOME/.local/share (the XDG data dir Godot's user:// maps to) is made
# unwritable - $HOME/.cache stays writable, matching this project's own Docker
# environment where $HOME is read-only except for a few whitelisted subdirs.
# A plain `chmod 555 "$FAKE_HOME"` does NOT reproduce this: it also blocks
# $HOME/.cache, which test_main.cpp's own temp-dir setup needs, so the test
# binary bails out gracefully before even reaching the TEST_CASEs below.
mkdir -p "$FAKE_HOME/.cache" "$FAKE_HOME/.local/share"
chmod 555 "$FAKE_HOME/.local/share"
trap 'chmod 755 "$FAKE_HOME/.local/share"; rm -rf "$FAKE_HOME"' EXIT

echo "=== Running with \$HOME/.local/share unwritable (simulates a read-only user-data dir / restricted container) ==="
set +e
HOME="$FAKE_HOME" "$BIN" --headless --test --test-case="*Logger*"
STATUS=$?
set -e

echo
echo "=== exit status: $STATUS ==="
if [ "$STATUS" -ge 128 ]; then
	SIG=$((STATUS - 128))
	# Godot's own crash handler catches the SIGSEGV (11), prints the "handle_crash" backtrace
	# above, then calls abort() - so the process's own exit signal is usually SIGABRT (6), not
	# a raw 11. The "handle_crash: ... signal 11" line above is the actual proof of the SIGSEGV.
	echo "Process was killed by signal $SIG - CRASH reproduced (see 'handle_crash' backtrace above for the original SIGSEGV)."
	exit 0
else
	echo "Process exited normally (no crash) - either the fix is applied, or this environment doesn't reproduce the trigger condition."
	exit 1
fi
