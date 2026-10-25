"""Category: `identity_chain_safe` - a different generation PARADIGM, not
another per-category forward-computation template.

Every other category in `templates/` computes a value forward in Rust and
predicts the same value independently in Python, so a self-check mismatch can
mean either "the template's Python oracle is wrong" or "rustc is wrong" - see
README.md's "why the third oracle matters". This module sidesteps that
ambiguity entirely: it starts from a single `std::hint::black_box`'d constant
and builds an arbitrarily deep/wide tree of control-flow structures around
it, where EVERY node is drawn from a small library of "identity transforms" -
each one provably `v -> v` for every possible input, by a short, self-evident
argument that never needs Miri-level stacked-borrows reasoning. Because every
node is individually an identity, the WHOLE tree is an identity no matter how
deep or wide it gets, so the oracle never has to mirror the generated control
flow - it only has to assert the final printed value equals the starting
constant. `--depth`/`--breadth` (see `generator.py`) are therefore a pure
lever for arbitrarily complex nesting with no corresponding growth in oracle
complexity, which is the whole point: a self-check mismatch found here can
only mean one of this module's transforms isn't actually an identity (a bug
in this file to fix), never a "the Python mirror drifted from the Rust"
mistake, since there is no mirror.

## Types varied

`u8`/`u16`/`u32`/`u64`/`i32` (plain integers, every bit pattern valid),
`bool`, `char` (restricted to a safe, unescaped ASCII letter/digit pool - see
`_CHAR_POOL` - so no Rust-literal-escaping or Debug-quoting corner cases are
needed), and three small Copy composites built only from unsigned-int fields
(`(u32, u16)` tuple, `[u16; 3]` array, a two-field `#[derive(Clone, Copy,
Debug, PartialEq)]` struct) - restricting composite fields to unsigned ints
means Debug formatting and Display formatting coincide field-by-field, so one
`fmt_value` function serves both the top-level `{}` print and nested
`{:?}` formatting without a separate escaping path.

## The identity transform library (one paragraph each, the soundness bar
## every template in this tool already holds itself to - see README.md
## "UB-freedom, by construction")

- **Arithmetic round-trip** (`t_arith`, integer kinds only): `v.wrapping_add(k)
  .wrapping_sub(k)`, `v ^ k ^ k`, and `v.rotate_left(s).rotate_right(s)`. Each
  is a documented, total, panic-free operation for every input (wrapping_*
  never panics by definition; xor is self-inverse; rotate_left/rotate_right
  are exact inverses of each other for ANY shift amount, including amounts
  past the type's own bit width, because Rust defines rotation modulo the bit
  width) - so the round trip returns the original value for every possible
  `v` and `k`, with no case analysis needed. `k` (and the rotate shift) are
  generation-time literals, individually wrapped in `std::hint::black_box`
  so the optimizer can't constant-fold the round trip away before it has to
  reason about it.
- **Branch identity** (`t_branch`): `if cond { v } else { v }` where both
  arms are the SAME value `v` (each independently re-derived by its own
  identity sub-chain, so they need not be textually identical, only
  semantically equal to `v`) - correctness here can never depend on which
  branch actually executes, only on the optimizer correctly handling a
  branch at all, which is exactly the surface this transform stresses.
  `cond` is itself an arbitrarily complex `bool` expression, built the same
  way: a fresh identity sub-chain rooted at a `black_box`'d literal `bool`,
  recursing through this same library with `vty = bool`.
- **Match identity** (`t_match`): every arm of a `match` on an unrelated
  `u32` selector independently reduces, via its own freshly-generated
  sub-chain, back to `v` - the one place the tree genuinely branches width-
  wise (controlled by `--breadth`), since each arm's sub-chain is built
  separately. Exhaustive via a trailing `_ =>` arm, so this never depends on
  whether the selector's value happens to be in range.
- **Loop identity** (`t_loop`): `for _ in 0..n { acc = <sub-chain>(acc) }`
  where the loop body is itself a FULL identity sub-chain (not a partial
  transform that needs exactly `n` applications to cancel out) - so it maps
  `acc -> acc` on every single iteration regardless of what `n` (itself
  `black_box`'d, so not statically known) turns out to be at runtime. This
  is why the loop is safe to use here even though per-category random
  templates elsewhere have to be careful about iteration-count-dependent
  state: there is no state here that depends on the count.
- **Struct/tuple/array wrap-unwrap** (`t_wrap`): `(v,).0`, `[v][0]`, and a
  dedicated one-field newtype `Wrapper(v).0` - constructing a container
  around a value and immediately reading the same field back is definitionally
  the identity, independent of what `v`'s type is (never requires `Copy`,
  `Clone`, or any trait bound - it is a single-owner move in and a
  single-owner move out).
- **Enum wrap-unwrap** (`t_enum_wrap`): `match Some(v) { Some(x) => x, None
  => unreachable!() }` - the `None` arm is unreachable by construction
  (nothing ever produces a `None` of this specific, freshly-constructed
  `Option`), so this never panics and always re-extracts the same `v` that
  was just wrapped.
- **Function/closure call indirection** (`t_call`): calling a vetted,
  freshly-declared `fn ident_fn(x: T) -> T { x }` directly, through an
  explicit `fn(T) -> T` function pointer, or via a `move` closure that
  captures `v` itself (not a parameter) and returns the captured value - all
  three are definitionally the identity function applied once, so this adds
  inlining/devirtualization stress (direct call vs. indirect call through a
  pointer vs. a closure environment) without any risk of the wrong value
  coming back.
- **Bounded recursion** (`t_bounded_recursion`): a freshly-declared `fn
  rec_ident(x: T, n: u32) -> T { if n == 0 { x } else { rec_ident([x][0], n -
  1) } }` - the per-level step (`[x][0]`, the same array wrap-unwrap already
  argued above) is an identity at every level, so by induction the whole
  bounded recursion is the identity for any `n`, letting this exercise a
  self-recursive call chain down to a `--depth`-independent parameter without
  needing a different soundness argument per type.

If you add a transform and can't write an equally short, unconditional (no
"as long as the RNG picks X" caveat) `v -> v` argument for it, don't add it -
the entire point of this category is that the oracle computation stays
`expected == starting_constant` no matter how deep `--depth`/`--breadth` go,
and that only holds if every node really is an identity.

## Code-size safety valve

`--depth`/`--breadth` alone could blow up the emitted source exponentially
(a `match` node multiplies by its arm count, and arms can themselves contain
more `match` nodes). `ChainCtx.budget` is a shared, mutable node-count cap
threaded through every recursive `build_chain` call (independent of
`--depth`/`--breadth`'s own nominal ranges): once it hits zero, `build_chain`
stops recursing and returns the current expression unchanged (a valid leaf
at any position in the tree), so arbitrarily large `--depth`/`--breadth`
values degrade gracefully into "no more nesting", never into a stalled
`rustc` invocation on a multi-megabyte file.
"""

from __future__ import annotations

import random
import string
from dataclasses import dataclass, field

from . import register
from .common import Program, rust_bool

# --------------------------------------------------------------------------
# Value-type system: scalar + a few small Copy composites built only from
# unsigned-int fields (keeps nested Debug formatting identical to Display,
# so one `fmt_value` serves both - see module docstring).
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class VType:
    rust_ty: str
    kind: str  # "uint" | "sint" | "bool" | "char" | "tuple" | "array" | "struct"
    tag: str
    bit_width: int | None = None
    fields: tuple["VType", ...] | None = None
    elem: "VType | None" = None
    length: int | None = None
    struct_name: str | None = None
    struct_field_names: tuple[str, ...] | None = None
    struct_decl: str | None = None


U8 = VType("u8", "uint", "u8", bit_width=8)
U16 = VType("u16", "uint", "u16", bit_width=16)
U32 = VType("u32", "uint", "u32", bit_width=32)
U64 = VType("u64", "uint", "u64", bit_width=64)
I32 = VType("i32", "sint", "i32", bit_width=32)
BOOL = VType("bool", "bool", "bool")
CHAR = VType("char", "char", "char")

SCALAR_VTYS = [U8, U16, U32, U64, I32]

# Only ASCII letters/digits: never need Rust-char-literal escaping, and Debug
# formatting of a `char` (quoted, e.g. `'a'`) is never hit since `char` is
# only ever printed at the top level via `{}` (Display), not nested inside a
# composite (composite fields are restricted to unsigned ints - see above).
_CHAR_POOL = [c for c in (string.ascii_letters + string.digits)]


def make_tuple2_vty() -> VType:
    fields = (U32, U16)
    return VType(rust_ty="(u32, u16)", kind="tuple", tag="tuple2", fields=fields)


def make_array3_vty() -> VType:
    elem = U16
    length = 3
    return VType(rust_ty=f"[{elem.rust_ty}; {length}]", kind="array", tag=f"array{length}_{elem.tag}", elem=elem, length=length)


def make_struct_vty(idx: int) -> VType:
    name = f"ChainPair{idx}"
    fields = (U32, U16)
    field_names = ("a", "b")
    decl = (
        f"#[derive(Clone, Copy, Debug, PartialEq)]\n"
        f"struct {name} {{ {field_names[0]}: {fields[0].rust_ty}, {field_names[1]}: {fields[1].rust_ty} }}"
    )
    return VType(
        rust_ty=name,
        kind="struct",
        tag=name,
        fields=fields,
        struct_name=name,
        struct_field_names=field_names,
        struct_decl=decl,
    )


def rand_value(rng: random.Random, vty: VType):
    """A fresh Python value of the shape `vty` describes - only ever used to
    pick the ONE starting constant (and small internal literals like `k` in
    `t_arith`); never used to re-derive an intermediate value through the
    chain, because every chain node is an identity by construction.
    """
    if vty.kind == "uint":
        return rng.randint(0, 2**vty.bit_width - 1)
    if vty.kind == "sint":
        half = 2 ** (vty.bit_width - 1)
        return rng.randint(-half, half - 1)
    if vty.kind == "bool":
        return rng.choice([True, False])
    if vty.kind == "char":
        return rng.choice(_CHAR_POOL)
    if vty.kind == "tuple":
        return tuple(rand_value(rng, ft) for ft in vty.fields)
    if vty.kind == "array":
        return [rand_value(rng, vty.elem) for _ in range(vty.length)]
    if vty.kind == "struct":
        return tuple(rand_value(rng, ft) for ft in vty.fields)
    raise ValueError(f"unknown vty kind: {vty.kind}")


def lit(value, vty: VType) -> str:
    """A Rust literal expression that evaluates to exactly `value: vty`."""
    if vty.kind == "uint":
        return f"{value}{vty.rust_ty}"
    if vty.kind == "sint":
        return f"({value}{vty.rust_ty})" if value < 0 else f"{value}{vty.rust_ty}"
    if vty.kind == "bool":
        return rust_bool(value)
    if vty.kind == "char":
        return f"'{value}'"
    if vty.kind == "tuple":
        return "(" + ", ".join(lit(v, ft) for v, ft in zip(value, vty.fields)) + ")"
    if vty.kind == "array":
        return "[" + ", ".join(lit(v, vty.elem) for v in value) + "]"
    if vty.kind == "struct":
        parts = (f"{fn}: {lit(v, ft)}" for fn, ft, v in zip(vty.struct_field_names, vty.fields, value))
        return f"{vty.struct_name} {{ " + ", ".join(parts) + " }"
    raise ValueError(f"unknown vty kind: {vty.kind}")


def fmt_value(value, vty: VType) -> str:
    """The expected stdout text for `value: vty`, used both for a scalar's
    top-level `{}` (Display) print and a composite's top-level `{:?}`
    (derived Debug) print - safe to share because every composite field here
    is a plain unsigned int, whose Debug output is identical to its Display
    output (no quoting/escaping divergence like `char` has).
    """
    if vty.kind in ("uint", "sint"):
        return str(value)
    if vty.kind == "bool":
        return "true" if value else "false"
    if vty.kind == "char":
        return value
    if vty.kind == "tuple":
        return "(" + ", ".join(fmt_value(v, ft) for v, ft in zip(value, vty.fields)) + ")"
    if vty.kind == "array":
        return "[" + ", ".join(fmt_value(v, vty.elem) for v in value) + "]"
    if vty.kind == "struct":
        parts = (f"{fn}: {fmt_value(v, ft)}" for fn, ft, v in zip(vty.struct_field_names, vty.fields, value))
        return f"{vty.struct_name} {{ " + ", ".join(parts) + " }"
    raise ValueError(f"unknown vty kind: {vty.kind}")


# --------------------------------------------------------------------------
# --depth/--breadth knobs (module-level so `generator.py` can set them once
# per run via `configure()` - every category's generate(rng, idx) signature
# stays untouched, matching the convention `batching.py`/`noise.py` already
# use for run-level, not-per-program config).
# --------------------------------------------------------------------------

_DEFAULT_DEPTH = 8
_DEFAULT_BREADTH = 3
_CONFIG = {"depth": _DEFAULT_DEPTH, "breadth": _DEFAULT_BREADTH}


def configure(depth: int | None = None, breadth: int | None = None) -> None:
    if depth is not None:
        _CONFIG["depth"] = depth
    if breadth is not None:
        _CONFIG["breadth"] = breadth


def current_params() -> tuple[int, int]:
    return _CONFIG["depth"], _CONFIG["breadth"]


# --------------------------------------------------------------------------
# Chain-building engine.
# --------------------------------------------------------------------------


@dataclass
class ChainCtx:
    rng: random.Random
    idx: int
    items: list[str] = field(default_factory=list)
    budget: int = 10**9
    extra_transforms: list = field(default_factory=list)
    _counter: int = 0
    _wrapper_cache: dict = field(default_factory=dict)
    _fn_cache: dict = field(default_factory=dict)
    _rec_cache: dict = field(default_factory=dict)

    def fresh_name(self, prefix: str) -> str:
        self._counter += 1
        return f"__{prefix}_{self.idx}_{self._counter}"

    def get_wrapper(self, vty: VType) -> str:
        key = vty.tag
        if key not in self._wrapper_cache:
            name = f"Wrap_{self.idx}_{key}_{len(self._wrapper_cache)}"
            self._wrapper_cache[key] = name
            self.items.append(f"struct {name}({vty.rust_ty});")
        return self._wrapper_cache[key]

    def get_identity_fn(self, vty: VType) -> str:
        key = vty.tag
        if key not in self._fn_cache:
            name = f"ident_fn_{self.idx}_{key}"
            self._fn_cache[key] = name
            self.items.append(f"fn {name}(x: {vty.rust_ty}) -> {vty.rust_ty} {{ x }}")
        return self._fn_cache[key]

    def get_recursive_fn(self, vty: VType) -> str:
        key = vty.tag
        if key not in self._rec_cache:
            name = f"rec_ident_{self.idx}_{key}"
            self._rec_cache[key] = name
            self.items.append(
                f"fn {name}(x: {vty.rust_ty}, n: u32) -> {vty.rust_ty} {{\n"
                f"    if n == 0 {{ x }} else {{ {name}([x][0], n - 1) }}\n"
                f"}}"
            )
        return self._rec_cache[key]


def _t_arith(ctx: ChainCtx, vty: VType, expr: str, depth: int, breadth: int) -> str:
    rng = ctx.rng
    child = build_chain(ctx, vty, expr, depth - 1, breadth)
    choice = rng.randint(0, 2)
    if choice == 0:
        k_lit = f"std::hint::black_box({lit(rand_value(rng, vty), vty)})"
        return f"(({child}).wrapping_add({k_lit}).wrapping_sub({k_lit}))"
    if choice == 1:
        k_lit = f"std::hint::black_box({lit(rand_value(rng, vty), vty)})"
        return f"(({child}) ^ {k_lit} ^ {k_lit})"
    shift_lit = f"std::hint::black_box({rng.randint(1, 63)}u32)"
    return f"(({child}).rotate_left({shift_lit}).rotate_right({shift_lit}))"


def _t_branch(ctx: ChainCtx, vty: VType, expr: str, depth: int, breadth: int) -> str:
    rng = ctx.rng
    cond_start = rust_bool(rng.choice([True, False]))
    cond_depth = min(depth - 1, 3)  # bounded: the condition is a side exploration, not the main tree
    cond_expr = build_chain(ctx, BOOL, f"std::hint::black_box({cond_start})", cond_depth, breadth)
    then_e = build_chain(ctx, vty, expr, depth - 1, breadth)
    else_e = build_chain(ctx, vty, expr, depth - 1, breadth)
    return f"(if {cond_expr} {{ {then_e} }} else {{ {else_e} }})"


def _t_match(ctx: ChainCtx, vty: VType, expr: str, depth: int, breadth: int) -> str:
    rng = ctx.rng
    n_arms = rng.randint(2, max(2, breadth))
    selector = rng.randint(0, n_arms + 2)
    arms = []
    for i in range(n_arms - 1):
        sub = build_chain(ctx, vty, expr, depth - 1, breadth)
        arms.append(f"{i}u32 => {{ {sub} }},")
    last_sub = build_chain(ctx, vty, expr, depth - 1, breadth)
    arms.append(f"_ => {{ {last_sub} }},")
    return f"(match std::hint::black_box({selector}u32) {{\n" + "\n".join(arms) + "\n})"


def _t_loop(ctx: ChainCtx, vty: VType, expr: str, depth: int, breadth: int) -> str:
    rng = ctx.rng
    n = rng.randint(1, 6)
    var = ctx.fresh_name("acc")
    body = build_chain(ctx, vty, var, depth - 1, breadth)
    return (
        f"{{ let mut {var}: {vty.rust_ty} = {expr}; "
        f"for _ in 0..std::hint::black_box({n}u32) {{ {var} = {body}; }} {var} }}"
    )


def _t_wrap(ctx: ChainCtx, vty: VType, expr: str, depth: int, breadth: int) -> str:
    rng = ctx.rng
    child = build_chain(ctx, vty, expr, depth - 1, breadth)
    kind = rng.choice(["tuple1", "array1", "newtype"])
    if kind == "tuple1":
        return f"(({child},).0)"
    if kind == "array1":
        return f"([{child}][0])"
    wrapper = ctx.get_wrapper(vty)
    return f"({wrapper}({child}).0)"


def _t_enum_wrap(ctx: ChainCtx, vty: VType, expr: str, depth: int, breadth: int) -> str:
    child = build_chain(ctx, vty, expr, depth - 1, breadth)
    return f"(match Some({child}) {{ Some(x) => x, None => unreachable!() }})"


def _t_call(ctx: ChainCtx, vty: VType, expr: str, depth: int, breadth: int) -> str:
    rng = ctx.rng
    child = build_chain(ctx, vty, expr, depth - 1, breadth)
    mode = rng.choice(["direct", "fnptr", "closure"])
    if mode == "direct":
        return f"({ctx.get_identity_fn(vty)}({child}))"
    if mode == "fnptr":
        fn_name = ctx.get_identity_fn(vty)
        return f"{{ let f: fn({vty.rust_ty}) -> {vty.rust_ty} = {fn_name}; f({child}) }}"
    return f"{{ let __captured: {vty.rust_ty} = {child}; let cl = move || __captured; cl() }}"


def _t_bounded_recursion(ctx: ChainCtx, vty: VType, expr: str, depth: int, breadth: int) -> str:
    rng = ctx.rng
    child = build_chain(ctx, vty, expr, depth - 1, breadth)
    fn_name = ctx.get_recursive_fn(vty)
    n = rng.randint(1, 4)
    return f"({fn_name}({child}, {n}u32))"


_BASE_TRANSFORMS = [_t_branch, _t_match, _t_loop, _t_wrap, _t_enum_wrap, _t_call, _t_bounded_recursion]
_INT_ONLY_TRANSFORMS = [_t_arith]


def applicable_transforms(vty: VType, ctx: ChainCtx) -> list:
    pool = list(_BASE_TRANSFORMS)
    if vty.kind in ("uint", "sint"):
        pool += _INT_ONLY_TRANSFORMS
    pool += ctx.extra_transforms
    return pool


def build_chain(ctx: ChainCtx, vty: VType, expr: str, depth: int, breadth: int) -> str:
    """Return a Rust expression of type `vty` that is provably equal to
    whatever `expr` (itself an expression of type `vty`) evaluates to, built
    by recursing `depth` levels deep through `applicable_transforms`, each
    one independently an identity - see module docstring. Bottoms out either
    when `depth` reaches 0 or when `ctx.budget` (a shared node-count safety
    valve - see "Code-size safety valve" above) is exhausted, in which case
    `expr` is returned unchanged: always a valid leaf, since every caller
    only ever substitutes the returned text into a position that expects a
    `vty`-typed expression.
    """
    if depth <= 0 or ctx.budget <= 0:
        return expr
    ctx.budget -= 1
    transform = ctx.rng.choice(applicable_transforms(vty, ctx))
    return transform(ctx, vty, expr, depth, breadth)


def pick_vty(rng: random.Random, idx: int) -> VType:
    r = rng.random()
    if r < 0.55:
        return rng.choice(SCALAR_VTYS)
    if r < 0.75:
        return BOOL
    if r < 0.90:
        return CHAR
    kind = rng.choice(["tuple", "array", "struct"])
    if kind == "tuple":
        return make_tuple2_vty()
    if kind == "array":
        return make_array3_vty()
    return make_struct_vty(idx)


def _budget_for(depth: int, breadth: int) -> int:
    return min(500, max(60, depth * breadth * 6))


@register("identity_chain_safe")
def generate(rng, idx) -> Program:
    depth, breadth = current_params()
    vty = pick_vty(rng, idx)
    value = rand_value(rng, vty)

    ctx = ChainCtx(rng=rng, idx=idx)
    ctx.budget = _budget_for(depth, breadth)

    start_expr = f"std::hint::black_box({lit(value, vty)})"
    chain_expr = build_chain(ctx, vty, start_expr, depth, breadth)

    print_fmt = "{}" if vty.kind in ("uint", "sint", "bool", "char") else "{:?}"

    items = []
    if vty.struct_decl:
        items.append(vty.struct_decl)
    items.extend(ctx.items)
    items_src = "\n\n".join(items)

    source = f"""#![allow(dead_code)]

{items_src}

fn main() {{
    let result: {vty.rust_ty} = {chain_expr};
    println!("{print_fmt}", result);
}}
"""
    expected_stdout = fmt_value(value, vty) + "\n"

    return Program(
        name=f"identity_chain_safe_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="identity_chain_safe",
    )
