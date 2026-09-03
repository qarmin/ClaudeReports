# BUMP_ESTIMATOR_LINETO_UNWRAP_PANIC

`BumpEstimator::count_path` (vello_encoding/src/estimate.rs, gated behind the `bump_estimate`
feature) walks a `PathEl` iterator tracking `first_pt`/`last_pt`, implicitly assuming every
subpath starts with a `MoveTo`. The `QuadTo`/`CurveTo` arms defensively guard against a missing
leading `MoveTo` with `let Some(p0) = last_pt.or(first_pt) else { continue };`, but the sibling
`LineTo` arm had no such guard and called `first_pt.unwrap()`/`last_pt.unwrap()` unconditionally.
Candidate came in pre-identified (already reproduced once by whoever filed it); this session
re-verified it independently and built the permanent repro/fix artifacts.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct reproduction via public API | `cargo run --release`, standalone crate depending on `vello_encoding` (path dep) with `bump_estimate` feature, calling `BumpEstimator::new().count_path(vec![PathEl::LineTo(Point::new(5.0,5.0))].into_iter(), &Transform::from_kurbo(&Affine::IDENTITY), None)` | Panicked deterministically: `called \`Option::unwrap()\` on a \`None\` value` at estimate.rs:99:66 |
| Repeat-run check | Same binary, release build, 5 consecutive runs | 5/5 panicked identically - fully deterministic, no layout/seed dependency |
| Boundary-value variants after fix | Two bare LineTos with no MoveTo; MoveTo-then-LineTo (well-formed); bare ClosePath | All completed normally against the patched tree - fix holds across the class, not just the exact triggering input |
| Project test suite (before/after) | `cargo test -p vello_encoding --features bump_estimate` | 31 passed / 0 failed both before and after the fix - no regression |

No `miri`/ASAN/fuzzing was needed: this is a plain safe-Rust `Option::unwrap()` panic on a
state-machine invariant, directly constructible by hand (boundary-value construction was the
right-sized tool per HEURISTICS.md, not memory-safety instrumentation).

## Status

**Confirmed**. Reproduces 5/5 runs, release build, fully deterministic (no fixed seed needed - the
panic depends only on the input path shape, not on memory layout or timing).

**Fully reproducible**: **Yes**. The repro calls `BumpEstimator::count_path` - a real, documented,
public API of `vello_encoding` - directly with a `Vec<PathEl>` built via ordinary `peniko::kurbo`
constructors. This is a single-participant finding (how the estimator's own state machine reacts to
an input shape a caller can construct and pass straight through vello's public API - `peniko::kurbo::
BezPath::from_vec`/manual `Vec<PathEl>` construction never enforces "starts with MoveTo", and
`vello::Scene::fill`/`stroke` accept `&impl Shape` with no such precondition documented either), so
there is no second participant to fake here - CONFIRMATION.md section 11's bar is about not
faking a second participant's traffic, and none was needed or faked in this repro.

## Class

crash

## Root cause

`count_path`'s `LineTo` arm (estimate.rs line ~99) unconditionally called
`count_segments_for_line(first_pt.unwrap(), last_pt.unwrap(), t)`. If the very first `PathEl` the
iterator yields is a `LineTo` (no preceding `MoveTo`), `first_pt` is still `None` at that point,
so `first_pt.unwrap()` panics. The `MoveTo`, `QuadTo`, and `CurveTo` arms in the exact same
function already handle a missing-leading-`MoveTo`/missing-last-point state gracefully (`MoveTo`
via `let Some(point) = last_pt else { continue; }`; `QuadTo`/`CurveTo` via
`let Some(p0) = last_pt.or(first_pt) else { continue; }`), and the real path encoder
(`vello_encoding/src/path.rs`, `PathEncoder::line_to`) explicitly documents and implements the same
handling ("This copies the behavior of kurbo which treats an initial line, quad or curve as a
move.") - so the `LineTo` arm here is a straightforward oversight that missed the treatment its own
siblings already carry, not an intentional design choice.

## Introduced by

829eb2c7a36d2ade6fa6fc79960abc0786ab5abd "[encoding] Bump estimate for segments (#454)"
(2024-03-15). This commit added the `segments` field and the `count_segments_for_line(...)` call
in the `LineTo` arm (confirmed via `git show 829eb2c7 -- crates/encoding/src/estimate.rs`, which
shows the exact `first_pt.unwrap()`/`last_pt.unwrap()` line being introduced fresh - the
surrounding `LineTo` arm itself, without that call, predates this commit back to the file's
original addition in f55f82f2 "[encoding] Initial attempt at a BumpEstimator utility (#436)", but
that earlier version had no unwrap-panic potential since it didn't yet read `first_pt`/`last_pt`
in this arm).

## The fix

Added the same guard the `QuadTo`/`CurveTo` arms already use, applied to `LineTo`:

```rust
PathEl::LineTo(p0) => {
    let Some(start) = last_pt.or(first_pt) else {
        continue;
    };
    last_pt = Some(p0);
    joins += 1;
    lineto_lines += 1;
    segments += count_segments_for_line(start, p0, t);
}
```

A leading `LineTo` with no prior state is now silently skipped (consistent with how `QuadTo`/
`CurveTo` already treat the same situation in this function - conservative-estimate-only code, not
the real encoder, so this only affects the GPU bump-allocation size *estimate*, never rendering
correctness) instead of panicking. Verified: `repro/` no longer panics against the patched tree,
and three additional boundary-value inputs in the same class (two bare LineTos, a well-formed
MoveTo-then-LineTo, and a bare ClosePath) all still complete normally - see `output.txt`.

- Project test suite: `cargo test -p vello_encoding --features bump_estimate` - 31 passed / 0
  failed before the fix, 31 passed / 0 failed after (no test in the existing suite covered this
  input shape, so nothing regressed and nothing newly failed).

## What's not done / open questions

- The same missing-`MoveTo` state-machine gap in `QuadTo`/`CurveTo` (silently `continue`s instead
  of panicking) is *not* itself a crash, but it does mean the estimator quietly undercounts an
  ill-formed input's first curve entirely rather than treating it as an implicit `move`-then-emit
  the way `PathEncoder` does - this is a pre-existing behavior difference between the estimator and
  the real encoder, not something this fix changes or newly introduces; left unaddressed here since
  it's a logic/estimation-accuracy question, not a crash.
- Not checked: whether any other consumer of `BumpEstimator::count_path` (outside this crate, e.g.
  in `vello`'s own scene-encoding path when `bump_estimate` is enabled) could feed it a path whose
  first element is `ClosePath` immediately followed by further elements in a way that resurfaces a
  similar gap - the `ClosePath` arm's own guard (`if let Some(last_pt) = last_pt`) looked sound on
  inspection and no panic was found there in the boundary-value checks above, but only 4 shapes
  were tried in this session, not an exhaustive sweep.
