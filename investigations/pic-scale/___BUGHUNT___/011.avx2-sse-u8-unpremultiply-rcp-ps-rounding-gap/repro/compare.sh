#!/usr/bin/env bash
# Runs the AVX2-enabled, SSE-only-enabled, and scalar-only builds against the same seeds through
# the identical public resize call and diffs their output, for both WorkloadStrategy variants.
# A mismatch counts as this candidate's bug if it has the bug's signature: the SIMD build's color
# channel differs from the scalar reference's color channel by exactly 1 (never more), while the
# alpha channel (untouched by unpremultiply) is bit-identical between builds.
# Exit code convention: 0 = did not reproduce, 1 = reproduced.
set -euo pipefail
cd "$(dirname "$0")"

cargo build --release --manifest-path simd/Cargo.toml
cargo build --release --manifest-path sse/Cargo.toml
cargo build --release --manifest-path scalar/Cargo.toml

SIMD_BIN=simd/target/release/repro_simd
SSE_BIN=sse/target/release/repro_sse
SCALAR_BIN=scalar/target/release/repro_scalar

reproduced=0
total=0
for strategy in speed quality; do
    for seed in 1 2 3 42 100; do
        total=$((total + 1))
        "$SIMD_BIN" "$seed" "$strategy" > "/tmp/repro011_simd_${strategy}_$seed.txt"
        "$SSE_BIN" "$seed" "$strategy" > "/tmp/repro011_sse_${strategy}_$seed.txt"
        "$SCALAR_BIN" "$seed" "$strategy" > "/tmp/repro011_scalar_${strategy}_$seed.txt"
        result=$(python3 - "$strategy" "$seed" <<'PYEOF'
import sys
strategy, seed = sys.argv[1], sys.argv[2]
simd = [int(x) for x in open(f"/tmp/repro011_simd_{strategy}_{seed}.txt")]
sse = [int(x) for x in open(f"/tmp/repro011_sse_{strategy}_{seed}.txt")]
scalar = [int(x) for x in open(f"/tmp/repro011_scalar_{strategy}_{seed}.txt")]

def signature_hits(candidate):
    hits = []
    maxdiff = 0
    alpha_mismatches = 0
    for i, (c, s) in enumerate(zip(candidate, scalar)):
        if i % 4 == 3:
            if c != s:
                alpha_mismatches += 1
            continue
        d = abs(c - s)
        if d > 0:
            hits.append((i, c, s))
            maxdiff = max(maxdiff, d)
    return hits, maxdiff, alpha_mismatches

simd_hits, simd_maxdiff, simd_alpha_mm = signature_hits(simd)
sse_hits, sse_maxdiff, sse_alpha_mm = signature_hits(sse)
print(f"strategy={strategy} seed={seed}: AVX2 color-mismatches={len(simd_hits)} maxdiff={simd_maxdiff} alpha_mismatches={simd_alpha_mm} example={simd_hits[:2]} | SSE color-mismatches={len(sse_hits)} maxdiff={sse_maxdiff} alpha_mismatches={sse_alpha_mm} example={sse_hits[:2]}")
is_signature = (simd_maxdiff == 1 and simd_alpha_mm == 0 and len(simd_hits) > 0) or (sse_maxdiff == 1 and sse_alpha_mm == 0 and len(sse_hits) > 0)
print("REPRODUCED" if is_signature else "clean")
PYEOF
)
        echo "$result"
        if echo "$result" | grep -q "REPRODUCED"; then
            reproduced=$((reproduced + 1))
        fi
    done
done

if [ "$reproduced" -eq 0 ]; then
    echo "did not reproduce - no (strategy, seed) combination showed the rcp_ps-rounding-gap signature (SIMD color channel off by exactly 1 from scalar, alpha channel untouched)"
    exit 0
else
    echo "REPRODUCED - $reproduced/$total (strategy, seed) combinations show the rcp_ps-rounding-gap signature"
    exit 1
fi
