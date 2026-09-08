#!/usr/bin/env bash
# Standalone repro for mold's parse_fde_encoding() short-CIE remove_prefix() underflow
# (src/input-files.cc, ~line 557; CieRecord<E> in src/mold.h ~line 430).
#
# Usage: ./run.sh /path/to/mold-binary
#
# Builds a minimal x86_64 ET_REL object whose .eh_frame section contains one CIE record with an
# implausibly small declared size (size=1, a 5-byte record). CieRecord::get_contents() faithfully
# returns that 5-byte view. parse_fde_encoding() then reads data[8] (one byte past the view, but
# still inside the real mapped section - harmless) and calls data.remove_prefix(9) on a 5-byte
# view, violating remove_prefix's documented precondition (n <= size()). Under a build with
# _GLIBCXX_ASSERTIONS (mold's own CMakeLists.txt enables this for CMAKE_BUILD_TYPE=Debug), this
# aborts via __glibcxx_assert. Without it, the size_t underflows silently (real UB, no externally
# visible symptom observed - see RESULT.md for why).
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/gen_evil_elf.py" "$WORK/evil.o"

ulimit -c 0
set +e
out="$("$MOLD_BIN" -m elf_x86_64 --thread-count 4 -shared -o "$WORK/out.so" "$WORK/evil.o" 2>&1)"
status=$?
set -e

echo "$out"
echo "mold exit status: $status"
if [ "$status" -ge 128 ] || echo "$out" | grep -qE "Assertion|glibcxx_assert|terminate called|AddressSanitizer|Segmentation fault|Aborted"; then
  echo "REPRO: mold crashed/aborted on the malformed short-CIE .eh_frame section (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status, no crash marker in output)."
