#!/usr/bin/env bash
# Runs the full candidate-010 investigation against a checkout of rustc_codegen_gcc at $REPO_DIR
# (defaults to the current directory if it looks like the project root, otherwise pass it as $1).
#
# Requires the project's own build already done once (`./y.sh prepare && ./y.sh build --sysroot
# --release`) - see ___BUGHUNT___/README.md's toolchain table. This script only rebuilds the
# backend .so itself (fast, a few seconds), it never rebuilds the sysroot.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_FILE="$SCRIPT_DIR/u128_as_f32.rs"

REPO_DIR="$(cd "${1:-$(pwd)}" && pwd)"
cd "$REPO_DIR"

if [ ! -f "y.sh" ] || [ ! -d "src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

echo "=== 0. Reference: plain rustc/LLVM (ground truth) - must pass ==="
rustc -O "$TEST_FILE" -o /tmp/u128_as_f32_native --edition 2021
/tmp/u128_as_f32_native

echo
echo "=== 1. rustc_codegen_gcc, WITHOUT optimizations (-C opt-level=0) - expected: passes ==="
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -o /tmp/u128_as_f32_gcc_o0
/tmp/u128_as_f32_gcc_o0

echo
echo "=== 2. rustc_codegen_gcc, WITH optimizations (-O, i.e. -C opt-level=3) - this is the bug ==="
CHANNEL=release ./y.sh rustc "$TEST_FILE" -O -o /tmp/u128_as_f32_gcc_o3
set +e
/tmp/u128_as_f32_gcc_o3
status=$?
set -e
echo "exit status: $status"
if [ "$status" -eq 0 ]; then
    echo "NOT REPRODUCED: -O build passed (bug may have been fixed upstream in libgccjit)."
    exit 1
else
    echo "REPRODUCED: -O build panicked, matching issue #705."
fi
