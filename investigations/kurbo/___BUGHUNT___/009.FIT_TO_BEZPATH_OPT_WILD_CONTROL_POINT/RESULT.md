# FIT_TO_BEZPATH_OPT_WILD_CONTROL_POINT

`fit_to_bezpath_opt`/`simplify_bezpath` (at `SimplifyOptLevel::Optimize`) can select a candidate
cubic whose control point is tens of thousands of units outside the input's bounding box, while
`fit.rs`'s own error estimator (`CurveDist::eval_ray`, used inside `fit_to_cubic`) scores that
candidate as an almost-exact fit (`err^2` on the order of `1e-20` against `accuracy = 2.0`), so
the wildly wrong result is silently accepted with no signal anything went wrong. Investigated from
a fully-diagnosed lead in `IDEAS.md`/issue #604 (reporter had already root-caused three
contributing defects and supplied an exact reproduction).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Run the issue's exact reproduction (public API: `simplify_bezpath` + `SimplifyOptions` + a 5-point polyline) via a `cargo run --example`, debug build | `kurbo` at HEAD (ca27349), default (debug) profile | Reproduced immediately, first try: output cubic's second control point at `x = -43264.783879186034`, matching the issue's reported value exactly (`max abs coordinate: 43264.783879186034`) |
| Repeat the same run 3 times | Same debug build, same input, no randomness in the code path | Bit-for-bit identical result all 3 runs (deterministic - pure floating-point computation, no RNG/threading/layout dependence) |
| Instrument `fit_to_cubic`'s candidate loop with a temporary `eprintln!` of `(d0, d1, err2, c)` for every candidate cubic considered, rerun the repro | Same debug build | Found the accepted wild candidate has `d1 = 558.6459122545855` (control-arm length in *unit-chord* terms) against every other candidate seen in the same run having `d0`/`d1` in roughly the `0`-`3` range; `err2` for the wild candidate was `~9.37e-26` to `~3.23e-26` across near-duplicate candidates - i.e. the estimator reports it as essentially perfect. Debug print reverted before writing the fix. |
| Read `fit.rs` around the acceptance path (`fit_to_cubic`, `cubic_fit`, `CurveDist::eval_ray`, the `D_PENALTY_ELBOW`/`D_PENALTY_SLOPE` multiplicative penalty) | Manual code reading | Found the exact mechanism (see Root cause) and a comment already marking the spot (`// We could implement a maximum d value here.`) where the missing absolute bound belongs |

## Status

**Confirmed.** Reproduces 3/3 runs, debug build, deterministic (no seed/layout dependence - pure
`f64` arithmetic on a fixed literal input). Matches the issue's own numbers exactly
(`-43264.783879186034`).

**Fully reproducible**: **Yes**. This is a single-participant finding: a caller passes a `BezPath`
directly into the public `simplify_bezpath` API, exactly as a genuine caller would (no second
participant/peer is being impersonated - see `CONFIRMATION.md` section 11, which does not apply
here). `repro/` calls that same public API on the literal input from the issue.

## Class

logic

## Root cause

Three factors compound (all three were independently diagnosed and verified by the issue's
reporter; this session independently re-derived and confirmed the third one, which is what the
fix in this candidate targets):

1. **`simplify.rs`'s corner-detection test is unsigned.** `last_tan.cross(this_tan).abs() >
   last_tan.dot(this_tan).abs() * angle_thresh` is equivalent to `|tan(theta)| > thresh`, which is
   periodic with period `pi` - a near-180-degree reversal (a fold-back) scores identically to a
   shallow bend, so the simplifier hands a folded, effectively-reversing run of points to the
   cubic fitter as if it were smooth, instead of splitting at the fold.
2. **The moment-integral arithmetic in `fit_to_cubic` (`kurbo/src/fit.rs`) is done in absolute
   path coordinates and is not translation-invariant in practice**, even though the final
   quantity it computes (`area`, the x-moment) is supposed to be. With the path's `y` coordinates
   around 324 (versus an O(1)-to-O(80) signal of interest), the raw values returned by
   `source.moment_integrals(range)` and the chord-derived terms subtracted from them in
   `fit_to_cubic` (`area -= dx * (y0 + 0.5 * dy)`, and especially `y -= dx * (y0 * y0 + y0 * dy +
   dy_3 * dy)`) are both on the order of `1e6`-`1e7`, while their difference (the true, meaningful
   moment) is `O(1)`. That is roughly 14 digits of cancellation against `f64`'s ~16, so the `mx`
   handed to `cubic_fit` for this fold is numerically close to noise.
3. **`cubic_fit`'s quartic solver, fed that noisy `mx`, returns a root with an enormous
   control-arm length (`d1 ~= 558.6`, in units of the chord length) that happens to place the
   control point far off to one side and back**, and **`CurveDist::eval_ray`'s error estimate is
   one-sided** (for each *source* sample point, it finds the *nearest* intersection of a
   perpendicular ray with the *candidate* curve) - a candidate whose long excursion still passes
   exactly through every source sample scores as if it were an exact fit. The multiplicative
   penalty `scale_f` (`D_PENALTY_ELBOW`/`D_PENALTY_SLOPE`), applied specifically to suppress
   long-control-arm candidates like this one, cannot rescue an `err2` that is already
   `~1e-20`-`~1e-26`: even a scale factor in the millions leaves the scaled error far under
   `acc2 = 4.0`, so the wild candidate is accepted as the best (in fact only sufficiently-low-error)
   fit.

This candidate's fix addresses (3) directly: there was already no code path that rejects a
candidate purely for having an absurd control-arm length, independent of its (unreliable, in this
case) measured error - the comment `// We could implement a maximum d value here.` in `cubic_fit`
marks exactly where that check was missing.

(2) and (1) are real, independently-diagnosed contributing defects in their own right (see the
issue's own "Diagnosis" section for the reporter's instrumentation of the catastrophic
cancellation), but were not independently fixed by this candidate - see "What's not done" below.

## Introduced by

5196345f6ec7697a5491f8b819338fe2e2af82a1 "Penalize large d values in cubic fit" (2023-04-17) -
this is the commit that introduced the `d0`/`d1`-based multiplicative penalty mechanism itself
(`D_PENALTY_ELBOW`/`D_PENALTY_SLOPE`/`scale_f`) and, in the same change, added the comment `// We
could implement a maximum d value here.` immediately next to the unconditional `if d0 >= 0.0 &&
d1 >= 0.0` accept check - i.e. the commit that both introduced the (insufficient) mitigation and
explicitly flagged, but did not implement, the missing absolute bound this candidate's fix adds.
Found via `git log -S"maximum d value" -- kurbo/src/fit.rs` (also confirmed via `git log -S`
against the `D_PENALTY_ELBOW` symbol, same commit) and inspecting the commit's diff directly. Note
this predates defect (2)'s own introduction (the absolute-coordinate moment computation traces
back further, at least to "Rework moments in ParamCurveFit", 2022-12-27) - not independently
tracked down this session since the fix applied here doesn't touch that code.

## The fix

Added an absolute cap (`MAX_ARM_LEN = 10.0`, in units of chord length - well above the `0`-`3`
range observed for every legitimate candidate seen while instrumenting the repro, and above the
`D_PENALTY_ELBOW = 0.65` value that already starts penalizing candidates, but far below the
`558.6` seen for the wild one) on `d0`/`d1` inside `cubic_fit`'s candidate filter. A candidate
whose control arm exceeds the cap is rejected outright (returns `None` from the `filter_map`),
independent of what `err2`/the multiplicative penalty scale would otherwise compute for it. This
directly targets defect (3) above: no matter how small (or numerically meaningless) the measured
error is, an outright absurd control-arm length is never accepted.

- Before the fix: `repro/` reports `REPRODUCED - a control/end point escaped the input bounding
  box (+/-10 slack) by 43670.52387918603 units` (exit code 1). See `output.txt`'s first half.
- After the fix (patch applied to the same clone): `repro/` reports `did not reproduce - every
  point stayed within the input bounding box` (exit code 0); the final cubic segment for the
  problem range collapses to `C493.9300000000002,324 415.74,324 415.74,324` - effectively a
  straight line through the collinear points, which is the geometrically correct fit for that
  segment. See `output.txt`'s second half.
- Verified against a second boundary point in the same class, not just the exact triggering
  input: the debug instrumentation (see "What was tried") showed several *other*, unrelated
  candidates in the same `fit_to_cubic` call with `d0`/`d1` in the ordinary `0`-`3` range and
  correspondingly ordinary `err2` values continuing to be accepted unaffected by the new cap
  (`MAX_ARM_LEN = 10.0` sits well above all of them) - the fix only removes the one pathological
  root, not legitimate low-arm-length fits.
- Project test suite (`cargo test -p kurbo`, the crate this fix touches):
  - Before: 187 unit tests passed, 55 doctests passed, 0 failed.
  - After: 187 unit tests passed, 55 doctests passed, 0 failed (identical counts, no new
    failures).

## What's not done / open questions

- Defects (1) (`simplify.rs`'s unsigned corner test) and (2) (the catastrophic-cancellation
  moment-integral arithmetic in `fit_to_cubic`/`SimplifyBezPath::moment_integrals`) are real,
  independently diagnosed by the issue reporter, and not fixed by this candidate. The issue's own
  suggested fix #1 (make the corner test signed, in `simplify.rs`) is described as removing the
  *reported* symptom on its own; this candidate's fix instead closes off defect (3) directly, so
  the two are complementary, not redundant - a future session could reasonably treat "make the
  corner test signed" and "compute moment integrals relative to the range's start point" as
  further, separate candidates.
- `MAX_ARM_LEN = 10.0` is a heuristic threshold picked from the observed `0`-`3` range of ordinary
  candidates in this one repro; it has not been tuned against the project's broader fitting test
  suite/corpus (which passed unchanged, but wasn't specifically designed to probe this boundary).
  A tighter or more principled bound (e.g. derived from `D_PENALTY_ELBOW` or from the segment's
  own tangent angles) might be preferable to a maintainer, but a generous, clearly-safe absolute
  cap was chosen here to avoid rejecting any legitimate fit.
- `fit_to_bezpath` (the non-`Optimize`, `Subdivide`-style / non-optimized fitting entry point) was
  not checked for the same class of issue - the issue itself notes `SimplifyOptLevel::Subdivide`
  on the same input does not exhibit the bug, and this session did not independently verify why
  (most likely: `Subdivide` never calls into `cubic_fit`'s quartic-root candidate search the same
  way, or splits before reaching the same degenerate fold).
- The one-sided nature of `CurveDist::eval_ray` (never detecting a candidate that "cheats" by
  passing through every source sample while excursing wildly in between) remains true in general -
  this fix only prevents that blind spot from being reachable via an absurd control-arm length; a
  differently-shaped pathological candidate that stayed within a smaller arm length but still
  excursed between samples is not ruled out by this fix and was not investigated this session.
