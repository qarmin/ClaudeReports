# GAUSSIAN_KERNEL_WRONG_SHAPE

`gaussian` in `src/math/gaussian.rs:43-48` is the kernel weight function wired up for
`ResamplingFunction::Gaussian` (`src/sampler.rs:273`, called from `src/compute_weights.rs` with a
raw, non-squared source-pixel distance `x = dx.abs()`, `x` in roughly `[0, 2]`). It computes the
denominator as `2*sigma*sigma` then squares it again (`den *= den`), and exponentiates raw `x`
instead of `x*x` - looked like a badly wrong Gaussian kernel shape, differential-testable against
the textbook normal density.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential test against the textbook Gaussian PDF (`normal_pdf`, independently transcribed from the standard formula, same `sigma = 0.35f32 as f64`) over `x` in `[0, 2]` (the kernel's actual support per `min_kernel_size = 2.0`) | `repro/`, release build, calls the real `pub(crate) fn gaussian` via `include!()` of the actual source file (see below) | Diverges immediately; max abs error `9.47e-1` at `x=0.159` (tolerance `1e-6`) |
| Boundary-value checks derived from the shape a Gaussian must have: zero slope at the centre (bell, not cusp), `w(sigma)/w(0) == exp(-1/2)`, `w(1)/w(0)` non-negligible | Same repro run | All three fail: slope at `x=0` is `-18.99` (should be ~0), `w(sigma)/w(0) = 0.00294` (should be `0.6065`), `w(1)/w(0) = 5.82e-8` (should be non-negligible - this collapses the filter to nearest-neighbour once weights are normalized in `compute_weights.rs`) |

Only one approach was needed - the differential test against the textbook formula immediately
gave a clear, deterministic, reproducible signal (this is a pure function of one float argument,
no adversarial-input/crash angle applies, so fuzzing/miri/ASAN would have had nothing to add here
per `HEURISTICS.md`'s guidance for logic bugs).

Independent corroboration: the sibling (non-`-safe`) `pic-scale` repo checked out locally at
`/home/rafal/test/pic-scale` has already fixed the identical bug in its own
`src/math/gaussian.rs` (`den = 2*sigma*sigma` without the extra `den *= den`, and
`exp(-(x*x)/den)` instead of `exp(-x/den)`), with its own test suite whose comments describe
this exact failure mode verbatim ("the pre-fix kernel used `exp(-x)` rather than `exp(-x*x)`,
which put a cusp there instead"; "the pre-fix kernel decayed so fast (`w(1)/w(0) = 5.8e-8`) that
after normalization the filter collapsed to nearest-neighbour" - matching this repro's own
`5.817987982875154e-8` almost to the last digit).

## Status

**Confirmed**. Bit-for-bit repeatable: 3/3 runs against the pristine source produced byte-for-byte
identical output (pure, deterministic float function, no seed/layout dependence), and 2/2 runs
against the patched source likewise. Also verified via the candidate's own standalone `justfile`
(`just test` step by step: fresh `git clone` of `https://github.com/awxkee/pic-scale-safe.git` at
`83c4801428c436678d7c6b6bea74af800924a469`, `reproduce-before` reproduces, `apply` + `reproduce-after`
no longer reproduces), not just against the live working tree.

## Class

logic

## Root cause

The intended formula is the standard Gaussian density `1/(sigma*sqrt(2*pi)) * exp(-x^2/(2*sigma^2))`.
The code instead:

1. Computes `den = 2*sigma*sigma` (correctly `2*sigma^2`), then does `den *= den`, squaring it
   again to `(2*sigma^2)^2 = 4*sigma^4` - one squaring too many.
2. Exponentiates the raw, unsquared `x` (`(-x / den).exp()`) instead of `x*x`.

Both callers of `gaussian` (`compute_weights.rs`) always pass `x = dx.abs() >= 0`, so this isn't a
sign/domain bug - `-x/den` is a genuine (monotonically decreasing) exponential decay, not a
parabola, so the two errors don't cancel out. The practical effect: since `den` ends up far larger
than it should (`4*sigma^4 ≈ 0.06` vs. the correct `2*sigma^2 ≈ 0.245`) *and* the exponent no
longer grows quadratically with distance, the resulting curve decays far too fast per unit
distance close to the centre (cusp, not a bell - confirmed by the near-zero slope check) and is
wrong at every other radius tested. At `x = 1` source pixel away, the weight is `~5.8e-8` relative
to the centre tap - after the per-destination-pixel weight normalization in
`compute_weights.rs:144-161`/`208-227`, this makes `ResamplingFunction::Gaussian` behave close to
nearest-neighbour resampling instead of a Gaussian blur, with no crash or panic to make it visible
otherwise (silent quality bug, not a memory-safety/crash issue).

## Introduced by

5e72885 "Initial commit" (2024-10-16) - `git log -S"fn gaussian"` finds this as the commit that introduced `gaussian()` with both defects (`den` squared twice, `exp(-x/den)` instead of `exp(-x*x/den)`) already present.

**Already fixed upstream**: commit b26f840 "Fixes" (2026-08-25, merged into current HEAD via PR #22) rewrote `gaussian()` to use `den = 2*sigma*sigma` (not squared again) and `exp(-(x*x)/den)`, and added a `gaussian_matches_textbook_formula` regression test - the current working tree no longer reproduces this bug. See this candidate's own `CORRECTION.md`.

## The fix

```
-    let mut den = 2f32.as_() * sigma * sigma;
-    den *= den;
-    (1f32.as_() / ((2f32.as_() * pi).sqrt() * sigma)) * (-x / den).exp()
+    let den = 2f32.as_() * sigma * sigma;
+    (1f32.as_() / ((2f32.as_() * pi).sqrt() * sigma)) * (-(x * x) / den).exp()
```

Removes the extra `den *= den` squaring and exponentiates `x*x` instead of `x`, matching the
textbook formula and the already-fixed sibling `pic-scale` repo's implementation exactly. Checked
against the whole tested domain (`x` in `[0, 2]` in `0.001` steps, plus the specific boundary
points `x = 0`, `x = sigma`, `x = 1`), not just the one value that first showed the divergence -
after the fix the max absolute error against the independent textbook reference over that whole
range is `2.22e-16` (float epsilon), `w(sigma)/w(0)` matches `exp(-1/2)` exactly, and the centre
slope is `~4.65e-6` (zero within numerical noise).

- Project test suite (`cargo test -p pic-scale-safe`): 0 passed / 0 failed / 0 ignored before, and
  0 passed / 0 failed / 0 ignored after - the library crate has no `#[test]`s at all (known,
  documented project fact - see `___BUGHUNT___/README.md` / the skill's own
  `projects/pic-scale-safe.md`), not a check that was skipped.
- `cargo clippy -p pic-scale-safe --lib` after the fix: clean, no new warnings (the removed
  `mut den` and now-unused `MulAssign` trait bound on `gaussian`'s generic parameter list did not
  trigger any lint; the bound was left in place since removing it is an unrelated, non-bug-fixing
  cleanup).

## What's not done / open questions

- This session only checked `gaussian`, the one candidate named in `IDEAS.md`. The same backlog
  entry (and this session's own discovery) flags several other hand-derived kernel functions in
  `src/math/*.rs` (`bartlett`, `bartlett_hann`, `jinc`, `kaiser`, `welch`) as suspected-wrong by
  inspection but not yet differentially tested - each is its own candidate, not covered by this
  fix.
- Not verified end-to-end through the actual public resize pipeline (e.g. `resize_plane_f32` with
  `ResamplingFunction::Gaussian` on a real image) that the visual/numeric output changes as
  expected with the fix - the fix was verified at the kernel-function level only, which is where
  the bug actually lives and is fully sufficient to characterize it, but a maintainer wanting a
  visual before/after diff of an actual resized image would need to do that separately.
- No fuzzing was run (and none was warranted) - this is a pure, low-dimensional (`f32`/`f64` in,
  `f32`/`f64` out) deterministic function with a known-correct reference formula; differential
  testing against that reference is strictly more informative here than fuzzing would be.
