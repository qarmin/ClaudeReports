# VELLO_HYBRID_SCHEDULER_ROUND_DEADLOCK_PANIC

`sparse_strips/vello_hybrid/src/schedule/round.rs:91-124`, `Rounds::bind_at_or_after`. This
linearly advances `point.round` looking for a scheduling round with a compatible texture binding,
guarded only by `if point.round > MAX_ROUNDS (40_000) { panic!("possible deadlock in scheduler
detected") }`. The surrounding comment admits this bound is unproven ("this should be more than
enough for any sane scene... it should never be possible to trigger an endless loop" / "TODO: Turn
this into an error once we have refactored error handling"), which is exactly the shape of
suspect this skill exists to check: is the bound actually load-bearing, or can a real scene defeat
it?

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `schedule/mod.rs`'s own doc comment ("Batching into rounds") and its existing test suite (`schedule_tests.rs`) for how round count scales with scene shape | N/A (static reading) | Found the project's own `nested_children_spilled` test already demonstrates *non-batching* growth: a ternary layer tree under a tiny (8x8) texture needs 21/39/201/363 rounds at depths 3-6 - i.e. round count scales roughly linearly with layer count once atlas space is small enough to defeat page reuse |
| In-crate debug-build probe test (`cargo test --release --lib`), reusing the crate's own internal `SceneCase`/`ScheduledCase` test harness with `add_tree(depth, children=3)` and `texture_size = 8x8`, `max_textures = 1000` | depth=9 -> 9,841 layers / 16,401 rounds (no panic); depth=10 -> 29,524 layers / 29,523 rounds (no panic); depth=11 -> 88,573 layers -> **panics** at `round.rs:106` ("possible deadlock in scheduler detected") | Confirmed the panic is reachable from ordinary (non-adversarial-looking) scene shapes: a sufficiently large nested/branching plain-layer tree (no filters/blends needed) with a small configured intermediate-texture size is enough |
| Standalone `repro/` crate driving the *real* public `vello_hybrid::Renderer::render` wgpu entry point end-to-end, via a real (software) Vulkan device (SwiftShader), with a `Scene` built purely through the crate's own public `push_layer`/`pop_layer`/`fill_rect`/`set_paint` API (ternary tree, depth 11, `LayersConfig::min_texture_size = 8x8`) | `cargo run --release` against a pristine clone at base_commit, repeated 4/4 times | Panics identically 4/4 times: `thread 'main' panicked at .../schedule/round.rs:106:17: possible deadlock in scheduler detected` |

## Status

**Confirmed**. Reproduces deterministically 4/4 runs (release build) via the real public wgpu
render entry point driven by a real (software) Vulkan device, plus 1/1 via the crate's own
internal test harness at a smaller scale (depth 9-10 already show the round count scaling toward
the limit; depth 11 crosses it).

**Fully reproducible**: **Yes**. `repro/` builds a `Scene` using only the crate's own public
recording API (`push_layer`/`pop_layer`/`set_paint`/`fill_rect`) - the same kind of input a real
application produces (e.g. an SVG/PDF importer or a procedurally generated scene with many nested
groups/clips) - and feeds it into the real, unmodified `vello_hybrid::Renderer::render` public
entry point, running against a real `wgpu::Device`/`Queue` obtained from a real (software) Vulkan
adapter (Google SwiftShader, `AdapterInfo { name: "SwiftShader Device (Subzero)", device_type: Cpu,
backend: Vulkan }` - see this workspace's `README.md` for how this sandbox already has this ICD
available). There is no second participant to fake here (this is a single-process scene-to-render
pipeline, not a client/server or wire protocol), so `CONFIRMATION.md` section 11's bar is
satisfied by using the crate's own real public API and a real GPU device throughout, rather than
hand-constructing the scheduler's internal `Scene`/`Rounds` state directly.

## Class

crash (unconditional `panic!`, reachable from ordinary public-API usage, not a logic error in the
rendered output)

## Root cause

`Rounds::bind_at_or_after` looks for the first round, starting at `point.round`, whose
`RoundBindings` (the even/odd intermediate-texture page pair already committed to that round) is
compatible with the operation's own requirement; if incompatible, it advances to the next round
and retries, `round += 1` per failed attempt with no bound other than a hardcoded
`MAX_ROUNDS = 40_000` panic guard.

The scheduler's own documented design ("Batching into rounds" in `schedule/mod.rs`) is
monotonic/greedy: once the cursor advances past a round, it never revisits it, and it never
allocates additional intermediate-texture pages purely to improve batching (by design, to keep
memory usage minimal). This means the number of *rounds* a scene needs is essentially proportional
to the number of layer operations whose required texture-page pair conflicts with whatever is
already bound in the current round - which, as the project's own `nested_children_spilled` test
already shows, scales close to 1:1 with the total layer count once the configured intermediate
texture size is small enough that atlas pages can't be shared across sibling/nested layers. A
scene with tens of thousands of layers (a large nested/branching layer graph - deeply nested
groups, clips, or opacity layers - which is not inherently pathological input, just large) combined
with a small `min_texture_size`/`max_texture_size` (a supported, documented configuration point
intended for e.g. mobile GPUs) is therefore enough to legitimately need more than 40,000 rounds,
at which point the scheduler panics instead of reporting a normal, catchable error.

## Introduced by

c3343862 "vello_hybrid: A complete rewrite of the core logic (#1759)" (2026-07-24) - this is the
commit that introduced `schedule/round.rs` (and the whole current scheduler) in its present form;
`git log --follow` on this file has no earlier history, and the panic and its `MAX_ROUNDS` guard
were both already present in that commit's own version of the file.

## The fix

Turned the panic into the `RenderError` the surrounding code's own `TODO` comment already
anticipated ("TODO: Turn this into an error once we have refactored error handling" - the error
handling this refactor needed was already in place, every call site was already inside a function
returning `Result<_, RenderError>` or one call away from one):

- Added `RenderError::SchedulingLimitExceeded { max_rounds: usize }` (`lib.rs`).
- `Rounds::bind_at_or_after` now returns `Result<SchedulePoint, RenderError>`, returning
  `Err(RenderError::SchedulingLimitExceeded { max_rounds: MAX_ROUNDS })` instead of panicking once
  `point.round` exceeds `MAX_ROUNDS` (`schedule/round.rs`).
- Propagated this `Result` up through `Rounds::build_draw`, `Scheduler::push_draws`,
  `Scheduler::compose_simple_layer`, and `Scheduler::compose_layer`/`schedule_layer`/
  `schedule_root` (all of which already returned `Result<_, RenderError>` or were one hop away
  from a caller that did) via `?` (`schedule/mod.rs`).
- Updated the existing unit tests in `schedule/round.rs` that called `bind_at_or_after` directly
  to `.unwrap()` the new `Result` (behavior unchanged for all of them, since none of those tests
  approach the round limit).
- Added a new regression test (`schedule::schedule_tests::deadlock_probe`) asserting that the
  depth-11 ternary-tree scene now returns `Err(RenderError::SchedulingLimitExceeded { max_rounds:
  40_000 })` instead of panicking.

This is sufficient because it doesn't just patch over the one specific scene shape that was found:
it fixes the actual mechanism (an unconditional `panic!` on a condition that's provably reachable
from within-spec scene sizes/configurations) by making the *existing* safety guard's job (bounding
worst-case scheduling work) surface as a normal, catchable `RenderError` - exactly like every
other resource-limit condition in this crate (`IntermediateTextureError::LimitReached`,
`TooLarge`, etc.) - rather than removing or loosening the guard itself, which would only turn a
clean panic into a silent unbounded loop for the same inputs.

- Project test suite (`cargo test --lib`, `vello_hybrid` crate): **122 passed, 0 failed** before
  the fix (with the new `deadlock_probe` test not yet added) -> **123 passed, 0 failed** after (122
  pre-existing tests unaffected + the new regression test, which now observes a clean `Err` where
  it previously observed a panic). Also verified `cargo check --all-features` (including the
  `webgl` feature, which has its own `Schedule::try_new` call site in `render/webgl/mod.rs`)
  still compiles cleanly with this change.

## What's not done / open questions

- The fix converts the panic into an error but does not change the underlying round-count growth
  behavior itself - a caller that doesn't set `LayersConfig::max_textures`/leaves texture sizes at
  their (larger) defaults is very unlikely to hit this in practice; this is about giving callers
  who do configure small textures (or otherwise end up with adversarially large/branchy scenes) a
  recoverable error instead of a hard crash, not about making arbitrarily large scenes render
  successfully.
- Whether `MAX_ROUNDS = 40_000` itself is the right threshold is out of scope for this fix; that's
  a product/performance tradeoff for the maintainers (a lower threshold surfaces the error sooner
  at the cost of rejecting some large-but-legitimate scenes; a higher one delays the check for
  more pathological cases at the cost of more wasted CPU time before erroring out).
- Did not check whether other iterative bounded-by-a-panic loops exist elsewhere in the scheduler
  (`schedule/allocate.rs`, `schedule/cursor.rs`) with a similar "should never happen but just in
  case" panic guard - this session only investigated the one flagged by the candidate.
