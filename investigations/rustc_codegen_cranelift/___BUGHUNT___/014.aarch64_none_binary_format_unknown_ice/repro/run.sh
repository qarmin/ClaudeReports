#!/usr/bin/env bash
# LLVM-vs-cg_clif differential repro for the `os = "none"` aarch64 ICE.
#
#   CLIF_ROOT=<a built rustc_codegen_cranelift checkout> bash run.sh
#
# CLIF_ROOT defaults to ../clone (what the candidate justfile creates). It must already have been
# built with ./y.sh build, so that $CLIF_ROOT/dist/{rustc-clif,cargo-clif} exist.
#
# Needs: qemu-aarch64 (user-mode emulation, to actually RUN the aarch64 binary) and the rust-src
# component (to build `core` for the bare-metal target - there is no rustup rust-std for it).
set -uo pipefail
ulimit -c 0

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF_ROOT="${CLIF_ROOT:-$HERE/../clone}"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
CARGO_CLIF="$CLIF_ROOT/dist/cargo-clif"
WORK="$HERE/work"
TARGETS="${TARGETS:-aarch64-unknown-none aarch64-unknown-none-softfloat}"
EXPECTED="7890598189674834828"

for tool in qemu-aarch64 rustc cargo; do
    command -v "$tool" >/dev/null || { echo "MISSING TOOL: $tool"; exit 2; }
done
[ -x "$RUSTC_CLIF" ] || { echo "MISSING: $RUSTC_CLIF (run ./y.sh build in $CLIF_ROOT)"; exit 2; }

SYSROOT="$(rustc --print sysroot)"
LLD="$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld"
CORE_SRC="$SYSROOT/lib/rustlib/src/rust/library/core/src/lib.rs"
[ -x "$LLD" ] || { echo "MISSING: rust-lld at $LLD"; exit 2; }
[ -f "$CORE_SRC" ] || { echo "MISSING: rust-src component ($CORE_SRC)"; exit 2; }

rm -rf "$WORK"; mkdir -p "$WORK"
fail=0

echo "=============================================================="
echo "PART 1 - the realistic path: cargo build --target aarch64-unknown-none -Zbuild-std=core"
echo "=============================================================="
echo "--- plain cargo (LLVM) ---"
CARGO_TARGET_DIR="$WORK/t_llvm" cargo build -Zbuild-std=core \
    --target aarch64-unknown-none --offline --manifest-path "$HERE/Cargo.toml" \
    >"$WORK/cargo_llvm.log" 2>&1
if [ $? -eq 0 ]; then
    out=$(qemu-aarch64 "$WORK/t_llvm/aarch64-unknown-none/debug/bare_aarch64_none")
    echo "built OK; qemu-aarch64 printed: $out"
    [ "$out" = "$EXPECTED" ] || { echo "UNEXPECTED LLVM OUTPUT (wanted $EXPECTED)"; fail=1; }
else
    echo "plain cargo FAILED (see $WORK/cargo_llvm.log):"; tail -20 "$WORK/cargo_llvm.log"; fail=1
fi

echo "--- cargo-clif (cg_clif) ---"
CARGO_TARGET_DIR="$WORK/t_clif" "$CARGO_CLIF" build -Zbuild-std=core \
    --target aarch64-unknown-none --offline --manifest-path "$HERE/Cargo.toml" \
    >"$WORK/cargo_clif.log" 2>&1
if grep -q "driver/aot.rs" "$WORK/cargo_clif.log" && \
   grep -q "binary format is unknown" "$WORK/cargo_clif.log"; then
    echo "cg_clif ICEd:"
    grep -m2 -E "panicked at|Err. value|unexpectedly panicked" "$WORK/cargo_clif.log"
    fail=1
else
    echo "no 'binary format is unknown' ICE from cargo-clif. Remaining diagnostics (if any):"
    grep -m6 -E "^error|panicked at" "$WORK/cargo_clif.log" || echo "  (none - build succeeded)"
fi

echo
echo "=============================================================="
echo "PART 2 - build + link + RUN the aarch64 binary, LLVM vs cg_clif"
echo "=============================================================="
for target in $TARGETS; do
    echo "----- $target -----"
    o="$WORK/$target"; mkdir -p "$o"
    rustc --edition 2024 --crate-type lib --crate-name core --target "$target" \
        -C opt-level=0 -o "$o/libcore.rlib" "$CORE_SRC" >"$o/core.log" 2>&1 || {
        echo "could not build core for $target:"; tail -10 "$o/core.log"; fail=1; continue; }
    rustc --edition 2024 --crate-type lib --crate-name compiler_builtins --target "$target" \
        -C opt-level=0 -C panic=abort --extern core="$o/libcore.rlib" \
        -A stable_features -o "$o/libcompiler_builtins.rlib" "$HERE/cb_stub.rs" \
        >"$o/cb.log" 2>&1 || {
        echo "could not build compiler_builtins stub:"; tail -10 "$o/cb.log"; fail=1; continue; }

    build_and_run() { # $1 = rustc binary, $2 = tag
        "$1" --edition 2024 --crate-type bin --target "$target" -C opt-level=0 -C panic=abort \
            --extern core="$o/libcore.rlib" \
            --extern compiler_builtins="$o/libcompiler_builtins.rlib" \
            -C link-arg=-e_start \
            -o "$o/bin_$2" "$HERE/src/main.rs" >"$o/build_$2.log" 2>&1
        if [ ! -x "$o/bin_$2" ]; then
            if grep -q "unexpectedly panicked" "$o/build_$2.log"; then
                echo "  $2: ICE -> $(grep -m1 -A1 'panicked at' "$o/build_$2.log" | tr '\n' ' ')"
            else
                echo "  $2: build failed -> $(grep -m1 '^error' "$o/build_$2.log")"
            fi
            return 1
        fi
        qemu-aarch64 "$o/bin_$2"
    }

    llvm_out=$(build_and_run rustc llvm); llvm_rc=$?
    clif_out=$(build_and_run "$RUSTC_CLIF" clif); clif_rc=$?
    echo "  LLVM   : rc=$llvm_rc out=[$llvm_out]"
    echo "  cg_clif: rc=$clif_rc out=[$clif_out]"
    if [ $llvm_rc -ne 0 ]; then echo "  BASELINE BROKEN for $target"; fail=1; continue; fi
    if [ "$llvm_out" != "$EXPECTED" ]; then echo "  UNEXPECTED LLVM VALUE"; fail=1; fi
    if [ $clif_rc -ne 0 ]; then
        echo "  => MISMATCH: cg_clif could not produce a runnable binary"; fail=1
    elif [ "$llvm_out" != "$clif_out" ]; then
        echo "  => MISMATCH: different run-time output"; fail=1
    else
        echo "  => MATCH"
    fi
done

echo
if [ $fail -ne 0 ]; then
    echo "RESULT: BUG PRESENT (cg_clif ICEd / disagreed with LLVM)"
    exit 1
fi
echo "RESULT: OK (cg_clif matches LLVM on every target above)"
