# LINE_CROSSING_POINT_NEAR_ZERO_DETERMINANT_WILD_RESULT

`Line::crossing_point` (kurbo/src/line.rs) only special-cases an *exactly* zero cross-product
determinant (`pcd == 0.0`) before dividing by it. It is called from `CubicBez::try_approx_quadratic`
(kurbo/src/cubicbez.rs), itself reachable from the public `CubicBez::approx_spline`/`to_quads` -
looked suspicious because a near-zero-but-nonzero `pcd` with large-magnitude/denormal inputs can
make the division overflow, and the result is used unchecked as a quadratic control point.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (arithmetic reasoning first, then a plain f64 scratch program) | Constructed `ab`/`cd` vectors where one line's displacement has an exactly-zero component and the other has a denormal (`~1e-320`) component, driving `pcd` to a nonzero denormal and the division `h = numerator/pcd` to overflow | Found `h` overflows to `+-inf`, and `cd * h` then produces a `0.0 * inf = NaN` component in the resulting point - confirmed with a standalone f64 script before touching kurbo itself |
| Real end-to-end repro via the public API (`CubicBez::approx_spline`), debug build | `CubicBez::new((0,0), (1, 1e-320), (0,1), (1,1)).approx_spline(0.1)` | Deterministic stack overflow / abort (3/3 runs) |
| Same repro, release build | same input | Deterministic stack overflow / abort (1/1 run, then re-confirmed via `just test` from a fresh clone) |
| Same repro, two additional inputs in the same class (different axis/sign for the exactly-zero component and the denormal) | `(1e-320, 1.0)` swapped onto the other axis; a negated-denormal variant | `swap-axes` also stack-overflows on the unpatched tree; a `neg`-sign variant happened not to trigger it (h still overflowed but the resulting NaN never made it past `fit_inside`'s initial bound check for that particular geometry) - kept `swap-axes` in the shipped `repro/` as the second same-class input, dropped `neg` since it wasn't itself a positive case |

## Status

**Confirmed**

Reproduces deterministically: 3/3 debug-build runs and 1/1 release-build run of the direct repro,
plus a further clean run of the standalone `repro/` against a *fresh clone* at `base_commit` via
`just test` (see `output.txt`) - both the `orig` and `swap-axes` inputs stack-overflow/abort on the
pristine tree.

**Fully reproducible**: **Yes**. The repro calls the real public entry point
(`CubicBez::approx_spline`) with a single, directly-supplied cubic - the same shape of external
numeric input a real caller (e.g. font-outline/curve-fitting tooling constructing or receiving a
cubic with degenerate/extreme control points) would hand to this API. There is no second
participant to fake here (no peer/caller boundary is being stood in for) - this is the "malformed
external artifact handed to a public API" shape CONFIRMATION.md section 11 says needs no such
justification, the same way a malformed file handed to a parser's real `open_file` doesn't.

## Class

crash (deterministic stack overflow via unbounded recursion)

## Root cause

`Line::crossing_point` computes `h = ab.cross(self.p0 - other.p0) / pcd` and only guards against
`pcd == 0.0` exactly. When the two lines are nearly (but not exactly) parallel and one line's
displacement vector (`cd`) has an *exactly* zero component, `pcd` can be a tiny nonzero (even
denormal) value while the numerator stays normal-magnitude, so `h` overflows to `+-infinity`. The
returned point is then `other.p0 + cd * h`: the zero component of `cd` multiplied by an infinite
`h` is IEEE-754 `0.0 * inf = NaN`, so `crossing_point` returns a point with one infinite and one
`NaN` coordinate instead of `None` or a bounded value.

`CubicBez::try_approx_quadratic` takes that point as its candidate quadratic control point `q1`
without any finiteness check, builds a difference cubic from it, and calls `fit_inside` on it.
`fit_inside`'s only two loop-terminating conditions are `hypot() <= distance` (accept) and
`mid.hypot() > distance` (reject) - both comparisons are `false` for a `NaN` operand under
IEEE-754, so neither ever fires once a `NaN` control point is in play. `fit_inside` then falls
through to `self.subdivide()` and recurses on both halves, which stay `NaN` forever (arithmetic on
`NaN` stays `NaN`) - an unbounded recursion that overflows the stack rather than an infinite loop a
watchdog could interrupt.

## Introduced by

138c89e "Two utility functions" (2021-09-07) introduced the `pcd == 0.0`-only check in the
original `Line::intersects` (later renamed `crossing_point` in f12b52f). The same-day commit
f043ea2 "Approximate a cubic with a single quadratic" (2021-09-07) is what actually wires this
unchecked result into the public curve-fitting path via `try_approx_quadratic`, and 43dacf3 "fit_inside
function" (2021-09-07, same day) is what gives the resulting NaN control point its unbounded-recursion
consequence (fit_inside's two termination checks both being false for NaN). All three are needed
together to produce the observed stack overflow; 138c89e is cited as the most direct match to this
candidate's own description (crossing_point's determinant check).

## The fix

Reject a non-finite `h` in `Line::crossing_point` before using it, returning `None` the same way
the existing `pcd == 0.0` case already does:

```rust
let h = ab.cross(self.p0 - other.p0) / pcd;
if !h.is_finite() {
    return None;
}
Some(other.p0 + cd * h)
```

This is sufficient because once `h` itself is finite, `cd * h` cannot introduce a `0 * inf = NaN`
component (both operands finite), so the returned point is always finite whenever `Some` is
returned - the same guarantee callers already implicitly assumed. Verified against both inputs in
`repro/` (the original `orig` case and the independently-constructed `swap-axes` case, which has
the exactly-zero component on the other axis) - both stack-overflow on the pristine tree and both
complete normally, returning a bounded `QuadSpline`, on the patched tree. Also spot-checked with a
third (`neg`-sign) variant during investigation, though that one did not reproduce the crash on the
unpatched tree in the first place (see "What was tried"), so it isn't part of the shipped repro.

- Project test suite (`cargo test --workspace`): before = 187 kurbo unit tests + 10 polycool unit
  tests + 55 doctests passed, 0 failed, 1 doctest ignored; after = identical (187 + 10 + 55 passed,
  0 failed, 1 ignored) - no new failures, no regressions.

## What's not done / open questions

- `CubicBez::try_approx_quadratic` itself still has no defensive finiteness check on `q1` before
  using it - the fix closes the specific unbounded-source (`crossing_point`) but a different future
  caller of `crossing_point`-like wild results, or a different route to a non-finite quadratic
  control point, would not be caught by this patch. Not changed this session, since the description
  and the confirmed mechanism both center on `crossing_point` itself.
- `fit_inside`'s own lack of a NaN-safe termination check (both its bounding comparisons are
  silently `false` for NaN, causing unbounded recursion for *any* NaN control point, regardless of
  source) is a second, independent hardening opportunity not fixed here - it would still allow the
  same class of unbounded recursion if a NaN control point ever reached it through some other path.
  Worth a maintainer decision on whether to also add a `.is_finite()`/depth-limit guard directly in
  `fit_inside` as defense in depth; left alone here to keep the fix narrowly scoped to the confirmed
  root cause in `crossing_point`.
- Other callers of `Line::crossing_point` in the crate (e.g. any offset/intersection code) were not
  swept for the same near-zero-determinant class of issue this session - only the
  `try_approx_quadratic` path named in this candidate was investigated end-to-end.
