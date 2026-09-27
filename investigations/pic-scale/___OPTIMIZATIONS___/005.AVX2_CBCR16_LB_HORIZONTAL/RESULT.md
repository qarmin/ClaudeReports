# AVX2 horizontal convolution tier for CbCr `u16` (bit_depth <= 12)

Candidate 002 added `src/sse/cbcr_u16_lb.rs`, an SSE4.1-only kernel for `src/factory/cbcr16.rs`'s
`bit_depth <= 12` x86_64 path (before that, this path had no x86 SIMD kernel at all - only
`aarch64`+`neon`). Candidate 002 explicitly scoped out an AVX2 tier. This candidate adds one:
`src/avx2/cbcr_u16_lb.rs`, widening the SSE kernel's "reorder taps into per-channel groups via
`shuffle_epi8`, then `madd_epi16`" trick to 256-bit, and wires it into `cbcr16.rs`'s dispatch
above the SSE4.1 branch (same tiering pattern already used by `src/factory/rgba_u16.rs`).

## What was tried

Only one variant. Because a `bit_depth <= 12` value already fits in an `i16` (no zero-extend
needed, unlike candidate 004's `u8` case), a single `u16` tap already occupies a full 16-bit
lane - so one `__m256i` load covers the same 8 taps the SSE kernel's `conv_cb_cr16_8` needed two
separate `__m128i` loads for (taps 0-3 in the low 128 bits, taps 4-7 in the high 128 bits -
contiguous in memory, so one 256-bit load gets both). The actual win is instruction count, not
taps-per-load: one `_mm256_shuffle_epi8`/`_mm256_madd_epi16` pair replaces the SSE kernel's two
separate 128-bit shuffle+madd pairs for those same 8 taps, since `_mm256_shuffle_epi8` shuffles
each 128-bit lane independently. The weight-side control mask is *not* a plain 128-bit-replicated
broadcast (unlike the source-side `idx_deinterleave256`) - its low lane carries the SSE kernel's
`weights_lo` pattern and its high lane carries `weights_hi`, so one `_mm256_shuffle_epi8` against
a broadcast-replicated raw-weight register produces both `w0` and `w1` (the two lanes of one
`__m256i`) in one instruction. A 16-tap batch (`conv_cb_cr16_16`) repeats this twice (taps 0-7,
then 8-15) per outer-loop iteration and defers folding lane0+lane1 together until the whole
16-tap loop for a given output pixel is done, rather than after every 8 taps, to keep the
cross-lane op off the hot path. Falls back to (locally-duplicated, `target_feature = "avx2"`)
copies of the SSE kernel's 8/4/2/1-tap remainder functions for the tail, same shape as
candidate 004's `avx2/cbcr8.rs`.

Bench: `app/benches/resize_cbcr/main.rs`, `"Pic scale CbCr10: Lanczos 3"` case
(`ResamplingFunction::Lanczos3`, `WorkloadStrategy::PreferSpeed`, `ThreadingPolicy::Single`,
`bit_depth = 10`, same case candidate 002 used). `app/Cargo.toml`'s `pic-scale` feature list
needed `"sse"` (baseline) or `"sse", "avx"` (patched) to reach the respective tier - see
`README.md`. Two prebuilt binaries were compared via `tools/paired.sh` (`RUNS=9`, alternating
A/B, point estimate = criterion's reported mean):

- **Baseline** ("A"): candidate 002's SSE4.1 kernel active (this candidate's `patch.diff` applied,
  but `app/Cargo.toml` given only `"sse"`, not `"avx"`, so the new AVX2 branch in `cbcr16.rs` is
  compiled out and dispatch falls to the existing SSE4.1 branch) - i.e. "SSE4.1-only", isolating
  this candidate's own AVX2 contribution rather than comparing against the pristine scalar
  fallback candidate 002 already measured.
- **Patched** ("B"): same source, `app/Cargo.toml` given `"sse", "avx"` - AVX2 branch active on
  this box (Zen4, has `avx2`).

`uptime` showed load ~2-17 across the runs on this 24-logical-core box (consistent with this
workspace's already-documented host-level noise for `resize_cbcr` - see `README.md`).

| Variant | single-core (`taskset -c 3`), median of 9 | multi-core (unpinned), median of 9 | notes |
|---|---|---|---|
| baseline (SSE4.1 only) vs. AVX2 (this patch), repeat 1 | 15.949 -> 9.768 ms (**-38.8%**), 8/9 pairs | - | "consistent" |
| same, repeat 2 (pinned) | 15.858 -> 8.762 ms (**-44.7%**), 9/9 pairs | - | "consistent" |
| same, repeat 1 (unpinned) | - | 15.648 -> 8.505 ms (**-45.6%**), 9/9 pairs | "consistent" |
| same, repeat 2 (unpinned) | - | 15.660 -> 8.498 ms (**-45.7%**), 9/9 pairs | "consistent" |

Raw pairs (ms), pinned repeat 2 (the cleanest of the two pinned runs):
`A: 22.792 15.888 15.795 15.733 15.858 15.511 15.244 18.369 17.641` /
`B: 9.2129 8.6694 12.522 8.6172 8.599 8.5558 12.254 10.012 8.7622`

Raw pairs (ms), unpinned repeat 2:
`A: 15.647 15.66 15.701 16.12 15.679 15.724 15.62 15.544 15.613` /
`B: 8.446 8.5178 8.4723 8.5118 8.4608 8.4982 8.5094 8.5217 8.4657`

## Result

A large, consistent improvement: **roughly -39% to -46% median wall-clock** (tightening to a
narrow -44% to -46% band across the 3 of 4 runs that landed 9/9 "consistent" by `paired.sh`'s own
rule) for the `"Pic scale CbCr10: Lanczos 3"` case, both pinned single-core and unpinned
multi-core. This is a substantially larger win than expected going in - the task framing
("SSE4.1->AVX2, a narrower gap" than scalar->SSE4.1) predicted something closer to candidate 004's
u8 CbCr AVX2 result (-4% to -7%), but the measured delta here is actually *larger* in relative
terms than candidate 002's original scalar->SSE4.1 win (-18% to -26%). See "Why the result looks
the way it does" below for the likely explanation (halved shuffle+madd instruction count per tap,
not just wider loads).

## Does it change behavior/output?

- **Bit-exact?** Yes. `bit_exact_check/` (candidate-local standalone crate, extending candidate
  002's 11 synthetic-case sweep with 4 more extreme-downscale cases specifically chosen to push
  `bounds_size` past 16 taps/output pixel, exercising the new AVX2 16-tap main loop and its
  jx==0/jx>0 fold-vs-no-fold branch) resamples synthetic CbCr `u16` sources - `bit_depth` in
  `{1, 8, 10, 12}`, both `WorkloadStrategy::PreferSpeed`/`PreferQuality`, sizes/ratios sweeping
  every tap-count remainder (1/2/4/8/16-tap batches and all combinations) - and hashes the
  concatenated output with `DefaultHasher`. Built once with `sse` only and once with `sse, avx`
  (both against the same patched source): both print the identical
  `FINAL_HASH=12406396843747482019` and identical per-case `first8=[...]` samples.
- **Is this a breaking change?** No - internal dispatch only, no public API change.
- Project test suite (`cargo test -p pic-scale --features avx,sse,colorspaces,rdm --lib`): 66
  passed, 0 failed - same count candidate 002 reported (no dedicated CbCr16 low-bit-depth test in
  the existing suite; `bit_exact_check` is this candidate's own coverage for that gap, same as
  002's).

## Why the result looks the way it does

Unlike candidate 004 (AVX2 CbCr `u8`, where a `u8` tap needs a zero-extend widen to reach a
16-bit lane, so the AVX2 win was "twice the taps per load, same shuffle/madd shape" - a genuine
but modest throughput gain), this `u16` case gets a different, larger win: because a `u16` tap
*already* fills a 16-bit lane, the existing SSE kernel's 8-tap step was artificially split across
two separate 128-bit registers and two separate shuffle+madd pairs purely because SSE registers
are only 128 bits - not because the data needed splitting. Packing that same 8-tap step into a
single 256-bit register collapses two shuffles and two madds into one of each, roughly halving
the per-tap instruction count for what was already the kernel's hottest steady-state loop (the
8-tap batch handles the bulk of typical Lanczos3 filter widths), on top of also processing double
the taps once the new 16-tap outer loop is reached. That combination - fewer instructions *and*
more work per instruction - is a better fit for this box than candidate 004's CbCr-`u8` case
(shuffle-bound either way) or candidate 003's plane-`u8` case (no interleave/shuffle cost to
amortize at all), which likely explains why this candidate's relative win is larger than either.

## What's not done / open questions

- No AVX512 tier added (out of scope - see candidate 001's regression finding for a similarly
  shuffle-heavy dot-product kernel shape on this Zen4 box before assuming a VNNI-based version
  would win here).
- Not re-measured on hardware without AVX2 (dispatch already falls back to SSE4.1/scalar
  correctly there; no behavior change expected, not specifically re-verified this session).
- Only 2 pinned + 2 unpinned repeats were run (not 3+3 like candidate 002 needed) because the
  effect size here was large enough (-39% to -46%) to clear `paired.sh`'s decisiveness bar in 3
  of 4 runs on the first attempt - the one "not fully 9/9" pinned run (8/9) was still a clean,
  large, same-direction result, not the borderline noise candidates 002/004 hit.
- This candidate's `patch.diff` bundles candidate 002's (still-unmerged) SSE4.1 kernel as a
  prerequisite, since the AVX2 tier dispatches above it - see the note in `justfile`.
