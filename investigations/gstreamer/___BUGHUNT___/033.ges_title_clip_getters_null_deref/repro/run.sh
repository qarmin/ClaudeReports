#!/usr/bin/env bash
# Compiles and runs repro.c against a meson build directory produced by this candidate's own
# justfile (BUILD_DIR env var, an uninstalled build under clone/build-ges). Real public GES API,
# no hand-faked state - see RESULT.md's "Fully reproducible" section.
set -euo pipefail

: "${BUILD_DIR:?set BUILD_DIR to the meson build directory (e.g. clone/build-ges)}"

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
clang -g -O0 -fsanitize=address,undefined \
    -o "$SCRIPT_DIR/bin/repro" "$SCRIPT_DIR/repro.c" \
    $(pkg-config --cflags --libs gst-editing-services-1.0)

ulimit -c 0
rm -f "$BUILD_DIR/repro_registry.bin"
# Leak detection is unrelated to this candidate's finding (fontconfig/pango allocate at init and
# are never expected to be freed by this short-lived process) - focus purely on the crash/logic
# under test.
ASAN_OPTIONS=detect_leaks=0 GST_REGISTRY="$BUILD_DIR/repro_registry.bin" "$SCRIPT_DIR/bin/repro"
