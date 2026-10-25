"""Category: nested loops with labeled break/continue over mutable state.

Targets control-flow-merging passes (SimplifyCfg / EarlyOtherwiseBranch /
JumpThreading) mishandling a labeled `break`/`continue` out of a nested loop
while mutable accumulator state is live (the family behind rust-lang/rust
#163311: "nested loop/labeled-block control flow with mutable state"). The
accumulator type itself is randomized across the full int-type pool to also
exercise the checked/wrapping-arithmetic surface inside the loop body.

UB-safety argument: `state` is a plain local updated only through
`wrapping_add`/`wrapping_mul` (always defined, no overflow panic possible);
loop bounds `n`/`m` are small fixed `i64` values with no arithmetic that can
overflow `i64`; the `as` casts are numeric truncating casts (always defined
in Rust, never UB).
"""

from __future__ import annotations

from . import register
from .common import Program, ident, pick_int_type, rand_value, wrap


@register("nested_loop_mutable_state")
def generate(rng, idx) -> Program:
    ty_tuple = pick_int_type(rng)
    ty, lo, hi = ty_tuple

    n = rng.randint(3, 8)
    m = rng.randint(3, 8)
    total = n * m

    acc_start = rand_value(rng, ty_tuple)
    delta_cont = rand_value(rng, ty_tuple)
    delta_break = rand_value(rng, ty_tuple)
    delta_step = rand_value(rng, ty_tuple)

    # ~1/3 chance each of: never fires, fires at a random in-range idx.
    def pick_trigger():
        r = rng.random()
        if r < 0.34:
            return -1
        return rng.randrange(total)

    continue_at = pick_trigger()
    break_at = pick_trigger()

    source = f"""#![allow(dead_code)]

fn main() {{
    let n: i64 = std::hint::black_box({n});
    let m: i64 = std::hint::black_box({m});
    let mut state: {ty} = {acc_start};
    let mut count: i64 = 0;
    'outer: for i in 0..n {{
        for j in 0..m {{
            let idx = i * m + j;
            if idx == {continue_at} {{
                state = state.wrapping_add({delta_cont});
                continue 'outer;
            }}
            if idx == {break_at} {{
                state = state.wrapping_add({delta_break});
                break 'outer;
            }}
            state = state.wrapping_add(({delta_step}{ty}).wrapping_mul((j + 1) as {ty}));
            count += 1;
        }}
    }}
    println!("{{}} {{}}", state, count);
}}
"""

    state = acc_start
    count = 0
    broke = False
    for i in range(n):
        for j in range(m):
            iidx = i * m + j
            if iidx == continue_at:
                state = wrap(state + delta_cont, ty_tuple)
                break
            if iidx == break_at:
                state = wrap(state + delta_break, ty_tuple)
                broke = True
                break
            state = wrap(state + wrap(delta_step * (j + 1), ty_tuple), ty_tuple)
            count += 1
        if broke:
            break

    expected_stdout = f"{state} {count}\n"

    return Program(
        name=f"nested_loop_mutable_state_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="nested_loop_mutable_state",
    )
