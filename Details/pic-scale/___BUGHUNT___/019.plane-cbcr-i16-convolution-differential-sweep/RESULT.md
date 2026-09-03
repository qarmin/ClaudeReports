# Plane (1-channel)/CbCr (2-channel)/i16 (signed planar) convolution differential-test sweep

From `IDEAS.md` (source: `013.simd-convolution-multiply-accumulate-audit/RESULT.md`): candidate
`013`'s differential test against the scalar reference swept RGB/RGBA (3-/4-channel), `u8`/`u16`/
`f32` SIMD convolution kernels only (all clean) and explicitly did not extend the same sweep to
the Plane (1-channel), CbCr (2-channel), or `i16` (signed planar) convolution kernels. This
candidate extends candidate `013`'s exact differential-test harness/methodology to
`Scaler::plan_planar_resampling`/`plan_cbcr_resampling` (`u8`), `plan_planar_resampling16`/
`plan_cbcr_resampling16` (`u16`), and `plan_planar_resampling_s16` (`i16`) for the same three
suspected bug classes: accumulator overflow, kernel-window (tap) indexing off-by-one, and
fixed-point/floating-point rounding discrepancies vs. the scalar reference.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential release-build testing: two binaries built from identical source (`probe/simd`, default features -> AVX2/SSE dispatch; `probe/scalar`, `default-features = false, features = ["threading"]` -> scalar-only fallback), run against identical deterministic content through the public API (`plan_planar_resampling`/`plan_cbcr_resampling` u8, `plan_planar_resampling16`/`plan_cbcr_resampling16` u16, `plan_planar_resampling_s16` i16) across 11 `ResamplingFunction` variants x 16 sizes (including widths/heights not a multiple of any SIMD lane width, and heights >= 2 to exercise the vertical convolution pass) x bit depths 8/10/12/16 (u16/i16), plus 4 adversarial extreme-downscale (up to 200,000:1) cases per type | u8 Plane: 641,050-char output line compared byte-for-byte; u8 CbCr: 1,301,019-char line; u16 Plane/CbCr and i16 Plane: 704 rows each x 3 output value types compared; re-ran the full sweep 3x on each binary | u8 Plane, u8 CbCr: **0 diffs** (bit-exact). `u16`/`i16` bit_depth in {8,10,12} (fixed-point AVX2 path): **0 diffs** across all types, functions, sizes. `u16` bit_depth=16 Plane (`f32`-accumulator path): 32/704 rows differ, max abs diff 2, always `Nearest`-free (same summation-order class candidate `013` found for RGB/RGBA). `u16` bit_depth=16 CbCr: 2/704 rows differ (3 sample values), max abs diff 1. `i16` bit_depth=16 Plane: **0 diffs** (see Root cause - this path has no x86_64 SIMD dispatch at all, confirmed by code reading and empirically). Extreme-overflow probes: no panic, no crash, byte-identical simd vs. scalar in every case. All results reproduced identically across 3 reruns of each binary (deterministic) |
| `-C overflow-checks=on -C debug-assertions=on` rebuild of both `probe/simd` and `probe/scalar`, full sweep re-run (including the 4 extreme-downscale cases) | Same input sweep as above, checked-arithmetic build | No panic in either binary; output byte-for-byte identical to the plain-release-build output for both binaries (0 diffs) - confirms no silent integer overflow and no arithmetic panic anywhere in the swept Plane/CbCr/i16 range |
| Manual code reading: AVX2/SSE Plane and CbCr kernels (`src/avx2/plane_u16.rs`, `src/avx2/plane_s16_lb.rs`, `src/sse/plane_u8.rs`, `src/sse/cbcr8.rs`) vs. the scalar reference, plus the dispatch conditions in `src/factory/plane_u16.rs`, `src/factory/cbcr16.rs`, `src/factory/plane_s16.rs` | `grep -rn 'bounds\.start'` across the checked files; read every `#[cfg(...)]`-gated dispatch branch to determine which bit-depth/channel-count combinations actually reach AVX2/SSE on this host vs. always falling to scalar | Same `bounds.start + jx`/`bounds_start` tap-indexing pattern as candidate `013`'s RGB/RGBA kernels - no variant found. Also found (see Root cause) that `CbCr` `u16` horizontal has no AVX2/SSE dispatch at all on x86_64 (any bit depth), and `i16` Plane bit_depth>12 also has no AVX2/SSE dispatch on x86_64 - both always route through the scalar reference regardless of which probe binary runs them, which is why those specific combinations show 0 diffs by construction, not just by outcome |

## Status

**Not reproducible.**

- **Accumulator overflow**: not found. No panic under `overflow-checks=on` for any of the 12
  extreme-downscale (up to 200,000:1 kernel-size) cases across Plane u16, CbCr u16, and signed
  `i16` Plane (alternating `i16::MIN`/`i16::MAX` content to maximize the signed accumulator's
  swing), and SIMD/scalar outputs stayed byte-identical in every one of those cases.
- **Kernel-window (tap) indexing off-by-one**: not found. Every AVX2/SSE Plane/CbCr kernel checked
  uses the same `bounds.start + jx` addressing scheme as the scalar reference and as the RGB/RGBA
  kernels candidate `013` already audited; the differential sweep (176 size/function combos,
  including heights >= 2 to exercise the vertical pass and sizes deliberately not a multiple of
  2/4/8 to exercise remainder/tail branches) produced zero mismatches for every integer
  (`u8`, `u16`/`i16` bit_depth<=12) code path that actually has an x86_64 SIMD implementation.
- **Fixed-point/floating-point rounding discrepancies**: not found as a distinct defect. The
  `u16`/`i16` bit_depth<=12 fixed-point paths are bit-exact between SIMD and scalar in every case
  tested. The `u16` bit_depth=16 `f32`-accumulator path shows the same expected
  floating-point-summation-order divergence (SIMD's multi-lane tree accumulation vs. scalar's
  strictly sequential `mul_add`) already documented as non-a-bug in candidate `013` - this sweep's
  larger Plane16 sample count surfaced a couple of `+/-2` cases (013's RGB/RGBA sweep only saw
  `+/-1`), still consistent with the same underlying phenomenon: deterministic (identical across
  3 reruns), absent for `Nearest` (`kernel_size` always 1, so there is no summation-order freedom
  to begin with), and only appears where multiple floating-point terms are actually summed in a
  different order.
- **A structural finding, not a bug**: by code reading and confirmed empirically (identical output
  from both binaries by construction, not by chance), two of the swept entry points have **no**
  x86_64 AVX2/SSE convolution kernel at all and always use the scalar reference on this host
  regardless of which probe binary calls them: `CbCr` `u16` **horizontal** convolution
  (`src/factory/cbcr16.rs` - both the bit_depth<=12 branch and the bit_depth>12 branch route
  straight to `convolve_row_handler_fixed_point`/`convolve_row_handler_floating_point`, the scalar
  functions, inside the `#[cfg(not(all(target_arch = "aarch64", feature = "neon")))]` block, with
  no intervening `#[cfg(target_arch = "x86_64")]` AVX2/SSE branch the way the sibling
  `plane_u16.rs`/`rgba_u16.rs` have), and `i16` Plane bit_depth>12 (`src/factory/plane_s16.rs`,
  both horizontal and vertical - the AVX2 branches are unconditionally inside `if options.bit_depth
  <= 12`, and bit_depth>12 falls through to `floating_point_horizontal`/`floating_point_vertical`
  scalar handlers with no AVX2 alternative on x86_64, only a NEON+`rdm` one). This is a
  performance-reachability gap (an SSE/AVX2 kernel exists for `CbCr` `u16` vertical and `i16`
  Plane bit_depth<=12, but not the combinations named above), not a correctness defect - logged to
  `IDEAS.md` as a new suspect for a dedicated `perf-optimize` look, not investigated further here.

## Class

N/A (not reproduced as a distinct bug in any of the three suspected classes).

## Root cause

N/A for the negative result. For the two real-but-non-bug findings:
- The `f32`-accumulator ULP/summation-order divergence: identical mechanism to candidate `013`'s
  already-documented root cause (SIMD's multi-tap-at-once tree accumulation vs. scalar's strictly
  sequential per-tap `mul_add`, both valid IEEE-754 evaluation orders of the same sum).
- The "no AVX2/SSE dispatch at all" combinations (`CbCr` u16 horizontal, `i16` Plane bit_depth>12):
  the `#[cfg(...)]`/`if bit_depth` gating in `src/factory/cbcr16.rs` and `src/factory/plane_s16.rs`
  simply never emits an x86_64-specific branch for those combinations - by inspection this looks
  like an intentional (if perf-suboptimal) scope decision rather than a mistake, since the
  sibling `u16` Plane (`plane_u16.rs`) and `u16`/`i16` bit_depth<=12 paths do have one.

## The fix

N/A - no bug to fix.

- Project test suite: N/A, no source change was made this candidate. For reference,
  `cargo test --lib --features nightly_f16` against the pristine clone (`just test` in this
  folder, `baseline-tests` recipe, re-verified against a fresh scratch clone this session):
  75 passed; 0 failed; 0 ignored.

## What's not done / open questions

- AVX-512 (`avxvnni`-gated) and NEON/SVE2 Plane/CbCr/i16 convolution kernels - same hardware
  limitation as candidate `013`'s own open items, not exercised this session.
- The two "no x86_64 SIMD dispatch at all" combinations found this session (`CbCr` u16 horizontal,
  `i16` Plane bit_depth>12) mean this sweep's "0 diffs" result for those specific rows is
  necessarily true by construction (same scalar code runs regardless of which probe binary calls
  it) rather than a confirmation that an existing SIMD implementation is correct - added to
  `IDEAS.md` as a new performance-reachability suspect, not a correctness one.
- The `f32`-accumulator summation-order divergence's rate/magnitude wasn't characterized further
  (which functions/sizes are more prone) beyond confirming it's the same class as candidate `013`
  already ruled a non-bug - not chased further here, same reasoning as `013`.
