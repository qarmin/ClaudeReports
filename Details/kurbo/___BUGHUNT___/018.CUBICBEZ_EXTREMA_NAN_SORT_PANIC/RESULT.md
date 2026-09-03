# CUBICBEZ_EXTREMA_NAN_SORT_PANIC

`kurbo/src/cubicbez.rs:762`, `CubicBez::extrema()`, ends with
`result.sort_by(|a, b| a.partial_cmp(b).unwrap())` over the roots `one_coord()` pushed from
`solve_quadratic()`. `f64::partial_cmp` returns `None` when either operand is NaN, so `.unwrap()`
there panics if a NaN root ever lands in `result`. The candidate suspected this is reachable by
constructing a `CubicBez` with non-finite (NaN/Infinity) control points, e.g. as could result from
an upstream degenerate computation (candidate #001's `from_svg_arc` NaN-producing bug) feeding a
downstream cubic.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | release build; 8 hand-picked `CubicBez` cases with all-NaN, single-NaN-per-point, Infinity, and mixed Infinity/NaN control points, each call wrapped in `catch_unwind` | No panic in any of the 8 cases |
| Exhaustive structured grid | release build; every one of 8 special f64 values (`0.0, 1.0, -1.0, NaN, +Inf, -Inf, 1e300, -1e300`) assigned to each of the 8 scalar coordinates (`p0.x..p3.y`) - full 8^8 = 16,777,216 combinations, each `.extrema()` call wrapped in `catch_unwind` | 0 panics out of 16,777,216 combinations |
| Random raw-bit-pattern fuzz | release build (repeated once more in debug/dev build with a smaller N=3,000,000 for cross-check); a small xorshift64 PRNG generates raw `u64` bit patterns reinterpreted via `f64::from_bits` for all 8 coordinates (covers arbitrary NaN payloads, signaling NaNs, denormals, negative zero - not just the canonical `f64::NAN`/`f64::INFINITY` constants); 20,000,000 iterations in release, 3,000,000 in debug, each wrapped in `catch_unwind` | 0 panics in both runs |

Also read `solve_quadratic()` in `kurbo/src/common.rs:246-289` directly: every code path either
checks `!sc0.is_finite() || !sc1.is_finite()` up front (falling back to the linear-equation branch,
which only ever `result.push()`es a root after checking `root.is_finite()`, or pushes the literal
constant `0.0` for the fully-degenerate `c0 == 0.0 && c1 == 0.0` case - note NaN never compares
equal to `0.0`, so that branch is also NaN-safe), or takes the quadratic branch where `root1` is
only ever pushed when it was derived from already-finite `sc0`/`sc1` via finite arithmetic
(`-sc1`, or `-0.5 * (sc1 + arg.sqrt().copysign(sc1))` with `arg` itself checked `is_finite()` and
`>= 0.0` first), and `root2 = sc0 / root1` is only pushed after `root2.is_finite()` is checked.
There is no path in `solve_quadratic()` that can push a NaN value into its returned `ArrayVec`.
`one_coord()` in `cubicbez.rs` additionally filters each root through `t > 0.0 && t < 1.0` before
pushing it into `extrema()`'s own `result` - a NaN root would fail that comparison anyway (NaN
compares false against everything), so even a hypothetical future `solve_quadratic()` regression
that started returning NaN would still be filtered out at this call site, not just at the
`solve_quadratic()` level as it stands today.

## Status

**Not reproducible.** Covered: 8 hand-picked boundary cases (Approach 1), an exhaustive grid of
16,777,216 combinations of 8 special float values across all 8 control-point scalar coordinates
(Approach 2, release build), and 23,000,000 total random raw-bit-pattern trials across release and
debug builds (Approach 3) - none produced a panic. Combined with the code reading above, which
shows `solve_quadratic()` structurally cannot return NaN (every push site is gated behind an
explicit `.is_finite()` check or a literal constant), and that `one_coord()`'s own `t > 0.0 && t <
1.0` filter would additionally reject a NaN root if one were ever produced, this is a structural,
not just empirical, guarantee that `CubicBez::extrema()` cannot reach the `.unwrap()` on
`partial_cmp` via non-finite control points as of the checked commit.

**Fully reproducible**: N/A (Not reproducible - no bug to reproduce).

## Class

N/A (Not reproducible).

## Root cause

Not applicable as a bug - the suspected mechanism does not exist in the current code.
`solve_quadratic()` (`kurbo/src/common.rs`) was written defensively: every branch that pushes a
root into its result checks `is_finite()` on that root first (or pushes a literal `0.0` constant
in the fully-degenerate case, which is also NaN-safe since NaN never compares equal to anything
including `0.0`). This means the `ArrayVec<f64, 2>` it returns to `CubicBez::extrema()`'s
`one_coord()` helper is a strict invariant: it never contains NaN, regardless of what NaN/Infinity
values are fed into its `c0`/`c1`/`c2` coefficients. `one_coord()`'s own `t > 0.0 && t < 1.0`
range filter would be a second independent line of defense even if that invariant were ever
broken. So `result.sort_by(|a, b| a.partial_cmp(b).unwrap())` in `extrema()` only ever sorts
values that are already known-finite by construction - it can panic only if some future change to
`solve_quadratic()` or `one_coord()` broke one of those two guarantees, not from any input
constructible today.

## Introduced by

N/A (Not reproducible - no bug was found to trace to an introducing commit).

## The fix

None needed - no bounded fix applies since no reproducible bug exists. Project test suite: not
run against a code change, since no change was made (N/A).

## What's not done / open questions

- This only checked `CubicBez::extrema()`'s own `sort_by(...unwrap())` call directly, via
  `CubicBez::new()` with adversarial control points (the widest, most general single-process input
  to this exact function - not a stand-in for a specific upstream caller). It did not attempt to
  actually drive a NaN through `from_svg_arc()` (candidate #001) into a downstream `CubicBez` to
  see whether *that* path can even construct a `CubicBez` with non-finite control points at all -
  moot here, since `extrema()` was shown to be safe against non-finite control points regardless
  of how they'd be produced, but worth noting `from_svg_arc()` was not itself re-exercised as part
  of this candidate.
- `QuadBez` (and any other `ParamCurveExtrema` implementor) has its own, separate `extrema()`/
  `solve_quadratic`-adjacent code that was not audited here; if any of them has a `sort_by(...
  unwrap())` or `partial_cmp().unwrap()` pattern that does *not* route through
  `solve_quadratic()`'s finite-only guarantee, that would be a distinct, unchecked candidate.
- The exhaustive grid (Approach 2) covers 8 representative special values per coordinate, not
  literally every f64 bit pattern; the random raw-bit fuzz (Approach 3) covers arbitrary bit
  patterns but only probabilistically (23M samples out of 2^64 possible values per coordinate,
  8 coordinates). Given the structural proof from code reading, a longer campaign is not expected
  to find anything new, but a dedicated `fuzz-bug-repro` corpus is the way to extend this
  empirical coverage further if desired.
