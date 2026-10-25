#!/usr/bin/env bash
# Compiles and runs repro.c against a meson build directory produced by this candidate's own
# justfile (BUILD_DIR env var, an uninstalled core-only build under clone/build-029). Real public
# GstDevice API, no hand-faked internal state.
set -euo pipefail

: "${BUILD_DIR:?set BUILD_DIR to the meson build directory (e.g. clone/build-029)}"

export PATH="$HOME/.local/bin:$PATH"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

REPO_ROOT="$(cd "$BUILD_DIR/.." && pwd)"
python3 "$REPO_ROOT/gst-env.py" --builddir="$BUILD_DIR" --only-environment > "$BUILD_DIR/env.sh"
# env.sh references some vars that may be unset on first source - disable nounset just for it.
set +u
# shellcheck disable=SC1090
source "$BUILD_DIR/env.sh"
set -u

mkdir -p "$SCRIPT_DIR/bin"
clang -g -O0 -fsanitize=address,undefined \
    -o "$SCRIPT_DIR/bin/repro" "$SCRIPT_DIR/repro.c" \
    $(pkg-config --cflags --libs gstreamer-1.0)

ulimit -c 0
ASAN_OPTIONS=detect_leaks=0 "$SCRIPT_DIR/bin/repro"
