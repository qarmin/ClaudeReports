#!/usr/bin/env bash
# Compile-only sweep: for each target, build `core` and then the REAL `compiler_builtins` crate
# from rust-src, once with plain rustc (LLVM, the oracle) and once with rustc-clif.
#
#   CLIF_ROOT=<built cg_clif checkout> bash sweep_targets.sh
#
# Prints one line per target: <target> llvm=<OK|FAIL> clif=<OK|FAIL[:reason]>.
# The point is to check the *whole bug class* (every CmpResult != host CmpResult target), not just
# the one triple the main repro runs, and to check that the pointer-sized targets don't regress.
set -uo pipefail
ulimit -c 0

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF_ROOT="${CLIF_ROOT:-$HERE/../clone}"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
WORK="$HERE/sweep_work"
[ -x "$RUSTC_CLIF" ] || { echo "MISSING: $RUSTC_CLIF"; exit 2; }

SYSROOT="$(rustc --print sysroot)"
SRCROOT="$SYSROOT/lib/rustlib/src/rust/library"
CORE_SRC="$SRCROOT/core/src/lib.rs"
CB_SRC="$SRCROOT/compiler-builtins/compiler-builtins/src/lib.rs"

TARGETS="${TARGETS:-aarch64-unknown-linux-gnu aarch64-unknown-linux-musl aarch64-apple-darwin \
aarch64-linux-android s390x-unknown-linux-gnu x86_64-unknown-linux-gnu \
riscv64gc-unknown-linux-gnu}"

CB_CFGS=(--cfg f16_enabled --cfg f128_enabled
         --cfg 'feature="compiler-builtins"' --cfg 'feature="arch"'
         --cfg 'feature="mem"' --cfg 'feature="unmangled-names"')

rm -rf "$WORK"; mkdir -p "$WORK"
for target in $TARGETS; do
    o="$WORK/$target"; mkdir -p "$o"
    if ! rustc --edition 2024 --crate-type lib --crate-name core --target "$target" \
        -C opt-level=0 -C panic=abort -o "$o/libcore.rlib" "$CORE_SRC" >"$o/core.log" 2>&1; then
        echo "$target  core-build-FAILED (skipped)"; continue
    fi
    res=""
    for tag in llvm clif; do
        if [ "$tag" = llvm ]; then rc=rustc; else rc="$RUSTC_CLIF"; fi
        if "$rc" --edition 2024 --crate-type lib --crate-name compiler_builtins --target "$target" \
            -C opt-level=0 -C panic=abort --extern core="$o/libcore.rlib" "${CB_CFGS[@]}" \
            -A stable_features -o "$o/libcb_$tag.rlib" "$CB_SRC" >"$o/cb_$tag.log" 2>&1; then
            res="$res $tag=OK"
        else
            why=$(grep -m1 -oP "attempt to declare .\K__\w+" "$o/cb_$tag.log")
            [ -z "$why" ] && why=$(grep -m1 -oP "^error: \K.{0,60}" "$o/cb_$tag.log")
            res="$res $tag=FAIL:${why:-unknown}"
        fi
    done
    echo "$target $res"
done
