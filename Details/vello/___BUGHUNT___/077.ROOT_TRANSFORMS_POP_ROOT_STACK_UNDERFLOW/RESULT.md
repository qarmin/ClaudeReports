# ROOT_TRANSFORMS_POP_ROOT_STACK_UNDERFLOW

`vello_common::transforms::RootTransforms::pop_root` (`sparse_strips/vello_common/src/transforms.rs:126`)
does a bare `self.transforms.pop();` with no check that a matching `push_root` preceded the call -
unlike every sibling stack in this codebase (`CommandRecorder::pop_layer`'s `layer_stack`,
`ClipContext`/`ClipState::pop_clip`'s clip stacks, `ClipState::pop_root_viewport`'s `frames`), this
one doesn't even `.unwrap()`/`.expect()` the popped value - `Vec::pop()`'s `None` on an
already-empty stack is silently discarded, so `pop_root` itself never panics on underflow. The
delayed symptom would instead be the *next* call to `root_transform()` (`transforms.rs:105`,
`self.transforms.last().expect("root transform stack should never be empty")`) panicking on an
unrelated, later operation. `push_root`/`pop_root` are called, unconditionally, on *every*
`push_layer`/`pop_layer` call (not just filter layers) from `vello_cpu::RenderContext::push_layer`/
`pop_layer` (`render.rs:504`/`577`) and `vello_hybrid::Scene::push_layer`/`pop_layer` (`scene.rs:627`/
`704`) - candidate 046 already flagged this stack as an open, unverified lead structurally similar
to its own (already-ruled-out) `ClipState::pop_root_viewport` finding.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static call-graph analysis: read every call site of `RootTransforms::push_root`/`pop_root` (`vello_cpu::render.rs:504`/`577`, `vello_hybrid::scene.rs:627`/`704`) and the `push_layer`/`pop_layer` dispatcher/recorder logic they're paired with (`record.rs`, `dispatch/single_threaded.rs`, `dispatch/multi_threaded.rs`) | Manual reading, no build | `push_root` is called unconditionally at the top of every `push_layer` (filter or not), immediately before `recorder.push_layer`/`dispatcher.push_layer` (which unconditionally pushes onto `layer_stack`/increments `layer_depth`); `pop_root` is called unconditionally at the end of every `pop_layer`, immediately *after* `dispatcher.pop_layer()`/`recorder.pop_layer()` returns (which panics first - bare `.unwrap()` at record.rs:307 for the single-threaded path and `CommandRecorder`, or `.expect("layer stack underflow")` at multi_threaded.rs:584 for the multi-threaded dispatcher's own `layer_depth` counter - on any unbalanced/extra pop). Since both stacks are pushed together (push_root before the layer push) and popped together (dispatcher/recorder pop before pop_root, and only reached if that pop didn't panic), `root_transforms.len()` can only ever equal `layer_stack.len() + 1` (the permanent initial `Affine::IDENTITY`) after every call - never less - so `pop_root()` is never reached with fewer than 2 elements left through this API. |
| Manual boundary-value construction via the real public API: 9 unbalanced-`pop_layer()` shapes - `vello_cpu::RenderContext` under the single-threaded dispatcher (lone extra pop with no push; balanced regular `push_blend_layer`/pop then one extra pop; two pushes/three pops; balanced filter push/pop then one extra pop) and the multi-threaded dispatcher (`num_threads: 1`; lone extra pop; balanced regular push/pop then one extra pop), plus `vello_hybrid::Scene` (lone extra pop; balanced regular push/pop then one extra pop; balanced filter push/pop then one extra pop) - each run in its own subprocess (`repro/src/main.rs`, `--case N`) so the multi-threaded dispatcher's own known process-abort behavior (see candidate 031) never swallows a later case's signal | Release build, real crate via a standalone `repro077` binary depending only on `vello_cpu`'s/`vello_hybrid`'s/`vello_common`'s public API, 3 full repeat runs of all 9 cases | All 9 cases panicked (or, for the two multi-threaded cases, aborted the process with SIGABRT/exit 134) every one of 3/3 runs, always at `record.rs:307` (`called \`Option::unwrap()\` on a \`None\` value`, `CommandRecorder::pop_layer`) or `multi_threaded.rs:584` (`"layer stack underflow"`, `MultiThreadedDispatcher::pop_layer`'s own `layer_depth` check) - never at `transforms.rs:105`/`126` (`RootTransforms`). See `output.txt` for the full captured run. |

Both approaches use only the real, public entry points (`vello_cpu::RenderContext::new_with` +
`push_blend_layer`/`push_filter_layer`/`pop_layer`, and `vello_hybrid::Scene::new` + the same two
methods) with no fabricated internal state - see "Fully reproducible" below.

A third tool (fuzzing/miri) was judged unnecessary: the static call-graph analysis already rules
the code path out structurally (not just empirically for the inputs tried), and the empirical
9-case/3-run sweep found zero exceptions to that structural argument.

## Status

**Not reproducible.** The suspected unguarded pop in `RootTransforms::pop_root` is unreachable
through every real public entry point checked this session: `CommandRecorder::pop_layer`'s own
unrelated, already-panicking layer-stack check (`record.rs:307`, the separately-confirmed candidate
031) or the multi-threaded dispatcher's own `layer_depth` underflow check (`multi_threaded.rs:584`)
always fires first for any caller-side `push_layer`/`pop_layer` mismatch, before `pop_root()` is
ever reached. The static call-graph analysis above additionally rules out the possibility of a
caller reaching `pop_root()` with an out-of-sync stack some other way: `push_root` and the
`layer_stack`/`layer_depth` push always happen together (in that order) inside `push_layer`, and
`pop_root` is only ever reached after the corresponding `layer_stack`/`layer_depth` pop already
succeeded without panicking - so `root_transforms.len()` can never fall below `layer_stack.len() +
1` through this API. This was verified with two escalating approaches (static call-graph reasoning,
then a 9-case boundary-value repro against both `vello_cpu` and `vello_hybrid`'s real public API,
run 3 times each), matching (and extending, to non-filter layers and the multi-threaded dispatcher)
the same reasoning candidate 046 used to rule out the structurally similar
`ClipState::pop_root_viewport`/`ViewportState::pop_root_viewport` finding. No layout/seed dependency
was found or expected (this is a plain arity/logic question, not memory-layout-dependent UB), and
the 3/3 repeat-run count was deterministic and identical each time.

**Fully reproducible**: N/A (not applicable - `Status` is `Not reproducible`, not `Confirmed`).

## Class

N/A (not applicable - see `Status` above; no bug was confirmed).

## Root cause

Not a bug in the reachable sense, for the same structural reason candidate 046 found for the
sibling `ClipState`/`ViewportState::pop_root_viewport` stacks: `RootTransforms::pop_root`'s
`self.transforms.pop()` is indeed unguarded (no length check, and it doesn't even `.unwrap()`/
`.expect()` the popped `Option` the way every sibling stack in this codebase does - it silently
discards a `None`), matching the superficial shape described in the candidate. But its only two
real callers (`vello_cpu::RenderContext::pop_layer` and `vello_hybrid::Scene::pop_layer`) both call
it unconditionally at the very end of `pop_layer`, *after* the underlying dispatcher's/recorder's
own layer-stack pop has already run and succeeded. That underlying pop
(`CommandRecorder::pop_layer`'s `layer_stack.pop().unwrap()` for the single-threaded path and
`vello_hybrid`, or `MultiThreadedDispatcher::pop_layer`'s `layer_depth.checked_sub(1).expect(...)`
for the multi-threaded dispatcher) panics first on any unbalanced/extra `pop_layer()` call, so
`RootTransforms::pop_root()`'s own line is never reached in that case. Because
`push_root`/`push_layer`'s underlying stack push always happen together (in `push_layer`, `push_root`
runs first, then `recorder.push_layer`/`dispatcher.push_layer`, both unconditional - regardless of
whether the layer is a filter layer, unlike the `frames`/`strip_generator_stack` stacks candidate 046
checked, which are only pushed for filter layers), `root_transforms.len()` tracks
`layer_stack.len() + 1` (the permanent initial identity transform) exactly, at every point reachable
through the public API. The unguarded `self.transforms.pop()` is therefore currently dead code with
respect to underflow (it documents an invariant the surrounding code already enforces structurally,
via the earlier-firing, already-known-panicking check in candidate 031, not a live, independently
reachable crash surface) - the surrounding code was not modified to add an explicit guard, since
there is nothing to fix without a real reachable path.

## Introduced by

N/A - no bug was confirmed to exist at this location under the current call graph, so there is
nothing to attribute to an introducing commit.

## The fix

N/A - `Status` is `Not reproducible`; no fix was attempted, and no `patch.diff` is included.

- Project test suite: N/A - no change was made to the source tree, so no before/after run was
  needed for this candidate (the main project's working tree was never modified; the digging above
  was done in a separate `git worktree` at `/tmp/vello-bughunt-077`, discarded after use).

## What's not done / open questions

- This candidate only checked whether an unbalanced `push_layer`/`pop_layer` call sequence (via
  the real public API, both dispatchers, both `vello_cpu` and `vello_hybrid`) can reach
  `RootTransforms::pop_root` with an empty/too-short stack. It does not re-check `pop_layer`/
  `pop_clip`'s own underflows (candidates 031/045), which remain real, confirmed, independently
  reachable bugs with their own proposed fixes (not applied to the tree, per this skill's "main
  tree always clean" rule).
- The static reasoning above is specific to the current call graph in this codebase revision
  (7516831426fb70f36d62c521c5477f6a7265b36a). If a future change adds a code path that calls
  `RootTransforms::pop_root()` other than from the tail end of `pop_layer` (after the dispatcher's/
  recorder's own layer-stack pop already succeeded), or that calls `push_root()` without the
  matching `layer_stack`/`layer_depth` push happening in the same operation, this analysis would
  need to be redone for that new path.
- Once candidate 031's `CommandRecorder::pop_layer` underflow (and the multi-threaded dispatcher's
  own `layer_depth` check) is ever changed to *not* panic on an unbalanced call (e.g. if a future
  change turns `pop_layer` into a `Result`-returning, non-panicking API, per that candidate's own
  "what's not done" section), `RootTransforms::pop_root`'s currently-dead unguarded pop would
  become live and should be revisited at that point - it does not independently protect itself
  today, it is only shielded by the other stack's earlier check.
- `vello_common`'s own crate-level doc explicitly states "This crate should not be used on its own,
  and you should instead use one of the renderers which use it" - so a hypothetical third-party
  crate directly constructing a `RootTransforms` and calling `pop_root()` without a matching
  `push_root()` (bypassing `vello_cpu`/`vello_hybrid` entirely) was not investigated as a "real
  public entry point" for this candidate's purposes, consistent with how candidates 031/045/046
  scoped their own "reachable via a public entry point" language to `vello_cpu`'s/`vello_hybrid`'s
  actual public APIs, not `vello_common`'s.
