# AVX2 u16 unpremultiply wrongly zeroes color channels on zero-alpha pixels

Backlog item from `IDEAS.md` (sourced from `006.avx2-sse-u16-premultiply-bit-depth-16-rounding/RESULT.md`):
audit `_mm256_madd_epi16`/`_mm_madd_epi16`-based fixed-point multiplies in `src/avx2/`/`src/sse/`
beyond candidate 006 (which fixed the RGBA u16 premultiply *forward* direction), specifically the
*unpremultiply* direction and any sibling u16 kernel using the same zero-extend-then-`madd` idiom.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `grep -rn "madd_epi16" src/avx2/ src/sse/ src/avx512/ src/neon/` + manual read of every call site's surrounding function | Full repo source read | 90+ call sites classified into two shapes: (a) a zero-extended pixel value multiplied against a legitimately-signed filter weight (`rgba_u8.rs`, `rgb_u8.rs`, `cbcr8.rs`, `vertical_u8.rs`, AR30 kernels, the generic `_mm256_dot16_avx_epi32`/`_mm_dot16_avx_epi32` helpers) - correct, intended signed-weight usage, not the candidate-006 bug class; (b) `alpha_u16.rs`'s premultiply `Default` executor - the exact spot candidate 006 already covers (only reachable at `bit_depth` 10/12 in the current unpatched source, so not independently live). The `_lb` ("low bit-depth") sibling family (`vertical_u16_lb.rs`, `rgba_u16_lb.rs`, `rgb_u16_lb.rs`, `plane_u16_lb.rs`, both backends) that zero-extends a pixel value into `madd_epi16` is statically gated by the factory dispatch (`src/factory/rgba_u16.rs:46`, `rgb_u16.rs:44`, `plane_u16.rs:43`/`191`) to `bit_depth <= 12` (max value 4095, always positive as `i16`) - `bit_depth == 16` never reaches them. |
| Full read of both backends' *unpremultiply* functions (`avx_unpremultiply_alpha_rgba_u16_row_avx` in `src/avx2/alpha_u16.rs`, `DisassociateAlphaDefault`/`sse_unpremultiply_row_u16` in `src/sse/alpha_u16.rs`) | Manual code reading | Neither calls `madd_epi16`/`_mm_madd_epi16` at all - both use a per-pixel `f32` reciprocal divide for every bit depth, so the specific signed-lane-`madd` trap does not apply to the unpremultiply direction. This ruled out the originally-suspected bug class for this direction, but reading the code closely surfaced a *different* logic bug in the same functions (see below). |
| Direct differential test of `avx_unpremultiply_alpha_rgba_u16`/`unpremultiply_alpha_sse_rgba_u16` (the crate's actual production dispatch functions, called via the real `bit_depth==16` + CPU-feature-detected dispatch path in `src/alpha_handle_u16.rs`) against the crate's own scalar reference (`unpremultiply_alpha_rgba_row`), full u16 domain, boundary values + 50k random samples per backend | `cargo test --lib bughunt_diff_tests`, debug build | **AVX2 diverged**: every mismatch was an `alpha == 0` pixel with nonzero color, e.g. `src=[4043,1661,1092,0]` (already-convolved premultiplied color, alpha exactly 0) -> AVX2 output `[0,0,0,0]`, scalar reference `[4043,1661,1092,0]` (scalar's `if a != 0 { .. }` leaves the pixel untouched). SSE passed with **0** mismatches - it already handles this correctly via `_mm_select_si128(is_zero_mask, x, pixel)`, which preserves the original pixel; AVX2 instead `blendv`s in an all-zero vector for zero-alpha pixels, incorrectly zeroing R/G/B. |
| Real end-to-end reproduction via the public API: two release builds (`repro/simd`, AVX2-enabled default features; `repro/scalar`, `default-features = false, features = ["threading"]`) of identical source, differing only in which Cargo features are compiled into the `pic-scale` dependency, both calling `Scaler::plan_rgba_resampling16(.., premultiply_alpha: true, bit_depth: 16)` with `ResamplingFunction::Lanczos3` (a negative-lobe filter, needed so the convolution stage genuinely drives some output pixels' alpha to exactly 0 while color stays nonzero - `ResamplingFunction::Bilinear`, used by candidate 006, never does this since its weights are non-negative), 512->256 downscale, full-range random RGBA, seeds 1-50 | release build, `repro/compare.sh` | **Reproduced: 11/50 seeds** show a large (173-4043, out of 65535) per-value divergence between the two builds - traced back (via temporary instrumentation) to exactly the same alpha==0/color-zeroed mechanism found in the direct differential test. This is the real, public-API-only repro in `repro/`. |
| Root-cause search for a *second*, distinct bug originally suspected in this same function: does `_mm256_cvtps_epi32`/`_mm_cvtps_epi32` overflow to the "integer indefinite" value (0x80000000) when `color * (max_colors / alpha)` exceeds `i32::MAX` for a *small-but-nonzero* alpha (not exactly 0), which `_mm256_packus_epi32`'s unsigned saturation would then collapse to 0 instead of clamping high? | Direct-function boundary test (`color=32769, alpha=1`) confirmed this is a real, distinct, deterministic divergence at the function level, in both backends. An extensive automated search for a public-API trigger followed: 9+ resampling filters (Lanczos3/6, CatmullRom, MitchellNetravalli, RobidouxSharp, EwaLanczos4Sharpest, Bicubic, Robidoux, EwaRobidouxSharp, Ginseng, EwaGinseng, Lanczos2), extreme downscale/upscale ratios (up to 4096:1 and 1:2048), 2D (compounded horizontal+vertical) convolution, hand-constructed adversarial edge/spike patterns, and thousands of random-seed full-range trials, all instrumented to detect the exact overflow condition | **No public-API trigger found.** The adjacent "alpha exactly 0" (division by zero, `inf`) case fires constantly through the real pipeline (as above), but a *finite* overflow (small positive alpha, large color) was never observed - every random/adversarial construction that drove alpha down also drove color down proportionally, never reaching the needed ~32769:1 color:alpha ratio while alpha stayed a nonzero small integer. This finding was **not pursued as this candidate's confirmed bug** (see IDEAS.md - logged as a new backlog item for a future session) since a real end-to-end trigger could not be established despite substantial effort, unlike the alpha==0 bug above which reproduces readily. |

## Status

**Confirmed.**

Reproduction rate: 11/50 random seeds (~22%) at the chosen input distribution (full 0..=65535
color/alpha range, 512->256 Lanczos3 downscale, `bit_depth=16`, `premultiply_alpha=true`), each
divergence deterministic and repeat-stable across reruns of the same build. Root-caused down to
individual mismatching pixels (traced via temporary instrumentation) confirming every one is an
`alpha == 0` output pixel whose color channels were wrongly zeroed by the AVX2 path. This is the
real end-to-end public API entry point (`Scaler::plan_rgba_resampling16` -> `Resampling::resample`),
run against two builds of the identical source differing only in which CPU-dispatch Cargo features
are compiled into the `pic-scale` dependency - not a hand-constructed intermediate state.

**Fully reproducible**: **Yes.** `repro/compare.sh` builds both binaries from a pristine clone at
`base_commit` and diffs their real output for 50 seeds; `just test` runs the whole clone -> before
-> apply -> after cycle unattended and was actually executed (not just read) during this session.

## Class

logic

## Root cause

`avx_unpremultiply_alpha_rgba_u16_row_avx` (`src/avx2/alpha_u16.rs`) computes the unpremultiplied
color for every pixel unconditionally (dividing by alpha via a reciprocal float multiply), then
tries to special-case `alpha == 0` pixels (where the scalar reference `unpremultiply_alpha_rgba_row`
simply leaves `dst` untouched, per its `if a != 0 { .. }` guard) by blending in an all-zero vector:

```rust
packed0 = _mm256_blendv_epi8(packed0, _mm256_setzero_si256(), is_zero_alpha_mask0);
packed0 = _mm256_blendv_epi8(packed0, rgba0, copy_alpha_mask);
```

The first blend forces R/G/B (and A) to `0` for every zero-alpha pixel; the second blend then only
restores the *alpha* byte-pair from the original `rgba0`, leaving R/G/B at `0` - discarding whatever
nonzero premultiplied color the convolution stage had actually produced there instead of preserving
it, as the scalar reference (and the SSE sibling, via `_mm_select_si128(is_zero_mask, x, pixel)`,
which correctly selects the *entire original pixel*) both do. This is reachable in real usage
because a resize with a negative-lobe filter (Lanczos-family, CatmullRom, Mitchell, etc.) can
genuinely drive an output pixel's convolved alpha channel down to exactly 0 while its color channel
convolves to something nonzero (ringing near a translucent/opaque edge) - the scalar reference and
SSE both correctly preserve that nonzero color; AVX2 silently discards it, producing a black/zeroed
pixel instead. All three loops in the function (32-wide main loop, 16-wide remainder, scalar-buffer
tail) share the identical bug (copy-pasted structure), so all three needed the same fix.

This is a distinct root cause from candidate 006's `madd_epi16` signedness trap (that intrinsic
does not even appear in the unpremultiply direction, in either backend) - it's a blend/masking
logic bug, not a signed-vs-unsigned lane misinterpretation.

## The fix

In `avx_unpremultiply_alpha_rgba_u16_row_avx` (`src/avx2/alpha_u16.rs`), in all three loops: replace
`_mm256_blendv_epi8(packed, _mm256_setzero_si256(), is_zero_alpha_mask)` with
`_mm256_blendv_epi8(packed, rgba, is_zero_alpha_mask)` - i.e. blend in the *original, untouched*
pixel for zero-alpha pixels (matching the scalar reference and the already-correct SSE
implementation) instead of an all-zero vector. The subsequent `copy_alpha_mask` blend (which
restores the true alpha byte-pair for the *non*-zero-alpha case, undoing the unpremultiply
formula's own division-by-itself on the alpha lane) is unaffected and still needed.

Verified: `repro/compare.sh` against a pristine clone shows **REPRODUCED, 11/50 seeds** (max
per-value diff up to 4043) before the patch, and **did not reproduce, 0/50 seeds** after - see
`output.txt`. The crate's own internal differential test (`bughunt_diff_tests`, added via
`verification_tests.diff`, full u16 domain at `bit_depth=16`) fails identically before the fix and
passes after, for both AVX2 and SSE (SSE already passed even without any SSE-side change).

- Project test suite: **66 passed, 0 failed** before the patch; **68 passed, 0 failed** after
  (patch.diff + verification_tests.diff applied - the 2 new `bughunt_diff_tests` account for the
  difference). No test regressed; no test newly failed. (`cargo test --lib`, default features,
  this host.)

## What's not done / open questions

- A **second, distinct** bug was found in the same function during this investigation but is
  **not** part of this candidate's confirmed finding or its patch: `_mm256_cvtps_epi32`/
  `_mm_cvtps_epi32` can overflow to the "integer indefinite" value when `color *
  (max_colors / alpha)` exceeds `i32::MAX` for a small-but-*nonzero* alpha, which
  `_mm256_packus_epi32`'s unsigned saturation then collapses to `0` instead of clamping to
  `max_colors` - confirmed deterministically reproducible at the direct-function level (both
  AVX2 and SSE) across the full u16 domain, but despite an extensive automated search (9+
  negative-lobe filters, extreme ratios, 2D convolution, adversarial constructions, thousands of
  seeds) no input was found that drives the real public `Scaler` pipeline into this specific
  state (only the adjacent, already-fixed `alpha == 0` case fires in practice). Logged as a new
  `IDEAS.md` backlog item rather than claimed `Confirmed` here, since `CONFIRMATION.md` section 11's
  bar for a real end-to-end trigger wasn't met for it, unlike the bug fixed in this candidate.
- A small (+/-1..3 LSB out of 65535), separate, pre-existing rounding gap remains visible in
  `repro/compare.sh`'s output even after this patch (see the AVX2 premultiply `AnyBit` executor's
  f32-reciprocal approximation, already documented as candidate 006's own finding, unfixed in the
  current tree since that candidate's patch was reverted per this skill's standing rule) - this is
  expected, unrelated to this candidate, and does not trip `compare.sh`'s >50 threshold.
- Only x86_64 AVX2 was found broken; SSE4.1 already handles this correctly via `_mm_select_si128`
  and needed no fix (confirmed via the same differential test, 0 mismatches without any SSE
  change).
- `src/neon/` unpremultiply was not audited for an analogous zero-alpha-masking bug or the
  `madd`-equivalent signedness trap - no ARM hardware available on this host (already tracked as
  its own lower-priority `IDEAS.md` row from candidate 006).
- gray-alpha (LA) u16 unpremultiply was not in scope (see candidate 003: no LA SIMD kernels exist
  in this codebase at all).
