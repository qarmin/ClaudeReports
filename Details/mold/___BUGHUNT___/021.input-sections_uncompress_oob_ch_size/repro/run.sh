#!/usr/bin/env bash
# Standalone repro for mold's InputSection ch_size-driven huge-allocation abort
# (src/input-sections.cc: InputSection<E>::InputSection() / uncompress()).
#
# Usage: ./run.sh /path/to/mold-binary
#
# Builds a minimal x86_64 ET_REL object with one section that has
# SHF_MERGE|SHF_STRINGS|SHF_COMPRESSED set. Its content begins with a well-formed-looking
# Elf64_Chdr (ch_type=ELFCOMPRESS_ZLIB) but a huge, attacker-controlled ch_size (0xFFFFFFFFFFFF0000).
#
# InputSection<E>::InputSection() (src/input-sections.cc) reads `sh_size = chdr.ch_size` verbatim,
# with no upper-bound check. Because the section also carries SHF_MERGE,
# ObjectFile<E>::convert_mergeable_sections() (src/input-files.cc, called unconditionally by
# create_merged_sections() on every link) constructs a MergeableSection for it, and
# MergeableSection<E>::MergeableSection() (src/input-sections.cc) immediately calls
# `isec->uncompress(ctx)`, whose very first statement is `u8 *buf = new u8[sh_size];`
# (src/input-sections.cc) - i.e. `new u8[0xFFFFFFFFFFFF0000]` - before any real bounds or
# decompression check ever runs. mold's mimalloc-based allocator aborts the process outright
# (SIGABRT, no exception, no diagnostic on stdout/stderr) when this allocation fails.
#
# No special CLI flag is needed - `mold -shared -o out.so evil.o` is enough.
set -euo pipefail

MOLD_BIN="${1:?usage: run.sh /path/to/mold}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

python3 "$DIR/gen_evil_elf.py" "$WORK/evil.o"

# Safety net: this specific ch_size (~1.8e19) makes mimalloc fail the allocation request
# immediately and abort - it does not actually try to touch that much memory. The ulimit/timeout
# below are still set defensively in case this is ever re-run with a different, moderate-huge
# ch_size value (see RESULT.md: a value like 1TiB can succeed as a virtual-memory-only mapping
# and spin for a long time instead of aborting cleanly).
ulimit -v 12000000  # ~12 GiB - well above mold's own ~8GiB startup reservation, far below any
                    # value that would let the malicious ch_size allocation actually succeed
ulimit -c 0

set +e
out="$(timeout 30 "$MOLD_BIN" -shared -o "$WORK/out.so" "$WORK/evil.o" 2>&1)"
status=$?
set -e

echo "$out"
echo "mold exit status: $status"
if [ "$status" -ge 128 ] || echo "$out" | grep -qE "AddressSanitizer|SEGV|Segmentation fault|Aborted"; then
  echo "REPRO: mold crashed/aborted on the malformed object (bug present)."
  exit 0
fi
echo "REPRO: mold did NOT crash (exited $status) - bug not reproduced (fix likely applied)."
echo "(expected with the fix applied: exit 1, 'corrupted compressed section: ch_size is unreasonably large')"
