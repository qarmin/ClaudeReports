#!/usr/bin/env bash
# Standalone repro for mold's read_riscv_attributes() multi-bug cluster
# (src/input-files.cc, ~line 187, ".riscv.attributes" section parsing).
#
# Runs three independent crafted RV64LE ET_REL objects against the real mold binary:
#   1. evil.o                  - sz<4 integer underflow (sub-case 2) -> SIGBUS
#   2. evil2.o                 - short arch-string sub-section, throwing substr (sub-case 4) -> SIGABRT
#   3. evil_hang.o (generated) - sz==0 with non-matching vendor name (sub-case 3) -> infinite loop/hang
#
# Usage: ./run.sh /path/to/mold-binary
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/gen_evil_elf.py" "$WORK/evil.o"
python3 "$DIR/gen_evil_elf_variant2_substr.py" "$WORK/evil2.o"
python3 "$DIR/gen_evil_elf_hang.py" "$WORK/evil_hang.o"

ulimit -c 0
overall_ok=1

echo "=== sub-case 2: sz<4 integer underflow (expect SIGBUS / exit 135) ==="
set +e
out1="$("$MOLD_BIN" -m elf64lriscv --thread-count 5 -shared -o "$WORK/out1.so" "$WORK/evil.o" 2>&1)"
status1=$?
set -e
echo "$out1"
echo "exit status: $status1"
if [ "$status1" -ge 128 ]; then
  echo "REPRO 1: mold crashed (bug present)."
else
  echo "REPRO 1: mold did NOT crash (bug not reproduced / fix likely applied)."
  overall_ok=0
fi

echo
echo "=== sub-case 4: throwing substr on short arch sub-section (expect SIGABRT / exit 134) ==="
set +e
out2="$("$MOLD_BIN" -m elf64lriscv --thread-count 5 -shared -o "$WORK/out2.so" "$WORK/evil2.o" 2>&1)"
status2=$?
set -e
echo "$out2"
echo "exit status: $status2"
if [ "$status2" -ge 128 ] || echo "$out2" | grep -qE "out_of_range|terminate called|Aborted"; then
  echo "REPRO 2: mold crashed (bug present)."
else
  echo "REPRO 2: mold did NOT crash (bug not reproduced / fix likely applied)."
  overall_ok=0
fi

echo
echo "=== sub-case 3: sz==0 infinite loop (expect 'timeout' to kill it, exit 124) ==="
set +e
timeout 6 "$MOLD_BIN" -m elf64lriscv --thread-count 1 -shared -o "$WORK/out3.so" "$WORK/evil_hang.o" >"$WORK/hang.out" 2>&1
status3=$?
set -e
cat "$WORK/hang.out"
echo "exit status: $status3"
if [ "$status3" -eq 124 ]; then
  echo "REPRO 3: mold hung (killed by timeout) - bug present."
else
  echo "REPRO 3: mold did NOT hang (bug not reproduced / fix likely applied)."
  overall_ok=0
fi

echo
if [ "$overall_ok" -eq 1 ]; then
  echo "ALL REPROS: bug present (pristine code)."
else
  echo "SOME REPROS did not reproduce a crash/hang - likely running against a patched build."
fi
