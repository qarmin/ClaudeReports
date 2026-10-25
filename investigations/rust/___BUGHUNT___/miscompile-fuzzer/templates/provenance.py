"""Category: pointer provenance round-trips and pointer identity.

Targets the exposed-provenance / int2ptr-ptr2int optimizer interactions
behind rust-lang/rust#107975, #130388, #147538, #160267, #160722, #162078,
#163541: GVN/SCCP/instcombine treating pointer identity or exposed-provenance
round-trips as foldable when they must stay opaque to such folding.

UB-safety argument:
  - All pointers are derived from `&arr[i]` for `i` within the array's bounds,
    or `base.add(k)` for `k` within `[0, len)` (defined: stays within the same
    allocation, one-past-end never dereferenced).
  - Every `expose_provenance` round-trip reconstructs a pointer for the exact
    address it was just exposed for (the documented-safe pattern from
    `std::ptr::with_exposed_provenance`'s own docs), with no arithmetic on the
    exposed integer in between.
  - Pointer equality comparisons never rely on address reuse after free: both
    sides are always pointers into the same still-live array.
  - Two array indices used in an inequality check are chosen distinct, so the
    "must compare unequal" expectation is a hard guarantee (two elements of a
    live array can never share an address), not a probabilistic one.
"""

from __future__ import annotations

from . import register
from .common import Program, ident, pick_int_type, rand_value, rust_bool, wrap


@register("pointer_provenance_roundtrip")
def generate(rng, idx) -> Program:
    elem_type = pick_int_type(rng)
    n = rng.randint(5, 10)
    vals = [rand_value(rng, elem_type) for _ in range(n)]

    i, j = rng.sample(range(n), 2)
    k = rng.randrange(n)

    arr_name = ident("ARR", idx)
    ty = elem_type[0]
    vals_src = ", ".join(f"{v}{ty}" for v in vals)

    fn_name = ident("helper", idx)

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
        rust_bool(False),
        str(vals[i]),
        str(vals[k]),
        rust_bool(True),
        rust_bool(True),
    ]
    expected.append(str(wrap(vals[0] + 1, elem_type)))

    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"pointer_provenance_roundtrip_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="pointer_provenance_roundtrip",
    )
