#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: modinfo / modprobe -c / modprobe -D / depmod -n with stdout redirected to /dev/full (every write fails with ENOSPC). Exit 1 = bug reproduced (a tool exits 0 although its output was lost).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
k=$root/lib/modules/1.0/kernel
mkdir -p "$k"
touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
open(sys.argv[1] + '/m0.ko', 'wb').write(module(['s0'], [], ['license=GPL', 'alias=x']))" "$k"
"$bld/depmod" -b "$root" 1.0 || exit 2
fail=0
while IFS= read -r line; do
	cmd=${line//@ROOT@/$root}
	eval "\"$bld/\"$cmd" >/dev/full 2>/dev/null; rc=$?
	echo "### $line > /dev/full: exit status $rc"
	[ "$rc" = 0 ] && fail=1
done <<'CMDS'
modinfo -b @ROOT@ -k 1.0 m0
modprobe -d @ROOT@ -S 1.0 -c
modprobe -d @ROOT@ -S 1.0 -D m0
depmod -n -b @ROOT@ 1.0
CMDS
exit $fail
