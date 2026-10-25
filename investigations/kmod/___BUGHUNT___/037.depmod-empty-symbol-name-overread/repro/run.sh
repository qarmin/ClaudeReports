#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: depmod -e -F on a tree holding one crafted module whose __versions/.symtab import a symbol with an empty name. Exit 1 = bug reproduced (AddressSanitizer heap-buffer-overflow in depmod).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
mkdir -p "$root/lib/modules/1.0/kernel"
touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
open(sys.argv[1] + '/lib/modules/1.0/kernel/m0.ko', 'wb').write(module(['x'], ['', 'x']))" "$root"
out=$("$bld/depmod" -e -F /dev/null -b "$root" 1.0 2>&1); rc=$?
echo "$out" | grep -E "ERROR: AddressSanitizer|^READ|^    #[0-4] " | sed 's|0x[0-9a-f]* ||'
echo "### depmod exit status $rc"
if echo "$out" | grep -q AddressSanitizer; then exit 1; fi
