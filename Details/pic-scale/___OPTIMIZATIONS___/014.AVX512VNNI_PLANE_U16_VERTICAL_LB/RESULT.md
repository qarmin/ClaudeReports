# AVX-512VNNI dispatch branch for Plane/CbCr/RGB/RGBA u16 vertical (column) convolution (bit_depth<=12)

`src/factory/plane_u16.rs`'s `default_u16_column_plan` (shared by `plane_u16.rs`, `cbcr16.rs`,
`rgb_u16.rs`, `rgba_u16.rs` via `default_u16_column_plan::<CN>`) only ever activates
`src/avx2/vertical_u16_lb.rs`'s dot-product column kernel in its VEX (`avxvnni`) form when
`is_x86_feature_detected!("avxvnni")` is true - flagged in `IDEAS.md` as an untested, structurally
different kernel shape from the horizontal dot-product kernels candidates 001/012/013 already
measured (all null/regression on this box). `avxvnni` is the narrow VEX-encoded extension found on
Alder Lake+/Sierra Forest; this dev/bench box (AMD Ryzen 9 7900, Zen4) has `avx512vnni`+`avx512vl`
instead (confirmed via a small `is_x86_feature_detected!` probe binary), so that branch never
fires here as shipped - the AVX2 fallback (`_mm256_add_epi32(a, _mm256_madd_epi16(b, c))` instead
of the fused `vpdpwssd_avx`) is what actually runs today.

Reading `src/avx2/vertical_u16_lb.rs` first (per the backlog item's own instruction) confirmed the
kernel shape difference: unlike the horizontal kernels' `_mm_shuffle_epi8`/`_mm256_shuffle_epi8`
lane-reformatting before every dot product, the column kernel loads a contiguous run of pixels
straight down a column and does *no* shuffle at all - it just widens each `u16` lane
(`_mm256_unpacklo_epi16`/`_mm256_unpackhi_epi16` against a zero vector) and accumulates. Same
`const HAS_DOT: bool>`-selected `_mm256_dot16_avx_epi32`/`_mm_dot16_avx_epi32` helpers from
`src/avx2/utils.rs` as before, so still a "swap the final accumulate instruction, everything else
identical" comparison - but with proportionally *less* per-tap overhead surrounding that
accumulate than the horizontal kernels had, which is the mechanism this candidate's result below
is consistent with.

## What was tried

Added an `avx512vnni`+`avx512vl`-gated counterpart of the existing `avxvnni`-gated column kernel:
new `_mm256_dot16_avx512_epi32`/`_mm_dot16_avx512_epi32` helpers in `src/avx2/utils.rs` (EVEX
`vpdpwssd_epi32`/`vpdpwssd_epi32`, mirroring candidate 012's helpers of the same name/shape - not
reused directly since that candidate's patch was reverted at the end of its own session, per this
workspace's one-patch-per-candidate rule), and byte-for-byte copies of
`convolve_column_lb_32`/`_16`/`_8`/`convolve_column_lb_avx_u16_impl` in
`src/avx2/vertical_u16_lb.rs` (suffixed `_512`) with the generic `<HAS_DOT>`-selected dot call
replaced by the unconditional EVEX one. Wired into `src/factory/plane_u16.rs`'s
`default_u16_column_plan`, checked before the existing `avxvnni` branch (same order candidates
001/012 used). Because this function is generic over `CN` and shared by `plane_u16.rs`,
`cbcr16.rs`, `rgb_u16.rs` and `rgba_u16.rs`, wiring it once here benefits all four factories'
vertical/column pass - only `plane_u16.rs`'s own bench case was actually measured (see "What's not
done" below).

Bench: `app/benches/resize_plane/main.rs`, `"Pic scale Plane10: Lanczos 3"` case
(`ResamplingFunction::Lanczos3`, `WorkloadStrategy::PreferSpeed`, `ThreadingPolicy::Single`,
4928x3279 -> 1232x819, bit_depth=10, `../assets/nasa-4928x3279-rgba.png`). This case already
exists in the harness and exercises both the horizontal and vertical LB u16 passes (Lanczos3
downscale needs both) - no bench-file addition needed. `app/Cargo.toml` needed `avx`/`sse`/`avx512`
added to the `pic-scale` path-dependency's feature list to reach any x86 SIMD tier at all (same
instrumentation as every other AVX-512 candidate in this workspace) - included in `patch.diff`.

Paired via `___OPTIMIZATIONS___/tools/paired.sh` (alternating A/B, 9 pairs each; run from `app/`
so the bench's relative asset path resolves), point estimate = criterion's reported mean (middle
of `[low mean high]`). A = baseline (patched `app/Cargo.toml` only, so AVX2 fallback ships, same
as today), B = patched (new `avx512vnni`+`avx512vl` branch active).

| Variant | pinned (`taskset -c 3`) | unpinned | notes |
|---|---|---|---|
| baseline (AVX2 fallback, madd+add - avxvnni gate never fires on this box) | run1 median 4.089 ms; run2 median 6.786 ms | run1 median 4.098 ms; run2 median 4.176 ms | A in all 4 runs |
| patched (new avx512vnni+avx512vl branch, EVEX vpdpwssd) | run1 median 3.784 ms (-7.5%, B faster 9/9); run2 median 6.177 ms (-9.0%, B faster 6/9) | run1 median 3.754 ms (-8.4%, B faster 9/9); run2 median 3.968 ms (-5.0%, B faster 8/9) | B in all 4 runs |

Raw pairs:

- Pinned run1: `A: 4.0894 4.1014 4.1102 4.0882 4.0652 4.1129 4.0772 4.0608 4.0906` /
  `B: 3.6394 3.8938 3.8121 3.7916 3.7844 3.7434 3.763 3.7556 3.8646`
- Pinned run2: `A: 4.4076 6.8134 4.2189 6.7857 4.1809 8.5017 3.9389 13.439 12.169` /
  `B: 6.1772 3.8903 6.0717 4.0124 3.7539 7.5324 8.0333 7.8742 6.6768`
- Unpinned run1: `A: 4.0107 4.027 4.0282 4.1532 4.1513 4.1795 4.0978 4.1033 4.0896` /
  `B: 3.6834 3.7043 3.7106 3.7728 3.8107 3.8181 3.7543 3.7811 3.7386`
- Unpinned run2: `A: 5.0175 4.1757 5.2635 4.4142 4.936 4.1133 3.9852 4.1423 4.1595` /
  `B: 4.1332 4.4493 4.4738 3.9679 4.7044 3.6414 3.6712 3.8421 3.8108`

3 of 4 repeats hit `paired.sh`'s consistency bar (9/9, 9/9, 8/9); pinned run2 landed on the same
bimodal ~4ms/~7-13ms host-noise clustering already documented in `README.md`/`IDEAS.md` for
`resize_plane` (candidates 009/013) - both A and B affected identically within that run, and it
still leans the same direction (-9.0%, 6/9, just under the 8/9 bar) as the other three repeats.

## Result

A real, consistent effect in one direction across all 4 repeats: -5.0% to -9.0% (median ~-7.5%),
patched faster every time, 3 of 4 repeats individually reaching `paired.sh`'s own consistency bar.
This is a different outcome from every other `avxvnni`/`avx512vnni` dispatch-branch candidate in
this workspace so far (001: strong regression; 012/013: no separable effect) - the vertical/column
kernel shape genuinely responded differently to the same "swap VEX dot for EVEX dot" change.

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone correctness-check crate (`bit_exact_check/` in this folder)
  resizes the same `nasa-4928x3279-rgba.png` asset with `Lanczos3`/`PreferSpeed`/bit_depth=10/
  Plane u16 (`"Pic scale Plane10: Lanczos 3"`'s exact transform), built once against baseline
  source and once against patched source: both produce `len=1009008 hash=0ee2599b9387b63f`
  (`DefaultHasher` over the output buffer).
- **Is this a breaking change?** No public API change either way.
- Project test suite (`cargo test --release`, no avx512 feature - this patch's new code is
  entirely behind `#[cfg(feature = "avx512")]` so this run doesn't compile it at all): baseline
  66 passed / 0 failed (lib) + 0 passed / 20 ignored (doctests); patched: identical, 66 passed /
  0 failed + 0 passed / 20 ignored.
- Project test suite (`cargo test --release --features avx512`, exercises the new cfg-gated
  code path if any existing test happens to touch bit_depth<=12 u16 vertical convolution):
  baseline 68 passed / 0 failed (lib) + 0 passed / 20 ignored (doctests); patched: identical,
  68 passed / 0 failed + 0 passed / 20 ignored.

## Why the result looks the way it does

The horizontal kernels candidates 001/012/013 tested all spend a large, fixed share of their
per-tap instruction stream on `_mm_shuffle_epi8`/`_mm256_shuffle_epi8` lane reformatting (turning
interleaved RGB/RGBA/CbCr pixels into dot-product-friendly lanes) - swapping the final
`vpmaddwd`+`vpaddd` pair for one `vpdpwssd` only removes a small fraction of that per-tap cost, and
on this Zen4 core that fraction was too small to separate from host noise (012/013) or was
outweighed by other effects entirely (001, a structurally different, shuffle-*light* baseline
kernel). This column kernel does no shuffle at all - each tap is just a zero-extend
(`_mm256_unpacklo_epi16`/`_mm256_unpackhi_epi16` against a zero vector, already required either
way) followed directly by the accumulate this patch changes. With less competing per-tap overhead,
saving one instruction (two down to one) is a proportionally larger share of the loop body, which
is consistent with this candidate actually producing a measurable, repeatable win where 012/013 did
not.

## What's not done / open questions

- Only `plane_u16.rs`'s own bench case (`resize_plane`, CN=1) was measured. `default_u16_column_plan`
  is shared by `cbcr16.rs` (CN=2), `rgb_u16.rs` (CN=3), and `rgba_u16.rs` (CN=4) via the same
  generic function - wiring this candidate's dispatch branch benefits all four automatically (it's
  one shared function, not four separate ones), but `resize_cbcr`/`resize_rgb`/`resize_rgba`'s
  u16 vertical passes were not separately benched to confirm the same magnitude holds for CN != 1.
- Not tested on real `avxvnni` hardware (Alder Lake+/Sierra Forest) - same caveat as every other
  `avx512vnni`-vs-`avxvnni` candidate in this workspace, this only establishes the EVEX/avx512vnni
  path's behavior relative to the AVX2 fallback on Zen4, not relative to the VEX `avxvnni` path
  itself (which cannot run on this box either way).
- Given one of the four repeats (pinned run2) landed on this workspace's known bimodal host-noise
  pattern, a 5th repeat (pinned) could firm up whether the true effect is closer to the ~-7.5%
  clean-run estimate or the noisier ~-9.0%/6-of-9 one - not done here since 3 of 4 repeats already
  agreed closely enough (-5.0% to -8.4%) to call the direction and rough magnitude settled without
  it.
- A `perf stat` instruction-count comparison (already an open backlog item from 001/012/013,
  RGBA_PLANE_U16_HORIZONTAL) would help confirm the "less shuffle overhead -> instruction-count
  saving matters more" explanation directly rather than inferring it from wall-clock alone.
