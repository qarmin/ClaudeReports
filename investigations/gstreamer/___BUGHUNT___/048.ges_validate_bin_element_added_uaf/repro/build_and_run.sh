#!/usr/bin/env bash
# Builds delay_shim.so + harness_test against a given meson build directory (which must already
# be configured/built - see justfile), then runs the race N times and reports how many crashed
# with a heap-use-after-free vs completed cleanly.
#
# Usage: build_and_run.sh <path-to-meson-builddir> <path-to-project-root> [n-runs]
set -eo pipefail

BUILDDIR="$(cd "$1" && pwd)"
PROJECT_ROOT="$(cd "$2" && pwd)"
N="${3:-5}"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

TOOLSDIR="$PROJECT_ROOT/subprojects/gst-editing-services/tools"
GESVOBJ="$BUILDDIR/subprojects/gst-editing-services/tools/ges-launch-1.0.p/ges-validate.c.o"
UTILSOBJ="$BUILDDIR/subprojects/gst-editing-services/tools/ges-launch-1.0.p/utils.c.o"

if [ ! -f "$GESVOBJ" ]; then
  echo "error: $GESVOBJ not built - run 'ninja -C $BUILDDIR subprojects/gst-editing-services/tools/ges-launch-1.0' first" >&2
  exit 1
fi

python3 "$PROJECT_ROOT/gst-env.py" --builddir="$BUILDDIR" --only-environment > "$BUILDDIR/env.sh"
# shellcheck disable=SC1090
source "$BUILDDIR/env.sh"
export PKG_CONFIG_PATH="$BUILDDIR/meson-uninstalled:${PKG_CONFIG_PATH:-}"

cd "$HERE"
clang -g -O0 -shared -fPIC delay_shim.c -o libdelay_shim.so \
  $(pkg-config --cflags glib-2.0 gobject-2.0) $(pkg-config --libs glib-2.0 gobject-2.0) -ldl -lpthread

clang -g -O0 -fsanitize=address,undefined -fno-sanitize=function \
  -I"$TOOLSDIR" \
  harness.c link_stubs.c "$GESVOBJ" "$UTILSOBJ" \
  -L. -ldelay_shim \
  $(pkg-config --cflags gst-editing-services-1.0 gstreamer-validate-1.0) \
  $(pkg-config --libs gst-editing-services-1.0 gstreamer-validate-1.0) \
  -lgio-2.0 -lpthread \
  -Wl,-rpath,'$ORIGIN' \
  -o harness_test

ulimit -c 0
crashes=0
for i in $(seq 1 "$N"); do
  echo "--- run $i/$N ---"
  set +e
  ASAN_OPTIONS=detect_leaks=0 UBSAN_OPTIONS=print_stacktrace=0:halt_on_error=0 \
    LD_PRELOAD="$HERE/libdelay_shim.so" timeout 10 ./harness_test
  rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    crashes=$((crashes + 1))
  fi
done
echo "=== $crashes/$N runs crashed (heap-use-after-free expected before the fix, 0 after) ==="
