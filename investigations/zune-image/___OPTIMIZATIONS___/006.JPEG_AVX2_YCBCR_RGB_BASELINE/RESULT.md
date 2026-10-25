# JPEG AVX2 YCbCr->RGB baseline color conversion: does RGB get a worse kernel than BGR?

`crates/zune-jpeg/src/color_convert/avx.rs`, `ycbcr_to_rgb_avx2_1::<false>` (the plain-RGB AVX2
YCbCr->RGB color converter) plus its driver call site `color_convert_ycbcr` in `worker.rs`. The
lead (from direct `perf` profiling of `speed_bench.jpg`) was that `ycbcr_to_rgb_avx2_1::<false>`
alone accounts for ~12.50% of total decode cycles, plus ~2.00% in the driver, and asked whether
the plain-RGB (`<false>`) path received the same SIMD treatment as the BGR/BGRA variant that was
vectorized in commit `f51cff38` ("perf(jpeg): vectorize BGR and BGRA conversion"), or is a
less-optimized sibling.

rustc 1.98.1 (48a229cea 2026-09-01), cargo 1.98.1 (797e8a9bc 2026-08-05).

## What was tried

This candidate did not end up applying a source patch - see "Why the result looks the way it
does" for why, and "What was tried" below for the investigation that led there.

1. **Read the code.** `ycbcr_to_rgb_avx2` (RGB) and `ycbcr_to_bgr_avx2` (BGR) both call the exact
   same private function, `ycbcr_to_rgb_avx2_1::<const BGR: bool>`, added in commit `f51cff38`.
   The only difference between the two colorspaces is `if BGR { core::mem::swap(&mut r, &mut b); }`
   right after the shared `ycbcr_to_rgb_baseline_no_clamp` computation - the entire shuffle
   /permute/blend/store pipeline that follows is byte-for-byte identical code for both. Same
   structure for RGBA/BGRA via `ycbcr_to_rgba_unsafe::<const BGRA: bool>`. So the premise that RGB
   might be a "less-optimized sibling" of BGR is answered directly by the source: **they are the
   same kernel**, not two independently-maintained code paths.

2. **Re-profile to confirm/refine the cycle share**, since the original 12.50%/2.00% figures came
   from a separate profiling session not reproduced here. Built a throwaway `profile_only.rs`
   example (no per-iteration hashing, to avoid the harness's own `fnv1a64` hash dominating the
   profile - it does, confirmed: with the hash included, `bench_decode::main` itself samples at
   ~50% because it hashes the full RGB output every iteration) in a clean worktree pinned to this
   workspace's `base_commit`, decoding `test-images/jpeg/benchmarks/speed_bench.jpg` (default
   `DecoderOptions`, whose `out_colorspace` default is `ColorSpace::RGB`) 200 times,
   `perf record -F 999 --call-graph dwarf` under `taskset -c 2`:
   - `ycbcr_to_rgb_avx2_1::<false>`: **14.30%** self (matches the lead's 12.50%, same order of
     magnitude - difference plausibly explained by different `perf` sampling/call-graph settings
     or a different input file variant, not a discrepancy worth chasing further).
   - `worker::color_convert_ycbcr` (the driver, the chunking/bounds-check loop around the call):
     **2.45%** self (matches the lead's 2.00%).
   - Combined: ~16.75% of total decode cycles in this run.
   - `perf annotate` on `ycbcr_to_rgb_avx2_1::<false>` shows LLVM narrowed the whole 256-bit
     shuffle/blend pipeline described in the source down to 128-bit (`xmm`) instructions for the
     back half of the function (3x `vextracti128`, 5x `vpshufb`, 3x `vpor`, 6x `vpblendvb`, 3x
     narrow `vmovdqu` stores) - it already found and exploited the fact that half of each 256-bit
     register is dead (zero) after the `_mm256_packus_epi16(x, zero)` + `permute4x64` step that
     packs 16 valid pixels into one 128-bit lane. Full annotated output saved in
     `annotate_ycbcr_to_rgb_avx2_1.txt` in this folder.

3. **Checked whether an alternative combine strategy could reduce instruction count.** The classic
   SIMD "3-plane SoA -> interleaved AoS" trick (shuffle each of R/G/B with a `pshufb` table that
   zeros unselected byte positions, then `vpor`-combine, instead of `vpblendv`-selecting) was
   considered on paper: it needs a *separate* zeroing shuffle per output chunk per channel (9
   shuffles total for 3 output chunks x 3 channels) versus the current 5 shuffles (3 channel
   shuffles reused across all 3 output chunks via 2 recurring blend-mask constants), while still
   needing the same 6 combine ops (2 per output chunk) either way - `vpor` instead of `vpblendv`.
   On paper this trades more shuffles for marginally cheaper combine ops, with no clear net win,
   and AVX2 has no single cross-128-lane byte permute (`vpermb` needs AVX512VBMI) that would let a
   3-byte-per-pixel interleave collapse further - so hand-redesigning this kernel wasn't pursued;
   see "What's not done" below.

4. **Checked test coverage while reading the code** (not a performance finding, noted for
   completeness): `crates/zune-jpeg/src/color_convert.rs`'s own `tests` module has
   `avx2_bgr_and_bgra_match_scalar` (added alongside the BGR/BGRA vectorization in `f51cff38`) but
   **no equivalent `avx2_rgb_and_rgba_match_scalar`** test for the plain RGB/RGBA AVX2 paths -
   despite RGB being `DecoderOptions`'s *default* output colorspace. Wrote a throwaway test
   (`assert_converter_matches_scalar(ycbcr_to_rgb_avx2, ycbcr_to_rgb_inner_16_scalar::<false>, ...)`
   and the RGBA equivalent) in the worktree and ran it: **it passes** - the AVX2 RGB/RGBA output is
   bit-exact against the scalar reference across the existing test's input vectors (all-zero,
   all-255, all-128, a ramp, and 256 LCG-random vectors), so this is a test-coverage gap, not a
   correctness bug. Not carried into `patch.diff` (reverted before capturing it, per this
   investigation being measurement-only) - noted as a backlog idea instead.

## Result

No patch was measured - this candidate is a code-reading + profiling investigation with a
conclusive negative answer to the question it was asked ("is RGB a less-optimized sibling of
BGR?" - no, they share one kernel), not a timing comparison. No before/after numbers apply.

## Does it change behavior/output?

N/A - measurement/investigation only, no patch applied to real source. `ycbcr_to_rgb_avx2_1` and
`ycbcr_to_rgba_unsafe` were not modified.

- Project test suite: not run before/after for this candidate specifically, since no source was
  changed (per `SKILL.md`'s "Correctness" section, the before/after run is "mandatory... other
  than a pure investigation with no patch" - this is that case). The one-off correctness check
  actually performed (step 4 above: a throwaway `assert_converter_matches_scalar` test for
  RGB/RGBA AVX2 vs. scalar) passed and was not committed.

## Why the result looks the way it does

`ycbcr_to_rgb_avx2_1::<false>` (RGB) and `ycbcr_to_rgb_avx2_1::<true>` (BGR) are literally the same
function with a two-register swap - RGB already received the identical optimization treatment as
BGR in `f51cff38`; there is no separate, less-optimized RGB kernel to fix. The ~12-17% of total
decode cycles this function accounts for is inherent to the problem it solves: AVX2 can pack 4
bytes/pixel (RGBA/BGRA) cheaply because 4 divides evenly into SIMD lanes (that sibling function,
`ycbcr_to_rgba_unsafe`, only needs unpack + 2 `vpblendd`-style blends + 2 full-width 32-byte
stores - no 128-bit narrowing needed), but 3 bytes/pixel (RGB/BGR) does not divide evenly into any
power-of-two SIMD width, forcing the permute/shuffle/`vpblendv` machinery this profile is pointing
at. AVX2 has no single instruction that solves "interleave 3 planes at byte granularity across
lanes" the way `vpermb` (AVX512VBMI) would - so the function's cost is best explained as "the
AVX2-achievable floor for 3-byte-per-pixel packing," not a missed optimization. A deliberately
different combine strategy (`pshufb`-zero + `vpor` instead of nested `vpblendv`) was worked out on
paper and doesn't obviously reduce total instruction count for the reasons in step 3 above, so
implementing and measuring it was judged not worth the correctness risk on a codec's default output
path for this candidate's scope - flagged as an open idea below instead of attempted blind.

## What's not done / open questions

- The `pshufb`-zero + `vpor` alternative combine strategy (step 3) was reasoned about but not
  implemented/measured - it's a plausible-but-unproven idea, not a ruled-out one. A future
  candidate with more time budget could implement both `ycbcr_to_rgb_avx2_1` variants this way
  behind a feature flag and measure against the existing `vpblendv` version using this candidate's
  `bench/bench_decode.rs`.
- A larger structural idea not attempted here (bigger scope than one candidate): processing 32
  pixels per AVX2 call instead of 16 would let the packus/permute step fill both 128-bit lanes
  with live data instead of zero-padding one of them, which is the root cause of the "compiler
  narrows to 128-bit ops" behavior observed in the annotated assembly - but that needs restructuring
  the `[i16; 16]` fixed-size interface used throughout `color_convert.rs`/`worker.rs`/`idct/avx2.rs`,
  well beyond this candidate's scope.
- The missing `avx2_rgb_and_rgba_match_scalar` unit test (step 4) is a real, reproducible coverage
  gap (confirmed passing when added, so not currently masking a bug) - worth adding as a small,
  independent, non-performance fix; not done here since it's out of this candidate's scope
  (performance only) and not a confirmed bug for `___BUGHUNT___`.
- Not tried on NEON (`crates/zune-jpeg/src/color_convert/neon64.rs`) - this investigation was
  scoped to the AVX2 lead as given.
