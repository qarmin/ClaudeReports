#!/usr/bin/env bash
# run.sh <kmod_src_dir> <ignored>: builds a plain (non-ASAN) debug build of the source, then fails each allocation of `modprobe -n -v foo` in turn (LD_PRELOAD shim). Non-zero = some allocation failure crashes modprobe.
set -u
here=$(cd "$(dirname "$0")" && pwd)
src=$(realpath "${1:?src dir}")
ulimit -c 0
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
CC=clang meson setup "$work/b" "$src" -Dbuildtype=debug -Ddocs=false -Dmanpages=false -Dwerror=false -Dmbedtls=disabled -Dbuild-tests=false >/dev/null 2>&1 \
	&& ninja -C "$work/b" >/dev/null 2>&1 || { echo "build failed"; exit 2; }
clang -O1 -shared -fPIC "$here/failmalloc.c" -ldl -o "$work/failmalloc.so"
python3 "$here/gen_tree.py" "$work/root"
"$work/b/depmod" -b "$work/root" 1.0 >/dev/null 2>&1
total=$(FAILMALLOC_COUNT=1 LD_PRELOAD="$work/failmalloc.so" "$work/b/modprobe" -n -v -C "$work/root/etc" -d "$work/root" -S 1.0 foo 2>&1 >/dev/null | grep -a -o 'FAILMALLOC_TOTAL=[0-9]*' | cut -d= -f2)
echo "allocations in a normal run: $total"
bad=0
for n in $(seq 1 "$total"); do
	timeout -s KILL 20 env FAILMALLOC_N=$n LD_PRELOAD="$work/failmalloc.so" "$work/b/modprobe" -n -v -C "$work/root/etc" -d "$work/root" -S 1.0 foo >/dev/null 2>&1
	rc=$?
	if [ "$rc" -ge 128 ]; then echo "failing allocation #$n -> modprobe killed by signal $((rc - 128))"; bad=1; fi
done
exit $bad
