#!/usr/bin/env bash
# Build the testbed's helper drivers against a kmod build dir (default ../__build-fuzz, ASAN+UBSAN).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
bld=$(realpath "${1:-$here/../__build-fuzz}")
src=$(realpath "$here/../_src")
mkdir -p "$here/bin"
for d in kinfo idxcheck; do
	clang -g -O1 -fsanitize=address,undefined -fno-sanitize-recover=undefined -I"$src" \
		"$here/$d.c" "$bld/libkmod-internal.a" "$bld/libshared.a" -ldl -o "$here/bin/$d"
done
clang -g -O1 -fsanitize=address,undefined -fno-sanitize-recover=undefined -w \
	-include "$bld/config.h" -I"$src" -I"$bld" -I"$src/tools" "$here/envtok.c" "$src/tools/log.c" \
	"$src/tools/opt.c" "$bld/libkmod-internal.a" "$bld/libshared.a" -ldl -o "$here/bin/envtok"
ls "$here/bin"
