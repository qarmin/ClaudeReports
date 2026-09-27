# AVX-512VNNI dispatch branch for RGB u16 horizontal convolution (bit_depth<=12)

`src/factory/rgb_u16.rs` only ever activates the AVX2-dot-product horizontal kernel
(`src/avx2/rgb_u16_lb.rs`) in its fused-multiply-add form when `is_x86_feature_detected!("avxvnni")`
is true - same pattern candidate 001 tried for RGBA u8, flagged in `IDEAS.md` as untested for the
`u16` factories (`rgb_u16.rs`/`rgba_u16.rs`/`plane_u16.rs`). `avxvnni` is the narrow VEX-encoded
extension found on Alder Lake+/Sierra Forest; this dev/bench box (AMD Ryzen 9 7900, Zen4) has
`avx512vnni`+`avx512vl` instead, so that branch never fires here as shipped.

Unlike candidate 001's RGBA u8 kernel, reading `src/avx2/rgb_u16_lb.rs` first (per the backlog
item's own instruction to check the kernel shape before wiring anything up) showed this one has a
**simpler, lower-risk shape**: the existing `avxvnni`-gated kernel and the plain-AVX2 fallback
that actually ships on this box both go through the exact same shuffle/interleave code
(`_mm256_shuffle_epi8`/`_mm_shuffle_epi8` reformatting into dot-product-friendly lanes) - the
*only* difference between them is the final accumulate instruction, selected by a `const D: bool`
generic that picks `_mm256_dpwssd_avx_epi32`/`_mm_dpwssd_avx_epi32` (VEX, fused multiply-add) vs.
`_mm256_add_epi32(a, _mm256_madd_epi16(b, c))`/`_mm_add_epi32(a, _mm_madd_epi16(b, c))` (two
instructions), in `src/avx2/utils.rs`'s `_mm256_dot16_avx_epi32`/`_mm_dot16_avx_epi32`. This is a
much cleaner "swap one instruction, everything else identical" comparison than candidate 001's,
where the AVX2 fallback used a structurally different, shuffle-light kernel and the avxvnni path
used a shuffle-heavy one - so the two candidates cannot be assumed to behave the same way.

## What was tried

Added `avx512vnni`+`avx512vl`-gated counterparts of the existing avxvnni row4/row1 kernels in
`src/avx2/rgb_u16_lb.rs`, using the EVEX-encoded `_mm256_dpwssd_epi32`/`_mm_dpwssd_epi32`
intrinsics (new `_mm256_dot16_avx512_epi32`/`_mm_dot16_avx512_epi32` helpers in
`src/avx2/utils.rs`) instead of touching the existing, widely-shared `_mm256_dot16_avx_epi32`/
`_mm_dot16_avx_epi32` (which are also used by `rgb_u8.rs`, `vertical_u8.rs`, `vertical_u16_lb.rs`,
`vertical_s16_lb.rs`, `plane_s16_lb.rs`, `rgba_u16_lb.rs`, `plane_u16_lb.rs` - adding a const
generic there would have meant touching dozens of unrelated call sites for a single-candidate
change). Concretely: fully separate `acc_1_dot_rgb_512`/`acc_2_dot_rgb_512`/`acc_4_dot_rgb_512`/
`acc_8_dot_rgb_512` free functions and `Row4ExecutionHandlerRgb512`/`OneRowExecutionUnitRgb512`
structs, byte-for-byte the same shuffle/load logic as the existing `avxvnni`-gated ones, just
calling the EVEX dot helper unconditionally instead of the generic `<D>`-selected one. Wired into
`src/factory/rgb_u16.rs`'s `HorizontalFilterPass<u16, f32, 3>::horizontal_plan`, checked before
the existing `avxvnni` branch (same order candidate 001 used).

Only the RGB (CN=3) `u16` factory was wired up and measured - `rgba_u16.rs`/`plane_u16.rs` share
the exact same `_mm256_dot16_avx_epi32`/`_mm_dot16_avx_epi32` pattern (confirmed by reading
`src/avx2/rgba_u16_lb.rs`/`src/avx2/plane_u16_lb.rs`) but were left untouched to keep this a
single, bounded candidate - see "What's not done" below.

Bench: `app/benches/resize_rgb/main.rs`, `"Pic scale RGB10: Lanczos 3"` case
(`ResamplingFunction::Lanczos3`, default workload strategy, `ThreadingPolicy::Single`,
4928x3279 -> 1232x819, bit_depth=10, `../assets/nasa-4928x3279-rgba.png`). This case already
exists in the harness (no bench-file addition needed, unlike several other candidates in this
workspace) and reaches both the row4 and row1/remainder kernels this patch touches. Single-core =
`taskset -c 3`-pinned; multi-core = same unpinned. `app/Cargo.toml` needed
`avx`/`sse`/`avx512` added to the `pic-scale` path-dependency's feature list to reach any x86 SIMD
tier at all (same instrumentation as candidate 001/README.md) - included in `patch.diff`.

Paired via `___OPTIMIZATIONS___/tools/paired.sh` (alternating A/B, 9 pairs each; run from `app/`
so the bench's relative asset path resolves), point estimate = criterion's reported mean (middle
of `[low mean high]`).

| Variant | pinned (`taskset -c 3`) median | unpinned median | notes |
|---|---|---|---|
| baseline (today's code - AVX2 fallback with madd+add, `avxvnni` gate never fires) | run1: 13.872 ms; run2: 28.137 ms | run1: 13.937 ms; run2: 14.862 ms | A in all 4 runs |
| patched (new `avx512vnni`+`avx512vl` branch active, EVEX `vpdpwssd`) | run1: 13.801 ms (-0.5%, B faster 6/9); run2: 27.547 ms (-2.1%, B faster 5/9) | run1: 13.650 ms (-2.1%, B faster 7/9); run2: 15.018 ms (+1.0%, B faster 4/9) | B in all 4 runs |

All 4 repeats landed **outside** `paired.sh`'s consistency bar (needs `wins <= 1` or `wins >= 8`
out of 9): pinned repeats were 6/9 and 5/9, unpinned repeats were 7/9 and 4/9. Raw pairs:

- Pinned run1: `A: 13.823 13.843 13.777 13.792 13.872 14.511 14.283 27.977 17.442` /
  `B: 13.653 13.731 13.623 13.715 13.801 14.647 27.684 28.571 14.384`
- Pinned run2: `A: 14.857 29.433 28.192 26.095 28.137 29.045 14.718 16.102 28.508` /
  `B: 16.412 28.858 28.199 27.954 27.547 26.895 14.272 14.228 28.541`
- Unpinned run1: `A: 13.937 14.505 14.319 14.322 13.783 13.693 13.729 13.833 14.25` /
  `B: 14.195 13.65 14.381 13.613 13.584 13.613 13.601 13.666 13.903`
- Unpinned run2: `A: 13.759 14.293 13.906 14.476 15.768 14.862 15.639 15.732 15.578` /
  `B: 14.984 14.59 15.36 14.894 15.018 15.392 15.567 15.394 13.606`

Both pinned repeats show the same bimodal ~14ms/~27-29ms clustering across the *same* 9-run
sequence (both A and B affected identically) already documented in `README.md`/`IDEAS.md` for
`resize_rgb` (candidate 011's wide-filter case saw this on both pinned and unpinned; here it
recurred on pinned only for this narrow-filter case). Unpinned runs were mostly clean (~13-16ms)
but still landed on opposite sides of no-effect (-2.1% then +1.0%).

## Result

No consistent effect either direction: 4 of 4 repeats (2 pinned, 2 unpinned) failed
`paired.sh`'s own consistency check, with the point estimate swinging between -2.1% and +1.0%
depending on the repeat. Unlike candidate 001's RGBA u8 case (which measured a large, consistent
~26-34% regression in every repeat), swapping the VEX `vpdpwssd_avx`-style fused multiply-add for
the EVEX `vpdpwssd` one - with byte-identical surrounding shuffle/load code either way - produces
a difference too small to separate from this host's measurement noise on this kernel/input.

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone correctness-check crate (`bit_exact_check/` in this folder)
  resizes the same `nasa-4928x3279-rgba.png` asset with `Lanczos3`/bit_depth=10/RGB u16
  (`"Pic scale RGB10: Lanczos 3"`'s exact transform), built once against baseline source and once
  against patched source: both produce `len=3027024 hash=633b7f995986e40f` (`DefaultHasher` over
  the output buffer).
- **Is this a breaking change?** No public API change either way.
- Project test suite (`cargo test --release`, no avx512 feature): baseline 66 passed / 0 failed
  (lib) + 0 passed / 20 ignored (doctests); patched: identical, 66 passed / 0 failed + 0 passed /
  20 ignored.
- Project test suite (`cargo test --release --features avx512`, exercises the new cfg-gated
  code): baseline 68 passed / 0 failed (lib) + 0 passed / 20 ignored (doctests); patched:
  identical, 68 passed / 0 failed + 0 passed / 20 ignored. (The 2 extra lib tests under
  `avx512` vs. without are pre-existing, unrelated to this patch - present in both baseline and
  patched builds.)

## Why the result looks the way it does

Consistent with the hypothesis in the backlog item that motivated checking this before wiring it
up: this kernel's shuffle-heavy reformatting (splitting each u16 RGB triple into dot-product-ready
lanes via several `_mm_shuffle_epi8`/`_mm256_shuffle_epi8` calls) is identical between the
baseline and patched code paths - only the final per-tap accumulate instruction changes (one fused
`vpdpwssd` vs. two instructions, `vpmaddwd`+`vpaddd`). That is a much smaller change in the
instruction stream than candidate 001's comparison (which additionally introduced the shuffle
overhead itself, since the AVX2 fallback there used a shuffle-*light* kernel). On this Zen4 core,
saving one instruction per tap inside an already shuffle-dominated loop apparently isn't large
enough, relative to this host's own measurement noise for this bench case (visible bimodal
clustering independent of either binary), to produce a signal in either direction across 4
repeats.

## What's not done / open questions

- `rgba_u16.rs`/`plane_u16.rs` (CN=4 and CN=1) were not wired up or measured, despite sharing the
  identical `_mm256_dot16_avx_epi32`/`_mm_dot16_avx_epi32` pattern - reading their kernels
  (`src/avx2/rgba_u16_lb.rs`, `src/avx2/plane_u16_lb.rs`) confirms the same "identical
  shuffle, only the final accumulate instruction differs" shape, so the same null result is a
  reasonable prior for them, but this wasn't measured. Also untouched: `plane_u16.rs`'s *second*
  `avxvnni` gate (the vertical/column path, `convolve_column_lb_avx2_u16_vnni`) flagged in the
  same backlog item - that one is a different kernel shape (vertical accumulation, not horizontal
  dot-product-over-shuffled-lanes) and would need its own check before assuming this result
  applies.
- Not tested on real `avxvnni` hardware (Alder Lake+/Sierra Forest) - same caveat as candidate
  001, this only establishes the EVEX/avx512vnni path's behavior on Zen4.
- Given the noisy, inconclusive result, more repeats (or a `perf stat` instruction-count
  comparison instead of wall-clock) could narrow whether there's a true small effect hiding under
  this host's noise floor for this bench case - not done here per the bounded-investigation
  guidance in SKILL.md's pitfall #10, since the backlog item's main open question (kernel shape)
  was already answered by reading the code before measuring.
