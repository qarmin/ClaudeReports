#!/bin/bash
# Repeatability harness: run Case A (fill80 .debug_abbrev, DWARF5) N times and
# report how many produce the ASAN heap-buffer-overflow in read_uleb.
set -u
MOLD="$(readlink -f "${1:?usage: repeat.sh /path/to/mold [N]}")"
N="${2:-5}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ulimit -c 0
WORK=$(mktemp -d); trap 'rm -rf "$WORK"' EXIT; cd "$WORK"
cp "$SCRIPT_DIR/test.c" "$SCRIPT_DIR/corrupt_uleb.py" .
export ASAN_OPTIONS=detect_leaks=0
CRT1=/usr/lib/x86_64-linux-gnu/crt1.o; CRTI=/usr/lib/x86_64-linux-gnu/crti.o
CRTN=/usr/lib/x86_64-linux-gnu/crtn.o; LIBDIR=/usr/lib/x86_64-linux-gnu
DYNLD=/lib64/ld-linux-x86-64.so.2
gcc -c -g -O0 test.c -o real.o
python3 corrupt_uleb.py real.o evil.o .debug_abbrev fill80 >/dev/null
hit=0
for i in $(seq 1 "$N"); do
  OUT=$("$MOLD" --gdb-index --compress-debug-sections=zlib -o out.exe \
        "$CRT1" "$CRTI" evil.o -L"$LIBDIR" -lc "$CRTN" \
        --dynamic-linker "$DYNLD" -e main 2>&1)
  if echo "$OUT" | grep -q "heap-buffer-overflow.*\|read_uleb"; then
    frame=$(echo "$OUT" | grep -m1 "in mold::read_uleb")
    echo "run $i: HIT  ($frame)"
    hit=$((hit+1))
  else
    echo "run $i: no crash (exit path)"
  fi
done
echo "RESULT: $hit/$N runs reproduced the read_uleb heap-buffer-overflow"
