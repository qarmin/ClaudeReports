#!/usr/bin/env bash
# LLVM-vs-cg_clif repro for the big-endian AArch64 Cranelift assert ICE.
#
#   CLIF_ROOT=<a built rustc_codegen_cranelift checkout> bash run.sh
#
# CLIF_ROOT defaults to ../clone (what the candidate justfile creates). It must already have been
# built with ./y.sh build, so that $CLIF_ROOT/dist/{rustc-clif,cargo-clif} exist.
#
# Needs: qemu-aarch64_be (user-mode big-endian aarch64 emulation, so the produced binary is
# actually RUN) and the rust-src component (there is no rustup rust-std for any aarch64_be
# target - `core` is built from source).
#
# Pass criterion: LLVM must build/link/run the big-endian binary and print $EXPECTED, and cg_clif
# must NOT ICE on any aarch64_be target. Cranelift has no big-endian aarch64 backend at all, so
# the best cg_clif can do is a clean `error: can't compile for ...` diagnostic - that counts as
# fixed. An ICE ("the compiler unexpectedly panicked") is the bug.
set -uo pipefail
ulimit -c 0
export RUSTC_ICE=0   # don't drop rustc-ice-*.txt dumps into the cwd of every rustc invocation

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF_ROOT="${CLIF_ROOT:-$HERE/../clone}"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
CARGO_CLIF="$CLIF_ROOT/dist/cargo-clif"
WORK="$HERE/work"
RUN_TARGET="aarch64_be-unknown-none-softfloat"   # the only aarch64_be target rust-lld can link
CARGO_TARGET="aarch64_be-unknown-linux-gnu"
ALL_BE_TARGETS="aarch64_be-unknown-hermit aarch64_be-unknown-linux-gnu \
aarch64_be-unknown-linux-gnu_ilp32 aarch64_be-unknown-linux-musl aarch64_be-unknown-netbsd \
aarch64_be-unknown-none-softfloat"
CONTROL_TARGETS="aarch64-unknown-linux-gnu aarch64-apple-darwin s390x-unknown-linux-gnu \
x86_64-unknown-linux-gnu riscv64gc-unknown-linux-gnu"
EXPECTED="7890598189674834828"

for tool in qemu-aarch64_be rustc cargo; do
    command -v "$tool" >/dev/null || { echo "MISSING TOOL: $tool"; exit 2; }
done
[ -x "$RUSTC_CLIF" ] || { echo "MISSING: $RUSTC_CLIF (run ./y.sh build in $CLIF_ROOT)"; exit 2; }

SYSROOT="$(rustc --print sysroot)"
CORE_SRC="$SYSROOT/lib/rustlib/src/rust/library/core/src/lib.rs"
[ -f "$CORE_SRC" ] || { echo "MISSING: rust-src component ($CORE_SRC)"; exit 2; }

rm -rf "$WORK"; mkdir -p "$WORK"
fail=0
ice=0

echo "=============================================================="
echo "PART 1 - the realistic path: cargo build -Zbuild-std=core --target $CARGO_TARGET"
echo "         (an ordinary no_std lib crate; cargo also has to codegen a real core)"
echo "=============================================================="
echo "--- plain cargo (LLVM) ---"
CARGO_TARGET_DIR="$WORK/t_llvm" cargo build -Zbuild-std=core --target "$CARGO_TARGET" \
    --manifest-path "$HERE/lib_probe/Cargo.toml" >"$WORK/cargo_llvm.log" 2>&1
if [ $? -eq 0 ]; then
    echo "built OK: $(ls "$WORK/t_llvm/$CARGO_TARGET/debug/"libbe_probe.* 2>/dev/null | tr '\n' ' ')"
else
    echo "plain cargo FAILED (see $WORK/cargo_llvm.log):"; tail -20 "$WORK/cargo_llvm.log"; fail=1
fi

echo "--- cargo-clif (cg_clif) ---"
CARGO_TARGET_DIR="$WORK/t_clif" "$CARGO_CLIF" build -Zbuild-std=core --target "$CARGO_TARGET" \
    --manifest-path "$HERE/lib_probe/Cargo.toml" >"$WORK/cargo_clif.log" 2>&1
if grep -q "unexpectedly panicked" "$WORK/cargo_clif.log"; then
    echo "cg_clif ICEd:"
    grep -m2 -E "panicked at|assertion failed" "$WORK/cargo_clif.log"
    ice=1
else
    echo "no ICE. Diagnostics from cargo-clif:"
    grep -m4 -E "^error" "$WORK/cargo_clif.log" || echo "  (none - build succeeded)"
fi

echo
echo "=============================================================="
echo "PART 2 - build + link + RUN a real big-endian aarch64 binary ($RUN_TARGET)"
echo "=============================================================="
o="$WORK/$RUN_TARGET"; mkdir -p "$o"
rustc --edition 2024 --crate-type lib --crate-name core --target "$RUN_TARGET" \
    -C opt-level=0 -o "$o/libcore.rlib" "$CORE_SRC" >"$o/core.log" 2>&1 || {
    echo "could not build core for $RUN_TARGET:"; tail -10 "$o/core.log"; fail=1; }
rustc --edition 2024 --crate-type lib --crate-name compiler_builtins --target "$RUN_TARGET" \
    -C opt-level=0 -C panic=abort --extern core="$o/libcore.rlib" \
    -A stable_features -o "$o/libcompiler_builtins.rlib" "$HERE/cb_stub.rs" \
    >"$o/cb.log" 2>&1 || { echo "could not build compiler_builtins stub:"; tail -10 "$o/cb.log"; fail=1; }

build_and_run() { # $1 = rustc binary, $2 = tag
    "$1" --edition 2024 --crate-type bin --target "$RUN_TARGET" -C opt-level=0 -C panic=abort \
        --extern core="$o/libcore.rlib" \
        --extern compiler_builtins="$o/libcompiler_builtins.rlib" \
        -C link-arg=-e_start \
        -o "$o/bin_$2" "$HERE/src/main.rs" >"$o/build_$2.log" 2>&1
    if [ ! -x "$o/bin_$2" ]; then
        if grep -q "unexpectedly panicked" "$o/build_$2.log"; then
            echo "  $2: ICE -> $(grep -m1 -A1 'panicked at' "$o/build_$2.log" | tr '\n' ' ')"
            return 2
        fi
        echo "  $2: build failed -> $(grep -m1 '^error' "$o/build_$2.log")"
        return 1
    fi
    echo "  $2: $(file -b "$o/bin_$2")"
    echo "  $2: qemu-aarch64_be printed [$(qemu-aarch64_be "$o/bin_$2")]"
    return 0
}

build_and_run rustc llvm; llvm_rc=$?
llvm_out=$(qemu-aarch64_be "$o/bin_llvm" 2>/dev/null)
build_and_run "$RUSTC_CLIF" clif; clif_rc=$?
if [ $llvm_rc -ne 0 ] || [ "$llvm_out" != "$EXPECTED" ]; then
    echo "  BASELINE BROKEN (LLVM rc=$llvm_rc out=[$llvm_out], wanted $EXPECTED)"; fail=1
fi
[ $clif_rc -eq 2 ] && ice=1

echo
echo "=============================================================="
echo "PART 3 - all six aarch64_be targets, plus control targets that must keep working"
echo "         (tiny #![no_core] crate, --emit=obj, no sysroot needed)"
echo "=============================================================="
classify() { # $1 = target
    local out rc
    out=$("$RUSTC_CLIF" --edition 2021 --crate-type lib --target "$1" --emit=obj \
        -o "$WORK/c.o" "$HERE/sweep_nocore.rs" 2>&1); rc=$?
    if grep -q "unexpectedly panicked" <<<"$out"; then
        echo "ICE|$(grep -m1 'assertion failed' <<<"$out" | head -c 110)"
    elif [ $rc -eq 0 ]; then
        echo "CLIF_OK|"
    else
        echo "ERROR|$(grep -m1 '^error' <<<"$out" | head -c 130)"
    fi
}
for t in $ALL_BE_TARGETS; do
    r=$(classify "$t")
    printf '  %-40s %s\n' "$t" "$r"
    [[ "$r" == ICE* ]] && ice=1
done
echo "  --- controls (must stay CLIF_OK) ---"
for t in $CONTROL_TARGETS; do
    r=$(classify "$t")
    printf '  %-40s %s\n' "$t" "$r"
    [[ "$r" == CLIF_OK* ]] || { echo "  CONTROL TARGET REGRESSED"; fail=1; }
done

echo
echo "  --- the -Ctarget-cpu=native path on $CARGO_TARGET ---"
rm -f "$WORK/cn.o"
nout=$("$RUSTC_CLIF" --edition 2021 --crate-type lib --target "$CARGO_TARGET" \
    -C target-cpu=native --emit=obj -o "$WORK/cn.o" "$HERE/sweep_nocore.rs" 2>&1)
if grep -q "unexpectedly panicked" <<<"$nout"; then
    echo "  ICE"; ice=1
else
    echo "  $(grep -m1 '^error' <<<"$nout" || echo 'no error')"
    echo "  emitted object: $(file -b "$WORK/cn.o" 2>/dev/null || echo none)"
fi

echo
if [ $ice -ne 0 ]; then
    echo "RESULT: BUG PRESENT (cg_clif ICEd on big-endian aarch64)"
    exit 1
fi
if [ $fail -ne 0 ]; then
    echo "RESULT: HARNESS PROBLEM (LLVM baseline or a control target broke) - see above"
    exit 2
fi
echo "RESULT: OK (no ICE; cg_clif rejects big-endian aarch64 with a clean diagnostic)"
