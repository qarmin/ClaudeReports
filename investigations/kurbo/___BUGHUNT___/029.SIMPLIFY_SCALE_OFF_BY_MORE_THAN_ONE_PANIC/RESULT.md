# SIMPLIFY_SCALE_OFF_BY_MORE_THAN_ONE_PANIC

`SimplifyBezPath::scale` (kurbo/src/simplify.rs) maps a curve parameter `t` to
`(segment_index, local_t)` via `t_scale = t * n; i = t_scale.floor() as usize`, with no clamping of
`t` to `[0, 1]`. Both trait-method call sites that consume this - `sample_pt_deriv` and
`sample_pt_tangent`, the two methods `ParamCurveFit` (re-exported at the crate root) requires -
only special-case `i == n` (`t` exactly at the curve's end), not `i > n` (`t` further out of
range), leaving an unguarded `self.0[i]` index into the internal `Vec<SimplifyCubic>`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: call the real public API (`kurbo::simplify::SimplifyBezPath::new` + `ParamCurveFit::sample_pt_deriv`/`sample_pt_tangent`, `SimplifyBezPath` is `pub struct` in a `pub mod`, `ParamCurveFit` is re-exported at the crate root) with a 2-segment path and `t = 1.6` | Debug build (`cargo run`), standalone crate against the worktree via a path dependency | `sample_pt_deriv(1.6)` panics `index out of bounds: the len is 2 but the index is 3` at simplify.rs:198; `sample_pt_tangent(1.6, 1.0)` panics the same way at simplify.rs:208. Reproduced 3/3 runs, deterministic (no RNG involved) |
| Manual boundary-value sweep across a wider range of `t` (-1.6, -0.1, 0.0, 0.5, 1.0, 1.0000001, 1.6, 2.9, 100.0) against the same 2-segment path | Same debug build | Negative `t` and `t` only barely above 1.0 (still landing on `i == n` exactly) do **not** panic - `t_scale.floor() as usize`'s saturating float-to-int cast clamps a negative `t_floor` to index 0 rather than panicking, and the existing `i == n` check already catches the boundary case. The panic requires `t` far enough past 1.0 that `i > n` (concretely, `t >= 1.5` for this 2-segment path); it also reproduces for arbitrarily large `t` (checked up to 100.0) |
| Read `fit.rs`'s own callers of `sample_pt_deriv`/`sample_pt_tangent` (the only in-crate caller of `ParamCurveFit`, via `fit_to_bezpath`/`fit_to_bezpath_opt`) to check whether the search machinery itself can ever produce a `t` outside `[0, 1]` and hit this internally, not just via a direct external call | Manual code reading of `fit_to_bezpath_rec`, `fit_to_bezpath_opt_inner`, `CurveDist::from_curve`/`compute_arc_params`, `fit_opt_segment` | Every one of `fit.rs`'s own recursive range subdivisions narrows strictly within a `Range<f64>` that starts at `0.0..1.0` (`fit_to_bezpath`'s only entry range) - midpoint subdivision picks `t` strictly between `start`/`end`, and `SimplifyBezPath::break_cusp` (called first at every recursion level) always returns `None`, so no branch ever widens the range past the outer `0.0..1.0` bound. This is a structural observation from reading the recursion, not an exhaustive proof (not fuzzed) - it did not turn up an internal path that reaches `i > n` through `fit_to_bezpath`/`fit_to_bezpath_opt`/`simplify_bezpath`/`simplify` with this session's reading; not investigated further via fuzzing since the direct public-API call above is already sufficient for a `Confirmed` finding independent of this question (see below) |

## Status

**Confirmed** - reproduces 3/3 runs, deterministic (no RNG, no build-order dependence), debug and
release builds both panic identically (checked both while building the final `repro/`).

**Fully reproducible**: **Yes**. `repro/` calls the real, fully public, unmodified API exactly as
an external crate would: `kurbo::simplify::SimplifyBezPath::new` (a `pub struct`/`pub fn` in a
`pub mod`) followed by `ParamCurveFit::sample_pt_deriv`/`sample_pt_tangent` (`ParamCurveFit` is
re-exported at the crate root in `kurbo/src/lib.rs`). This is a single-process, single-participant
finding - a direct caller supplying an out-of-contract argument to a public method, not one
participant reacting to another's produced input - so `CONFIRMATION.md` section 11's two-participant
recipe does not apply (see that section's own "single-process... remains completely fine" carve-out).
Note: `sample_pt_deriv`/`sample_pt_tangent`'s doc comments say "Generally `t` is in the range
`[0..1]`" - not a hard documented precondition or an `unsafe`/panic-on-misuse contract, which is
part of why this is worth reporting as a real defect rather than dismissing as pure caller error.

## Class

crash

## Root cause

`SimplifyBezPath::scale(t)` computes `t_scale = t * n` (`n` = number of cubic segments) and
`i = t_scale.floor() as usize` with no clamping of `t` (or of the resulting `i`) to a valid range.
For `t` far enough past `1.0` (concretely `t >= (n+1)/n`, i.e. `t >= 1.5` for a 2-segment path),
`i` exceeds `n` by 2 or more. Both call sites that consume `scale`'s result -
`ParamCurveFit::sample_pt_deriv` and `ParamCurveFit::sample_pt_tangent` - only guard the single
boundary case `i == n` (clamping it back to the last valid segment with `t0 = 1.0`), which was
presumably meant to handle `t == 1.0` landing exactly on `n` due to `t_scale.floor()` rounding an
exact integer up. Neither guard was written as `i >= n`, so any `i > n` falls straight through to
the unguarded `self.0[i].c` index, panicking with "index out of bounds" against the internal
`Vec<SimplifyCubic>` (length `n`). Negative `t` does not hit this same panic only because Rust's
float-to-int cast (`as usize`) saturates negative floats to `0` rather than wrapping or panicking
(a happy accident of the cast's semantics, not a guard the code itself provides), so `i` from a
negative `t` always lands in-bounds even though the returned local `t0` can still be a nonsensical
value for that segment.

## Introduced by

7eb67e32fee21e832cd92647bf570daaed098ded "Implement simplify for bez paths" (2022-12-27) - this
commit introduced `SimplifyBezPath::scale`, `sample_pt_deriv`, and `sample_pt_tangent` in
essentially their current shape (confirmed via `git show 7eb67e3 -- src/simplify.rs`, since renamed
to `kurbo/src/simplify.rs` by the later workspace-restructuring commit d67b9dd, which only moved
the file). The `i == n` special case (not `i >= n`) has been present unchanged since this
introducing commit.

## The fix

Applied directly to `kurbo/src/simplify.rs`, widening both call sites' clamp-back-into-range check
from `i == n` to `i >= n` (equivalently `i == self.0.len()` to `i >= self.0.len()`), matching what
the existing "clamp to the last segment, `t0 = 1.0`" behavior was clearly meant to cover for any
`t >= 1.0`, not just the one value that happens to land exactly on `n`:

```rust
fn sample_pt_deriv(&self, t: f64) -> (Point, Vec2) {
    let (mut i, mut t0) = self.scale(t);
    let n = self.0.len();
    if i >= n {
        i = n - 1;
        t0 = 1.0;
    }
    ...
}

fn sample_pt_tangent(&self, t: f64, _: f64) -> CurveFitSample {
    let (mut i, mut t0) = self.scale(t);
    if i >= self.0.len() {
        i = self.0.len() - 1;
        t0 = 1.0;
    }
    ...
}
```

Verified:

- `repro/`'s original triggering input (`t = 1.6` against a 2-segment path): before the fix, both
  `sample_pt_deriv` and `sample_pt_tangent` panic with "index out of bounds"; after the fix, both
  return `(Point::new(10.0, 10.0), Vec2::new(0.0, 0.0))`/an equivalent `CurveFitSample` (the last
  segment's endpoint, `t0 = 1.0`) with no panic. See `output.txt` for the actual `just test` run
  (fresh clone of the pinned base commit, `reproduce-before` panics, `apply`, `reproduce-after`
  succeeds).
- Varied beyond the one triggering input (CONFIRMATION.md section 7): re-checked `t` in
  `{-1.6, -0.1, 0.0, 0.5, 1.0, 1.0000001, 1.6, 2.9, 100.0}` against the patched tree - every value,
  including `t = 100.0` (far beyond the original `1.6`), completes without panicking and clamps to
  the last segment's endpoint for any `t >= 1.0`.
- Project test suite (`cargo test --workspace`, run in the worktree used to develop the patch,
  before and after applying the change, then reverted):
  - Before (pristine): kurbo lib tests 187 passed / 0 failed; polycool lib tests 10 passed / 0
    failed; combined doctests 55 passed / 0 failed (1 ignored, pre-existing/unrelated) - 252
    passed / 0 failed total.
  - After (patched): kurbo lib tests 187 passed / 0 failed; polycool lib tests 10 passed / 0
    failed; combined doctests 55 passed / 0 failed (1 ignored) - identical counts, 252 passed / 0
    failed total, no regressions.

## What's not done / open questions

- **Internal reachability through `fit.rs`'s own search (`fit_to_bezpath`/`fit_to_bezpath_opt`/the
  free `simplify`/`simplify_bezpath` functions) was not proven or disproven exhaustively** - code
  reading (see "What was tried") shows every recursive range narrows within the outer `0.0..1.0`
  bound `fit_to_bezpath` starts with, and `SimplifyBezPath::break_cusp` always returns `None` (so
  no cusp-driven range widening), which suggests `i > n` is not reachable this way - but this
  wasn't fuzzed or otherwise stress-tested, so it is not a proof. Regardless, the direct public-API
  call this candidate confirms (an external crate calling `sample_pt_deriv`/`sample_pt_tangent`
  with a `t` outside `[0, 1]`) is a real, fully public, unmodified-API panic surface on its own,
  independent of whether `fit.rs`'s own internal search can also reach it - see "Fully
  reproducible" above.
- `SimplifyBezPath::moment_integrals` (the `ParamCurveFit` trait method, not the free function of
  the same name) also calls `self.scale` twice (on `range.start`/`range.end`) and has **no**
  `i == n`/`i >= n` clamp at all - checked directly this session (`ParamCurveFit::moment_integrals(&s,
  0.0..1.6)` against the same 2-segment path) and it panics too, at a different line
  (`self.0[i].c.subsegment(range)` in the private `moment_integrals(i, range)` helper,
  simplify.rs:185, not the two lines this candidate's patch touches). This is the same root-cause
  file/mechanism but a distinct call site with its own indexing pattern (no existing clamp to
  extend, unlike the two fixed here) - left unpatched, out of scope for this candidate's "one bug
  per candidate" boundary, and noted as a follow-up suspect.
- Only the two-segment case was checked by hand; a 1-segment (`n = 1`) or a much larger `n` were
  not separately verified, though the fix (`i >= n` vs. `i == n`) is not expected to depend on `n`.
