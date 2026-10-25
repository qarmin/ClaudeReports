"""Category: reborrow chains through nested field projections - STATIC variant.

Same target as `../templates/reborrow.py` (ReferencePropagation collapsing a
reborrow-through-projection chain incorrectly - rust-lang/rust#163777,
#132898). All borrow-checked safe Rust, no `unsafe` at all - any output
divergence is a pure compiler bug, not a UB question.

Every program here comes from an explicitly declared combination in
SCENARIO_AXES below - no RNG.
"""

from __future__ import annotations

from templates.common import Program, ident, wrap

from . import register_static
from .common import product_scenarios

# The entire test matrix. Currently 3 * 2 = 6 programs.
SCENARIO_AXES = {
    "depth": [2, 3, 5],
    "leaf_type": [("u16", 0, 65535), ("i32", -(2**31), 2**31 - 1)],
}


def _mk_nested_struct(idx, depth, leaf_type):
    """Build `depth` nested single-field structs wrapping a leaf int field."""
    names = [ident("SLvl", idx, d) for d in range(depth)]
    decls = []
    for d in range(depth):
        inner_ty = leaf_type[0] if d == 0 else names[d - 1]
        decls.append(f"struct {names[d]} {{ inner: {inner_ty}, tag: u8 }}")
    return names, decls


def _build(scenario: dict, idx: int) -> Program:
    depth = scenario["depth"]
    leaf_type = scenario["leaf_type"]
    names, decls = _mk_nested_struct(idx, depth, leaf_type)

    leaf_val = wrap(12_345 + idx * 17, leaf_type)
    tags = [(idx * 31 + d * 7) % 256 for d in range(depth)]
    delta = 1 + (idx % 40)

    ctor_inner = str(leaf_val)
    for d in range(depth):
        ctor_inner = f"{names[d]} {{ inner: {ctor_inner}, tag: {tags[d]} }}"

    fn_name = ident("sbump", idx)

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

    # `val` + d "inner" hops lands on the shell built at ctor depth
    # (depth-1-d), since the ctor nests from the leaf outward (SLvl0
    # innermost, SLvl{depth-1} outermost == val's own type).
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
        name=f"reborrow_chain_static_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="reborrow_chain_static",
    )


@register_static("reborrow_chain_static")
def generate_all() -> list[Program]:
    scenarios = product_scenarios(SCENARIO_AXES)
    return [_build(s, idx) for idx, s in enumerate(scenarios)]
