#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: crafted modules.alias.bin through the real modprobe. Non-zero = bug reproduced.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
python3 "$here/gen_modules_dir.py" "$root"
out=$("$bld/modprobe" -d "$root" -S 1.0 --show-depends foo 2>&1); echo "$out" | head -12
! grep -q "AddressSanitizer" <<<"$out"
