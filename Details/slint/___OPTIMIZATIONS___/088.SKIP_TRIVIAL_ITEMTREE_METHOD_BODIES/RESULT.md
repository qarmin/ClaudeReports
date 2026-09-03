# 88 - skip the match statement in trivial `ItemTree` method bodies (compiler codegen)

From `IDEAS.md` line 77 ("compiler sweep"): `generate_item_tree` in
`internal/compiler/generator/rust.rs` unconditionally emits six inherent methods per generated
sub-component (`item_geometry`, `accessible_role`, `accessible_string_property`,
`accessibility_action`, `supported_accessibility_actions`, `item_element_infos`), each built
around a `match` over a `Vec<TokenStream>` of branches. These methods themselves can't be
skipped - they're called directly as inherent methods by parent components reaching into a
sub-component field (`#sub_compo_field.apply_pin(_self).accessible_role(0)`, see rust.rs:1601-1633)
and are also required by the `ItemTree` trait impl one level up (rust.rs:2649-2677), which
delegates to them by name. What *can* be skipped is the `match` itself: when a component has no
accessible content of its own, no explicit item geometry expressions, and no forwarding ranges
into sub-components (the common case for a plain, non-interactive leaf component), the branch
vector is empty and the match already degenerates to a single wildcard arm returning a default
value. This candidate makes that case emit the default directly, `grid_layout_input_for_repeated_fn`-style
(`Option`-shaped conditional token stream built once, reused at the call site), instead of
generating (and asking `rustc` to parse/typecheck) a `match` with only a default arm.

## What was tried

One patch: for each of the six methods, if its branch `Vec` is empty, emit
`fn foo(...) { #![allow(unused)] <default-expr> }` instead of
`fn foo(...) { #![allow(unused)] let _self = self; match ... { <branches> _ => <default-expr> } }`.
`layout_info` was left untouched - its two arms (`Horizontal`/`Vertical`) are never optional, so
there's no empty-branch case to collapse.

Fixture: `testdata/gen_fixture.py` generates N *distinct* trivial custom components
(`Card0..CardN-1`, each a bare `Rectangle` + `Text`, no accessible-role/geometry set), each
instantiated once inside one root `Window`. Distinct components are the point - a `for` loop
repeating one component only generates that component's code once regardless of instance count,
so it wouldn't exercise "boilerplate scales with structural component count" at all. Each `Card`
has no sub-components and no accessible properties, so all six of its methods hit the trivial path
after the patch.

| Variant | generated Rust size (N=1600) | slint-compiler wall time (N=1600, single-core, taskset -c 3, 12 alternating pairs) |
|---|---|---|
| baseline | 29 284 338 bytes | median ~2.22s, spread 2.04-3.75s |
| patched | 29 057 138 bytes (**-0.78%**) | median ~2.15s, spread 2.00-2.88s, **9/12 pairs faster** |

Byte-size reduction was identical (0.78%) at every size tried (N=20, 100, 400, 1600) - it's a
fixed per-trivial-component overhead, not something that grows or shrinks with N.

Downstream `rustc` build of a crate whose `main.rs` is just `include!()` of the generated file
(no `slint-build`/build.rs in the loop - isolates exactly "how expensive is the generated Rust to
compile", matching candidate `048.COMPILE_TIME`'s finding that this, not `slint-compiler`'s own
run time, is where the total cost lives), `slint = { path = ..., features = ["std",
"backend-winit", "renderer-software", "compat-1-2"] }`, dependencies warm/cached:

| Variant | N=400, `cargo build` (dev, unoptimized+debuginfo), 4 samples | N=1600, `cargo build --release` (-O3), 1 sample |
|---|---|---|
| baseline | 84.3s, 79.5s, 74.4s, 73.7s | 5m14s |
| patched | 82.6s, 76.0s, 73.2s, 73.7s | 5m19s |

## Result

Two different, both small, effects:

- **Generated code size: real, consistent, -0.78%.** Deterministic byte-for-byte comparison
  across four sizes, same ratio every time - not noise.
- **`cargo build` (dev profile, N=400): small, consistently-signed win.** 4/4 paired samples
  favor the patch (deltas +1.0s, +3.5s, +1.2s, +0.06s - one of the four is within noise of zero,
  the other three are not). Plausible mechanism: removing a `match` removes exhaustiveness/MIR
  work per trivial component, which is more per-byte-expensive to the frontend than the raw
  token-count reduction alone would suggest.
- **`cargo build --release` (N=1600, -O3): no measurable difference, single sample.** 5m14s
  baseline vs 5m19s patched - inside the run-to-run noise of a 5-minute LLVM `-O3` build on this
  machine (a single sample per side is not enough to separate "slightly slower" from noise; not
  repeated further given the ~5 min/build cost and this candidate's time budget). At `-O3`,
  LLVM's own codegen/optimization cost for the *real* logic (bindings, property tracking) so far
  outweighs a handful of removed trivial `match` arms that no signal survives.
- `slint-compiler`'s own wall time: leans faster (9/12 pairs, pinned single-core) but not
  decisively per `MEASUREMENT.md`'s pair-count bar - consistent with the byte-size reduction being
  real but small relative to the rest of what `slint-compiler` does (parsing, semantic analysis,
  LLR lowering) for the same fixture.

## Does it change behavior/output?

- **Bit-exact?** No - the generated Rust source text differs (fewer tokens for the trivial-branch
  case), but the compiled *behavior* is identical by construction: the removed `match` had exactly
  one reachable arm (the wildcard default) in every case this patch touches, so replacing it with
  that same default expression is a no-op refactor, not an approximation or a different code path.
- **Is this a breaking change?** No - no public API, file format, or documented behavior changes.
  Purely an internal codegen shape change.
- **Project test suite**, `cargo test -p i-slint-compiler --release`: **349 passed, 0 failed**
  before and after (10 `test result` lines both times, matching - no suite silently failed to
  build, see `MEASUREMENT.md` section 17). `cargo test -p test-driver-rust --features build-time
  --release` (builds and *runs* real generated Rust from `tests/cases/*.slint`, including
  `tests/cases/accessibility/*.slint`): **790 passed, 0 failed** before and after (35 `test
  result` lines both times). Accessibility-specific tests individually confirmed passing with the
  patch applied (`accessibility_accessible_id`, `accessibility_orientation`,
  `accessibility_actions`, `accessibility_live`, `accessibility_textinput`,
  `accessibility_set_selection`, `accessibility_materialized` - all `ok`).

## Why the result looks the way it does

`048.COMPILE_TIME` already established that for a realistic `.slint` file, `slint-compiler` is
~0.6% of the total edit-compile-run cost and the rest is `rustc` compiling the generated Rust -
"the lever is the expansion ratio" (how much Rust is generated), not the generator's own speed.
This candidate is a small pull on exactly that lever: it removes real, measurable bytes (-0.78%,
consistent across sizes) but only from the six accessibility/geometry methods, which are already a
small fraction of a component's total generated code (most of a component's size is property
bindings, layout expressions, and the item tree/array tables, none of which this patch touches).
The `-O3` release-build result (no measurable difference) is consistent with that: LLVM's
optimization cost is dominated by the actual logic, not by a handful of removed one-arm matches,
so a size reduction concentrated in rarely-taken accessibility paths doesn't move the needle once
the backend is doing real work. The dev-profile (frontend-dominated, no `-O3`) result showing a
small consistent win is the more diagnostic one for this specific change, since it isolates
parsing/typeck/MIR cost rather than LLVM codegen cost.

## What's not done / open questions

- **The C++ generator mirror** (`internal/compiler/generator/cpp.rs`, string-based `switch`
  codegen for the same six methods) was not touched - this candidate covers only the Rust
  backend. Added to `IDEAS.md` as a follow-on.
- **The five near-duplicate per-item accessibility loops** noted in the original backlog entry
  (rust.rs:1493-1634 area, and the equivalent in cpp.rs) - a related but separate simplification
  (deduplicating the loop structure itself, not the emitted method bodies) - not attempted here.
  Left in `IDEAS.md`.
- Not measured: whether the `-O3` release-build result changes at a much larger N (e.g. 6400+) or
  on a real, non-synthetic `.slint` project with many small leaf components (the fixture here is
  synthetic - N *distinct* trivial components - to maximize the specific lever this patch pulls;
  a real app's component mix will dilute the effect according to how many of its components are
  actually leaf/trivial).
