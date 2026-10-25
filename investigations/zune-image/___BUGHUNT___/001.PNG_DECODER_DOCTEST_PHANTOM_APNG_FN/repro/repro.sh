#!/usr/bin/env bash
# Reproduction for: zune-png decoder.rs doctest examples fail to compile (E0432/E0425).
#
# This bug is a compile-time doctest failure, not a runtime crash, so the "real affected entry
# point" is rustdoc's own doctest runner - the exact command `cargo test --doc -p zune-png` that
# CI/`cargo publish`/docs.rs effectively exercise - rather than a standalone Rust binary calling
# the library (a dependent crate can't observe a doctest failure in zune-png's own doc comments).
#
# Usage: ./repro.sh <path-to-zune-image-checkout>  (defaults to ../clone, set up by `just clone`)
#
# Exit code convention: 0 = did not reproduce (both doctests now pass), non-zero = reproduced
# (at least one of the two known-bad doctests still fails to compile).
set -uo pipefail

checkout="${1:-$(dirname "$0")/../clone}"

if [ ! -d "$checkout" ]; then
    echo "ERROR: checkout not found at $checkout (run 'just clone' first, or pass a path)" >&2
    exit 2
fi

out="$(cd "$checkout" && cargo test --doc -p zune-png 2>&1)"
status=$?

echo "$out"
echo "---"

failed_decoder_line68=false
failed_more_frames=false

if echo "$out" | grep -qE "decoder \(line (68|25)\) - compile \.\.\. FAILED"; then
    failed_decoder_line68=true
fi
if echo "$out" | grep -qE "more_frames \(line (507|494)\) - compile \.\.\. FAILED"; then
    failed_more_frames=true
fi

if $failed_decoder_line68 || $failed_more_frames; then
    echo "REPRODUCED - doctest(s) failed to compile:"
    $failed_decoder_line68 && echo "  - module-level 'Example: Decoding an APNG' doctest (E0432 unresolved import post_process_image_apng)"
    $failed_more_frames && echo "  - PngDecoder::more_frames doctest (E0425 cannot find value 'decoder')"
    exit 1
else
    if [ "$status" -eq 0 ]; then
        echo "did not reproduce - all zune-png doctests passed (cargo test --doc exit 0)"
        exit 0
    else
        echo "cargo test --doc exited non-zero but neither known-bad doctest name matched output - inspect manually" >&2
        exit 3
    fi
fi
