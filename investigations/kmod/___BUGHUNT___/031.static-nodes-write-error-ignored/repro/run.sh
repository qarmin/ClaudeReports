#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `kmod static-nodes -o /dev/full` must fail. Non-zero = bug reproduced (exit status 0 although the output could not be written). The build must use -Dmoduledir=<scratch dir> (the justfile does).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
moddir=$(sed -n 's/^#define MODULE_DIRECTORY "\(.*\)"/\1/p' "$bld/config.h")
[ "$moddir" != /lib/modules ] || { echo "build must use -Dmoduledir=<scratch dir>"; exit 2; }
mkdir -p "$moddir/$(uname -r)"
printf 'snd_foo snd/foo c14:3\nsnd_bar bar b7:0\n' > "$moddir/$(uname -r)/modules.devname"
"$bld/kmod" static-nodes -f tmpfiles -o /dev/full >/dev/null; rc=$?
echo "kmod static-nodes -o /dev/full: exit code $rc"
[ "$rc" -ne 0 ]
