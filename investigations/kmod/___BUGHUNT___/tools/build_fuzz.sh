#!/usr/bin/env bash
# Build a libFuzzer harness against kmod's internal static libs (needs an ASAN build dir first).
#   build_fuzz.sh <kmod_src_dir> <asan_build_dir> <harness.c> <out_binary>
set -euo pipefail
src=$(realpath "$1"); bld=$(realpath "$2")
clang -g -O1 -fsanitize=fuzzer,address,undefined -fno-sanitize-recover=undefined \
    -include "$bld/config.h" -I"$src" -I"$bld" -I"$src/libkmod" \
    "$3" "$bld/libkmod-internal.a" "$bld/libshared.a" -ldl -o "$4"
echo "built: $4"
