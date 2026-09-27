#!/usr/bin/env bash
# Runs the full candidate-014 investigation against a checkout of rustc_codegen_gcc at $REPO_DIR
# (defaults to the current directory if it looks like the project root, otherwise pass it as $1).
#
# Requires the project's own build already done once (`./y.sh prepare && ./y.sh build --sysroot
# --release`) - see ___BUGHUNT___/README.md's toolchain table. This script only rebuilds the
# backend .so itself (fast, a few seconds), it never rebuilds the sysroot.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_FILE="$SCRIPT_DIR/width_cast_boundary.rs"
PATCH_FILE="$SCRIPT_DIR/force_non_native.patch"

REPO_DIR="$(cd "${1:-$(pwd)}" && pwd)"
cd "$REPO_DIR"

if [ ! -f "y.sh" ] || [ ! -d "src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

echo "=== 0. Reference: plain rustc/LLVM (ground truth) ==="
rustc "$TEST_FILE" -o /tmp/width_cast_native --edition 2021
/tmp/width_cast_native

echo
echo "=== 1. rustc_codegen_gcc, native i128/u128 GCC types (baseline, supports_128bit_integers=true on this host) ==="
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -o /tmp/width_cast_gcc_native
/tmp/width_cast_gcc_native

echo
echo "=== 2. rustc_codegen_gcc, forced non-native (array-based) i128/u128 path, release backend ==="
git apply "$PATCH_FILE"
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -o /tmp/width_cast_gcc_forced_release
/tmp/width_cast_gcc_forced_release

echo
echo "=== 3. rustc_codegen_gcc, forced non-native (array-based) i128/u128 path, DEBUG backend (debug_assert! active in src/int.rs) ==="
./y.sh build >/dev/null
./y.sh rustc "$TEST_FILE" -o /tmp/width_cast_gcc_forced_debug
/tmp/width_cast_gcc_forced_debug

git checkout -- src/context.rs
CHANNEL=release ./y.sh build --release >/dev/null

echo
echo "=== 4. Diffs against rustc/LLVM reference ==="
ok=1
if diff <(/tmp/width_cast_native) <(/tmp/width_cast_gcc_native); then
    echo "MATCH: native-128-bit-int backend path."
else
    echo "MISMATCH: native-128-bit-int backend path."
    ok=0
fi
if diff <(/tmp/width_cast_native) <(/tmp/width_cast_gcc_forced_release); then
    echo "MATCH: forced array-based (non-native) path, release backend."
else
    echo "MISMATCH: forced array-based (non-native) path, release backend."
    ok=0
fi
if diff <(/tmp/width_cast_native) <(/tmp/width_cast_gcc_forced_debug); then
    echo "MATCH: forced array-based (non-native) path, debug backend (debug_assert! active)."
else
    echo "MISMATCH: forced array-based (non-native) path, debug backend."
    ok=0
fi

if [ "$ok" -eq 1 ]; then
    echo
    echo "RESULT: no width-cast logic bug found in src/int.rs - all three backend configurations match rustc/LLVM."
else
    echo
    echo "RESULT: a mismatch was found above - this would indicate a real width-cast codegen bug."
fi
