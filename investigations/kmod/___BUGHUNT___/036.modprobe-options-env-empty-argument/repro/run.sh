#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: `modprobe m0` (dry run) with MODPROBE_OPTIONS holding two consecutive spaces, and with a quoted value. Exit 1 = bug reproduced (the tokenizer yields an empty argument that modprobe takes as the module name).
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
"$bld/depmod" -b "$root" 1.0 2>/dev/null
fail=0
for opts in '-n -v' '-n  -v' "-C \"$root/conf\" -n"; do
	out=$(MODPROBE_OPTIONS="$opts" "$bld/modprobe" -d "$root" -S 1.0 m0 2>&1); rc=$?
	echo "### MODPROBE_OPTIONS='${opts//$root/ROOT}': exit $rc: $(echo "$out" | head -1 | sed "s|$root|ROOT|g")"
	[ $rc != 0 ] && fail=1
done
exit $fail
