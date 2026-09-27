# SIMPLIFY_BEZPATH_EMPTY_CLOSED_SUBPATH_PANIC

`simplify_bezpath` (`kurbo/src/simplify.rs`, `ClosePath` arm of the main loop, lines 335-341)
unconditionally calls `state.result.close_path()` after `state.flush()`, with no check that
`state.result` has actually received a `MoveTo` for the current subpath. `flush()` is a no-op when
`self.queue` is empty, which happens whenever a `MoveTo` is immediately followed by `ClosePath`
with no segments queued in between (e.g. SVG `M0,0Z`). This is a well-formed input (unlike
candidate 003's malformed-input unwrap panic), so the panic is entirely internal to
`simplify_bezpath`'s own accumulator state. Reported as candidate 058.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (debug build, `cargo run`) | `BezPath::new(); move_to((0,0)); close_path();` piped through `simplify_bezpath(path, 0.1, &Default::default())` | Confirmed on the first try: panics at `bezpath.rs:275` (`debug_assert!(!self.0.is_empty(), "uninitialized subpath (missing MoveTo)")`) inside `BezPath::close_path`. This is a deterministic, pure-control-flow bug with no adversarial/randomized input space and no `unsafe`, so escalating to miri/ASAN/fuzzing was judged to add no signal - one clean, immediately-confirmed manual construction is sufficient per the soft-cap guidance. |
| Same input, release build (`cargo run --release`) | same input as above | No panic (`debug_assert!` is compiled out), but confirms the release-mode failure mode instead: returns `BezPath([ClosePath])`, a malformed path that does not start with `MoveTo`, violating the documented invariant. |
| Cross-check against sibling entry points that handle the same input | `kurbo::stroke(path, &Stroke::new(1.0), &Default::default(), 0.1)` and `kurbo::expand_path(path, Diagonal2::new(1.0,1.0), Join::Round, 10.0, 0.1)` on the identical `M0,0Z` input | Both return `BezPath([])` (the empty closed subpath is silently dropped) - neither panics nor produces a malformed path. This confirmed `simplify_bezpath` is the outlier and gave the target "graceful" behavior to match when designing the fix. |
| Extended manual construction: a degenerate empty closed subpath in the *middle* of an otherwise well-formed multi-subpath path (`M0,0 L1,1 Z M2,2 Z M3,3 L4,4 Z`) | debug and release builds | Found a related failure mode by the same root cause that the candidate report didn't call out: this never hits the `debug_assert!` (the global element vec is already non-empty by then, since `close_path()`'s check is on the *whole* accumulated path, not the current subpath), but it silently emits a second, moveto-less `ClosePath` into the result in both debug and release builds - also a malformed `BezPath`. Recorded here since the same fix (see below) also happens to close it. |

## Status

**Confirmed**. Reproduces deterministically 3/3 runs in a debug build (`cargo run`, see
`output.txt`) with an identical panic message and location each time, and deterministically in a
release build (`cargo run --release`) returning the identical malformed `BezPath([ClosePath])`
each time - this is pure, input-driven control flow with no randomness, allocator-dependence, or
threading involved, so it will reproduce 100% of the time on any build of this exact source.

**Fully reproducible**: **Yes**. This is a single-participant bug: `simplify_bezpath` is a pure
function called directly by whatever code constructs the input `BezPath` (e.g. an SVG path parser
handed `M0,0Z`, or any other caller assembling a path programmatically) - the input itself is a
completely ordinary, well-formed `BezPath` any real caller could construct with the public
`BezPath` builder API, not a stand-in for a second participant's wire format or internal state.
`repro/` calls the real public `kurbo::simplify::simplify_bezpath` directly with such a path.

## Class

crash (debug-mode panic) / logic (release-mode malformed output, invariant violation)

## Root cause

`SimplifyState::flush()` (`kurbo/src/simplify.rs:261-264`) returns immediately with no side effect
when `self.queue.is_empty()`. The `queue` only gets a `MoveTo` pushed into it via `add_seg()`, which
is only called for an actual line/quad/cubic segment - so a subpath consisting of just `MoveTo`
followed directly by `ClosePath` never populates `queue` at all. Consequently, when the `ClosePath`
arm of `simplify_bezpath`'s main loop calls `state.flush(accuracy, options)` immediately followed by
`state.result.close_path()`, `flush()` does nothing and `state.result` never received the `MoveTo`
for this subpath. `BezPath::close_path()`'s own `debug_assert!(!self.0.is_empty(), "uninitialized
subpath (missing MoveTo)")` only checks whether the *entire accumulated* `state.result` vector is
non-empty - not whether the *current* subpath specifically has an open `MoveTo` - so:

- If this is the very first subpath in the whole input (`state.result` is completely empty), the
  assert fires and the program panics in a debug build; in a release build (assert compiled out)
  `close_path()` pushes `ClosePath` as the very first and only element, producing
  `BezPath([ClosePath])`, which violates the crate's documented "every `BezPath` must start with
  `MoveTo`" invariant.
- If an earlier subpath already emitted content to `state.result` (the "middle of the path" case
  found during this investigation), the assert never fires at all (the vector isn't empty), but the
  same missing-MoveTo condition still holds for *this* subpath, so a moveto-less `ClosePath` is
  silently appended right after the previous subpath's own `ClosePath` - malformed in both debug and
  release builds, with no panic to signal it.

`state.needs_moveto` (set `true` by the `MoveTo` arm, and only ever set back to `false` inside
`flush()` when it actually emits content) turns out to already be the exact signal needed to detect
this: it stays `true` across a `ClosePath` iff `flush()` was a no-op for the current subpath, i.e.
iff nothing was ever emitted to `result` for it.

## Introduced by

5196345 "Penalize large d values in cubic fit" (2023-04-17) - this commit introduced the
`simplify_bezpath` function itself (found via `git log -S "fn simplify_bezpath" -- kurbo/src/simplify.rs src/simplify.rs`,
confirmed by reading the commit's diff, which adds the exact `state.flush(...); state.result.close_path();`
pair unchanged since). The file already existed before this commit (from 7eb67e3 "Implement
simplify for bez paths", 2022-12-27), but `simplify_bezpath` itself, and the bug, did not exist
until 5196345.

## The fix

Guard the `close_path()` call in the `ClosePath` arm on whether the current subpath actually
emitted anything to `result` (equivalently: skip it when `state.needs_moveto` is still `true` after
the `flush()` call, which now serves double duty as "was this subpath's `flush()` a no-op"):

```rust
PathEl::ClosePath => {
    state.flush(accuracy, options);
    if !state.needs_moveto {
        state.result.close_path();
    }
    state.needs_moveto = true;
    last_seg = None;
    continue;
}
```

This makes `simplify_bezpath` silently drop a degenerate empty closed subpath instead of
panicking/emitting malformed output - matching the already-graceful behavior independently verified
above for `kurbo::stroke` and `kurbo::expand_path` on the identical input (both already drop such a
subpath entirely, returning `BezPath([])`).

Verified against `repro/` (see `output.txt` for the actual captured `just test` run, a real clone of
the project at `base_commit` with `patch.diff` applied in between): before the fix, case 1 panics
in debug and returns `BezPath([ClosePath])` in release; after the fix, case 1 returns `BezPath([])`
in both builds. Case 2 (the middle-of-path variant) also produces a well-formed
`BezPath([MoveTo((0.0, 0.0)), LineTo((1.0, 1.0)), ClosePath, MoveTo((3.0, 3.0)), LineTo((4.0, 4.0)),
ClosePath])` after the fix - the degenerate `M2,2 Z` subpath is silently dropped and the two
genuine subpaths on either side of it are unaffected, in both debug and release builds.

- Project test suite (`cargo test` from the `kurbo/` package directory): 187 unit tests + 55
  doctests passing before the fix, 187 unit tests + 55 doctests passing after the fix (0 failures
  either way; no new tests were added by this fix). Ran directly in a git worktree of the project
  (`git stash` / `git stash pop` around the patch), not just inside `repro/`, which only depends on
  the library's public API.

## What's not done / open questions

- No test was added to the crate's own test suite exercising this exact input (`M0,0Z` / a
  moveto-only-then-close subpath, both standalone and mid-path) - the fix was verified via the
  standalone `repro/` and the unmodified existing test suite only, per this session's scope.
- Didn't check whether `simplify_bezpath`'s own initial-`MoveTo`-arm or end-of-loop handling has any
  further variant of the same "did this subpath ever emit to `result`" gap beyond the `ClosePath`
  arm fixed here (e.g. a path that ends with a trailing `MoveTo` and no following segment/ClosePath
  at all) - the final `state.flush(accuracy, options)` call after the loop is a no-op in that case
  already (queue empty), which is already correct/harmless, but wasn't independently re-derived as
  part of this candidate.
- Didn't sweep for the same "global emptiness instead of per-subpath" gap in `BezPath::close_path`'s
  own `debug_assert!` elsewhere in the crate - other callers of `BezPath::close_path` (e.g.
  hand-rolled path builders in other modules) could in principle have the same "no panic but
  moveto-less ClosePath mid-path" failure mode; this candidate only checked `simplify_bezpath`.
