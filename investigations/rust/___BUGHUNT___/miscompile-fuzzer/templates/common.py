"""Shared helpers for template modules.

Keep this UB-avoidance-critical: every helper here exists so category modules
don't each reinvent "pick a type/value that is safe to use the way I'm about
to use it". If you add a helper that can produce an unsafe combination (e.g.
a value that isn't a valid bit pattern for some type it'll be transmuted/union
-punned into), document the invariant it relies on right next to it.
"""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class Program:
    """One generated corpus entry."""

    name: str
    source: str
    expected_stdout: str
    category: str


# Plain (non-niche, no padding surprises) integer types safe to use
# interchangeably for packed-struct fields, union fields, arithmetic, etc.
INT_TYPES = [
    ("u8", 0, 255),
    ("u16", 0, 65535),
    ("u32", 0, 2**32 - 1),
    ("u64", 0, 2**64 - 1),
    ("i8", -128, 127),
    ("i16", -32768, 32767),
    ("i32", -(2**31), 2**31 - 1),
    ("i64", -(2**63), 2**63 - 1),
    ("usize", 0, 2**64 - 1),
    ("isize", -(2**63), 2**63 - 1),
]


def pick_int_type(rng):
    return rng.choice(INT_TYPES)


def rand_value(rng, ty_tuple):
    """A value guaranteed in-range for the given (name, lo, hi) int type."""
    _, lo, hi = ty_tuple
    return rng.randint(lo, hi)


def wrap(value: int, ty_tuple) -> int:
    """Wrap `value` into the representable range of `ty_tuple`, mirroring
    Rust `wrapping_add`/two's-complement semantics for the oracle. Driven
    purely by (lo, hi) so it works uniformly for sized and pointer-width
    (usize/isize) types without needing to parse a bit width out of the name.
    """
    _, lo, hi = ty_tuple
    span = hi - lo + 1
    return (value - lo) % span + lo


def ident(prefix: str, idx: int, *parts) -> str:
    suffix = "_".join(str(p) for p in parts)
    return f"{prefix}_{idx}_{suffix}" if suffix else f"{prefix}_{idx}"


def rust_bool(b: bool) -> str:
    return "true" if b else "false"


def expected_lines(*lines: str) -> str:
    return "".join(line + "\n" for line in lines)
