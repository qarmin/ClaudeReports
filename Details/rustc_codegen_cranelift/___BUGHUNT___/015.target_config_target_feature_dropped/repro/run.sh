#!/usr/bin/env bash
# Standalone reproduction: `-C target-feature=+...` is silently dropped by cg_clif.
#
#   CLIF_ROOT=<a built rustc_codegen_cranelift checkout> bash run.sh
#
# Builds this crate twice with the REAL toolchains - plain `cargo` (LLVM = the oracle) and
# `$CLIF_ROOT/dist/cargo-clif` - at opt-level 0 with the exact same RUSTFLAGS, runs the resulting
# binaries, and diffs their output.
#
# Exit 0 = both binaries behave identically under both backends.
# Exit 1 = `cfgpath` disagrees: `-C target-feature` was dropped from `cfg(target_feature)`, so the
#          generated program took a different branch than LLVM's. This is what patch.diff fixes.
# Exit 2 = only `abi` disagrees. That facet is a Cranelift capability gap (no >128-bit vector
#          type), NOT fixed by patch.diff - see RESULT.md. Exit 2 is the expected post-patch state.
set -uo pipefail
ulimit -c 0

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF_ROOT="${CLIF_ROOT:?set CLIF_ROOT to a built rustc_codegen_cranelift checkout}"
CARGO_CLIF="$CLIF_ROOT/dist/cargo-clif"
[ -x "$CARGO_CLIF" ] || { echo "missing $CARGO_CLIF - run ./y.sh build in $CLIF_ROOT" >&2; exit 2; }

FLAGS='-Ctarget-feature=+avx2,+fma,+bmi2,+popcnt'
ABI_FLAGS='-Ctarget-feature=+avx'
cd "$HERE"
rm -rf target-llvm target-clif

cfgpath_bad=0
abi_bad=0

build_and_run() {   # $1=label $2=cargo $3=target-dir $4=bin $5=rustflags
    local log="$HERE/$1.$4.log"
    if ! RUSTFLAGS="$5" CARGO_TARGET_DIR="$3" "$2" build --quiet --bin "$4" > "$log" 2>&1; then
        echo "COMPILE FAILED"
        sed -n '1,6p' "$log"
        return 101
    fi
    "$3/debug/$4"
}

for bin in cfgpath abi; do
    rf="$FLAGS"; [ "$bin" = abi ] && rf="$ABI_FLAGS"
    echo "=================== $bin   RUSTFLAGS=$rf"
    echo "--- LLVM (cargo) ---"
    llvm="$(build_and_run llvm cargo "$HERE/target-llvm" "$bin" "$rf" 2>&1)"; echo "$llvm"
    echo "--- cg_clif (cargo-clif) ---"
    clif="$(build_and_run clif "$CARGO_CLIF" "$HERE/target-clif" "$bin" "$rf" 2>&1)"; echo "$clif"
    if [ "$llvm" = "$clif" ]; then
        echo ">>> $bin: SAME"
    else
        echo ">>> $bin: DIFFERENT"
        [ "$bin" = cfgpath ] && cfgpath_bad=1
        [ "$bin" = abi ] && abi_bad=1
    fi
done

echo
if [ "$cfgpath_bad" -ne 0 ]; then
    echo "RESULT: BUG PRESENT - cfg(target_feature) ignores -C target-feature, the generated"
    echo "        program takes a different branch than the LLVM-built one (exit 1)"
    exit 1
elif [ "$abi_bad" -ne 0 ]; then
    echo "RESULT: cfg(target_feature) now matches LLVM; only the >128-bit vector ABI facet"
    echo "        remains, which is a Cranelift capability gap, not this patch's job (exit 2)"
    exit 2
else
    echo "RESULT: no difference (exit 0)"
    exit 0
fi
