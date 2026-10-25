#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: modprobe m2 where m2 -> m1 -> m0 are hard dependencies and the config has `softdep m2 pre: m0` plus install commands (append to a log, so no root is needed) for m0 and m1. Non-zero = bug reproduced (the install command of m0 ran more than once).
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
mkdir -p "$root/lib/modules/1.0/kernel" "$root/conf"
PYTHONPATH="$here" python3 -c "
import sys; from synth import module
k = sys.argv[1] + '/lib/modules/1.0/kernel/'
open(k + 'm0.ko', 'wb').write(module(['s0'], []))
open(k + 'm1.ko', 'wb').write(module(['s1'], ['s0']))
open(k + 'm2.ko', 'wb').write(module(['s2'], ['s1']))" "$root"
"$bld/depmod" -b "$root" 1.0 2>/dev/null
cat > "$root/conf/a.conf" <<CONF
softdep m2 pre: m0
install m0 echo m0 >> $root/log
install m1 echo m1 >> $root/log
CONF
mp=("$bld/modprobe" -d "$root" -S 1.0 -C "$root/conf")
echo "### dry run (modprobe -n -v m2)"
"${mp[@]}" -n -v m2 2>&1 | sed "s|$root|ROOT|"
"${mp[@]}" m2 >/dev/null 2>&1
n=$(grep -c '^m0$' "$root/log")
echo "### real run: install command of m0 executed $n time(s)"
[ "$n" -le 1 ]
