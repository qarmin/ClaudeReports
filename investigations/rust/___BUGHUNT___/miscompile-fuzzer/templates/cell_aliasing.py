"""Category: shared-reference interior-mutability aliasing (`Cell<T>`).

Targets whether LLVM ever applies a `noalias`/`readonly` attribute (or an
equivalent "this value can't have changed since I last read it" assumption)
to a `&Holder` parameter that it isn't allowed to apply, because `Holder`
contains a `Cell<T>` field. Rust's aliasing model explicitly forbids treating
`&T` as `noalias`/invariant whenever `T` contains an `UnsafeCell` (which
`Cell` wraps): a shared reference to a type with interior mutability may be
mutated through *any* copy of that same reference at any point, including
from inside a call the optimizer might otherwise want to treat as "can't see
my caller's locals". This is a different surface from every existing
category here - `unions.py`/`packed_structs.py` target GVN/JumpThreading
losing track of a union/packed-field *write*, not a shared-reference
*read-after-interior-mutation* ordering assumption; `reborrow.py` targets
`&mut` reborrow-chain collapsing, not `&`-to-`Cell` aliasing.

The generated program never creates two independent allocations that alias -
it only ever copies the *same* `&Holder` reference into a second local
binding (`h2 = h`, legal because `&T` is `Copy`) and threads both names
through a chain of helper functions with randomly assigned
`#[inline(always)]`/`#[inline(never)]`, alternating reads (`.get()`) and
writes (`.set()`) between the two names. A correct compiler must treat every
read after any `.set()` call (through either name) as potentially changed,
regardless of inlining.

UB-freedom, by construction: `Cell::get`/`Cell::set` are always safe and
total for any `Copy` integer type - no panics, no unsafe, no raw pointers,
no `&mut` ever created (so no aliasing-xor-mutation rule is even in play,
only the *weaker* "shared refs to `Cell`s aren't noalias" rule). The plain
`tag: TY` field is only ever read, never written, so there's nothing to get
out of sync there. All arithmetic goes through `wrapping_add`, so no step can
overflow/panic regardless of which random values were drawn. The initial
counter/tag/deltas are read through `std::hint::black_box` so the whole
pipeline can't be constant-folded away at compile time.
"""

from __future__ import annotations

from . import register
from .common import INT_TYPES, Program, ident, rand_value, wrap


@register("cell_shared_ref_aliasing")
def generate(rng, idx) -> Program:
    ty_tuple = rng.choice(INT_TYPES)
    ty, lo, hi = ty_tuple

    init_counter = rand_value(rng, ty_tuple)
    tag = rand_value(rng, ty_tuple)
    n_steps = rng.randint(4, 10)
    deltas = [rand_value(rng, ty_tuple) for _ in range(n_steps)]
    # Which of the two aliases (h / h2, both the same &Holder) does the
    # write for this step; the other alias does the read right after it.
    writer_is_h = [rng.choice([True, False]) for _ in range(n_steps)]

    inline_read = ident("ca_read", idx)
    inline_write = ident("ca_write", idx)
    inline_tag = ident("ca_tag", idx)
    read_attr = rng.choice(["#[inline(always)]", "#[inline(never)]", "#[inline]"])
    write_attr = rng.choice(["#[inline(always)]", "#[inline(never)]", "#[inline]"])
    tag_attr = rng.choice(["#[inline(always)]", "#[inline(never)]", "#[inline]"])

    step_lines = []
    for k in range(n_steps):
        writer, reader = ("h", "h2") if writer_is_h[k] else ("h2", "h")
        step_lines.append(f"    {inline_write}({writer}, {deltas[k]});")
        step_lines.append(f"    acc = acc.wrapping_add({inline_read}({reader}));")

    source = f"""#![allow(dead_code)]

use std::cell::Cell;

struct Holder {{
    counter: Cell<{ty}>,
    tag: {ty},
}}

{read_attr}
fn {inline_read}(h: &Holder) -> {ty} {{
    h.counter.get()
}}

{write_attr}
fn {inline_write}(h: &Holder, delta: {ty}) {{
    h.counter.set(h.counter.get().wrapping_add(delta));
}}

{tag_attr}
fn {inline_tag}(h: &Holder) -> {ty} {{
    h.tag
}}

fn main() {{
    let holder = Holder {{
        counter: Cell::new(std::hint::black_box({init_counter})),
        tag: std::hint::black_box({tag}),
    }};
    let h: &Holder = &holder;
    let h2: &Holder = h;

    let mut acc: {ty} = {inline_read}(h);
{chr(10).join(step_lines)}
    acc = acc.wrapping_add({inline_tag}(h2));

    println!("{{}}", acc);
    println!("{{}}", holder.counter.get());
}}
"""

    acc = init_counter
    counter = init_counter
    for k in range(n_steps):
        counter = wrap(counter + deltas[k], ty_tuple)
        acc = wrap(acc + counter, ty_tuple)
    acc = wrap(acc + tag, ty_tuple)

    expected_stdout = f"{acc}\n{counter}\n"

    return Program(
        name=f"cell_shared_ref_aliasing_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="cell_shared_ref_aliasing",
    )
