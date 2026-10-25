"""Category: Option<T> / custom niche-layout enums.

Targets niche-optimization interactions (the discriminant is packed into an
otherwise-invalid bit pattern of the payload, e.g. `Option<&T>`'s None is the
null pointer, `Option<NonZeroU32>`'s None is 0): a wrong niche computation or
a optimization that forgets the payload/discriminant share storage can
silently corrupt either the tag or the payload. No unsafe: every value here
is built through safe constructors (`Some`/`None`, `NonZeroU32::new`), so
correctness is guaranteed by the type system - any divergence is a pure
codegen/layout bug.
"""

from __future__ import annotations

from . import register
from .common import Program, ident, rust_bool


@register("enum_niche_layout")
def generate(rng, idx) -> Program:
    v1 = rng.randint(1, 1000)
    present1 = rng.random() < 0.5

    v2 = rng.randint(1, 1000)
    present2 = rng.random() < 0.5

    nested_state = rng.choice(["none", "some_none", "some_some"])
    nested_val = rng.randint(1, 1000)
    nested_selector = {"none": 0, "some_none": 1, "some_some": 2}[nested_state]

    ref_present = rng.random() < 0.5
    ref_val = rng.randint(1, 1000)

    status_running = rng.random() < 0.5
    status_val = rng.randint(1, 1000)

    name = ident("Status", idx)

    source = f"""#![allow(dead_code)]

use std::num::NonZeroU32;

enum {name} {{
    Idle,
    Running(NonZeroU32),
}}

fn main() {{
    let o1: Option<i32> = if std::hint::black_box({rust_bool(present1)}) {{ Some({v1}) }} else {{ None }};
    println!("{{}}", o1.unwrap_or(-1));

    let o2: Option<NonZeroU32> = if std::hint::black_box({rust_bool(present2)}) {{
        NonZeroU32::new({v2})
    }} else {{
        None
    }};
    println!("{{}}", o2.map_or(0u32, |n| n.get()));

    let nested: Option<Option<u8>> = match std::hint::black_box({nested_selector}u8) {{
        0 => None,
        1 => Some(None),
        _ => Some(Some({nested_val % 256})),
    }};
    let nested_code = match nested {{
        None => -1,
        Some(None) => -2,
        Some(Some(x)) => x as i32,
    }};
    println!("{{}}", nested_code);

    let local = {ref_val};
    let oref: Option<&i32> = if std::hint::black_box({rust_bool(ref_present)}) {{ Some(&local) }} else {{ None }};
    println!("{{}}", oref.copied().unwrap_or(-1));

    let status = if std::hint::black_box({rust_bool(status_running)}) {{
        {name}::Running(NonZeroU32::new({status_val}).unwrap())
    }} else {{
        {name}::Idle
    }};
    let status_code = match status {{
        {name}::Idle => -1i64,
        {name}::Running(n) => n.get() as i64,
    }};
    println!("{{}}", status_code);
}}
"""

    expected = [
        str(v1 if present1 else -1),
        str(v2 if present2 else 0),
        {"none": "-1", "some_none": "-2", "some_some": str(nested_val % 256)}[nested_state],
        str(ref_val if ref_present else -1),
        str(status_val if status_running else -1),
    ]
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"enum_niche_layout_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="enum_niche_layout",
    )
