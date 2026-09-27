#!/usr/bin/env bash
# Standalone repro for the mold `is_gcc_lto_obj` OOB-read crash (src/filetype.cc).
#
# Usage: ./run.sh /path/to/mold-binary
#
# Builds (by the caller, see ../justfile) an ASAN-instrumented mold from a clean checkout,
# then feeds it a 64-byte relocatable ELF object whose e_shoff points far past EOF while
# e_shnum=1 - mold's own file-type sniffer (is_gcc_lto_obj, which runs before any other
# validation of the object) dereferences a section header at that bogus offset before any
# bounds check exists, causing an out-of-bounds read (SIGSEGV under a plain build, an
# AddressSanitizer SEGV report under an ASAN build).
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/make_bad_elf.py" "$WORK/bad.o" 0x7fff0000 1 0

ulimit -c 0
set +e
out="$("$MOLD_BIN" -o "$WORK/out" "$WORK/bad.o" 2>&1)"
status=$?
set -e

echo "$out"
echo "mold exit status: $status"
# Under a plain build this is a real SIGSEGV (signal-terminated, status 128+11=139); under an
# ASAN build ASAN intercepts the SEGV, prints a report, and calls exit(1) itself - so status
# alone doesn't distinguish a crash from mold's own clean "e_shoff or e_shnum corrupted" Fatal()
# exit (also status 1). Check for ASAN's/the kernel's crash markers instead.
if [ "$status" -ge 128 ] || echo "$out" | grep -qE "AddressSanitizer|SEGV|Segmentation fault"; then
  echo "REPRO: mold crashed on the malformed object file (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status, no crash marker in output) - bug not reproduced (fix likely applied)."
