# VARYING_ALPHA_FIXED_POINT_SATURATION_DESYNC

`src/imageops/resize.rs`'s premultiply-alpha-aware resize path (`DynamicImage::resize`/
`resize_exact`, non-`Nearest` filters). Neither candidate 010 nor candidate 016 tested a source
with alpha genuinely varying pixel-to-pixel (both used constant-alpha sources, which take the
`has_constant_alpha => true` shortcut and never exercise `premultiply_alpha`/`unpremultiply_alpha`)
- flagged in `IDEAS.md` as worth a more careful check than plain equality, since legitimate
blending can shift intermediate values even in a correct implementation.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Empirical differential probe: spatially-constant-RGB, varying-alpha `RgbaImage` (u8) through `DynamicImage::resize_exact`, all 4 non-Nearest filters, 5 geometries each | Release build, `cargo run --example` scratch probe | Deviations up to 15/255 (~6%) near low-alpha (~alpha 8-27) output pixels - plausible as ordinary 8-bit premultiply/unpremultiply quantization noise on its own |
| Same probe on `Rgba32F` (f32) as an independent reference/oracle, same geometries/filters | Release build | Max deviation ~1e-7 (machine epsilon) in every case, **even at very low alpha (down to 0.05)** - proves the "recover exact source color" invariant genuinely holds for a correct implementation, ruling out "it's just expected ringing/blending drift" |
| Source read of `pic-scale-safe` 0.1.12's `premultiply_rgba8`/`unpremultiply_rgba8` (`alpha.rs`) | Read-only | Both use correctly-rounded, table-based/bit-trick divide-by-255 - confirms the u8 low-alpha noise above is inherent 8-bit quantization, not a rounding-direction bug |
| Same probe on `Rgba16` (u16), same geometries/filters, plus a per-pixel dump of one extreme case (2x2 -> 50x50, `Lanczos3`) | Release build | At **fully-opaque** output pixels (alpha == 65535, no ambiguity about blending) near the source's alpha step edge, color deviated up to 14242/65535 (**21.7%**) from the true constant source color - far beyond quantization noise, and not explainable by "legitimate blending" since those pixels are reported as 100% opaque |
| Source read of `pic-scale-safe`'s fixed-point convolution (`fixed_point_horizontal.rs`/`fixed_point_vertical.rs`) vs. its floating-point convolution (`resize_floating_point.rs`/`floating_point_horizontal.rs`) | Read-only | Fixed-point path calls `sums.saturate_narrow(bit_depth)` (independent per-channel saturating clamp) after **every pass** (horizontal, then vertical); floating-point path has no clamp/saturate call anywhere - confirms the mechanism (see Root cause) |
| Attempted fix: route the 4 alpha-bearing fixed-point pixel types (`LumaA8`/`Rgba8`/`LumaA16`/`Rgba16`) through pic-scale-safe's f32 resize/premultiply/unpremultiply functions instead, when premultiply is needed | `src/imageops/resize.rs`, +70/-5 lines | Eliminated the deviation in the synthetic repro entirely (max_dev 0 across all filters/geometries/bit-depths tried) **but** regressed the project's own pre-existing `tests/regression.rs::resizing_with_catmul` reference-image test from 13/8580 differing channels (max off-by-1) to 2081/8580 (max off-by-255) on a real photographic image - rejected, not shipped as `patch.diff` (see "The fix" below) |

## Status

**Confirmed**. Deterministic and bit-exact reproducible: 3/3 identical runs (`repro/`, release
build) against a pristine clone of `image-rs/image` at the base commit, and identically
reproducible live in the main checkout throughout the session.

**Fully reproducible**: **Yes**. `repro/` calls only the crate's real public API
(`DynamicImage::resize_exact`) exactly as any caller would - this is a pure, deterministic,
single-process logic bug (wrong output value for a given input), not a finding about one
participant reacting to another's input, so `CONFIRMATION.md` section 11's second-participant bar
does not apply.

## Class

logic

## Root cause

`resize_impl` (`src/imageops/resize.rs`) implements alpha-aware resizing by (1) premultiplying
color channels by alpha in place, in the image's native bit depth, (2) calling one of
`pic-scale-safe`'s generic N-channel resize functions - which treats the now-premultiplied color
channels and the alpha channel as four unrelated data channels, with no special-casing of their
relationship - and (3) unpremultiplying the result.

For 8-bit/16-bit ("fixed-point") pixel types, `pic-scale-safe`'s convolution
(`fixed_point_horizontal.rs::convolve_row_handler_fixed_point[_4]`,
`fixed_point_vertical.rs`) computes each output channel's weighted sum in a wider integer
accumulator and then calls `.saturate_narrow(bit_depth)` - an **independent, per-channel
saturating clamp back to the representable range** - after the horizontal pass, and again after
the vertical pass.

Filters with negative lobes (`CatmullRom`, `Gaussian`, `Lanczos3` - anything but `Nearest`/
`Triangle`) can produce a weighted sum that overshoots the representable range (classic filter
ringing near a sharp source transition - here, the alpha channel's own steep gradient, since the
premultiplied-color channels only vary because alpha does). When that happens, the **alpha
channel's** overshoot gets silently clamped to its ceiling (e.g. 65535), while the **premultiplied
color channel's** overshoot - proportionally similar, since it's the same weights applied to
`color = C * alpha`, but numerically smaller since `C < 1.0` - stays below its own ceiling and
never triggers a clamp. Because the two channels get saturated independently and inconsistently,
the ratio `color / alpha` (which is exactly the invariant `unpremultiply_alpha` relies on to
recover the true color `C`) is corrupted. The corruption compounds across two passes (horizontal
then vertical) for a 2D resize.

The floating-point resize path (`resize_floating_point.rs`/`floating_point_horizontal.rs`/
`floating_point_vertical.rs`) has no equivalent clamp anywhere in the convolution - it stores raw
`f32` sums between passes and only the final `unpremultiply_rgba_f32` divides by alpha - so the
ratio is preserved exactly (mod float rounding) regardless of how far the weighted sum overshoots
`[0, 1]`. This is why the f32 oracle in this repro stays at machine epsilon on the exact same
geometry/filter that produces a 21.7% error in the u16 path.

The buggy per-pass saturating clamp itself lives in the external `pic-scale-safe` dependency
(same as candidates 017/018/019's root causes), but this repo's own `resize.rs` is what chooses to
feed a premultiplied-alpha buffer into that generic, alpha-unaware, per-channel-clamping resize
function and rely on a consistency it doesn't actually preserve for negative-lobe filters.

## Introduced by

d0a92afc "Import the resize adapter from pic-scale-safe to DynamicImage from wondermagick; not wired up to anything yet" (2025-11-09)

## The fix

An attempted fix was implemented, built, and empirically verified to eliminate this repro's
deviation - **but was ultimately rejected and is not included as `patch.diff`**, because a wider
differential check found it substantially regresses accuracy on real-world (non-synthetic) images.

**What was tried**: in `resize_impl`, when premultiplication is needed (`algorithm != Nearest &&
!has_constant_alpha(image)`) and the pixel type is one of the four alpha-bearing fixed-point types
(`LumaA8`, `Rgba8`, `LumaA16`, `Rgba16`), convert to the crate's own `f32` representation first
(`to_luma_alpha32f()`/`to_rgba32f()`), premultiply/resize/unpremultiply entirely in `f32` via
`pic-scale-safe`'s existing floating-point functions, then convert back to the original type -
instead of premultiplying and resizing in the native fixed-point representation. This avoids the
buggy per-pass saturating clamp entirely for the affected combination.

**Verification of the attempt**:
- Re-ran this candidate's `repro/` (and the broader ad hoc probe covering all 4 non-Nearest
  filters, both `u8` and `u16`, 5 geometries each) against the patched tree: max deviation dropped
  to exactly 0 in every case.
- Ran the project's full test suite: **pass/fail counts unchanged**, before and after
  (`cargo test --release`: unit `331 passed / 0 failed`; `tests/regression.rs`
  `8 passed / 1 failed` both before and after - the 1 failure is `resizing_with_catmul`,
  already failing on a clean, unpatched checkout per candidate 021, not introduced by this
  change).
- However, a magnitude-level differential check on that same already-failing
  `resizing_with_catmul` test (which compares a real photographic PNG resized with `CatmullRom`
  against a committed reference image) showed the fix makes the **existing** failure dramatically
  worse: `13/8580` differing RGBA8 channels (max off-by-1) on unpatched `main`, versus
  `2081/8580` differing channels (**max off-by-255** - full black/white saturation) with the fix
  applied. Root cause of the regression: the `f32` path's `unpremultiply_rgba_f32` does an
  unclamped `1.0 / alpha` reciprocal multiply for any nonzero alpha, which can blow up
  unboundedly for the very-small-but-nonzero alpha values common at soft/anti-aliased edges of
  real images with actual color content behind them (as opposed to this repro's synthetic
  spatially-constant-color source, where the blow-up cancels exactly because the ratio truly is
  constant) - the resulting huge intermediate values then saturate to solid black or white on the
  final `f32 -> u8` conversion. The original fixed-point path's `saturate_narrow` bounds
  intermediate values at every stage, which is exactly what causes this candidate's bug, but also
  incidentally bounds this other failure mode.

In short: the two `pic-scale-safe` code paths make different, non-overlapping trade-offs (bounded-
but-ratio-corrupting vs. unbounded-but-ratio-preserving), and naively picking the second one for
this repo's specific alpha-bearing integer types trades a subtle, rare-in-practice color-fringing
bug (only visible with strong alpha gradients + a negative-lobe filter) for a much coarser, more
visible regression on ordinary photographic content with soft/anti-aliased alpha edges. No safe
bounded fix confined to `resize.rs` was found this session; a real fix likely needs either (a) an
upstream `pic-scale-safe` change so the fixed-point convolution's saturating clamp is
alpha-aware (e.g. clamping the color channels to `<=` the *unclamped* alpha value before either is
narrowed, rather than clamping each channel to its own independent ceiling), or (b) a hybrid
approach in `resize.rs` that clamps the `f32` premultiplied source's alpha away from zero before
resizing (a "for scaling alpha must be *associated*" caveat `pic-scale-safe`'s own docs already
flag) combined with (a). Both are architecture-level decisions beyond a bounded single-session
patch.

- Project test suite (main checkout, before *and* after the rejected attempted fix - included for
  the record, since the attempt was built/tested/reverted during this session):
  `cargo test --release`: unit tests `331 passed / 0 failed` (unchanged); `tests/regression.rs`
  `8 passed / 1 failed` (unchanged - `resizing_with_catmul` already failing pre-existing, per
  candidate 021). No new test failures either way. The working tree has been reverted to a clean
  checkout (`git status` shows no changes beyond this workspace's own new files) - the attempted
  fix was **not** kept.

## What's not done / open questions

- The exact minimum alpha-gradient steepness / filter-support-width combination that triggers a
  user-visible (not just synthetic-test-visible) instance of this bug on realistic content wasn't
  characterized - the repro deliberately uses an extreme 2-pixel-wide alpha step upscaled 25x to
  make the effect unambiguous and large; real images with gentler alpha gradients would show a
  smaller (but still real) effect.
- `Luma32F`/`LumaA32F`/`Rgb32F`/`Rgba32F` are not affected (they already use the unclamped f32
  path) and were not exhaustively re-verified beyond the oracle role they already played here.
- Whether `resize_to_fill`/`thumbnail`/`thumbnail_exact` (which compose `resize_impl` with
  cropping/nearest-neighbor pre-passes, per candidates 008/020/022) can amplify or dampen this
  effect wasn't checked.
- The rejected fix attempt's code was not preserved as a `patch.diff` in this folder (per the
  skill's rule that `patch.diff` is only included for a *verified* bounded fix) - its description
  above (the `to_*32f()` / premultiply-f32 / resize-f32 / unpremultiply-f32 / convert-back
  approach) is precise enough to reconstruct if a future session wants to build on it, e.g.
  combined with an alpha-floor clamp to address the regression found here.
- Whether the same fixed-point-saturation-desync mechanism is reachable through
  `resize_plane8_with_alpha`/`resize_plane16_with_alpha` (the `LumaA` variants) with a real
  (non-synthetic) low-detail grayscale-with-alpha source wasn't separately empirically probed
  beyond the `u8`/`u16` sweep already covering `LumaA` implicitly via the same code path -  only
  `Rgba` was used for the extreme per-pixel dump.
