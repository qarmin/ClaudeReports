#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: depmod into a directory it cannot write to must fail. Non-zero = bug reproduced (depmod exit status 0 although no index was written). Needs a non-root user.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
[ "$(id -u)" -ne 0 ] || { echo "run as a non-root user"; exit 2; }
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'chmod -R u+w "$root"; rm -rf "$root"' EXIT
mkdir -p "$root/lib/modules/1.0/kernel"
PYTHONPATH="$here" python3 -c "
import sys; from synth import module
open(sys.argv[1], 'wb').write(module(['s'], [], modinfo=['license=GPL']))" "$root/lib/modules/1.0/kernel/foo.ko"
chmod a-w "$root/lib/modules/1.0"
"$bld/depmod" -b "$root" 1.0 >"$root/out.txt" 2>&1; rc=$?
grep -v WARNING "$root/out.txt" | sort | uniq -c | head -3
echo "depmod exit code: $rc, index files written: $(ls "$root/lib/modules/1.0" | grep -c '^modules\.')"
[ "$rc" -ne 0 ]
