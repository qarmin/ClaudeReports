#!/usr/bin/env bash
# Sweep all 32 x86 float-compare predicate immediates (0x00..0x1f) through
# _mm_cmp_ps / _mm_cmp_pd and diff cg_clif against LLVM one immediate at a time.
# One file per immediate is required: an ICE in the backend kills the whole crate,
# so a single program containing all 32 would only ever show the first failure.
#
# usage: [CLIF_ROOT=<worktree>] bash cmp_imm_sweep.sh [outdir]
set -u
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="${1:-$ROOT/tmp/cmp_sweep}"
mkdir -p "$OUT"
pass=0; ice=0; wrong=0
for fn in ps pd; do
  if [ "$fn" = ps ]; then st=f32; n=4; set_fn=_mm_set_ps; store=_mm_storeu_ps; else st=f64; n=2; set_fn=_mm_set_pd; store=_mm_storeu_pd; fi
  for i in $(seq 0 31); do
    hex=$(printf '0x%02x' "$i")
    f="$OUT/cmp_${fn}_${hex}.rs"
    cat > "$f" <<EOF
use std::arch::x86_64::*;
fn main() {
    unsafe {
        // covers ==, <, >, unordered(NaN) and -0.0/0.0 so every predicate differs somewhere
        let cases: [([$st; $n], [$st; $n]); 4] = [
            ([1.0, 2.0, 3.0, 4.0][..$n].try_into().unwrap(), [1.0, 5.0, 0.0, 4.0][..$n].try_into().unwrap()),
            ([$st::NAN, 1.0, $st::NAN, 0.0][..$n].try_into().unwrap(), [1.0, $st::NAN, $st::NAN, 0.0][..$n].try_into().unwrap()),
            ([-0.0, 0.0, $st::INFINITY, -$st::INFINITY][..$n].try_into().unwrap(), [0.0, -0.0, $st::INFINITY, $st::INFINITY][..$n].try_into().unwrap()),
            ([1e300 as $st, -1.0, 0.5, -0.5][..$n].try_into().unwrap(), [1.0, -2.0, 0.5, 0.5][..$n].try_into().unwrap()),
        ];
        for (a, b) in cases {
            let va = ${set_fn}(a[$n - 1], a[$n - 2], $( [ "$fn" = ps ] && echo 'a[1], a[0],' ));
            let vb = ${set_fn}(b[$n - 1], b[$n - 2], $( [ "$fn" = ps ] && echo 'b[1], b[0],' ));
            let r = _mm_cmp_${fn}::<$hex>(va, vb);
            let mut o = [0 as $st; $n];
            ${store}(o.as_mut_ptr(), r);
            println!("{:?} {:?} -> {:?}", a, b, o.map(|v| v.to_bits()));
        }
    }
}
EOF
    log="$OUT/cmp_${fn}_${hex}.log"
    bash "$ROOT/___BUGHUNT___/tools/diffcheck.sh" "$f" > "$log" 2>&1
    if grep -q 'COMPILE FAILED' "$log"; then
      if grep -q "panicked at src/intrinsics" "$log"; then
        echo "ICE     _mm_cmp_${fn}::<$hex>  $(grep -m1 -oP 'panicked at \K.*' "$log")"
      else
        echo "NOBUILD _mm_cmp_${fn}::<$hex>"
      fi
      ice=$((ice + 1))
    elif grep -q 'OUTPUT DIFFERS\|=== DIFF' "$log" || ! grep -q 'outputs match' "$log"; then
      echo "WRONG   _mm_cmp_${fn}::<$hex>"; wrong=$((wrong + 1))
    else
      pass=$((pass + 1))
    fi
  done
done
echo "---- pass=$pass ice/nobuild=$ice wrong=$wrong  (logs in $OUT)"
