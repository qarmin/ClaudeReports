#!/usr/bin/env bash
# Standalone reproduction for candidate 060: clippy::wrong_self_convention on
# Timespec::to_sec_usec / Timespec::to_timeval in src/timespec.rs.
#
# This is a static-analysis / API-convention finding, not a runtime bug - `to_sec_usec`/
# `to_timeval` are `pub(crate)` helpers, so there is no second participant to drive (nothing
# external ever calls them; clippy's lint is a single deterministic analysis pass over fixed
# source text). The real end-to-end trigger path IS simply running the real `cargo clippy`
# command against the real crate source in a real checkout, which is exactly what this script
# does - no hand-authored stand-in involved. See CONFIRMATION.md section 11's "single external
# artifact" exception: the real source file IS the genuine input, there's no caller to impersonate.
#
# Usage: run.sh <path-to-rustix-checkout>
set -euo pipefail

REPO_DIR="${1:?usage: run.sh <path-to-rustix-checkout>}"
cd "$REPO_DIR"

# Note: `to_sec_usec`/`to_timeval` are only reached from the `net`/`event` modules, which are not
# in the crate's bare `default = ["std"]` feature set - so `all-apis` must be enabled for clippy to
# even see these functions as live code. No `-W` opt-in flag is needed: `wrong_self_convention` is
# warn-by-default in clippy itself once the code is actually compiled in.
echo "=== 1) 'cargo clippy --features std,all-apis' (linux_raw backend) - no -W flag needed ==="
set +e
OUT_LINUXRAW=$(cargo clippy --no-default-features --features std,all-apis 2>&1)
set -e
echo "$OUT_LINUXRAW" | grep -A6 "timespec.rs:210" || echo "(no hit on line 210 under linux_raw backend)"

echo
echo "=== 2) 'cargo clippy --features std,all-apis,use-libc' (libc backend, both to_* methods compiled) ==="
set +e
OUT_LIBC=$(cargo clippy --no-default-features --features std,all-apis,use-libc 2>&1)
set -e
echo "$OUT_LIBC" | grep -A6 "timespec.rs:210" || echo "(no hit on line 210 under libc backend)"
echo "$OUT_LIBC" | grep -A6 "timespec.rs:223" || echo "(no hit on line 223 under libc backend)"

echo
echo "--- summary ---"
if echo "$OUT_LINUXRAW" | grep -q "timespec.rs:210" && echo "$OUT_LIBC" | grep -q "timespec.rs:223"; then
    echo "REPRODUCED: clippy::wrong_self_convention (warn-by-default, no opt-in flag needed) fires on"
    echo "Timespec::to_sec_usec (src/timespec.rs:210) under the linux_raw backend, and, once the"
    echo "libc-backend-only to_timeval is compiled in too, also on src/timespec.rs:223."
    exit 1
else
    echo "did not reproduce - cargo clippy did not flag the expected lines"
    exit 0
fi
