#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `kmod list` (lsmod) for a module whose /sys/module/<name>/refcnt does not exist (kernel without module unloading support; kernel side simulated by repro/sysfake.c). Exit 1 = bug reproduced (the "Used by" column shows the raw -errno "-2").
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0:verify_asan_link_order=0
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
cc -O1 -shared -fPIC -w "$here/sysfake.c" -ldl -o "$work/sysfake.so" || exit 2
F=$work/fake; mkdir -p "$F/proc"
for m in a b; do mkdir -p "$F/sys/module/$m/holders"; echo live > "$F/sys/module/$m/initstate"; done
echo 0 > "$F/sys/module/b/refcnt"
printf 'a 4096 -1 - Live 0x0\nb 4096 0 - Live 0x0\n' > "$F/proc/modules"
out=$(env SYSFAKE="$F" LD_PRELOAD="$work/sysfake.so" "$bld/kmod" list)
echo "$out"
echo "$out" | awk 'NR>1 && $1=="a" {print $3}' | grep -q '^-[0-9]' && exit 1
exit 0
