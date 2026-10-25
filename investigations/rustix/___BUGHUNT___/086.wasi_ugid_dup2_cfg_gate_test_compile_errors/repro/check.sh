#!/usr/bin/env bash
# Standalone reproduction for candidate 086.
#
# This bug lives entirely inside rustix's own `#[cfg(test)]` code (one line
# in src/fs/constants.rs, one test function in src/backend/libc/fs/dir.rs),
# unreachable through any public API - there is no "caller" to simulate, the
# bug is that rustix's *own test suite* does not compile for the
# wasm32-wasip1 target. So the real, end-to-end reproduction is simply:
# compile rustix's real, unmodified lib tests for the real wasm32-wasip1
# target, using the project's own real vendored `libc` dependency (nothing
# mocked/hand-authored anywhere in this script) and a real installed
# `wasm32-wasip1` Rust target.
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

UGID_HIT=$(echo "$OUT" | grep -c "cannot find \`ugid\` in")
DUP2_HIT=$(echo "$OUT" | grep -c "cannot find function \`dup2\`")

echo "=== ugid error occurrences: $UGID_HIT ==="
echo "=== dup2 error occurrences: $DUP2_HIT ==="

if [ "$UGID_HIT" -gt 0 ] || [ "$DUP2_HIT" -gt 0 ]; then
    echo "=== REPRO: the ugid/dup2 cfg-gate bug IS present ==="
else
    echo "=== the ugid/dup2 cfg-gate bug is NOT present (fixed) ==="
fi

# Note: this candidate's own fix does not touch the separate, already-known
# st_atime/st_mtime/st_ctime field-name errors (candidate 081's bug) - those
# are expected to still be present in both the before and after run here,
# since this repro clones the pristine, unpatched commit and this
# candidate's patch.diff only fixes the ugid/dup2 lines.

# Always exit 0: this script's job is to report which state was observed,
# not to fail `just`'s own control flow (both "before" and "after" are
# expected, correct outcomes for their respective step).
exit 0
