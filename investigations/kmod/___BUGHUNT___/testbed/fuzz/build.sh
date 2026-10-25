#!/usr/bin/env bash
# Build every libFuzzer target against the coverage-instrumented testbed build.
#   build.sh [build_dir]   (default ../__build-fuzz)
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${1:-$here/../__build-fuzz}")
src=$(realpath "$here/../_src")
out="$here/bin"
mkdir -p "$out"
CFLAGS=(-g -O1 -fsanitize=fuzzer,address,undefined -fno-sanitize-recover=undefined
	-include "$bld/config.h" -I"$src" -I"$bld" -I"$src/libkmod" -I"$src/tools" -w)
LIBS=("$bld/libkmod-internal.a" "$bld/libshared.a" -ldl)
for t in elf modfile index config builtin util api; do
	extra=()
	[ "$t" = config ] && extra=(-Wl,--wrap=open)
	clang "${CFLAGS[@]}" "$here/fuzz_$t.c" "${LIBS[@]}" "${extra[@]}" -o "$out/fuzz_$t"
done
clang "${CFLAGS[@]}" "$here/fuzz_modprobe_env.c" "$src/tools/log.c" "$src/tools/opt.c" \
	"${LIBS[@]}" -o "$out/fuzz_modprobe_env"
ls "$out"
