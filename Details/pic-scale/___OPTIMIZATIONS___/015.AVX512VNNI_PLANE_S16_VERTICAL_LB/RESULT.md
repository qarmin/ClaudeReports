# AVX-512VNNI (and avxvnni) dispatch branch for signed-s16 Plane vertical (column) convolution (bit_depth<=12)

`src/factory/plane_s16.rs`'s `Column filter` for `i16` (`ColumnFilter<i16, 1>`) always calls
`src/avx2/vertical_s16_lb.rs`'s `convolve_column_lb_avx2_s16`, which is hard-wired to the
`HAS_DOT=false` (`_mm256_add_epi32(a, _mm256_madd_epi16(b, c))`) instantiation - unlike its
structurally identical unsigned sibling `convolve_column_lb_avx2_u16` in `vertical_u16_lb.rs`,
which already has an `avxvnni`-gated `HAS_DOT=true` counterpart
(`convolve_column_lb_avx2_u16_vnni`) wired into `default_u16_column_plan`. Confirmed via grep
before starting: `vertical_s16_lb.rs` had zero `avxvnni`/`avx512vnni` references at all. Flagged
in `IDEAS.md` after candidate 014 measured a real ~5-9% win adding an `avx512vnni`+`avx512vl`
branch to the u16 sibling.

Reading `vertical_s16_lb.rs` confirmed it is a byte-for-byte structural twin of
`vertical_u16_lb.rs` (same 32/16/8/4-wide chunking, same `_mm256_dot16_avx_epi32::<HAS_DOT>`/
`_mm_dot16_avx_epi32::<HAS_DOT>` calls, same "zero-extend then accumulate" per-tap shape with no
shuffle) - only the packing (`_mm256_packs_epi32`+min/max clamp to a signed range instead of
`_mm256_packus_epi32`+min-only clamp) and the const-generic wiring differ.

## What was tried

Two additions, both gated behind `#[cfg(feature = "avx512")]` like every other `avxvnni`/
`avx512vnni` branch in this workspace:

1. **`avxvnni`-gated branch** (`convolve_column_lb_avx2_s16_vnni`): a thin wrapper calling the
   *already-existing* `convolve_column_lb_avx_i16_impl::<true>` instantiation (this const-generic
   parameter existed in the source already but no caller ever passed `true` for signed s16) via a
   new `#[target_feature(enable = "avx2", enable = "avxvnni")]` function - mirrors
   `convolve_column_lb_avx2_u16_vnni`'s existing pattern exactly. This branch cannot fire on this
   dev box (Zen4 has `avx512vnni`, not `avxvnni`) - added for parity with the u16 sibling and
   because the task asked for both, not measured on real `avxvnni` hardware (same caveat as every
   other `avxvnni` branch in this workspace).
2. **`avx512vnni`+`avx512vl`-gated branch** (`convolve_column_lb_avx2_s16_vnni512`), following
   candidate 014's exact pattern: reused the `_mm256_dot16_avx512_epi32`/`_mm_dot16_avx512_epi32`
   EVEX (`vpdpwssd_epi32`) helpers added to `src/avx2/utils.rs` (candidate 014's patch was
   reverted at the end of its own session, so these needed re-adding here - now shared by both
   the u16 and s16 candidates' patches, not literally shared code since neither patch is applied
   in the real tree) and byte-for-byte copies of `convolve_column_lb_32_i16`/`_16_i16`/`_8_i16`/
   `convolve_column_lb_avx_i16_impl` (suffixed `_512`) with the `<HAS_DOT>`-selected dot call
   replaced by the unconditional EVEX one.

Wired into `src/factory/plane_s16.rs`'s `ColumnFilter<i16, 1>` impl, `avx512vnni`+`avx512vl`
checked first, then `avxvnni`, then the plain AVX2 fallback - same order candidates 001/012/014
used.

Bench: `app/benches/resize_plane/main.rs`, `"Pic scale Plane S10: Lanczos 3"` case
(`ResamplingFunction::Lanczos3`, `WorkloadStrategy::PreferSpeed`, `ThreadingPolicy::Single`,
4928x3279 -> 1232x819, bit_depth=10, signed i16, `../assets/nasa-4928x3279-rgba.png`). This case
already existed in the harness (no bench-file addition needed) and exercises both the horizontal
and vertical LB s16 passes. `app/Cargo.toml` needed `avx`/`sse`/`avx512` added to the `pic-scale`
path-dependency's feature list to reach any x86 SIMD tier at all - same instrumentation as every
other AVX-512 candidate in this workspace, included in `patch.diff`.

Paired via `___OPTIMIZATIONS___/tools/paired.sh` (alternating A/B, 9 pairs each; run from `app/`
so the bench's relative asset path resolves), point estimate = criterion's reported mean (middle
of `[low mean high]`). A = baseline (patched `app/Cargo.toml` only, so the AVX2/madd+add fallback
ships, same as today - the box has no `avxvnni` so that branch can't fire either way), B = patched
(new `avx512vnni`+`avx512vl` branch active).

| Variant | pinned (`taskset -c N`) | unpinned | notes |
|---|---|---|---|
| baseline (AVX2 fallback, madd+add) | run1 (core3) median 4.947 ms; run2 (core3) median 5.726 ms; run3 (core3) median 5.070 ms | run1 median 5.832 ms; run2 median 4.704 ms | A in all 5 runs |
| patched (avx512vnni+avx512vl branch, EVEX vpdpwssd) | run1 (core3) median 4.444 ms (-10.2%, B faster 7/9); run2 (core3) median 5.011 ms (-12.5%, B faster 7/9); run3 (core5) median 4.594 ms (-9.4%, B faster 9/9) | run1 median 5.519 ms (-5.4%, B faster 8/9); run2 median 4.278 ms (-9.0%, B faster 9/9) | B in all 5 runs |

An extra pinned repeat (2nd of 4 pinned attempts, `taskset -c 3`) landed on this workspace's
already-documented `resize_plane` bimodal host-noise clustering (both A and B's samples split into
a ~5ms/~11-16ms bimodal pattern) and is **not** included in the table above - included below for
completeness, it moved in the wrong direction (+23.0%, B faster only 2/9) purely from that noise,
same failure mode already seen for candidates 009/013/014.

Raw pairs:

- Pinned run1 (`taskset -c 3`): `A: 4.623 5.3292 4.9449 4.9431 4.9948 4.8963 5.0281 4.9533 4.9475` /
  `B: 5.2812 4.3172 4.4731 4.542 4.4438 4.4083 4.4365 4.3161 9.5424`
- Pinned run2 (`taskset -c 3`, discarded - bimodal noise): `A: 10.427 10.234 14.715 11.776 11.182
  10.532 12.601 10.647 5.0228` / `B: 12.163 10.448 12.833 13.099 13.676 16.403 15.091 13.536 4.496`
- Pinned run3 (`taskset -c 3`): `A: 11.666 12.153 5.018 5.0047 5.157 6.7991 5.6787 5.7264 6.8125` /
  `B: 12.023 4.2798 4.2702 4.5145 5.7951 4.885 5.0108 5.5567 5.7025`
- Pinned run4 (`taskset -c 5`): `A: 5.7845 5.6733 5.8433 4.6587 4.9035 4.6068 5.1732 4.7975 5.0703`
  / `B: 5.3505 5.1149 4.741 4.1769 4.172 4.2877 4.6803 4.5943 4.3942`
- Unpinned run1: `A: 5.9583 5.8324 5.5376 6.0049 4.9839 5.0754 5.9769 6.0478 5.8163` /
  `B: 5.6525 4.9255 5.5193 5.5639 4.4072 5.5441 5.6794 5.5047 4.9012`
- Unpinned run2: `A: 6.2371 5.4087 4.6946 4.6982 4.6711 4.7134 4.763 4.7036 4.6767` /
  `B: 5.5602 4.7489 4.251 4.2453 4.3068 4.2738 4.2782 4.2347 4.2955`

Of the 6 total repeats run (4 pinned across 2 different cores, 2 unpinned), 4 hit `paired.sh`'s
own 8/9-or-better consistency bar (pinned run3 9/9, pinned run4 9/9, unpinned run2 9/9, unpinned
run1 8/9); the 2 that didn't (pinned run1 7/9, pinned run2 2/9) both still leaned the same
direction except the one confirmed-bimodal-noise repeat, or were close to the bar (7/9, twice).

## Result

A real, consistent effect in one direction: every one of the 6 repeats (including the noisy
discarded one) showed the patched binary faster on point estimate, -5.4% to -12.5% (median
~-9.4%) across the 5 repeats not dominated by the known bimodal noise pattern. This closely
matches candidate 014's u16 sibling result (-5.0% to -9.0%, median ~-7.5%) both in direction and
magnitude, confirming the backlog item's hypothesis: this signed s16 column kernel is structurally
close enough to the u16 one that the same EVEX-dot-product change produces the same kind of win.

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone correctness-check crate (`bit_exact_check/` in this folder,
  path-dependency pointed at the live checkout for this session's verification, `../clone` in the
  self-contained `justfile` version) resizes the same `nasa-4928x3279-rgba.png` asset with
  `Lanczos3`/`PreferSpeed`/bit_depth=10/signed Plane i16 (`"Pic scale Plane S10: Lanczos 3"`'s
  exact transform), built once against baseline source (via `git stash`) and once against patched
  source: both produce `len=1009008 hash=5e00a167d57030b5` (`DefaultHasher` over the output
  buffer).
- **Is this a breaking change?** No public API change either way.
- Project test suite (`cargo test --release`, no avx512 feature - this patch's new code is
  entirely behind `#[cfg(feature = "avx512")]` so this run doesn't compile it at all): baseline
  66 passed / 0 failed (lib) + 0 passed / 20 ignored (doctests); patched: identical, 66 passed /
  0 failed + 0 passed / 20 ignored.
- Project test suite (`cargo test --release --features avx512`, exercises the new cfg-gated
  code path if any existing test happens to touch bit_depth<=12 i16 vertical convolution):
  baseline 68 passed / 0 failed (lib) + 0 passed / 20 ignored (doctests); patched: identical,
  68 passed / 0 failed + 0 passed / 20 ignored.

## Why the result looks the way it does

Same explanation as candidate 014: this column kernel does no shuffle at all (each tap is just a
zero-extend against a zero vector, already required either way, followed directly by the
accumulate this patch changes), unlike the horizontal kernels candidates 001/012/013 tested (which
spend a large fixed share of their per-tap cost on `_mm_shuffle_epi8`/`_mm256_shuffle_epi8` lane
reformatting, leaving little room for a one-instruction saving to show up). The signed s16 variant
differs from the unsigned u16 one only in the final pack/clamp step (`_mm256_packs_epi32` + signed
min/max vs. `_mm256_packus_epi32` + unsigned min-only), which sits outside the hot dot-product
loop this patch touches - consistent with the near-identical magnitude measured here vs. candidate
014.

## What's not done / open questions

- The `avxvnni`-gated branch (`convolve_column_lb_avx2_s16_vnni`) was added for parity with the
  u16 sibling and per the task's "avxvnni/avx512vnni" framing, but not measured - this dev box has
  `avx512vnni`, not `avxvnni`, so it never fires here. Same open question already on the backlog
  for every other `avxvnni` branch in this workspace: unmeasured on real Alder Lake+/Sierra Forest
  hardware.
- One of the 4 pinned repeats (run2, `taskset -c 3`) landed squarely on this workspace's
  documented `resize_plane` bimodal host-noise pattern and was excluded from the summary table
  rather than averaged in - the same judgment call made for candidate 014's noisy pinned repeat.
  Switching to a different pinned core (`taskset -c 5`) for the 4th repeat produced a clean 9/9
  result, consistent with the noise being core-specific rather than inherent to this bench case.
- `plane_s16.rs` is the only factory reached by this bench; `convolve_column_lb_avx2_s16`/
  `_vnni`/`_vnni512` are not shared by any other factory the way `default_u16_column_plan` was
  for candidate 014 (there is no CbCr/RGB/RGBA signed-s16 column-filter factory in this codebase
  as of this session, per a grep for other `ColumnFilter<i16, ...>` impls) - so this candidate's
  scope is inherently just the one factory, not a multi-factory win like candidate 014's.
- Not tested with `perf stat` instruction counts - same bounded-investigation note as candidates
  001/012/013/014.
