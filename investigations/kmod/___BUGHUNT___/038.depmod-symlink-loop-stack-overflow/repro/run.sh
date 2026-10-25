#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: depmod on a module tree containing a directory symlink that points to its own parent (kernel/loop -> .). Exit 1 = bug reproduced (depmod dies from a signal / stack overflow instead of finishing).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
mkdir -p "$root/lib/modules/1.0/kernel"
touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
open(sys.argv[1] + '/lib/modules/1.0/kernel/m0.ko', 'wb').write(module(['s0'], []))" "$root"
ln -s . "$root/lib/modules/1.0/kernel/loop"
echo "### ulimit -n $(ulimit -n), ulimit -s $(ulimit -s)"
out=$(timeout -s KILL 60 "$bld/depmod" -b "$root" 1.0 2>&1); rc=$?
echo "$out" | grep -E "ERROR: AddressSanitizer|^    #[0-2] " | sed 's/0x[0-9a-f]* //'
echo "### depmod exit status $rc ($(( rc > 128 ? rc - 128 : 0 )) = signal), modules.dep written: $([ -s "$root/lib/modules/1.0/modules.dep" ] && echo yes || echo no)"
if [ "$rc" -gt 128 ] || echo "$out" | grep -q "stack-overflow"; then exit 1; fi
exit 0
