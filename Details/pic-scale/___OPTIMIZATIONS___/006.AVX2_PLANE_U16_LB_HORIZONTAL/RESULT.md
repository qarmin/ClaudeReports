# AVX2 horizontal convolution widening for planar (1-channel) `u16` low-bit-depth (`bit_depth <= 12`)

`src/avx2/plane_u16_lb.rs`'s horizontal-convolution path (`Row4ExecutionHandler` and
`OneRowExecutionUnit`, both `#[target_feature(enable = "avx2")]`) never actually issued a single
`__m256i` instruction - every load/shuffle/dot step (`acc_1_dot`/`acc_2_dot`/`acc_4_dot`/`acc_8_dot`,
all built on `_mm_dot16_avx_epi32`) stayed at 128-bit width, capping the main loop at 8 taps per
`madd_epi16`. Candidate 005 found that widening the structurally similar `cbcr_u16_lb.rs` (2-channel)
kernel from 128-bit to 256-bit was a large win (-39% to -46%) because the CbCr SSE 8-tap step was
*artificially split* across two separate 128-bit registers purely due to the channel-interleave
shuffle needing two lanes worth of source data. This candidate applies the same mechanical widening
here: adds `acc_16_dot` (one `_mm256_loadu_si256` + `_mm256_dot16_avx_epi32` per 16 taps) and a
`fold_store_256` (add low128+high128) to fold the 256-bit accumulator back into the existing 128-bit
reduction path, wired in as a new `jx + 16 <= bounds_size` loop ahead of the existing 8/4/2/1-tap
loops, in both `Row4ExecutionHandler::pass` and `OneRowExecutionUnit::pass`.

## What was tried

Only one variant (the straightforward 256-bit widening described above; no deinterleave/shuffle is
needed for the 1-channel case, unlike CbCr, so `acc_16_dot` is a plain load+dot, no `shuffle_epi8`).

Bench: `app`'s `"Pic scale Plane10: Lanczos 3"` criterion case (`resize_plane/main.rs`,
`ResamplingFunction::Lanczos3`, `WorkloadStrategy::PreferSpeed`, `ThreadingPolicy::Single`,
`bit_depth = 10`, nasa asset resized to 1/4) - the only existing case that reaches
`plane_u16_lb.rs`'s `bit_depth <= 12` path at all. `app/Cargo.toml`'s `pic-scale` feature list was
given `"avx", "sse"` (not part of `patch.diff`, reverted after measurement, same as every other
candidate in this workspace - see `README.md`). Two prebuilt `resize_plane` bench binaries
(baseline = pristine `src/avx2/plane_u16_lb.rs`, patched = this candidate's diff applied) were
compared via `tools/paired.sh` (`RUNS=9`, alternating A/B, point estimate = criterion's reported
mean), run from `app/` (the bench opens `../assets/...` relative to cwd).

| Variant | single-core (`taskset -c 3`), median of 9 | multi-core (unpinned), median of 9 | notes |
|---|---|---|---|
| baseline vs. AVX2-16-tap (this patch), pinned repeat 1 | 8.313 -> 8.361 ms (**+0.6%**), B faster in 4/9 | - | "NOT consistent" |
| same, pinned repeat 2 | 4.414 -> 4.590 ms (**+4.0%**), B faster in 4/9 | - | "NOT consistent" |
| same, unpinned repeat 1 | - | 4.284 -> 4.304 ms (**+0.5%**), B faster in 4/9 | "NOT consistent" |
| same, unpinned repeat 2 | - | 4.057 -> 4.018 ms (**-0.9%**), B faster in 5/9 | "NOT consistent" |

`uptime` load ranged ~2.4-15 across the 4 runs (this box's already-documented `resize_plane`/
`resize_cbcr` bimodal host-level noise - see `README.md`; pinned repeat 1 landed in the ~8ms mode,
the other 3 runs mostly in the ~4ms mode, visible directly in each run's raw A/B arrays below).

Raw pairs (ms), pinned repeat 1:
`A: 8.5252 6.4528 8.3131 8.4458 8.5218 8.352 4.1015 4.4039 4.5265` /
`B: 8.5525 8.6493 5.5575 9.2173 8.4181 8.3607 4.407 4.3738 4.0141`

Raw pairs (ms), unpinned repeat 2 (the cleanest/least bimodal of the four):
`A: 4.0324 4.1417 4.2655 4.0567 4.1193 4.1454 4.0049 4.0278 4.0431` /
`B: 4.1014 4.3414 4.027 3.9971 4.1238 3.9682 4.0178 4.0182 4.0007`

## Result

No measurable change. All 4 `paired.sh` runs (2 pinned, 2 unpinned) reported "NOT consistent" (B
faster in only 4-5 of 9 pairs each time, i.e. a coin flip), with point-estimate deltas ranging from
-0.9% to +4.0% - all comfortably inside this box's own noise floor for this bench (medians alone
swung 4.0ms -> 8.3ms between repeats of the *same* binary). Unlike candidate 005's CbCr case, this
is a genuine null result, not an under-measured win.

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone `bit_exact_check/` crate (adapted from candidate 005's, 15
  size/scale/bit-depth combinations x `PreferSpeed`/`PreferQuality`, including several
  extreme-downscale cases chosen to push `bounds_size` well past 16 taps/output pixel to exercise
  the new AVX2 16-tap loop and its `jx==0` fold path) resamples synthetic planar `u16` sources and
  hashes the concatenated output with `DefaultHasher`. Built once against pristine
  `plane_u16_lb.rs` and once against the patched source (same `pic-scale` checkout, just the one
  file swapped via `git checkout`/`git apply` in between): both runs printed the identical
  `FINAL_HASH=10119632667376043265` and identical per-case `first8=[...]` samples.
- **Is this a breaking change?** No - internal dispatch only, no public API change.
- Project test suite (`cargo test -p pic-scale --features avx,sse,colorspaces,rdm --lib`): **66
  passed, 0 failed** before (pristine) and **66 passed, 0 failed** after (patched) - identical
  counts, no new failures.

## Why the result looks the way it does

Candidate 005's large CbCr win came from *halving instruction count* for the existing 8-tap step:
CbCr's SSE 8-tap kernel needed two separate 128-bit `shuffle_epi8`+`madd_epi16` pairs purely because
the 2-channel deinterleave shuffle needed two lanes worth of source, so AVX2 collapsed two shuffles
and two madds into one of each for the *same* 8 taps. Planar (1-channel) `u16` has no such
artificial split to begin with: the existing SSE `acc_8_dot` already covers exactly 8 taps in a
single 128-bit `_mm_dot16_avx_epi32` call with no shuffle at all - it was already "one instruction
per 8 taps," not "two instructions for what should be one." This patch's `acc_16_dot` is therefore
a plain doubling of SIMD width (16 taps in one 256-bit `_mm256_dot16_avx_epi32` call, no shuffle
either) with no instruction-count reduction underneath it - the same shape candidate 004 measured
for CN=1 `u8` (a modest -4% to -7%), except here the win doesn't show up at all: `bounds_size` for
this bench's `Lanczos3`/`PreferSpeed`/1/4-downscale case appears to rarely if ever exceed 16 taps,
so the new outer loop likely runs 0 or 1 iterations per output pixel most of the time (the tail
loops - 8/4/2/1 - handle the rest either way, unchanged), leaving nothing for the wider path to
speed up on this specific scene. This matches the already-open `IDEAS.md` note (from candidate
005) that the existing criterion cases don't exercise wide (`PreferQuality`-scale, 16+ tap)
Lanczos3 filter windows.

## What's not done / open questions

- Not measured against a criterion case with a genuinely wide filter window (e.g. a much larger
  downscale ratio, or `WorkloadStrategy::PreferQuality`) that would make the new 16-tap loop fire
  many times per output pixel instead of ~0-1 times - the `bit_exact_check` cases exercise that
  path for correctness only, not performance (no timing harness there). Building/adding such a
  criterion case (already flagged as an open idea from candidate 005, still unaddressed) would be
  needed to tell whether this patch helps at all under wide-filter conditions, or whether the lack
  of an instruction-count reduction (see above) means it's a wash even there.
- No AVX512 tier touched (out of scope - see candidate 001's regression finding for a similarly
  shuffle-heavy dot-product kernel shape on this Zen4 box).
- Not re-measured on hardware without AVX2 (dispatch already falls back correctly there; no
  behavior change expected, not specifically re-verified this session).
