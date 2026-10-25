"""Category: #[repr(packed)] structs, including const-evaluated instances.

Targets GVN folding a packed-field projection into a literal with a mismatched
alignment assumption (rust-lang/rust#163782), and more generally store/load
forwarding across misaligned fields at various opt levels.

UB-safety argument: we never take `&s.field` (a real reference) to a packed
field - only `let v = s.field;` (a copy-out, which rustc lowers to an
unaligned load; always defined for Copy types regardless of alignment). All
field types are plain integers with no invalid bit patterns.
"""

from __future__ import annotations

from . import register
from .common import Program, ident, pick_int_type, rand_value, wrap


@register("packed_struct_fields")
def generate(rng, idx) -> Program:
    n_fields = rng.randint(3, 6)
    field_types = [pick_int_type(rng) for _ in range(n_fields)]
    field_names = [f"f{i}" for i in range(n_fields)]

    const_vals = [rand_value(rng, t) for t in field_types]
    alt_vals = [rand_value(rng, t) for t in field_types]

    struct_name = ident("Packed", idx)
    const_name = ident("CONST", idx)

    fields_decl = "\n".join(f"    {n}: {t[0]}," for n, t in zip(field_names, field_types))
    const_fields = ", ".join(f"{n}: {v}" for n, v in zip(field_names, const_vals))
    alt_fields = ", ".join(f"{n}: {v}" for n, v in zip(field_names, alt_vals))

    cond1 = rng.random() < 0.5
    current_vals = list(const_vals if cond1 else alt_vals)

    mut_field_idx = rng.randrange(n_fields)
    other_idx = (mut_field_idx + 1) % n_fields
    mut_delta = rng.randint(1, 50)
    other_delta = rng.randint(1, 50)
    cond2 = rng.random() < 0.5

    # Exactly one of these two textual branches actually executes (matching
    # cond2), so only that one's field update is reflected in current_vals -
    # the other remains dead code whose delta is never applied.
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
    mutate_branch, dead_branch = if_branch_text, else_branch_text

    copies = "\n".join(f"    let v{i} = s.{n};" for i, n in enumerate(field_names))
    print_args = ", ".join(f"v{i}" for i in range(n_fields))
    print_fmt = " ".join(["{}"] * n_fields)

    const_copy_idx = rng.randrange(n_fields)

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
{mutate_branch}
    }} else {{
{dead_branch}
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
        name=f"packed_struct_fields_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="packed_struct_fields",
    )
