#!/usr/bin/env bash
# Build and run one repro under a matrix of build configs, report which show the bug.
#
#   sanitizer_matrix.sh <path to repro/ crate dir>
#
# A finding that only shows up under one specific config is still real (CONFIRMATION.md section
# 2) - this script exists to make checking the whole matrix a single command instead of manually
# re-running each variant by hand and losing track of which one was tried.
#
# ADAPTED for image-png (2026-09-02): the miri/ASAN/UBSAN rows the generic template ships with
# have been REMOVED, not commented out. This crate is `#![forbid(unsafe_code)]` (src/lib.rs:62) -
# there is no unsafe anywhere in it, so those three tools have nothing to find and only burn a
# candidate's 3-approach budget. See ___BUGHUNT___/README.md.
#
# What's left is the distinction that actually matters here: debug (overflow checks ON, so
# arithmetic bugs panic) vs release (overflow wraps silently). A repro that fires in debug but
# not release is usually an integer-overflow finding; one that fires in both is an index/logic
# bug. Record which.
set -uo pipefail

repro_dir="${1:?usage: sanitizer_matrix.sh <path to repro/ crate dir>}"
cd "$repro_dir" || exit 1

# A crashing repro would otherwise dump a core file per row - see confirm_repro.sh's own note.
ulimit -c 0

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

run_row "debug (overflow checks on)"   bash -c "cargo run --quiet"
run_row "release (overflow wraps)"     bash -c "cargo run --release --quiet"
run_row "release + overflow checks"    bash -c "RUSTFLAGS='-C overflow-checks=on' cargo run --release --quiet"

echo "Record in RESULT.md which rows reproduced and which didn't - a bug real only under one" \
     "config is still Confirmed, per CONFIRMATION.md section 2. debug-only => almost certainly" \
     "an arithmetic-overflow finding; state that explicitly rather than calling it a crash."
