# PARAM_CURVE_CURVATURE_CUSP_NAN

`ParamCurveCurvature::curvature`'s default trait-method implementation
(`kurbo/src/param_curve.rs:167-182`) computes `d2.cross(d) * d.hypot2().powf(-1.5)` with no guard
for a zero first derivative (a cusp). This looked suspicious because `offset.rs`'s own
`cusp_sign`/`endpoint_cusp` hand-roll an epsilon-guarded version of the exact same
`curvature * offset + 1` computation for the same reason (a vanishing derivative divides by
zero), but the generic `curvature()` trait method used by `CubicBez`/`QuadBez` never got the same
guard.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: `CubicBez` with `p0 == p1` (start cusp), `curvature(0.0)` | release build, standalone crate depending on kurbo via path | `NaN` on first try |
| Repeat runs of the same input | release build, 3 additional `cargo run --release` invocations | `NaN` every time (4/4 total) |
| Same construction at the other end (`p2 == p3`, `curvature(1.0)`) | release build | Also `NaN` - confirms both cusp endpoints hit it, not just `t=0` |
| Sanity check: an ordinary non-degenerate `CubicBez` at several `t` values | release build | All finite, non-NaN, matches expected curvature values (e.g. `0.6667` at the endpoints of a symmetric S-curve) - confirms the bug is specific to the zero-derivative case, not a general breakage |
| Near-cusp (tiny nonzero derivative, `p1` offset from `p0` by `1e-6`) | release build | Large-but-finite negative curvature (`~-6.67e11`), not NaN - confirms the failure is specifically the exact-zero case (`0.0.powf(-1.5) == inf`, `0 * inf == NaN`), not a general precision issue near cusps |

No sanitizer/miri/fuzzing needed - this is deterministic floating-point boundary-value math on a
public API with a directly-constructible degenerate input (`CubicBez::new` with two duplicated
control points), not `unsafe`/parsing/adversarial-input code, so the first approach (manual
boundary-value construction) already gave a clean, repeatable signal and no escalation was
needed.

## Status

**Confirmed** - reproduces 4/4 runs (1 initial + 3 repeats), release build, no sanitizer/seed
dependency. Deterministic: `0.0_f64.powf(-1.5)` is exactly `f64::INFINITY`, and `Vec2::cross` of
the zero vector against anything is exactly `0.0`, so `0.0 * f64::INFINITY` is exactly `NaN` on
every run, every platform IEEE-754 f64 arithmetic behaves normally on.

**Fully reproducible**: **Yes**. The repro calls `CubicBez::curvature` - a genuine public API
entry point - directly with a value constructed via kurbo's own public `CubicBez::new`
constructor. There is no second participant (peer/caller/wire) whose input is being stood in for
here (see `CONFIRMATION.md` section 11's carve-out): a cusped `CubicBez` (`p0 == p1`) is an
entirely ordinary, directly-constructible value any caller of kurbo's public API can produce, not
something requiring an upstream participant to be faked.

## Class

logic (also arguably a documentation/API-contract gap - the trait method's doc comment makes no
mention that a zero-derivative input yields `NaN`, unlike, e.g., `Affine::inverse`'s and
`Triangle::inscribed_circle`'s documented NaN/Inf-on-degenerate-input behavior noted in
candidate 023's `RESULT.md`)

## Root cause

`curvature(t)` computes the standard curvature formula `k = (d2 x d) / |d|^3` via
`d2.cross(d) * d.hypot2().powf(-1.5)`. At a cusp, the first derivative `d` is the zero vector
(e.g. because two adjacent Bezier control points coincide, `p0 == p1`). `d.hypot2()` is then
exactly `0.0`, and `0.0_f64.powf(-1.5)` is exactly `f64::INFINITY` (IEEE-754: `0^negative ==
+inf`). Meanwhile `d2.cross(d)` (cross product against the zero vector) is exactly `0.0`
regardless of `d2`. The final multiplication is therefore `0.0 * f64::INFINITY`, which IEEE-754
defines as `NaN`. This silently poisons anything downstream that consumes the returned curvature
(e.g. curvature-based subdivision/error-metric code that assumes a finite value), with no panic
or other visible signal at the point of failure.

The default trait method has zero internal callers in the crate itself (`grep` across
`kurbo/src/` found only the trait definition and the three `impl ParamCurveCurvature` blocks for
`Line` (fully overridden, hardcoded `0.0`), `QuadBez` (empty impl, uses the default), and
`CubicBez` (empty impl, uses the default)) - so kurbo's own code never happens to hit this path,
which is presumably why it was never noticed: `offset.rs`'s cusp-handling code needed the same
underlying computation for its own purposes and independently wrote a properly epsilon-guarded
version (`endpoint_cusp`) rather than reusing/fixing this trait method.

## Introduced by

1e556c4 "Add curvature trait" (2018-12-26) - this is the commit that originally added
`ParamCurveCurvature` and this exact `curvature()` default implementation; the vulnerable
`d2.cross(d) * d.hypot2().powf(-1.5)` line (or an equivalent form differing only in whether
`d`/`d2` were computed inline vs. via local variables) has been present unchanged in every
subsequent revision found via `git log -p`/`git blame` on `kurbo/src/param_curve.rs`. The bug has
existed since the feature's inception, not introduced by a later regression.

## The fix

Guard the denominator away from exactly zero before raising it to a negative power, mirroring the
style `offset.rs`'s `endpoint_cusp` already uses for the identical singularity (there:
`tan.to_vec2().hypot().max(TAN_DIST_EPSILON)` on a non-squared magnitude with
`TAN_DIST_EPSILON = 1e-12`; here: the squared-magnitude equivalent, `d.hypot2().max(1e-24)`, since
`d.hypot2()` is already squared and `1e-24 == (1e-12)^2` keeps the same effective distance floor):

```rust
const MIN_DERIV_LEN_SQ: f64 = 1e-24;
d2.cross(d) * d.hypot2().max(MIN_DERIV_LEN_SQ).powf(-1.5)
```

With this guard, `d.hypot2().max(MIN_DERIV_LEN_SQ).powf(-1.5)` is always a large-but-finite
number instead of infinity when `d` is exactly zero, so `d2.cross(d) * (large finite number)` is
`0.0 * (large finite number) == 0.0` (since `d2.cross(d)` is itself exactly `0.0` whenever `d` is
the zero vector) rather than `NaN`. This is a defensible convention (zero curvature reported at an
exact cusp) consistent with `Line::curvature`'s own hardcoded `0.0` for its always-degenerate
case; away from a cusp, the guard is a no-op (`1e-24` is far below any `d.hypot2()` a
non-degenerate curve would ever produce) so ordinary curvature values are bit-for-bit unaffected.

Verified against more than the one triggering input (per `CONFIRMATION.md` section 7):
- Start-cusp (`p0 == p1`, `curvature(0.0)`): `NaN` -> `0.0`.
- End-cusp (`p2 == p3`, `curvature(1.0)`): `NaN` -> `0.0`.
- Ordinary non-degenerate curve at `t = 0.0, 0.25, 0.5, 0.75, 1.0`: unchanged, still finite,
  matching pre-fix values exactly (e.g. `curvature(0.5) == 2.6666666666666665` both before and
  after).
- Near-cusp (tiny nonzero derivative, `1e-6`): still a large finite value (`~-6.67e11`), not
  pushed to `0.0` by the guard and not `NaN` - confirms the guard only engages for the
  exact-zero case, not a broader range that would distort near-cusp curvature.

- Project test suite: `cargo test --workspace` (kurbo + polycool), before and after the patch:
  **252 passed, 0 failed** both times (187 unit/integration tests + 10 in a second test binary +
  55 doctests, 1 doctest ignored, identical before and after - no new failures, no fewer passes).

## What's not done / open questions

- `QuadBez` also uses this same default trait method (empty `impl ParamCurveCurvature for
  QuadBez {}`) and would hit the identical `NaN` at a quadratic cusp (`p0 == p1` or `p1 == p2`);
  not separately re-verified with a `QuadBez`-specific repro this session since the fix is in the
  shared default method both types dispatch to, but a maintainer applying this patch may want to
  add a `QuadBez` regression test alongside a `CubicBez` one.
- No check was done for whether any *downstream* consumer outside kurbo's own codebase (crates
  depending on kurbo) relies on the current NaN-on-cusp behavior in a way this fix would change;
  the fix changes the return value (from `NaN` to `0.0`) for cusped inputs, which is a behavior
  change, not merely a crash fix - worth calling out explicitly to maintainers deciding whether to
  take this patch, even though no internal caller in kurbo itself was found to depend on the old
  behavior.
- Whether `0.0` is the "right" convention for curvature at an exact cusp (vs., e.g., a signed
  infinity, or leaving it as an explicit `Option<f64>`/documented-NaN contract as candidate 023
  did for `Triangle`'s degenerate case) is a design decision left to the project maintainers; this
  fix only guarantees a finite, non-panicking value consistent with `Line`'s existing convention,
  not a claim that `0.0` is uniquely correct.
