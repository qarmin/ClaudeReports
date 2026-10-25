#!/usr/bin/env bash
# Real, standalone reproduction: compiles repro.c against a real, ASAN+UBSAN-instrumented
# build of gstreamer's own core library (real public API: gst_event_new_sink_message(),
# gst_event_has_name()) and runs it.
#
# Usage: BUILD_DIR=<path to an ASAN-instrumented core-only build> run.sh
set -eo pipefail
# (not -u: gst-env.py's --only-environment output references PYTHONPATH/LD_LIBRARY_PATH
# unconditionally even when unset, which trips `set -u`)
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$HERE/../clone/build-asan-core}"

if [ ! -f "$BUILD_DIR/meson-uninstalled/gstreamer-1.0-uninstalled.pc" ]; then
  echo "no gstreamer-1.0-uninstalled.pc in $BUILD_DIR - build it first (see ../justfile's clone recipe)" >&2
  exit 1
fi

export PKG_CONFIG_PATH="$BUILD_DIR/meson-uninstalled"

clang -g -O0 -fsanitize=address,undefined -fno-sanitize=function \
  $(pkg-config --cflags gstreamer-1.0) "$HERE/repro.c" -o "$HERE/repro" \
  $(pkg-config --libs gstreamer-1.0)

# gstreamer-1.0-uninstalled.pc's own Libs already puts -L on the built gst/ dir, but the
# runtime loader still needs LD_LIBRARY_PATH to find libgstreamer-1.0.so.0 at exec time.
export LD_LIBRARY_PATH="$BUILD_DIR/subprojects/gstreamer/gst:${LD_LIBRARY_PATH:-}"
export ASAN_OPTIONS="${ASAN_OPTIONS:-detect_leaks=0}"

ulimit -c 0
"$HERE/repro"
