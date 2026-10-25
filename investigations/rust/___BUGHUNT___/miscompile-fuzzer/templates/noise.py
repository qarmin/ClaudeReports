"""Shared, provably-inert noise injection, reused by both generator.py
(`templates/`) and static_generator.py (`templates_static/`).

Why this exists: a real rustc miscompile class only triggers when
unrelated-looking code sits near the actual trigger, because several
LLVM/MIR optimization passes gate on function size, instruction count, or
inlining-budget heuristics - padding a function's neighborhood with inert
filler can flip which optimization path gets taken. `--noise-level {0,1,2}`
(see `generator.py`/`static_generator.py`) controls how much of this filler
gets woven around each sub-case's real test logic.

CRITICAL INERTNESS INVARIANT (also documented in README.md "Noise
injection"): every piece of noise this module emits must be PROVABLY inert:

  - it must never read or write any variable/static/field that the real test
    logic (the actual tested pattern) reads or writes - noise only ever
    touches its own freshly-declared locals, or extra types/functions that
    this module itself declared;
  - it must be fully deterministic - every "random" choice happens here, at
    *generation* time, via the `rng` passed in by the caller, and is baked
    into the emitted Rust source as literal values. Nothing emitted ever
    calls an RNG, reads the clock, or reads an address at *program* runtime;
  - its own result is never printed and never compared - every noise value
    is threaded through `std::hint::black_box(..)` and then discarded (`let
    _ = ...`), so it can still be "real work" for the optimizer to reason
    about without ever becoming part of a program's observable stdout.

Because of this, any stdout divergence a diff run finds in a noisy file is
still attributable only to the real tested pattern in that sub-case, never to
the injected noise - the noise never participates in what gets printed or
compared.

Placement (see `templates/batching.py`): noise top-level items are spliced
in among the sub-case's own top-level items (order-independent in Rust); the
`#[inline]`-friendly pre/post statement groups are spliced immediately before
and after the real logic, which is itself wrapped in its own nested `{ }`
block so this splice point is always syntactically valid regardless of
whether the real logic's last statement happens to be a tail expression.
"""

from __future__ import annotations

import random
from dataclasses import dataclass, field


@dataclass
class NoiseBundle:
    """One sub-case's worth of inert filler, already rendered to Rust source
    fragments. Empty (all fields empty lists) at `level=0`.
    """

    items: list[str] = field(default_factory=list)
    pre_stmts: list[str] = field(default_factory=list)
    post_stmts: list[str] = field(default_factory=list)

    def is_empty(self) -> bool:
        return not (self.items or self.pre_stmts or self.post_stmts)


# Tuned small (level 1) vs. deliberately large (level 2 - aiming to cross
# typical rustc/LLVM inlining-cost thresholds, which key off emitted
# instruction/MIR-statement count - these literal sizes are a heuristic
# approximation, not a guarantee tied to any specific version's constant).
_LEVEL_PARAMS = {
    1: dict(n_helper_fns=2, n_big_fns=0, n_types=1, n_asserts=2, n_dead_calls=2),
    2: dict(n_helper_fns=4, n_big_fns=2, n_types=3, n_asserts=6, n_dead_calls=6),
}

_TAUTOLOGY_SHAPES = 6


def _make_tautology(rng: random.Random) -> str:
    """A `assert!`/`debug_assert!` whose condition is provably always true
    by construction (a wrapping-arithmetic or bitwise identity), with the
    operands baked in as generation-time literals - varies shape so a batch
    doesn't repeat one condition everywhere.
    """
    shape = rng.randint(0, _TAUTOLOGY_SHAPES - 1)
    x = rng.randint(0, 2**32 - 1)
    y = rng.randint(1, 31)
    z = rng.randint(0, 2**32 - 1)
    if shape == 0:
        return f"assert!(({x}u64).wrapping_add({x}u64).wrapping_sub({x}u64) == {x}u64);"
    if shape == 1:
        return f"debug_assert!(({x}u32).rotate_left({y}).rotate_right({y}) == {x}u32);"
    if shape == 2:
        return f"assert!(({x}u64) ^ ({x}u64) == 0u64);"
    if shape == 3:
        return f"debug_assert!(({x}u64).wrapping_mul(1) == {x}u64);"
    if shape == 4:
        return f"assert!(!({x}u64) == ({x}u64) ^ u64::MAX);"
    return f"debug_assert!(({x}u32) | ({z}u32) | ({x}u32) == ({x}u32) | ({z}u32));"


def _make_helper_fn(rng: random.Random, name: str, big: bool) -> str:
    """A small/large unrelated arithmetic helper. `big` chains many more
    sequential wrapping ops to inflate this function's own instruction
    count (level 2's "cross an inlining-budget threshold" intent) rather
    than relying on a runtime loop trip count, which the inliner's static
    cost model does not see.
    """
    a = rng.randint(1, 10_000)
    b = rng.randint(1, 10_000)
    if not big:
        op = rng.choice(["wrapping_add", "wrapping_mul", "wrapping_sub"])
        return f"fn {name}(x: u64) -> u64 {{ x.{op}({a}u64).rotate_left({b % 31 + 1}) ^ {b}u64 }}"

    n_ops = rng.randint(60, 120)
    lines = [f"fn {name}(x: u64) -> u64 {{", "    let mut acc: u64 = x;"]
    for i in range(n_ops):
        c = rng.randint(1, 1_000_000)
        choice = rng.randint(0, 3)
        if choice == 0:
            lines.append(f"    acc = acc.wrapping_add({c}u64);")
        elif choice == 1:
            lines.append(f"    acc = acc.wrapping_mul({(c % 65535) | 1}u64);")
        elif choice == 2:
            lines.append(f"    acc = acc ^ {c}u64;")
        else:
            lines.append(f"    acc = acc.rotate_left({(i * 7 + 3) % 31 + 1});")
    lines.append("    acc")
    lines.append("}")
    return "\n".join(lines)


def _make_type(rng: random.Random, name: str) -> str:
    """An extra, unrelated struct/enum definition, plus an inherent
    `sample()` constructor so callers can build a dead instance of it
    without needing to know (or match on) which shape got picked.
    """
    kind = rng.choice(["struct", "enum"])
    av = rng.randint(0, 1000)
    bv = rng.randint(-1000, 1000)
    if kind == "struct":
        decl = f"struct {name} {{ a: u64, b: i32, c: bool }}"
        ctor = f"impl {name} {{ fn sample() -> Self {{ {name} {{ a: {av}u64, b: {bv}i32, c: {rng.choice(['true', 'false'])} }} }} }}"
        return decl + "\n" + ctor
    n_variants = rng.randint(2, 4)
    variants = ", ".join(f"V{i}(u64)" for i in range(n_variants))
    picked = rng.randrange(n_variants)
    decl = f"enum {name} {{ {variants} }}"
    ctor = f"impl {name} {{ fn sample() -> Self {{ {name}::V{picked}({av}u64) }} }}"
    return decl + "\n" + ctor


def generate(rng: random.Random, level: int) -> NoiseBundle:
    """Build one sub-case's noise bundle. `rng` must be a `random.Random`
    the caller seeded deterministically (and that is otherwise unused by the
    real test logic's own generation, so noise draws can never perturb the
    real sub-case's values) - see `templates/batching.py` callers for the
    seeding convention used in each generator mode.
    """
    if level == 0:
        return NoiseBundle()
    if level not in _LEVEL_PARAMS:
        raise ValueError(f"unknown noise level: {level} (expected 0, 1, or 2)")
    params = _LEVEL_PARAMS[level]

    items: list[str] = []
    helper_names: list[str] = []
    fn_counter = 0
    for _ in range(params["n_helper_fns"]):
        name = f"noise_fn_{fn_counter}"
        fn_counter += 1
        helper_names.append(name)
        items.append(_make_helper_fn(rng, name, big=False))
    for _ in range(params["n_big_fns"]):
        name = f"noise_fn_{fn_counter}"
        fn_counter += 1
        helper_names.append(name)
        items.append(_make_helper_fn(rng, name, big=True))

    type_names = []
    for i in range(params["n_types"]):
        name = f"NoiseType{i}"
        type_names.append(name)
        items.append(_make_type(rng, name))

    pre_stmts: list[str] = []
    post_stmts: list[str] = []

    for i in range(params["n_asserts"]):
        (pre_stmts if i % 2 == 0 else post_stmts).append(_make_tautology(rng))

    for i in range(params["n_dead_calls"]):
        seed = rng.randint(0, 2**32 - 1)
        if helper_names:
            fname = rng.choice(helper_names)
            stmt = f"let _ = std::hint::black_box({fname}(std::hint::black_box({seed}u64)));"
        else:
            stmt = f"let _ = std::hint::black_box({seed}u64.wrapping_mul(2654435761u64));"
        (pre_stmts if i % 2 == 0 else post_stmts).append(stmt)

    for name in type_names:
        stmt = f"let _ = std::hint::black_box({name}::sample());"
        post_stmts.append(stmt)

    return NoiseBundle(items=items, pre_stmts=pre_stmts, post_stmts=post_stmts)
