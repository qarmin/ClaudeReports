"""Category: match/if-trees with shared conditions and aliasing locals.

Targets EarlyOtherwiseBranch and JumpThreading miscompiling matches that
re-derive the same discriminant/condition more than once (rust-lang/rust
#159618, #163311, and the broader EarlyOtherwiseBranch bug family): a tuple
match on `(a, b)` checking "same variant" is the textbook EarlyOtherwiseBranch
shape (it factors out a shared discriminant read), and two separate matches
over the same scrutinee with the same range boundaries is the textbook
JumpThreading shape (it tries to reuse one branch's condition result for the
other). All plain integers/enums, no unsafe.
"""

from __future__ import annotations

from . import register
from .common import Program, ident, rand_value


@register("match_shared_condition")
def generate(rng, idx) -> Program:
    nv = rng.randint(3, 5)
    enum_name = ident("E", idx)
    variants = [f"V{v}" for v in range(nv)]
    payload_ty = ("i32", -(2**31), 2**31 - 1)
    payloads_a = [rand_value(rng, payload_ty) for _ in range(nv)]
    payloads_b = [rand_value(rng, payload_ty) for _ in range(nv)]

    variant_decl = ", ".join(f"{v}(i32)" for v in variants)

    classify_arms = "\n".join(
        f"        {enum_name}::{v}(x) => *x + {k},"  for k, v in enumerate(variants)
    )
    classify_fn = ident("classify", idx)

    same_arms = "\n".join(
        f"        ({enum_name}::{v}(_), {enum_name}::{v}(_)) => true," for v in variants
    )
    same_fn = ident("same_variant", idx)

    va_idx = rng.randrange(nv)
    vb_idx = rng.randrange(nv)
    vc_idx = rng.randrange(nv)

    a_expr = f"{enum_name}::{variants[va_idx]}({payloads_a[va_idx]})"
    b_expr = f"{enum_name}::{variants[vb_idx]}({payloads_b[vb_idx]})"
    c_expr = f"{enum_name}::{variants[vc_idx]}({payloads_a[vc_idx]})"

    v_val = rng.randint(0, 100)
    bp1 = rng.randint(1, 40)
    bp2 = rng.randint(bp1 + 1, 90)

    range_fn_tag = ident("tag_of", idx)
    range_fn_code = ident("code_of", idx)

    source = f"""#![allow(dead_code)]

enum {enum_name} {{ {variant_decl} }}

fn {classify_fn}(e: &{enum_name}) -> i32 {{
    match e {{
{classify_arms}
    }}
}}

fn {same_fn}(a: &{enum_name}, b: &{enum_name}) -> bool {{
    match (a, b) {{
{same_arms}
        _ => false,
    }}
}}

fn {range_fn_tag}(v: i32) -> &'static str {{
    match v {{
        0..={bp1} => "a",
        {bp1 + 1}..={bp2} => "b",
        _ => "c",
    }}
}}

fn {range_fn_code}(v: i32) -> i32 {{
    match v {{
        0..={bp1} => 100,
        {bp1 + 1}..={bp2} => 200,
        _ => 300,
    }}
}}

fn main() {{
    let a = {a_expr};
    let b = {b_expr};
    let c = {c_expr};

    println!("{{}}", {classify_fn}(&a));
    println!("{{}}", {classify_fn}(&b));
    println!("{{}}", {classify_fn}(&c));
    println!("{{}}", {same_fn}(&a, &b));
    println!("{{}}", {same_fn}(&a, &c));

    let v = std::hint::black_box({v_val});
    println!("{{}}", {range_fn_tag}(v));
    println!("{{}}", {range_fn_code}(v));
}}
"""

    def classify(payload, variant_idx):
        return payload + variant_idx

    def bucket_tag(v):
        if v <= bp1:
            return "a"
        if v <= bp2:
            return "b"
        return "c"

    def bucket_code(v):
        if v <= bp1:
            return 100
        if v <= bp2:
            return 200
        return 300

    expected = [
        str(classify(payloads_a[va_idx], va_idx)),
        str(classify(payloads_b[vb_idx], vb_idx)),
        str(classify(payloads_a[vc_idx], vc_idx)),
        "true" if va_idx == vb_idx else "false",
        "true" if va_idx == vc_idx else "false",
        bucket_tag(v_val),
        str(bucket_code(v_val)),
    ]
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"match_shared_condition_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="match_shared_condition",
    )
