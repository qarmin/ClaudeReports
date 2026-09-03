# CLIP_STATE_POP_CLIP_STACK_UNDERFLOW

`sparse_strips/vello_common/src/clip.rs` has two `pop_clip` methods that both do a bare
`self.<stack>.pop().expect("clip stack underflowed")` with no check that a matching `push_clip`
preceded the call: `ClipContext::pop_clip` (line 131) and `ClipState::pop_clip` (line 276). Both
are reachable from `vello_cpu`'s fully public, undocumented `RenderContext::pop_clip_path()`
(`sparse_strips/vello_cpu/src/render.rs:740`) - through the single-threaded dispatcher
(`dispatch/single_threaded.rs` -> `vello_common::viewport::ViewportState::pop_clip` ->
`ClipState::pop_clip`) and through the multi-threaded dispatcher
(`dispatch/multi_threaded.rs:716-718`, which holds its own `ClipContext` field directly and calls
`ClipContext::pop_clip`) - as well as from `vello_hybrid::Scene::pop_clip_path`. Same bug class as
already-confirmed candidate 031 (`RECORD_POP_LAYER_STACK_UNDERFLOW`, `CommandRecorder::pop_layer`)
but a distinct struct/call path (the clip stack, not the layer stack), not covered by that
candidate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: call the public `vello_cpu::RenderContext::pop_clip_path()` once with no preceding `push_clip_path()`, via a standalone crate using only the public API, forcing the single-threaded dispatcher (`RenderSettings { num_threads: 0, .. }`) | Debug and release builds, `multithreading` feature compiled in but single-threaded dispatcher explicitly selected | Panics deterministically, 3/3 runs each build: `pop_clip called without a matching push_clip` (after fix) / `clip stack underflowed` (before fix) at `clip.rs:276` (`ClipState::pop_clip`) |
| Same, but forcing the multi-threaded dispatcher (`RenderSettings { num_threads: 1, .. }`) | Debug and release builds, `multithreading` feature | Panics deterministically, 3/3 runs each build, at `clip.rs:131` (`ClipContext::pop_clip`, reached directly since `MultiThreadedDispatcher` holds its own `ClipContext` and calls `pop_clip()` on it without going through `ClipState`) |
| Same bug class, different input: two `push_clip_path()` calls followed by three `pop_clip_path()` calls (one extra pop after correctly-balanced nesting), default `RenderContext::new()` | Release build | Panics deterministically, 3/3 runs, same code path/message as above |

All three inputs use only the real, public entry point (`RenderContext::new`/`new_with` +
`push_clip_path`/`pop_clip_path`) with no fabricated internal state - see "Fully reproducible"
below.

Note: an initial attempt to force the single-threaded dispatcher by simply calling
`RenderContext::new()` in a binary built with the `multithreading` Cargo feature enabled
accidentally exercised the multi-threaded dispatcher instead, because `RenderSettings::default()`
sets `num_threads` to `available_parallelism()` (not `0`) whenever that feature is compiled in.
This was caught by reading the panic backtrace (it named
`MultiThreadedDispatcher::pop_clip_path`) and fixed by passing `RenderSettings { num_threads: 0,
.. }` explicitly for the single-threaded variant - noted here so a future session doesn't repeat
the same false assumption about which dispatcher `RenderContext::new()` selects once
`multithreading` is enabled.

## Status

**Confirmed**, reproduces 3/3 runs for all three exercised inputs, in both debug and release
builds, under both the default single-threaded dispatcher and (with the `multithreading` feature)
the multi-threaded dispatcher. No layout/seed dependency - fully deterministic (a plain
arity/logic bug, not memory-layout-dependent UB). Also re-verified by an independent full
before/after run against a separate pristine clone (see `output.txt`), 3/3 runs each side.

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls only `vello_cpu`'s real public API
(`RenderContext::new`/`new_with` + `push_clip_path`/`pop_clip_path`) exactly the way an
application/importer would - there is no second participant or wire format involved here; the
caller itself misusing this exact public API (calling `pop_clip_path()` without, or one too many
times beyond, a matching `push_clip_path()`) *is* the real external input a document-format
translation layer (SVG/PDF/HTML-canvas clip-stack replay) with unbalanced clip nesting would
produce, so there is nothing to fake.

## Class

crash

## Root cause

Both `ClipContext::pop_clip` and `ClipState::pop_clip` (`sparse_strips/vello_common/src/clip.rs`)
unconditionally pop their respective stack (`clip_stack: Vec<ClipData>` /
`raw_clips: Vec<RawClip>`) and unwrap the result, assuming every `pop_clip()` call is preceded by
a matching `push_clip()`. Neither `vello_cpu::RenderContext::pop_clip_path` nor
`vello_hybrid::Scene::pop_clip_path` (the public entry points that forward to these, via
`ViewportState::pop_clip`/`ClipState::pop_clip` on the single-threaded path, or directly via a
dispatcher-owned `ClipContext` on the multi-threaded path) validated this invariant or documented
it as a panic condition before this fix, so a caller with unbalanced clip nesting (a malformed
input document, or simply an application bug) got an unqualified, undocumented
`Option::unwrap()`-style panic (`"clip stack underflowed"`) with no indication in the public API
docs that this could happen.

Unlike candidate 031's `pop_layer`/`MultiThreadedDispatcher` interaction, there is no
enqueue-before-check task-queue hazard here: `MultiThreadedDispatcher::pop_clip_path` calls
`flush_tasks()` then `self.clip_context.pop_clip()` directly (no deferred `RenderTaskType` is
enqueued for a clip pop the way there is for a layer pop), so an unbalanced call under
multi-threaded rendering is an ordinary, single, `catch_unwind`-catchable panic - not a process
abort. This was directly verified (see `output.txt` and the "What was tried" table above): both
dispatchers panic exactly once per call, with no second panic during unwind/`Drop`.

## Introduced by

5d6bf90d "Introduce a new clipping algorithm for non-layer based clipping (#1203)" (2025-11-15) -
this is where `ClipContext::pop_clip`'s unchecked `self.clip_stack.pop().expect("clip stack
underflowed")` first appears (`git log -S "clip stack underflowed"` on `clip.rs`). The second,
sibling occurrence - `ClipState::pop_clip`'s `self.raw_clips.pop().expect("clip stack
underflowed")` - was added later by 9c292860 "vello_cpu: A full rewrite of the frontend (#1701)"
(2026-06-18), which introduced the `ClipState` wrapper (then still living in `vello_cpu`) around
`ClipContext` to support multiple viewports/filter layers. Both occurrences were later moved
unchanged (in this respect) from `vello_cpu` to `vello_common` by 93162ef3 "sparse_strips: Move
some code from `vello_cpu` to `vello_common` (#1744)".

## The fix

Three small, additive changes, no architecture change (same shape as candidate 031's fix):

1. `vello_common::clip::ClipContext::pop_clip` and `vello_common::clip::ClipState::pop_clip`:
   replace the bare `.expect("clip stack underflowed")` with
   `.expect("pop_clip called without a matching push_clip")` and add a `# Panics` doc comment to
   each. This keeps the existing panic-on-misuse contract (turning `pop_clip` into a
   `Result`-returning API would be a larger, API-breaking change out of scope for this bounded
   fix), but replaces the confusing generic message with one that names the actual mistake.
2. `vello_cpu::RenderContext::pop_clip_path` and `vello_hybrid::Scene::pop_clip_path`: add a
   `# Panics` doc section stating the same contract, so it's no longer an undocumented panic on
   the public API surface.

No task-queue reordering fix (candidate 031's third change) was needed here, since - as noted
under "Root cause" above - `pop_clip_path` never enqueues a deferred task before checking the
stack; there is no equivalent double-panic/process-abort hazard to fix.

This does **not** eliminate the panic itself for an unbalanced call - by design this is already
treated as an error condition worth panicking over (same reasoning as candidate 031); silently
tolerating an extra `pop_clip_path()` call (e.g. by making it a no-op) would hide a real caller
bug instead of surfacing it, and was judged out of scope for a bounded fix without a maintainer
decision on the desired public API (panic vs. `Result`).

Verified: re-ran `repro/` against the patched tree - both dispatchers, plus the "extra pop after
balanced nesting" input, now panic with the new documented message
(`pop_clip called without a matching push_clip`) instead of the raw `clip stack underflowed`
message, 3/3 runs each. See `output.txt` for the full before/after output of all three inputs,
captured against an independent pristine/patched clone (not just the working copy used to develop
the fix).

Also checked one more input in the same class (per `CONFIRMATION.md` section 7, and included
directly in `repro/`): two `push_clip_path` calls followed by three `pop_clip_path` calls (one
extra pop after correctly-balanced nesting) hits the identical code path and is fixed identically,
since the bug is purely about stack-empty detection, not about any specific call count.

- Project test suite (run in a separate worktree used to develop/verify the fix, reverted before
  this candidate folder was written - see "the one rule that matters most" in `SKILL.md`):
  `cargo test -p vello_common --lib`: 159 passed / 0 failed, both before and after the patch.
  `cargo test -p vello_cpu --lib`: 81 passed / 0 failed, both before and after.
  `cargo test -p vello_cpu --lib --features multithreading`: 87 passed / 0 failed, both before and
  after.
  `cargo test -p vello_hybrid --lib`: 122 passed / 0 failed, both before and after.
  (all four ran once against the pristine tree, then again with the patch applied via `git stash`/
  `git stash pop`, identical pass counts both times - no regressions.)

## What's not done / open questions

- Whether `pop_clip_path` should instead return a `Result`/an `Option` so callers (in particular a
  document-format translation layer that expects malformed input) can recover instead of the
  whole render being aborted by a panic is a public-API design decision for the maintainers, not
  attempted here (same open question candidate 031 left for `pop_layer`).
- `ClipState::pop_root_viewport` (`self.frames.pop().expect("filter clip stack underflow")`,
  `sparse_strips/vello_common/src/clip.rs`) has the exact same unchecked-pop shape for the
  filter-layer viewport stack, but was not independently confirmed as reachable/exploitable via a
  public entry point this session - worth a quick follow-up look.
- `vello_hybrid`'s own dispatch/scheduling code path for `pop_clip_path` was read (it forwards
  directly to the same shared `vello_common::clip`/`viewport` code, no separate task-queue logic
  of its own for clips) but not independently exercised with a standalone `vello_hybrid` repro -
  the `vello_cpu` repro already demonstrates the shared root cause.
