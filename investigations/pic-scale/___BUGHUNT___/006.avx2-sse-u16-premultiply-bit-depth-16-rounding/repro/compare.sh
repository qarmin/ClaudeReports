#!/usr/bin/env bash
# Runs the SIMD-enabled and scalar-only builds against the same seeds through the identical
# public resize call and diffs their output. Exit code convention (matches repro_template.rs):
# 0 = did not reproduce (outputs identical for every seed - the goal after the fix),
# 1 = reproduced (at least one seed's output differs between the two builds).
set -euo pipefail
cd "$(dirname "$0")"

cargo build --release --manifest-path simd/Cargo.toml
cargo build --release --manifest-path scalar/Cargo.toml

SIMD_BIN=simd/target/release/repro_simd
SCALAR_BIN=scalar/target/release/repro_scalar

mismatches=0
for seed in $(seq 1 50); do
    "$SIMD_BIN" "$seed" > /tmp/repro_simd_out_$seed.txt
    "$SCALAR_BIN" "$seed" > /tmp/repro_scalar_out_$seed.txt
    if ! diff -q /tmp/repro_simd_out_$seed.txt /tmp/repro_scalar_out_$seed.txt > /dev/null; then
        n=$(diff /tmp/repro_simd_out_$seed.txt /tmp/repro_scalar_out_$seed.txt | grep -c '^<' || true)
        echo "seed $seed: $n of 1024 output u16 values differ between SIMD-enabled and scalar-only builds"
        mismatches=$((mismatches + 1))
    fi
done

if [ "$mismatches" -eq 0 ]; then
    echo "did not reproduce - SIMD-enabled and scalar-only builds agree bit-for-bit across all 50 seeds"
    exit 0
else
    echo "REPRODUCED - $mismatches/50 seeds disagree between the SIMD-enabled and scalar-only builds"
    exit 1
fi
