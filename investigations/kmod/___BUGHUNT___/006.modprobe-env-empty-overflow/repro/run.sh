#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: real `modprobe -V` with MODPROBE_OPTIONS set to the empty string. Non-zero = bug reproduced.
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0:halt_on_error=1
MODPROBE_OPTIONS= "$bld/modprobe" -V
