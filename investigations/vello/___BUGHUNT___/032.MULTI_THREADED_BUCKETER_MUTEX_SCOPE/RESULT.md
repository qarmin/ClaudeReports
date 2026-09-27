# MultiThreadedDispatcher holds the `bucketer` Mutex across the whole parallel rasterization region

`sparse_strips/vello_cpu/src/dispatch/multi_threaded.rs`'s `rasterize_with` (lines ~397-455) locks
`self.bucketer` (a `Mutex<CommandBucketer>`) once at the top of the function and keeps the
`MutexGuard` alive across `thread_pool.install(|| regions.update_par(...))` - i.e. across the
entire parallel per-region rasterization pass for a whole frame. Flagged by
`cargo clippy -- -W clippy::significant_drop_tightening`. The candidate report's own framing was
explicit that this is not memory-unsafe by itself, but asked whether any *other* code path ever
tries to lock `self.bucketer` during that window, which would serialize on this one lock for a
whole frame - no such second caller had been found yet when this candidate was picked up.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Type-system proof using the real public API (`vello_cpu::RenderContext`, the only owner of a `MultiThreadedDispatcher`) | `assert_not_impl_any!(vello_cpu::RenderContext: Sync)` (`static_assertions` crate), release build, run via `just test` against a pristine clone at the base commit | Compiles and runs clean: `RenderContext` is **not** `Sync`. As a cross-check, the inverse claim (`assert_sync::<vello_cpu::RenderContext>()`) was also tried and fails to compile with `` `(dyn vello_cpu::dispatch::Dispatcher + 'static)` cannot be shared between threads safely ``, tracing through `Box<dyn Dispatcher>` to `RenderContext` itself - confirming the positive check isn't vacuous. Reproduces deterministically, 2/2 runs (once via `just test`'s fresh clone, once more via a direct `cargo run --release` rerun) - this is a static, compile-time property, not something that can vary run-to-run. |
| `cargo clippy -p vello_cpu --features multithreading -- -W clippy::significant_drop_tightening` | Debug build, whole-crate lint pass | Confirms the lint fires exactly where the candidate report says (`multi_threaded.rs:398`, `bucketer` guard alive until the function's closing brace at line 457) - the underlying style observation is real. |
| Whole-workspace grep for every `.lock()` call and every `bucketer`/`MultiThreadedDispatcher` reference | Static, no build | `self.bucketer.lock()` is called from exactly one place in the entire workspace (`multi_threaded.rs:398`). The only other `.lock()` calls in the same file (lines 914/925/935) are on `MaybePresent<T>::value`, an unrelated generic `Arc<Mutex<T>>` handoff wrapper used for `alpha_storage`, not `bucketer`. `MultiThreadedDispatcher` itself is referenced only from its own module and from `render.rs` (construction/ownership); no other crate in the workspace touches it. No `unsafe impl Sync`/`unsafe impl Send` exists anywhere near `dispatch`/`RenderContext`/`bucketer`, and nothing wraps `RenderContext` in an `Arc` anywhere in the tree. |

## Status

**Not reproducible** (as a real contention/concurrency bug).

What was covered: (1) a compile-time proof, using the project's own real public API type, that
`RenderContext` - and therefore the `MultiThreadedDispatcher` it exclusively owns, and therefore
`bucketer`'s `Mutex` - can never be shared between two threads through safe Rust, because
`RenderContext` is not `Sync` (`Box<dyn Dispatcher>` only requires `Dispatcher: Debug + Send`, not
`Sync`, so the box itself isn't `Sync`); (2) confirmation that the flagged clippy lint does fire
exactly as described, i.e. the style observation itself is real; (3) an exhaustive, whole-workspace
grep confirming there is no second call site, no second owner, and no way to bypass the `!Sync`
type-system barrier (no `unsafe impl Sync`, no `Arc<RenderContext>` anywhere). No fuzzing/miri/ASAN
was attempted - this is not a memory-safety or crash-shaped suspicion, it's a "does a second caller
exist" question, and a static reachability/type-system check answers it directly and exhaustively
rather than needing a runtime tool.

**Fully reproducible**: N/A (no bug to reproduce - see Status; the *investigation itself*,
i.e. the type-system proof, is fully reproducible on demand via `just test` against a pristine
clone of the base commit, verified 2/2 runs, byte-identical output both times).

## Class

N/A (not Confirmed).

## Root cause

Not a bug. The `clippy::significant_drop_tightening` lint is a legitimate, narrowly-scoped style
observation: the `MutexGuard` for `bucketer` genuinely does stay alive longer than its last actual
read/write inside `rasterize_with` (it's read again at line ~448 to pass `&bucketer` into
`rasterize_region`, so the guard can't be dropped much earlier than it already is without
restructuring the function, but it does technically stay alive through the `thread_pool.install`
call purely because it's still in scope, not because anything inside that closure needs to
re-acquire it).

The suspected *consequence* - "any other code path that tries to lock `self.bucketer` during that
window would serialize on this single lock for a whole frame's rasterization" - does not hold,
because there is no other code path that could ever attempt it. `bucketer` is a private field of
`MultiThreadedDispatcher`, which is owned exclusively (via `Box<dyn Dispatcher>`) by exactly one
`RenderContext`. `RenderContext` is not `Sync` (transitively, because the `Dispatcher` trait object
is only bounded by `Debug + Send`, not `Sync`), so no two OS threads can ever hold a `&RenderContext`
(or reach into its dispatcher) at the same time through safe Rust - there is no `unsafe impl Sync`
anywhere in the crate that would re-enable this. Combined with the grep confirming `self.bucketer`
is locked from exactly one call site in the whole workspace, the "second caller serializes on this
lock" scenario the candidate raised is structurally impossible, not merely unobserved.

## Introduced by

Not determined - not worth a full `git blame`/bisect this session given Status is Not reproducible
(per SKILL.md step 5, this lookup is optional and skipped once a candidate resolves to a non-bug).

## The fix

No fix attempted or needed - Status is Not reproducible (no real contention exists to fix). The
underlying clippy lint (holding the guard slightly longer than its last use) is real but purely
cosmetic/style: fixing it would mean re-deriving `bucketer.width()`/`bucketer.rows().len()`/
`&bucketer.filter_paints` into locals before dropping the guard, then passing those locals (and a
raw reference for the `&bucketer` argument to `rasterize_region`, which is still needed inside the
parallel closure) - a purely internal refactor with no behavior change, out of scope for this
candidate since there is no reproduced defect to verify a fix against.

- Project test suite: N/A, no fix was applied. Baseline (`cargo test -p vello_cpu --release
  --features multithreading --lib --bins --tests`, at the investigated commit, no changes, run
  once as a sanity check): **87 passed; 0 failed; 0 ignored**. (`--doc` was excluded: the doctest
  target fails to build in `--release` mode regardless of this candidate - `--release` disables
  `debug_assertions`, and `vello_cpu::text_debug` unconditionally imports
  `glifo::atlas::GlyphCacheStats`, which `glifo` only exports under
  `#[cfg(all(debug_assertions, feature = "std"))]`; this is a pre-existing, unrelated build
  quirk of running the doctest harness in release mode, not something this candidate touches.)

## What's not done / open questions

- This candidate only checked whether a second caller *could* contend for `bucketer`'s lock. It
  did not attempt to actually fix the `clippy::significant_drop_tightening` lint itself (narrowing
  the guard's scope), since doing so with no observable behavior change and no reproduced defect
  to verify against would be an unverifiable, purely cosmetic patch - out of scope for what this
  candidate loop verifies (a reproduced bug, not a lint).
- The type-system argument (`RenderContext: !Sync`) is specific to the *current* trait bound on
  `Dispatcher` (`Debug + Send`, not `+ Sync`). If a future change added `Sync` to that bound (e.g.
  to support some other use case), this analysis would need to be redone - noted here so a future
  session touching that trait bound knows to re-check this spot.
