#!/usr/bin/env bash
# Runs the full candidate-017 investigation against a checkout of rustc_codegen_gcc at $REPO_DIR
# (defaults to the current directory if it looks like the project root, otherwise pass it as $1).
#
# Exercises `Builder::checked_binop` -> `gcc_checked_binop` (src/int.rs:294+) and the checked/
# overflowing/wrapping/saturating add/sub/mul methods that lower to it, across every integer type,
# on boundary values, comparing the runtime (JIT-codegen'd) result against a `const`-evaluated
# (CTFE, backend-independent) expected value baked into the same program.
#
# Runs it twice: once against the backend as shipped (native GCC __int128 path on a normal x86_64
# host, taken by `is_native_int_type`), and once with `force_non_native.patch` applied (forces
# `u128_type_supported = false`, per candidate 003's technique) and the sysroot rebuilt, to
# exercise the otherwise-unreachable-on-this-host non-native fallback path (`operation_with_overflow`
# / `__rust_i128_addo` / `__rust_i128_subo` / `__rust_i128_mulo` / `__rust_u128_mulo` etc.).
#
# Requires the project's own build already done once (`./y.sh prepare && ./y.sh build --sysroot
# --release`) - see ___BUGHUNT___/README.md's toolchain table. Rebuilding the sysroot under the
# forced patch (step 2) takes about a minute; this script always operates on $REPO_DIR's own
# build/build_sysroot directly (safe here since `clone/` is a private, throwaway checkout - never
# do this against a shared/symlinked build_sysroot).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_FILE="$SCRIPT_DIR/overflow_boundary.rs"
PATCH_FILE="$SCRIPT_DIR/force_non_native.patch"

REPO_DIR="$(cd "${1:-$(pwd)}" && pwd)"
cd "$REPO_DIR"

if [ ! -f "y.sh" ] || [ ! -d "src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

echo "=== 0. Reference: plain rustc/LLVM (ground truth for the OVERALL line only - the real oracle is each check's own CTFE const) ==="
rustc "$TEST_FILE" -o /tmp/overflow_boundary_native --edition 2021 -O
/tmp/overflow_boundary_native

echo
echo "=== 1. rustc_codegen_gcc, native i128/u128 GCC types (baseline, as shipped) ==="
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -o /tmp/overflow_boundary_gcc_native
/tmp/overflow_boundary_gcc_native

echo
echo "=== 2. rustc_codegen_gcc, forced non-native (array-based) i128/u128 path ==="
git apply "$PATCH_FILE"
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh build --release --sysroot >/dev/null
CHANNEL=release ./y.sh rustc "$TEST_FILE" -o /tmp/overflow_boundary_gcc_forced
echo "-- confirming the non-native path was actually reached (looking for __rust_*128*o calls) --"
objdump -d /tmp/overflow_boundary_gcc_forced | grep -o '__rust_[iu]128_[a-z]*o' | sort -u
/tmp/overflow_boundary_gcc_forced
git checkout -- src/base.rs
CHANNEL=release ./y.sh build --release >/dev/null
CHANNEL=release ./y.sh build --release --sysroot >/dev/null

echo
echo "=== Done: both backend runs above should print OVERALL: PASS, same as the rustc/LLVM reference. ==="
