"""Category: checked/wrapping/saturating arithmetic across inlining boundaries.

Targets overflow-checked-arithmetic codegen interacting badly with inlining
decisions (the #149294/#155952 "store-to-load forwarding at -O1" family, and
general SROA/instcombine-vs-checked-arith bugs): a chain of helper functions,
alternately marked `#[inline(always)]` and `#[inline(never)]`, each performing
one `wrapping_*` / `checked_*` / `saturating_*` step, threading an accumulator
through all of them.

UB-safety argument: we never use a plain `+`/`-`/`*` that could panic or wrap
implicitly - every step uses an explicit checked/wrapping/saturating method,
so behavior is fully specified by the method's documented semantics
regardless of `-C overflow-checks` (which the diff runner pins identically
across configs anyway; see `../diff_runner.py`).
"""

from __future__ import annotations

from . import register
from .common import Program, ident, pick_int_type, rand_value, wrap

OPS = ["wrapping_add", "wrapping_sub", "wrapping_mul", "checked_add", "checked_sub", "saturating_add", "saturating_mul"]


def _apply(op, acc, x, ty_tuple, fallback):
    _, lo, hi = ty_tuple
    if op == "wrapping_add":
        return wrap(acc + x, ty_tuple)
    if op == "wrapping_sub":
        return wrap(acc - x, ty_tuple)
    if op == "wrapping_mul":
        return wrap(acc * x, ty_tuple)
    if op == "checked_add":
        raw = acc + x
        return raw if lo <= raw <= hi else fallback
    if op == "checked_sub":
        raw = acc - x
        return raw if lo <= raw <= hi else fallback
    if op == "saturating_add":
        return max(lo, min(hi, acc + x))
    if op == "saturating_mul":
        return max(lo, min(hi, acc * x))
    raise ValueError(op)


@register("checked_wrapping_arithmetic")
def generate(rng, idx) -> Program:
    ty_tuple = pick_int_type(rng)
    ty, lo, hi = ty_tuple
    acc = rand_value(rng, ty_tuple)
    fallback = rand_value(rng, ty_tuple)

    n_ops = rng.randint(4, 8)
    chosen_ops = [rng.choice(OPS) for _ in range(n_ops)]
    operands = [rand_value(rng, ty_tuple) for _ in range(n_ops)]
    inline_attrs = [rng.choice(["#[inline(always)]", "#[inline(never)]", "#[inline]"]) for _ in range(n_ops)]

    fns = []
    for k, op in enumerate(chosen_ops):
        fn_name = ident("step", idx, k)
        if op.startswith("checked_"):
            body = f"a.{op}(x).unwrap_or({fallback})"
        else:
            body = f"a.{op}(x)"
        fns.append(f"{inline_attrs[k]}\nfn {fn_name}(a: {ty}, x: {ty}) -> {ty} {{ {body} }}")

    call_chain = "\n".join(
        f"    acc = {ident('step', idx, k)}(acc, {operands[k]});" for k in range(n_ops)
    )

    source = f"""#![allow(dead_code)]

{chr(10).join(fns)}

fn main() {{
    let mut acc: {ty} = {acc};
{call_chain}
    println!("{{}}", acc);
    println!("{{}} {{}}", {ty}::MIN, {ty}::MAX);
}}
"""

    expected_acc = acc
    for k, op in enumerate(chosen_ops):
        expected_acc = _apply(op, expected_acc, operands[k], ty_tuple, fallback)

    expected = [str(expected_acc), f"{lo} {hi}"]
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"checked_wrapping_arithmetic_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="checked_wrapping_arithmetic",
    )
