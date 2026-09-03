# RECORD_POP_LAYER_STACK_UNDERFLOW

`vello_common::record::CommandRecorder::pop_layer` (`sparse_strips/vello_common/src/record.rs:307`)
does `self.layer_stack.pop().unwrap()` with no check that a matching `push_layer` preceded it.
This is reachable from `vello_cpu`'s public `RenderContext::pop_layer` (`sparse_strips/vello_cpu/src/render.rs:575`)
and `vello_hybrid`'s public `Scene::pop_layer` with no arity/Result checking of its own and no
`# Panics` documentation, so any caller (e.g. a document-format translation layer with
unbalanced/malformed group nesting) that calls `pop_layer()` one time too many panics with a bare,
uninformative `Option::unwrap()` message.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: call the public `vello_cpu::RenderContext::pop_layer()` once with no preceding `push_layer()` call, via a standalone crate using only the public API | Release and debug builds, default features (single-threaded dispatcher) | Panics deterministically, 3/3 runs each build: `called \`Option::unwrap()\` on a \`None\` value` at record.rs:307 |
| Same, but with the `multithreading` feature enabled and `num_threads: 1` | Debug and release builds | Panics twice: first an immediate `expect("layer stack underflow")` in `dispatch/multi_threaded.rs`, then a second panic (the same record.rs unwrap) while the `MultiThreadedDispatcher`'s `Drop` impl processes the already-queued `PopLayer` task during unwind - `panic in a destructor during cleanup`, and the whole process **aborts** (SIGABRT / exit 134), not just a catchable panic. Reproduced consistently. |

Both approaches use the real, public entry point (`RenderContext::pop_layer` / `RenderContext::new_with`)
with no fabricated internal state - see "Fully reproducible" below.

## Status

**Confirmed**, reproduces 3/3 runs, both debug and release builds, both the default
single-threaded dispatcher and (with the `multithreading` feature) the multi-threaded dispatcher.
No layout/seed dependency - fully deterministic (a plain arity/logic bug, not memory-layout
dependent UB).

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls only `vello_cpu`'s real public API
(`RenderContext::new` + `RenderContext::pop_layer`) exactly the way an application/importer would
- there is no second participant or wire format involved here; the caller itself misusing this
exact public API (calling `pop_layer` without a matching `push_layer`) *is* the real external
input a document-format translation layer with unbalanced group nesting would produce, so there is
nothing to fake.

## Class

crash

## Root cause

`CommandRecorder::pop_layer` (`sparse_strips/vello_common/src/record.rs`) unconditionally pops
`self.layer_stack` and unwraps the result, assuming every `pop_layer()` call is preceded by a
matching `push_layer()`. Neither `vello_cpu::RenderContext::pop_layer` nor
`vello_hybrid::Scene::pop_layer` (the public entry points) validate this invariant or document it
as a panic condition before forwarding the call, so a caller with unbalanced group/layer nesting
(e.g. a malformed input document, or simply an application bug) gets an unqualified
`Option::unwrap()` panic with no indication of what went wrong.

The multi-threaded dispatcher (`vello_cpu::dispatch::multi_threaded::MultiThreadedDispatcher`)
makes this worse: `pop_layer()` first unconditionally enqueues a `RenderTaskType::PopLayer` render
task via `register_task`, and only afterwards checks `self.layer_depth.checked_sub(1)`, panicking
immediately with `"layer stack underflow"` if it underflows. Because the bad `PopLayer` task was
already enqueued before that immediate panic fires, if the `RenderContext` is later dropped while
unwinding from that panic (the ordinary case - nothing special has to be done to hit this), `Drop`
calls `flush()`, which replays the queued `PopLayer` task against the same `CommandRecorder`,
hitting the exact same `record.rs` unwrap a second time - **during** the unwind of the first panic.
Rust aborts the process outright in this situation (`panic in a destructor during cleanup`), so an
unbalanced `pop_layer()` call under multi-threaded rendering isn't just an uncaught panic, it's an
unrecoverable process abort that a `std::panic::catch_unwind` around the call cannot catch either.

## Introduced by

9c292860 "vello_cpu: A full rewrite of the frontend (#1701)" (2026-06-18) - this is where both the
unchecked `layer_stack.pop().unwrap()` in the (then `vello_cpu`-local) command recorder and the
multi-threaded dispatcher's enqueue-before-check ordering in `pop_layer` first appear. The file
was later moved from `vello_cpu/src/record.rs` to `vello_common/src/record.rs` unchanged in this
respect by dc23e938 / 93162ef3 ("Generalize the command recorder abstraction..." / "Move some code
from `vello_cpu` to `vello_common`").

## The fix

Three small, additive changes, no architecture change:

1. `vello_common::record::CommandRecorder::pop_layer`: replace the bare `.unwrap()` with
   `.expect("pop_layer called without a matching push_layer")` and add a `# Panics` doc comment.
   This keeps the existing panic-on-misuse contract (turning `pop_layer` into a `Result`-returning
   API would be a larger, API-breaking change out of scope for this bounded fix, and wouldn't by
   itself help the multi-threaded case below, since that dispatcher's `pop_layer` doesn't return
   a `Result` either and defers the real work to a later `flush()`), but replaces the confusing
   generic `Option::unwrap()` message with one that names the actual mistake.
2. `vello_cpu::RenderContext::pop_layer` and `vello_hybrid::Scene::pop_layer`: add a `# Panics`
   doc section stating the same contract, so it's no longer an undocumented panic on the public
   API surface.
3. `vello_cpu::dispatch::multi_threaded::MultiThreadedDispatcher::pop_layer`: reorder the
   underflow check (`checked_sub().expect(...)`) to run **before** `register_task`, so a mismatched
   `pop_layer()` call never enqueues the bad `PopLayer` task in the first place. This means the
   later `flush()`/`Drop` never replays it, eliminating the second, destructor-time panic and the
   resulting process abort - an unbalanced call under multi-threaded rendering is now a single,
   ordinary, `catch_unwind`-catchable panic, matching the single-threaded dispatcher's behavior.

This does **not** eliminate the panic itself for an unbalanced call - by design, both dispatchers
already treated this as an error condition worth panicking over (the multi-threaded one already
had its own explicit `expect("layer stack underflow")` before this fix), so silently tolerating an
extra `pop_layer()` call (e.g. by making it a no-op) would hide a real caller bug instead of
surfacing it, and was judged out of scope for a bounded fix without a maintainer decision on the
desired public API (panic vs. `Result`).

Verified: re-ran `repro/` against the patched tree - the single-threaded dispatcher now panics
with the new documented message (`pop_layer called without a matching push_layer`) instead of the
raw `Option::unwrap()` message, 3/3 runs; the multi-threaded dispatcher now panics exactly once
with the same message and the process survives `catch_unwind` cleanly (previously: two panics,
`SIGABRT`, exit code 134). See `output.txt` for the full before/after output of both variants.

Also checked one more input in the same class (per `CONFIRMATION.md` section 7): two `push_layer`
calls followed by three `pop_layer` calls (one extra pop after correctly-balanced nesting) hits
the identical code path and is fixed identically, since the bug is purely about stack-empty
detection, not about any specific call count.

- Project test suite: `cargo test -p vello_common --lib`: 159 passed / 0 failed before and after.
  `cargo test -p vello_cpu --lib`: 81 passed / 0 failed before and after.
  `cargo test -p vello_cpu --lib --features multithreading`: 87 passed / 0 failed before and after.
  `cargo test -p vello_hybrid --lib`: 122 passed / 0 failed before and after.
  (all four ran once against the pristine tree via `git stash`, then again with the patch applied
  via `git stash pop`, identical pass counts both times - no regressions.)

## What's not done / open questions

- Whether `pop_layer` should instead return a `Result`/an `Option` so callers (in particular a
  document-format translation layer that expects malformed input) can recover instead of the
  whole render being aborted by a panic is a public-API design decision for the maintainers, not
  attempted here.
- `vello_hybrid`'s own multi-threaded/scheduling code path (if any) wasn't specifically checked
  for the same enqueue-before-check ordering hazard the `vello_cpu` multi-threaded dispatcher had;
  worth a quick look in a future session.
- The `push_root`/`pop_root` stack in `RootTransforms` (`self.root_transforms.pop_root()`, called
  right after `dispatcher.pop_layer()` in both `RenderContext::pop_layer` and `Scene::pop_layer`)
  was not specifically audited for the same "no matching push" panic risk - it may have an
  identical unchecked-pop pattern, but by the time it's reached in the real call path,
  `dispatcher.pop_layer()` has already panicked on an unbalanced call before `pop_root()` is ever
  invoked, so it wasn't independently confirmed as reachable on its own.
