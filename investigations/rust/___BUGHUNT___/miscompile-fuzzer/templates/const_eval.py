"""Category: const-eval of nontrivial constant expressions.

Targets agreement between the const-evaluator (which builds `TABLE` and
`SUM_FIRST_HALF` at compile time, inside the interpreter CTFE uses) and
regular codegen executing the *exact same* function (`step`) at runtime - a
divergence between the two paths (e.g. a const-propagation/GVN pass folding a
`TABLE[idx]` read into a wrong literal, or disagreeing with what the
identical runtime loop computes) is a real bug no matter which side is wrong.
`step` and `sum_range` are plain (non-unsafe) `const fn`s using a `while`
loop and an `if`/`else` branch, called both in a `const` context to build
`TABLE`/`SUM_FIRST_HALF` and in a normal runtime loop in `main`.

UB-safety argument: every arithmetic step goes through `wrapping_*`, so no
`const fn` evaluation can ever hit a const-eval-time overflow panic/error
regardless of the random seed/multiplier chosen. `TABLE[idx]` and
`arr[i]`/`arr[lo..hi]` indices are always taken modulo `N` or bounded by a
`while i < N` condition, so every array access (const and runtime) is
in-bounds by construction. No `unsafe`, no pointers, no uninitialized memory.
"""

from __future__ import annotations

from . import register
from .common import INT_TYPES, Program, ident, wrap

I64 = next(t for t in INT_TYPES if t[0] == "i64")


def _step(x: int, k: int) -> int:
    if k % 2 == 0:
        return wrap(wrap(x * wrap(k + 3, I64), I64) + 11, I64)
    else:
        return wrap(wrap(x + wrap(k * 5, I64), I64) - 2, I64)


@register("const_eval_array_ops")
def generate(rng, idx) -> Program:
    n = rng.randint(4, 10)
    seed = rng.randint(-(2**40), 2**40)
    runtime_idx_seed = rng.randint(0, 10_000)

    step_fn = ident("step", idx)
    build_fn = ident("build", idx)
    sum_fn = ident("sum_range", idx)
    n_const = ident("N", idx).upper()
    seed_const = ident("SEED", idx).upper()
    table_const = ident("TABLE", idx).upper()
    sum_const = ident("SUM_FIRST_HALF", idx).upper()

    source = f"""#![allow(dead_code)]

const fn {step_fn}(x: i64, k: i64) -> i64 {{
    if k % 2 == 0 {{
        x.wrapping_mul(k.wrapping_add(3)).wrapping_add(11)
    }} else {{
        x.wrapping_add(k.wrapping_mul(5)).wrapping_sub(2)
    }}
}}

const {n_const}: usize = {n};
const {seed_const}: i64 = {seed};

const fn {build_fn}() -> [i64; {n_const}] {{
    let mut arr = [0i64; {n_const}];
    let mut i = 0usize;
    let mut acc = {seed_const};
    while i < {n_const} {{
        acc = {step_fn}(acc, i as i64);
        arr[i] = acc;
        i += 1;
    }}
    arr
}}

const fn {sum_fn}(arr: &[i64; {n_const}], lo: usize, hi: usize) -> i64 {{
    let mut i = lo;
    let mut s: i64 = 0;
    while i < hi {{
        s = s.wrapping_add(arr[i]);
        i += 1;
    }}
    s
}}

const {table_const}: [i64; {n_const}] = {build_fn}();
const {sum_const}: i64 = {sum_fn}(&{table_const}, 0, {n_const} / 2);

fn main() {{
    let idx = std::hint::black_box({runtime_idx_seed}usize) % {n_const};
    println!("{{}}", {table_const}[idx]);
    println!("{{}}", {sum_const});

    let r = &{table_const}[idx];
    println!("{{}}", *r);

    let mut acc = {seed_const};
    for i in 0..=idx {{
        acc = {step_fn}(acc, i as i64);
    }}
    println!("{{}}", acc);
}}
"""

    runtime_idx = runtime_idx_seed % n

    table = []
    acc = seed
    for i in range(n):
        acc = _step(acc, i)
        table.append(acc)

    sum_first_half = 0
    for i in range(n // 2):
        sum_first_half = wrap(sum_first_half + table[i], I64)

    acc2 = seed
    for i in range(runtime_idx + 1):
        acc2 = _step(acc2, i)

    expected = [
        str(table[runtime_idx]),
        str(sum_first_half),
        str(table[runtime_idx]),
        str(acc2),
    ]
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"const_eval_array_ops_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="const_eval_array_ops",
    )
