# YUKSEL_FIND_ROOT_NONPOSITIVE_X_ERROR_POTENTIAL_HANG

`polycool/src/yuksel.rs:9-79`, `find_root()` (Yuksel's robust Newton-bisection root finder). The
loop `while step.abs() > x_error && x.is_finite() { ... }` never validates that `x_error` (the
caller-supplied root accuracy, which is `nearest()`'s/`roots_between()`'s `accuracy` parameter
passed straight through, fully reachable via the public `ParamCurveNearest::nearest(p, accuracy)`)
is positive. The candidate theorized that if a Newton step underflows to exactly `0.0` at a point
that is not the true root (`val_x` a tiny nonzero rounding residue, `deriv_x` large enough that
`val_x / deriv_x` rounds to `0.0`), `new_x == x`, and `x_error <= 0` removes the normal
`step.abs() <= x_error` exit, this could be an infinite loop with no progress.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual invariant reasoning over the algorithm's structure | reading `find_root` line by line | Every iteration sets `lower = x` or `upper = x` *before* computing the Newton step from the same `x`. If the resulting `new_x` equals `x` (the fixed-point/underflow scenario), it necessarily equals whichever of `lower`/`upper` was just set to `x`, so the very next check (`new_x <= lower \|\| new_x >= upper`) is true by construction and the bisection fallback fires immediately, replacing `x` with the midpoint of the (already shrunk) interval - or returning outright if that midpoint already equals an interval endpoint (adjacent floats). This makes the loop's termination depend only on geometric interval bisection (bounded by float precision, independent of `x_error`'s sign/magnitude), not on `x_error` being positive. |
| Direct pub(crate) unit-test harness against the real `find_root`, with hand-constructed adversarial `f`/`deriv` engineered to force the theorized underflow-to-`0.0` fixed point on every iteration (`f` returns a constant `+-1e-300` depending on which side of a `target` `x` is on, `deriv` returns a constant `1e300`, so `-val_x/deriv_x` always rounds to `-0.0`/`0.0`) | debug build (`cargo test`, debug assertions on), 4 test functions, `x_error` in `{-1.0, 0.0, -1e300, -inf}`, plus a wide-interval variant (`[-1e10, 1e10]`), plus an adjacent-representable-floats interval, plus 500 random targets with `deriv() == 0` everywhere (forcing `+-inf` Newton steps, i.e. the interval-out-of-bounds bisection path on literally every iteration) | All terminated in 1-73 iterations (well under the 100,000/2,000-iteration budget asserted in each test); none hung, none panicked. See `internal_test/yuksel_bughunt_034_append.rs` and `output.txt`. |
| Real public-API end-to-end check | release build, `CubicBez::nearest(p, accuracy)` on 2 different cubics, `accuracy` in `{-1.0, 0.0, -1e300, -inf, -f64::MIN_POSITIVE}` | All 10 calls returned in under 2 microseconds each; none hung. See `repro/src/main.rs` and `output.txt`. |

## Status

**Not reproducible.** Covered: a structural/invariant proof (interval bisection fallback fires
unconditionally whenever a Newton step fails to make strict progress, regardless of `x_error`'s
sign), a direct-call adversarial test harness specifically engineered to hit the exact underflow
scenario the candidate hypothesized (across 4 explicit `x_error` values plus 500 randomized
targets under an even more adversarial all-zero-derivative variant), and a real end-to-end check
through the public `nearest()` API with the same non-positive `accuracy` values. None hung or
looped unboundedly.

**Fully reproducible**: N/A (Not reproducible - no bug to reproduce).

## Class

N/A (Not reproducible).

## Root cause

Not applicable as a bug - the theorized mechanism does not exist in the current code. The
candidate's reasoning assumed that once a Newton step underflows to `new_x == x`, the loop could
simply repeat that same non-progress forever. But `find_root` always updates `lower` or `upper`
to the *current* `x` (based on the sign of `f(x)` relative to `val_lower`) immediately before
computing that iteration's Newton step - so a Newton step that fails to move `x` at all is, by
construction, always caught by the very next `new_x <= lower || new_x >= upper` check in the same
iteration (since `new_x == x == lower` or `new_x == x == upper` exactly), forcing the bisection
fallback (`new_x = lower + (upper - lower) / 2.0`) instead of silently repeating. That fallback
either strictly shrinks the bracket (guaranteeing eventual termination, bounded by the number of
representable doubles between the original `lower`/`upper` - a few dozen to ~1074 iterations in
the worst case, never observed above 73 in any test here) or, if the bracket has already shrunk to
two adjacent representable floats, returns immediately via the existing `new_x == upper || new_x
== lower` early-return. This is the standard guarantee of Yuksel-style robust Newton/bisection
hybrids: bisection is used as a fallback to force geometric convergence whenever Newton fails to
make bounded progress, independent of the caller's requested tolerance. `x_error`'s sign/value
only ever affects the loop's *normal* early exit (`step.abs() > x_error`); it plays no role in the
bisection fallback's own convergence, which is what actually bounds the iteration count once
`x_error <= 0` disables that normal exit.

## Introduced by

N/A (Not reproducible - no bug was found to trace to an introducing commit).

## The fix

None needed - no bounded fix applies since no reproducible bug exists. Project test suite: not
run against a code change, since no change was made to the main project's working tree (N/A). (The
adversarial test module was only ever added inside the isolated worktree/clone used to
investigate this candidate, and was reverted there before this candidate folder was written; see
`internal_test/` for a durable copy of what was run.)

## What's not done / open questions

- This checked `find_root` directly (the real pub(crate) function, called from inside its own
  crate via an injected test module, since it isn't reachable from outside `polycool`) and the one
  real public caller chain named in the candidate (`CubicBez::nearest` -> `roots_between` ->
  `find_root`). It did not exhaustively check every other `ParamCurveNearest`/`Poly::roots_between`
  call site in `kurbo` (e.g. other curve types' `nearest()` implementations) for a *different*
  non-`find_root` unbounded-loop bug under non-positive accuracy - though since they all funnel
  into the same `find_root`, and this candidate shows `find_root` itself is bounded regardless of
  `x_error`, that residual risk looks low.
  - Same underlying bug class as already-confirmed candidates 005/007/021/028 (unguarded
    non-positive accuracy causing unbounded work elsewhere in this codebase) - `find_root`
    specifically turns out to already be structurally immune to it, unlike those other spots.
- The adversarial `f`/`deriv` pairs used here are hand-constructed to be maximally hostile to the
  Newton branch (constant tiny value / huge derivative, or an all-zero derivative), not derived
  from a real cubic/quintic polynomial's actual coefficients - but this is a stronger, not weaker,
  test of the theorized mechanism specifically, since it removes any dependence on whether a real
  polynomial's Newton dynamics happen to produce the exact underflow coincidence at all.
