#!/usr/bin/env bash
# Drives the three real rustc flag combinations that reach src/back/write.rs:47-70's
# `if lto_supported { ... }` sub-branches, through the real, unmodified `rustc` + this backend
# (via the project's own `./y.sh rustc` wrapper), 3 times each. Run from inside a checkout with
# probe.diff already applied and `./y.sh build --release` already done (see ../justfile).
#
# This is a negative-result repro: candidate 013 asks whether this code crashes the way candidate
# 001's write.rs:122 does. Every run below is expected to exit 0 (no crash) - that IS the
# reproduction of "Not reproducible", not an error in this script.
set -uo pipefail
script_dir="$(cd "$(dirname "$0")" && pwd)"
src="$script_dir/main.rs"
out_dir=$(mktemp -d)
cd "$script_dir/../clone"
fail=0

echo "=== (a) config.emit_bc branch: --emit=llvm-bc ==="
for i in 1 2 3; do
    CHANNEL=release ./y.sh rustc "$src" --emit=llvm-bc -o "$out_dir/a_$i" 2>&1 | tail -5
    ec=${PIPESTATUS[0]}
    echo "run $i exit=$ec"
    [ "$ec" -eq 0 ] || fail=1
done

echo "=== (b) EmitObj::Bitcode branch: -C linker-plugin-lto --emit=obj ==="
for i in 1 2 3; do
    CHANNEL=release ./y.sh rustc "$src" -C linker-plugin-lto --emit=obj -o "$out_dir/b_$i" 2>&1 | tail -5
    ec=${PIPESTATUS[0]}
    echo "run $i exit=$ec"
    [ "$ec" -eq 0 ] || fail=1
done

echo "=== (c) EmitObj::ObjectCode(BitcodeSection::Full) branch: --crate-type=rlib --emit=obj,link -C embed-bitcode=yes ==="
for i in 1 2 3; do
    CHANNEL=release ./y.sh rustc "$src" --crate-type=rlib --emit=obj,link -C embed-bitcode=yes -o "$out_dir/c_$i" 2>&1 | tail -5
    ec=${PIPESTATUS[0]}
    echo "run $i exit=$ec"
    [ "$ec" -eq 0 ] || fail=1
done

rm -rf "$out_dir"

if [ "$fail" -eq 0 ]; then
    echo "RESULT: NOT REPRODUCIBLE - all 9 runs across the 3 guarded sub-branches exited 0 (no crash)."
else
    echo "RESULT: a run exited non-zero / crashed - see output above (this would mean the bug DID reproduce)."
fi
exit "$fail"
