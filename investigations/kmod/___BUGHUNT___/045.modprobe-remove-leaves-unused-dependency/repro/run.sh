#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `modprobe -r c` where c uses b and b uses a, all loaded (kernel interface simulated by repro/sysfake.c via LD_PRELOAD: /sys/module, /proc/modules, delete_module). Exit 1 = bug reproduced (a stays loaded although nothing uses it any more).
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0:verify_asan_link_order=0
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
cc -O1 -shared -fPIC -w "$here/sysfake.c" -ldl -o "$work/sysfake.so" || exit 2
T=$work/tree; F=$work/fake
k=$T/lib/modules/1.0/kernel; mkdir -p "$k" "$F/proc"
touch "$T"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$here" python3 -c "
import sys; from synth import module
k = sys.argv[1]
for n, imp in (('a', []), ('b', ['sa']), ('c', ['sb'])):
    open('%s/%s.ko' % (k, n), 'wb').write(module(['s' + n], imp))" "$k"
"$bld/depmod" -b "$T" 1.0 2>/dev/null
for m in a b c; do mkdir -p "$F/sys/module/$m/holders"; echo live > "$F/sys/module/$m/initstate"; done
touch "$F/sys/module/a/holders/b" "$F/sys/module/b/holders/c"
echo 1 > "$F/sys/module/a/refcnt"; echo 1 > "$F/sys/module/b/refcnt"; echo 0 > "$F/sys/module/c/refcnt"
env SYSFAKE="$F" LD_PRELOAD="$work/sysfake.so" "$bld/modprobe" -d "$T" -S 1.0 -v -r c
echo "### delete_module calls: $(tr '\n' ';' < "$F/log")"
left=$(ls "$F/sys/module" | tr '\n' ' ')
echo "### still loaded: ${left:-nothing}"
[ -z "$left" ] || exit 1
exit 0
