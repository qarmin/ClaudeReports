#!/usr/bin/env bash
# Builds and runs repro.c against gsthlsdemux-util.c directly (compiled together as one
# translation unit, not linked against the prebuilt plugin .so, since handle_pat()/handle_pmt()
# are `static` and the plugin build uses -fvisibility=hidden for everything else). This mirrors
# the project's existing gsth266parser.c fuzz-harness approach (see
# ___BUGHUNT___/projects/gstreamer.md and 019.fuzz_h266parser_new_harness/).
#
# Usage: SRC_DIR=<gstreamer checkout> bash build_and_run.sh
set -eo pipefail

SRC_DIR="${SRC_DIR:?set SRC_DIR to the gstreamer checkout}"
export PATH="$HOME/.local/bin:$PATH"

cd "$SRC_DIR"
if [ ! -d build-hls-repro ]; then
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
      build-hls-repro
fi
ninja -C build-hls-repro subprojects/gst-plugins-bad/ext/hls/libgsthls.so

python3 gst-env.py --builddir=build-hls-repro --only-environment > build-hls-repro/env.sh
# shellcheck disable=SC1091
source build-hls-repro/env.sh
export PKG_CONFIG_PATH="$SRC_DIR/build-hls-repro/meson-uninstalled"

REPRO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILDDIR="$SRC_DIR/build-hls-repro"

# Compile gsthlsdemux-util.c directly (default visibility, no -fvisibility=hidden) so
# gst_hlsdemux_tsreader_find_pcrs() (which calls the static handle_pat()/handle_pmt()
# internally) is externally linkable from our harness.
INCLUDES=(
  -I"$BUILDDIR/subprojects/gst-plugins-bad/ext/hls/libgsthls.so.p"
  -I"$SRC_DIR/subprojects/gst-plugins-bad/ext/hls" -I"$BUILDDIR/subprojects/gst-plugins-bad/ext/hls"
  -I"$SRC_DIR/subprojects/gst-plugins-bad" -I"$BUILDDIR/subprojects/gst-plugins-bad"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs" -I"$BUILDDIR/subprojects/gst-plugins-base/gst-libs"
  -I"$SRC_DIR/subprojects/gstreamer" -I"$BUILDDIR/subprojects/gstreamer"
  -I"$SRC_DIR/subprojects/gstreamer/libs" -I"$BUILDDIR/subprojects/gstreamer/libs"
  -I"$SRC_DIR/subprojects/orc" -I"$BUILDDIR/subprojects/orc"
  -I"$SRC_DIR/subprojects/gst-plugins-bad/gst-libs" -I"$BUILDDIR/subprojects/gst-plugins-bad/gst-libs"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs/gst/pbutils"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs/gst/video"
  -I"$BUILDDIR/subprojects/gstreamer/gst" -I"$SRC_DIR/subprojects/gstreamer/gst"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs/gst/audio"
  -I"$SRC_DIR/subprojects/gst-plugins-base/gst-libs/gst/tag"
)

clang -fsanitize=address,undefined -fno-sanitize=function -g -O0 \
  "${INCLUDES[@]}" \
  $(pkg-config --cflags glib-2.0) \
  -DHAVE_CONFIG_H -DGST_USE_UNSTABLE_API -DHAVE_NETTLE \
  -o /tmp/gsthlsdemux-util-repro.o -c "$SRC_DIR/subprojects/gst-plugins-bad/ext/hls/gsthlsdemux-util.c"

clang -fsanitize=address,undefined -fno-sanitize=function -g -O0 \
  -I"$SRC_DIR/subprojects/gst-plugins-bad/ext/hls" \
  "${INCLUDES[@]}" \
  $(pkg-config --cflags glib-2.0) \
  -DHAVE_CONFIG_H -DGST_USE_UNSTABLE_API -DHAVE_NETTLE \
  -o /tmp/hlsdemux-util-repro-main.o -c "$REPRO_DIR/repro.c"

clang -fsanitize=address,undefined -fno-sanitize=function -g -O0 \
  -o /tmp/hlsdemux_util_repro /tmp/hlsdemux-util-repro-main.o /tmp/gsthlsdemux-util-repro.o \
  $(pkg-config --cflags --libs gstreamer-1.0 gstreamer-tag-1.0) -lnettle

/tmp/hlsdemux_util_repro
