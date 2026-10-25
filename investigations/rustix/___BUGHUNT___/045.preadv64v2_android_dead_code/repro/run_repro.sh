#!/usr/bin/env bash
# Standalone repro for candidate 045: preadv64v2/pwritev64v2 dead-code warning
# on Android cross-compilation.
#
# This is a static-analysis finding (a build-time lint, not a runtime crash),
# so the "real affected entry point" is `cargo clippy` run on rustix's own
# source tree cross-compiled for Android - there is no separate downstream
# crate to call a public API from, since the bug is entirely about whether
# `preadv64v2`/`pwritev64v2` (internal, non-pub(crate) items) are reachable
# from any caller inside the crate itself under this target+feature
# combination.
#
# Usage: run_repro.sh <path-to-rustix-checkout> <expect: present|absent>
#   present - assert the dead-code warnings for preadv64v2/pwritev64v2 ARE
#             present (demonstrates the bug, run against pristine source)
#   absent  - assert the warnings are ABSENT (demonstrates the fix, run
#             against the patched source)
set -euo pipefail

CLONE_DIR="${1:?usage: run_repro.sh <path-to-rustix-checkout> <present|absent>}"
EXPECT="${2:?usage: run_repro.sh <path-to-rustix-checkout> <present|absent>}"

if [[ "$EXPECT" != "present" && "$EXPECT" != "absent" ]]; then
    echo "second argument must be 'present' or 'absent', got: $EXPECT" >&2
    exit 2
fi

rustup target add aarch64-linux-android >/dev/null 2>&1 || true

cd "$CLONE_DIR"
echo "+ cargo clippy --target aarch64-linux-android --features std,all-apis,try_close"
cargo clean -p rustix >/dev/null 2>&1 || true
OUTPUT=$(cargo clippy --target aarch64-linux-android --features std,all-apis,try_close 2>&1 || true)
echo "$OUTPUT"
echo "---"

HAS_PREADV=0
HAS_PWRITEV=0
echo "$OUTPUT" | grep -q "function \`preadv64v2\` is never used" && HAS_PREADV=1
echo "$OUTPUT" | grep -q "function \`pwritev64v2\` is never used" && HAS_PWRITEV=1

if [[ "$HAS_PREADV" -eq 1 && "$HAS_PWRITEV" -eq 1 ]]; then
    ACTUAL="present"
else
    ACTUAL="absent"
fi

echo "dead-code warnings for preadv64v2/pwritev64v2: $ACTUAL (expected: $EXPECT)"

if [[ "$ACTUAL" == "$EXPECT" ]]; then
    echo "REPRO: matches expectation (OK)"
    exit 0
else
    echo "REPRO: does NOT match expectation (FAIL)"
    exit 1
fi
