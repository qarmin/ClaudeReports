#!/usr/bin/env bash
# Standalone repro for candidate 026: InputFile<E>::InputFile()'s Fatal(ctx) << *this
# calls perform a C-style downcast to ObjectFile<E>/SharedFile<E> (in the operator<<
# they invoke, src/input-files.cc ~line 1490) while *this is still mid-construction
# in the InputFile<E> BASE constructor - the derived (ObjectFile<E>) part of the
# object has not been constructed yet at that point. This is undefined behavior
# (a downcast through an incomplete-dynamic-type object) and is flagged live by
# g++'s UBSan vptr check ("-fsanitize=undefined", part of MOLD_USE_ASAN's default
# flag set - see CMakeLists.txt lines 106-111, no extra flag needed).
#
# IMPORTANT: unlike the task that spawned this candidate assumed, this bug is
# reachable RIGHT NOW on pristine/unpatched mold - candidate 002's patch is NOT
# required to trigger it. Pristine InputFile<E>::InputFile() already has TWO such
# calls:
#   if (mf->size < sizeof(ElfEhdr<E>)) Fatal(ctx) << *this << ": file too small";
#   if (memcmp(mf->data, "\177ELF", 4)) Fatal(ctx) << *this << ": not an ELF file";
# (src/input-files.cc, current lines ~81-84 pre-fix). The first of these is reachable
# via the real mold CLI on a genuinely truncated but ELF-magic'd ET_REL object file
# (get_file_type() in filetype.cc classifies it as ELF_OBJ from just the first ~20
# bytes without ever checking total file size, so a file smaller than
# sizeof(Elf64_Ehdr)=64 bytes sails through to ObjectFile<E>'s constructor).
#
# This script demonstrates BOTH triggers for completeness:
#   1. Scenario A (no patch needed at all): a 20-byte truncated ELF object hits the
#      pristine "file too small" Fatal(ctx) << *this call directly.
#   2. Scenario B (candidate 002's patch bundled here as candidate_002_patch.diff,
#      applied temporarily): an out-of-range e_shoff hits 002's own newly-added
#      Fatal(ctx) << *this << ": e_shoff is out of range" call, showing the same
#      hazard would resurface the moment that (separately valid, already-Confirmed)
#      fix is ever adopted for real.
#
# Usage: ./run.sh /path/to/mold-source-checkout
#   (a checkout of the exact base_commit this candidate targets; the script builds
#   build-asan/ itself if not already built, and does NOT modify the checkout
#   permanently - candidate 002's patch is applied and then reverted within this
#   script's own run)
set -euo pipefail

SRC="${1:?usage: run.sh /path/to/mold-source-checkout}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

cd "$SRC"

build() {
  if [ ! -d build-asan ]; then
    cmake -S . -B build-asan -DCMAKE_BUILD_TYPE=Debug -DMOLD_USE_ASAN=ON \
      -DCMAKE_C_COMPILER=gcc -DCMAKE_CXX_COMPILER=g++ >/dev/null
  fi
  # Always rebuild (even if build-asan/mold already exists) - this script may be
  # re-run against the same clone after a patch has just been applied/reverted to
  # its source, and an incremental `cmake --build` after touching one .cc file is
  # fast (a few seconds), so there's no reason to risk running a stale binary.
  cmake --build build-asan -j4
}

build

python3 "$DIR/make_truncated_elf.py" "$WORK/truncated.o" 20
python3 "$DIR/make_bad_elf_002.py" "$WORK/bad_shoff.o" 0x7fff0000 0 0

echo "================================================================"
echo "SCENARIO A: pristine code, 20-byte truncated ELF object"
echo "(triggers the PRE-EXISTING 'file too small' Fatal(ctx) << *this call)"
echo "================================================================"
set +e
out=$(./build-asan/mold "$WORK/truncated.o" -o "$WORK/out" 2>&1)
status=$?
set -e
echo "$out"
echo "exit status: $status"
if echo "$out" | grep -q "runtime error: downcast of address"; then
  echo "REPRO A: UBSan invalid-downcast diagnostic observed (bug present)."
else
  echo "REPRO A: no UBSan diagnostic seen (bug fixed, or UBSan not active)."
fi

echo
echo "================================================================"
echo "SCENARIO B: pristine + candidate 002's patch applied (hypothetical"
echo "future fix), bad_shoff.o (e_shoff=0x7fff0000)"
echo "(triggers 002's NEW 'e_shoff is out of range' Fatal(ctx) << *this call)"
echo "================================================================"
git apply "$DIR/candidate_002_patch.diff"
build
set +e
out=$(./build-asan/mold "$WORK/bad_shoff.o" -o "$WORK/out2" 2>&1)
status=$?
set -e
echo "$out"
echo "exit status: $status"
git apply -R "$DIR/candidate_002_patch.diff"
if echo "$out" | grep -q "runtime error: downcast of address"; then
  echo "REPRO B: UBSan invalid-downcast diagnostic observed (bug present)."
else
  echo "REPRO B: no UBSan diagnostic seen (bug fixed, or UBSan not active)."
fi
