# EVAL_NAN_AT_ENDPOINTS_EXTREME_MAGNITUDE

`Line::eval`, `QuadBez::eval`, and `CubicBez::eval` all evaluate their curve as a
lerp/polynomial-in-`t` expression. When an intermediate control point has extreme magnitude
relative to the endpoints, an intermediate term of that expression can overflow to `+-inf`
*before* it's multiplied away by the `t`/`mt` factor that is exactly `0.0` at `t == 0.0`/`1.0`,
turning `0.0 * inf` into `NaN` (or leaving a stray `inf`) even though the exact mathematical
result at an endpoint is trivially just that endpoint. Flagged by kurbo issue #598 with concrete
repro code for all three types.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction, `cargo run --example` against the crate's public API (`Line`/`QuadBez`/`CubicBez::eval`, `ParamCurve` trait) with `1e308`-magnitude intermediate control points, release build | Ran the exact repro from issue #598 plus several variants with the extreme point in different control-point slots (`p1` only, `p1`+`p2`, endpoints at `+-1e308` for `Line`), release build, 3 repeat runs | Reproduced identically every run: `eval(0.0)` returns `(NaN, NaN)`/`(NaN, y)` for all three curve types when an intermediate control point is `1e308`-scale relative to `p0`; `Line::eval(1.0)`/`eval(0.0)` return an infinite coordinate when the two endpoints themselves are `+-1e308` (`other - self` overflows in `lerp`). `QuadBez`/`CubicBez` `eval(1.0)` also reproduces in variants where the extreme magnitude sits in the control point nearest `p2`/`p3` instead of `p0`. |
| Code reading of every other `ParamCurve::eval` impl in the crate (`Arc::eval`, `PathSeg::eval`, `ConstPoint::eval`) for the same lerp-chain pattern | Read `src/arc.rs`, `src/bezpath.rs`, `src/line.rs` | `PathSeg::eval` just delegates to `Line`/`QuadBez`/`CubicBez::eval`, so it inherits the same bug (not separately investigated further - same root cause). `Arc::eval` samples via `sin`/`cos` of an angle plus a center offset, not a `t`/`mt`-weighted polynomial of control points, so it isn't built from the same "term overflows, then gets zeroed by a `0.0` factor" pattern; not independently reproduced as sharing this specific bug (a different bug class - e.g. extreme radii feeding `sin_cos` - was not investigated, out of scope for this candidate). `ConstPoint::eval` is a trivial constant, not applicable. |

## Status

**Confirmed**. Reproduces 3/3 runs, release build, deterministic (pure floating-point arithmetic
with fixed inputs, no randomness/seed involved) - re-running the exact same repro always produces
the exact same `NaN`/`inf` outputs.

**Fully reproducible**: **Yes**. This is a single-participant, pure-function bug (no
caller/peer/wire boundary involved) - `repro/` constructs `Line`/`QuadBez`/`CubicBez` values
directly through kurbo's own public API (`Line::new`, `QuadBez::new`, `CubicBez::new`) with
plain finite `f64` literal coordinates, exactly as any real caller would, and calls the real
`ParamCurve::eval` entry point on them. There is no second participant to fake here - `eval`'s
only input is the struct's own fields, which the repro sets the ordinary way.

## Class

logic

## Root cause

`Line::eval` calls `Point::lerp` -> `Vec2::lerp`, computed as `self + t * (other - self)`. When
`self`/`other` are separated by more than `f64::MAX` (e.g. `p0 = 1e308`, `p1 = -1e308`), `other -
self` overflows to `+-inf`. At `t == 0.0` this becomes `self + 0.0 * inf == self + NaN == NaN`; at
`t == 1.0` it's `self + 1.0 * inf`, an infinite coordinate instead of the finite `p1`.

`QuadBez::eval`/`CubicBez::eval` evaluate a Horner-style polynomial in `t`/`mt = 1.0 - t` directly
against the control points, e.g. cubic: `p0*mt^3 + (p1*(mt^2*3) + (p2*(mt*3) + p3*t)*t)*t`. At `t
== 0.0` (`mt == 1.0`), the `p1*(mt^2*3)` subterm is computed as `p1 * 3.0` *before* it is
multiplied by the outer `t == 0.0` that should zero it out; if `p1` has magnitude greater than
roughly `f64::MAX / 3`, `p1 * 3.0` alone already overflows to `inf`, and `inf * 0.0 == NaN`
poisons the whole sum regardless of how well-behaved `p0`/`p3` are. The symmetric case (extreme
`p2`, evaluating near `t == 1.0`) can leave a stray `inf` term that isn't cancelled by anything,
producing an infinite coordinate instead of `p3`. Same mechanism, one fewer term, for `QuadBez`.

In all three cases the bug is that intermediate terms are allowed to overflow *before* being
scaled down by a factor that would otherwise make them irrelevant - the evaluation order doesn't
special-case the two points (`t == 0.0`, `t == 1.0`) where the exact answer is knowable without
touching the "other" control points at all.

## Introduced by

not determined - present in the crate's very first commit (92feb79 "Start curves crate",
2018-12-25). `git log --follow` on `src/line.rs`/`src/quadbez.rs`/`src/cubicbez.rs` traces the
`eval` implementations (the `lerp`/Horner-polynomial shape) continuously back to that initial
commit; `git show 92feb79:src/cubicbez.rs` already contains the identical
`self.p0 * (mt*mt*mt) + (self.p1 * (mt*mt*3.0) + (self.p2*(mt*3.0) + self.p3*t)*t)*t` formula.
This is not a regression introduced by a later change - it's inherent to the original polynomial
evaluation design, which was never guarded against control points of extreme relative magnitude.

## The fix

Special-case `t == 0.0` and `t == 1.0` in all three `eval` implementations to return the
corresponding endpoint (`p0`/`p1` for `Line`, `p0`/`p2` for `QuadBez`, `p0`/`p3` for `CubicBez`)
directly, bypassing the lerp/polynomial arithmetic entirely at exactly those two points. This is
sufficient because the mathematical value of the Bezier/line polynomial at `t == 0.0`/`1.0` is
*exactly* the corresponding endpoint regardless of the other control points' values - no
approximation is introduced, this only skips computation that was always meant to reduce to the
endpoint and instead lets it produce a wrong intermediate value under extreme inputs.

Verified against more than the one triggering input (CONFIRMATION.md section 7): re-ran the
patched build against the original issue #598 repro (extreme point at `p1` for `Quad`/`Cubic`,
matched endpoints for `Line`) *and* several variants moving the extreme-magnitude point to a
different slot (`p2` instead of `p1`, both endpoints extreme for `Line`, extreme point paired
with `eval(1.0)` instead of `eval(0.0)`) - all variants now return the exact finite endpoint
instead of `NaN`/`inf`.

Before/after (`repro/`, real captured output in `output.txt`):
```
before: Line eval(0): got (NaN, 0.0), expected (1e308, 0.0) -> MISMATCH  [... 3 MISMATCH lines total]
after:  Line eval(0): got (1e308, 0.0), expected (1e308, 0.0) -> OK      [... all 6 lines OK]
```

- Project test suite (`cargo test --release` in `kurbo/`): 185 passed, 2 failed both before and
  after the patch - the same 2 pre-existing failures (`bezpath::tests::test_elements_to_segments_starts_on_closepath`,
  `bezpath::tests::test_must_not_start_on_quad`), unrelated to this change (tracked separately as
  candidate 004.SEGMENTS_LEADING_CLOSEPATH_PANIC in this workspace). No new failures introduced.
  Doctests (`cargo test --release --doc`): 55 passed, 0 failed, both before and after.

## What's not done / open questions

- `Arc::eval` was read but not independently fuzzed/boundary-tested for an analogous
  extreme-magnitude NaN-at-endpoint issue (e.g. via extreme `radii`) - it doesn't share this exact
  lerp/polynomial-overflow mechanism (it samples via `sin_cos` of an angle instead), so it was
  judged out of scope for this candidate rather than confirmed clean.
- `PathSeg::eval`, `BezPath` iteration, and anything else built on top of
  `Line`/`QuadBez`/`CubicBez::eval` (stroking, flattening, offsetting, etc.) inherits this fix for
  free since they all call through the same `eval` entry points, but none of those downstream
  callers were separately exercised with extreme-magnitude inputs this session.
- The fix only guards the exact endpoints `t == 0.0`/`1.0`; values of `t` extremely close to but
  not exactly 0.0/1.0 (e.g. `f64::EPSILON`) can still see degraded precision from the same
  overflow mechanism for sufficiently extreme control points - this is a separate, softer
  precision question (there's no single "exact" correct answer to compare against away from the
  endpoints) that this fix does not attempt to address.
