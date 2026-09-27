# CLIP_STATE_POP_ROOT_VIEWPORT_FRAMES_UNDERFLOW

`vello_common::clip::ClipState::pop_root_viewport` (`sparse_strips/vello_common/src/clip.rs:234`)
does `self.frames.pop().expect("filter clip stack underflow")` on its own `frames: Vec<ClipFrame>`
stack - a distinct stack from the `pop_layer`/`pop_clip` stacks already confirmed unguarded in
candidates 031 and 045. It's called from `vello_common::viewport::ViewportState::pop_root_viewport`
(`viewport.rs:110`), which itself first does an equally unchecked
`self.strip_generator_stack.pop().expect("root viewport stack underflow")` (`viewport.rs:114`)
immediately before. Candidate 045's `RESULT.md` explicitly flagged this pair as an open,
unverified lead worth checking independently.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static call-graph analysis: read every call site of `ViewportState::pop_root_viewport` (`vello_cpu::dispatch::single_threaded.rs:399`, `vello_hybrid::scene.rs:702`) and of `push_root_viewport` (`single_threaded.rs:357`, `scene.rs:629`), and the `CommandRecorder::push_layer`/`pop_layer` logic they're paired with (`record.rs`) | Manual reading, no build | Both call sites only invoke `pop_root_viewport()` from inside `match self.recorder.pop_layer() { PoppedLayer::Filter => ... }`; `recorder.pop_layer()` itself does `self.layer_stack.pop().unwrap()` (record.rs:307) *before* that match ever runs. Every `push_layer` call that carries `filter_data` unconditionally calls `viewport.push_root_viewport(..)` *before* `recorder.push_layer(..)`, and `PoppedLayer::Filter` is only ever returned for a layer that was itself pushed via `push_filter_layer` (i.e. one whose `frames`/`strip_generator_stack` push already happened). So `frames.len()` can only ever be >= the number of currently-open filter layers on `layer_stack`, never less - there is no code path where `recorder.pop_layer()` succeeds and returns `PoppedLayer::Filter` without a corresponding, already-pushed `frames` entry waiting to be popped. |
| Manual boundary-value construction via the real public API: `vello_cpu::RenderContext` - (a) a lone `pop_layer()` with no push at all, (b) one balanced `push_filter_layer()`/`pop_layer()` pair followed by one extra `pop_layer()`, (c) two `push_filter_layer()` calls followed by three `pop_layer()` calls (one extra after balanced nesting) - each call wrapped in `catch_unwind` | Release build, default (single-threaded) dispatcher, real crate via a standalone `repro046` binary depending only on `vello_cpu`'s/`vello_common`'s public API | All 3 cases panic, 100% of runs, always at `record.rs:307` (`called \`Option::unwrap()\` on a \`None\` value`, `CommandRecorder::pop_layer`'s own layer-stack underflow, i.e. candidate 031's bug) - never at `clip.rs:234` or `viewport.rs:114` |
| Same two of the three shapes, repeated against `vello_hybrid::Scene`'s equivalent public API (`push_filter_layer`/`pop_layer`) | Release build, `vello_hybrid` with `default-features = false` (no `wgpu`) | Both cases panic at `record.rs:307` as well, same message, never at `clip.rs:234`/`viewport.rs:114` |

All inputs use only the real, public entry points (`vello_cpu::RenderContext::new` +
`push_filter_layer`/`pop_layer`, and `vello_hybrid::Scene::new` + the same two methods) with no
fabricated internal state.

## Status

**Not reproducible.** The suspected unchecked pop in `ClipState::pop_root_viewport` (and its
sibling in `ViewportState::pop_root_viewport`) is unreachable through every real public entry
point checked this session: `CommandRecorder::pop_layer`'s own unrelated, already-panicking
layer-stack check (`record.rs:307`, the separately-confirmed candidate 031) always fires first for
any caller-side pop/push mismatch, before `pop_root_viewport` is ever reached. The static call-graph
analysis above additionally rules out the possibility of a caller somehow reaching
`PoppedLayer::Filter` without `frames`/`strip_generator_stack` already having a matching, unpopped
entry: every `push_layer` call that carries `filter_data` does the `viewport.push_root_viewport(..)`
side effect *before* `recorder.push_layer(..)` runs, and both are only ever undone together inside
`pop_layer` (`viewport.pop_root_viewport()` only runs when `recorder.pop_layer()` returned
`PoppedLayer::Filter`, which by construction can't happen without a preceding, still-unmatched
`push_root_viewport` call on the same stack traversal). This was verified with two escalating
approaches (static call-graph reasoning, then an actual boundary-value repro against both
`vello_cpu` and `vello_hybrid`'s real public API, 5 unbalanced-pop shapes total, each panicking
deterministically at the unrelated `record.rs:307` site instead) - a third tool (fuzzing/miri) was
judged unnecessary given the static analysis already rules the code path out structurally, not just
empirically for the inputs tried.

**Fully reproducible**: N/A (not applicable - `Status` is `Not reproducible`, not `Confirmed`).

## Class

N/A (not applicable - see `Status` above; no bug was confirmed).

## Root cause

Not a bug in the reachable sense: `ClipState::pop_root_viewport`'s `frames.pop()` and
`ViewportState::pop_root_viewport`'s `strip_generator_stack.pop()` are indeed unchecked (no
`Result`/no length check before popping), matching the same superficial shape as the already-fixed
candidates 031/045 - but unlike those two, this pair's only real callers
(`vello_cpu::dispatch::single_threaded::pop_layer` and `vello_hybrid::Scene::pop_layer`) never call
it except from the `PoppedLayer::Filter` arm of matching on `CommandRecorder::pop_layer()`'s return
value, and `CommandRecorder::pop_layer()` panics on its own, unrelated `layer_stack.pop().unwrap()`
(record.rs:307) for any unbalanced/extra pop before that arm is ever reached. Because every
`push_layer` call that pushes a filter layer does so by first calling
`viewport.push_root_viewport(..)` (an infallible operation - just `Vec::push` plus an `Affine`
multiply, no panicking paths found in it or in the `strip_generator`/clip-path generation code
between it and the later `recorder.push_layer(..)` call) and only afterward records the
corresponding `PoppedLayer::Filter`-tagged entry on `layer_stack`, the `frames`/
`strip_generator_stack` push always happens no later than the matching `layer_stack` push. This
means `frames.len()` (and `strip_generator_stack.len()`) can only ever be greater than or equal to
the number of currently-open filter layers recorded on `layer_stack` - never less - so
`pop_root_viewport`'s pop is unconditionally safe on every call actually reachable through
`pop_layer`. The unchecked `.expect(...)` calls are therefore currently dead code with respect to
underflow (they document an invariant that the surrounding code already enforces structurally, not
a live crash surface) - the surrounding code was not modified to add an explicit guard, since there
is nothing to fix without a real reachable path.

## Introduced by

N/A - no bug was confirmed to exist at this location under the current call graph, so there is
nothing to attribute to an introducing commit.

## The fix

N/A - `Status` is `Not reproducible`; no fix was attempted, and no `patch.diff` is included.

- Project test suite: N/A - no change was made to the source tree, so no before/after run was
  needed for this candidate (the main project's working tree was never modified).

## What's not done / open questions

- This candidate only checked the `frames.pop()` in `ClipState::pop_root_viewport` and its sibling
  `strip_generator_stack.pop()` in `ViewportState::pop_root_viewport`. It does not re-check
  `pop_layer`/`pop_clip`'s own underflows (candidates 031/045), which remain real, confirmed,
  independently-reachable bugs with their own fixes proposed (not applied to the tree, per this
  skill's "main tree always clean" rule).
- The static reasoning above is specific to the current call graph in this codebase revision
  (`75168314`). If a future change adds a new caller of `ViewportState::pop_root_viewport` /
  `ClipState::pop_root_viewport` that doesn't route exclusively through
  `CommandRecorder::pop_layer`'s `PoppedLayer::Filter` arm (e.g. a hypothetical direct
  `ClipState`/`ViewportState` API consumer, or a future MT-dispatcher filter-layer implementation -
  today's MT dispatcher explicitly does not support filters, see
  `multi_threaded.rs:364`, `unreachable!("filters are not supported by MT")`), this analysis would
  need to be redone for that new path.
- `vello_common`'s own crate-level doc explicitly states "This crate should not be used on its own,
  and you should instead use one of the renderers which use it" - so a hypothetical third-party
  crate directly constructing a `ClipState`/`ViewportState` and calling `pop_root_viewport()`
  without a matching push (bypassing `vello_cpu`/`vello_hybrid` entirely) was not investigated as a
  "real public entry point" for this candidate's purposes, consistent with how candidates 031/045
  scoped their own "reachable via a public entry point" language to `vello_cpu`'s/`vello_hybrid`'s
  actual public APIs, not `vello_common`'s.
