"""Category: drop-order-sensitive nested scopes, structs, and tuples.

Targets optimizations reordering or eliding Drop side effects (the family
behind rust-lang/rust#156008, "AsyncDrop broken normal Drop" - a sibling
concern in the same spirit, just for plain synchronous `Drop`): a randomly
generated tree of nested blocks containing single `Logger` locals,
`Pair { a, b }` structs, and `(Logger, Logger)` tuples, with the occasional
early explicit `drop(x)`. Each `Logger::drop` appends its id to a shared
`Rc<RefCell<Vec<u32>>>` log, so the final printed vector *is* the exact
observed drop order - any divergence from the Rust-reference-guaranteed order
(locals drop in reverse declaration order within a scope; struct/tuple fields
drop in forward declaration order; a nested block fully resolves before its
enclosing scope continues) is a genuine miscompile, confirmed empirically
against both `-C opt-level=0` and `=3` before relying on it (see
`../README.md`'s validation section).

UB-safety argument: every `Logger` holds only an `Rc<RefCell<Vec<u32>>>`
clone and a `u32` id, both always validly initialized before first use;
`RefCell::borrow_mut()` is only ever live for the single statement inside
`Drop::drop`, and the one `.borrow()` in `main` happens only after every
`Logger` has already been dropped, so no two borrows are ever simultaneously
active (no `BorrowMutError` panic possible). No `unsafe`, no raw pointers, no
arithmetic that could overflow.
"""

from __future__ import annotations

from . import register
from .common import Program, ident


def _make_slots(rng, counter, depth, max_depth):
    n = rng.randint(1, 3)
    slots = []
    for _ in range(n):
        choices = ["single", "pair", "tuple"]
        if depth < max_depth:
            choices.append("nested")
        kind = rng.choice(choices)
        if kind == "single":
            counter[0] += 1
            slots.append({"kind": "single", "id": counter[0], "explicit_drop": rng.random() < 0.3})
        elif kind in ("pair", "tuple"):
            counter[0] += 1
            a = counter[0]
            counter[0] += 1
            b = counter[0]
            slots.append({"kind": kind, "ids": (a, b), "explicit_drop": rng.random() < 0.2})
        else:
            slots.append({"kind": "nested", "slots": _make_slots(rng, counter, depth + 1, max_depth)})
    return slots


def _emit_scope(slots, pair_struct, indent="        "):
    lines = []
    for slot in slots:
        if slot["kind"] == "single":
            var = f"s{slot['id']}"
            lines.append(f"{indent}let {var} = Logger::new(log.clone(), {slot['id']});")
            if slot["explicit_drop"]:
                lines.append(f"{indent}drop({var});")
        elif slot["kind"] == "pair":
            a, b = slot["ids"]
            var = f"p{a}"
            lines.append(
                f"{indent}let {var} = {pair_struct} {{ a: Logger::new(log.clone(), {a}), "
                f"b: Logger::new(log.clone(), {b}) }};"
            )
            if slot["explicit_drop"]:
                lines.append(f"{indent}drop({var});")
        elif slot["kind"] == "tuple":
            a, b = slot["ids"]
            var = f"t{a}"
            lines.append(f"{indent}let {var} = (Logger::new(log.clone(), {a}), Logger::new(log.clone(), {b}));")
            if slot["explicit_drop"]:
                lines.append(f"{indent}drop({var});")
        else:  # nested
            lines.append(f"{indent}{{")
            lines.extend(_emit_scope(slot["slots"], pair_struct, indent + "    "))
            lines.append(f"{indent}}}")
    return lines


def _simulate(slots, log):
    pending = []
    for slot in slots:
        if slot["kind"] == "nested":
            _simulate(slot["slots"], log)
        elif slot["kind"] == "single":
            if slot["explicit_drop"]:
                log.append(slot["id"])
            else:
                pending.append(slot)
        else:  # pair/tuple
            if slot["explicit_drop"]:
                a, b = slot["ids"]
                log.append(a)
                log.append(b)
            else:
                pending.append(slot)

    for slot in reversed(pending):
        if slot["kind"] == "single":
            log.append(slot["id"])
        else:
            a, b = slot["ids"]
            log.append(a)
            log.append(b)


@register("drop_order_sensitive")
def generate(rng, idx) -> Program:
    counter = [0]
    slots = _make_slots(rng, counter, depth=0, max_depth=2)

    pair_struct = ident("Pair", idx)
    body_lines = _emit_scope(slots, pair_struct)

    source = f"""#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

struct Logger {{
    log: Rc<RefCell<Vec<u32>>>,
    id: u32,
}}

impl Logger {{
    fn new(log: Rc<RefCell<Vec<u32>>>, id: u32) -> Self {{
        Logger {{ log, id }}
    }}
}}

impl Drop for Logger {{
    fn drop(&mut self) {{
        self.log.borrow_mut().push(self.id);
    }}
}}

struct {pair_struct} {{
    a: Logger,
    b: Logger,
}}

fn main() {{
    let log: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
    {{
{chr(10).join(body_lines)}
    }}
    let result = log.borrow().clone();
    println!("{{:?}}", result);
}}
"""

    log: list[int] = []
    _simulate(slots, log)
    expected_stdout = "[" + ", ".join(str(x) for x in log) + "]\n"

    return Program(
        name=f"drop_order_sensitive_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="drop_order_sensitive",
    )
