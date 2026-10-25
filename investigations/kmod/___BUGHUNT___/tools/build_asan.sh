#!/usr/bin/env bash
# Configure (once) and build kmod with clang + ASAN/UBSAN.
#   build_asan.sh <kmod_src_dir> <build_dir>
set -euo pipefail
src=$(realpath "$1"); bld=$2
if [ ! -f "$bld/build.ninja" ]; then
    CC=clang meson setup "$bld" "$src" --native-file "$src/build-dev.ini" \
        -Ddocs=false -Dmanpages=false -Dwerror=false >/dev/null
fi
ninja -C "$bld" >/dev/null
echo "built: $bld"
