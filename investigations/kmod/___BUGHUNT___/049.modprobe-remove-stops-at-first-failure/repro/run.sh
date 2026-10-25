#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `modprobe -r a e` where a is still used by loaded module d and e is free (kernel side simulated by repro/sysfake.c). Exit 1 = bug reproduced (modprobe dies with "FATAL: Module a is in use." and never removes e, although rmmod_all() is written to continue with the remaining modules).
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0:verify_asan_link_order=0
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
cc -O1 -shared -fPIC -w "$here/sysfake.c" -ldl -o "$work/sysfake.so" || exit 2
T=$work/tree; F=$work/fake
k=$T/lib/modules/1.0/kernel; mkdir -p "$k" "$F/proc" "$work/conf"
touch "$T"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$here" python3 -c "
import sys; from synth import module
for n in 'ade':
    open('%s/%s.ko' % (sys.argv[1], n), 'wb').write(module(['s' + n], []))" "$k"
"$bld/depmod" -b "$T" 1.0 2>/dev/null
for m in a d e; do mkdir -p "$F/sys/module/$m/holders"; echo live > "$F/sys/module/$m/initstate"; echo 0 > "$F/sys/module/$m/refcnt"; done
touch "$F/sys/module/a/holders/d"; echo 1 > "$F/sys/module/a/refcnt"
env SYSFAKE="$F" LD_PRELOAD="$work/sysfake.so" "$bld/modprobe" -d "$T" -S 1.0 -C "$work/conf" -r a e; rc=$?
echo "### exit status $rc, delete_module calls: $(cat "$F/log" 2>/dev/null | tr '\n' ';')"
echo "### still loaded: $(ls "$F/sys/module" | tr '\n' ' ')"
# a stays loaded (in use) so the exit status is non-zero, but e must be gone
[ ! -d "$F/sys/module/e" ] && exit 0
exit 1
