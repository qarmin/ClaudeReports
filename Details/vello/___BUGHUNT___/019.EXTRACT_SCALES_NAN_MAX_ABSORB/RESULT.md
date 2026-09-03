# EXTRACT_SCALES_NAN_MAX_ABSORB

`vello_common::util::extract_scales` (`sparse_strips/vello_common/src/util.rs`) extracts
`(scale_x, scale_y)` from an affine transform via an SVD-style closed form, feeding
`gaussian_blur::transform_blur_params`. It downcasts the transform's coefficients to `f32` before
computing `s1` and `s2` independently, then does `scale_y = (0.5*(s1-s2)).sqrt()` - for a
near-singular transform, `s1-s2` can round to a small *negative* `f32` even though it is
mathematically always >= 0, making `.sqrt()` return `NaN`; the subsequent `.max(1e-6)` then
silently replaces that `NaN` with the unrelated 1e-6 floor (`f32::max` returns the non-NaN operand
when exactly one argument is NaN) instead of the small-but-larger, transform-specific value the
singular value actually is.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value/derivation reasoning | Proved algebraically that `s1^2 - s2^2 = 4*((a2+c2)*(b2+d2) - (ab+cd)^2) >= 0` by Cauchy-Schwarz, so `s1 >= s2` always holds mathematically, i.e. any observed `s1 < s2` must be pure floating-point rounding, not a real math flaw | Confirmed the invariant is real but gave no concrete triggering input by itself |
| Short one-off randomized search (release build, plain `rustc -O`, not cargo-fuzz) over exactly `extract_scales`' own `f32` formula, 200,000,000 draws of near-diagonal matrices (`a = s+eps`, `d = s`, `b`/`c` random in [-1,1]) | ~1 second, single run | 180/200,000,000 draws produced `diff = 0.5*(s1-s2) < 0` in `f32`, and `.sqrt()` of that is `NaN` in every hit; confirmed `f32::max(NaN, 1e-6) == 1e-6` (and the reverse argument order) separately |
| Ran the crate's real public API (`vello_common::kurbo::Affine::new` + `vello_common::util::extract_scales`) against 3 of the found triggering coefficient sets, both via a path-dependency against the working checkout and via the standalone `repro/` crate against a fresh clone | `cargo run --release`, 4 total runs against pristine code (this session) | All 4 runs: `scale_y == 0.000001` (the exact floor) for all 3 cases, deterministically, no flakiness - `f64` recomputation of the same formula (never downcasting to `f32`) gives the true values `~5.06e-5`, `~8.19e-5`, `~1.32e-4` respectively (50x-130x larger than the floor substituted) |

## Status

**Confirmed**. Reproduces 4/4 runs (release build, `cargo run --release`, the real public
`vello_common::util::extract_scales` entry point) - fully deterministic, no randomness/seed
involved once the triggering coefficients are fixed.

**Fully reproducible**: **Yes**. `extract_scales` is a pure function over a single caller-supplied
value (an `Affine` transform) with no second participant to fake - the same class CONFIRMATION.md
section 11 says a single-process repro against the real public entry point is sufficient for (a
malformed file handed to a parser's real `open_file`, or here, an affine transform handed to the
real public scale-extraction function). `repro/` calls the crate's actual public API
(`vello_common::kurbo::Affine` + `vello_common::util::extract_scales`), not a hand-simulated
stand-in for it.

## Class

logic

## Root cause

`extract_scales` downcasts the transform's `f64` coefficients to `f32` immediately, then computes
`s1 = a2+b2+c2+d2` and `s2 = sqrt((a2-b2+c2-d2)^2 + 4*(ab+cd)^2)` independently in `f32`.
Algebraically `s1^2 - s2^2 = 4*((a2+c2)*(b2+d2) - (ab+cd)^2)`, which is always >= 0 by
Cauchy-Schwarz, so `s1 >= s2` mathematically for any real matrix - but `s1` and `s2` are computed
through separate rounding paths (one is a plain sum, the other a `sqrt` of a sum of squares), and
for a near-singular transform (determinant close to 0) the true `s1 - s2` is a value many orders of
magnitude smaller than `s1`/`s2` themselves (e.g. `~1e-8` vs. operands of magnitude `~1.4` in the
found cases) - well below what `f32`'s ~7 decimal digits of precision can resolve once `a`/`b`/`c`/
`d` have already been rounded to `f32`. The subtraction then rounds to a small negative number,
`.sqrt()` of a negative `f32` is `NaN`, and `f32::max(NaN, 1e-6)` (used to floor the result "to
avoid division by zero" per the function's own doc comment) silently returns `1e-6` - `f32::max`'s
documented behavior is to return the non-NaN operand whenever exactly one side is `NaN`, with no
distinguishing trace that a `NaN` ever occurred. The floor value (1e-6) is unrelated to the true
scale (which, for the 3 triggering cases found, is 50x-130x larger), so this isn't "the input was
genuinely singular, the floor is appropriate" - it's the precision loss from the premature `f32`
downcast, disguised as a legitimate floor.

This value feeds `gaussian_blur::transform_blur_params` (`sparse_strips/vello_common/src/filter/
gaussian_blur.rs:26-33`): `uniform_scale = (scale_x + scale_y) / 2.0`, `std_deviation *
uniform_scale` - a `scale_y` six orders of magnitude smaller than it should be for a near-singular
(but not exactly singular) blur transform pulls `uniform_scale` down to roughly half of the correct
value, halving the effective device-space blur radius for that draw. This code path is used by
both `filter::mod.rs`'s general filter application and `filter::drop_shadow.rs`.

## Introduced by

ee4394aa "feat(vello_cpu): Image Filters (Filter Effects) implementation (#1286)" (2025-11-21) -
`git log -S`/`git blame` both point to this commit as the one that introduced `extract_scales` in
its current form (the whole function, including the `f32` downcast and the `s1-s2` subtraction, was
added together, unchanged since).

## The fix

Keep the transform's coefficients (and every intermediate quantity: `a2`/`b2`/`c2`/`d2`/`s1`/`s2`)
in `f64` throughout, only converting `scale_x`/`scale_y` to `f32` at the very end (`Affine::
as_coeffs()` already returns `f64`, so this removes an unnecessary early downcast rather than
adding one). `f64`'s ~15-16 decimal digits of precision comfortably resolves the `~1e-9`-relative
differences the found triggering cases need (verified: the patched code's `scale_y` for all 3 cases
matches the independently-computed `f64` "ground truth" almost exactly - see `output.txt`). Also
changed `(0.5 * (s1 - s2)).sqrt()` to `(0.5 * (s1 - s2)).max(0.0).sqrt()` as a defense-in-depth
clamp, since residual `f64` rounding can in principle still occasionally push a genuinely-singular
transform's difference just below zero (this clamp alone, without the `f64` precision fix, would
NOT have fixed the reported cases - verified by testing it in isolation in `f32`: the 3 triggering
cases still produced `scale_y == 1e-6` because the true difference is below `f32`'s noise floor
for these operand magnitudes, not merely a sign-flip of an otherwise-accurate small value).

Checked against 3 different triggering coefficient sets (not just one), found independently by the
randomized search, all in the same near-singular class - all 3 went from `scale_y == 1e-6` (wrong)
to matching their `f64` ground truth (correct) after the fix; see `output.txt` for the full
before/after transcript.

- Project test suite (`cargo test -p vello_common --release`, the only crate that calls
  `extract_scales`): 159 passed; 0 failed; 1 ignored - before the fix. 159 passed; 0 failed; 1
  ignored - after the fix. No regressions, no new failures.

## What's not done / open questions

- Not checked whether `kurbo`'s own internal `Affine::svd()` (which this function's doc comment
  says it duplicates/mirrors) has the same `f32`-downcast issue - it operates in `f64` throughout
  (kurbo's own `Affine` is `f64`-based end to end), so it's very unlikely to hit the same class of
  bug, but this wasn't independently verified this session.
- The concrete user-visible symptom (gaussian blur radius roughly halved for a near-singular,
  non-invertible-looking scale/skew transform) was reasoned about from reading
  `transform_blur_params`'s use of `scale_y`, but no rendered-pixel-level test was built this
  session to visually confirm the blur artifact end-to-end through the full filter pipeline -
  the fix's correctness was verified at the `extract_scales` numerical level (which is what the
  bug is actually about) rather than by rendering a scene.
- Whether a longer/dedicated fuzzing campaign (via `fuzz-bug-repro`, run separately) over
  `extract_scales`' input space would turn up triggering cases with an even smaller true `scale_y`
  (potentially still below even `f64`'s noise floor for pathological operand magnitudes) wasn't
  explored - the 3 cases found and fixed here are all in the "true value comfortably representable
  in f64, just not in f32" regime.
