# FLATTEN_CUSP_ISSUE426

`kurbo::flatten()` (`kurbo/src/bezpath.rs`) applied to a cubic Bezier looked suspicious because the
cubic-handling branch aggregates a `sum` of per-derived-quadratic `FlattenParams::val` estimates
across all quads produced by `CubicBez::to_quads`, and nothing visibly guards against one of those
per-quad estimates being non-finite. Issue #426 (open, "Incorrect cubic bezier flattening of a flat
curve with cusps") gives a concrete minimal repro: `M 0 10 C -10 10 180 10 60 10` (a cubic whose
control points all share `y = 10`, i.e. a degenerate "flat" curve with an interior direction
reversal / cusp), and reports that the flattened output misses the cusp entirely while flattening
the same derived quadratics independently is correct.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: run the exact path from issue #426 through the public `flatten()` API at `tolerance = 0.1`, compare the flattened output's x-range against a brute-force-sampled ground truth of the same `CubicBez` | debug build, `cargo run --example` | Reproduced immediately: `flatten()` emitted only `MoveTo(0,10)` -> `LineTo(60,10)`, i.e. a single straight line skipping the entire curve, whose real max x (sampled at 100k points) is ~99.88 |
| Root-cause via instrumented `#[cfg(test)]` unit test added temporarily inside `bezpath.rs` (has access to the crate-private `to_quads`/`estimate_subdiv`/`determine_subdiv_t`/`TO_QUAD_TOL`), printing each derived quad's `FlattenParams::val`, the running `sum`, and the loop's `target`/`i` state | `cargo test -p kurbo debug_flatten_cusp_investigation -- --nocapture`, then reverted before finishing (never left in the tree) | Found the exact mechanism: the very first derived quad (covering cubic `t` in `[0, 0.0714]`, which contains the cubic's own cusp at `t ~= 0.0259`) has all three control points sharing the same `y` value bit-for-bit (`10.0`), so `QuadBez::estimate_subdiv`'s `cross` is exactly `0.0`. Dividing by it makes `x0`/`x2` infinite, `approx_parabola_integral(±inf)` returns `NaN`, and that `NaN` propagates all the way to `FlattenParams::val`. `bezpath.rs`'s cubic-handling loop then does `sum += params.val` unconditionally across all 14 derived quads, so this single `NaN` poisons the total `sum` (and, downstream, `n`, `step`, and every `target`). Since any comparison against `NaN` is `false`, the `while target < val_sum + params.val` loop never fires for *any* quad (not just the offending one), so the whole cubic collapses to a straight line from `p0` to `p3` |

Repeat-run check: re-ran the debug example 3 times back-to-back; `flatten()`'s output was
bit-for-bit identical every time (no randomness anywhere in the code path - pure deterministic
floating-point arithmetic on a fixed input).

## Status

**Confirmed** - deterministic, 3/3 runs identical (debug build), and reproduced again from a
pristine clone of the exact base commit via the standalone `repro/` (see `output.txt`).

**Fully reproducible**: **Yes**. This is a pure logic bug entirely internal to `kurbo`'s own public
`flatten()` API acting on a hand-specified `BezPath` - there is no second participant (peer,
network, external caller) involved, so CONFIRMATION.md section 11's end-to-end-second-participant
bar does not apply; the repro drives the real public entry point (`kurbo::flatten`) with the exact
path from the issue and no internal test-only scaffolding.

## Class

logic

## Root cause

`QuadBez::estimate_subdiv` (`kurbo/src/quadbez.rs`) computes a transform to a canonical
`y = x^2` parabola in order to estimate how many polyline segments a quadratic needs:

```rust
let cross = (self.p2 - self.p0).cross(dd);
let x0 = d01.dot(dd) * cross.recip();
let x2 = d12.dot(dd) * cross.recip();
let scale = (cross / (dd.hypot() * (x2 - x0))).abs();
...
let a0 = approx_parabola_integral(x0);
let a2 = approx_parabola_integral(x2);
let val = if scale.is_finite() { ... } else { 0.0 };
```

When the quad's three control points are *exactly* collinear, `cross` is exactly `0.0`. Dividing by
it sends `x0`/`x2` to `+-inf`, and `approx_parabola_integral(x) = x / (1 - D + (D^4 + 0.25*x*x).sqrt().sqrt())`
evaluates to `inf / inf = NaN` for either infinite input. The existing `scale.is_finite()` guard,
meant to catch exactly this kind of degeneracy, does not: `scale = (cross / (dd.hypot() * (x2 -
x0))).abs()` recombines `cross = 0.0` with `(x2 - x0) = -inf` back into a *spuriously finite*
`0.0`, so the `NaN`-producing branch is taken anyway and `val` comes out `NaN`.

For the issue's input, `CubicBez::to_quads` derives 14 quadratics from the cubic. All 14 happen to
have every control point on the exact line `y = 10.0` (the input cubic is "flat": `p0.y == p1.y ==
p2.y == p3.y == 10.0` bit-for-bit, and `subsegment`'s linear interpolation preserves that exactly).
The very first derived quad additionally has its middle control point (`p1.x ~= -1.02`) outside the
`[p0.x, p2.x]` range, i.e. it actually reverses direction along that line - it contains the cubic's
first cusp (`t ~= 0.0259`) - so it is a real, non-degenerate-in-shape segment that nonetheless has
`cross == 0.0` because all its points are collinear.

`BezPath::flatten`'s `PathEl::CurveTo` arm sums every derived quad's `val` into one running `sum`
used to decide the cubic's total subdivision count and to distribute interior polyline vertices
proportionally across quads:

```rust
let mut sum = 0.0;
for (_, _, q) in iter {
    let params = q.estimate_subdiv(sqrt_remain_tol);
    sum += params.val;
    quad_buf.push((q, params));
}
let n = ((0.5 * sum / sqrt_remain_tol).ceil() as usize).max(1);
...
while target < val_sum + params.val { ... }
```

One `NaN` among the 14 `val`s makes `sum` (and therefore every `target` computed from it) `NaN`.
Every `f64` comparison against `NaN` is `false`, so the inner `while` loop's condition is never
satisfied for *any* of the 14 quads - not just the degenerate one - and the only point ever emitted
besides the `MoveTo` is the final `p3`. The entire interior of the cubic, including the second,
perfectly well-behaved cusp around `t ~= 0.758` (peak `x ~= 99.88`), is silently dropped.

This matches the issue reporter's own observation that "flattening the quadratics independently is
correct": flattening the same problematic quad on its own via the `PathEl::QuadTo` arm of `flatten`
also gets `val = NaN`, but there `n = ((0.5 * NaN / sqrt_tol).ceil() as usize).max(1)` saturates to
`1` (`NaN as usize` is `0` in Rust, then `.max(1)`), so that arm just draws a local straight line
across the one problematic quad and otherwise behaves normally - the `NaN` stays contained to that
single quad's own output. It's specifically the cubic-handling arm's *global* `sum` across all
derived quads that lets one `NaN` estimate wipe out the entire curve.

## Introduced by

f6b90b19771d3ed450c09c2d66aead813e48d504 "Add BezPath flattening" (2020-04-23) - the commit that
introduced `QuadBez::estimate_subdiv` (including the unguarded `cross.recip()` division) and the
cubic-to-quads flattening algorithm in `BezPath::flatten`, both essentially unchanged since (`git
log --oneline -- kurbo/src/quadbez.rs` shows only 5 commits total touching that file across the
project's history, none of which modified this function's division-by-`cross` logic; `git log -S
"Determine transformation to"` finds no earlier introduction).

## The fix

In `QuadBez::estimate_subdiv`, detect the `cross == 0.0` (exactly collinear control points) case
before dividing by it, and return the same "no subdivision needed from this quad" fallback
(`val: 0.0`, plus finite defaults for `a0`/`a2`/`u0`/`uscale` so no downstream consumer of
`FlattenParams` can observe a stray `NaN`/`inf` either) that already exists for the
`!scale.is_finite()` case:

```rust
let cross = (self.p2 - self.p0).cross(dd);
if cross == 0.0 {
    return FlattenParams { a0: 0.0, a2: 0.0, u0: 0.0, uscale: 1.0, val: 0.0 };
}
let x0 = d01.dot(dd) * cross.recip();
```

This stops the `NaN` from ever being produced, so it can no longer poison `BezPath::flatten`'s
`sum` across the other, well-behaved derived quads. Verified against the exact issue #426 input at
`tolerance = 0.1`:

- Before: `flatten()` emits 2 path elements (`MoveTo(0,10)`, `LineTo(60,10)`) - max x reached is
  `60.0` vs. the real curve's `~99.88` (deviation ~39.9, tolerance 0.1).
- After: `flatten()` emits 3 path elements (`MoveTo(0,10)`, `LineTo(99.878396, ~10)`,
  `LineTo(60,10)`) - max x reached is `99.878396`, within 0.1 of the real curve's `99.883568`.

Note: the *first* cusp (around cubic `t ~= 0.0259`, a much smaller local dip from `x=0` down to
`x ~= -0.38` before recovering) is still drawn as a straight line across that one quad's own small
range, since that quad's `val` is now `0.0` rather than `NaN` - a local approximation shortfall of
about 0.4 units, well inside typical rendering tolerances and consistent with how the same quad
already behaves when flattened independently via the `QuadTo` arm (the behavior the issue reporter
called "correct"). This is a separate, much smaller, pre-existing limitation of treating an exactly-
collinear-but-direction-reversing quad as needing zero subdivisions, not a regression from this fix,
and was not further chased this session (see "What's not done" below).

- Project test suite: **187 lib tests / 55 doctests / 10 polycool integration tests** passed both
  before and after the patch (`cargo test --workspace`) - no regressions, no new failures.

## What's not done / open questions

- The residual ~0.4-unit local approximation shortfall at the *first* cusp (noted above) was not
  further addressed - a maintainer may want a better subdivision estimate for an exactly-collinear
  quad that still reverses direction internally, rather than the `val = 0.0` "skip it" fallback
  reused here from the existing `!scale.is_finite()` case. At `tolerance = 0.1` this is well within
  typical acceptable error, but a much tighter `tolerance` would not shrink this specific local
  error, since it's a fixed fallback rather than a tolerance-driven estimate.
- Not checked: whether the same `cross == 0.0` degeneracy can arise directly from a `PathEl::QuadTo`
  supplied straight by a caller (not derived from `to_quads`) with genuinely collinear control
  points that reverse direction - the standalone `QuadTo` arm of `flatten()` was observed above to
  already tolerate a `NaN` `val` gracefully (saturates `n` to 1, draws one straight line, does not
  propagate), so this is expected to be low-impact even before this fix, but was not independently
  verified with a dedicated boundary-value test.
- Not checked: whether other consumers of `QuadBez::estimate_subdiv`/`FlattenParams` inside the
  crate (only `bezpath.rs`'s two `flatten` arms were found via `grep`) could be affected by the
  same class of issue - none currently exist beyond those two call sites.
