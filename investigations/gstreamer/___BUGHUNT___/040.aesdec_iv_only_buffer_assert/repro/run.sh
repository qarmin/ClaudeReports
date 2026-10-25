#!/usr/bin/env bash
# Real, standalone reproduction: builds a real GStreamer pipeline (appsrc ! aesdec ! fakesink /
# appsink), with the real "aesdec" element loaded through the real plugin-scanner/registry
# mechanism (GST_PLUGIN_PATH), and pushes a real GstBuffer into it via appsrc's public
# push-buffer API - exactly the path a real upstream element (network source, file source doing
# a short first read) drives when it hands aesdec its first buffer. No hand-built stand-in for
# the pipeline/pad-push machinery is used anywhere.
#
# Usage: BUILD_DIR=<path to build-aes> SRC_DIR=<path to the checkout that produced it,
#   for gst-env.py> run.sh [repro|boundary]
set -eo pipefail
# (not -u: gst-env.py's --only-environment output references variables unconditionally even
# when unset, which trips `set -u`)
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$HERE/../clone/build-aes}"
SRC_DIR="${SRC_DIR:-$HERE/../clone}"

if [ ! -x "$SRC_DIR/gst-env.py" ]; then
  echo "no gst-env.py at $SRC_DIR - build it first (see ../justfile's clone recipe)" >&2
  exit 1
fi

WHICH="${1:-repro}"

if [ ! -f "$BUILD_DIR/meson-uninstalled/gstreamer-1.0-uninstalled.pc" ]; then
  echo "no gstreamer-1.0-uninstalled.pc in $BUILD_DIR - build it first (see ../justfile's clone recipe)" >&2
  exit 1
fi

export PKG_CONFIG_PATH="$BUILD_DIR/meson-uninstalled"

gcc -g -O0 \
  $(pkg-config --cflags gstreamer-app-1.0 gstreamer-1.0) "$HERE/$WHICH.c" -o "$HERE/$WHICH" \
  $(pkg-config --libs gstreamer-app-1.0 gstreamer-1.0)

ENV_FILE="$(mktemp)"
trap 'rm -f "$ENV_FILE"' EXIT
python3 "$SRC_DIR/gst-env.py" --builddir "$BUILD_DIR" --only-environment > "$ENV_FILE"
# shellcheck source=/dev/null
source "$ENV_FILE"

export GST_STATE_IGNORE_ELEMENTS=""

ulimit -c 0
"$HERE/$WHICH"
