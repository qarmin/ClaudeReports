"""Category: reborrow chains through nested field projections.

Targets ReferencePropagation collapsing a reborrow-through-projection chain
incorrectly (rust-lang/rust#163777, #132898): `&mut` references are taken at
increasing depth through a nested struct, passed through helper functions
that reborrow one level further, and mutated. All borrow-checked safe Rust -
no unsafe at all in this category - so any output divergence is a pure
compiler bug, not a UB question.
"""

from __future__ import annotations

from . import register
from .common import Program, ident, pick_int_type, rand_value, wrap


def _mk_nested_struct(rng, idx, depth, leaf_type):
    """Build `depth` nested single-field structs wrapping a leaf int field."""
    names = [ident("Lvl", idx, d) for d in range(depth)]
    decls = []
    for d in range(depth):
        inner_ty = leaf_type[0] if d == 0 else names[d - 1]
        decls.append(f"struct {names[d]} {{ inner: {inner_ty}, tag: u8 }}")
    return names, decls


@register("reborrow_chain")
def generate(rng, idx) -> Program:
    depth = rng.randint(2, 5)
    leaf_type = pick_int_type(rng)
    names, decls = _mk_nested_struct(rng, idx, depth, leaf_type)

    leaf_val = rand_value(rng, leaf_type)
    tags = [rng.randint(0, 255) for _ in range(depth)]

    ctor_inner = str(leaf_val)
    for d in range(depth):
        ctor_inner = f"{names[d]} {{ inner: {ctor_inner}, tag: {tags[d]} }}"

    fn_name = ident("bump", idx)
    delta = rng.randint(1, 40)

    helper_fns = []
    cur_ty = names[-1]
    helper_fns.append(
        f"fn {fn_name}_0(v: &mut {cur_ty}) {{ {fn_name}_1(&mut v.inner); }}"
        if depth > 1
        else f"fn {fn_name}_0(v: &mut {cur_ty}) {{ v.inner = v.inner.wrapping_add({delta}); }}"
    )
    for level in range(1, depth):
        cur_ty = names[depth - 1 - level]
        is_last = level == depth - 1
        if is_last:
            helper_fns.append(f"fn {fn_name}_{level}(v: &mut {cur_ty}) {{ v.inner = v.inner.wrapping_add({delta}); }}")
        else:
            helper_fns.append(f"fn {fn_name}_{level}(v: &mut {cur_ty}) {{ {fn_name}_{level + 1}(&mut v.inner); }}")

    read_path = ".".join(["val"] + ["inner"] * depth)

    # `val` + d "inner" hops lands on the shell built at ctor depth (depth-1-d),
    # since the ctor nests from the leaf outward (Lvl0 innermost, Lvl{depth-1}
    # outermost == val's own type).
    tag_reads = "\n".join(
        f'    println!("tag{d}={{}}", {".".join(["val"] + ["inner"] * d)}.tag);' for d in range(depth)
    )

    source = f"""#![allow(dead_code)]

{chr(10).join(decls)}

{chr(10).join(helper_fns)}

fn main() {{
    let mut val = {ctor_inner};
    {fn_name}_0(&mut val);
    let leaf = std::hint::black_box({read_path});
    println!("{{}}", leaf);
{tag_reads}
}}
"""
    expected_leaf = wrap(leaf_val + delta, leaf_type)
    expected_lines = [str(expected_leaf)] + [f"tag{d}={tags[depth - 1 - d]}" for d in range(depth)]
    expected_stdout = "".join(line + "\n" for line in expected_lines)

    return Program(
        name=f"reborrow_chain_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="reborrow_chain",
    )
