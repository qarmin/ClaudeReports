#!/usr/bin/env bash
# Reproduction for: zune-jpeg decoder.rs `set_options_refreshes_cached_dispatch` test fails
# deterministically in --release builds (assert_eq! on fn-pointer addresses).
#
# The real affected entry point is the crate's own test suite - the exact command CI runs,
# `cargo test --release -p zune-jpeg --lib`, restricted here to the one failing test for speed.
#
# Usage: ./repro.sh <path-to-zune-image-checkout>  (defaults to ../clone, set up by `just clone`)
#
# Exit code convention: 0 = did not reproduce (test passed), non-zero = reproduced (test failed
# with the known assert_eq! panic on decoder.rs's idct_1x1_func comparison).
set -uo pipefail

checkout="${1:-$(dirname "$0")/../clone}"

if [ ! -d "$checkout" ]; then
    echo "ERROR: checkout not found at $checkout (run 'just clone' first, or pass a path)" >&2
    exit 2
fi

out="$(cd "$checkout" && cargo test --release -p zune-jpeg --lib decoder::planar_layout_helpers::set_options_refreshes_cached_dispatch 2>&1)"
status=$?

echo "$out"
echo "---"

if echo "$out" | grep -q "assertion \`left == right\` failed" && echo "$out" | grep -q "decoder.rs:2783"; then
    echo "REPRODUCED - set_options_refreshes_cached_dispatch panicked comparing idct_1x1_func addresses at decoder.rs:2783"
    exit 1
fi

if [ "$status" -eq 0 ] && echo "$out" | grep -q "test result: ok. 1 passed"; then
    echo "did not reproduce - set_options_refreshes_cached_dispatch passed"
    exit 0
fi

echo "cargo test exited unexpectedly - inspect manually (status=$status)" >&2
exit 3
