"""Category: unions with reference/pointer sibling fields.

Targets GVN / JumpThreading / DataflowConstProp's "aliasing flood" across
union field writes (the pattern behind rust-lang/rust#160004, #161898,
#163825): a value read through one union field must see a write performed
through a *different* (sibling) field of the same union, because they occupy
the same bytes.

UB-safety argument: both union fields (`r: &'static i64`, `p: *const i64`)
only ever hold an address obtained as `&VALS[i]` / `&VALS[i] as *const i64`
for some `i` in range, i.e. always a non-null, aligned pointer into a live
`'static` array. So no matter which field is read back, the bits are a valid
`&i64` (reference-validity requires non-null+aligned+live, all guaranteed)
or a valid `*const i64` (no validity constraints beyond being a pointer).
Dereferencing is therefore always defined, and comparing or copying the
union's current value never touches a trap representation.
"""

from __future__ import annotations

from . import register
from .common import Program, ident


@register("union_ref_fields")
def generate(rng, idx) -> Program:
    n_vals = rng.randint(4, 8)
    vals = [rng.randint(-(2**62), 2**62) for _ in range(n_vals)]

    static_name = ident("VALS", idx)
    union_name = ident("U", idx)

    n_steps = rng.randint(3, 6)
    stmts = []
    expected = []
    cur = rng.randrange(n_vals)
    stmts.append(f"    let mut u = {union_name} {{ r: &{static_name}[{cur}] }};")

    for step in range(n_steps):
        write_field = rng.choice(["r", "p"])
        read_field = rng.choice(["r", "p"])
        new_idx = rng.randrange(n_vals)
        use_branch = rng.random() < 0.5

        if use_branch:
            alt_idx = rng.randrange(n_vals)
            cond = rng.random() < 0.5
            cond_lit = "true" if cond else "false"
            chosen = new_idx if cond else alt_idx
            if write_field == "r":
                stmts.append(
                    f"    if std::hint::black_box({cond_lit}) {{ u.r = &{static_name}[{new_idx}]; }} "
                    f"else {{ u.r = &{static_name}[{alt_idx}]; }}"
                )
            else:
                stmts.append(
                    f"    if std::hint::black_box({cond_lit}) {{ u.p = &{static_name}[{new_idx}] as *const i64; }} "
                    f"else {{ u.p = &{static_name}[{alt_idx}] as *const i64; }}"
                )
            cur = chosen
        else:
            if write_field == "r":
                stmts.append(f"    u.r = &{static_name}[std::hint::black_box({new_idx}usize)];")
            else:
                stmts.append(
                    f"    u.p = &{static_name}[std::hint::black_box({new_idx}usize)] as *const i64;"
                )
            cur = new_idx

        read_var = ident("read", idx, step)
        if read_field == "r":
            stmts.append(f"    let {read_var} = unsafe {{ *u.r }};")
        else:
            stmts.append(f"    let {read_var} = unsafe {{ *u.p }};")
        stmts.append(f'    println!("{{}}", {read_var});')
        expected.append(str(vals[cur]))

    vals_src = ", ".join(f"{v}i64" for v in vals)
    source = f"""#![allow(dead_code)]

static {static_name}: [i64; {n_vals}] = [{vals_src}];

union {union_name} {{
    r: &'static i64,
    p: *const i64,
}}

fn main() {{
{chr(10).join(stmts)}
}}
"""
    expected_stdout = "".join(line + "\n" for line in expected)
    return Program(
        name=f"union_ref_fields_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="union_ref_fields",
    )
