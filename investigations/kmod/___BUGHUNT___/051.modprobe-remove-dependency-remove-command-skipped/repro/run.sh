#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `modprobe -r c` where c uses b and the configuration has `remove b <command>` (kernel side simulated by repro/sysfake.c). modprobe(8) for -i: "any dependent modules are still subject to commands set for them in the configuration file". Exit 1 = bug reproduced (b is unloaded behind the back of its remove command, which never runs).
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
echo "remove b echo remove-command-of-b-ran >> $work/ran" > "$work/conf/a.conf"
PYTHONPATH="$here" python3 -c "
import sys; from synth import module
open('%s/b.ko' % sys.argv[1], 'wb').write(module(['sb'], []))
open('%s/c.ko' % sys.argv[1], 'wb').write(module(['sc'], ['sb']))" "$k"
"$bld/depmod" -b "$T" 1.0 2>/dev/null
for m in b c; do mkdir -p "$F/sys/module/$m/holders"; echo live > "$F/sys/module/$m/initstate"; done
touch "$F/sys/module/b/holders/c"; echo 1 > "$F/sys/module/b/refcnt"; echo 0 > "$F/sys/module/c/refcnt"
env SYSFAKE="$F" LD_PRELOAD="$work/sysfake.so" "$bld/modprobe" -d "$T" -S 1.0 -C "$work/conf" -r c
echo "### delete_module calls: $(cat "$F/log" 2>/dev/null | tr '\n' ';')"
echo "### remove command of b ran: $([ -s "$work/ran" ] && echo yes || echo no)"
[ -s "$work/ran" ] && exit 0
exit 1
