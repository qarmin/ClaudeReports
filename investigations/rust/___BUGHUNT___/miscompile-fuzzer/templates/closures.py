"""Category: closures capturing by mutable reference vs by move, threaded
across #[inline(always)]/#[inline(never)] boundaries.

Targets inlining decisions losing track of a closure's capture-by-reference
side effects (stage 1: a non-`move` closure that mutates a captured local
through `&mut` on every call) vs. capture-by-value semantics (stage 2: a
`move` closure that captured a *snapshot* `i64` - further mutation of the
source variable afterward must not be visible to it) vs. dynamic dispatch
through `Box<dyn FnMut(i64) -> i64>` (stage 3). Each stage is driven through
one of two higher-order wrapper functions chosen at random per stage - one
`#[inline(always)]`, one `#[inline(never)]` - both of which forward to the
same shared `apply_n` loop, so a correct compilation must produce identical
results regardless of which wrapper (and thus how much gets inlined) is used.

UB-safety argument: every captured value is a plain owned `i64` (`Copy`, so
"captured by move" never leaves a moved-from-and-then-used local behind) or a
`&mut i64` borrow of a local that outlives every call through it (the
borrow's scope is exactly `main`'s body, the closures never outlive it, and
`i64` is a type with no validity invariants beyond "some bit pattern", so
there is no reference/uninitialized-read hazard). All arithmetic uses
`wrapping_*`, so no step can overflow-panic. No `unsafe`.
"""

from __future__ import annotations

from . import register
from .common import INT_TYPES, Program, ident, wrap

I64 = next(t for t in INT_TYPES if t[0] == "i64")

FORMULAS2 = {
    "mul2_add_cap": ("x.wrapping_mul(2).wrapping_add({cap})", lambda x, cap: wrap(wrap(x * 2, I64) + cap, I64)),
    "sub_cap": ("x.wrapping_sub({cap})", lambda x, cap: wrap(x - cap, I64)),
    "cap_sub_x": ("({cap}).wrapping_sub(x)", lambda x, cap: wrap(cap - x, I64)),
}


def _rand_i64(rng, lo=-(10**9), hi=10**9):
    return rng.randint(lo, hi)


@register("closure_capture_boundary")
def generate(rng, idx) -> Program:
    apply_n = ident("apply_n", idx)
    apply_n_a = ident("apply_n_a", idx)
    apply_n_b = ident("apply_n_b", idx)

    # Randomly assign which of the two wrapper fns is #[inline(always)] vs
    # #[inline(never)]; stages below pick between apply_n_a/apply_n_b by name,
    # so both attribute assignments get exercised across the corpus.
    if rng.random() < 0.5:
        attr_a, attr_b = "#[inline(always)]", "#[inline(never)]"
    else:
        attr_a, attr_b = "#[inline(never)]", "#[inline(always)]"

    captured_val = _rand_i64(rng)
    n1 = rng.randint(3, 8)
    start1 = _rand_i64(rng, -1000, 1000)
    wrapper1 = rng.choice([apply_n_a, apply_n_b])

    formula2_key = rng.choice(list(FORMULAS2))
    n2 = rng.randint(3, 8)
    start2 = _rand_i64(rng, -1000, 1000)
    wrapper2 = rng.choice([apply_n_a, apply_n_b])

    formula3_key = rng.choice(list(FORMULAS2))
    n3 = rng.randint(3, 8)
    start3 = _rand_i64(rng, -1000, 1000)
    wrapper3 = rng.choice([apply_n_a, apply_n_b])

    f2_rust, f2_py = FORMULAS2[formula2_key]
    f3_rust, f3_py = FORMULAS2[formula3_key]

    source = f"""#![allow(dead_code)]

fn {apply_n}<F: FnMut(i64) -> i64>(mut f: F, n: i64, start: i64) -> i64 {{
    let mut acc = start;
    for i in 0..n {{
        acc = f(acc.wrapping_add(i));
    }}
    acc
}}

{attr_a}
fn {apply_n_a}<F: FnMut(i64) -> i64>(f: F, n: i64, start: i64) -> i64 {{
    {apply_n}(f, n, start)
}}

{attr_b}
fn {apply_n_b}<F: FnMut(i64) -> i64>(f: F, n: i64, start: i64) -> i64 {{
    {apply_n}(f, n, start)
}}

fn main() {{
    let mut counter: i64 = 0;
    let captured_val: i64 = {captured_val};

    let mut by_ref_closure = |x: i64| {{
        counter = counter.wrapping_add(1);
        x.wrapping_add(captured_val).wrapping_add(counter)
    }};
    let result1 = {wrapper1}(&mut by_ref_closure, {n1}, {start1});
    println!("{{}} {{}}", result1, counter);

    let move_val = counter;
    let moved_closure = move |x: i64| {f2_rust.format(cap="move_val")};
    let result2 = {wrapper2}(moved_closure, {n2}, {start2});
    println!("{{}}", result2);

    let mut boxed: Box<dyn FnMut(i64) -> i64> = Box::new(move |x| {f3_rust.format(cap="captured_val")});
    let result3 = {wrapper3}(&mut *boxed, {n3}, {start3});
    println!("{{}}", result3);
}}
"""

    # --- Python oracle, mirroring the Rust control flow exactly ---
    counter = 0

    def by_ref(x):
        nonlocal counter
        counter = wrap(counter + 1, I64)
        return wrap(wrap(x + captured_val, I64) + counter, I64)

    def run_apply_n(f, n, start):
        acc = start
        for i in range(n):
            acc = f(wrap(acc + i, I64))
        return acc

    result1 = run_apply_n(by_ref, n1, start1)
    counter_after1 = counter

    move_val = counter_after1
    result2 = run_apply_n(lambda x: f2_py(x, move_val), n2, start2)

    result3 = run_apply_n(lambda x: f3_py(x, captured_val), n3, start3)

    expected = [
        f"{result1} {counter_after1}",
        str(result2),
        str(result3),
    ]
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"closure_capture_boundary_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="closure_capture_boundary",
    )
