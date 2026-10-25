#!/usr/bin/env bash
# Standalone reproduction for candidate 022: clippy::never_loop on
# examples/buffer_errors.rs::allocate_loop.
#
# This is a static-analysis / build-tooling finding, not a runtime API bug - there is no second
# participant to drive (nothing produces adversarial input at runtime; `cargo clippy` is a single
# deterministic analysis pass over fixed source text). The real end-to-end trigger path IS simply
# running the real `cargo clippy` command against the real example file in a real checkout of the
# project, which is exactly what this script does - no hand-authored stand-in involved.
#
# Usage: run.sh <path-to-rustix-checkout>
set -euo pipefail

REPO_DIR="${1:?usage: run.sh <path-to-rustix-checkout>}"
cd "$REPO_DIR"

set +e
OUTPUT=$(cargo clippy --example buffer_errors --features all-apis 2>&1)
STATUS=$?
set -e

echo "$OUTPUT"
echo "---"
echo "cargo clippy exit code: $STATUS"

if echo "$OUTPUT" | grep -q "this loop never actually loops"; then
    echo "REPRODUCED: clippy::correctness 'never_loop' (deny-by-default) fired on"
    echo "allocate_loop() in examples/buffer_errors.rs - cargo clippy failed to compile the example."
    exit 1
elif [ "$STATUS" -eq 0 ]; then
    echo "did not reproduce - cargo clippy succeeded cleanly (no never_loop diagnostic)"
    exit 0
else
    echo "cargo clippy failed for a DIFFERENT reason (not the never_loop lint) - investigate"
    exit 2
fi
