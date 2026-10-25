# HEIC YCbCr->RGB color conversion AVX2

`crates/zune-heic/src/hevc_decoder/utils.rs::ycbcr_to_rgb_inner_16_scalar` (called from
`raw_frame.rs:142` and `:180` on every decoded HEIC pixel block) was pure scalar despite using
the exact same fixed-point YCbCr->RGB coefficients and per-16-pixel loop shape as
`zune-jpeg/src/color_convert/scalar.rs`, whose AVX2/NEON kernels were added in this repo's own
recent history (f51cff38, ea8efb73). Ported the AVX2 kernel structure (`color_convert/avx.rs`'s
`ycbcr_to_rgb_avx2_1`/`ycbcr_to_rgb_baseline_no_clamp`) into a new
`crates/zune-heic/src/hevc_decoder/utils/avx2.rs`, dispatched at runtime
(`std::is_x86_feature_detected!("avx2")`) from a new `ycbcr_to_rgb_inner_16` wrapper that
`raw_frame.rs` now calls instead of the scalar function directly (the scalar function itself is
kept, unchanged, as the fallback).

Toolchain: `rustc 1.98.1 (48a229cea 2026-09-01)`, `cargo 1.98.1 (797e8a9bc 2026-08-05)`.

## What was tried

Only one variant (AVX2, runtime-dispatched via `is_x86_feature_detected!`, x86_64 only - no NEON
kernel was written in this pass, see "What's not done" below). Bench: a standalone
`examples/bench_decode.rs` driver (copied into the cloned tree by `justfile`, not part of
`patch.diff`) that runs `HeifDecoder::decode()` end-to-end (software/non-videotoolbox path
forced explicitly) against `crates/zune-heic/fuzz-samples/IMG_4862.HEIC` (2.75 MiB, the only
real HEIC sample already in the repo), hashing (fnv1a64) the final RGB output every iteration.

Full end-to-end HEIC decode (not just the color-conversion function in isolation - no existing
harness isolates it further, and the color-conversion step depends on the full HEVC
tile/plane state, so end-to-end is what's actually measurable):

| Variant | single-core p50 (`taskset -c2`, 20 iters, 2 runs each) | multi-core p50 (unpinned, 20 iters, 2 runs each) | notes |
|---|---|---|---|
| baseline | 482.999 ms, 469.790 ms | 150.731 ms, 151.698 ms | scalar `ycbcr_to_rgb_inner_16_scalar` |
| patched (AVX2) | 487.148 ms, 478.507 ms | 151.816 ms, 153.507 ms | direct (non-paired) runs, see paired comparison below for the number that actually carries the conclusion |

Direct (non-paired) runs above are noisy in both directions (see "Result"); the paired
alternating comparison (`tools/paired.sh`, `RUNS=9`, `-- <file> 6`) is what actually
distinguishes signal from noise here:

- Single-core (`taskset -c2`): baseline median 485.223 ms -> patched median 478.101 ms
  (**-1.5%**), patched faster in **8 of 9** pairs -> `paired.sh`'s own "consistent" threshold
  (>= n-1).
- Multi-core (unpinned): baseline median 150.152 ms -> patched median 149.154 ms (-0.7%),
  patched faster in only **6 of 9** pairs -> `paired.sh`'s own "NOT consistent" verdict (noise,
  not signal).

## Result

**Time.** A real but small single-core win (-1.5% end-to-end decode time, paired-confirmed,
8/9 pairs), not detectable above the noise floor on the multi-core (unpinned, rayon-parallel
tile decode) path for this test image with this measurement's pair count. `perf record`/`perf
report` (`--no-children`, single-threaded run, 10 iters) attributes the color-conversion
call chain (`ycbcr_to_rgb_inner_16*` + the `write_rgb_420` driver frame) **7.17%** of total
decode cycles before the patch (5.75% `ycbcr_to_rgb_inner_16_scalar` + 1.42%
`write_rgb_420`) vs **5.94%** after (2.28% `ycbcr_to_rgb_inner_16_avx2` + 0.36% the
`ycbcr_to_rgb_inner_16` dispatch wrapper + 3.30% `write_rgb_420`, which rose in relative share
since the kernel itself got faster) - consistent in direction and rough magnitude with the
measured -1.5% end-to-end delta. No memory/allocation-count claim made (this patch doesn't
change allocation patterns - same output buffer, same iteration structure).

## Does it change behavior/output?

- **Bit-exact?** yes. fnv1a64 hash of the full decoded RGB buffer (`c56cfb9cd2725d0b`) is
  identical baseline vs patched, every iteration, for `IMG_4862.HEIC`.
- **Is this a breaking change?** no - same public API, same output.
- Project test suite (`cargo test --release -p zune-heic`, the only crate whose source changed):
  **42 passed / 0 failed**, baseline and patched, identical set.

## Why the result looks the way it does

Color conversion is a real but minority slice of total HEIC decode cost - `perf` attributes it
~5-7% of cycles for this sample, with CABAC residual-block entropy decode
(`decode_residual_block` and friends, ~26% alone) dominating instead. So even though the AVX2
kernel itself is substantially faster than the scalar one in isolation (the kernel's own sample
share roughly halved, 5.75% -> 2.28%), the end-to-end win is bounded by Amdahl's law to a small
single-digit-percent number, and on the unpinned multi-core path (where several HEVC tiles
decode in parallel via rayon) that small a delta falls inside this measurement's own run-to-run
noise (~3% swings observed between repeat baseline runs).

## What's not done / open questions

- No NEON (aarch64) kernel was written in this pass - only the x86_64 AVX2 path. The candidate
  description's own template (`zune-jpeg/src/color_convert/neon64.rs`) exists and could be
  ported the same way; not done here purely for time-budget reasons within this single-candidate
  session, not because of any known blocker.
- No SSE2/SSE4.1 fallback tier was added for x86_64 CPUs without AVX2 (rare in practice in 2026,
  and the scalar fallback already covers that case correctly) - `zune-heic`'s own existing IDCT
  SIMD (`hevc_decoder/idct/x86_64.rs`) uses always-available SSE2 with no runtime dispatch at
  all; this candidate instead followed the JPEG color-convert template's own runtime-dispatched
  AVX2-or-scalar shape per the candidate brief. A maintainer picking this up may want to align
  one way or the other for consistency with the rest of the crate.
- Only one real-world test image was available in the repo (`fuzz-samples/IMG_4862.HEIC`) - no
  monochrome (`is_monochrome`) HEIC sample was exercised, though the AVX2 kernel's own inputs
  (`y`/`cb`/`cr` arrays) are identical in shape/semantics regardless of how the caller filled
  them (monochrome fills `cb`/`cr` with a constant 128), so no separate code path risk is
  expected there - just not independently verified against a real monochrome sample.
