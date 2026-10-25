#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>
# Builds repro.c against <build_dir>/libkmod.so (clang ASAN/UBSAN build, asserts on), runs it,
# then runs the real <build_dir>/modinfo on the same crafted files and the real depmod on a
# module tree containing one of them.
# Exit 0 = bug not present, 1 = bug reproduced, 2 = setup error.
set -uo pipefail
here=$(dirname "$(realpath "$0")")
src=$(realpath "$1")
bld=$(realpath "$2")
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
ulimit -c 0

# The sanitizer build links the shared clang runtime; make it findable without LD_PRELOAD.
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)")${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
export ASAN_OPTIONS=${ASAN_OPTIONS:-detect_leaks=0}

clang -g -O1 -fsanitize=address,undefined -shared-libsan -I"$src" \
    "$here/repro.c" -o "$work/repro" -L"$bld" -lkmod -Wl,-rpath,"$bld" || exit 2

echo "== public API: kmod_module_new_from_path() + kmod_module_get_info() =="
"$work/repro" "$work"
api=$?

echo
echo "== real modinfo binary ($bld/modinfo) =="
cli=0
for f in shstrndx-oob shstrndx-eq-count shstrndx-undef shstrndx-valid; do
    "$bld/modinfo" "$work/$f.ko" >"$work/out" 2>&1
    rc=$?
    echo "modinfo $f.ko -> exit $rc"
    sed 's/^/    /' "$work/out"
    [ "$rc" -ge 128 ] && cli=1
done

echo
echo "== real depmod binary on a module tree containing shstrndx-oob.ko =="
mkdir -p "$work/root/lib/modules/9.9.9/kernel"
cp "$work/shstrndx-oob.ko" "$work/root/lib/modules/9.9.9/kernel/evil.ko"
"$bld/depmod" -b "$work/root" 9.9.9 >"$work/out" 2>&1
rc=$?
echo "depmod -b <root> 9.9.9 -> exit $rc"
sed 's/^/    /' "$work/out"
[ "$rc" -ge 128 ] && cli=1

echo
if [ "$api" -eq 0 ] && [ "$cli" -eq 0 ]; then
    echo "OVERALL: bug not present"
    exit 0
fi
[ "$api" -eq 2 ] && exit 2
echo "OVERALL: bug reproduced"
exit 1
