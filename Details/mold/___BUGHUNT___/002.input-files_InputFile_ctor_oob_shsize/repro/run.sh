#!/usr/bin/env bash
# Standalone repro for the mold InputFile<E>::InputFile() OOB-read crash (src/input-files.cc).
#
# Usage: ./run.sh /path/to/mold-binary
#
# Builds (by the caller, see ../justfile) a mold binary from a clean checkout, then feeds it a
# 64-byte relocatable ELF object with e_shnum=0 and e_shoff pointing far past EOF. Per the ELF
# spec, e_shnum=0 means "the real count overflowed 16 bits, read it from the first section
# header's sh_size field instead" - mold's InputFile constructor does exactly that
# (`sh_begin->sh_size`) *before* validating that e_shoff/sh_begin actually points inside the
# mapped file, causing an out-of-bounds read (SIGSEGV).
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/make_bad_elf.py" "$WORK/bad.o" 0x7fff0000 0 0

ulimit -c 0
set +e
out="$("$MOLD_BIN" -o "$WORK/out" "$WORK/bad.o" 2>&1)"
status=$?
set -e

echo "$out"
echo "mold exit status: $status"
if [ "$status" -ge 128 ] || echo "$out" | grep -qE "AddressSanitizer|SEGV|Segmentation fault"; then
  echo "REPRO: mold crashed on the malformed object file (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status, no crash marker in output) - bug not reproduced (fix likely applied)."
