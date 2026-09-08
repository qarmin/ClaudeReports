#!/usr/bin/env bash
# Standalone repro for the --defsym null-Symbol-pointer deref in mold's
# parse_defsym_value (src/cmdline.cc).
#
# Usage: run.sh <path-to-mold-binary>
#
# Compiles a trivial valid object file, then links it with a malformed
# --defsym value ("0x12z" - a hex prefix followed by non-hex trailing
# garbage). On the buggy binary this segfaults (exit 139) instead of
# printing a diagnostic. On a fixed binary it should print a clean
# "mold: fatal: -defsym: not a number: 0x12z" and exit 1.
set -u

MOLD="${1:?usage: run.sh <path-to-mold-binary>}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

ulimit -c 0

gcc -c -o "$DIR/valid.o" "$DIR/trivial.c" || exit 2

echo "=== malformed --defsym value (the bug) ==="
"$MOLD" -o /tmp/mold_defsym_repro_out --defsym=foo=0x12z "$DIR/valid.o"
status=$?
echo "exit status: $status"
if [ "$status" -ge 128 ]; then
    sig=$((status - 128))
    echo "CRASHED: killed by signal $sig"
else
    echo "did not crash (expected on a fixed binary: exit 1 with a diagnostic)"
fi

echo
echo "=== sanity: valid --defsym value still works ==="
"$MOLD" -o /tmp/mold_defsym_repro_out_valid --defsym=foo=0x1234 "$DIR/valid.o"
echo "exit status: $?"

exit "$status"
