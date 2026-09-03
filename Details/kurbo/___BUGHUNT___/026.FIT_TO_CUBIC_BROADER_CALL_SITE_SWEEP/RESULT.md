# FIT_TO_CUBIC_BROADER_CALL_SITE_SWEEP

A broader sweep of `fit_to_cubic`'s call sites/shared safety valve in `kurbo/src/fit.rs`, beyond
the two already-fixed defects (candidate 008's build-loop `unwrap` panic, candidate 009's extreme
wild-control-point case with an arm length of ~558). This candidate was explicitly flagged as an
open follow-up in 009's own "What's not done" section: "A differently-shaped pathological
candidate that stayed within a smaller arm length but still excursed between samples is not ruled
out by this fix and was not investigated this session." This session investigated exactly that,
and confirmed it: candidate 009's chosen mitigation (`MAX_ARM_LEN = 10.0`) is not tight enough - an
ordinary, non-adversarial 4-vertex polyline with a single sharp corner already produces a visibly
wrong control point with an arm length of ~7.04, comfortably under that cap.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read every call site of `fit_to_cubic` in `fit.rs` (4 total: `fit_to_bezpath_rec` line 194, the initial single-cubic check and the `.unwrap()` build loop in `fit_to_bezpath_opt_inner` lines 595/646, and `measure_one_seg` used internally by the search-pass helpers) to look for other "coarser-accuracy-pass accepted without re-verification" patterns beyond 008/009 | Manual code reading | Found no *additional* distinct call-site defect of that specific shape - the only two "two-pass accuracy mismatch" spots are the ones already fixed in 008/009. This ruled out finding a *third, distinct call site*, but motivated checking whether the already-*mitigated* defect (009's `MAX_ARM_LEN` cap) actually closes the underlying `CurveDist` blind spot it targets, or only the one extreme case it was tuned against |
| Short property-fuzz harness (own `kurbo/examples/fuzz026.rs`, deleted before revert; debug build) generating random polylines (later widened to 3-40 vertices, coordinate scale 1 to 1e7, accuracy 1e-4 to 50) fed through the public `simplify_bezpath` API at both `SimplifyOptLevel::Optimize` and `::Subdivide`, checking for panics, non-finite output, a coarse Hausdorff-style accuracy-violation check, and (the oracle actually validated here) control points straying far outside the input polyline's own bounding box - the same oracle already validated by candidate 009's own repro | Debug build, `cargo run --example`, 50,000 iterations against a tree with 008's and 009's patches (`MAX_ARM_LEN = 10.0`) pre-applied (to avoid rediscovering those two already-recorded defects) | 1 failure: seed 12494, a control point at (6504.4, 5457.8) against an input bounding box of roughly [-957,767]x[-686,930] - the same `CurveDist`-blind-spot mechanism as 009, but with `d1 ~= 7.04`, under 009's own `MAX_ARM_LEN = 10.0` cap, so not rejected by that fix. (An initial version of this harness also generated random *cubic* source paths and flagged many more "failures" there - those were re-checked and are **false positives**: Bezier control points routinely lie outside a curve's own tight bounding box for legitimate, correctly-fitting curvy input, so the bounding-box oracle only holds for a *polyline* source, exactly as candidate 009's own repro used it. The final harness and this result use polyline sources only.) |
| Minimized the failing case by hand (from the fuzzer's 10-vertex random polyline down to the 4 vertices around the one sharp corner that reproduced it) and re-ran against a fresh clone of vanilla HEAD with *neither* 008 nor 009 applied | Release build, real clone at ca27349 | Reproduces identically (same wild control point, same coordinates) on plain vanilla HEAD - this is an independent manifestation of the underlying defect, not something introduced or exposed by 008's or 009's own patches |
| Instrumented `fit_to_cubic`'s candidate loop with a temporary `eprintln!` behind an env-var guard (reverted before finalizing) to print every candidate's `d0`/`d1`/raw and scaled `err2`/chord length | Same debug build, minimized 4-vertex repro | Confirmed the accepted candidate has `d0 ~= 0.000115`, `d1 ~= 7.037`, `raw_err2 ~= 1.04e-4` (already tiny before the multiplicative penalty even applies), `acc2 = 4.0` (`accuracy = 2.0`) - the call site is the *build-pass* `fit_to_cubic(source, t0..t1, accuracy)` call inside `fit_to_bezpath_opt_inner`'s per-segment loop (candidate 008's fixed call site), here returning `Some` with a bad candidate rather than `None`/panicking |
| Repeat-run the minimized repro 3 times against the same pristine clone | Release build, no RNG/seed involved | Bit-for-bit identical output all 3 runs |
| Tried tightening `MAX_ARM_LEN` from 10.0 down to 3.0 (matching the "0-3 range observed for every legitimate candidate" already noted in 009's own RESULT.md) and re-ran the same property-fuzz harness, widened further (150,000 iterations, 3-40 vertices, scale 1 to 1e7, accuracy 1e-4 to 50) | Debug build | 0 failures at `MAX_ARM_LEN = 3.0`, both against a tree with 008/009 pre-applied and against plain vanilla HEAD with only this candidate's own standalone patch applied |

## Status

**Confirmed** - reproduces 3/3 runs, deterministic, release build, on a fresh clone of vanilla
HEAD (no 008/009 patches needed or involved).

**Fully reproducible**: **Yes**. `repro/` calls the real public `simplify_bezpath` API (via
`kurbo::simplify`) on a real, ordinary `BezPath` built from a plain polyline - the same kind of
input a genuine caller of `simplify_bezpath(..., SimplifyOptLevel::Optimize)` would pass in. There
is no second participant to fake here (a single-process pure-function logic bug on a specific
polyline/accuracy combination, not a peer/protocol interaction), so `CONFIRMATION.md` section 11's
two-participant recipe does not apply.

## Class

logic

## Root cause

Same underlying mechanism as candidate 009, demonstrated via a different (non-extreme, ordinary)
trigger and shown to survive 009's own chosen mitigation:

`fit_to_cubic`'s candidate quartic solver (`cubic_fit`) can return a root with a large control-arm
length (`d0`/`d1`, in units of chord length) whenever the boundary tangent angles it's fed
(`th0`/`th1`) are extremely close to - but not exactly - zero (i.e. the two ends of the requested
range are very nearly, but not perfectly, tangent-continuous with the chord). This happens
naturally whenever `fit_to_bezpath_opt_inner`'s optimizing search tries a sub-range whose start
falls exactly at a genuine sharp corner of the source polyline: `SimplifyBezPath::break_cusp`
always returns `None` (a documented, acknowledged limitation - "this method does not report
corners" - not itself a new defect), so no cusp is ever reported there, and the tangent sampled at
that boundary can differ from the chord direction by a value that is numerically tiny but not
exactly zero (floating-point roundoff in `SimplifyBezPath::scale`'s `t * self.0.len()` can even
pick up the *wrong* adjacent segment's tangent at an exact segment-boundary parameter value).
Candidate 009 already root-caused and partially fixed this: `CurveDist::eval_ray`, the cheap
per-candidate error estimator, casts one ray per sample point and takes the *nearest* intersection
with the candidate cubic - a candidate whose arm swings far from the chord and back can still
cross every ray close to the correct point, so its estimated error is spuriously tiny and the
`D_PENALTY_ELBOW`/`D_PENALTY_SLOPE` multiplicative penalty (which only scales an already-near-zero
error) cannot reject it.

009's fix added an absolute, unconditional cap (`MAX_ARM_LEN = 10.0`) on `d0`/`d1`, chosen as
"well above the 0-3 range observed for every legitimate candidate seen while instrumenting" that
one repro. This session's broader sweep shows that bound is not conservative enough: this
candidate's minimized 4-vertex repro produces `d1 ~= 7.04` - visibly wrong (the control point lands
roughly 5x further from the chord than any real vertex in the input, for an ordinary, non-crafted
single-corner polyline), yet still comfortably under the `10.0` cap, so 009's own fix does not
reject it.

## Introduced by

5196345 "Penalize large d values in cubic fit" (2023-04-17) - the same commit candidate 009
identified: it introduced the `d0`/`d1`-based multiplicative penalty mechanism
(`D_PENALTY_ELBOW`/`D_PENALTY_SLOPE`/`scale_f`) and, in the same change, added the comment "We
could implement a maximum d value here." immediately next to the unconditional `if d0 >= 0.0 &&
d1 >= 0.0` accept check, without implementing that bound. This candidate and 009 both target the
same never-implemented check flagged by that comment; this candidate additionally shows that a
cap chosen without a broad enough empirical sweep (009's own repro alone) under-constrains it.
Found via `git log -S"maximum d value" -- kurbo/src/fit.rs` and inspecting the commit's diff
directly (reused from 009's own lookup, independently re-verified this session).

## The fix

Applied as a standalone patch against vanilla HEAD (does not depend on, and was not combined with,
candidate 008's or 009's own recorded patches - see "What's not done" below for how the three
relate). Adds the same kind of absolute cap 009 proposed, at a substantially tighter value chosen
from this session's own broader empirical sweep rather than from a single repro:

```rust
const MAX_ARM_LEN: f64 = 3.0;
// ...
if d0 >= 0.0 && d1 >= 0.0 && d0 <= MAX_ARM_LEN && d1 <= MAX_ARM_LEN {
```

Verified:

- `repro/` (the minimized 4-vertex polyline): before the fix, a control point at (6504.4, 5457.8)
  escapes the input's bounding box by ~6089 units (exit code 1); after the fix, the same call
  produces a well-behaved 2-cubic-segment fit with every control point inside the bounding box
  (+/-50 slack), exit code 0. See `output.txt`.
- Varied beyond the one triggering input: the same property-fuzz harness (polylines, 3-40
  vertices, scale 1 to 1e7, accuracy 1e-4 to 50), re-run with only this candidate's own
  `MAX_ARM_LEN = 3.0` patch applied to plain vanilla HEAD (no 008/009), found 0 failures in
  150,000 iterations - both the one specific repro and the broader class it represents are
  addressed.
- Project test suite (`cargo test --workspace`, run in a fresh clone of the pinned base commit,
  before and after `git apply patch.diff`, matching the same 187/10/55 counts recorded by 008 and
  009):
  - Before: kurbo lib tests 187 passed / 0 failed; polycool lib tests 10 passed / 0 failed;
    combined doctests 55 passed / 0 failed (1 ignored, pre-existing/unrelated).
  - After: kurbo lib tests 187 passed / 0 failed; polycool lib tests 10 passed / 0 failed;
    combined doctests 55 passed / 0 failed (1 ignored) - identical counts, no regressions.
  - One separate, unpatched re-run of the full workspace suite showed `poly::tests::
    planted_root_deg5` fail (9/10 polycool tests passed); three immediate reruns of just that
    test package all passed 10/10. This is the pre-existing, independently-documented flaky/
    ill-conditioned test already recorded in this workspace's own candidate
    017.POLYCOOL_PLANTED_ROOT_ILL_CONDITIONED_TOLERANCE - unrelated to `fit.rs` and to this
    candidate's patch. See `output.txt` for the full detail.

## What's not done / open questions

- **This candidate's `patch.diff` and candidate 009's own `patch.diff` both touch the same
  mechanism (adding `MAX_ARM_LEN` at the same location in `cubic_fit`) with different values (3.0
  here vs. 10.0 in 009) and cannot both be applied at once without a merge conflict** - by this
  workspace's design, neither patch is ever applied for real to the main project, so this is not a
  problem for the record itself, but a maintainer who wants to take one of these should treat them
  as alternatives (or take this candidate's value as a straightforward tightening of 009's same
  constant) rather than trying to apply both. This candidate's value (3.0) is validated against a
  substantially broader randomized sweep (tens of thousands of inputs across two sessions/three
  runs, up to 150,000 iterations, vs. 009's own single triggering repro) and is recommended as the
  one to keep if only one is taken.
- **The underlying `CurveDist::eval_ray` blind spot itself (a one-sided nearest-ray-intersection
  check that can be fooled by any candidate that swings away from the chord and back between
  sample points) is still not directly fixed** - both this candidate's and 009's fixes are
  external caps on the symptom (control-arm length), not a fix to the estimator's own blind spot.
  A candidate whose control arm is *even shorter* than 3.0 chord-lengths but still manages to
  "cheat" the discrete sample check (e.g. by oscillating within the gaps between the 20 fixed
  sample points `CurveDist::from_curve` uses) is not ruled out by either fix and was not found or
  specifically searched for this session; a more targeted fix (adding intermediate sample checks,
  or using arc-length parametrization more broadly rather than only for curves already flagged
  "spicy") is out of scope for a single-session bounded fix and is left as an open question for a
  maintainer.
- 150,000 iterations at `MAX_ARM_LEN = 3.0` is a broad but still finite empirical sweep, not a
  proof; a longer, dedicated fuzzing campaign (via `fuzz-bug-repro`, run separately) against
  `simplify_bezpath`/`fit_to_bezpath_opt` would be a reasonable next step if a maintainer wants
  stronger assurance before picking a final constant.
- The random-cubic-source branch of this session's own fuzz harness was found to have an invalid
  oracle (see "What was tried" above) and was dropped rather than fixed; a similarly-shaped sweep
  over genuinely curvy (non-polyline) source curves, with a *correct* oracle (e.g. checking
  against the sampled trace of the source curve rather than raw control points), was not attempted
  this session and might be worth a future candidate.
