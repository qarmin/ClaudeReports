#!/usr/bin/env bash
# Generate + run the f16/f128 cast sweep used by candidate 013 through diffcheck.sh.
#
#   bash ___BUGHUNT___/tools/f16_f128_cast_sweep.sh                 # main tree's dist/ (unpatched)
#   CLIF_ROOT=tmp/wt/013.foo bash ___BUGHUNT___/tools/f16_f128_cast_sweep.sh   # a patched worktree
#
# 90 single-file programs, all at opt-level 0 (see README's MIR-GVN gotcha). Covers:
#   * identity simd_cast/simd_as on f16 and f128 at lane counts 1/2/4/8/16 - the ICE
#   * widening/narrowing f16<->f32, f16<->f128, f64<->f128 SIMD casts - controls
#   * scalar `x as f16` / `x as f128` identity
#   * the full 10-int x 4-float scalar cast matrix with 0/1/MIN/MAX/NaN/inf/subnormal values
# Prints one line per program that disagrees with LLVM, then a TOTAL_DIFFERENT count.
set -u
ROOT=${ROOT:-$(git rev-parse --show-toplevel)}
cd "$ROOT"
DIR=${DIR:-tmp/f16_f128_cast_sweep}
rm -rf "$DIR"
python3 ___BUGHUNT___/tools/f16_f128_cast_sweep_gen.py "$DIR/simd" >/dev/null
python3 ___BUGHUNT___/tools/scalar_cast_matrix_gen.py "$DIR/scalar" >/dev/null
bad=0
total=0
for f in "$DIR"/simd/*.rs "$DIR"/scalar/*.rs; do
    total=$((total + 1))
    out=$(bash ___BUGHUNT___/tools/diffcheck.sh "$f" 2>&1) || {
        ice=$(printf '%s' "$out" | grep -oP 'panicked at \Ksrc/[^ ]*' | head -1)
        printf 'DIFF %-42s %s %s\n' \
            "$(basename "$f" .rs)" "$(printf '%s' "$out" | grep -m1 '^RESULT:')" "$ice"
        bad=$((bad + 1))
    }
done
rm -f rustc-ice-*.txt
echo "TOTAL_DIFFERENT=$bad / $total"
