#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: modprobe with an `install` / `remove` command that is killed by a signal. Exit 1 = bug reproduced (modprobe exits 0 as if the command had succeeded).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
mkdir -p "$root/lib/modules/1.0/kernel" "$root/conf"
touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
open(sys.argv[1] + '/lib/modules/1.0/kernel/m0.ko', 'wb').write(module(['s0'], []))" "$root"
"$bld/depmod" -b "$root" 1.0
fail=0
for kind in install remove; do
	for body in 'exit 3' 'kill -9 $$'; do
		echo "$kind m0 $body" > "$root/conf/a.conf"
		flag=""; [ "$kind" = remove ] && flag="-r"
		"$bld/modprobe" -d "$root" -S 1.0 -C "$root/conf" $flag m0 >/dev/null 2>&1; rc=$?
		echo "### $kind command '$body': modprobe exit status $rc"
		[ "$body" != 'exit 3' ] && [ "$rc" = 0 ] && fail=1
	done
done
exit $fail
