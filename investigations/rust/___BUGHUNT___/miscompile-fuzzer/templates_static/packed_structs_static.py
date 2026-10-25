"""Category: #[repr(packed)] structs, including const-evaluated instances -
STATIC variant.

Same target and UB-safety argument as `../templates/packed_structs.py` (GVN
folding a packed-field projection into a misaligned literal -
rust-lang/rust#163782): we never take `&s.field` on a packed field, only
`let v = s.field;` (a copy-out; always defined for Copy types regardless of
alignment). All field types are plain integers with no invalid bit patterns.

Every program here comes from an explicitly declared combination in
SCENARIO_AXES below - no RNG. Field values are computed by a fixed formula
(so they vary across the declared index space, deterministically) rather than
sampled.
"""

from __future__ import annotations

from templates.common import Program, ident, wrap

from . import register_static
from .common import product_scenarios

# The entire test matrix. Currently 3 * 2 * 2 * 2 = 24 programs.
SCENARIO_AXES = {
    "field_types": [
        (("u8", 0, 255), ("u16", 0, 65535), ("u32", 0, 2**32 - 1)),
        (("i8", -128, 127), ("i64", -(2**63), 2**63 - 1), ("usize", 0, 2**64 - 1)),
        (("u8", 0, 255), ("u8", 0, 255), ("u8", 0, 255), ("u8", 0, 255)),
    ],
    "cond1": [True, False],
    "cond2": [True, False],
    "mut_field_offset": [0, 1],
}


def _build(scenario: dict, idx: int) -> Program:
    field_types = list(scenario["field_types"])
    n_fields = len(field_types)
    field_names = [f"f{i}" for i in range(n_fields)]

    # Deterministic per-field values, distinct across fields and across idx,
    # always wrapped into the field's own representable range.
    const_vals = [wrap(1000 + 37 * i + idx * 7, t) for i, t in enumerate(field_types)]
    alt_vals = [wrap(2000 + 53 * i + idx * 11, t) for i, t in enumerate(field_types)]

    struct_name = ident("SPacked", idx)
    const_name = ident("SCONST", idx)

    fields_decl = "\n".join(f"    {n}: {t[0]}," for n, t in zip(field_names, field_types))
    const_fields = ", ".join(f"{n}: {v}" for n, v in zip(field_names, const_vals))
    alt_fields = ", ".join(f"{n}: {v}" for n, v in zip(field_names, alt_vals))

    cond1 = scenario["cond1"]
    current_vals = list(const_vals if cond1 else alt_vals)

    mut_field_idx = scenario["mut_field_offset"] % n_fields
    other_idx = (mut_field_idx + 1) % n_fields
    mut_delta = 13 + idx % 50
    other_delta = 29 + idx % 40
    cond2 = scenario["cond2"]

    if_branch_text = (
        f"        s.{field_names[mut_field_idx]} = "
        f"s.{field_names[mut_field_idx]}.wrapping_add({mut_delta});"
    )
    else_branch_text = (
        f"        s.{field_names[other_idx]} = "
        f"s.{field_names[other_idx]}.wrapping_add({other_delta});"
    )
    if cond2:
        current_vals[mut_field_idx] = wrap(current_vals[mut_field_idx] + mut_delta, field_types[mut_field_idx])
    else:
        current_vals[other_idx] = wrap(current_vals[other_idx] + other_delta, field_types[other_idx])

    copies = "\n".join(f"    let v{i} = s.{n};" for i, n in enumerate(field_names))
    print_args = ", ".join(f"v{i}" for i in range(n_fields))
    print_fmt = " ".join(["{}"] * n_fields)

    const_copy_idx = idx % n_fields

    source = f"""#![allow(dead_code)]

#[repr(packed)]
#[derive(Clone, Copy)]
struct {struct_name} {{
{fields_decl}
}}

const {const_name}: {struct_name} = {struct_name} {{ {const_fields} }};
const {const_name}_ALT: {struct_name} = {struct_name} {{ {alt_fields} }};

fn main() {{
    let mut s = if std::hint::black_box({str(cond1).lower()}) {{ {const_name} }} else {{ {const_name}_ALT }};
    if std::hint::black_box({str(cond2).lower()}) {{
{if_branch_text}
    }} else {{
{else_branch_text}
    }}
{copies}
    println!("{print_fmt}", {print_args});

    let c = {const_name}.{field_names[const_copy_idx]};
    println!("{{}}", c);
}}
"""
    expected_line1 = " ".join(str(v) for v in current_vals)
    expected_line2 = str(const_vals[const_copy_idx])
    expected_stdout = expected_line1 + "\n" + expected_line2 + "\n"

    return Program(
        name=f"packed_struct_fields_static_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="packed_struct_fields_static",
    )


@register_static("packed_struct_fields_static")
def generate_all() -> list[Program]:
    scenarios = product_scenarios(SCENARIO_AXES)
    return [_build(s, idx) for idx, s in enumerate(scenarios)]
