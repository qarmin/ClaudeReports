#!/usr/bin/env bash
# Usage: ./run.sh <path-to-mold-binary>
set -eu
MOLD="${1:?usage: run.sh <path-to-mold-binary>}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/gen_evil_elf.py" "$WORK/good.o" "$WORK/evil.o"

ulimit -c 0
set +e
"$MOLD" -o "$WORK/out.exe" "$WORK/good.o" "$WORK/evil.o"
STATUS=$?
set -e

echo "mold exit status: $STATUS"
exit "$STATUS"
