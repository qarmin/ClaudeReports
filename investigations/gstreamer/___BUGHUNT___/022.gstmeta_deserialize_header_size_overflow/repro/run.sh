#!/usr/bin/env bash
# Build+run repro.c against a real core-only ASAN+UBSAN build of libgstreamer-1.0.
#
# Usage: BUILD_DIR=<meson build dir with subprojects/gstreamer/gst/libgstreamer-1.0.so* built> run.sh
set -eo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$HERE/../clone/build-asan}"

LIBDIR="$BUILD_DIR/subprojects/gstreamer/gst"
if ! compgen -G "$LIBDIR/libgstreamer-1.0.so*" > /dev/null; then
  echo "no libgstreamer-1.0.so in $LIBDIR - build it first" >&2
  exit 1
fi

export PATH="$HOME/.local/bin:$PATH"
PKGFLAGS=$(PKG_CONFIG_PATH="$BUILD_DIR/meson-uninstalled" pkg-config --cflags --libs gstreamer-1.0)

clang -g -fsanitize=address,undefined -fno-sanitize=function \
  -o "$HERE/repro" "$HERE/repro.c" $PKGFLAGS

ulimit -c 0
LD_LIBRARY_PATH="$LIBDIR" ASAN_OPTIONS="${ASAN_OPTIONS:-detect_leaks=0}" "$HERE/repro"
