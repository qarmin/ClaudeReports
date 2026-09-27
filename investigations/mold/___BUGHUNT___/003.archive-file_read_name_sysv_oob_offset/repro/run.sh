#!/usr/bin/env bash
# Standalone repro for mold's ArHdr::read_name SysV-long-filename OOB read (src/archive-file.cc).
#
# Usage: ./run.sh /path/to/mold-binary
#
# Builds a 1-member ".a" archive whose sole member uses the SysV extended-filename convention
# ("/N" where N is a decimal offset into the archive's string table), but sets N to a huge,
# out-of-range value and includes no actual string table. mold's ArHdr::read_name computes
# `strtab.data() + atoi(ar_name + 1)` with no bounds check and immediately passes that wild
# pointer to strstr(), causing a SIGSEGV before any real linking work happens.
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/gen_evil_ar.py" "$WORK/evil.a"

ulimit -c 0
set +e
out="$("$MOLD_BIN" -o "$WORK/out" "$WORK/evil.a" 2>&1)"
status=$?
set -e

echo "$out"
echo "mold exit status: $status"
if [ "$status" -ge 128 ] || echo "$out" | grep -qE "AddressSanitizer|SEGV|Segmentation fault"; then
  echo "REPRO: mold crashed on the malformed archive (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status, no crash marker in output) - bug not reproduced (fix likely applied)."
