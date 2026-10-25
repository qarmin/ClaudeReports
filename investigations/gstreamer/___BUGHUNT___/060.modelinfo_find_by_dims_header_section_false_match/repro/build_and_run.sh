#!/usr/bin/env bash
# Builds repro.c against a scoped meson build (core+base+bad, for libgstanalytics-1.0) and runs
# it - gst_analytics_modelinfo_find_tensor_name() and modelinfo.h/gsttensor.h are real public
# API, no special visibility workaround needed here (unlike candidates #058/#059's plugin-internal
# static functions).
#
# Usage: SRC_DIR=<gstreamer checkout> bash build_and_run.sh
set -eo pipefail

SRC_DIR="${SRC_DIR:?set SRC_DIR to the gstreamer checkout}"
export PATH="$HOME/.local/bin:$PATH"

cd "$SRC_DIR"
if [ ! -d build-analytics-repro ]; then
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
      build-analytics-repro
fi
ninja -C build-analytics-repro

python3 gst-env.py --builddir=build-analytics-repro --only-environment > build-analytics-repro/env.sh
# shellcheck disable=SC1091
source build-analytics-repro/env.sh
export PKG_CONFIG_PATH="$SRC_DIR/build-analytics-repro/meson-uninstalled"

REPRO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
gcc -g -O0 -o /tmp/modelinfo_repro_bin "$REPRO_DIR/repro.c" \
  $(pkg-config --cflags --libs gstreamer-1.0 gstreamer-analytics-1.0)

/tmp/modelinfo_repro_bin
