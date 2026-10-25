#!/usr/bin/env bash
# Compiles the given checkout's real, unmodified ext/hls/m3u8.c together with repro.c
# (clang + ASAN/UBSAN), links against a system-installed gstreamer-1.0 (core only - m3u8.c
# needs nothing beyond glib + stable core API: gst_uri_is_valid(), GST_DEBUG_CATEGORY,
# GMutex, g_atomic_int, GstClockTime, GList), and runs it.
#
# Usage: SRC_ROOT=<path to a gstreamer checkout> run.sh
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_ROOT="${SRC_ROOT:?set SRC_ROOT to the checkout to build against}"
HLSDIR="$SRC_ROOT/subprojects/gst-plugins-bad/ext/hls"

if [ ! -f "$HLSDIR/m3u8.c" ]; then
  echo "no $HLSDIR/m3u8.c - is SRC_ROOT a gstreamer checkout?" >&2
  exit 1
fi

if ! pkg-config --exists gstreamer-1.0; then
  echo "gstreamer-1.0 not found via pkg-config - install libgstreamer1.0-dev (or similar)" >&2
  exit 1
fi

BIN="$HERE/repro_bin"
clang -g -O0 -fsanitize=address,undefined -fno-sanitize=function \
  -I "$HLSDIR" \
  $(pkg-config --cflags gstreamer-1.0) \
  "$HERE/repro.c" "$HLSDIR/m3u8.c" \
  $(pkg-config --libs gstreamer-1.0) \
  -o "$BIN"

ulimit -c 0
ASAN_OPTIONS="${ASAN_OPTIONS:-detect_leaks=0}" GST_DEBUG=0 "$BIN"
