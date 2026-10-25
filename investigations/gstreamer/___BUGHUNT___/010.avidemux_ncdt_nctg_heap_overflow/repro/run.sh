#!/usr/bin/env bash
# Real, standalone reproduction: a genuine GStreamer pipeline (filesrc ! avidemux ! fakesink)
# processing a crafted .avi file through the actual gst-launch-1.0 binary - not a hand-built
# stand-in for avidemux's internals.
#
# Usage: BUILD_DIR=<path to an ASAN-instrumented build with gst-plugins-good:avi=enabled> run.sh
set -eo pipefail
# (not -u: gst-env.py's --only-environment output references PYTHONPATH/LD_LIBRARY_PATH
# unconditionally even when unset, which trips `set -u`)
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$HERE/../../../build-asan-fuzz}"

if [ ! -f "$BUILD_DIR/subprojects/gstreamer/tools/gst-launch-1.0" ]; then
  echo "no gst-launch-1.0 in $BUILD_DIR - build it first (see ../../tools/build_asan_fuzz.sh)" >&2
  exit 1
fi

export PATH="$HOME/.local/bin:$PATH"
eval "$(cd "$BUILD_DIR/.." && python3 gst-env.py --builddir="$(basename "$BUILD_DIR")" --only-environment)"

if [ ! -f "$HERE/evil.avi" ]; then
  python3 "$HERE/craft_avi.py" "$HERE/base_avi_h264_pcm.avi" "$HERE/evil.avi"
fi

ulimit -c 0
ASAN_OPTIONS="${ASAN_OPTIONS:-detect_leaks=0}" \
  "$BUILD_DIR/subprojects/gstreamer/tools/gst-launch-1.0" \
  filesrc location="$HERE/evil.avi" ! avidemux ! fakesink
