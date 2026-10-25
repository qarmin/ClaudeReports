#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>
# Builds repro.c against <build_dir>'s internal libs (clang ASAN/UBSAN, matches 001's convention),
# generates the crafted module, runs the internal-API repro AND the real insmod --force binary on
# an xz-compressed copy. Exit 0 = bug not present, 1 = reproduced, 2 = setup error.
set -uo pipefail
here=$(dirname "$(realpath "$0")")
src=$(realpath "$1")
bld=$(realpath "$2")
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)")${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
export ASAN_OPTIONS=${ASAN_OPTIONS:-detect_leaks=0}

python3 "$here/gen_min.py" "$work/min_elf.bin" || exit 2

# Internal-API repro: compile libkmod-elf.c with coverage-free ASAN into a tiny driver.
clang -g -O1 -fsanitize=address,undefined -fno-sanitize-recover=undefined -DNDEBUG \
    -include "$bld/config.h" -I"$src" -I"$bld" -I"$src/libkmod" \
    "$here/repro.c" "$src/libkmod/libkmod-elf.c" \
    "$bld/libkmod-internal.a" "$bld/libshared.a" -ldl -Wl,--allow-multiple-definition \
    -o "$work/repro" || exit 2

echo "== internal API: kmod_elf_new + kmod_elf_strip(FORCE_VERMAGIC) on exact-size heap buffer =="
"$work/repro" "$work/min_elf.bin"
api=$?

echo
echo "== real insmod --force binary on an xz-compressed crafted module =="
cli=0
if command -v xz >/dev/null && [ -x "$bld/insmod" ]; then
    cp "$work/min_elf.bin" "$work/evil.ko"
    xz -k -f "$work/evil.ko"
    "$bld/insmod" --force "$work/evil.ko.xz" >"$work/out" 2>&1
    rc=$?
    echo "insmod --force evil.ko.xz -> exit $rc"
    sed 's/^/    /' "$work/out"
    grep -q 'AddressSanitizer\|runtime error' "$work/out" && cli=1
    [ "$rc" -ge 128 ] && cli=1
else
    echo "    (skipped: xz or insmod unavailable)"
fi

echo
if [ "$api" -eq 0 ] && [ "$cli" -eq 0 ]; then
    echo "OVERALL: bug not present"
    exit 0
fi
[ "$api" -eq 2 ] && exit 2
echo "OVERALL: bug reproduced"
exit 1
