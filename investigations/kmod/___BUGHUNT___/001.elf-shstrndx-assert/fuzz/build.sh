#!/usr/bin/env bash
# build.sh <kmod_src> <asan_build_dir> <out>: harness + libkmod-elf.c compiled with coverage
# (libkmod-internal.a itself is not coverage-instrumented, so linking only the .a gives cov: 2).
set -euo pipefail
here=$(dirname "$(realpath "$0")"); src=$(realpath "$1"); bld=$(realpath "$2")
clang -g -O1 -fsanitize=fuzzer,address,undefined -fno-sanitize-recover=undefined \
    -include "$bld/config.h" -I"$src" -I"$bld" -I"$src/libkmod" \
    "$here/fuzz_elf_new.c" "$src/libkmod/libkmod-elf.c" \
    "$bld/libkmod-internal.a" "$bld/libshared.a" -ldl -Wl,--allow-multiple-definition -o "$3"
echo "built: $3"
