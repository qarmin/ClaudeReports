#!/usr/bin/env bash
# build.sh <kmod_src> <asan_build_dir> <out>
# The harness #includes libkmod/libkmod-index.c so the index reader gets libFuzzer coverage and
# its static functions/structs are reachable; strbuf/util come from the build's static libs.
set -euo pipefail
here=$(dirname "$(realpath "$0")"); src=$(realpath "$1"); bld=$(realpath "$2"); out=$3
clang -g -O1 -fsanitize=fuzzer,address,undefined -fno-sanitize-recover=undefined \
    -include "$bld/config.h" -I"$src" -I"$bld" -I"$src/libkmod" \
    "$here/fuzz_index.c" "$bld/libkmod-internal.a" "$bld/libshared.a" -ldl \
    -Wl,--allow-multiple-definition -o "$out"
echo "built: $out"
