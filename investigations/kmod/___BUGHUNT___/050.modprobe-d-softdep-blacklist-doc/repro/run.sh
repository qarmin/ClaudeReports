#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `modprobe -b -n -v c` with `softdep c pre: a post: d` and `blacklist a` / `blacklist d`. modprobe.d(5) says "Flags such as --use-blacklist are applied to all the specified modules". Exit 1 = documented behavior not met (a and d are still inserted).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
k=$root/lib/modules/1.0/kernel; mkdir -p "$k" "$root/conf"
touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
printf 'softdep c pre: a post: d\nblacklist a\nblacklist d\n' > "$root/conf/a.conf"
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
for n in 'acd':
    open('%s/%s.ko' % (sys.argv[1], n), 'wb').write(module(['s' + n], []))" "$k"
"$bld/depmod" -b "$root" 1.0 2>/dev/null
out=$("$bld/modprobe" -d "$root" -S 1.0 -C "$root/conf" -b -n -v c 2>&1 | sed "s|$root/lib/modules/1.0/kernel/||" | tr '\n' ' ')
echo "### modprobe -b -n -v c -> $out"
case "$out" in *"insmod a.ko"*|*"insmod d.ko"*) exit 1;; esac
exit 0
