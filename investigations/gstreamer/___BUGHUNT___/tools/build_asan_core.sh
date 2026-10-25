#!/usr/bin/env bash
# Configure + build a local ASAN+UBSAN instrumented tree with EVERYTHING except the gstreamer
# core library disabled (good/bad/ugly/base/libav/ges/rs/python/etc all off, no plugins beyond
# coreelements). Much cheaper/faster than tools/build_asan_fuzz.sh's full plugin-scoped build -
# use this instead whenever a candidate's suspect code lives entirely in
# subprojects/gstreamer/gst/*.c (core GstMeta/GstBuffer/GstCaps/... APIs) and needs no demuxer/
# parser/plugin to reach, i.e. a "self-contained core-lib API call" bug.
#
# Usage: tools/build_asan_core.sh [build-dir-name]   (run from the project root, not from inside
# ___BUGHUNT___/)
set -euo pipefail

# See build_asan_fuzz.sh's identical comment: uv-installed meson/ninja live in ~/.local/bin, which
# a non-interactive shell (nohup bash script.sh &) doesn't source ~/.bashrc for.
export PATH="$HOME/.local/bin:$PATH"

BUILDDIR="${1:-build-asan-core}"
cd "$(git rev-parse --show-toplevel)"

# clang, -fno-sanitize=function: see build_asan_fuzz.sh's own comments for why (libclang-rt-21-dev
# needed for clang's ASAN/UBSAN; UBSan's function-pointer-type check false-positives constantly on
# GObject-style vtables/callbacks).
CC=clang CXX=clang++ meson setup \
    --buildtype=debugoptimized \
    -Db_sanitize=address,undefined \
    -Db_lundef=false \
    -Dc_args=-fno-sanitize=function \
    -Dcpp_args=-fno-sanitize=function \
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
    -Dorc=disabled \
    -Dgstreamer:tracer_hooks=false \
    -Dbase=disabled \
    "$BUILDDIR"

ninja -C "$BUILDDIR"

echo
echo "Built core-only ASAN+UBSAN gstreamer-1.0. To run something against it uninstalled:"
echo "  python3 gst-env.py --builddir=$BUILDDIR --only-environment > $BUILDDIR/env.sh"
echo "  source $BUILDDIR/env.sh"
echo "  export PKG_CONFIG_PATH=\"\$PWD/$BUILDDIR/meson-uninstalled:\$PKG_CONFIG_PATH\""
echo "  clang -g -fsanitize=address,undefined -fno-sanitize=function -o repro repro.c \\"
echo "    \$(pkg-config --cflags --libs gstreamer-1.0)"
