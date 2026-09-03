# YBR_FULL/YBR_FULL_422 -> RGB colorspace conversion: hand-written SSE4.1 SIMD rewrite

`convert_colorspace_u8`/`convert_colorspace_u16` (`pixeldata/src/lib.rs`) convert a decoded
YBR_FULL/YBR_FULL_422 pixel buffer to RGB in-place. Candidate 002 replaced the original
`f32`+`floor()` arithmetic with fixed-point integer arithmetic (a real, measured win - see that
candidate). Candidate 031's disassembly then confirmed neither the float nor the fixed-point
version actually auto-vectorizes: `chunks_mut(3)`/`par_chunks_mut(3)` hands exactly one 3-byte
pixel to the closure per call, which is a real function-call boundary (and, under rayon, an
additional per-chunk dispatch call) that blocks LLVM from ever seeing more than one pixel's worth
of work at a time. This candidate is a genuine hand-written SIMD rewrite: x86-64 SSE4.1/SSSE3
intrinsics processing 4 pixels/iteration (u8) or 2 pixels/iteration (u16), plus a necessary
increase in rayon's chunk granularity (large batches instead of one pixel per task) so the SIMD
loop actually has more than one pixel to work with per call under rayon too.

Base commit: `df2b9e4236298efa3fa37a1f360fc4bf284d586a` (same as candidates 002/031).

## What was tried

Kept the same fixed-point coefficients as candidate 002 (Q16.16, rounded to the nearest 1/65536:
1.402, 0.114*1.772/0.587, -0.299*1.402/0.587, 1.772) so the SIMD kernel is a pure vectorization of
the same arithmetic, not a new approximation. Added, for both `u8` and `u16`:

- A scalar fixed-point fallback function (`convert_colorspace_u8_scalar`/`_u16_scalar`, identical
  to candidate 002's per-pixel loop but written as a plain function over a whole slice instead of
  a `chunks_mut(3)` closure) - used for the tail (buffer length not a multiple of the SIMD width)
  and as the only path on non-x86_64 targets or x86_64 CPUs without SSE4.1.
- A `#[target_feature(enable = "sse4.1,ssse3")]` unsafe kernel, dispatched only after a runtime
  `is_x86_feature_detected!("sse4.1") && is_x86_feature_detected!("ssse3")` check:
  - **u8** (`convert_colorspace_u8_sse41`): loads 16 bytes unaligned per iteration (12 of which -
    4 pixels - are consumed; the trailing 4 are read but discarded, which is what makes the loadu
    safe without a masked/partial load), uses `_mm_shuffle_epi8` with three fixed byte-gather
    masks to simultaneously deinterleave and zero-extend Y/B/R into three `i32x4` registers (one
    `pshufb` per channel replaces what would otherwise be 4 separate scalar loads+widens),
    computes `cr`/`cg`/`cb` via `_mm_mullo_epi32`/`_mm_slli_epi32`/`_mm_srai_epi32` on all 4 lanes
    at once, clamps with `_mm_max_epi32`/`_mm_min_epi32`, then extracts the 12 result bytes via
    `_mm_extract_epi32` and stores them with plain scalar indexing (only the arithmetic is
    vectorized; the final store is scalar data movement, same as the pre-SIMD version already
    did per pixel).
  - **u16** (`convert_colorspace_u16_sse41`): same idea but needs an `i64` accumulator (as
    candidate 002 noted: `y << 16` alone can reach ~4.3e9 for 16-bit samples, over `i32::MAX`).
    Gathers Y/B/R for 2 pixels into 32-bit lanes 0/2 of an `i32x4` (the lanes `_mm_mul_epi32`
    reads), uses `_mm_mul_epi32` both as the real 32x32->64 multiply against each coefficient and
    as a "multiply-by-1" trick to widen `y` to 64-bit, adds in 64-bit lanes via `_mm_add_epi64`/
    `_mm_slli_epi64`, then extracts the two 64-bit accumulators and does the final `+HALF >> 16`
    shift and clamp in scalar code (SSE has no signed 64-bit arithmetic-shift-right instruction
    without AVX512VL) - so only the multiply-add portion is vectorized for `u16`, at half the
    pixels/iteration of the `u8` kernel.
- `convert_colorspace_u8`/`_u16` (the public-to-the-module entry points) now batch rayon into
  large chunks (`par_chunks_mut(3 * 4096)` pixels) instead of one pixel per task, then call the
  SIMD-dispatching slice function on each batch; the non-`rayon` build calls the same slice
  function directly over the whole buffer, no `chunks_mut` at all. This chunk-size increase is
  not a separable change from the SIMD rewrite - dispatching one 3-byte pixel per rayon task, as
  before, would leave the SIMD loop with only one pixel to work on per call regardless of how it's
  written internally, so the multi-core number below reflects both effects together.

Correctness validation before the real bench: a temporary (not part of `patch.diff`) unit test
compared the SSE4.1 kernel's output against the scalar fixed-point path for `n_pixels` in `0..40`
(covering every possible SIMD/tail-remainder split) x 20 random-seeded trials each, for both `u8`
and `u16` - all passed bit-exact, confirming the shuffle-mask lane arithmetic is correct before
spending time on the full bench.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`, `colorspace` mode (`decode_pixel_data()` +
`to_dynamic_image(0)`, hashing the output), same fixtures as candidates 002/031:
`testdata/ybr_big.dcm` (2,000,000 pixels, real tiled photographic content) and
`testdata/ybr_random.dcm` (same pixel count, deterministic xorshift PRNG content, needed to show
the rounding-boundary difference since the tiled real-content fixture is bit-exact between builds).
Single-core numbers use a `--no-default-features` build (no `rayon` linked, confirmed via
`nm <bin> | grep -c rayon_core` = 0) under `taskset -c 0`/`taskset -c 3`; multi-core numbers use
the default (`rayon`-enabled) build, unpinned.

| Variant | single-core p50 (no-rayon build, `taskset`) | multi-core p50 (default rayon build, unpinned) | notes |
|---|---|---|---|
| baseline (f32+floor, per-pixel `chunks_mut(3)`) | 15.679 ms / 15.730 ms (2 rounds) | 5.152 ms / 5.050 ms (2 rounds) | same as candidate 002/031's baseline shape |
| candidate 002 (fixed-point, scalar, per-pixel `chunks_mut(3)`) | ~8.0-9.3 ms (from 002/031's own RESULT.md, not re-run here) | ~4.9-5.9 ms (from 002/031's own RESULT.md, not re-run here) | reference point only, not re-measured in this session |
| this candidate (fixed-point, SSE4.1 SIMD, batched rayon chunks) | 4.434 ms / 4.462 ms (2 rounds) | 3.626 ms / 3.613 ms (2 rounds) | 4 px/iter (u8 kernel exercised by this fixture) |

`just test` (this candidate's own justfile, run for real from a fresh clone) reproduced closely
consistent numbers on a separate run: single-core -71.9% (9/9 pairs), multi-core -28.8% (9/9
pairs) - see that run's own transcript, not re-tabulated above since it's the same comparison.

## Result

**Single-core: -71.6% and -71.7% across two independent `paired.sh` rounds (RUNS=9), 9/9 pairs
faster both times, plus a third confirming run via `just test` at -71.9% (9/9) - highly
consistent.** **Multi-core (unpinned, default rayon-enabled build): -29.6% and -28.5% across two
rounds (9/9 pairs faster both times), plus a third confirming run via `just test` at -28.8%
(9/9) - also highly consistent.** This is a substantially larger multi-core win than candidate
002's own -17.5%/-18.7% (fixed-point arithmetic alone, still one pixel per rayon task) - consistent
with this candidate's mechanism being SIMD *plus* a ~4096x larger rayon chunk size, both
contributing. Time only was measured (no peak-RSS/allocation-count claim - this change doesn't
plausibly affect either, it's a pure CPU-arithmetic rewrite of an in-place buffer transform).

## Does it change behavior/output?

- **Bit-exact?** Same characterization as candidate 002, since the SIMD kernel computes the exact
  same fixed-point arithmetic (confirmed identical via the temporary unit test described above,
  and by the fact that both this candidate's and 002's random-fixture diffs match exactly:
  1,630 of 6,000,000 bytes differ, always by exactly 1 (max diff = 1 LSB), i.e. 0.0272% of
  output bytes). **Yes, bit-exact vs. the float baseline** on `testdata/ybr_big.dcm` (real tiled
  photographic content, `sha256sum` match) - **no** on `testdata/ybr_random.dcm` (synthetic random
  content), same 1-LSB/0.027% characterization root-caused by candidate 002 (each fixed-point
  coefficient is rounded to the nearest 1/65536, less precise than an `f32`'s 23-bit mantissa, so
  a small fraction of inputs land on a rounding boundary the two representations resolve
  differently).
- **Is this a breaking change?** No public API/type signature changes - both functions are
  private (`fn`, not `pub fn`); the new helper functions (`_scalar`/`_sse41`/`_slice`) are also
  all private, module-internal implementation detail. The only externally-observable effect is the
  same up-to-1-LSB output difference already present in (and unchanged since) candidate 002, for
  pixel values on a rounding boundary.
- Project test suite (`cargo test --workspace --release`): **537 passed / 0 failed / 6 ignored**
  before (clean checkout, no patch), **539 passed / 0 failed / 6 ignored** after (patched - the
  extra 2 passing tests are the temporary SIMD-vs-scalar correctness tests described above, which
  are NOT part of `patch.diff` and were removed before capturing it). No new failures either way.

## Why the result looks the way it does

The single-core win is large (-71.6% to -71.9%, vs. candidate 002's own -46.9%/-50.8%/-64.3% for
fixed-point-alone) because the SIMD kernel does the multiply/shift/clamp arithmetic for 4 pixels
per set of vector instructions instead of 4 separate scalar sequences, on top of already having
removed the `floorf()` libm calls (candidate 002's win). Confirmed via `objdump -d -C` on the
patched binary: `convert_colorspace_u8_sse41` contains real packed instructions operating on
`%xmm`/`%ymm`-width data (`pshufb`, `paddd`, `movdqu` visible in the disassembly), unlike candidate
031's finding that neither the float nor fixed-point-scalar version ever touches a packed SIMD
register for cross-pixel work.

The multi-core win (-28.5% to -29.6%) is larger than candidate 002's own multi-core win
(-17.5%/-18.7%) for two combined reasons that can't be cleanly separated without a third,
narrower patch nobody asked for: (a) the same per-pixel arithmetic speedup as the single-core
case, and (b) `par_chunks_mut(3 * 4096)` instead of `par_chunks_mut(3)` means far fewer, far
larger rayon tasks - directly addressing the per-call dispatch overhead that both candidates
002's and 031's own `RESULT.md` identified as the reason the multi-core win was much smaller than
the single-core win ("rayon's fixed per-call cost is often the larger term next to cheap
per-element work"). Since dispatching one pixel per rayon task would make the SIMD rewrite
pointless anyway (there'd only ever be one pixel available per call), enlarging the batch size was
not an optional add-on to this candidate, it's a precondition for the SIMD rewrite to do anything
under rayon at all.

The `u16` kernel processes only 2 pixels/iteration (half the `u8` kernel's 4) because the
intermediate accumulator needs 64 bits (`y << 16` alone can reach ~4.3e9 for 16-bit samples), and
SSE's `_mm_mul_epi32` only produces 64-bit products from 32-bit inputs in 2 lanes at a time (no
signed 64-bit arithmetic right shift exists below AVX512VL, so the final shift+clamp for `u16` is
done in scalar code after extracting the 64-bit accumulators - only the multiply-add is
vectorized). This was validated for correctness (see above) but **not benchmarked end-to-end**
through the real pipeline - no real or synthetic `dicom-test-files`-based fixture with a 16-bit
YBR_FULL/YBR_FULL_422 sample exists in this harness (`gen-ybr`/`gen-ybr-random` only tile/randomize
an 8-bit source), and building one was out of scope for this session - see "What's not done"
below.

## What's not done / open questions

- **The `u16` SIMD kernel's real-pipeline speedup was not measured** - only its bit-exactness
  against the scalar fixed-point path (temporary unit test, described above). A future session
  wanting this number would need a synthetic 16-bit-sample YBR_FULL fixture (extending
  `gen-ybr`/`gen-ybr-random` with an optional bits-allocated parameter, the same way
  `gen-planar-rgb` already has one for a different code path - candidate 034).
- **AVX2 was not attempted.** An AVX2 kernel could double the `u8` kernel's pixels/iteration (8
  instead of 4, using 256-bit registers) and would help the `u16` kernel more (since AVX2's wider
  `_mm256_mul_epi32` still only produces 4 64-bit lanes per call, but from 8 32-bit inputs instead
  of 4, doubling `u16` throughput too) - left as a follow-up idea since SSE4.1 is a safe universal
  baseline for any x86-64 CPI, while AVX2 needs an additional runtime feature-detection branch and
  a separate kernel to fall back from.
- **The store side of the `u8` kernel is still scalar** (`_mm_extract_epi32` x12, plain indexed
  byte stores) - a `pshufb`-based interleave-and-pack of the three output registers directly into
  12 output bytes (avoiding the 12 extracts) was considered but not implemented, given the time
  budget for this candidate; plausibly a further, smaller win on top of what's measured here.
- Not independently re-verified: whether a different rayon chunk size (something between 3 bytes
  and 3*4096 pixels) would perform even better than the specific `4096` picked here - not swept,
  just chosen as "clearly large enough to amortize per-task overhead" without a parameter search.
