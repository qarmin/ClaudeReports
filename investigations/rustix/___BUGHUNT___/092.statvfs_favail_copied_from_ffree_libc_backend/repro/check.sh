#!/usr/bin/env bash
# Builds and runs the statvfs f_favail repro against a rustix checkout.
#
# Usage: check.sh <path-to-rustix-checkout> <before|after>
#
# "before" expects the pristine bug (rustix substitutes f_ffree for f_favail); "after" expects
# the fix (rustix correctly reports the real f_favail). Exits non-zero if the expected marker
# isn't found in the shimmed run's output, so `just test` fails loudly instead of silently if the
# build environment can't actually exercise this (e.g. no gcc, no glibc, or the fix wasn't really
# applied - see CONFIRMATION.md section 9's stale-binary trap).
set -euo pipefail

CHECKOUT="$1"
MODE="$2"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cp "$HERE/statvfs_check.rs" "$CHECKOUT/examples/statvfs_check.rs"

echo "--- building LD_PRELOAD shim ---"
gcc -shared -fPIC -o "$HERE/libstatvfsshim.so" "$HERE/shim.c" -ldl

echo "--- building rustix example (libc backend forced via --cfg=rustix_use_libc) ---"
(
  cd "$CHECKOUT"
  RUSTFLAGS="--cfg=rustix_use_libc" cargo build --example statvfs_check --features "std,fs"
)

BIN="$CHECKOUT/target/debug/examples/statvfs_check"

echo "--- sanity run, no shim (normal real-world case on this sandbox's filesystem) ---"
"$BIN"

echo "--- 3x runs WITH the shim (simulates a real OS/filesystem with f_ffree != f_favail) ---"
LAST_OUT=""
for i in 1 2 3; do
  echo "[run $i]"
  LAST_OUT="$(LD_PRELOAD="$HERE/libstatvfsshim.so" "$BIN")"
  echo "$LAST_OUT"
  echo
  BUG_COUNT="$(echo "$LAST_OUT" | grep -c "BUG REPRODUCED" || true)"
  CORRECT_COUNT="$(echo "$LAST_OUT" | grep -c "CORRECT" || true)"
  if [ "$MODE" = "before" ]; then
    [ "$BUG_COUNT" = "2" ] || {
      echo "FAIL: expected BUG REPRODUCED for both statvfs and fstatvfs on run $i, got $BUG_COUNT" >&2
      exit 1
    }
  else
    [ "$BUG_COUNT" = "0" ] || {
      echo "FAIL: expected the bug to be fixed on run $i, but it still reproduced ($BUG_COUNT times)" >&2
      exit 1
    }
    [ "$CORRECT_COUNT" = "2" ] || {
      echo "FAIL: expected CORRECT for both statvfs and fstatvfs on run $i, got $CORRECT_COUNT" >&2
      exit 1
    }
  fi
done

echo "RESULT ($MODE): all 3 runs matched the expected ($([ "$MODE" = before ] && echo 'buggy' || echo 'fixed')) behavior for both statvfs and fstatvfs"
