#!/usr/bin/env bash
# Standalone repro for mold's InputSection constructor unchecked-sh_offset crash
# (src/input-sections.cc).
#
# Usage: ./run.sh /path/to/mold-binary
#
# Builds a minimal i386 ET_REL object with one SHF_ALLOC ".text" section whose sh_offset is a
# fabricated, huge value. InputSection<E>::InputSection() computes
# `contents = file.mf->data + shdr().sh_offset` with no bounds check at all - during output
# writing, copy_contents_to()'s `memcpy(buf, contents, sz)` dereferences that wild pointer.
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/gen_evil_elf.py" "$WORK/evil.o"

ulimit -c 0
set +e
out="$("$MOLD_BIN" -m elf_i386 --thread-count 5 -shared -o "$WORK/out.so" "$WORK/evil.o" 2>&1)"
status=$?
set -e

echo "$out"
echo "mold exit status: $status"
if [ "$status" -ge 128 ] || echo "$out" | grep -qE "AddressSanitizer|SEGV|Segmentation fault"; then
  echo "REPRO: mold crashed on the malformed object (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status, no crash marker in output) - bug not reproduced (fix likely applied)."
