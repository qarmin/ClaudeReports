#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: modinfo -F with upper-case field names; modinfo(8) says "Field names are case-insensitive". Exit 1 = bug reproduced (an upper-case field name prints nothing).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
d=$(mktemp -d); trap 'rm -rf "$d"' EXIT
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
open(sys.argv[1] + '/m.ko', 'wb').write(module([], [], ['license=GPL', 'author=Me', 'parm=a:desc', 'parmtype=a:int']))" "$d"
fail=0
for f in license LICENSE License author AUTHOR filename FILENAME parm PARM; do
	out=$("$bld/modinfo" -F "$f" "$d/m.ko" | tr '\n' '|' | sed "s|$d|D|")
	echo "### -F $f -> '$out'"
	[ -z "$out" ] && fail=1
done
exit $fail
