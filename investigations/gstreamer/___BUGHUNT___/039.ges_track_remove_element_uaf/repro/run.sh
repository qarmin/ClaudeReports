#!/usr/bin/env bash
# Compiles and runs repro.c against a meson build directory produced by this candidate's own
# justfile (BUILD_DIR env var, an uninstalled build under clone/build-asan-ges). Real public GES
# API (ges_track_add_element/ges_track_remove_element called twice on the same still-alive
# element), no hand-faked state - see RESULT.md's "Fully reproducible" section.
set -euo pipefail

: "${BUILD_DIR:?set BUILD_DIR to the meson build directory (e.g. clone/build-asan-ges)}"

export PATH="$HOME/.local/bin:$PATH"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

REPO_ROOT="$(cd "$BUILD_DIR/.." && pwd)"
python3 "$REPO_ROOT/gst-env.py" --builddir="$BUILD_DIR" --only-environment > "$BUILD_DIR/env.sh"
# env.sh references some vars (e.g. GST_PLUGIN_PATH) that may be unset on first source - disable
# nounset just for sourcing it.
set +u
# shellcheck disable=SC1090
source "$BUILD_DIR/env.sh"
set -u

mkdir -p "$SCRIPT_DIR/bin"
clang -g -O0 -fsanitize=address -fno-sanitize=function \
    -o "$SCRIPT_DIR/bin/repro" "$SCRIPT_DIR/repro.c" \
    $(pkg-config --cflags --libs gst-editing-services-1.0 gstreamer-1.0)

ulimit -c 0
rm -f "$BUILD_DIR/repro_registry.bin"
# Leak detection is unrelated to this candidate's finding - focus purely on the UAF under test.
# halt_on_error=1 so a real ASAN report reliably aborts (rather than printing and continuing into
# a second, confusing crash once the corrupted GSequence/hash-table state is touched again).
ASAN_OPTIONS=detect_leaks=0:halt_on_error=1 \
    GST_REGISTRY="$BUILD_DIR/repro_registry.bin" \
    "$SCRIPT_DIR/bin/repro"
