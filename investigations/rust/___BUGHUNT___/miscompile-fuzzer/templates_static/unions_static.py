"""Category: unions with reference/pointer sibling fields - STATIC variant.

Same target and UB-safety argument as `../templates/unions.py` (GVN /
JumpThreading / DataflowConstProp "aliasing flood" bugs across union
sibling-field writes - rust-lang/rust#160004, #161898, #163825): both union
fields (`r: &'static i64`, `p: *const i64`) only ever hold an address
obtained as `&SVALS[i]` for `i` in range, so every read back is a valid
reference or a valid raw pointer no matter which field produced the bits.

Every program here comes from an explicitly declared combination in
SCENARIO_AXES below - no RNG. Add a value to any axis list to broaden
coverage (e.g. a longer `sequence`, a new `n_vals`); the combination count is
always exactly the product of the axis list lengths, declared up front.
"""

from __future__ import annotations

from templates.common import Program, ident

from . import register_static
from .common import product_scenarios

# The entire test matrix. Currently 3 * 4 * 2 = 24 programs.
SCENARIO_AXES = {
    "n_vals": [4, 6, 8],
    "sequence": [
        # Each sequence is a tuple of (write_field, read_field, index_mode) steps.
        (("r", "r", "first"), ("p", "p", "last")),
        (("r", "p", "next"), ("p", "r", "next")),
        (("p", "p", "first"), ("r", "r", "last"), ("p", "r", "mid")),
        (("r", "r", "first"), ("r", "p", "last"), ("p", "p", "first"), ("p", "r", "last")),
    ],
    "start_field": ["r", "p"],
}


def _resolve_index(mode: str, n_vals: int, step: int) -> int:
    if mode == "first":
        return 0
    if mode == "last":
        return n_vals - 1
    if mode == "mid":
        return n_vals // 2
    if mode == "next":
        return step % n_vals
    raise ValueError(f"unknown index_mode {mode!r}")


def _field_init_expr(field: str, static_name: str, i: int) -> str:
    if field == "r":
        return f"&{static_name}[{i}]"
    return f"&{static_name}[{i}] as *const i64"


def _build(scenario: dict, idx: int) -> Program:
    n_vals = scenario["n_vals"]
    sequence = scenario["sequence"]
    start_field = scenario["start_field"]

    # Deterministic, pairwise-distinct values - a prime multiplier gives
    # distinct values per index without needing randomness.
    vals = [(i + 1) * 1_000_003 for i in range(n_vals)]

    static_name = ident("SVALS", idx)
    union_name = ident("SU", idx)

    cur = 0
    stmts = [f"    let mut u = {union_name} {{ {start_field}: {_field_init_expr(start_field, static_name, cur)} }};"]
    expected = []

    for step_num, (write_field, read_field, index_mode) in enumerate(sequence):
        new_idx = _resolve_index(index_mode, n_vals, step_num)
        stmts.append(f"    u.{write_field} = {_field_init_expr(write_field, static_name, new_idx)};")
        cur = new_idx

        read_var = ident("read", idx, step_num)
        stmts.append(f"    let {read_var} = unsafe {{ *u.{read_field} }};")
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
        name=f"union_ref_fields_static_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="union_ref_fields_static",
    )


@register_static("union_ref_fields_static")
def generate_all() -> list[Program]:
    scenarios = product_scenarios(SCENARIO_AXES)
    return [_build(s, idx) for idx, s in enumerate(scenarios)]
