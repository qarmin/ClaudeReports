#!/usr/bin/env bash
# Real, standalone reproduction: calls the actual public GStreamer core API
# (gst_meta_register_custom_simple -> gst_buffer_add_meta -> gst_meta_serialize_simple, which
# internally calls the real gst_meta_serialize() in subprojects/gstreamer/gst/gstmeta.c) with a
# custom meta whose name is >= 246 bytes long. No plugin, no media file, no second participant to
# fake - this is a single-process API-misuse bug entirely within gstreamer core.
#
# Usage: BUILD_DIR=<path to an ASAN-instrumented core-only build, see ../../tools/build_asan_core.sh>
#        NAME_LEN=<override, default 246> run.sh
set -eo pipefail
# (not -u: gst-env.py's --only-environment output references some vars unconditionally even when
# unset, which trips `set -u`)
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$HERE/../../../build-asan-core}"
NAME_LEN="${NAME_LEN:-246}"

if [ ! -f "$BUILD_DIR/subprojects/gstreamer/gst/libgstreamer-1.0.so.0" ] && \
   [ ! -f "$BUILD_DIR/subprojects/gstreamer/gst/libgstreamer-1.0.so" ]; then
  echo "no libgstreamer-1.0 in $BUILD_DIR - build it first (see ../../tools/build_asan_core.sh)" >&2
  exit 1
fi

export PATH="$HOME/.local/bin:$PATH"

BUILD_DIR_ABS="$(cd "$BUILD_DIR" && pwd)"
eval "$(cd "$BUILD_DIR_ABS/.." && python3 gst-env.py --builddir="$(basename "$BUILD_DIR_ABS")" --only-environment)"
export PKG_CONFIG_PATH="$BUILD_DIR_ABS/meson-uninstalled:${PKG_CONFIG_PATH:-}"

BIN="$HERE/repro_bin"
clang -g -fsanitize=address,undefined -fno-sanitize=function \
  -o "$BIN" "$HERE/repro.c" $(pkg-config --cflags --libs gstreamer-1.0)

ulimit -c 0
ASAN_OPTIONS="${ASAN_OPTIONS:-halt_on_error=1:abort_on_error=1}" \
UBSAN_OPTIONS="${UBSAN_OPTIONS:-halt_on_error=1}" \
  "$BIN" "$NAME_LEN"
