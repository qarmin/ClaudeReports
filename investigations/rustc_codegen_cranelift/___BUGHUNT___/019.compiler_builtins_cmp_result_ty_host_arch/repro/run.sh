#!/usr/bin/env bash
# LLVM-vs-cg_clif differential repro for `CMP_RESULT_TY` being picked from the HOST's
# cfg(target_arch) instead of the target being compiled for.
#
#   CLIF_ROOT=<a built rustc_codegen_cranelift checkout> bash run.sh
#
# CLIF_ROOT defaults to ../clone (what the candidate justfile creates). It must already have been
# built with ./y.sh build so that $CLIF_ROOT/dist/rustc-clif exists.
#
# Host requirements: an x86_64 (or any non-aarch64) host, the pinned nightly with the `rust-src`
# component (there is no rustup rust-std for the cross target, so `core` and `compiler_builtins`
# are built from source), and `qemu-aarch64` (user-mode emulation) to actually RUN the aarch64
# binary that comes out.
#
# Three parts, all real compiler invocations:
#   1. cg_clif compiles the REAL `compiler_builtins` crate (rust-src) for aarch64 -> hard error,
#      while plain rustc compiles it fine.
#   2. A real aarch64 `no_std` program doing real f128 comparisons is built by rustc and by
#      rustc-clif, linked static, and EXECUTED under qemu-aarch64; the printed comparison
#      bitmasks are diffed. cg_clif's binary answers `a > b` where `a < b` is true.
#   3. (only possible once the bug is fixed) the same program, but linked against the
#      `compiler_builtins` that CG_CLIF ITSELF produced for aarch64.
set -uo pipefail
ulimit -c 0

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF_ROOT="${CLIF_ROOT:-$HERE/../clone}"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
WORK="$HERE/work"
TARGET="${TARGET:-aarch64-unknown-linux-gnu}"

for tool in qemu-aarch64 rustc; do
    command -v "$tool" >/dev/null || { echo "MISSING TOOL: $tool"; exit 2; }
done
[ -x "$RUSTC_CLIF" ] || { echo "MISSING: $RUSTC_CLIF (run ./y.sh build in $CLIF_ROOT)"; exit 2; }

HOST_ARCH="$(rustc -vV | sed -n 's/^host: \([^-]*\).*/\1/p')"
case "$HOST_ARCH" in
    aarch64) echo "This repro needs a non-aarch64 host (the bug is host-vs-target confusion)."; exit 2;;
esac

SYSROOT="$(rustc --print sysroot)"
LLD="$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld"
SRCROOT="$SYSROOT/lib/rustlib/src/rust/library"
CORE_SRC="$SRCROOT/core/src/lib.rs"
CB_SRC="$SRCROOT/compiler-builtins/compiler-builtins/src/lib.rs"
[ -x "$LLD" ] || { echo "MISSING: rust-lld at $LLD"; exit 2; }
[ -f "$CORE_SRC" ] || { echo "MISSING: rust-src component ($CORE_SRC)"; exit 2; }
[ -f "$CB_SRC" ] || { echo "MISSING: rust-src component ($CB_SRC)"; exit 2; }

rm -rf "$WORK"; mkdir -p "$WORK"
fail=0

# rust-lld wants its flavor as the first argument, so wrap it for -Clinker=.
printf '#!/bin/sh\nexec %s -flavor gnu "$@"\n' "$LLD" > "$WORK/ld_wrapper.sh"
chmod +x "$WORK/ld_wrapper.sh"

CB_CFGS=(--cfg f16_enabled --cfg f128_enabled
         --cfg 'feature="compiler-builtins"' --cfg 'feature="arch"'
         --cfg 'feature="mem"' --cfg 'feature="unmangled-names"')

echo "=============================================================="
echo "PART 0 - build core for $TARGET with plain rustc (LLVM), the shared baseline"
echo "=============================================================="
rustc --edition 2024 --crate-type lib --crate-name core --target "$TARGET" \
    -C opt-level=0 -C panic=abort -o "$WORK/libcore.rlib" "$CORE_SRC" \
    >"$WORK/core.log" 2>&1 || { echo "could not build core:"; tail -20 "$WORK/core.log"; exit 2; }
echo "libcore.rlib: $(stat -c%s "$WORK/libcore.rlib") bytes"

echo
echo "=============================================================="
echo "PART 1 - compile the REAL compiler_builtins crate for $TARGET"
echo "=============================================================="
build_cb() { # $1 = rustc binary, $2 = tag
    "$1" --edition 2024 --crate-type lib --crate-name compiler_builtins --target "$TARGET" \
        -C opt-level=0 -C panic=abort --extern core="$WORK/libcore.rlib" \
        "${CB_CFGS[@]}" -A stable_features \
        -o "$WORK/libcompiler_builtins_$2.rlib" "$CB_SRC" >"$WORK/cb_$2.log" 2>&1
}
echo "--- plain rustc (LLVM) ---"
if build_cb rustc llvm; then
    echo "OK: $(stat -c%s "$WORK/libcompiler_builtins_llvm.rlib") bytes"
else
    echo "BASELINE BROKEN - plain rustc could not build compiler_builtins:"
    tail -20 "$WORK/cb_llvm.log"; exit 2
fi
echo "--- rustc-clif (cg_clif) ---"
if build_cb "$RUSTC_CLIF" clif; then
    echo "OK: $(stat -c%s "$WORK/libcompiler_builtins_clif.rlib") bytes"
else
    echo "cg_clif REFUSED to compile compiler_builtins:"
    grep -m3 -E "^error" "$WORK/cb_clif.log"
    fail=1
fi

echo
echo "=============================================================="
echo "PART 2 - build + link + RUN the f128-comparison program, LLVM vs cg_clif"
echo "         (callee side = the LLVM-built compiler_builtins, i.e. what a"
echo "          normal cross build gets out of the prebuilt sysroot)"
echo "=============================================================="
build_and_run() { # $1 = rustc binary, $2 = tag, $3 = compiler_builtins rlib
    "$1" --edition 2024 --crate-type bin --target "$TARGET" -C opt-level=0 -C panic=abort \
        --extern core="$WORK/libcore.rlib" --extern compiler_builtins="$3" \
        -C linker="$WORK/ld_wrapper.sh" -C linker-flavor=ld \
        -C link-arg=-static -C link-arg=-e_start -C default-linker-libraries=no \
        -o "$WORK/bin_$2" "$HERE/src/main.rs" >"$WORK/build_$2.log" 2>&1
    if [ ! -x "$WORK/bin_$2" ]; then
        if grep -q "unexpectedly panicked" "$WORK/build_$2.log"; then
            echo "  $2: ICE -> $(grep -m1 -A1 'panicked at' "$WORK/build_$2.log" | tr '\n' ' ')" >&2
        else
            echo "  $2: build failed -> $(grep -m1 '^error' "$WORK/build_$2.log")" >&2
        fi
        return 1
    fi
    qemu-aarch64 "$WORK/bin_$2" >"$WORK/out_$2.txt" 2>&1
}

build_and_run rustc llvm "$WORK/libcompiler_builtins_llvm.rlib"; llvm_rc=$?
build_and_run "$RUSTC_CLIF" clif "$WORK/libcompiler_builtins_llvm.rlib"; clif_rc=$?
echo "  LLVM   : rc=$llvm_rc, $(wc -l <"$WORK/out_llvm.txt" 2>/dev/null || echo 0) lines"
echo "  cg_clif: rc=$clif_rc, $(wc -l <"$WORK/out_clif.txt" 2>/dev/null || echo 0) lines"
if [ $llvm_rc -ne 0 ]; then
    echo "  BASELINE BROKEN"; exit 2
fi
if ! diff -q "$HERE/expected.txt" "$WORK/out_llvm.txt" >/dev/null; then
    echo "  LLVM output does not match the checked-in expected.txt - baseline changed?"
    diff "$HERE/expected.txt" "$WORK/out_llvm.txt" | head -10; fail=1
fi
if [ $clif_rc -ne 0 ]; then
    echo "  => MISMATCH: cg_clif produced no runnable binary"; fail=1
elif ! diff -q "$WORK/out_llvm.txt" "$WORK/out_clif.txt" >/dev/null; then
    echo "  => MISMATCH: $(diff "$WORK/out_llvm.txt" "$WORK/out_clif.txt" | grep -c '^<') of"\
         "$(wc -l <"$WORK/out_llvm.txt") comparison lines differ. First few:"
    diff "$WORK/out_llvm.txt" "$WORK/out_clif.txt" | head -9
    echo "  (mask bits: 1=eq 2=ne 4=lt 8=le 16=gt 32=ge; 14 = 'a<b', 50 = 'a>b')"
    fail=1
else
    echo "  => MATCH"
fi

echo
echo "=============================================================="
echo "PART 3 - same program, but the callee compiler_builtins is the one CG_CLIF built"
echo "         (only reachable once PART 1 succeeds)"
echo "=============================================================="
if [ -f "$WORK/libcompiler_builtins_clif.rlib" ]; then
    build_and_run "$RUSTC_CLIF" clif2 "$WORK/libcompiler_builtins_clif.rlib"; clif2_rc=$?
    if [ $clif2_rc -ne 0 ]; then
        echo "  => cg_clif produced no runnable binary"; fail=1
    elif ! diff -q "$WORK/out_llvm.txt" "$WORK/out_clif2.txt" >/dev/null; then
        echo "  => MISMATCH against LLVM:"
        diff "$WORK/out_llvm.txt" "$WORK/out_clif2.txt" | head -9; fail=1
    else
        echo "  => MATCH (all-cg_clif aarch64 f128 comparison agrees with LLVM)"
    fi
else
    echo "  skipped: cg_clif could not build compiler_builtins (PART 1 failed)"
fi

echo
if [ $fail -ne 0 ]; then
    echo "RESULT: BUG PRESENT (cg_clif refused compiler_builtins and/or disagreed with LLVM)"
    exit 1
fi
echo "RESULT: OK (cg_clif builds compiler_builtins for aarch64 and matches LLVM at run time)"
