#!/usr/bin/env bash
# Runs the full candidate-003 investigation against a checkout of rustc_codegen_gcc at $REPO_DIR
# (defaults to the current directory if it looks like the project root, otherwise pass it as $1).
#
# Requires the project's own build already done once (`./y.sh prepare && ./y.sh build --sysroot
# --release`) - see ___BUGHUNT___/README.md's toolchain table. This script only rebuilds the
# backend .so itself (fast, a few seconds), it never rebuilds the sysroot.
set -euo pipefail

# Resolve these before any `cd` below - both must stay absolute regardless of cwd changes.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_FILE="$SCRIPT_DIR/i128_boundary.rs"
PATCH_FILE="$SCRIPT_DIR/force_non_native.patch"

REPO_DIR="$(cd "${1:-$(pwd)}" && pwd)"
cd "$REPO_DIR"

if [ ! -f "y.sh" ] || [ ! -d "src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

echo "=== 0. Reference: plain rustc/LLVM (ground truth) ==="
rustc "$TEST_FILE" -o /tmp/i128_boundary_native --edition 2021
/tmp/i128_boundary_native

echo
echo "=== 1. rustc_codegen_gcc, native i128/u128 GCC types (baseline, supports_128bit_integers=true on this host) ==="
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -o /tmp/i128_boundary_gcc_native
/tmp/i128_boundary_gcc_native

echo
echo "=== 2. rustc_codegen_gcc, forced non-native (array-based) i128/u128 path ==="
git apply "$PATCH_FILE"
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -o /tmp/i128_boundary_gcc_forced
/tmp/i128_boundary_gcc_forced
git checkout -- src/context.rs
CHANNEL=release ./y.sh build --release >/dev/null

echo
echo "=== 3. Diff: forced-non-native output vs. native rustc/LLVM reference ==="
if diff <(/tmp/i128_boundary_native) <(/tmp/i128_boundary_gcc_forced); then
    echo "MATCH: array-based (non-native) i128/u128 division/remainder gives the same results as rustc/LLVM."
else
    echo "MISMATCH: see diff above - this would indicate a real sign-selection bug."
fi
