#!/usr/bin/env bash
# build.sh <kmod_src> <asan_build_dir> <out> [extra cflags for the libkmod sources, e.g. -DNDEBUG]
# Harness + libkmod-elf.c + libkmod-signature*.c compiled with coverage (libkmod-internal.a itself
# has no -fsanitize=fuzzer instrumentation, so linking only the .a leaves libFuzzer blind).
set -euo pipefail
here=$(dirname "$(realpath "$0")"); src=$(realpath "$1"); bld=$(realpath "$2"); out=$3; extra=${4:-}
flags=(-g -O1 -fsanitize=fuzzer,address,undefined -fno-sanitize-recover=undefined
       -include "$bld/config.h" -I"$src" -I"$bld" -I"$src/libkmod")
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
for f in libkmod-elf libkmod-signature libkmod-signature-openssl; do
    # shellcheck disable=SC2086
    clang "${flags[@]}" $extra -c "$src/libkmod/$f.c" -o "$__tmp/$f.o"
done
clang "${flags[@]}" -c "$here/fuzz_elf_getters.c" -o "$__tmp/harness.o"
clang "${flags[@]}" "$__tmp/harness.o" "$tmp"/libkmod-*.o \
    "$bld/libkmod-internal.a" "$bld/libshared.a" -ldl -Wl,--allow-multiple-definition -o "$out"
echo "built: $out"
