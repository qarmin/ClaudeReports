#!/usr/bin/env bash
# Builds repro.c against gstxmlhelper.c directly (default visibility, since the real plugin build
# uses -fvisibility=hidden and gst_xml_helper_get_prop_dateTime(), while non-static, isn't
# otherwise dynamically linkable from the prebuilt plugin .so) and runs it under valgrind
# (ASAN does not reliably catch this specific 1-byte-past-allocation read in this environment -
# see RESULT.md - valgrind does, precisely).
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
      build-dash-repro
fi
ninja -C build-dash-repro subprojects/gst-plugins-bad/ext/dash/libgsthdash.so 2>/dev/null || \
ninja -C build-dash-repro subprojects/gst-plugins-bad/ext/dash/libgstdash.so

python3 gst-env.py --builddir=build-dash-repro --only-environment > build-dash-repro/env.sh
# shellcheck disable=SC1091
source build-dash-repro/env.sh
export PKG_CONFIG_PATH="$SRC_DIR/build-dash-repro/meson-uninstalled"

REPRO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILDDIR="$SRC_DIR/build-dash-repro"

INCLUDES=(
  -I"$BUILDDIR/subprojects/gst-plugins-bad/ext/dash/libgstdash.so.p"
  -I"$SRC_DIR/subprojects/gst-plugins-bad/ext/dash"
  -I"$SRC_DIR/subprojects/gst-plugins-bad" -I"$BUILDDIR/subprojects/gst-plugins-bad"
  -I"$SRC_DIR/subprojects/gst-plugins-bad/gst-libs" -I"$BUILDDIR/subprojects/gst-plugins-bad/gst-libs"
  -I"$SRC_DIR/subprojects/gstreamer/libs" -I"$BUILDDIR/subprojects/gstreamer/libs"
  -I"$SRC_DIR/subprojects/gstreamer" -I"$BUILDDIR/subprojects/gstreamer"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs" -I"$BUILDDIR/subprojects/gst-plugins-base/gst-libs"
  -I"$SRC_DIR/subprojects/orc" -I"$BUILDDIR/subprojects/orc"
  -I"$BUILDDIR/subprojects/gstreamer/gst" -I"$SRC_DIR/subprojects/gstreamer/gst"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs/gst/tag"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs/gst/pbutils"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs/gst/video"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs/gst/audio"
)

cc "${INCLUDES[@]}" $(pkg-config --cflags glib-2.0 libxml-2.0) \
  -DHAVE_CONFIG_H -DGST_USE_UNSTABLE_API -g -O0 \
  -o /tmp/gstxmlhelper-repro.o -c "$SRC_DIR/subprojects/gst-plugins-bad/ext/dash/gstxmlhelper.c"

cc -I"$SRC_DIR/subprojects/gst-plugins-bad/ext/dash" "${INCLUDES[@]}" \
  $(pkg-config --cflags glib-2.0 libxml-2.0) \
  -DHAVE_CONFIG_H -DGST_USE_UNSTABLE_API -g -O0 \
  -o /tmp/xmlhelper-repro-main.o -c "$REPRO_DIR/repro.c"

cc -g -O0 -o /tmp/xmlhelper_repro /tmp/xmlhelper-repro-main.o /tmp/gstxmlhelper-repro.o \
  $(pkg-config --cflags --libs gstreamer-1.0 libxml-2.0)

valgrind --error-exitcode=99 /tmp/xmlhelper_repro
