#!/usr/bin/env bash
# Standalone repro for mold's parse_ehframe() unchecked CIE-length crash (src/input-files.cc).
#
# Usage: ./run.sh /path/to/mold-binary
#
# Builds a minimal x86_64 ET_REL object whose ".eh_frame" section contains one "CIE" record with
# a fabricated, huge length field. parse_ehframe() uses that length directly as
# `data.substr(size + 4)` (the single-arg std::string_view::substr(pos) overload), which throws
# std::out_of_range once pos exceeds the section's real size - mold doesn't catch it, so it
# propagates to std::terminate()/SIGABRT.
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
  echo "REPRO: mold crashed on the malformed .eh_frame section (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status, no crash marker in output) - bug not reproduced (fix likely applied)."
