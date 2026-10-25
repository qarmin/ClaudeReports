#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `modprobe -c` must print "pre: ... post: ..." separated by a space. Non-zero = bug reproduced.
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
cfg=$(mktemp -d); trap 'rm -rf "$cfg"' EXIT
printf 'softdep x pre: p1 p2 post: q1 q2\n' > "$cfg/a.conf"
out=$("$bld/modprobe" -C "$cfg" -c 2>&1 | grep '^softdep')
echo "config : $(cat "$cfg/a.conf")"
echo "printed: $out"
[ "$out" = "softdep x pre: p1 p2 post: q1 q2" ]
