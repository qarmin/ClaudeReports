#!/usr/bin/env bash
# Build+run this crate twice - once with plain cargo (LLVM, the oracle) and once with the cg_clif
# checkout passed as $1 - then compare.
#
#   bash repro/run.sh <path-to-rustc_codegen_cranelift-checkout>
#
# Two binaries:
#   control  must build and print the same three lines under both compilers.
#   wide     builds and prints under LLVM. Unpatched cg_clif ICEs on it
#            (Compilation(Unsupported("Unexpected SSA-value type: f32x8")) reported via bug!()).
#            With patch.diff applied it is a clean `error:` diagnostic instead of an ICE.
#
# Exit 0 = control matches AND cg_clif did not ICE on wide. Exit 1 = otherwise.
set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF="${1:?usage: run.sh <path-to-cg_clif-checkout>}"
CLIF="$(cd "$CLIF" && pwd)"
CARGO_CLIF="$CLIF/dist/cargo-clif"
[ -x "$CARGO_CLIF" ] || { echo "missing $CARGO_CLIF - run ./y.sh build in $CLIF first" >&2; exit 2; }
ulimit -c 0
cd "$HERE"
rm -f rustc-ice-*.txt

echo "=== LLVM (plain cargo) ==="
if ! cargo build --quiet --target-dir target-llvm 2>&1; then
    echo "LLVM BUILD FAILED - the repro itself is broken, not the backend"; exit 2
fi
./target-llvm/debug/control > llvm.control.out 2>&1; llvm_ctl_rc=$?
./target-llvm/debug/wide    > llvm.wide.out    2>&1; llvm_wide_rc=$?
cat llvm.control.out; echo "control exit $llvm_ctl_rc"
cat llvm.wide.out;    echo "wide exit $llvm_wide_rc"

echo
echo "=== cg_clif ($CLIF) ==="
"$CARGO_CLIF" build --target-dir target-clif --bin control > clif.control.build 2>&1
ctl_build_rc=$?
if [ "$ctl_build_rc" != 0 ]; then
    echo "control: cg_clif BUILD FAILED (this control is supposed to work):"
    grep -E "Error while defining|^error|panicked at" clif.control.build | head -10
    control_ok=0
else
    ./target-clif/debug/control > clif.control.out 2>&1; clif_ctl_rc=$?
    cat clif.control.out; echo "control exit $clif_ctl_rc"
    if [ "$clif_ctl_rc" = "$llvm_ctl_rc" ] && diff -q llvm.control.out clif.control.out > /dev/null; then
        control_ok=1
    else
        control_ok=0
        echo "control: DIFFERS FROM LLVM"; diff llvm.control.out clif.control.out
    fi
fi

echo
"$CARGO_CLIF" build --target-dir target-clif --bin wide > clif.wide.build 2>&1
wide_build_rc=$?
if [ "$wide_build_rc" = 0 ]; then
    ./target-clif/debug/wide > clif.wide.out 2>&1; clif_wide_rc=$?
    cat clif.wide.out; echo "wide exit $clif_wide_rc"
    if [ "$clif_wide_rc" = "$llvm_wide_rc" ] && diff -q llvm.wide.out clif.wide.out > /dev/null; then
        wide_state="COMPILES AND MATCHES LLVM"
    else
        wide_state="COMPILES BUT DISAGREES WITH LLVM"
        diff llvm.wide.out clif.wide.out
    fi
elif grep -q "the compiler unexpectedly panicked" clif.wide.build; then
    wide_state="ICE"
    echo "wide: cg_clif ICEd:"
    grep -E "panicked at src/base.rs|Error while defining|the compiler unexpectedly panicked" clif.wide.build | head -4
else
    wide_state="CLEAN ERROR"
    echo "wide: cg_clif rejected the crate with a diagnostic (no ICE):"
    grep -E "^error" clif.wide.build | head -4
fi
rm -f rustc-ice-*.txt

echo
echo "control: $([ "${control_ok:-0}" = 1 ] && echo 'matches LLVM' || echo 'DIFFERS from LLVM')"
echo "wide:    $wide_state"
if [ "${control_ok:-0}" = 1 ] && [ "$wide_state" != "ICE" ] && [ "$wide_state" != "COMPILES BUT DISAGREES WITH LLVM" ]; then
    echo "RESULT: no ICE"
    exit 0
fi
echo "RESULT: BUG PRESENT"
exit 1
