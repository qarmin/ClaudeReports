#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir> [k] [cap_mb]: real depmod on k modules forming one dense dependency cycle (default 20) must finish within cap_mb (default 1024) of RAM. Non-zero = cap hit (memory blow-up reproduced).
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
k=${3:-20}; cap=${4:-1024}
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
python3 "$here/gen_tree.py" "$root" "$k"
timeout 120 "$here/memcap.sh" "$cap" "$bld/depmod" -b "$root" 1.0 >/dev/null 2>"$root/err"; rc=$?
echo "k=$k cap=${cap}MB: depmod exit code $rc, $(wc -l < "$root/err") lines on stderr"
grep -v "Cycle detected" "$root/err" | head -3 | cut -c1-160
[ "$rc" -ne 137 ] && [ "$rc" -ne 124 ]
