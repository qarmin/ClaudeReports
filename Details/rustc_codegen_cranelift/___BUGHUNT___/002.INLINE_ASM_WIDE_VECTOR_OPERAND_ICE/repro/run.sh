#!/usr/bin/env bash
# Build+run this crate twice - once with plain cargo (LLVM, the oracle) and once with the
# cg_clif checkout passed as $1 - then diff the two run-time outputs.
#
#   bash repro/run.sh <path-to-rustc_codegen_cranelift-checkout>
#
# Exit 0 = the two agree, exit 1 = they differ (or cg_clif failed to compile the crate at all).
set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF="${1:?usage: run.sh <path-to-cg_clif-checkout>}"
CLIF="$(cd "$CLIF" && pwd)"
CARGO_CLIF="$CLIF/dist/cargo-clif"
[ -x "$CARGO_CLIF" ] || { echo "missing $CARGO_CLIF - run ./y.sh build in $CLIF first" >&2; exit 2; }
ulimit -c 0
cd "$HERE"

echo "=== LLVM (plain cargo) ==="
if ! cargo build --quiet --target-dir target-llvm 2>&1; then
    echo "LLVM BUILD FAILED - the repro itself is broken, not the backend"; exit 2
fi
./target-llvm/debug/wide_asm_repro > llvm.out 2>&1; llvm_rc=$?
cat llvm.out; echo "exit $llvm_rc"

echo
echo "=== cg_clif ($CLIF) ==="
if ! RUSTFLAGS="-Ctarget-cpu=native" "$CARGO_CLIF" build --target-dir target-clif > clif.build 2>&1; then
    echo "cg_clif BUILD FAILED:"
    grep -E "Error while defining|^error|panicked at" clif.build | head -20
    echo
    rm -f rustc-ice-*.txt   # rustc drops an ICE dump per crashed worker in the cwd
    echo "RESULT: DIFFERENT (cg_clif could not compile the crate)"
    exit 1
fi
./target-clif/debug/wide_asm_repro > clif.out 2>&1; clif_rc=$?
cat clif.out; echo "exit $clif_rc"

echo
if [ "$llvm_rc" = "$clif_rc" ] && diff -q llvm.out clif.out > /dev/null; then
    echo "RESULT: identical - cg_clif matches LLVM on all 15 cases"
    exit 0
fi
echo "RESULT: DIFFERENT"
diff llvm.out clif.out
exit 1
