#!/usr/bin/env bash
# Runs the full candidate-022 investigation against a checkout of rustc_codegen_gcc at $REPO_DIR
# (defaults to the current directory if it looks like the project root, otherwise pass it as $1).
#
# Requires the project's own build already done once (`./y.sh prepare && ./y.sh build --sysroot
# --release`) - see ___BUGHUNT___/README.md's toolchain table. This script only rebuilds the
# backend .so itself (fast, a few seconds), it never rebuilds the sysroot.
#
# Compiles and runs three programs through the real backend end-to-end (./y.sh rustc, the
# project's own driver script wrapping rustc -Zcodegen-backend=<real librustc_codegen_gcc.so>):
#   1. scalar_sign_dedup.rs - scalar/pointer int comparisons after a pointer-to-integer transmute
#      (the same declared-sign-corruption trick that broke gcc_lshr/gcc_ashr in candidate 018).
#   2. simd_sign_dedup.rs   - the SIMD-vector analog, since gcc_icmp explicitly skips its
#      sign-correcting cast for vector types (portable_simd, requires a nightly toolchain -
#      this project's own pinned rust-toolchain is nightly already).
#   3. fnptr_cmp.rs         - function-pointer ordering comparisons (a_type/b_type end up being
#      function-pointer GCC types, not caught by the data-pointer branch).
# Each program exits 0 (and prints its own "... passed" line) if gcc_icmp behaves correctly;
# a wrong comparison result trips an `assert!`/`assert_eq!` and the program exits with a panic
# (non-zero exit code).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${1:-$(pwd)}" && pwd)"
cd "$REPO_DIR"

if [ ! -f "y.sh" ] || [ ! -d "src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

CHANNEL=release ./y.sh build --release >/dev/null

echo "=== 1. scalar_sign_dedup.rs (ptr->int transmute sign-corruption, scalar/pointer icmp) ==="
CHANNEL=release ./y.sh rustc "$SCRIPT_DIR/scalar_sign_dedup.rs" -o /tmp/candidate022_scalar
/tmp/candidate022_scalar

echo
echo "=== 2. simd_sign_dedup.rs (SIMD-vector analog, is_vector() sign-cast skip) ==="
CHANNEL=release ./y.sh rustc "$SCRIPT_DIR/simd_sign_dedup.rs" -o /tmp/candidate022_simd
/tmp/candidate022_simd

echo
echo "=== 3. fnptr_cmp.rs (function-pointer ordering comparison) ==="
CHANNEL=release ./y.sh rustc "$SCRIPT_DIR/fnptr_cmp.rs" -o /tmp/candidate022_fnptr
/tmp/candidate022_fnptr

echo
echo "All three checks passed - gcc_icmp's native comparison path did not reproduce the sign-mismatch bug class."
