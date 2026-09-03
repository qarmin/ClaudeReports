# `pic-scale-safe`'s `Robidoux` filter kernel uses `C = 13/(58+216*sqrt(2))` instead of the correct `C = 113/(58+216*sqrt(2))` - a missing digit in the transcribed constant

Follow-up to candidate 019's own open backlog item: candidate 019 found a hand-transcription
mistake in `pic-scale-safe`'s `src/math/gaussian.rs` and explicitly left the crate's other ~30
`src/math/` kernel functions unchecked against their own textbook closed-form definitions. This
candidate reads all of them and finds one more, real, distinct instance of the same mistake class
in `src/math/bc_spline.rs::robidoux()`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual line-by-line read of every function in `pic-scale-safe` 0.1.12's `src/math/` (`bartlett.rs`, `bc_spline.rs`, `bessel_order_one.rs`, `bilinear.rs`, `blackman.rs`, `bohman.rs`, `cubic.rs`, `hann.rs`, `kaiser.rs`, `lagrange.rs`, `lanczos.rs`, `quadric.rs`, `sinc.rs`, `sphinx.rs`, `spline_n.rs`, `welch.rs` - `gaussian.rs` already covered by candidate 019) against their own known closed-form/reference definitions | code reading only | Found `robidoux()`'s second BC-spline parameter (`C`) uses the literal `13f32.as_() / (58f32.as_() + 216f32.as_() * V::const_sqrt2())`. Also noted (see "What's not done") a structurally-similar-looking but *not exploitable* pattern: `welch`/`kaiser`/`lagrange` each only guard one side of their symmetric domain (e.g. `welch` never takes `x.abs()` and returns negative values for `x <= -1`) - flagged but not filed as a bug, see below for why. |
| Independent closed-form cross-check: (a) GStreamer's own `gst-plugins-base/gst-libs/gst/video/video-resampler.h` (a completely independent, non-`awxkee` codebase) documents the Robidoux filter's published coefficients as `B=0.3782, C=0.3109` in a comment table; (b) the sibling crate by the same author, `pic-scale` (not `-safe`), computes the identical `B` via `12f32.as_() / mla(9f32.as_(), V::const_sqrt2(), 19f32.as_())` but computes `C` via `113f32.as_() / mla(216f32.as_(), V::const_sqrt2(), 58f32.as_())` - i.e. with the leading `1` that `pic-scale-safe` is missing | `python3 -c` arithmetic check | `13/(58+216*sqrt2) = 0.03577`, nowhere near the documented `C≈0.3109`; `113/(58+216*sqrt2) = 0.31089212245300063`, matching the GStreamer-documented value and the sibling `pic-scale` crate's own formula to 8 significant digits. Conclusively confirms `13` is a typo for `113` (a single missing leading digit), not an intentional different constant. |
| Real end-to-end differential repro: `pic-scale-safe`'s own public `resize_plane_f32(..., ResamplingFunction::Robidoux)` API vs. an independent oracle (a from-scratch reimplementation of `compute_weights.rs`'s documented, publicly-readable weight-generation algorithm, using the textbook-correct `B`/`C`), release build, `repro/`, real crate calls (no faked internals) | 6-source-pixel row upscaled 6->17, 2 independent fresh-clone `just test` runs | The real crate's `Robidoux` output matches the buggy-`C` oracle bit-for-bit (max diff `0.0`, float rounding only) and diverges from the correct-`C` oracle by up to `0.0472` (out of a `[0,1]` range - about 12/255 in 8-bit terms) - both directions confirmed, deterministic, 2/2 identical runs from a fresh clone each time (see `output.txt`). |

Three approaches (code read -> independent numeric/cross-crate cross-check -> real end-to-end
differential repro), each escalating and each confirming the same conclusion - no dead ends hit,
so the soft cap wasn't needed.

## Status

**Confirmed.** Deterministic: `repro/`'s `just test` (`reproduce-before` against the pristine
`0.1.12` clone, `reproduce-after` against the same clone with `patch.diff` applied) produces
byte-identical diagnostics across 2/2 independent fresh-clone runs (see `output.txt`).

**Fully reproducible: Yes.** `repro/src/main.rs` calls only `pic_scale_safe::resize_plane_f32`
(the crate's own real, public resize API - the same one any downstream crate would call directly,
since this specific filter, `ResamplingFunction::Robidoux`, is not reachable through the `image`
crate's own public `FilterType` enum at all - see "Root cause" below) with a real source buffer.
This is a pure, single-process math/logic bug in one kernel function's constant, not a reaction to
another participant's input, the same reasoning already accepted for sibling candidates
003/017/018/019/023 in this same dependency.

## Class

logic

## Root cause

`pic-scale-safe`'s `src/math/bc_spline.rs::robidoux()`:

```rust
pub(crate) fn robidoux<...>(x: V) -> V {
    bc_spline(
        x,
        12f32.as_() / (19f32.as_() + 9f32.as_() * V::const_sqrt2()),   // B, correct
        13f32.as_() / (58f32.as_() + 216f32.as_() * V::const_sqrt2()), // C, should be 113f32
    )
}
```

The Robidoux filter is one specific `(B, C)` point in the general Mitchell-Netravali BC-spline
family (`bc_spline(x, B, C)`, itself verified correct against the textbook formula this session).
The published Robidoux constants are `B = 12/(19+9*sqrt(2)) ≈ 0.378216` (correctly transcribed
here) and `C = 113/(58+216*sqrt(2)) ≈ 0.310892` - but the code has `13/(58+216*sqrt(2)) ≈ 0.035766`
instead, a missing leading `1` digit. This produces a substantially different cubic kernel shape
(not just a uniform scale factor that `compute_weights.rs`'s per-output-pixel weight
renormalization would wash out - `compute_weights.rs::generate_weights` always normalizes weights
to sum to 1 before use, exactly the same mechanism that made the `hann`/`hanning` kernels' own odd
constant-scale-factor quirks harmless, but a change to `C` changes the *relative* shape of the
kernel between taps, which normalization cannot undo). Because `compute_weights.rs`'s call site
always evaluates the kernel at `dx.abs()` (non-negative), this is purely a shape-of-response bug,
not a sign/domain bug.

`ResamplingFunction::Robidoux => ResamplingFilter::new(robidoux, 2f32)` in `sampler.rs` is the only
call site. `Robidoux` is not one of the 5 filters exposed by this repo's own public
`FilterType` -> `ResamplingFunction` mapping (`src/imageops/resize.rs::convert_filter_type`:
`Nearest`, `Triangle`->`Bilinear`, `CatmullRom`, `Gaussian`, `Lanczos3` only), so this bug is not
reachable through `image`'s own public API - only through a downstream crate that depends on
`pic-scale-safe` directly and explicitly requests `ResamplingFunction::Robidoux` (or
`RobidouxSharp`, which is a separate, independently-verified-correct function, and unaffected).

## Introduced by

Not determined - bug resides in external crate pic-scale-safe's own source (`src/math/bc_spline.rs`), which is a crates.io dependency not vendored or tracked in this repo's git history.

## The fix

Fix the transcribed constant (one digit):

```diff
-        13f32.as_() / (58f32.as_() + 216f32.as_() * V::const_sqrt2()),
+        113f32.as_() / (58f32.as_() + 216f32.as_() * V::const_sqrt2()),
```

This patch is against `pic-scale-safe`'s own repository (`awxkee/pic-scale-safe`, tag `0.1.12`,
commit `b50e020d72a1c0063f39c2c0166343d4e44790ca` - matches this repo's `Cargo.lock`-resolved
version), not against this repo (`image-rs/image`) - the bug is entirely inside the dependency, and
(per "Root cause" above) not reachable through this repo's own public API at all, so this repo's
own behavior is unaffected either way.

Verified against more than the one originally-triggering scenario (per `CONFIRMATION.md` section
7): the independent oracle (built from `compute_weights.rs`'s own documented algorithm, not a copy
of the buggy code) matches the real crate's `Robidoux` output bit-for-bit both before the fix
(against the buggy-`C` oracle) and after the fix (against the correct-`C` oracle), across all 17
output positions in a single run, and identically across 2 independent fresh-clone runs.

- Project test suite (`cargo test --lib --release`, run in this repo's own working tree, with
  `pic-scale-safe` swapped to the local patched clone via a temporary `[patch.crates-io]` entry,
  reverted immediately after): **331 passed, 0 failed** before the patch -> **331 passed, 0
  failed** after the patch (identical pass count, no regressions) - expected, since `Robidoux` is
  not reachable through this repo's own code or test suite at all (see "Root cause"), so this
  check is a pure "the patch doesn't break anything else in this repo" sanity check, not a
  detection of the bug itself.

## What's not done / open questions

- This is an upstream fix, not something that can ship in this repo alone - it needs to land in
  `pic-scale-safe` itself to reach any downstream crate that uses `ResamplingFunction::Robidoux`
  directly. No PR was opened against `awxkee/pic-scale-safe` this session (matches candidates
  017/019/023's same open-upstream-fix pattern).
- `pic-scale-safe`'s sibling crate `pic-scale` (same author, not `-safe`) has the *correct* `C`
  value in its own `src/math/bc_spline.rs::robidoux()` (`113f32.as_() / mla(...)`) - this typo
  looks specific to `pic-scale-safe`'s own port/rewrite of that function, not present in the
  sibling crate. Not checked whether any *other* function differs between the two crates in a way
  that would point at more transcription mistakes specific to the `-safe` fork.
- Every other kernel function in `src/math/` was read and checked against its own known reference
  formula this session (`bartlett`/`bartlett_hann`, `bessel_order_one`/`jinc`, `bilinear`,
  `blackman`, `bohman`, `cubic_spline`/`bicubic_spline`, `bc_spline`/`hermite_spline`/`b_spline`/
  `mitchell_netravalli`/`catmull_rom`/`robidoux_sharp`, `hann`/`hamming`/`hanning`, `kaiser`,
  `lagrange`/`lagrange2`/`lagrange3`, `lanczos_jinc`/`lanczos_sinc` families, `quadric`, `sinc`,
  `sphinx`, `spline16`/`spline36`/`spline64`, `welch`) and no other formula-shape mismatch was
  found - `robidoux()`'s `C` constant is the only one. `robidoux_sharp()`'s own two constants were
  independently cross-checked the same way (against GStreamer's documented `B=0.2620, C=0.3690`)
  and are correct.
- `welch()`, `kaiser()`, and `lagrange()` each only clamp/guard one side of their nominally
  symmetric domain (e.g. `welch(x)` returns `1 - x*x` unclamped, including negative values, for any
  `x <= -1`, instead of `0`) - structurally the same "looks like a mistake" shape as candidate
  019's bug, but *not* filed as its own candidate: `compute_weights.rs::generate_weights` (the only
  call site for every kernel in this file, read this session) always evaluates every kernel at
  `dx.abs()` (a non-negative value) before calling it, both in the direct-kernel branch and the
  windowed-kernel (`Ginseng`/`HaasnSoft`) branch, so this asymmetry can never actually be reached
  with a negative argument through any current call path in this crate. Noted in `IDEAS.md` in
  case a future `pic-scale-safe` change ever calls one of these kernels with a signed offset
  directly.
- `hann()`/`hanning()`/`blackman_window()` each apply an extra constant scale factor not present in
  the standard normalized window formula (e.g. `hann`'s own `size_scale = 1/4` factor makes its
  peak value `0.25` instead of `1.0`) - also *not* filed as a bug, since
  `compute_weights.rs::generate_weights` always renormalizes each output pixel's weights to sum to
  1 before use, and a uniform (x-independent) scale factor is exactly cancelled by that
  renormalization, unlike `robidoux`'s `C` mistake which changes the kernel's *relative* shape
  between taps.
