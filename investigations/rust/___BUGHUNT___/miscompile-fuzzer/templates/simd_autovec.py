"""Category: SIMD/autovectorization-safe elementwise array loops.

Targets whether LLVM's loop vectorizer (LoopVectorize/SLP) produces output
identical to scalar execution for exactly the shapes it's designed to speed
up: fixed-size integer-array elementwise ops (`wrapping_add`, bitwise `^`,
`saturating_add`) plus a branchless select (`if a[i] > b[i] { a[i] } else
{ b[i] }`, the canonical `vpmax`/blend candidate) and a horizontal wrapping-sum
reduction of each result array. `O0` never vectorizes; `O3` typically does for
array lengths like these (8/16/32/64), so a mismatch between the two is a
vectorizer correctness bug, not an arithmetic-order artifact - every
reduction here only ever uses `wrapping_add`, which is associative and
commutative over any fixed-width two's-complement type, so a vectorizer
reassociating/reordering the horizontal sum can never legitimately change
the final wrapped bit pattern.

UB-freedom, by construction: every index used is a literal `0..N` loop over
`[T; N]` arrays of that exact declared length, so every access is in-bounds
by construction - no index is ever computed from random/unchecked input.
All arithmetic goes through `wrapping_*`/`saturating_*`, never a bare
operator (bitwise `^` can't overflow), so no step can panic or invoke UB
regardless of which random values were drawn. The two input arrays are
read through a single `std::hint::black_box` each at the point they're
produced, so the compiler can't constant-fold the whole computation away at
compile time, while the loop bodies themselves are left untouched so the
vectorizer still sees normal, vectorizable loops to optimize. No `unsafe`,
no pointers, no uninitialized memory.
"""

from __future__ import annotations

from . import register
from .common import INT_TYPES, Program, wrap

# Pointer-sized types are excluded here only because this category targets
# fixed SIMD lane widths (8/16/32/64-bit elements), not because usize/isize
# would be unsafe to use - every other helper in this module works the same
# for them.
_VECTORIZABLE_TYPES = [t for t in INT_TYPES if t[0] not in ("usize", "isize")]

_LENGTHS = [8, 16, 32, 64]


def _rand_vals(rng, n, ty):
    _, lo, hi = ty
    return [rng.randint(lo, hi) for _ in range(n)]


@register("simd_autovec_loops")
def generate(rng, idx) -> Program:
    ty = rng.choice(_VECTORIZABLE_TYPES)
    ty_name, lo, hi = ty
    n = rng.choice(_LENGTHS)
    mid = n // 2

    a_vals = _rand_vals(rng, n, ty)
    b_vals = _rand_vals(rng, n, ty)

    a_lit = ", ".join(f"{v}{ty_name}" for v in a_vals)
    b_lit = ", ".join(f"{v}{ty_name}" for v in b_vals)

    source = f"""#![allow(dead_code)]

fn main() {{
    let a: [{ty_name}; {n}] = std::hint::black_box([{a_lit}]);
    let b: [{ty_name}; {n}] = std::hint::black_box([{b_lit}]);

    let mut c = [0{ty_name}; {n}];
    let mut d = [0{ty_name}; {n}];
    let mut e = [0{ty_name}; {n}];
    let mut f = [0{ty_name}; {n}];

    for i in 0..{n} {{
        c[i] = a[i].wrapping_add(b[i]);
        d[i] = a[i] ^ b[i];
        e[i] = a[i].saturating_add(b[i]);
        f[i] = if a[i] > b[i] {{ a[i] }} else {{ b[i] }};
    }}

    let mut sum_c: {ty_name} = 0;
    let mut sum_d: {ty_name} = 0;
    let mut sum_f: {ty_name} = 0;
    for i in 0..{n} {{
        sum_c = sum_c.wrapping_add(c[i]);
        sum_d = sum_d.wrapping_add(d[i]);
        sum_f = sum_f.wrapping_add(f[i]);
    }}

    println!("{{}}", sum_c);
    println!("{{}}", sum_d);
    println!("{{}}", e[{mid}]);
    println!("{{}}", sum_f);
}}
"""

    c_vals = [wrap(a_vals[i] + b_vals[i], ty) for i in range(n)]
    # Bitwise xor of two values already within [lo, hi] of a fixed-width
    # two's-complement type always stays within [lo, hi] itself (it only
    # flips existing bits, never changes the bit width), so no wrap() needed.
    d_vals = [a_vals[i] ^ b_vals[i] for i in range(n)]
    e_val_mid = max(lo, min(hi, a_vals[mid] + b_vals[mid]))
    f_vals = [a_vals[i] if a_vals[i] > b_vals[i] else b_vals[i] for i in range(n)]

    sum_c = 0
    sum_d = 0
    sum_f = 0
    for i in range(n):
        sum_c = wrap(sum_c + c_vals[i], ty)
        sum_d = wrap(sum_d + d_vals[i], ty)
        sum_f = wrap(sum_f + f_vals[i], ty)

    expected = [str(sum_c), str(sum_d), str(e_val_mid), str(sum_f)]
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"simd_autovec_loops_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="simd_autovec_loops",
    )
