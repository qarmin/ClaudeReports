#!/usr/bin/env bash
# Build and run one repro under a matrix of build configs, report which show the bug.
#
#   sanitizer_matrix.sh <path to repro/ crate dir>
#
# A finding that only shows up under one specific config is still real (CONFIRMATION.md section
# 2) - this script exists to make checking the whole matrix a single command instead of manually
# re-running each variant by hand and losing track of which one was tried.
#
# ADAPT: the crate/binary name inside repro/, and drop any row this project can't run (e.g. no
# nightly toolchain available for the sanitizer rows, or miri can't run this code path at all -
# see COMMANDS.md).
set -uo pipefail

repro_dir="${1:?usage: sanitizer_matrix.sh <path to repro/ crate dir>}"
cd "$repro_dir" || exit 1

run_row() {
    local label="$1"; shift
    echo "=== $label ==="
    if "$@" >/tmp/sanmatrix_out.$$ 2>&1; then
        echo "  did not reproduce"
    else
        echo "  REPRODUCED"
        tail -20 /tmp/sanmatrix_out.$$ | sed 's/^/  | /'
    fi
    rm -f /tmp/sanmatrix_out.$$
    echo
}

# ADAPT: replace REPLACE_WITH_BIN with the actual binary/example name this repro builds.

run_row "debug"          bash -c "cargo run --quiet"
run_row "release"        bash -c "cargo run --release --quiet"
run_row "miri"           bash -c "cargo +nightly miri run --quiet"
run_row "ASAN"           bash -c "RUSTFLAGS='-Zsanitizer=address' cargo +nightly run --quiet --target x86_64-unknown-linux-gnu"
run_row "UBSAN"          bash -c "RUSTFLAGS='-Zsanitizer=undefined' cargo +nightly run --quiet --target x86_64-unknown-linux-gnu"

echo "Record in RESULT.md which rows reproduced and which didn't - a bug real only under one" \
     "config is still Confirmed, per CONFIRMATION.md section 2."
