#!/bin/bash
# Self-contained repro for the read_rnglist_range() bugs in src/gdb-index.cc
# (mold's --gdb-index DWARF5 .debug_rnglists parser).
#
# Builds a real DWARF5 object with gcc, corrupts one DW_RLE_* opcode byte in
# its real .debug_rnglists section, and links it with the mold binary under
# test (path given as $1) using --gdb-index.
#
# Usage: run.sh /path/to/mold
set -eu
MOLD="${1:?usage: run.sh /path/to/mold}"
ulimit -c 0

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
cd "$WORK"
cp "$(dirname "$0")/test.c" "$(dirname "$0")/find_rnglists.py" "$(dirname "$0")/make_evil.py" .

echo "=== Compiling real.o (gcc -gdwarf-5 -ffunction-sections -O2) ==="
gcc -c -gdwarf-5 -ffunction-sections -O2 test.c -o real.o
readelf -S real.o | grep -i rnglist

echo
echo "=== Corrupting one DW_RLE_start_length opcode byte (0x07) -> 0x01 (DW_RLE_base_addressx) ==="
echo "    (this CU has no DW_AT_addr_base, so addrx is null in the parser)"
python3 make_evil.py real.o evil_addrx.o 0 0x01

CRT1=/usr/lib/x86_64-linux-gnu/crt1.o
CRTI=/usr/lib/x86_64-linux-gnu/crti.o
CRTN=/usr/lib/x86_64-linux-gnu/crtn.o
LIBDIR=/usr/lib/x86_64-linux-gnu
DYNLD=/lib64/ld-linux-x86-64.so.2

echo
echo "=== Baseline: linking the UNMODIFIED real.o with --gdb-index (expect success) ==="
"$MOLD" --gdb-index -o out_real.exe "$CRT1" "$CRTI" real.o -L"$LIBDIR" -lc "$CRTN" \
  --dynamic-linker "$DYNLD" -e main
echo "real.o link exit code: $?"

echo
echo "=== Bug (A): linking evil_addrx.o (corrupted opcode -> null addrx deref) ==="
set +e
"$MOLD" --gdb-index -o out_evil_addrx.exe "$CRT1" "$CRTI" evil_addrx.o -L"$LIBDIR" -lc "$CRTN" \
  --dynamic-linker "$DYNLD" -e main
CODE=$?
set -e
echo "evil_addrx.o link exit code: $CODE"
if [ "$CODE" -eq 139 ]; then
  echo "==> SEGFAULT reproduced (unpatched mold)"
elif [ "$CODE" -ne 0 ]; then
  echo "==> mold exited non-zero but did NOT segfault (likely the patched build: clean Fatal() error)"
else
  echo "==> mold exited 0 -- bug NOT reproduced with this mold binary"
fi

echo
echo "=== Bug (B): missing default: case, unknown opcode 0x08 in .debug_rnglists ==="
python3 make_evil.py real.o evil_unknown.o 0 0x08
set +e
"$MOLD" --gdb-index -o out_evil_unknown.exe "$CRT1" "$CRTI" evil_unknown.o -L"$LIBDIR" -lc "$CRTN" \
  --dynamic-linker "$DYNLD" -e main
CODE2=$?
set -e
echo "evil_unknown.o link exit code: $CODE2"
if [ "$CODE2" -eq 0 ]; then
  echo "==> mold silently accepted the corrupted opcode (unpatched: no default: case, self-terminated on a stray zero byte, no diagnostic at all)"
else
  echo "==> mold rejected the corrupted opcode with a diagnostic (likely the patched build)"
fi
