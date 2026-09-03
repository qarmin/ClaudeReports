# Candidate 004's `interleave::<T>()` rayon guard, measured for the `u16` instantiation

Candidate 004's rayon-threshold guard for `interleave()` (`pixeldata/src/lib.rs`, generic over
`T: Copy + Default + Send + Sync`) was only ever measured against the `u8` instantiation (8-bit
`PlanarConfiguration::PixelFirst` RGB images). The `u16` instantiation is used for
`bits_allocated == 16` `PlanarConfiguration::PixelFirst` images (`to_dynamic_image_with_options`,
`pixeldata/src/lib.rs:867`) and shares the exact same generic function body, but no 16-bit
`PixelFirst` fixture existed to measure it - flagged as an open gap in `IDEAS.md`.

## What was tried

Extended the harness's `gen-planar-rgb` mode with an optional `bits-allocated` argument
(candidate 034; harness-only change, not part of `patch.diff`): when `16`, builds a
`BitsAllocated=16`/`OW` pixel data fixture instead of the existing 8-bit/`OB` one, otherwise
identical (same PRNG fill, same `PlanarConfiguration=1`/`RGB` scaffold). Source scaffold:
`pydicom/SC_rgb_16bit.dcm` (Columns=100, matching candidate 004's own 8-bit fixtures' column
count so the same target-rows values give the same pixel counts: 2.5M/10M/20M).

Two binaries (default features = rayon on; `--no-default-features` = rayon off, confirmed via
`nm | grep -c rayon_core` = 0), one baseline, one with candidate 004's `patch.diff` applied,
compared via `tools/paired.sh` (`RUNS=9`) using `colorspace-fast` (exercises
`to_dynamic_image_with_options`'s 16-bit RGB branch, which calls `interleave::<u16>()` for
`PlanarConfiguration::PixelFirst`).

| Variant | 2.5M px (below threshold), multi-core | 2.5M px, round 2 (re-check) | 10M px, multi-core | 20M px, multi-core | 10M px, single-core (no-rayon build) |
|---|---|---|---|---|---|
| baseline | 10.61 ms p50 | 10.60 ms p50 | 35.56 ms p50 | 45.02 ms p50 | 35.51 ms p50 |
| patched | 10.83 ms p50 | 10.87 ms p50 | 31.74 ms p50 | 45.03 ms p50 | 35.74 ms p50 |

## Result

- **Below the 3,000,000px threshold (2.5M px): a real, small, reproducible REGRESSION** - +2.1%
  (1/9) round 1, +2.6% (1/9) round 2, both consistent by this workspace's paired-run rule and
  agreeing in direction. **This differs from candidate 004's own `u8` finding at the same pixel
  count, which found no consistent change below threshold.** The guard's own `rayon_worth_it()`
  check (a `OnceLock`-cached `available_parallelism()` call) is `T`-independent and should cost
  the same regardless of instantiation - the regression's exact source (larger monomorphized code
  for `u16`? different codegen at the `if` boundary?) was not investigated further, see "Why the
  result looks the way it does" below.
- **At 10,000,000 pixels: a real, reproducible -10.7% multi-core win, 9/9 paired wins** - larger
  in magnitude than candidate 004's own `u8` result at the same pixel count (-8.0%), consistent
  with `u16`'s per-element copy being twice the byte width (more bandwidth to save by avoiding the
  sequential path's cache-unfriendly zip-of-three-iterators pattern).
- **At 20,000,000 pixels: no consistent change** (+0.0%, 4/9) - matches candidate 004's own `u8`
  finding of a narrow win window rather than a scaling trend, now confirmed for `u16` too.
- **Single-core (`--no-default-features`, no rayon linked): no consistent change** (+0.6%, 3/9) at
  10M pixels - expected, since the entire threshold/guard block compiles out without the `rayon`
  feature, same as candidate 004's own `u8` single-core result.

Time only was measured - no peak-RSS/allocation follow-up (this fix's mechanism, per candidate
004's own analysis, is about avoiding a pre-zeroed buffer's extra write pass and rayon dispatch
overhead, not reducing total bytes allocated).

## Does it change behavior/output?

- **Bit-exact?** Yes - verified with a real `sha256sum` of `dump-image` output (not just the
  length-only `colorspace-fast` checksum) on the 10M-pixel fixture: identical
  `f9665d01475c5b8a1d3af5b21c6ecc96fe38f21f399b0af4e274cecffa851f9b` baseline vs. patched. The
  `colorspace-fast` checksums also matched on every fixture size tested (`0x13b8700` at 10M, etc).
- **Is this a breaking change?** No - `interleave` is a private function of the `pixeldata` crate,
  unchanged from candidate 004's own analysis.
- Project test suite: `cargo test --release -p dicom-pixeldata`: **39 + 1 + 1 + 10 = 51 passed /
  0 failed** (patch applied), matching candidate 004's own recorded count for this crate exactly.
  Reverted-tree (baseline) count is identical.

## Why the result looks the way it does

The 10M/20M shape (a real win in a narrow window, not a scaling trend) reproduces candidate 004's
own `u8` finding almost exactly, just scaled up (~-10.7% vs. -8.0%) - consistent with `u16`'s
wider element type making the avoided pre-zeroing write pass and cache-unfriendly sequential zip
proportionally more expensive to avoid. The below-threshold regression is the one genuinely new
finding here, and is NOT explained by this candidate - a plausible but unconfirmed guess is that
`u16`'s version of `rayon_worth_it()`/the `if` check sits on a slightly different instruction-cache
line boundary than `u8`'s (both instantiations are monomorphized separately, so their code layout
differs), the same "small, real, mechanism-resistant regression" pattern this workspace has seen
before for candidates 006/027 (see `projects/dicom-rs.md`) - never confirmed via disassembly in
those cases either, and not attempted here either given the bounded scope of this candidate.

## What's not done / open questions

- The below-threshold `u16` regression's root cause is unconfirmed (see above) - an
  `objdump -d -C` comparison of the two instantiations' below-threshold code path would be the
  natural next step, same open item as candidates 006/027.
- The exact 10M-to-20M crossover (where the win stops) was not narrowed for `u16`, same gap as
  candidate 004 left open for `u8`.
- Peak-RSS not measured (see "Result" above for why this wasn't expected to matter).
