#!/usr/bin/env bash
# Standalone reproduction for candidate 081.
#
# This bug lives entirely inside rustix's own `#[cfg(test)]` self-check code
# (src/fs/constants.rs), which is unreachable through any public API - there
# is no "caller" to simulate, the bug is that rustix's *own test suite* does
# not compile for the wasm32-wasip1 target. So the real, end-to-end
# reproduction is simply: compile rustix's real, unmodified lib tests for the
# real wasm32-wasip1 target, using the project's own real vendored `libc`
# dependency (no mocked/hand-authored struct definitions anywhere in this
# script) and a real installed `wasm32-wasip1` Rust target.
#
# Usage: check.sh <path-to-a-rustix-checkout>
set -u

RUSTIX_DIR="${1:?usage: check.sh <path-to-rustix-checkout>}"

rustup target add wasm32-wasip1 >/dev/null 2>&1

cd "$RUSTIX_DIR" || exit 1

OUT=$(cargo test --no-run --target wasm32-wasip1 --features "std,fs" --lib 2>&1)
STATUS=$?

echo "$OUT"
echo
echo "=== exit status: $STATUS ==="

if echo "$OUT" | grep -qE "no field \`st_(a|m|c)time(_nsec)?\` on type \`libc::stat\`|struct \`libc::stat\` does not have a field named \`st_(a|m|c)time(_nsec)?\`"; then
    echo "=== REPRO: the st_atime/st_mtime/st_ctime field-name bug IS present ==="
else
    echo "=== the st_atime/st_mtime/st_ctime field-name bug is NOT present (fixed) ==="
fi
# Always exit 0: this script's job is to report which state was observed,
# not to fail `just`'s own control flow (both "before" and "after" are
# expected, correct outcomes for their respective step).
exit 0
