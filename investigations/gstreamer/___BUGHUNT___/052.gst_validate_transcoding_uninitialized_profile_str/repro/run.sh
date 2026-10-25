#!/usr/bin/env bash
# Runs the real gst-validate-transcoding-1.0 binary (built at $BUILD_DIR) with no -o flag, under
# valgrind memcheck --track-origins=yes, and checks whether it reports the uninitialised-value
# read at gst-validate-transcoding.c:354 (the `if (profile_str == NULL)` check).
#
# Usage: BUILD_DIR=<path> SRC_DIR=<path> bash run.sh
set -eo pipefail

BUILD_DIR="${BUILD_DIR:?set BUILD_DIR to the meson build dir}"
SRC_DIR="${SRC_DIR:?set SRC_DIR to the gstreamer checkout}"
BIN="$BUILD_DIR/subprojects/gst-devtools/validate/tools/gst-validate-transcoding-1.0"

export PATH="$HOME/.local/bin:$PATH"
cd "$SRC_DIR"
python3 gst-env.py --builddir="$BUILD_DIR" --only-environment > "$BUILD_DIR/env.sh"
# shellcheck disable=SC1090
source "$BUILD_DIR/env.sh"

LOG="$(mktemp)"
trap 'rm -f "$LOG"' EXIT

# No -o/--output-format passed - profile_str is read uninitialized if the fix isn't applied.
set +e
valgrind --track-origins=yes --log-file="$LOG" \
  "$BIN" file:///tmp/gsttranscode_repro_in.mp4 file:///tmp/gsttranscode_repro_out.mp4 \
  > /dev/null 2>&1
set -e

cat "$LOG"

if grep -q 'gst-validate-transcoding.c:354' "$LOG"; then
  echo "REPRO: valgrind confirms uninitialised-value read of profile_str at line 354"
  exit 42
else
  echo "NO-REPRO: no uninitialised-value warning at gst-validate-transcoding.c:354"
  exit 0
fi
