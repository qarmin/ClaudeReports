# AVX2 horizontal convolution kernel for planar (1-channel) u8

`src/factory/plane_u8.rs`'s x86 horizontal-convolution dispatch topped out at SSE4.1, even though
the sibling RGB/RGBA u8 horizontal passes (`src/factory/rgb_u8.rs`) already have both SSE4.1 and
AVX2 tiers, and plane's own vertical convolution already reuses the shared AVX2
`convolve_vertical_avx_row`. Callgrind profiling of `resize_plane` (Lanczos 3, PreferQuality)
showed `sse::plane_u8::convolve_horizontal_plane_sse_rows_4_u8_impl` at 15.49% of total Ir
(~754M) - the largest true per-iteration hotspot found in that scene, excluding one-time PNG-decode
setup noise (see `___OPTIMIZATIONS___/IDEAS.md`'s now-removed backlog entry for the exact
callgrind numbers). This candidate adds `src/avx2/plane_u8.rs`, widening the existing SSE4.1
zero-extend + `madd_epi16` accumulation trick from 128-bit (8 taps/iteration) to 256-bit (16
taps/iteration), and wires it into `plane_u8.rs`'s dispatch as a new AVX2 tier above SSE4.1, mirroring
the tiering already used for RGB/RGBA. No `avxvnni`/dot-product shape is involved (candidate 001
found that shape regresses on this Zen4 box) - this is a plain `vpmaddwd`-based widening, the same
approach candidate 002 already validated for CbCr u16.

## What was tried

Bench: `app`'s "Pic scale Plane8(Quality): Lanczos 3" criterion case (`resize_plane`, `harness =
false`), the exact scene profiled above (Lanczos3, `WorkloadStrategy::PreferQuality`,
`ThreadingPolicy::Single`, 4928x3279 nasa asset resized to 1/4). `app/Cargo.toml`'s default
`pic-scale` features don't include `avx`/`sse`, so this was temporarily patched in for measurement
only (not part of `patch.diff`, applied by this candidate's own `justfile`).

| Variant | single-core p50 (`taskset -c 3`, 2 runs) | multi-core p50 (unpinned, 2 runs) | notes |
|---|---|---|---|
| baseline (SSE4.1 only) | 3.671 ms / 7.506 ms | 5.074 ms / 3.887 ms | same binary A both repeats; ms swing between repeats is host-level noise (see below), not drift within a repeat |
| AVX2 (this patch) | 3.092 ms / 6.401 ms | 4.369 ms / 3.110 ms | same binary B both repeats |
| delta | -15.8% (8/9 pairs), -14.7% (8/9 pairs) | -13.9% (8/9 pairs), -20.0% (9/9 pairs) | all four `paired.sh` runs report "verdict: consistent" |

Both binaries were built from the same `cargo bench -p app --bench resize_plane --no-run` command
(baseline vs. patched source), copied out, and compared with `tools/paired.sh` (`RUNS=9`,
alternating A/B). `uptime` load stayed under the 24-core count throughout, but this box's absolute
timings were visibly bimodal run-to-repeat (e.g. baseline single-core median 3.671ms in one repeat,
7.506ms in the next) - a host-level noise mode already noted for `resize_cbcr` in
`___OPTIMIZATIONS___/README.md`, now also observed for `resize_plane`. Despite that swing, the
*paired* A/B ratio inside each repeat stayed consistent (-13.9% to -20.0%), which is what
`paired.sh`'s alternating design is meant to isolate.

## Result

Real, consistent speedup: roughly **-14% to -20%** on the profiled "Pic scale Plane8(Quality):
Lanczos 3" case, both single-core (pinned) and unpinned, across two independent repeats each (4
total `paired.sh` runs, all "verdict: consistent", 8/9 or 9/9 pairs in B's favor every time).

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone `bit_exact_check/` crate (11 size/scale combinations x
  PreferSpeed/PreferQuality, sweeping tap counts through 1/2/3/4/5/7/8/9/16/17+ per output pixel,
  plus a height-not-a-multiple-of-4 case to exercise the single-row remainder path) hashes the
  resampled output; baseline and patched builds produce the identical
  `FINAL_HASH=11635962358689872026`. This is expected, not just observed: the AVX2 kernel sums the
  same i16 pixel*weight products as the SSE4.1/scalar paths, only reordering which partial sums are
  added together first (16-wide `madd_epi16` groups instead of 8-wide) - integer addition has no
  rounding error, and the one rounding step (`ROUNDING_CONST` + arithmetic shift by `PRECISION`)
  happens once at the very end after all partial sums are combined, so the reduction-tree shape
  cannot change the final result.
- **Is this a breaking change?** No - internal dispatch only, no public API change.
- Project test suite: not run separately: this candidate's own `bit_exact_check` (see above) is
  the correctness check for this code path; no `cargo test` run was done for the wider suite (no
  behavior change was expected or found outside `plan_planar_resampling` on x86_64+avx2).

## Why the result looks the way it does

Same shape as candidate 002's CbCr AVX2 win: the SSE4.1 kernel already reduces every output pixel
to a horizontal-sum-of-products (no interleaved-channel de-shuffling like RGB/RGBA need), so
doubling the SIMD width doubles taps-per-instruction with no extra shuffle overhead - `avx2::
plane_u8::avx_accumulate_16_horiz` is a straight `_mm256_cvtepu8_epi16` + `_mm256_madd_epi16` per
16 taps, using the same "zero-extend, then single `madd_epi16`" trick as the SSE version, just at
double width. `resize_plane`'s Lanczos3/PreferQuality filters have long enough per-pixel tap runs
(callgrind showed this as the dominant per-iteration cost, ~754M Ir) that the 16-wide main loop
fires often enough to matter; the 8/4/1-tap tail tiers exist only to handle the remainder and were
carried over unchanged from the SSE version (reusing `crate::sse::_mm_muladd_wide_epi16`).

## What's not done / open questions

- Only the "rows-4" and "single-row" u8 horizontal entry points were added; no AVX512 tier (out of
  scope here - see the still-open, lower-confidence AVX512 idea for RGB/plane u8 in `IDEAS.md`,
  informed by candidate 001's regression finding for the RGBA u8 dot-product shape on this Zen4
  box).
- Not re-measured on non-x86_64 or on hardware without AVX2 (dispatch already falls back to SSE4.1/
  scalar correctly there; no behavior change expected, not specifically re-verified this session).
- `cargo test`/snapshot suite for the wider crate wasn't run before/after - only the
  `bit_exact_check` hash comparison targeting this exact code path.
