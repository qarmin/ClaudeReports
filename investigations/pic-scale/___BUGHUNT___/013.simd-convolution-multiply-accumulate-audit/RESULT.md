# SIMD convolution/multiply-accumulate correctness against the scalar `FilterWeights` reference

From `IDEAS.md` (source: `012.simd-hand-vectorized-kernel-formula-check/RESULT.md`): candidate
`012` ruled out a hand-vectorized kernel-*formula* re-implementation (none exists) but explicitly
did not audit the convolution/multiply-accumulate code that *consumes* `FilterWeights` - the
per-tap `weight * pixel` accumulation loops in `src/avx2`/`src/sse`/`src/avx512` that back
`Scaler::plan_*_resampling*`. This candidate audits that code for three suspected bug classes:
accumulator overflow, kernel-window (tap) indexing off-by-one, and fixed-point `i16` weight
rounding discrepancies vs. the scalar `fixed_point_horizontal.rs`/`fixed_point_vertical.rs`
reference.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading: AVX2 RGBA/RGB u8 horizontal kernels (`src/avx2/rgba_u8.rs`, `src/avx2/rgb_u16.rs`) vs. the scalar reference (`src/fixed_point_horizontal.rs`, `src/filter_weights.rs`) | Read the tap-index arithmetic (`bounds_start = bounds.start + jx`, matching the scalar `px = start_x * CN` + sequential-tap loop exactly), the accumulator width (`i32`, `ROUNDING_CONST = 1<<14`, `PRECISION = 15`), the weight-quantization function (`quantize_kernel` in `filter_weights.rs`, shared by every backend - not duplicated per-SIMD-target, so there is nowhere for a per-backend rounding *formula* discrepancy to live), and the final narrowing (`_mm256_srai_epi32::<PRECISION>` matching the scalar `>> PRECISION`) | No indexing or narrowing-formula mismatch found by inspection; `grep -rn 'bounds\.start'` across `src/avx2`/`src/sse`/`src/avx512` confirms the same `bounds.start + <tap index>` pattern everywhere, no variant found |
| Differential release-build testing: two binaries built from identical source (`probe/simd`, default features -> AVX2/SSE dispatch; `probe/scalar`, `default-features = false, features = ["threading"]` -> scalar-only fallback), run against identical deterministic content through the public API (`Scaler::plan_rgb_resampling`/`plan_rgba_resampling(premultiply_alpha=false)`/`plan_rgb_resampling16`/`plan_rgb_resampling_f32`) across 11 `ResamplingFunction` variants x 16 sizes (1x1 up to 1000x2, including widths not a multiple of any SIMD lane width: 1, 2, 3, 7, 13, 17, 31, 65, 99, 199, 251, 997-1000) x bit depths 8/10/12/16 for u16 | ~530k `u16` output samples + ~2.1M `u8` output bytes + ~450k `f32` output samples compared byte-for-byte/bit-for-bit; `just run-diff` in this folder reproduces it | `u8` RGB and RGBA(no-alpha): **0 diffs** (bit-exact). `u16` bit_depth in {8,10,12} (the `i16`-fixed-point convolution path): **0 diffs** (bit-exact) across all 11 functions x 16 sizes. `u16` bit_depth=16 (the `f32`-accumulator convolution path, used whenever `bit_depth > 12`): **54/704** rows differ, always by exactly `+/-1` on 1-2 samples per row - see Root cause. `f32` RGB: a small number of rows differ by 1 ULP in the raw bit pattern (expected float non-associativity, not investigated further - narrower/rarer than the u16 case and not what this candidate was scoped to) |
| Manual boundary-value construction: adversarial extreme-downscale inputs (`200,000x1 -> 1x1`, `200,000x1 -> 3x1`, `150,000x1 -> 2x1`) with alternating min/max pixel values (0 / `(1<<bit_depth)-1`) to maximize `sum(\|weight_i\| * pixel_i)` at very large `kernel_size` (probes `i32` accumulator overflow in the fixed-point path), run under both a normal release build and a `-C overflow-checks=on -C debug-assertions=on` debug build | 4 extreme cases x {simd, scalar} x {release, debug+overflow-checks} = 16 runs | No panic (no overflow) in any run; SIMD and scalar outputs bit-exact in every case - see `output.txt`'s "EXTREME" block |

## Status

**Not reproducible.**

- **Accumulator overflow**: not found. Weights are quantized to `i16` with `sum(weights) ==
  precision_scale` (`1<<15`) exactly (residual-redistribution in `quantize_kernel`,
  `src/filter_weights.rs`), and the `i32` accumulator (pixel up to `65535` for the one case where
  `bit_depth==16` reaches the fixed-point path - it doesn't, see below - or up to `4095` for the
  actual max fixed-point `bit_depth==12`) stays far below `i32::MAX` regardless of `kernel_size`,
  confirmed by construction (`sum(weights)` is bounded independent of tap count) and by an
  adversarial extreme-`kernel_size` boundary test (200,000-tap kernels, alternating min/max pixel
  content) that neither panics under `overflow-checks=on` nor diverges from the scalar reference.
- **Kernel-window (tap) indexing off-by-one**: not found. Every backend checked uses the identical
  `bounds.start + <tap index>` addressing scheme as the scalar reference; a 16-size x 11-function
  differential sweep (including sizes deliberately not a multiple of 2/4/8 to exercise every
  remainder/tail branch in the AVX2 kernels, and multi-row heights to exercise the vertical
  convolution pass too) produced zero mismatches for every integer (`u8`, `u16` bit_depth<=12)
  code path - an indexing bug would show up as a wrong pixel value here, and none did.
- **Fixed-point `i16` weight rounding discrepancies**: not found. The weight-quantization function
  (`quantize_kernel`) is shared by every backend (not duplicated per SIMD target), so scalar and
  SIMD always convolve with the *exact same* `i16` weight values; the differential sweep confirms
  bit-exact agreement for every fixed-point (`bit_depth<=12`) case tested.
- **A real, but different, discrepancy was found and is not being reported as a bug**: for
  `bit_depth==16` (which routes to a `f32`-accumulator convolution, not the `i16`-fixed-point one -
  see `src/factory/rgb_u16.rs`/`rgba_u16.rs`, `if options.bit_depth <= 12`), the AVX2 kernel
  (`src/avx2/rgb_u16.rs`, `OneRowExecutionHandlerRgb`) accumulates 8 taps at a time into a single
  256-bit register pair and folds the two 128-bit halves together at the end
  (`_mm_add_ps(low128, high128)`), while the scalar reference (`floating_point_horizontal.rs`)
  accumulates strictly sequentially, one tap at a time, via `sums.mul_add(new_px, weight)`. Both
  are valid IEEE-754 evaluations of the same sum, just in a different order, so they can (and,
  54/704 of the time in this sweep, do) differ by 1 ULP in the raw `f32` sum - which occasionally
  lands on the other side of an integer rounding boundary after `MixedStorage::to_mixed`/
  `_mm_cvtps_epi32`, producing a `+/-1` output difference. This is standard, expected
  floating-point non-associativity for any vectorized reduction (present in essentially every
  SIMD-accelerated image resizer), not an accumulator/indexing/rounding-*formula* defect: it
  requires `kernel_size > 1` (confirmed - `Nearest`, whose `kernel_size` is always 1, never
  appears among the 54 mismatching rows out of the many `Nearest` rows tested), it reproduces
  identically on every rerun (deterministic, not a flaky/layout-dependent difference), and it is
  the same underlying phenomenon as the (also-expected, not investigated further) 1-ULP `f32`
  output differences seen in the raw `f32` RGB block.

Covered this session: `u8` RGB/RGBA (no alpha) and `u16` bit_depth 8/10/12 fixed-point convolution,
both horizontal and vertical (via multi-row test sizes), on AVX2+SSE (this host's native SIMD
dispatch). **Not** covered: AVX-512 (this host's one `avxvnni`-independent AVX-512 path is
`check_alpha.rs`, already audited in candidate `004` - the convolution kernels under
`src/avx512/` gate on `avxvnni`, which this host lacks, per `projects/pic-scale.md`'s AVX-512
reachability caveat), NEON/SVE2 (no ARM hardware available), Plane/CbCr (1-/2-channel) convolution
specifically (only RGB/RGBA, 3-/4-channel, were swept), and `i16` (signed planar) output.

## Class

N/A (not reproduced as a distinct bug in any of the three suspected classes).

## Root cause

N/A for the negative result. For the one real (but non-bug) discrepancy found: see the "A real,
but different, discrepancy..." paragraph above - SIMD tree-style 8-tap-at-a-time accumulation vs.
scalar's strictly sequential per-tap accumulation is a different, equally valid floating-point
summation order, not a coding defect.

## The fix

N/A - no bug to fix. (The `f32`-accumulator ULP-level divergence has no "fix" in any meaningful
sense without abandoning SIMD-parallel accumulation entirely, which would defeat the point of the
AVX2/SSE code paths for a difference that's already within normal floating-point tolerance for
image resampling.)

- Project test suite: N/A, no source change was made this candidate. For reference,
  `cargo test --lib --features nightly_f16` against the pristine clone (see this folder's
  `justfile`'s `baseline-tests` recipe): 75 passed; 0 failed; 0 ignored.

## What's not done / open questions

- AVX-512 convolution kernels under `src/avx512/` (as opposed to `check_alpha.rs`, already
  covered) are gated on `avxvnni`, which this host doesn't have - not exercised at all this
  session. Worth a follow-up on a host with `avxvnni` (Intel client CPU).
- NEON/SVE2 convolution kernels under `src/neon/`/`src/sve2/` - no ARM hardware available on this
  host, same limitation noted by several prior candidates.
- Plane (1-channel) and CbCr (2-channel) convolution specifically weren't swept (only RGB/RGBA,
  3-/4-channel) - by code reading they share the same `fixed_point_horizontal.rs`/
  `floating_point_horizontal.rs` scalar reference and the same `bounds.start`-indexing pattern in
  their own `src/avx2/plane_*.rs`/`src/sse/plane_*.rs` kernels, so this is a low-priority
  follow-up, not a distinct suspect.
- `i16` (signed planar, `plan_planar_resampling_s16`) output wasn't swept.
- The `f32`-accumulator ULP-divergence rate (~7.7% of `bit_depth==16` u16 rows, i.e. rows with at
  least one +/-1 sample, across this sweep's mix of functions/sizes) could be characterized more
  precisely (which specific functions/kernel-sizes are more prone) if useful, but this candidate's
  read is that it's expected floating-point behavior, not worth further chasing as a "bug."
