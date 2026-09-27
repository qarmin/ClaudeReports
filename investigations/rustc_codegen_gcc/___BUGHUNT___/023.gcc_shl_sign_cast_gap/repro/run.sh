#!/usr/bin/env bash
# Runs candidate 023's investigation against a checkout of rustc_codegen_gcc at $REPO_DIR
# (defaults to the current directory if it looks like the project root, otherwise pass it as $1).
#
# Requires the project's own build already done once (`./y.sh prepare && ./y.sh build --sysroot
# --release`) - see ___BUGHUNT___/README.md's toolchain table. This script only rebuilds the
# backend .so itself (fast, a few seconds), it never rebuilds the sysroot.
#
# Compiles and runs shl_boundary.rs through the real backend (release/-O), exercising
# gcc_shl's mixed-sign branches (src/int.rs, a_type.is_unsigned()&&b_type.is_signed() and the
# mirror case) with boundary values (top-bit-set, all-ones, MIN, shift-by-width-1) across every
# native integer width the backend treats as native (8/16/32/64-bit). Exits 0 (prints "all shl
# checks passed") if every check matches Rust's own `wrapping_shl` reference; panics (exit 101)
# on the first mismatch otherwise.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_FILE="$SCRIPT_DIR/shl_boundary.rs"

REPO_DIR="$(cd "${1:-$(pwd)}" && pwd)"
cd "$REPO_DIR"

if [ ! -f "y.sh" ] || [ ! -d "src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

echo "=== rustc_codegen_gcc, release/-O, real backend ==="
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -O -o /tmp/shl_boundary_out
/tmp/shl_boundary_out
