#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: kmod_module_probe_insert_module(m, DRY_RUN|APPLY_BLACKLIST_ALL) where m's dependency d is blacklisted. Exit 1 = bug reproduced (the documented early return does not happen; m is probed without d).
set -u
here=$(cd "$(dirname "$0")" && pwd)
src=$(realpath "${1:?kmod src dir}"); bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)"):$bld ASAN_OPTIONS=detect_leaks=0
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
root=$work/root; d=$root/lib/modules/1.0; k=$d/kernel; mkdir -p "$k" "$work/conf"
touch "$d"/modules.{order,builtin,builtin.modinfo}
echo 'blacklist d' > "$work/conf/a.conf"
PYTHONPATH="$here" python3 -c "
import sys; from synth import module
open('%s/d.ko' % sys.argv[1], 'wb').write(module(['sd'], []))
open('%s/m.ko' % sys.argv[1], 'wb').write(module(['sm'], ['sd']))" "$k"
"$bld/depmod" -b "$root" 1.0 2>/dev/null
cc -w -I"$src/libkmod" "$here/repro.c" -L"$bld" -lkmod -Wl,-rpath,"$bld" -o "$work/repro" 2>/dev/null || exit 2
rt=$(ldd "$bld/libkmod.so" | awk '/asan/ {print $3}')
LD_PRELOAD=$rt "$work/repro" "$d" "$work/conf"
