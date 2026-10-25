#!/usr/bin/env bash
# Real, standalone reproduction: compiles repro.c against a real, ASAN+UBSAN-instrumented build of
# GStreamer's gst-plugins-base (with the real alsa plugin, ext/alsa/gstalsa*.c, enabled) and runs
# it. repro.c drives the REAL end-to-end trigger path:
#   - the real "device" GObject property on a real alsasink element (g_object_set)
#   - a real GST_STATE_READY transition (real gst_alsasink_open() -> real snd_pcm_open())
#   - a real gst_pad_query_caps() on the element's real sink pad, exactly what a real pipeline
#     does during caps negotiation - this is what actually calls gst_alsasink_getcaps() ->
#     gst_alsa_probe_supported_formats() -> gst_alsa_open_iec958_pcm(), the vulnerable function.
# The only local/self-contained input is a custom ALSA_CONFIG_PATH config file (written by
# repro.c itself) defining a "null"-type PCM under a very long name - the real, supported ALSA
# mechanism a user's own ~/.asoundrc uses, requiring no real audio hardware.
#
# Usage: BUILD_DIR=<path to an ASAN-instrumented gst-plugins-base build with alsa enabled> run.sh [device_name_len]
set -eo pipefail
# (not -u: gst-env.py's --only-environment output references PYTHONPATH/LD_LIBRARY_PATH
# unconditionally even when unset, which trips `set -u`)
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$HERE/../clone/build-asan-alsa}"

if [ ! -f "$BUILD_DIR/meson-uninstalled/gstreamer-1.0-uninstalled.pc" ]; then
  echo "no gstreamer-1.0-uninstalled.pc in $BUILD_DIR - build it first (see ../justfile's clone recipe)" >&2
  exit 1
fi

export PKG_CONFIG_PATH="$BUILD_DIR/meson-uninstalled"

clang -g -O0 -fsanitize=address,undefined -fno-sanitize=function -fno-omit-frame-pointer \
  $(pkg-config --cflags gstreamer-1.0) "$HERE/repro.c" -o "$HERE/repro" \
  $(pkg-config --libs gstreamer-1.0)

# Real dynamic plugin discovery: the repro calls gst_element_factory_make("alsasink", ...), which
# needs the real built libgstalsa.so found via GST_PLUGIN_PATH, exactly like a real application
# run uninstalled against this build tree would (see gst-env.py in the project root).
export GST_PLUGIN_PATH="$BUILD_DIR"
export GST_PLUGIN_SYSTEM_PATH_1_0=""
export LD_LIBRARY_PATH="$BUILD_DIR/subprojects/gstreamer/gst:${LD_LIBRARY_PATH:-}"
export ASAN_OPTIONS="${ASAN_OPTIONS:-detect_leaks=0}"
export UBSAN_OPTIONS="${UBSAN_OPTIONS:-print_stacktrace=1}"

ulimit -c 0
"$HERE/repro" "$@"
