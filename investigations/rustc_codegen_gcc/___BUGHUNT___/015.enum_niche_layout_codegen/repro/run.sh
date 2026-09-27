#!/usr/bin/env bash
# Runs the full candidate-015 investigation against a checkout of rustc_codegen_gcc at $REPO_DIR
# (defaults to the current directory if it looks like the project root, otherwise pass it as $1).
#
# Requires the project's own build already done once (`./y.sh prepare && ./y.sh build --sysroot
# --release`) - see ___BUGHUNT___/README.md's toolchain table. This script only rebuilds the
# backend .so itself (fast, a few seconds), it never rebuilds the sysroot.
#
# Each of the four programs below is a self-checking assertion program (no LLVM reference needed
# for correctness - the assertions encode the known-correct answer directly per Rust's documented
# enum/cast semantics), compiled and run under rustc_codegen_gcc at every optimization level
# 0-3 (candidate 010, a prior finding in this same workspace, showed GCC-optimizer-dependent
# codegen bugs only appear above -O0, so every level is checked, not just the default).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${1:-$(pwd)}" && pwd)"
cd "$REPO_DIR"

if [ ! -f "y.sh" ] || [ ! -d "src" ]; then
    echo "error: $REPO_DIR does not look like a rustc_codegen_gcc checkout" >&2
    exit 1
fi

CHANNEL=release ./y.sh build --release >/dev/null

run_at_opt_levels() {
    local src="$1"
    local name
    name="$(basename "$src" .rs)"
    for opt in 0 1 2 3; do
        echo "--- $name, opt-level=$opt ---"
        CHANNEL=release ./y.sh rustc "$src" -C "opt-level=$opt" -o "/tmp/cg015_${name}_o${opt}" --edition 2021
        "/tmp/cg015_${name}_o${opt}"
    done
}

echo "=== enum_niche_test.rs: signed Direct tag encoding + niche encoding both directions (low/high) + common std niches ==="
run_at_opt_levels "$SCRIPT_DIR/enum_niche_test.rs"

echo
echo "=== many_niche.rs: 140 fieldless niche variants around a signed 3-value niche source (forces the codegen_get_discr signed-comparison branch, per rustc_codegen_ssa's own comment about when it picks IntSLE/IntSGE over IntULE/IntUGE) ==="
run_at_opt_levels "$SCRIPT_DIR/many_niche.rs"

echo
echo "=== i128_enum.rs: repr(i128) Direct tag encoding, exercises the non-native (array-based) gcc_icmp/switch path for a 128-bit discriminant ==="
run_at_opt_levels "$SCRIPT_DIR/i128_enum.rs"

echo
echo "=== async_test.rs: a real coroutine (async fn) state-machine enum, matching inside a niche-optimized Option ==="
CHANNEL=release ./y.sh rustc "$SCRIPT_DIR/async_test.rs" -o /tmp/cg015_async --edition 2021
/tmp/cg015_async

echo
echo "All programs asserted their own correctness inline (see each .rs) - no crash/assertion failure across any of them at any optimization level means no enum/niche discriminant mismatch was found."
