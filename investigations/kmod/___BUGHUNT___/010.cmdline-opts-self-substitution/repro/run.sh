#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: real modprobe running an install command whose options contain the literal $CMDLINE_OPTS. Non-zero = unbounded growth reproduced.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0:hard_rss_limit_mb=500
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
python3 "$here/gen_modules_dir.py" "$root"
out=$(timeout 30 "$bld/modprobe" -C "$root/etc" -d "$root" -S 1.0 foo 'a=$CMDLINE_OPTS' 2>&1); rc=$?
echo "$out" | head -2
echo "modprobe exit code: $rc"
! grep -q "hard rss limit exhausted" <<<"$out" && [ "$rc" -ne 124 ]
