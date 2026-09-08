#!/usr/bin/env bash
# Standalone repro for mold's thin-archive SYMDEF-skip infinite loop (src/archive-file.cc).
#
# Usage: ./run.sh /path/to/mold-binary [timeout_seconds]
#
# Builds a thin archive whose one real member's long filename resolves to the literal string
# "__.SYMDEF". read_thin_archive_members()'s `continue` for that name fires before the loop's
# only `data` cursor advancement, so the loop re-parses the exact same header forever: this
# never crashes, it hangs (100% CPU, no output, never exits) - a real denial-of-service.
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold [timeout_seconds]}"
TIMEOUT="${2:-5}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/gen_evil_ar.py" "$WORK/evil.a"

ulimit -c 0
set +e
timeout "$TIMEOUT" "$MOLD_BIN" -m elf_x86_64 --thread-count 5 -o "$WORK/out" "$WORK/evil.a" > "$WORK/log.txt" 2>&1
status=$?
set -e

cat "$WORK/log.txt"
echo "mold exit status: $status"
if [ "$status" -eq 124 ]; then
  echo "REPRO: mold hung and was killed by timeout after ${TIMEOUT}s (bug present - infinite loop)."
  exit 0
fi
echo "REPRO: mold exited on its own (status $status, no timeout) - bug not reproduced (fix likely applied)."
