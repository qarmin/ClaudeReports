"""Category: pointer provenance round-trips and pointer identity - STATIC variant.

Same target and UB-safety argument as `../templates/provenance.py` (the
exposed-provenance / int2ptr-ptr2int optimizer interactions behind
rust-lang/rust#107975, #130388, #147538, #160267, #160722, #162078,
#163541): every pointer is derived from `&arr[i]`/`base.add(k)` within
bounds, every `expose_provenance` round-trip reconstructs a pointer for the
exact address just exposed, and the two indices used in an inequality check
are always declared distinct so the expectation is a hard guarantee, not a
probabilistic one.

Every program here comes from an explicitly declared combination in
SCENARIO_AXES below - no RNG.
"""

from __future__ import annotations

from templates.common import Program, ident, rust_bool, wrap

from . import register_static
from .common import product_scenarios

# The entire test matrix. Currently 2 * 2 * 2 = 8 programs.
SCENARIO_AXES = {
    "elem_type": [("u32", 0, 2**32 - 1), ("i64", -(2**63), 2**63 - 1)],
    "n": [5, 10],
    "index_pattern": ["spread", "edges"],
}


def _resolve_indices(pattern: str, n: int) -> tuple[int, int, int]:
    if pattern == "spread":
        return 0, n - 1, n // 2
    if pattern == "edges":
        return 0, n - 1, 1 if n > 1 else 0
    raise ValueError(f"unknown index_pattern {pattern!r}")


def _build(scenario: dict, idx: int) -> Program:
    elem_type = scenario["elem_type"]
    n = scenario["n"]
    i, j, k = _resolve_indices(scenario["index_pattern"], n)

    vals = [wrap(100_000 + 97 * p + idx * 3, elem_type) for p in range(n)]

    arr_name = ident("SARR", idx)
    ty = elem_type[0]
    vals_src = ", ".join(f"{v}{ty}" for v in vals)
    fn_name = ident("shelper", idx)

    source = f"""#![allow(dead_code)]

fn {fn_name}(x: {ty}) -> {ty} {{ x.wrapping_add(1) }}

fn main() {{
    let {arr_name}: [{ty}; {n}] = [{vals_src}];

    let i = std::hint::black_box({i}usize);
    let j = std::hint::black_box({j}usize);
    let k = std::hint::black_box({k}usize);

    let p_i = &{arr_name}[i] as *const {ty};
    let p_j = &{arr_name}[j] as *const {ty};

    println!("{{}}", p_i == p_i);
    println!("{{}}", p_i == p_j);

    let addr = p_i.expose_provenance();
    let p_i2 = std::ptr::with_exposed_provenance::<{ty}>(addr);
    let v_roundtrip = unsafe {{ *p_i2 }};
    println!("{{}}", v_roundtrip);

    let base = {arr_name}.as_ptr();
    let p_k = unsafe {{ base.add(k) }};
    let v_k = unsafe {{ *p_k }};
    println!("{{}}", v_k);
    println!("{{}}", p_k == &{arr_name}[k] as *const {ty});

    let f1: fn({ty}) -> {ty} = {fn_name};
    let f2: fn({ty}) -> {ty} = {fn_name};
    println!("{{}}", f1 == f2);
    println!("{{}}", f1({vals[0]}{ty}));
}}
"""
    expected = [
        rust_bool(True),
        rust_bool(i == j),
        str(vals[i]),
        str(vals[k]),
        rust_bool(True),
        rust_bool(True),
        str(wrap(vals[0] + 1, elem_type)),
    ]
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"pointer_provenance_roundtrip_static_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="pointer_provenance_roundtrip_static",
    )


@register_static("pointer_provenance_roundtrip_static")
def generate_all() -> list[Program]:
    scenarios = product_scenarios(SCENARIO_AXES)
    return [_build(s, idx) for idx, s in enumerate(scenarios)]
