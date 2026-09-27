# bartlett_hann kernel has the wrong shape (`src/math/bartlett.rs:47-63`)

`bartlett_hann()` (the `ResamplingFunction::BartlettHann` window used by the general resampling
path in `compute_weights.rs`) does not implement the Bartlett-Hann window correctly: it uses a
non-standard constant `0.4832` in place of `0.48`, and never normalizes its input by the kernel's
declared radius (`2.0`, from `ResamplingFilter::new(bartlett_hann, 2f32)`) before applying the
formula. Flagged by a prior `code-audit-structured` pass and picked as the first/cheapest entry in
`IDEAS.md`'s backlog.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (`f64`, standalone `repro/`, calling `bartlett_hann::<f64>(x)` directly) | `x` at `0`, `1`, `2` and swept `x` in `[0,2]` step `0.1` | Confirmed: `bartlett_hann(0) = -0.0016` (should be the peak, `1.0`), rises to `1.0` at `x=0.5`, falls unevenly, reaches `-0.4848` at `x=2` (the support edge) |
| Differential test against the textbook Bartlett-Hann window, `0.62 - 0.48*\|t-0.5\| - 0.38*cos(2*pi*t)` re-expressed over the normalized distance `d = \|x\|/radius` (the domain `compute_weights.rs` actually calls this kernel over, given `ResamplingFilter::new(bartlett_hann, 2f32)`'s `min_kernel_size == 2.0`) | Standalone `repro/`, `f64`, 4001 sample points over `x` in `[-2,2]`, plus a 201-point symmetry check and a 2001-point monotonicity/non-negativity sweep | Confirmed: max abs error `1.0016` at `x=0` (the centre); non-negativity check fails (min `-0.4848` at `x=2`); monotonicity check fails; symmetry check passes (the `x.abs()` at the top of the function already guarantees that part) |

Both checks are variants of the same "logic bug / differential testing" approach from
`HEURISTICS.md` (no crash/UB is involved - this is a pure wrong-value bug), run together in one
`repro/` binary; a third tool wasn't needed once the first two produced a consistent, well
above the confirmation-tolerance signal.

## Status

**Confirmed** - deterministic, bit-for-bit repeatable: `bartlett_hann` is a pure function with no
randomness or threading, so every run against the same input produces byte-identical output. Ran
the standalone `repro/`'s full check suite twice against the pristine clone (base_commit
`83c4801428c436678d7c6b6bea74af800924a469`), byte-for-byte identical stdout both times (see
`output.txt`).

## Class

logic

## Root cause

`bartlett_hann(x)` in `src/math/bartlett.rs` is:

```rust
let x = x.abs();
if x > 2f32.as_() {
    return 0f32.as_();
}
let l = 2.0f32.as_();
let fac = (x / (l - 1.0f32.as_()) - 0.5f32.as_()).abs();
0.62f32.as_() - 0.4832.as_() * fac + 0.38f32.as_() * (2f32.as_() * V::const_pi() * fac).cos()
```

Two independent problems, both visible from the boundary values:

1. **Wrong domain normalization.** The textbook Bartlett-Hann window is defined on an index
   `t = n/(N-1)` in `[0, 1]`, but every call site (`compute_weights.rs`'s non-window branch)
   invokes the kernel with a distance-from-centre `x` over the kernel's declared support
   `[-radius, radius]` with `radius = 2.0` (`ResamplingFilter::new(bartlett_hann, 2f32)`). The
   code computes `fac = |x/(l-1) - 0.5|` with `l = 2.0`, i.e. `fac = |x - 0.5|` - this divides by
   `l - 1 = 1`, not by the actual radius `2.0`, and the `- 0.5` offset assumes `x` is already in
   `[0, 1]` rather than `[0, 2]`. The net effect: `fac(0) = 0.5` instead of `0` at the true centre,
   so the peak of the window lands at `x = 0.5` instead of `x = 0`, and `fac` reaches `1.5` at
   `x = 2` instead of the intended `0.5` at the edge.
2. **Wrong constant, `0.4832` instead of `0.48`**, on the linear term - a much smaller effect on
   the overall shape than the domain error above, but still a deviation from the standard
   coefficients (`0.62`, `0.48`, `0.38`) used by every other reference of this window (including
   the sibling `pic-scale` repo's already-fixed version, see below). The reported "flipped sign"
   turned out to already match the standard formula's minus sign on this term (`- 0.4832 * fac`);
   the actual second defect is the wrong magnitude, not a wrong sign.

Net effect in the real resize pipeline: because `compute_weights.rs` normalizes weights by their
sum before writing them out, the *specific* failure mode differs from `bartlett`'s (candidate 006)
in exactly where the large negative excursion sits - here it's concentrated at the support edge
(`x = 2`, weight `-0.4848`, i.e. about `-48%` of the peak) rather than spread across the whole
domain, and the peak weight is displaced to a non-zero offset (`x = 0.5`) instead of sitting at the
true centre. Both effects together mean the sample closest to a destination pixel gets an
under-weighted (even negative-neighbouring) contribution while a sample half a source-pixel away
gets the largest weight, which the confirmation candidate's suspicion of "large negative weights
near the support edge" correctly anticipated (see `output.txt`'s closing section for the exact
number).

This is not specific to `pic-scale-safe`: the sibling (non-`-safe`) `pic-scale` repo, checked out
locally at `/home/rafal/test/pic-scale`, independently fixed this exact same bug in its own
`bartlett_hann` (re-deriving the window over the normalized distance `d = |x|/radius`, with tests
asserting `bartlett_hann(0) == 1.0`, `bartlett_hann(2.0) == 0.0`, and a differential check against
the textbook window) - a working, already-fixed reference implementation of the same function
this candidate targets.

## Introduced by

5e72885 "Initial commit" (2024-10-16) - same origin commit as sibling candidate 006 (`bartlett_hann` was introduced in the same file, at the same time, with its own analogous domain-normalization defect).

**Already fixed upstream**: commit b26f840 "Fixes" (2026-08-25, merged into current HEAD via PR #22) rewrote `bartlett_hann()`'s `fac` computation to `x / (l + l)` (correctly normalizing by the radius) and fixed the `0.4832` constant to `0.48`, plus added a `bartlett_hann_peaks_at_center_and_decays_to_zero` regression test - the current working tree no longer reproduces this bug. See this candidate's own `CORRECTION.md`.

## The fix

Changed `bartlett_hann()` to normalize `x` by the kernel's actual radius before applying the
standard Bartlett-Hann coefficients, using the same `d = |x|/radius` re-expression the sibling
`pic-scale` repo's fix uses (`|t-0.5|` becomes `d/2` i.e. a `0.24*d` linear term, and
`cos(2*pi*t)` folds to `-cos(pi*d)` i.e. a `+0.38*cos(pi*d)` term):

```rust
pub(crate) fn bartlett_hann<
    V: Copy + Sub<Output = V> + Mul<Output = V> + Div<Output = V> + Float + ConstPI + 'static,
>(
    x: V,
) -> V
where
    f32: AsPrimitive<V>,
{
    let x = x.abs();
    let radius = 2f32.as_();
    if x > radius {
        return 0f32.as_();
    }
    let d = x / radius;
    0.62f32.as_() - 0.24f32.as_() * d + 0.38f32.as_() * (V::const_pi() * d).cos()
}
```

(Required adding `Div` to the function's trait bounds and to the module's `use std::ops::{..}`
import - `bartlett()`, the sibling function in the same file, is untouched.)

Verified against more than just the triggering input: the differential test in `repro/` sweeps
4001 points across the entire `[-2, 2]` domain (not just `x=0`), and separately checks
non-negativity, monotonicity, and symmetry - all pass after the fix except for a residual `~1.5e-8`
floating-point rounding gap (see `output.txt`'s "AFTER" section), well inside the `1e-6`
tolerance used for the check. `bartlett_hann(0)` is exactly `1.0` (the peak) and
`bartlett_hann(2.0)` is `~1.49e-8` (effectively `0.0`, the support edge) after the fix, versus
`-0.0016` and `-0.4848` before it.

- Project test suite: `cargo test` at the workspace root has 0 lib tests, 0 doc tests, and an
  empty `tests/` directory both before and after the fix (ran explicitly both ways: post-fix, then
  reverted via `git checkout -- src/math/bartlett.rs` and re-run) - **N/A, no tests in this
  package**.

## What's not done / open questions

- `bartlett` (`src/math/bartlett.rs:34-45`) is a separate, already-confirmed bug (candidate 006,
  different formula, different symptom) and was not re-touched by this fix.
- Every other `src/math/*.rs` kernel flagged in `IDEAS.md` (jinc, kaiser, welch's missing `abs()`,
  etc.) shares the "wrong shape" bug class but lives in different functions with different
  formulas; none of them were re-checked as part of confirming this one.
- Not checked whether the sibling `pic-scale` repo's `bartlett` (a separate, still-unfixed bug per
  candidate 006's notes) should also get a similar fix - that repo was only used as a read-only
  reference, per this skill's workspace being scoped to `pic-scale-safe`.
- No public-API (full resize) observation was captured for this specific candidate, unlike
  candidate 006's "single-pixel-spike" demonstration - the kernel-level differential test alone
  already exceeded the confirmation bar, so the extra step wasn't needed to reach `Confirmed`; a
  future session wanting a visual/end-to-end demonstration for `BartlettHann` specifically could
  still add one using the same pattern as candidate 006's `repro`.
