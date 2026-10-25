#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: builds repro.c against the build's libkmod and runs it. Non-zero = fd leak reproduced.
set -u
here=$(cd "$(dirname "$0")" && pwd)
src=$(realpath "${1:?src dir}"); bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$bld:$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
out=$(mktemp -d); trap 'rm -rf "$out"' EXIT
clang -fsanitize=address -shared-libasan -I"$src/libkmod" "$here/repro.c" -L"$bld" -lkmod -o "$out/repro" || exit 2
"$out/repro"
