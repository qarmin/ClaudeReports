# SIMPLIFY_SAMPLE_PT_TANGENT_ATAN2_HAIRPIN (checking for the same pattern in CubicOffset)

Issue #605 / PR #606 (open, unmerged at base_commit) fix a hairpin bug in
`SimplifyBezPath::sample_pt_tangent` (kurbo/src/simplify.rs): a line segment's `to_cubic()` form
(`CubicBez::new(p0, p0, p1, p1)`) has a zero derivative at t=0/t=1, and `fit_to_cubic` (kurbo/src/fit.rs)
turns that zero tangent vector into a spurious `atan2(0, 0) == 0` (+x axis) constraint, silently
producing a reversed end tangent. This candidate checks the suspicion raised alongside that PR:
whether `CubicOffset` (kurbo/src/offset.rs), which also fits/approximates curves and could plausibly
share "the atan2(0,0) footgun ... in shared fitting code", exhibits the same failure for the same
kind of degenerate (zero-derivative-at-endpoint) input.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: compare `CubicOffset`'s tangent computation to `SimplifyBezPath`'s | Read kurbo/src/offset.rs, kurbo/src/fit.rs, kurbo/src/simplify.rs, kurbo/src/bezpath.rs, kurbo/src/stroke.rs, kurbo/src/vec2.rs in full | `CubicOffset` does **not** implement `ParamCurveFit` and never calls `fit.rs`'s `fit_to_cubic`/`fit_to_bezpath` at all - it has its own bespoke recursive-subdivision algorithm (`offset_rec`/`draw_arc`/`apply`). So there is no "shared fitting code" between it and `SimplifyBezPath`; the only other `ParamCurveFit` implementor in the crate is `examples/fit_poly.rs`'s `MyPoly`, whose derivative is never zero (a smooth quartic-derived curve), so it isn't a candidate either. `offset_cubic`'s endpoint tangents come from `PathSeg::tangents()` (kurbo/src/bezpath.rs), which is explicitly documented "robust to the path segment not being a regular curve" and implements a fallback chain (`p1-p0`, else `p2-p0`, else `p3-p0`) precisely for the degenerate-control-point case, unlike the raw `c.deriv().eval(t0)` `SimplifyBezPath::sample_pt_tangent` used pre-#606. `CubicOffset::endpoint_cusp` also has its own explicit `TAN_DIST_EPSILON` clamp for a near-zero tangent, separate from `tangents()`. |
| Manual boundary-value construction: standalone repro exercising the exact degenerate shape (`CubicBez::new(p0, p0, p1, p1)`, i.e. `PathSeg::Line::to_cubic()`'s output, the same input shape that broke `SimplifyBezPath`) through kurbo's public API | `repro/` (own `Cargo.toml`), `cargo run --release`: (1) `kurbo::offset::offset_cubic` called directly, (2) `kurbo::stroke()` (the real end-to-end public entry point) fed a path whose one segment is a `CurveTo` with the same degenerate control points. Deterministic, no randomness - single run is definitive; re-run to confirm identical output. | Both checks land on the correct, non-reversed offset geometry - see `output.txt`. No hairpin. |
| Manual boundary-value construction: fully-degenerate cubic (all four control points identical - a zero-length curve, stronger degeneracy than the line-as-cubic case) | Same `repro/` binary, third (informational) check | Produces `NaN` throughout (`Vec2::normalize()` of a zero vector is documented to return `NaN`), a **different failure mode** (NaN propagation, not a silently-reversed direction) from the hairpin pattern being checked - and `offset_cubic`'s own doc comment already discloses this class of input isn't supported ("not suitable for degenerate cubics with entirely co-linear control points ... should be handled before calling this function"). Also not reachable through the real public `stroke()` driver: `stroke.rs`'s `do_cubic()` explicitly detects near-colinear/degenerate control points (via a chord-projection + cross-product colinearity check) and routes to `do_linear()` *before* ever calling `offset_cubic`. Not investigated further as part of this candidate (see "What's not done" below). |

## Status

**Not reproducible.** The specific hypothesis - that `CubicOffset` shares `SimplifyBezPath`'s
pre-#606 `atan2(0,0)`-produces-a-hairpin pattern for a zero-derivative-at-endpoint input - does not
hold. Covered: (a) full read of every function in the suspected call path, confirming `CubicOffset`
doesn't route through `fit.rs`'s `atan2`-based tangent-angle code at all; (b) a deterministic,
directly-run reproduction attempt using the exact same degenerate cubic shape that broke
`SimplifyBezPath`, through both the internal `offset_cubic` entry point and the real public
`stroke()` API - both produced correct, non-reversed output every time (deterministic, no flakiness
to characterize). A future session shouldn't re-try this same input shape against `CubicOffset`
expecting a different result.

**Fully reproducible**: N/A (Not reproducible - no bug to reproduce).

## Class

N/A (Not reproducible).

## Root cause

Not applicable in the "why does the bug happen" sense - instead, why the suspected spot turns out
safe: `CubicOffset` doesn't use `fit.rs`'s shared `fit_to_cubic`/`fit_to_bezpath` machinery at all
(it's a separate algorithm), and the helper it does use for endpoint tangents
(`PathSeg::tangents()` in kurbo/src/bezpath.rs) was written with an explicit fallback chain to
avoid exactly the zero-derivative-at-endpoint case, unlike the naive `c.deriv().eval(t0)` that
`SimplifyBezPath::sample_pt_tangent` used before PR #606. The premise that "the atan2(0,0) footgun
lives in shared fitting code" doesn't hold: PR #606 itself only touches kurbo/src/simplify.rs
(confirmed by reading its diff) - `fit_to_cubic`'s own `atan2` call in kurbo/src/fit.rs is
unchanged, because the bug was in what `SimplifyBezPath` supplied as a tangent, not in `fit.rs`
itself.

## Introduced by

N/A (Not reproducible - no bug to trace).

## The fix

N/A (Not reproducible - no bug, no fix attempted). The main project's working tree was never
modified during this investigation (`git status` clean throughout).

- Project test suite: not run (no fix to verify - see "the one rule that matters most" in
  SKILL.md: a fix is only applied/reverted around step 6-8 of the candidate loop, which this
  candidate never reached).

## What's not done / open questions

- The fully-degenerate (all-four-points-equal) `offset_cubic` input producing `NaN` (check 3
  above) is a real, reproducible behavior, but it's a **pre-existing, already-documented**
  limitation (`offset_cubic`'s own doc comment), a different bug class (NaN propagation, not a
  silently-wrong direction) from what this candidate was checking, and not reachable via the
  public `stroke()` driver in the current code (`do_cubic()` filters it out first). Whether it's
  worth a dedicated candidate of its own (e.g., should `offset_cubic` return an `Option`/panic
  loudly instead of silently emitting `NaN` control points when called directly and out of
  contract?) was not investigated further this session.
- Other direct (non-`stroke()`) callers of `offset::offset_cubic` outside this crate were not
  surveyed - this session only checked kurbo's own `stroke.rs` caller and the two public API entry
  points (`offset_cubic` directly, and `stroke()`).
