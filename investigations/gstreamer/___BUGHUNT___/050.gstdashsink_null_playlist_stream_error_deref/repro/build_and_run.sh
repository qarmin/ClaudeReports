#!/usr/bin/env bash
# Builds+runs repro.c against a scoped gstreamer checkout at $SRC_DIR and runs it - should
# SIGSEGV (that IS the repro, before the fix).
#
# gst_dash_sink_write_mpd_file() (and gst_dash_sink_get_type()) are not part of any installed/
# public header, so this compiles ext/dash/*.c fresh (default visibility, stripped of the
# plugin's -fvisibility=hidden) alongside repro.c and links directly, rather than going through
# libgstdash.so - the standard workaround this project uses for testing GStreamer plugin internals
# that can't be linked from outside the .so. gstplugin.c (plugin_init/registration boilerplate)
# and gstdashdemux.c (not needed by dashsink, and pulls in a large chunk of unrelated demux code)
# are excluded from the compiled-together set; gstmpdparser.c still references gstdashdemux.c's
# `gst_dash_demux_debug` category, so repro.c provides a stub definition for it.
#
# Usage: SRC_DIR=<gstreamer checkout> bash build_and_run.sh
set -eo pipefail

SRC_DIR="${SRC_DIR:?set SRC_DIR to the gstreamer checkout}"
export PATH="$HOME/.local/bin:$PATH"

cd "$SRC_DIR"
if [ ! -d build-dash-repro ]; then
  meson setup \
      --buildtype=debug \
      -Dglib:tests=false \
      -Ddoc=disabled \
      -Dexamples=disabled \
      -Dintrospection=disabled \
      -Dgood=disabled \
      -Dugly=disabled \
      -Dlibav=disabled \
      -Dges=disabled \
      -Dsharp=disabled \
      -Drs=disabled \
      -Dpython=disabled \
      -Dlibnice=disabled \
      -Ddevtools=disabled \
      -Drtsp_server=disabled \
      -Dgst-examples=disabled \
      -Dqt5=disabled \
      -Dbase=enabled \
      -Dbad=enabled \
      -Dgst-plugins-bad:auto_features=disabled \
      -Dgst-plugins-bad:dash=enabled \
      build-dash-repro
fi
ninja -C build-dash-repro

DASH_SRC="$SRC_DIR/subprojects/gst-plugins-bad/ext/dash"
BUILD="$SRC_DIR/build-dash-repro"
OBJDIR="$(mktemp -d)"
trap 'rm -rf "$OBJDIR"' EXIT

python3 gst-env.py --builddir=build-dash-repro --only-environment > build-dash-repro/env.sh
# shellcheck disable=SC1091
source build-dash-repro/env.sh
export PKG_CONFIG_PATH="$BUILD/meson-uninstalled"

CFLAGS=$(pkg-config --cflags glib-2.0 gobject-2.0 gio-2.0 gstreamer-1.0 gstreamer-base-1.0 gstreamer-pbutils-1.0 libxml-2.0)
LIBS=$(pkg-config --libs glib-2.0 gobject-2.0 gio-2.0 gstreamer-1.0 gstreamer-base-1.0 gstreamer-pbutils-1.0 libxml-2.0)

for f in "$DASH_SRC"/*.c; do
  base="$(basename "$f")"
  case "$base" in
    gstplugin.c|gstdashdemux.c) continue ;;
  esac
  clang -c -fno-sanitize=function -g -O0 -fvisibility=default \
      $CFLAGS \
      -I "$DASH_SRC" \
      -I "$SRC_DIR/subprojects/gst-plugins-bad/gst-libs" \
      "$f" -o "$OBJDIR/${base%.c}.o"
done

REPRO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
clang -g -O0 \
    -I "$DASH_SRC" \
    -I "$SRC_DIR/subprojects/gst-plugins-bad/gst-libs" \
    $CFLAGS \
    "$REPRO_DIR/repro.c" "$OBJDIR"/*.o \
    $LIBS \
    -L "$BUILD/subprojects/gst-plugins-bad/gst-libs/gst/uridownloader" \
    -lgsturidownloader-1.0 \
    -Wl,-rpath,"$BUILD/subprojects/gst-plugins-bad/gst-libs/gst/uridownloader" \
    -o /tmp/dashsink_repro_bin

ulimit -c 0
set +e
/tmp/dashsink_repro_bin
RC=$?
set -e
echo "exit code: $RC"
