#!/usr/bin/env bash
# Real, standalone reproduction: directly drives the actual, unmodified gst_m3u8_update() entry
# point in subprojects/gst-plugins-bad/ext/hls/m3u8.c (the same function gsthlsdemux.c calls with
# downloaded playlist text) with two hand-crafted-but-well-formed M3U8 playlists - the genuine
# external input format for this parser, no plugin load, no second network participant to fake
# (see CONFIRMATION.md section 11: "a malformed file handed to a parser's real public entry
# point IS the genuine external input").
#
# repro.c follows this project's own unit-test convention (see
# subprojects/gst-plugins-bad/tests/check/elements/hlsdemux_m3u8.c) of #include-ing m3u8.c
# directly to reach its internal statics/struct layout - it needs only gstreamer-1.0 core (no
# gst-plugins-bad plugin build required), so this uses the cheaper core-only ASAN build.
#
# Usage: BUILD_DIR=<path to an ASAN-instrumented core-only build, see ../../tools/build_asan_core.sh>
#        M3U8_SRC_DIR=<path to the ext/hls source directory to compile against> run.sh
set -eo pipefail
# (not -u: gst-env.py's --only-environment output references some vars unconditionally even when
# unset, which trips `set -u`)
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$HERE/../../../build-asan-core}"
M3U8_SRC_DIR="${M3U8_SRC_DIR:-$HERE/../../../subprojects/gst-plugins-bad/ext/hls}"

if [ ! -f "$BUILD_DIR/subprojects/gstreamer/gst/libgstreamer-1.0.so.0" ] && \
   [ ! -f "$BUILD_DIR/subprojects/gstreamer/gst/libgstreamer-1.0.so" ]; then
  echo "no libgstreamer-1.0 in $BUILD_DIR - build it first (see ../../tools/build_asan_core.sh)" >&2
  exit 1
fi

if [ ! -f "$M3U8_SRC_DIR/m3u8.c" ]; then
  echo "no m3u8.c found under $M3U8_SRC_DIR" >&2
  exit 1
fi

export PATH="$HOME/.local/bin:$PATH"

BUILD_DIR_ABS="$(cd "$BUILD_DIR" && pwd)"
eval "$(cd "$BUILD_DIR_ABS/.." && python3 gst-env.py --builddir="$(basename "$BUILD_DIR_ABS")" --only-environment)"
export PKG_CONFIG_PATH="$BUILD_DIR_ABS/meson-uninstalled:${PKG_CONFIG_PATH:-}"

BIN="$HERE/repro_bin"
clang -g -O0 -fsanitize=address,undefined -fno-sanitize=function \
  -I"$M3U8_SRC_DIR" \
  -o "$BIN" "$HERE/repro.c" $(pkg-config --cflags --libs gstreamer-1.0)

ulimit -c 0
ASAN_OPTIONS="${ASAN_OPTIONS:-halt_on_error=1:abort_on_error=1}" \
UBSAN_OPTIONS="${UBSAN_OPTIONS:-halt_on_error=1}" \
  "$BIN"
