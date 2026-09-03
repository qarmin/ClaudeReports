#!/usr/bin/env bash
# Runs the full candidate-021 investigation against a checkout of rustc_codegen_gcc at $REPO_DIR
# (defaults to the current directory if it looks like the project root, otherwise pass it as $1).
#
# Requires the project's own build already done once (`./y.sh prepare && ./y.sh build --sysroot
# --release`) - see ___BUGHUNT___/README.md's toolchain table. This script only rebuilds the
# backend .so itself (fast, a few seconds), it never rebuilds the sysroot.
set -euo pipefail

# Resolve these before any `cd` below - both must stay absolute regardless of cwd changes.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_FILE="$SCRIPT_DIR/mul_boundary.rs"
PATCH_FILE="$SCRIPT_DIR/force_non_native.patch"

REPO_DIR="$(cd "${1:-$(pwd)}" && pwd)"
cd "$REPO_DIR"

if [ ! -f "y.sh" ] || [ ! -d "src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

echo "=== 0. Reference: plain rustc/LLVM (ground truth) ==="
rustc "$TEST_FILE" -o /tmp/mul_boundary_native --edition 2021
/tmp/mul_boundary_native

echo
echo "=== 1. rustc_codegen_gcc, native i128/u128 GCC types (baseline, supports_128bit_integers=true on this host) ==="
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -o /tmp/mul_boundary_gcc_native
/tmp/mul_boundary_gcc_native

echo
echo "=== 2. rustc_codegen_gcc, forced non-native (array-based) i128/u128 path (gcc_mul's hardcoded signed=true -> __multi3) ==="
git apply "$PATCH_FILE"
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -o /tmp/mul_boundary_gcc_forced
/tmp/mul_boundary_gcc_forced
git checkout -- src/context.rs
CHANNEL=release ./y.sh build --release >/dev/null

echo
echo "=== 3. Confirm __multi3 (not __umulti3) was actually called in the forced-non-native binary ==="
objdump -d /tmp/mul_boundary_gcc_forced | grep -c "call.*<__multi3>" || true
objdump -d /tmp/mul_boundary_gcc_forced | grep -i "umulti3" && echo "UNEXPECTED: __umulti3 reference found" || echo "confirmed: no __umulti3 reference exists (matches libgcc/compiler_builtins, which never ship one)"

echo
echo "=== 4. Diff: forced-non-native output vs. native rustc/LLVM reference ==="
if diff <(/tmp/mul_boundary_native) <(/tmp/mul_boundary_gcc_forced); then
    echo "MATCH: array-based (non-native) i128/u128 multiplication gives the same results as rustc/LLVM."
else
    echo "MISMATCH: see diff above - this would indicate a real sign-selection bug."
fi
