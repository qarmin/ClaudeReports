# `pic-scale-safe`'s `Gaussian` kernel function is missing the `x^2` in its exponent, so it computes an exponential/Laplace-shaped decay instead of an actual Gaussian

`DynamicImage::resize`/`resize_exact` (`src/imageops/resize.rs`) delegate all real resampling work
to the external `pic-scale-safe` crate. This candidate follows up on `IDEAS.md`'s entry noting
that candidate 017's linear-ramp reproducing-property sweep found `Gaussian` failing that
invariant both before and after 017's own (unrelated) window-misalignment fix, with the failures
even getting numerically larger post-fix on the same small-`src_w` cases - flagged as needing a
differential check to tell a genuine distinct bug from an inherent design characteristic of a
small, fixed-support Gaussian kernel at extreme upscale ratios.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential test against `image::imageops::resize`'s own independent `FilterType::Gaussian` implementation, real public API on both sides, `Luma8` ramps, `(src_w, dst_w)` in `{(5,40),(5,100),(5,400),(5,750),(10,80),(10,400),(10,1500)}` | release build, scratch binary | Real disagreement (5-11 gray levels out of 255), but confounded: `image`'s own oracle uses a different fixed sigma (`0.5`) and window (`radius 3`) than `pic-scale-safe`'s (`sigma 0.35`, `radius 2`) by design, so a modest disagreement between two intentionally-different Gaussian parameterizations isn't proof of a bug on its own - not conclusive by itself, escalated. |
| Code reading, `pic-scale-safe`'s `src/math/gaussian.rs` (the per-tap kernel function itself, as opposed to `compute_weights.rs`'s window-bounds code candidate 017 already fixed) | | Found the kernel function computes `(1/(sqrt(2*pi)*sigma)) * exp(-x/den)` where `den = (2*sigma^2)^2` - compare to the standard/correct Gaussian PDF `exp(-x^2/(2*sigma^2))` used by this same repo's own legacy `imageops::resize` oracle (`src/imageops/sample.rs:170`: `(-x.powi(2) / (2.0 * r.powi(2))).exp()`). `pic-scale-safe`'s version never squares `x`, and separately squares the `2*sigma^2` denominator a second time. The two mistakes don't cancel (an exponential/Laplace-shaped decay with numerically-much-narrower effective bandwidth than a real Gaussian with the same nominal sigma). |
| Self-consistent linear-ramp reproducing-property invariant (candidate 017's own technique - no external oracle, any symmetric normalized kernel must reproduce a linear ramp away from a clipped boundary window) through the real `image::DynamicImage::resize_exact` public API (`FilterType::Gaussian`), `Luma32F`, same 6 `(src_w, dst_w)` cases, before/after squaring-only fix to `pic-scale-safe`'s `gaussian()`, `pic-scale-safe`'s separate window-misalignment bug (candidate 017) deliberately left unpatched to isolate this bug's own contribution | release build, real end-to-end public API, `repro/` | Confirmed: max deviation from the linear-ramp invariant drops by 2.4x-2.7x after fixing only the squaring bug (`0.08167→0.03397`, `0.08993→0.03483`, `0.09142→0.03483`, `0.03630→0.01510`, `0.04050→0.01545`, `0.04063→0.01548`), consistent across every case. Deterministic, byte-identical across repeated runs. |

Three approaches were used (soft cap reached with a clear, consistent signal on the third): the
first (differential vs. a *differently-parameterized* Gaussian oracle) gave a real but ambiguous
signal, so escalated to reading the actual kernel math, which found a concrete coding mistake, then
verified with a self-consistent (unconfounded) invariant test isolating just that one fix's effect.

## Status

**Confirmed.** Deterministic, no layout/seed/timing dependence: the candidate's own `just test`
`reproduce-before`/`reproduce-after` cycle was run 2/2 times independently (once via a scratch
`/tmp` copy of this folder, once for the `output.txt` capture) from a fresh `git clone`, producing
byte-identical diagnostics both times (see `output.txt`).

**Fully reproducible: Yes.** `repro/`'s `main.rs` calls only `image::DynamicImage::resize_exact`
(the real subject, the crate's actual public resize API, which is what any real caller uses) with
`FilterType::Gaussian` on a real `ImageBuffer` built through the crate's own public constructor,
and checks a self-consistent mathematical invariant (no oracle/second participant needed) - a pure,
single-process computational logic bug in one kernel function's math, not a reaction to another
participant's data, the same reasoning as candidates 003/017/018 (sibling logic bugs in this same
resize code).

## Class

logic

## Root cause

`pic-scale-safe`'s `src/math/gaussian.rs`:

```rust
pub(crate) fn gaussian<...>(x: V) -> V {
    let sigma: V = 0.35f32.as_();
    let pi = V::const_pi();
    let mut den = 2f32.as_() * sigma * sigma;
    den *= den;                                          // <- (2*sigma^2)^2, not 2*sigma^2
    (1f32.as_() / ((2f32.as_() * pi).sqrt() * sigma)) * (-x / den).exp()   // <- -x, not -x^2
}
```

The normalizing constant `1/(sqrt(2*pi)*sigma)` in front is the correct Gaussian PDF constant, and
`compute_weights.rs::generate_weights` always calls this with `x = dx.abs()` (so the result is
still an even, symmetric function of the true offset) - but the exponent itself is `-x/den`
instead of the standard Gaussian `-x^2/(2*sigma^2)`, and `den` is separately computed as
`(2*sigma^2)^2` instead of `2*sigma^2`. With `sigma = 0.35`, the correct denominator would be
`2*0.35^2 = 0.245`; the buggy code instead uses `den = 0.245^2 = 0.060025` and never squares `x`.
The result is not a Gaussian at all - it's closer to a Laplace/exponential decay, and decays far
more steeply than a real `sigma = 0.35` Gaussian would (e.g. at `x = 1`: correct Gaussian gives
`exp(-1/0.49) ≈ 0.134`× the peak; the buggy version gives `exp(-1/0.060025) ≈ 8×10⁻⁸`× the peak -
six orders of magnitude tighter). Because `compute_weights.rs`'s window width (`kernel_size`,
`filter_radius`) is sized for `min_kernel_size = 2f32` independent of this per-tap shape, the
buggy kernel's effective weight is concentrated almost entirely on the single nearest source pixel
regardless of the window's nominal width - behaving close to nearest-neighbor rather than a
smooth Gaussian blend, which is what produces the large linear-ramp-reproduction failures noted in
`IDEAS.md`/candidate 017 (and explains why they didn't improve, or got numerically larger, after
017's *window*-alignment fix alone: the per-tap weight shape itself, not just which pixels are in
the window, was wrong).

This is independent of candidate 017's window-misalignment bug in `compute_weights.rs` - both bugs
are real and both currently affect `Gaussian` output; this candidate fixes only the kernel-math one
(the window one already has a documented, verified fix in candidate 017's own folder).

Only `ResamplingFunction::Gaussian` uses `src/math/gaussian.rs::gaussian` (confirmed via
`sampler.rs`'s `get_resampling_filter` match arm, `ResamplingFunction::Gaussian => ResamplingFilter::new(gaussian, 2f32)`)
- no other filter in this crate's public `FilterType` enum shares this specific bug, unlike
candidate 017's window bug which affects every non-`Area` filter.

Also worth correcting for the record: candidate 017's `RESULT.md` "What's not done" section
speculates Gaussian has a "fixed, scale-independent support width... doesn't widen its kernel for
upscaling... per `is_resizable_kernel`". Reading `sampler.rs` this session shows that's not
accurate for this dependency version - `ResamplingFunction::Gaussian => ResamplingFilter::new(gaussian, 2f32)`
uses `ResamplingFilter::new`, which sets `is_resizable_kernel: true` (the same as `CatmullRom`,
`Triangle`/`Bilinear`, etc.) - `generate_weights` does widen Gaussian's kernel by `filter_scale_cutoff`
identically to those other filters. Not filed as its own `CORRECTION.md` (017's own bottom-line
conclusion - "a real bug, not fully diagnosed this session" - still holds, just for a different
reason than the one it guessed at), but noted here since a future session reading 017's write-up
should rely on this candidate's version of that specific claim instead.

## Introduced by

Not determined - bug resides in external crate `pic-scale-safe`'s own source (`src/math/gaussian.rs`), not tracked in this repo's git history.

## The fix

Square `x` in the exponent and stop double-squaring `den`:

```diff
-    let mut den = 2f32.as_() * sigma * sigma;
-    den *= den;
-    (1f32.as_() / ((2f32.as_() * pi).sqrt() * sigma)) * (-x / den).exp()
+    let den = 2f32.as_() * sigma * sigma;
+    (1f32.as_() / ((2f32.as_() * pi).sqrt() * sigma)) * (-(x * x) / den).exp()
```

This patch is against `pic-scale-safe`'s own repository (`awxkee/pic-scale-safe`, `dep_base_commit`
`83c4801428c436678d7c6b6bea74af800924a469`, == published tag `0.1.11`, the same version candidate
017 targeted), not against this repo - the bug is entirely inside the dependency.

Verified against more than the one originally-triggering input, per `CONFIRMATION.md` section 7:
all 6 cases in `repro/` show a consistent 2.4x-2.7x reduction in max deviation from the
linear-ramp-reproduction invariant after the fix (see `output.txt`) - the residual (`~0.015-0.035`,
vs. `~0.009-0.023` for `CatmullRom` at the same sizes under the same still-unpatched window bug,
measured separately during triage) is consistent with candidate 017's own separate, deliberately
left-unpatched window-misalignment bug plus this being an inherently narrower/steeper kernel than
`CatmullRom` even when correctly computed (a real Gaussian with `sigma = 0.35` still decays faster
near a clipped boundary than a cubic spline does) - not evidence this candidate's specific fix is
incomplete for the bug it targets.

- Project test suite (`cargo test --lib --release -p image`, run in the main project's own working
  tree, dependency resolution swapped to the local patched `pic-scale-safe` clone via
  `[patch.crates-io]` + `cargo update -p pic-scale-safe`): **331 passed, 0 failed** before the
  patch → **331 passed, 0 failed** after the patch (identical pass count, no regressions, no new
  failures) - consistent with candidate 017/018's own finding that no existing test in this repo's
  suite exercises resize output closely enough to catch this class of bug either way.

## What's not done / open questions

- This is an upstream fix, not something that can ship in this repo alone - it needs to land in
  `pic-scale-safe` itself (and a subsequent version bump here) to actually reach users of
  `DynamicImage::resize`/`resize_exact`. No PR was opened against `awxkee/pic-scale-safe` this
  session.
- The residual deviation after this fix (still ~1.5x `CatmullRom`'s baseline at the same sizes) was
  not further decomposed between "candidate 017's still-unpatched window bug" and "Gaussian's
  inherently narrower support being more boundary-sensitive than CatmullRom even when correct" -
  applying *both* fixes together (this candidate's + candidate 017's) and re-running the same
  invariant sweep would isolate that cleanly, not done this session to keep this candidate's own
  measurement uncontaminated by a fix candidate 017 already owns and verified separately.
- Not checked whether any *other* `ResamplingFunction` kernel in `pic-scale-safe`'s `src/math/`
  (there are ~30) has a similar hand-transcription mistake in its own closed-form formula - this
  session only read `gaussian.rs`, prompted specifically by the `IDEAS.md` entry about `Gaussian`.
  Worth a systematic read of the whole `src/math/` directory against each function's textbook
  definition if a future session wants full coverage of this specific mistake class.
- Not tested for `Luma8`/`Rgb8`/etc. (fixed-point path) specifically - only `Luma32F` (floating-point
  path). `sampler.rs`'s `gaussian` is generic over the float type and shared by both
  `resize_fixed_point.rs` and `resize_floating_point.rs` (same reasoning candidate 018 used for the
  window bug), so no code-level reason to expect a fixed-point-specific difference, but not
  independently re-measured this session.
