#!/bin/bash
# Self-contained repro for the unchecked unit_length in read_compunits()
# (src/gdb-index.cc), reached via mold's --gdb-index flag.
#
# Compiles a real object with gcc, overwrites the first .debug_info compile-unit
# header's unit_length so that `i64 size = unit_length + 4/12` computes to 0,
# then links with the mold binary under test using --gdb-index.
#
# Unpatched mold spins in `for (u8 *p = begin; p < end;) { ...; p += size; }`
# with size == 0, pushing Compunit records forever (runaway memory -> abort/hang,
# a DoS). Patched mold reports a clean Fatal error.
#
# Usage: run.sh /path/to/mold
#
# SAFEGUARDS: each mold invocation runs under `timeout` AND an address-space
# `ulimit -v` (in a subshell) so a runaway allocation cannot thrash the host.
# NOTE: mold reserves ~8 GiB of *virtual* address space for its output up front,
# so the cap is set to 10 GiB (above that reservation) -- the runaway then trips
# the cap after committing only ~1-2 GiB of real memory, within a second or two.
set -u
MOLD="${1:?usage: run.sh /path/to/mold}"
MOLD="$(readlink -f "$MOLD")"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ulimit -c 0

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
cd "$WORK"
cp "$SCRIPT_DIR/test.c" "$SCRIPT_DIR/corrupt_debug_info.py" .

CRT1=/usr/lib/x86_64-linux-gnu/crt1.o
CRTI=/usr/lib/x86_64-linux-gnu/crti.o
CRTN=/usr/lib/x86_64-linux-gnu/crtn.o
LIBDIR=/usr/lib/x86_64-linux-gnu
DYNLD=/lib64/ld-linux-x86-64.so.2

TIMEOUT_S=25
VMEM_KB=$((10*1024*1024))   # 10 GiB address-space cap per mold invocation

link() {  # $1 = object, $2 = output-tag ; runs in a subshell with the vmem cap
  ( ulimit -v "$VMEM_KB"
    timeout -s KILL "$TIMEOUT_S" \
      "$MOLD" --gdb-index -o "out_$2.exe" "$CRT1" "$CRTI" "$1" \
        -L"$LIBDIR" -lc "$CRTN" --dynamic-linker "$DYNLD" -e main )
}

report() {  # $1 = exit code, $2 = tag
  local code=$1
  if [ "$code" -eq 137 ]; then
    echo "==> [$2] KILLED by timeout after ${TIMEOUT_S}s (137) -- HANG reproduced (unpatched)"
  elif [ "$code" -eq 134 ] || [ "$code" -eq 136 ]; then
    echo "==> [$2] aborted ($code) -- runaway allocation hit the ${VMEM_KB}KB cap (unpatched DoS)"
  elif [ "$code" -eq 0 ]; then
    echo "==> [$2] mold exited 0 -- linked cleanly"
  else
    echo "==> [$2] mold exited $code -- (patched build: clean Fatal(), or bad_alloc)"
  fi
}

echo "################################################################"
echo "# Case A1 (DWARF32): unit_length 0xFFFFFFFC ; size = len+4 wraps 0"
echo "################################################################"
gcc -c -g -gdwarf-4 -O0 test.c -o real32.o
python3 corrupt_debug_info.py real32.o evil32.o dwarf32
echo
echo "--- Baseline: unmodified real32.o (expect exit 0) ---"
link real32.o real32; report $? "baseline-dwarf32"
echo
echo "--- Malformed: evil32.o ---"
link evil32.o evil32; report $? "evil-dwarf32"

echo
echo "################################################################"
echo "# Case A2 (DWARF64): unit_length 0xFFFFFFFFFFFFFFF4 ; size = len+12 wraps 0"
echo "################################################################"
gcc -c -g -gdwarf-5 -gdwarf64 -O0 test.c -o real64.o
python3 corrupt_debug_info.py real64.o evil64z.o dwarf64
echo
echo "--- Malformed: evil64z.o ---"
link evil64z.o evil64z; report $? "evil-dwarf64-zero"

echo
echo "################################################################"
echo "# Case A3 (secondary): DWARF64 unit_length 0x8000000000000000"
echo "#   -> negative i64 stride; loop exits early recording a CORRUPT"
echo "#   CU (offset 0, size ~ -9.2e18) into the output .gdb_index CU list."
echo "#   Unpatched mold exits 0 silently (no hang); patched mold Fatals."
echo "################################################################"
python3 corrupt_debug_info.py real64.o evil64n.o dwarf64neg
echo
echo "--- Malformed: evil64n.o ---"
link evil64n.o evil64n; report $? "evil-dwarf64-neg"
