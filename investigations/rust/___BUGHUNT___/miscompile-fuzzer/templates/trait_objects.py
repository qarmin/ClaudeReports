"""Category: trait-object vtable dispatch vs devirtualized/monomorphized calls.

Targets the family behind rust-lang/rust#163856 ("DeduceParamAttrs misresolves
default-vs-override trait method through dyn Trait fn-pointer"): a trait with
a default-provided method (`apply`) that some implementors override and
others inherit as-is. Each shape's `apply`/`offset` is called four different
ways that must all agree: directly on the concrete type (fully monomorphized,
possibly inlined), through a generic function (`call_generic::<T: Op>`,
monomorphized per-instantiation), through an explicit `&dyn Op` (vtable
dispatch), and through a `Vec<Box<dyn Op>>` iterated in a fixed, source-order
sequence (heterogeneous vtable dispatch). Random `#[inline(..)]` attributes on
each impl's methods vary how much of this a given config can see through.

UB-safety argument: every shape only ever computes with `wrapping_*` methods
on plain `i64` fields (no bare `+`/`-`/`*`, so no overflow panic is possible
regardless of the random offsets/inputs chosen); all dispatch is safe trait
dispatch (static, generic-monomorphized, or vtable) with no `unsafe`, no raw
pointers, and no uninitialized state - every `Box`/`Vec` element is
constructed with a concrete, fully-initialized value before first use.
"""

from __future__ import annotations

from . import register
from .common import INT_TYPES, Program, ident, wrap

I64 = next(t for t in INT_TYPES if t[0] == "i64")

# Each override formula's Rust expression (using `self.off` and `x`) paired
# with its Python mirror. "default" is not in this table - it's the trait's
# own default-method body, reused as-is by any shape that doesn't override.
OVERRIDE_FORMULAS = {
    "double": ("x.wrapping_mul(2).wrapping_add(self.off)", lambda x, off: wrap(wrap(x * 2, I64) + off, I64)),
    "sub": ("x.wrapping_sub(self.off)", lambda x, off: wrap(x - off, I64)),
    "mulshift": ("x.wrapping_mul(self.off).wrapping_add(1)", lambda x, off: wrap(wrap(x * off, I64) + 1, I64)),
}


def _default_formula(x, off):
    return wrap(x + off, I64)


def _inline_attr(rng):
    return rng.choice(["#[inline(always)]", "#[inline(never)]", "#[inline]"])


@register("trait_object_devirt")
def generate(rng, idx) -> Program:
    trait_name = ident("Op", idx)
    shape_names = [ident("Shape", idx, letter) for letter in "ABC"]

    shapes = []  # list of dicts: name, off, formula_key (None == default), offset_attr, apply_attr
    for name in shape_names:
        off = rng.randint(-1000, 1000)
        override = rng.random() < 0.6
        formula_key = rng.choice(list(OVERRIDE_FORMULAS)) if override else None
        shapes.append(
            {
                "name": name,
                "off": off,
                "formula_key": formula_key,
                "offset_attr": _inline_attr(rng),
                "apply_attr": _inline_attr(rng),
            }
        )

    def resolved_value(shape, x):
        if shape["formula_key"] is None:
            return _default_formula(x, shape["off"])
        return OVERRIDE_FORMULAS[shape["formula_key"]][1](x, shape["off"])

    struct_defs = []
    impl_defs = []
    for s in shapes:
        struct_defs.append(f"struct {s['name']} {{ off: i64 }}")
        if s["formula_key"] is None:
            impl_defs.append(
                f"impl {trait_name} for {s['name']} {{\n"
                f"    {s['offset_attr']}\n"
                f"    fn offset(&self) -> i64 {{ self.off }}\n"
                f"}}"
            )
        else:
            rust_expr, _ = OVERRIDE_FORMULAS[s["formula_key"]]
            impl_defs.append(
                f"impl {trait_name} for {s['name']} {{\n"
                f"    {s['offset_attr']}\n"
                f"    fn offset(&self) -> i64 {{ self.off }}\n"
                f"    {s['apply_attr']}\n"
                f"    fn apply(&self, x: i64) -> i64 {{ {rust_expr} }}\n"
                f"}}"
            )

    call_generic_fn = ident("call_generic", idx)
    call_dyn_fn = ident("call_dyn", idx)

    direct_inputs = [rng.randint(-10_000, 10_000) for _ in shapes]
    vec_inputs = [rng.randint(-10_000, 10_000) for _ in shapes]

    instance_lines = []
    call_lines = []
    expected = []
    for s, x in zip(shapes, direct_inputs):
        local = ident("inst", idx, s["name"])
        instance_lines.append(f"    let {local} = {s['name']} {{ off: {s['off']} }};")
        call_lines.append(
            f'    println!("{{}} {{}} {{}}", {local}.apply({x}), '
            f"{call_generic_fn}(&{local}, {x}), {call_dyn_fn}(&{local}, {x}));"
        )
        v = resolved_value(s, x)
        expected.append(f"{v} {v} {v}")

    boxed_items = ", ".join(f"Box::new({s['name']} {{ off: {s['off']} }})" for s in shapes)
    vec_name = ident("items", idx)
    loop_lines = [f"    for (i, it) in {vec_name}.iter().enumerate() {{"]
    loop_lines.append(f"        let inputs: [i64; {len(shapes)}] = [{', '.join(str(v) for v in vec_inputs)}];")
    loop_lines.append("        println!(\"{}\", it.apply(std::hint::black_box(inputs[i])));")
    loop_lines.append("    }")

    for s, x in zip(shapes, vec_inputs):
        expected.append(str(resolved_value(s, x)))

    source = f"""#![allow(dead_code)]

trait {trait_name} {{
    fn offset(&self) -> i64;
    fn apply(&self, x: i64) -> i64 {{
        x.wrapping_add(self.offset())
    }}
}}

{chr(10).join(struct_defs)}

{chr(10).join(impl_defs)}

fn {call_generic_fn}<T: {trait_name}>(o: &T, x: i64) -> i64 {{
    o.apply(x)
}}

fn {call_dyn_fn}(o: &dyn {trait_name}, x: i64) -> i64 {{
    o.apply(x)
}}

fn main() {{
{chr(10).join(instance_lines)}
{chr(10).join(call_lines)}

    let {vec_name}: Vec<Box<dyn {trait_name}>> = vec![{boxed_items}];
{chr(10).join(loop_lines)}
}}
"""

    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"trait_object_devirt_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="trait_object_devirt",
    )
