"""Category: `identity_chain_unsafe` - a small, OPT-IN extension of
`identity_chain.py` (see that module's docstring for the paradigm and the
safe transform library this one builds on top of). NOT included in the
default category rotation (see `templates/__init__.py`'s `register(...,
default=False)` and `generator.py`'s `--list-categories`/default-selection
logic) - pass `--categories identity_chain_unsafe` explicitly to use it. The
bulk of this tool's coverage is meant to come from the safe path; this
module exists only for the two patterns below, which the user explicitly
scoped down to keep self-evidently sound without Miri-level reasoning.

## Pattern 1: pointer round-trip (`_t_ptr_roundtrip`, mixed into the normal
## transform pool via `ChainCtx.extra_transforms`, so it can appear ANYWHERE
## in the tree, composed with every safe transform)

`{ let v: T = <sub-chain>; unsafe { *(&v as *const T) } }` - a TRUE identity,
by the most basic pointer-validity rule there is: `&v` is a reference taken
of a local that is alive for the entire remainder of this block, the raw
pointer is cast from that reference and immediately dereferenced in the very
same expression, same scope, with no other reference or pointer to `v`
created or used concurrently, and no write to `v` happens between the cast
and the deref. There is no aliasing, no stacked-borrows retag history to
reason about, and no provenance laundering - just "you can always deref a
pointer you just made to still-live local data". Because it's a true
identity, it costs nothing extra to verify: same "expected == starting
constant" oracle as every safe transform.

## Pattern 2: union byte-reinterpretation (applied exactly ONCE, as the
## final step after a fully-identity safe+ptr-roundtrip chain - see
## `generate()` below)

`union ReinterpretU { whole: TInt, bytes: [u8; N] }`, write `whole`, read
back `bytes`. This is deliberately NOT presented as another entry in the
identity-transform pool, because it is genuinely not one: reading a `u16`/
`u32`/`u64` back out as its constituent bytes produces a real bit-level
reinterpretation of the value, not the value itself. Its "oracle" is
therefore not "equals the starting constant" - it's `struct.pack`/`unpack`
over whatever the fully-identity chain's value turns out to be (which IS
exactly known at generation time: the starting constant, since every
upstream node is a verified identity) - deterministic, mechanical byte math
in Python, not a logic-mirroring reimplementation of generated control flow.
That keeps this still low-risk in the same spirit as the rest of this tool,
even though it's the one place the oracle isn't trivially `== constant`.
Scoped to unsigned integer <-> same-width byte-array pairs only (no
pointers, no `bool`/`char` validity constraints to reason about): every bit
pattern of an unsigned integer is already a valid `[u8; N]` (plain data, no
padding, no niche - any of the 256 values is valid in every byte position),
and every `[u8; N]` is equally a valid bit pattern of the matching unsigned
integer, for the same reason - so both the write and the read are always
defined, regardless of which specific bytes happen to be there. Uses
`struct.pack("=...", ...)` (native byte order, no alignment padding) on the
Python side so the expected value matches whatever this host's Rust actually
lays the integer out as, instead of hardcoding an endianness assumption.
"""

from __future__ import annotations

import struct

from . import identity_chain as ic
from . import register
from .common import Program, ident


def _t_ptr_roundtrip(ctx: ic.ChainCtx, vty: ic.VType, expr: str, depth: int, breadth: int) -> str:
    child = ic.build_chain(ctx, vty, expr, depth - 1, breadth)
    var = ctx.fresh_name("ptrv")
    return f"{{ let {var}: {vty.rust_ty} = {child}; unsafe {{ *(&{var} as *const {vty.rust_ty}) }} }}"


_UNION_VTYS = [ic.U16, ic.U32, ic.U64]
_STRUCT_FMT = {16: "H", 32: "I", 64: "Q"}


@register("identity_chain_unsafe", default=False)
def generate(rng, idx) -> Program:
    depth, breadth = ic.current_params()
    vty = rng.choice(_UNION_VTYS)
    value = ic.rand_value(rng, vty)

    ctx = ic.ChainCtx(rng=rng, idx=idx, extra_transforms=[_t_ptr_roundtrip])
    ctx.budget = ic._budget_for(depth, breadth)

    start_expr = f"std::hint::black_box({ic.lit(value, vty)})"
    chain_expr = ic.build_chain(ctx, vty, start_expr, depth, breadth)

    nbytes = vty.bit_width // 8
    union_name = ident("ReinterpretU", idx)
    union_decl = f"#[repr(C)]\nunion {union_name} {{ whole: {vty.rust_ty}, bytes: [u8; {nbytes}] }}"

    items = [union_decl] + ctx.items
    items_src = "\n\n".join(items)

    source = f"""#![allow(dead_code)]

{items_src}

fn main() {{
    let chained: {vty.rust_ty} = {chain_expr};
    let reinterpreted: [u8; {nbytes}] = unsafe {{ {union_name} {{ whole: chained }}.bytes }};
    println!("{{:?}}", reinterpreted);
}}
"""

    packed = struct.pack("=" + _STRUCT_FMT[vty.bit_width], value)
    expected_stdout = "[" + ", ".join(str(b) for b in packed) + "]\n"

    return Program(
        name=f"identity_chain_unsafe_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="identity_chain_unsafe",
    )
