#!/usr/bin/env bash
# Builds repro.c against a core-only gstreamer checkout at $SRC_DIR (meson build in
# $SRC_DIR/build-core-repro) and runs it under valgrind memcheck.
#
# Usage: SRC_DIR=<gstreamer checkout> bash build_and_run.sh
set -eo pipefail

SRC_DIR="${SRC_DIR:?set SRC_DIR to the gstreamer checkout}"
export PATH="$HOME/.local/bin:$PATH"

cd "$SRC_DIR"
if [ ! -d build-core-repro ]; then
  meson setup \
      --buildtype=debug \
      -Dglib:tests=false \
      -Ddoc=disabled \
      -Dexamples=disabled \
      -Dintrospection=disabled \
      -Dgood=disabled \
      -Dbad=disabled \
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
      -Dbase=disabled \
      build-core-repro
fi
ninja -C build-core-repro

python3 gst-env.py --builddir=build-core-repro --only-environment > build-core-repro/env.sh
# shellcheck disable=SC1091
source build-core-repro/env.sh
export PKG_CONFIG_PATH="$SRC_DIR/build-core-repro/meson-uninstalled"

REPRO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
gcc -g -O0 -o /tmp/gstbuffer_repro_bin "$REPRO_DIR/repro.c" $(pkg-config --cflags --libs gstreamer-1.0)

valgrind --error-exitcode=99 /tmp/gstbuffer_repro_bin
