#!/usr/bin/env bash
# LLVM-vs-cg_clif differential repro for the target-spec-cpu ICE in `build_isa` (src/lib.rs).
#
#   CLIF_ROOT=<a built rustc_codegen_cranelift checkout> bash run.sh
#
# CLIF_ROOT defaults to ../clone (what the candidate justfile creates). It must already have been
# built with ./y.sh build, so that $CLIF_ROOT/dist/{rustc-clif,cargo-clif} exist.
#
# Needs the rust-src component (there is no rustup rust-std for x86_64-unknown-trusty nor for the
# custom specs, so `core` is compiled from source). No emulator: every binary built here is
# x86_64 and is executed natively.
#
# Exit 1 = the bug is present (cg_clif ICEd or disagreed with LLVM). Exit 0 = cg_clif matches LLVM.
set -uo pipefail
ulimit -c 0
# The ICE this reproduces makes rustc dump a rustc-ice-*.txt next to the cwd on every invocation.
export RUSTC_ICE=0

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF_ROOT="${CLIF_ROOT:-$HERE/../clone}"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
CARGO_CLIF="$CLIF_ROOT/dist/cargo-clif"
WORK="$HERE/work"
EXPECTED="7890598189674834828"

for tool in rustc cargo; do
    command -v "$tool" >/dev/null || { echo "MISSING TOOL: $tool"; exit 2; }
done
[ -x "$RUSTC_CLIF" ] || { echo "MISSING: $RUSTC_CLIF (run ./y.sh build in $CLIF_ROOT)"; exit 2; }

SYSROOT="$(rustc --print sysroot)"
CORE_SRC="$SYSROOT/lib/rustlib/src/rust/library/core/src/lib.rs"
[ -f "$CORE_SRC" ] || { echo "MISSING: rust-src component ($CORE_SRC)"; exit 2; }

rm -rf "$WORK"; mkdir -p "$WORK"
fail=0

echo "=============================================================="
echo "PART 1 - in-tree target: cargo build --target x86_64-unknown-trusty -Zbuild-std=core"
echo "         (the only rustc x86 target that leaves target-spec cpu at rustc's default"
echo "          \"generic\"; nothing unusual on the command line)"
echo "=============================================================="
echo "--- plain cargo (LLVM) ---"
CARGO_TARGET_DIR="$WORK/t_llvm" cargo build -Zbuild-std=core \
    --target x86_64-unknown-trusty --offline --manifest-path "$HERE/trusty_lib/Cargo.toml" \
    >"$WORK/cargo_llvm.log" 2>&1
if [ $? -eq 0 ]; then
    echo "built OK"
else
    echo "plain cargo FAILED (see $WORK/cargo_llvm.log):"; tail -20 "$WORK/cargo_llvm.log"; fail=1
fi

echo "--- cargo-clif (cg_clif) ---"
CARGO_TARGET_DIR="$WORK/t_clif" "$CARGO_CLIF" build -Zbuild-std=core \
    --target x86_64-unknown-trusty --offline --manifest-path "$HERE/trusty_lib/Cargo.toml" \
    >"$WORK/cargo_clif.log" 2>&1
if grep -q 'BadName' "$WORK/cargo_clif.log"; then
    echo "cg_clif ICEd:"
    grep -m2 -E "panicked at|Err. value" "$WORK/cargo_clif.log"
    fail=1
else
    echo "no BadName ICE from cargo-clif. Remaining diagnostics (if any):"
    grep -m6 -E "^error|panicked at" "$WORK/cargo_clif.log" || echo "  (none - build succeeded)"
fi

echo
echo "=============================================================="
echo "PART 2 - build + link + RUN a real x86_64 binary, LLVM vs cg_clif, for custom target specs"
echo "         spec/x86_64-nocpu-none.json  : no \"cpu\" key at all -> rustc defaults to \"generic\""
echo "         spec/x86_64-znver5-none.json : \"cpu\": \"znver5\", an llvm cpu Cranelift lacks"
echo "         core, compiler_builtins and the program are all compiled by the backend under test,"
echo "         then linked into a static freestanding ELF and executed natively."
echo "=============================================================="
for spec in x86_64-nocpu-none x86_64-znver5-none; do
    T="$HERE/spec/$spec.json"
    echo "----- $spec -----"
    o="$WORK/$spec"; mkdir -p "$o"

    build_and_run() { # $1 = rustc binary, $2 = tag
        local rc="$1" tag="$2"
        "$rc" -Zunstable-options --edition 2024 --crate-type lib --crate-name core --target "$T" \
            -C opt-level=0 -o "$o/libcore_$tag.rlib" "$CORE_SRC" >"$o/core_$tag.log" 2>&1 || {
            echo "  $tag: could not build core -> $(grep -m1 -E '^error|panicked at' "$o/core_$tag.log")"
            return 1; }
        "$rc" -Zunstable-options --edition 2024 --crate-type lib --crate-name compiler_builtins \
            --target "$T" -C opt-level=0 -C panic=abort --extern core="$o/libcore_$tag.rlib" \
            -A stable_features -o "$o/libcb_$tag.rlib" "$HERE/cb_stub.rs" \
            >"$o/cb_$tag.log" 2>&1 || {
            echo "  $tag: could not build compiler_builtins stub -> $(grep -m1 -E '^error|panicked at' "$o/cb_$tag.log")"
            return 1; }
        "$rc" -Zunstable-options --edition 2024 --crate-type bin --target "$T" \
            -C opt-level=0 -C panic=abort --extern core="$o/libcore_$tag.rlib" \
            --extern compiler_builtins="$o/libcb_$tag.rlib" -C link-arg=-e_start \
            -o "$o/bin_$tag" "$HERE/src/main.rs" >"$o/build_$tag.log" 2>&1
        if [ ! -x "$o/bin_$tag" ]; then
            if grep -q "unexpectedly panicked" "$o/build_$tag.log"; then
                echo "  $tag: ICE -> $(grep -m1 -A1 'panicked at' "$o/build_$tag.log" | tr '\n' ' ')"
            else
                echo "  $tag: build failed -> $(grep -m1 '^error' "$o/build_$tag.log")"
            fi
            return 1
        fi
        "$o/bin_$tag"
    }

    llvm_out=$(build_and_run rustc llvm); llvm_rc=$?
    clif_out=$(build_and_run "$RUSTC_CLIF" clif); clif_rc=$?
    echo "  LLVM   : rc=$llvm_rc out=[$llvm_out]"
    echo "  cg_clif: rc=$clif_rc out=[$clif_out]"
    if [ $llvm_rc -ne 0 ]; then echo "  BASELINE BROKEN for $spec"; fail=1; continue; fi
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
echo "RESULT: OK (cg_clif matches LLVM for every target spec above)"
