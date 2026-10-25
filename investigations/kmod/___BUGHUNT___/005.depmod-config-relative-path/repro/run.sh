#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: real `depmod -C t.conf` with a slash-less config file name. Non-zero = bug reproduced.
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0:halt_on_error=1 UBSAN_OPTIONS=halt_on_error=1
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
mkdir -p "$root/lib/modules/1.0"
echo "search extra built-in" > "$root/t.conf"
cd "$root" && "$bld/depmod" -b "$root" -C t.conf 1.0
