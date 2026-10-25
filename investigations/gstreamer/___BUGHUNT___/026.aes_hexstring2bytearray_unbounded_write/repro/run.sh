#!/usr/bin/env bash
# Real, standalone reproduction: compiles repro.c against a real, ASAN+UBSAN-instrumented
# build of GStreamer's own "aes" plugin (real public API: gst_element_factory_make("aesenc"/
# "aesdec"), g_object_set() on the "key"/"iv" properties) and runs it against the real
# built plugin, discovered through the real plugin-scanner registry mechanism (GST_PLUGIN_PATH),
# not a hand-built stand-in.
#
# Usage: BUILD_DIR=<path to build-aes-only> SRC_DIR=<path to the checkout that produced it,
#   for gst-env.py> run.sh [aesenc|aesdec] [key|iv] [hexlen]
set -eo pipefail
# (not -u: gst-env.py's --only-environment output references variables unconditionally
# even when unset, which trips `set -u`)
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$HERE/../clone/build-aes-only}"
SRC_DIR="${SRC_DIR:-$HERE/../clone}"

if [ ! -x "$SRC_DIR/gst-env.py" ]; then
  echo "no gst-env.py at $SRC_DIR - build it first (see ../justfile's clone recipe)" >&2
  exit 1
fi

ELEM="${1:-aesenc}"
PROP="${2:-key}"
HEXLEN="${3:-400}"

if [ ! -f "$BUILD_DIR/meson-uninstalled/gstreamer-1.0-uninstalled.pc" ]; then
  echo "no gstreamer-1.0-uninstalled.pc in $BUILD_DIR - build it first (see ../justfile's clone recipe)" >&2
  exit 1
fi

export PKG_CONFIG_PATH="$BUILD_DIR/meson-uninstalled"

clang -g -O0 -fsanitize=address,undefined -fno-sanitize=function \
  $(pkg-config --cflags gstreamer-1.0) "$HERE/repro.c" -o "$HERE/repro" \
  $(pkg-config --libs gstreamer-1.0)

ENV_FILE="$(mktemp)"
trap 'rm -f "$ENV_FILE"' EXIT
python3 "$SRC_DIR/gst-env.py" --builddir "$BUILD_DIR" --only-environment > "$ENV_FILE"
# shellcheck source=/dev/null
source "$ENV_FILE"

# gstreamer-1.0-uninstalled.pc's own Libs already puts -L on the built gst/ dir, but the
# runtime loader still needs LD_LIBRARY_PATH to find libgstreamer-1.0.so.0 at exec time -
# gst-env.py's --only-environment output above already includes it, keep it.
export ASAN_OPTIONS="${ASAN_OPTIONS:-detect_leaks=0}"

ulimit -c 0
"$HERE/repro" "$ELEM" "$PROP" "$HEXLEN"
