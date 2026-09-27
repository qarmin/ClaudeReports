#!/usr/bin/env bash
# Runs the SIMD-enabled and scalar-only builds against the same seeds through the identical
# public resize call and diffs their output. Exit code convention (matches repro_template.rs):
# 0 = did not reproduce (outputs identical for every seed - the goal after the fix),
# 1 = reproduced (at least one seed's output differs between the two builds by more than the
#     small, separately-documented +/-1..3 LSB rounding gap from a different, pre-existing issue
#     - see RESULT.md's "What's not done" - only a "large" (> 50) per-value difference counts as
#     this candidate's bug reproducing).
set -euo pipefail
cd "$(dirname "$0")"

cargo build --release --manifest-path simd/Cargo.toml
cargo build --release --manifest-path scalar/Cargo.toml

SIMD_BIN=simd/target/release/repro_simd
SCALAR_BIN=scalar/target/release/repro_scalar

reproduced=0
for seed in $(seq 1 50); do
    "$SIMD_BIN" "$seed" > "/tmp/repro007_simd_out_$seed.txt"
    "$SCALAR_BIN" "$seed" > "/tmp/repro007_scalar_out_$seed.txt"
    max_diff=$(python3 - "$seed" <<'PYEOF'
import sys
seed = sys.argv[1]
a = [int(x) for x in open(f"/tmp/repro007_simd_out_{seed}.txt")]
b = [int(x) for x in open(f"/tmp/repro007_scalar_out_{seed}.txt")]
print(max(abs(x - y) for x, y in zip(a, b)))
PYEOF
)
    if [ "$max_diff" -gt 50 ]; then
        echo "seed $seed: max per-value diff = $max_diff (> 50 - candidate 007's bug)"
        reproduced=$((reproduced + 1))
    fi
done

if [ "$reproduced" -eq 0 ]; then
    echo "did not reproduce - no seed showed a large (>50) SIMD-vs-scalar divergence across all 50 seeds"
    exit 0
else
    echo "REPRODUCED - $reproduced/50 seeds show a large SIMD-vs-scalar divergence"
    exit 1
fi
