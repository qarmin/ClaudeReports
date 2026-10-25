#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir> [k]: real depmod on k synthetic modules forming overlapping cycles. Non-zero = assertion/crash reproduced.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
python3 "$here/gen_tree.py" "$root" "${3:-3}"
out=$(timeout 60 "$here/memcap.sh" 2048 "$bld/depmod" -b "$root" 1.0 2>&1); rc=$?
grep -v WARNING <<<"$out" | head -4 | cut -c1-200
echo "depmod exit code: $rc"
! grep -qE "Assertion|AddressSanitizer" <<<"$out" && [ "$rc" -ne 124 ]
