#!/usr/bin/env bash
# Runs the AVX2-enabled, SSE-only-enabled, and scalar-only builds against the same seeds through
# the identical public resize call and diffs their output. A mismatch counts as this candidate's
# bug only if it has the bug's specific signature: SIMD output is exactly 0 where the scalar
# reference is exactly 65535 (max_colors) AND the pixel's alpha channel (unaffected by
# unpremultiply, so identical in both outputs) is nonzero - this isolates the cvtps_epi32 overflow
# from the separate, already-known, still-unpatched-in-this-tree "zero out color on alpha==0"
# bug (candidate 007), which would otherwise produce superficially similar-looking mismatches.
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
for seed in 765 815; do
    "$SIMD_BIN" "$seed" > "/tmp/repro008_simd_$seed.txt"
    "$SSE_BIN" "$seed" > "/tmp/repro008_sse_$seed.txt"
    "$SCALAR_BIN" "$seed" > "/tmp/repro008_scalar_$seed.txt"
    result=$(python3 - "$seed" <<'PYEOF'
import sys
seed = sys.argv[1]
simd = [int(x) for x in open(f"/tmp/repro008_simd_{seed}.txt")]
sse = [int(x) for x in open(f"/tmp/repro008_sse_{seed}.txt")]
scalar = [int(x) for x in open(f"/tmp/repro008_scalar_{seed}.txt")]

def overflow_hits(candidate):
    hits = []
    for i, (c, s) in enumerate(zip(candidate, scalar)):
        if c == 0 and s == 65535:
            pixel = i // 4
            alpha = scalar[pixel * 4 + 3]
            if alpha != 0:
                hits.append((i, alpha))
    return hits

simd_hits = overflow_hits(simd)
sse_hits = overflow_hits(sse)
print(f"seed {seed}: AVX2 overflow-signature hits={len(simd_hits)} {simd_hits[:3]} | SSE overflow-signature hits={len(sse_hits)} {sse_hits[:3]}")
print("REPRODUCED" if (simd_hits or sse_hits) else "clean")
PYEOF
)
    echo "$result"
    if echo "$result" | grep -q "REPRODUCED"; then
        reproduced=$((reproduced + 1))
    fi
done

if [ "$reproduced" -eq 0 ]; then
    echo "did not reproduce - no seed showed the cvtps_epi32-overflow signature (SIMD=0 vs scalar=65535 at a nonzero-alpha pixel) in either backend"
    exit 0
else
    echo "REPRODUCED - $reproduced/2 seeds show the cvtps_epi32-overflow signature"
    exit 1
fi
