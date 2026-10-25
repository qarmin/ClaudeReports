#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: real `modprobe --show-modversions` on a .ko whose __versions entry has an unterminated name. Non-zero = leak reproduced.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=1
out=$("$bld/modprobe" --show-modversions "$here/bad-versions.ko" 2>&1); echo "$out" | head -8
! grep -q LeakSanitizer <<<"$out"
