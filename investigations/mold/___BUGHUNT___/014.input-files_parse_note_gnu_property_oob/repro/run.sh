#!/usr/bin/env bash
# Standalone repro for mold's parse_note_gnu_property() unchecked n_namesz crash
# (src/input-files.cc, ObjectFile<E>::parse_note_gnu_property).
#
# Usage: ./run.sh /path/to/mold-binary
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/gen_evil_elf.py" "$WORK/evil.o"

ulimit -c 0
set +e
out="$("$MOLD_BIN" -m elf_x86_64 --thread-count 5 -shared -o "$WORK/out.so" "$WORK/evil.o" 2>&1)"
status=$?
set -e

echo "$out"
echo "mold exit status: $status"
if [ "$status" -ge 128 ] || echo "$out" | grep -qE "out_of_range|terminate called|AddressSanitizer|Segmentation fault|Aborted"; then
  echo "REPRO: mold crashed on the malformed .note.gnu.property section (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status, no crash marker in output) - bug not reproduced (fix likely applied)."
