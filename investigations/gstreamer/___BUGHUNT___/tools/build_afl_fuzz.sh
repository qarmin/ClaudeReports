#!/usr/bin/env bash
# Configure + build a local AFL++-instrumented GStreamer tree (coverage-guided fuzzing), separate
# from tools/build_asan_fuzz.sh's plain ASAN+UBSAN build.
#
# WHY TWO SEPARATE BUILDS instead of one AFL+ASAN combo: AFL_USE_ASAN=1 with afl-clang-fast was
# tried first (2026-09-15) and produced a binary with ASAN symbols present but NOT actually
# catching a deliberately-planted stack-buffer-overflow (exit 0 where plain `clang
# -fsanitize=address` on the identical source aborted) - some interaction between afl-clang-fast's
# instrumentation and ASAN's init wasn't triggering detection, not investigated further given the
# session's time budget. Standard AFL practice anyway: fuzz fast without sanitizers for max
# execs/sec, then replay any crashing input found here against the ASAN build
# (tools/build_asan_fuzz.sh's build-asan-fuzz/) to get real sanitizer diagnostics for root-causing.
#
# Same plugin scope as build_asan_fuzz.sh - see that script's own comment for why.
#
# Usage: tools/build_afl_fuzz.sh [build-dir-name]   (run from the project root)
set -euo pipefail

# See build_asan_fuzz.sh's identical comment: meson/ninja live in ~/.local/bin (uv tool install),
# not on PATH for a plain background `nohup bash script.sh &` job.
export PATH="$HOME/.local/bin:$PATH"

BUILDDIR="${1:-build-afl-fuzz}"
cd "$(git rev-parse --show-toplevel)"

CC=afl-clang-fast CXX=afl-clang-fast++ meson setup \
    --buildtype=debugoptimized \
    -Db_lundef=false \
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

# `-k 0`: FLAC's own CLI tools (metaflac/flac, unused by anything here) fail to link under
# afl-clang-fast with "undefined reference to alloc_check_counter/alloc_check_keep_failing"
# (confirmed 2026-09-15, not investigated - not our fuzz targets). `-k 0` keeps building
# everything else regardless, which is all that's actually needed (all 11 ci/fuzzing/ targets
# link fine).
ninja -C "$BUILDDIR" -k 0 || true

echo
echo "Built (FLAC CLI tool link failures above are expected/benign - see this script's comment)."
echo "Fuzz target binaries (AFL-instrumented):"
find "$BUILDDIR/ci/fuzzing" -maxdepth 1 -type f -executable -print
echo
echo "Run afl-fuzz uninstalled, e.g.:"
echo "  python3 gst-env.py --builddir=$BUILDDIR --only-environment > $BUILDDIR/env.sh"
echo "  source $BUILDDIR/env.sh"
echo "  mkdir -p /tmp/afl_out"
echo "  afl-fuzz -i ci/fuzzing/seeds/gst-value -o /tmp/afl_out -- $BUILDDIR/ci/fuzzing/gst-value @@"
echo
echo "Any crash found lands in /tmp/afl_out/default/crashes/ - replay it against the ASAN build"
echo "(build-asan-fuzz/ci/fuzzing/<same target>) to get sanitizer diagnostics."
