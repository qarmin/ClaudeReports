"""Category: distinct statics/consts with identical content.

Targets GVN/LLVM assuming distinct globals can be merged or aliased
(rust-lang/rust#162078, #163541): Rust guarantees every `static` item has an
address distinct from every other `static` item, even when their contents are
bit-for-bit identical (unlike plain C globals, which a linker may legally
fold). We declare several statics with identical values and assert pairwise
address inequality, plus read through pointers to confirm content is still
correct after any such (legal-for-C, illegal-for-Rust) folding attempt.

UB-safety argument: taking `&STATIC` and comparing/dereferencing it is always
defined - statics are always live for 'static and fully initialized at
program start.
"""

from __future__ import annotations

from . import register
from .common import Program, ident, pick_int_type, rand_value


@register("static_merge_packed_const")
def generate(rng, idx) -> Program:
    elem_type = pick_int_type(rng)
    ty = elem_type[0]
    shared_value = rand_value(rng, elem_type)

    n_statics = rng.randint(3, 5)
    names = [ident("S", idx, k) for k in range(n_statics)]

    decls = "\n".join(f"static {n}: {ty} = {shared_value};" for n in names)

    pairs = []
    for a in range(n_statics):
        for b in range(a + 1, n_statics):
            pairs.append((a, b))

    ineq_prints = "\n".join(
        f'    println!("{{}}", (&{names[a]} as *const {ty}) != (&{names[b]} as *const {ty}));'
        for a, b in pairs
    )
    value_prints = "\n".join(f'    println!("{{}}", {n});' for n in names)

    struct_name = ident("Packed", idx)
    packed_field_ty = pick_int_type(rng)
    pty = packed_field_ty[0]
    pval_a = rand_value(rng, packed_field_ty)
    pval_b = rand_value(rng, packed_field_ty)

    source = f"""#![allow(dead_code)]

{decls}

#[repr(packed)]
#[derive(Clone, Copy)]
struct {struct_name} {{
    a: {pty},
    b: {pty},
}}

const C1_{idx}: {struct_name} = {struct_name} {{ a: {pval_a}, b: {pval_b} }};
const C2_{idx}: {struct_name} = {struct_name} {{ a: {pval_a}, b: {pval_b} }};

fn main() {{
{ineq_prints}
{value_prints}

    // NOTE: we deliberately do NOT compare `&C1_{idx} != &C2_{idx}` here.
    // Unlike `static` items, two `const` items are not guaranteed distinct
    // addresses - the compiler may legally share backing storage for
    // bit-identical consts, so that comparison has no fixed expected value
    // and would be a harness bug (false "divergence"), not a real one. We
    // only check that reading through each const's field still yields the
    // right value no matter how its storage was folded.
    let ca = C1_{idx}.a;
    let cb = C2_{idx}.b;
    println!("{{}} {{}}", ca, cb);
}}
"""

    expected = ["true" for _ in pairs] + [str(shared_value) for _ in names]
    expected.append(f"{pval_a} {pval_b}")
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"static_merge_packed_const_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="static_merge_packed_const",
    )
