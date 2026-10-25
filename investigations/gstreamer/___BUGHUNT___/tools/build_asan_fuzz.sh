#!/usr/bin/env bash
# Configure + build a local ASAN+UBSAN instrumented GStreamer tree with the ci/fuzzing/ fuzz
# targets enabled, WITHOUT needing oss-fuzz's own build harness (no $SRC/$OUT/$WORK, no
# patchelf/install step, no static linking) - runs uninstalled via gst-env.py instead.
#
# Scope mirrors ci/fuzzing/build-oss-fuzz.sh's own plugin selection (a known-good, known-buildable
# subset): core + base + the good/bad demuxers/parsers most likely to see untrusted input
# (isomp4, matroska, flv, avi, audioparsers/wavparse/id3demux, mpegtsdemux, videoparsers,
# jpegformat). Extend the -D... list below if a candidate needs a plugin outside this set.
#
# Usage: tools/build_asan_fuzz.sh [build-dir-name]   (run from the project root, not from inside
# ___BUGHUNT___/)
#
# -fno-sanitize=function: UBSan's "function" check (call through a function pointer whose type
# doesn't match the pointee's actual signature) fires constantly on GObject-style C - vtables and
# GClosure-ish callbacks are routinely stored/called through a generic function pointer type and
# cast at the call site, which is idiomatic here, not a bug. Confirmed 2026-09-15: a single
# `typefind`/`gst-value` run against one seed produced a dozen+ of these from plugin
# registration/typefind-factory/iterator/bus-handler internals alone, before the fuzzed input is
# even touched - drowning any real finding and, worse, defeating UBSAN_OPTIONS=halt_on_error=1
# (see tools/mini_fuzz.py's own comment) by aborting on this boilerplate on every single run.
set -euo pipefail

# meson/ninja were installed via `uv tool install` into ~/.local/bin (no root, no system package
# available) - present on PATH in an interactive shell via ~/.bashrc, but NOT in a plain
# `nohup bash script.sh &` background job, which skips ~/.bashrc entirely. Make this script work
# either way.
export PATH="$HOME/.local/bin:$PATH"

BUILDDIR="${1:-build-asan-fuzz}"
cd "$(git rev-parse --show-toplevel)"

# clang: libclang-rt-21-dev was installed into the container (2026-09-15) specifically to make
# this work - before that, clang had no libclang_rt.asan*.a and this build used gcc's ASAN/UBSAN
# instead, which hit a gcc-15/abseil-cpp constexpr incompatibility under -fsanitize=address,undefined
# (absl::flags::reflection.cc, "is not a constant expression") that clang doesn't have.
CC=clang CXX=clang++ meson setup \
    --buildtype=debugoptimized \
    -Db_sanitize=address,undefined \
    -Db_lundef=false \
    -Dc_args=-fno-sanitize=function \
    -Dcpp_args=-fno-sanitize=function \
    -Doss_fuzz=enabled \
    -Dglib:tests=false \
    -Ddoc=disabled \
    -Dexamples=disabled \
    -Dintrospection=disabled \
    -Dgood=enabled \
    -Dgst-plugins-good:auto_features=disabled \
    -Dgst-plugins-good:isomp4=enabled \
    -Dgst-plugins-good:matroska=enabled \
    -Dgst-plugins-good:flv=enabled \
    -Dgst-plugins-good:avi=enabled \
    -Dgst-plugins-good:audioparsers=enabled \
    -Dgst-plugins-good:wavparse=enabled \
    -Dgst-plugins-good:id3demux=enabled \
    -Dbad=enabled \
    -Dgst-plugins-bad:auto_features=disabled \
    -Dgst-plugins-bad:mpegtsdemux=enabled \
    -Dgst-plugins-bad:videoparsers=enabled \
    -Dgst-plugins-bad:jpegformat=enabled \
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
    -Dgst-plugins-base:pango=disabled \
    "$BUILDDIR"

ninja -C "$BUILDDIR"

echo
echo "Built. Fuzz target binaries:"
find "$BUILDDIR/ci/fuzzing" -maxdepth 1 -type f -executable -print
echo
echo "Run one uninstalled, e.g.:"
echo "  python3 gst-env.py --builddir=$BUILDDIR --only-environment > $BUILDDIR/env.sh"
echo "  source $BUILDDIR/env.sh && $BUILDDIR/ci/fuzzing/typefind ci/fuzzing/seeds/gst-value/caps-simple"
