"""Category: niche-filling enum layout through generic instantiation.

Targets niche-optimization layout correctness specifically when the
niche-bearing payload type is a *generic parameter* instantiated with
different concrete niche types (`NonZeroU32`, `NonZeroU16`, `NonZeroU8`,
`&'static i64`), including generic enums nested two and three deep
(`Holder<Holder<T>>`, `Option<Holder<T>>`) and a two-parameter generic enum
(`Either<A, B>`) where both parameters independently carry a niche. This is
a different failure surface from `enum_niche_layout` (which only ever
matches on concrete, non-generic enum definitions written directly in the
template): here the compiler must derive each layout from a *monomorphized*
type parameter, so a layout bug could show up only for specific
instantiations of the generic definition, not for the generic body in the
abstract - and the nesting forces it to compose that derivation across two
independently-compiled generic types (`std::option::Option` and this
module's own `Holder`).

UB-freedom, by construction: every value is built through safe constructors
(`NonZeroU32::new(..).unwrap()`, `Some`/`None`, `Holder::Empty`/`Filled`,
plain struct-free enum literals) - never a transmute or raw bit pattern.
Every reference used is a borrow of a `static` with `'static` lifetime, so
dereferencing it is always valid for as long as the program runs. Every
`match` is exhaustive over the enum's actual declared variants (no `_`
fallback hiding an unreachable case). No `unsafe`, no raw pointers, no
uninitialized memory.
"""

from __future__ import annotations

from . import register
from .common import Program, ident, rust_bool


@register("niche_generic_layout")
def generate(rng, idx) -> Program:
    holder = ident("Holder", idx)
    either = ident("Either", idx)
    ref_static = ident("REF_VAL", idx).upper()

    present1 = rng.random() < 0.5
    v1 = rng.randint(1, 1000)

    selector2 = rng.choice([0, 1, 2])
    v2 = rng.randint(1, 60000)

    selector3 = rng.choice([0, 1, 2])
    ref_val = rng.randint(-1_000_000, 1_000_000)

    use_left4 = rng.random() < 0.5
    v4 = rng.randint(1, 255)
    flag4 = rng.random() < 0.5

    source = f"""#![allow(dead_code)]

use std::num::{{NonZeroU8, NonZeroU16, NonZeroU32}};

enum {holder}<T> {{
    Empty,
    Filled(T),
}}

enum {either}<A, B> {{
    Left(A),
    Right(B),
}}

static {ref_static}: i64 = {ref_val};

fn main() {{
    let h1: {holder}<NonZeroU32> = if std::hint::black_box({rust_bool(present1)}) {{
        {holder}::Filled(NonZeroU32::new({v1}).unwrap())
    }} else {{
        {holder}::Empty
    }};
    let code1: i64 = match h1 {{
        {holder}::Empty => -1,
        {holder}::Filled(n) => n.get() as i64,
    }};
    println!("{{}}", code1);

    let selector2 = std::hint::black_box({selector2}u8) % 3;
    let h2: {holder}<{holder}<NonZeroU16>> = match selector2 {{
        0 => {holder}::Empty,
        1 => {holder}::Filled({holder}::Empty),
        _ => {holder}::Filled({holder}::Filled(NonZeroU16::new({v2}).unwrap())),
    }};
    let code2: i64 = match h2 {{
        {holder}::Empty => -1,
        {holder}::Filled({holder}::Empty) => -2,
        {holder}::Filled({holder}::Filled(n)) => n.get() as i64,
    }};
    println!("{{}}", code2);

    let selector3 = std::hint::black_box({selector3}u8) % 3;
    let o3: Option<{holder}<&'static i64>> = match selector3 {{
        0 => None,
        1 => Some({holder}::Empty),
        _ => Some({holder}::Filled(&{ref_static})),
    }};
    let code3: i64 = match o3 {{
        None => -1,
        Some({holder}::Empty) => -2,
        Some({holder}::Filled(r)) => *r,
    }};
    println!("{{}}", code3);

    let e4: {either}<NonZeroU8, bool> = if std::hint::black_box({rust_bool(use_left4)}) {{
        {either}::Left(NonZeroU8::new({v4}).unwrap())
    }} else {{
        {either}::Right({rust_bool(flag4)})
    }};
    let code4: i64 = match e4 {{
        {either}::Left(n) => n.get() as i64,
        {either}::Right(b) => if b {{ 1000 }} else {{ 2000 }},
    }};
    println!("{{}}", code4);
}}
"""

    code1 = v1 if present1 else -1
    code2 = {0: -1, 1: -2, 2: v2}[selector2]
    code3 = {0: -1, 1: -2, 2: ref_val}[selector3]
    code4 = v4 if use_left4 else (1000 if flag4 else 2000)

    expected = [str(code1), str(code2), str(code3), str(code4)]
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"niche_generic_layout_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="niche_generic_layout",
    )
