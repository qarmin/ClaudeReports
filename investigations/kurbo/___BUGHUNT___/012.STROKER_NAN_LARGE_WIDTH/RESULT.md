# STROKER_NAN_LARGE_WIDTH

`kurbo::stroke()`'s cubic-offset math (`kurbo/src/offset.rs`, the new least-squares offset curve
algorithm that replaced the old quartic-solver-based one in 0.12), suspected of a width-dependent
division/normalization producing NaN, per issue #519 (source: `___BUGHUNT___/__issues_raw/issues/519.json`).
The reporter's own exact repro (a tiny, near-degenerate triangular path, `width: 10.0`) was used
directly, without modification.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Reporter's exact repro from issue #519 (`kurbo::stroke` on the given SVG path, width 10.0 vs 5.0) | debug build (`cargo run -p kurbo --example`), `cargo test --workspace --all-features` for the regression suite | Reproduced immediately: `width=10.0` -> `is_nan() == true`, `width=5.0` -> `is_nan() == false`, matching the issue exactly. 3/3 repeat runs, deterministic (no randomness/seed involved). |
| Manual boundary-value sweep of `width` in the same path (4.0 through 1000.0, 1.0 step near the boundary) | same debug build | NaN appears for `width` in `{10.0, 50.0}` in this sweep and not for the others - not monotonic in width, consistent with the root cause being a sign-flip condition (`cusp0 * cusp1 < 0.0`, i.e. "does a real cusp appear in the offset curve for this specific offset distance") rather than a simple magnitude/overflow threshold. |
| Manual root-cause tracing via temporary `eprintln!` instrumentation in `do_cubic`/`offset_rec` (removed before finishing; not part of the shipped fix) | debug build | Pinpointed the exact line and mechanism (see "Root cause" below) in a few iterations - no sanitizer/miri/fuzzing needed since this crate has zero `unsafe` and the bug is a pure floating-point logic error, not memory-unsafety (matches `___BUGHUNT___/README.md`'s existing toolchain notes for this project). |

Only one approach was needed to reproduce (the issue's own repro reproduced on the first try), and
one further approach (instrumented tracing) to root-cause it - no dead ends, no sanitizer/fuzz
tooling was relevant here (no `unsafe` anywhere in `kurbo/src`, and the input is a fixed, already-
known-triggering path, not something a fuzzer needed to discover).

## Status

**Confirmed**

Reproduces deterministically, 3/3 runs, debug build, using the exact input from issue #519 with no
modification. Also reproduces in the same way for `width = 50.0` on the same path (found during the
boundary sweep), showing the bug class isn't specific to the one width value from the original
report.

**Fully reproducible**: **Yes**. The repro calls `kurbo::stroke()` (kurbo's real, public stroking
entry point) directly on a `BezPath` built from literal SVG path data - the same public API and the
same kind of externally-supplied path data (e.g. parsed from an SVG file, or authored via
`BezPath`'s builder methods) a real caller would use. There is no second participant to fake here:
the "input" is a single external artifact (a path + a stroke width), not something produced by a
peer/caller whose behavior would need to be simulated (see `CONFIRMATION.md` section 11) - a
single-process repro calling the public API is the genuine end-to-end trigger path.

## Class

logic (NaN silently produced by ordinary floating-point arithmetic - not a panic, not a crash, not
memory-unsafety; the crate has zero `unsafe` in `kurbo/src`)

## Root cause

`crate::offset::offset_cubic` (used by the stroker's `StrokeCtx::do_cubic` for every "genuinely
curved" - i.e. not near-colinear - cubic segment) recursively subdivides a cubic Bézier into pieces
small enough that a simple arc + least-squares approximation stays within `tolerance` of the true
offset curve. Before approximating a given `[t0, t1]` interval, it first checks whether the offset
curve has a *cusp* somewhere inside that interval, by comparing the sign of a "cusp value"
(curvature times offset distance, plus 1) at both ends (`offset.rs`, `offset_rec`, the
`rec.cusp0 * rec.cusp1 < 0.0` branch). If the signs differ, it uses `solve_itp` to find the exact
`t` where the cusp value crosses zero, then computes a unit tangent at that `t` via
`self.q.eval(t).to_vec2().normalize()` (where `q` is the derivative of the source cubic).

For the reported input, the cubic segment that actually reaches `offset_cubic` is
`CubicBez { p0: (47.4321, 317.6239), p1: (46.7915, 317.7801), p2: (46.7915, 317.2176), p3: (46.7915, 317.2176) }`
- note `p2 == p3` exactly (the original SVG path has a repeated final control point on this
segment). This makes the curve's derivative *exactly zero* at `t = 1` (`q.eval(1) = 3*(p3 - p2) =
(0, 0)`), a genuine endpoint cusp. The library already has a robustness guard for this case at the
*interval endpoints* - `CubicOffset::endpoint_cusp` clamps the tangent length away from zero with a
`TAN_DIST_EPSILON` before taking a reciprocal, and `offset_cubic`'s own top-level call uses the
*separately computed*, fallback-protected tangent from `PathSeg::tangents()` (which never returns a
zero vector) for `rec.utan0`/`rec.utan1`.

The bug is that this endpoint protection isn't applied to the *found cusp root* itself: when a
larger stroke width pushes the cusp-value sign-crossing search (`cusp0 * cusp1 < 0.0`) to trigger at
all - which for this path only happens for some widths (10.0, 50.0) and not others (4-9, 15, 20,
100, 1000), since it depends on whether `curvature(t) * offset_distance + 1` actually changes sign
across the interval - the ITP solver (`solve_itp`) can land its found root *exactly at* the
degenerate endpoint `t = 1` (confirmed by instrumentation: `t0=1.00000000000000000000
t1=1.00000000000000000000` printed with full precision). At that exact point, the unprotected
`self.q.eval(t).to_vec2().normalize()` divides a zero-length vector by its own (zero) length,
producing `Vec2 { x: NaN, y: NaN }`. This NaN tangent (`utan_t`) is then passed into `subdivide()`
as the new interval boundary's tangent, which poisons every further recursive call on that side:
`cusp_sign` and `apply()` keep evaluating to NaN, the interval collapses to `t0 == t1 == 1.0`
(no further bisection progress is possible on an already-zero-width interval), and recursion runs
all the way to `MAX_DEPTH` (8) before finally calling `result.curve_to(c_approx.p1, c_approx.p2,
c_approx.p3)` with NaN control points - which is what makes `BezPath::is_nan()` return `true` for
the whole stroked path.

Width dependence: the *classification* of which cubic segments in this path are curved (go through
`offset_cubic`) vs. treated as linear (`do_linear`, which uses a different, cusp-length-based
tangent computation and isn't affected) is unrelated to width. But whether the cusp *search itself*
finds a sign crossing at all (`cusp0 * cusp1 < 0.0`) is a direct function of the offset distance `d
= 0.5 * width`, since the cusp value is literally `curvature * d + 1` - this is why only some widths
(here 10.0, 50.0) trigger the NaN-producing branch on this exact path, and not others.

## Introduced by

77cb963 "First cut at least-squares offset curves (#427)" (2025-07-22) - `git log -S
"self.q.eval(t).to_vec2().normalize()" -- kurbo/src/offset.rs` returns exactly this one commit,
which added `kurbo/src/offset.rs` (the new least-squares offset-curve algorithm) wholesale,
including this exact line, unchanged since. This matches the issue's own regression report (fine
in 0.11.3, broken in 0.12): 0.11.3 used the old quartic-solver-based offset method, replaced by this
commit's new algorithm ahead of the 0.12 release.

## The fix

In `offset_rec`'s cusp-subdivision branch, guard the tangent computation at the found root `t`:
only call `.normalize()` on `self.q.eval(t)` when its squared length is above a small epsilon
(`1e-12`, matching the `UTAN_EPSILON` convention already used elsewhere in this same file, in
`subdivide_for_tangent`); otherwise fall back to the nearer of the interval's own two
already-known-good endpoint tangents (`rec.utan0`/`rec.utan1` - the same tangents used everywhere
else in this function as the robust, non-degenerate boundary values), based on which endpoint `t`
is closer to.

```diff
-            let utan_t = self.q.eval(t).to_vec2().normalize();
+            const UTAN_EPSILON: f64 = 1e-12;
+            let q_t = self.q.eval(t).to_vec2();
+            let utan_t = if q_t.hypot2() >= UTAN_EPSILON {
+                q_t.normalize()
+            } else if t - a <= b - t {
+                rec.utan0
+            } else {
+                rec.utan1
+            };
```

Verified against the patched tree:

- The original issue #519 repro (width 10.0 on the reported path): `is_nan()` goes from `true` to
  `false`, re-run 3/3 times, deterministic.
- The full boundary sweep from the "What was tried" table (widths 4.0 through 1000.0, including the
  other width that also triggered it, 50.0): every width now produces `is_nan() == false` - the fix
  isn't narrowly specific to width 10.0 (per `CONFIRMATION.md` section 7).
- Project test suite (`cargo test --workspace --all-features`, both `kurbo` and `polycool`):
  before the fix: 187 + 10 + 55 = 252 passed, 0 failed, 1 ignored (doctest). After the fix:
  identical - 187 + 10 + 55 = 252 passed, 0 failed, 1 ignored. No regressions, no new failures.

## What's not done / open questions

- The same "unprotected `.normalize()` on a possibly-zero derivative at a found root" pattern is
  worth double-checking against `find_subdivision_point`/`subdivide_for_tangent` in the same file -
  those already have their own `UTAN_EPSILON`-guarded fallback logic (a different, pre-existing
  code path from the one this fix touches), so they were not re-suspected this session, but weren't
  independently stress-tested against this same input either.
- Not checked: whether a similarly-shaped degenerate cubic (repeated control point creating an
  exact-zero endpoint derivative) can also drive the *other* cusp-detection call site
  (`find_subdivision_point`'s own `cusp_sign` evaluation for the non-cusp-branch subdivision path)
  into a comparable NaN, since that path already has its own epsilon guards in `subdivide_for_tangent`
  and wasn't observed to trigger for this input.
- A longer fuzzing campaign (via `fuzz-bug-repro`, run separately) specifically targeting
  `offset_cubic` with randomly-generated near-degenerate cubics (repeated control points at either
  end, combined with a range of offset distances) would likely be worth pointing at this function -
  this session found the one specific input from the issue plus one nearby width value by hand, not
  via a systematic search.
