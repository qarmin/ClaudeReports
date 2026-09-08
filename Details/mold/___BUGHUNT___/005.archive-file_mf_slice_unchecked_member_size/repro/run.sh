#!/usr/bin/env bash
# Standalone repro for mold's archive-member-size validation gap (src/archive-file.cc).
#
# Usage: ./run.sh /path/to/mold-binary
#
# Builds a 1-member ".a" (fat) archive whose member header LIES about its own size (claims
# 5,000,000 bytes; only 64 real bytes - a fake ELF header - actually follow). mold's
# read_fat_archive_members() passes that unchecked, attacker-controlled size straight into
# MappedFile::slice(), producing a "file" whose declared .size doesn't match how much memory is
# actually mapped behind it. The embedded fake ELF header's e_shoff (3,000,000) is "in bounds"
# per that lie but points to genuinely unmapped memory - dereferencing it (via
# is_gcc_lto_obj()/get_file_type(), the very first ELF-parsing step) SIGSEGVs.
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
if [ "$status" -ge 128 ] || echo "$out" | grep -qE "AddressSanitizer|SEGV|Segmentation fault"; then
  echo "REPRO: mold crashed on the malformed archive (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status, no crash marker in output) - bug not reproduced (fix likely applied)."
