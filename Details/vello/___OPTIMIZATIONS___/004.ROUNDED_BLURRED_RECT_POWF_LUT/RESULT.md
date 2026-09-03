# Rounded blurred rect: replace per-pixel scalar `powf` p-norm with a precomputed LUT

`sparse_strips/vello_cpu/src/fine/common/rounded_blurred_rect.rs`'s `AlphaCalculator::next()`
(the per-8-pixel-group hot loop for every box-shadow / blurred-rounded-rect fill) computed the
SDF's 2D p-norm distance via `FloatExt::powf`, an explicitly `// TODO: SIMD` scalar loop that
unpacks each of the 8 SIMD lanes and calls `f32::powf` (a libm call) individually, three times
per iteration (`x1.powf(exp) + y1.powf(exp)`, then `.powf(recip_exp)`). Two open upstream PRs
(#1724, #1725) already target this exact spot: #1724 rewrites the expression into a normalized
`t = min/max` p-norm form (still 2 scalar `powf` calls, but `t` is now bounded to `[0, 1]`) and
#1725 builds on that to replace the now-bounded-domain `powf` calls with a precomputed lookup
table. Neither PR applied cleanly to this session's pinned base commit (their PRs sit on top of
an unrelated `Painter` trait refactor - `paint_u8`/`paint_f32` taking `&mut self` instead of
`self` by value - and #1724's shader-side hunk targets a `.wgsl` file that has since been
restructured into `.wesl`), so this candidate reimplements the same validated approach (p-norm
rewrite + LUT) directly against the current base rather than porting the PRs' diffs verbatim.
Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

## What was tried

A 256-entry (`P_NORM_LUT_SIZE = 255`, +1 guard entry) `f32` lookup table for
`(1 + t^exponent)^recip_exponent` is precomputed once per fill (in `SimdRoundedBlurredRect::new`,
called once per `BlurredRoundedRectFiller`, not once per pixel group) using the same two scalar
`powf` calls - but now only 256 times total instead of once per 8-pixel group. Inside the hot
loop, `p_norm_distance` computes `t = min(x,y)/max(x,y)` (always in `[0, 1]`, unchanged from
#1724's algebraic rewrite) and looks up `lut[idx]`/`lut[idx+1]` with linear interpolation via
`f32x8::from_fn`, replacing the two remaining scalar `powf` calls entirely. The now-unused
`FloatExt::powf` scalar-loop method (the `// TODO: SIMD` code) was removed.

Benchmark: existing `sparse_strips/vello_bench/src/fine/rounded_blurred_rect.rs` /
`fine_rounded_blurred_rect` criterion group (`cargo bench -p vello_bench --features extended
--bench main -- rounded_blurred_rect`), unmodified - reused as-is, no new harness needed. This
machine has no NEON target (x86_64), so the criterion group runs its `sse4_2` and `avx2` SIMD
levels only (fearless_simd auto-detected at runtime by the bench).

Single-core numbers below are `taskset -c 3`, run under `flock` serialized against sibling
candidates' own builds/benchmarks running concurrently on this machine as part of the same
batch session; baseline was measured twice back-to-back with no code change to establish the
noise floor before applying the patch (this machine's own noise floor between two identical
runs was itself up to ~20-30% on some variants under that concurrent load - see caveats below).

| Variant | single-core p50 (with_transform, sse4_2) | single-core p50 (with_transform, avx2) | single-core p50 (no_transform, sse4_2) | single-core p50 (no_transform, avx2) |
|---|---|---|---|---|
| baseline run 1 | 11.23 µs | 11.47 µs | 14.82 µs | 10.96 µs |
| baseline run 2 (noise-floor check) | 11.07 µs | 10.56 µs | 11.00 µs | 10.71 µs |
| patched run 1 | 8.84 µs | 6.50 µs | 9.34 µs | 6.46 µs |
| patched run 2 (repeat) | 8.22 µs | 6.45 µs | 8.25 µs | 6.36 µs |
| patched, unpinned (realistic wall-clock, same concurrent-load caveat) | 8.27 µs | 6.39 µs | 8.31 µs | 8.18 µs (noisy - see caveats) |

## Result

Time only (no allocation/RSS claim - this candidate doesn't change allocation patterns in the
hot loop; the LUT itself is one `Arc`-allocated array per fill, not per pixel).

Comparing the more stable of the two baseline runs against the patched runs (both pinned to
core 3, criterion's own paired significance test reported `p = 0.00` for every comparison,
"Performance has improved"):
- `with_transform_u8_sse4_2`: ~11.1 µs -> ~8.2-8.8 µs, roughly **-20% to -26%**
- `with_transform_u8_avx2`: ~10.6-11.5 µs -> ~6.45-6.50 µs, roughly **-38% to -41%**
- `no_transform_u8_sse4_2`: ~11.0 µs (best baseline run; the other was noisier at ~14.8 µs) -> ~8.25-9.34 µs, roughly **-15% to -25%**
- `no_transform_u8_avx2`: ~10.7-11.0 µs -> ~6.36-6.46 µs, roughly **-39% to -42%**

The AVX2 variants show the largest, most consistent win (~40%), plausibly because AVX2's wider
execution resources make the previously-dominant scalar `powf` calls (which can't use AVX2 at
all) an even larger fraction of total time relative to the surrounding vector work. Criterion's
own two consecutive patched-vs-patched comparisons (run 1 -> run 2) show much smaller deltas
(0-15%, mostly "within noise threshold" per criterion's own classification), consistent with the
baseline-vs-patched gap being real and not just noise.

**Caveat on the unpinned/multi-core number**: this session ran as part of a parallel batch, with
several sibling candidates' own `cargo build`/`cargo bench`/`cargo test` invocations serialized
through the same `flock` but still contending for shared cache/memory bandwidth on this machine
(confirmed via `ps aux` showing concurrent sibling `flock`-wrapped commands during this
candidate's own runs). The unpinned `no_transform_u8_avx2` run shows a +14.8% "regression" purely
from that contention (criterion compared it against the immediately preceding pinned patched run,
not against baseline) - not attributed to the patch; the pinned, repeated single-core numbers
above are the ones actually trusted for the headline comparison. This benchmark exercises a
single-threaded fill routine, so "multi-core" here means "unpinned, realistic scheduler
placement" rather than a multi-threaded workload; there's no separate multi-threaded rendering
path for this specific code to exercise.

## Does it change behavior/output?

- **Bit-exact?** No - characterized difference.
- What changes: the LUT (256 samples across `t ∈ [0, 1]`, linearly interpolated) approximates
  the exact `(1 + t^exponent)^recip_exponent` factor instead of computing it exactly via `powf`.
  Measured directly: rendered 13 diverse `fill_blurred_rounded_rect` scenes (radius 0-60,
  std_dev 0-40, identity/rotated/skewed transforms, inverted and non-inverted) at 200x200 RGBA8
  (2,080,000 total bytes) through a scratch example (`vello_cpu` built with `RenderContext`
  directly, output pixels diffed byte-for-byte, not compared via the repo's own LFS-backed PNG
  snapshots - see below) - **16 of 2,080,000 bytes differed, every one by exactly 1/255**, all
  near antialiased edges of the blur falloff. This is well within normal 8-bit rounding noise for
  antialiased rendering and would not be visible to a human viewer. No case showed a larger
  difference.
- **Is this a breaking change?** No - no public API changed (`BlurredRoundedRectFiller`,
  `AlphaCalculator`, `SimdRoundedBlurredRect`, and `p_norm_distance` are all private to
  `vello_cpu`), no file format or documented behavior changed. The ±1/255 pixel difference is the
  same category of change the project already accepted for PR #1724/#1725 (both PRs' own diffs
  updated the repo's committed snapshot PNGs to match their new output).
- **Project test suite**: `cargo test --release -p vello_cpu -p vello_common`.
  - Before: `vello_common` 159 passed, 0 failed, 1 ignored; `vello_cpu` 81 passed, 0 failed.
  - After: identical - `vello_common` 159 passed, 0 failed, 1 ignored; `vello_cpu` 81 passed, 0
    failed. **No new failures; pass counts are bit-for-bit identical before and after.**
  - `vello_cpu`'s doc-tests fail to even compile in this environment, both before and after this
    patch, for a reason entirely unrelated to it: `sparse_strips/vello_cpu/src/text_debug.rs`
    imports `glifo::atlas::GlyphCacheStats`, which is gated behind
    `#[cfg(all(debug_assertions, feature = "std"))]` and therefore doesn't exist in a `--release`
    doctest build. Pre-existing on the pristine base commit; not something this patch introduced
    or could fix.
  - `vello_sparse_tests` (the crate with the repo's own snapshot tests for this exact code path,
    including `blurred_rounded_rect_medium_std_dev`/`inverse_blurred_rounded_rect_medium_std_dev`
    - the two snapshots #1724/#1725 themselves had to update) was **not** run to completion: this
    environment doesn't have `git-lfs` installed, so the repo's snapshot PNGs are checked out as
    plain LFS pointer text files, not real images - snapshot comparison would fail identically
    before and after this patch, for a reason unrelated to it. Building the full crate (it pulls
    in `wgpu`/`vello_hybrid`) was also heavily contended for the shared build lock by sibling
    candidates in this batch and didn't finish within a reasonable wait. The scratch
    scene-rendering + raw-byte-diff check above (13 cases, 2.08M bytes) is the correctness
    evidence actually collected for this candidate instead.

## Why the result looks the way it does

Scalar `powf` (libm, typically a polynomial/table-based software implementation on top of
`log2`/`exp2`) is far more expensive per element than a table lookup + `mul_add` interpolation,
and the original code paid for it 8 times per SIMD lane group (unpacked in a `TODO: SIMD` loop)
across 3 separate `powf` invocations per pixel group. Reducing to a single interpolated lookup
per lane removes essentially all of that scalar overhead from the hot loop; the LUT itself is
tiny (256 `f32`s = 1 KiB, fits in L1) and built only once per fill, not per pixel.

## What's not done / open questions

- This candidate's implementation is independent of (not a port of) upstream #1724/#1725; if
  either PR is later rebased and merged, it should independently confirm the same numbers rather
  than assuming this candidate's measurement transfers, since the exact instruction sequences
  differ (e.g. this candidate keeps a per-fill `Arc<[f32; 256]>` rather than caching the LUT on
  `EncodedBlurredRoundedRectangle` via `OnceCell` the way #1725 does - the `OnceCell` approach
  would avoid rebuilding the LUT on every fill of the same encoded paint and could plausibly do
  slightly better than the numbers measured here for scenes that fill many pixel groups from the
  same paint object across multiple tiles/fills, which this micro-benchmark's single-fill-per-run
  shape does not exercise).
- The GPU/WGSL shader path (`vello_sparse_shaders`, exercised by `vello_hybrid`) has the same
  `pow`/`pow` pattern in `calculate_blurred_rounded_rect` and was not touched by this candidate -
  GPU `pow` isn't a scalar-libm bottleneck the way CPU `f32::powf` is, and #1724's own PR
  description notes "for GPU, it doesn't seem to have much of an effect" from porting the same
  algebraic rewrite there.
- `vello_sparse_tests`'s real snapshot-based correctness check for this code path could not be
  run in this environment (missing `git-lfs`); a maintainer with `git-lfs` installed should run
  `cargo test -p vello_sparse_tests blurred_rounded_rect` before/after to confirm the ±1/255
  characterization above against the project's own snapshot tooling.
