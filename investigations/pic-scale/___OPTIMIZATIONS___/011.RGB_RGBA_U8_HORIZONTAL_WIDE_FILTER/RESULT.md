# Re-measuring the already-shipped AVX2 horizontal convolution kernel for RGB/RGBA (CN=3/4) u8 under a genuinely wide Lanczos3 filter

`IDEAS.md` flagged that `app/benches/resize_rgb/main.rs`/`resize_rgba/main.rs` had no wide-filter
(1/16 downscale) criterion case at all, so - unlike plane (CN=1, candidate 009) and CbCr (CN=2,
candidate 010) u8 - the RGB/RGBA AVX2 horizontal kernels (`src/avx2/rgb_u8.rs`,
`src/avx2/rgba_u8.rs`) had never been re-checked under a filter width where the 16-tap main loop
actually dominates instead of firing ~0-1 times per output pixel.

**Important structural difference from candidates 009/010**: for plane/CbCr, the AVX2 tier being
re-measured was *added to this repo by this workspace* (candidates 003/004) - the pristine base
commit had no AVX2 kernel there at all, so "baseline" (unpatched) meant "no AVX2" and "patched"
meant "has AVX2". For RGB/RGBA, `src/avx2/rgb_u8.rs`/`rgba_u8.rs` already exist in the pristine
base commit and are already unconditionally dispatched to on any `avx2`-capable x86_64 host (see
`src/factory/rgb_u8.rs:127`, `src/factory/rgba_u8.rs:122` - no scale-factor or strategy gating,
unlike the `aarch64`/`avx512` branches in the same files). There is no "add AVX2" step left to
test. To still get a comparison point, this candidate's `patch.diff` **disables** the existing
AVX2 dispatch branch (`if false && std::arch::is_x86_feature_detected!("avx2")`) to force the
SSE4.1 fallback, and compares that against the real, unpatched, pristine build (which uses AVX2
for real). So in the table below: **baseline = pristine = real shipped AVX2 behavior**,
**"SSE4.1 forced" = patch.diff applied = AVX2 dispatch disabled** - this is the reverse
patched/baseline direction from every other candidate in this workspace, called out explicitly
here so the numbers aren't misread.

## What was tried

New bench cases only (`bench_addition.diff`): `"Pic scale RGB8 wide filter: Lanczos 3"` (u8,
CN=3, `WorkloadStrategy::PreferSpeed`, `ThreadingPolicy::Single`, nasa asset resized to 1/16 in
both dimensions instead of the existing cases' 1/4) added to `app/benches/resize_rgb/main.rs`,
and `"Pic scale RGBA8 wide filter: Lanczos 3"` (u8, CN=4, no alpha premultiply - mirrors the
existing `"...without alpha..."` case - `ThreadingPolicy::Single`, same 1/16 downscale) added to
`app/benches/resize_rgba/main.rs`. `app/Cargo.toml`'s `pic-scale` feature list was given `"avx",
"sse"` for both binaries (not part of either diff, reverted after measurement, its default list
has neither). Two prebuilt binaries per bench (baseline = pristine `rgb_u8.rs`/`rgba_u8.rs`/
`factory/*` + the new bench case; SSE4.1-forced = baseline + `patch.diff` disabling the AVX2
branch) were compared with `tools/paired.sh` (`RUNS=9`, alternating A/B, point estimate =
criterion's reported mean), run from `app/` (the bench opens `../assets/...` relative to cwd).

### RGB8 wide filter

| Variant | median (`taskset -c 3`) | median (unpinned) | B faster in | notes |
|---|---|---|---|---|
| baseline (AVX2) vs. SSE4.1 forced, pinned repeat 1 | 9.534 -> 10.920 ms (**+14.5%**) | - | 0/9 | "consistent" |
| same, pinned repeat 2 | 19.327 -> 16.710 ms (**-13.5%**) | - | 3/9 | "NOT consistent" - bimodal ~10-13ms/~19-24ms clustering in both A and B |
| same, pinned repeat 3 | 10.971 -> 10.797 ms (**-1.6%**) | - | 6/9 | "NOT consistent" |
| same, pinned repeat 4 | 20.968 -> 21.176 ms (**+1.0%**) | - | 3/9 | "NOT consistent" - bimodal ~11-13ms/~19-24ms clustering again |
| same, unpinned repeat 1 | - | 11.044 -> 10.906 ms (**-1.2%**) | 3/9 | "NOT consistent" |
| same, unpinned repeat 2 | - | 11.024 -> 10.217 ms (**-7.3%**) | 5/9 | "NOT consistent" |

### RGBA8 wide filter (no alpha premultiply)

| Variant | median (`taskset -c 3`) | median (unpinned) | B faster in | notes |
|---|---|---|---|---|
| baseline (AVX2) vs. SSE4.1 forced, pinned repeat 1 | 13.602 -> 14.407 ms (**+5.9%**) | - | 4/9 | "NOT consistent" - bimodal ~13.5-14.4ms/~27-28ms clustering |
| same, pinned repeat 2 | 27.787 -> 27.548 ms (**-0.9%**) | - | 6/9 | "NOT consistent" - same bimodal clustering, ~13-38ms spread |
| same, unpinned repeat 1 | - | 16.602 -> 18.298 ms (**+10.2%**) | 1/9 | "consistent" |
| same, unpinned repeat 2 | - | 13.845 -> 13.991 ms (**+1.1%**) | 3/9 | "NOT consistent" |

`uptime` load stayed under the 24-core count throughout (0.8-3.9 range). Both bench cases showed
a strong bimodal per-sample clustering (a "fast" mode around 10-14ms and a "slow" mode around
19-38ms, seen in *both* the AVX2 and SSE4.1-forced binary within the same 9-run sequence) on
several repeats, on **both** pinned and unpinned runs - unlike candidates 009/010, where the noise
was confined to the pinned measurement and unpinned was clean. Only 2 of the 10 total paired runs
reached `paired.sh`'s 8-or-9-of-9 consistency bar: RGB pinned repeat 1 (+14.5%, AVX2 faster) and
RGBA unpinned repeat 1 (+10.2%, AVX2 faster). Every other repeat (8 of 10) was "NOT consistent"
with small, inconsistently-signed deltas (-13.5% to +5.9%).

Raw pairs (ms), RGB pinned repeat 1 (the decisive one):
`A: 9.4663 9.682 9.5203 9.5618 9.5217 9.5083 9.5336 9.7438 11.726` /
`B: 10.677 10.81 10.637 10.933 10.219 10.92 10.98 10.963 12.557`

Raw pairs (ms), RGBA unpinned repeat 1 (the decisive one):
`A: 13.712 13.846 13.91 13.671 18.097 16.602 19.482 19.377 19.389` /
`B: 13.8 14.014 13.421 14.101 19.893 18.298 20.025 20.239 20.216`

## Result

Mostly inconclusive on this box this session, with the only two decisive signals (out of 10
paired runs across RGB and RGBA, pinned and unpinned) pointing the same direction: disabling the
existing AVX2 dispatch (forcing SSE4.1) was **+14.5%** slower for RGB (pinned repeat 1, 9/9) and
**+10.2%** slower for RGBA (unpinned repeat 1, 8/9) - i.e. keeping the shipped AVX2 tier is faster
by 10-15% in those two runs. The other 8 of 10 runs (both pinned and unpinned, for both RGB and
RGBA) failed `paired.sh`'s consistency bar, with small point estimates (-13.5% to +5.9%) whose
sign flipped between repeats - not usable evidence either way. Unlike candidates 009/010, the
noise here was not cleanly separated by pinned-vs-unpinned; both modes showed the same bimodal
per-sample clustering on multiple repeats.

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone `bit_exact_check/` crate (this candidate's own, not reused
  from 009/010, since it needed to cover both RGB and RGBA/premultiply) resamples several
  synthetic RGB- and RGBA-shaped u8 sources (13 size/scale combinations sweeping tap counts
  through 1/2/4/8/16/17+ per output pixel, both `WorkloadStrategy`s for RGB, both
  premultiply-on/off for RGBA, including a case at the same ~1/16 ratio as the new bench cases)
  and hashes the output bytes. Run once against the pristine checkout (real AVX2) and once with
  `patch.diff` applied (SSE4.1 forced), `--release`: both runs printed the identical
  `FINAL_HASH=8292937692884548467` and identical per-case `first8=[...]` samples.
- **Is this a breaking change?** No - internal dispatch only (and disabling it, at that; nothing
  about this candidate is a proposed change to ship). No public API change. The new criterion
  cases are bench-only, not part of the library's public surface.
- Project test suite (`cargo test -p pic-scale --features avx,sse,colorspaces,rdm --lib`): **66
  passed, 0 failed** before (pristine, real AVX2) and **66 passed, 0 failed** after (`patch.diff`
  applied, SSE4.1 forced) - identical counts, no new failures.

## Why the result looks the way it does

The two decisive results (RGB +14.5% pinned, RGBA +10.2% unpinned) are consistent with the
magnitude candidates 003/004 originally measured for plane/CbCr's *narrower* filter (-14% to -20%,
-4% to -7% respectively, expressed the opposite direction since those candidates measured
adding AVX2 rather than removing it) - i.e. nothing here suggests RGB/RGBA's heavier 3/4-byte
de-interleave shuffle behaves qualitatively differently from plane/CbCr's simpler kernels once it
does fire under a wide filter. What's different is measurement reliability: this box showed a
strong bimodal latency clustering (roughly a 10-14ms "fast" mode and a 19-38ms "slow" mode) for
both new bench cases, present in *both* binaries within the same alternating A/B sequence and on
*both* pinned and unpinned runs - the same host-noise pattern already documented in this
workspace's `README.md`/`projects/pic-scale.md` for `resize_cbcr` (candidates 002/004) and for
`resize_plane`'s pinned wide-filter case (candidate 009), but here affecting both measurement
modes rather than just the pinned one. No confound in the patch itself was found: the bit-exact
check and test suite both passed cleanly, and the sign of every inconclusive run's point estimate
still landed within a plausible noise band around "no meaningful difference to a large real
difference", never showing a decisive SSE4.1-forced win.

## What's not done / open questions

- This settles the previously-untested question in the narrow sense that nothing found here
  contradicts "the shipped AVX2 tier remains beneficial for RGB/RGBA under a wide filter, at
  roughly the same magnitude as the narrow-filter case" - but 8 of 10 paired runs did not clear
  this workspace's own consistency bar, so this is a weaker confirmation than candidates
  009/010 got for plane/CbCr. Someone with a quieter/dedicated box, more repeats, or `perf stat`
  instruction counts (rather than wall-clock) could get a firmer number.
- Root cause of the bimodal clustering (thermal/turbo state transition, a periodic background
  task, something about this specific asset/downscale ratio's working set vs. cache behavior)
  was not investigated - same open question already flagged in `IDEAS.md` for `resize_cbcr` and
  `resize_plane`'s wide-filter case, now also seen for `resize_rgb`/`resize_rgba` on both
  measurement modes.
- Not measured on hardware without AVX2 (dispatch already falls back to SSE4.1/scalar correctly
  there; no behavior change expected, not specifically re-verified this session).
- The new `"Pic scale RGB8 wide filter: Lanczos 3"` / `"Pic scale RGBA8 wide filter: Lanczos 3"`
  bench cases themselves remain uncommitted (`bench_addition.diff` in this folder, reapplied by
  this candidate's own `justfile`) - per this workspace's rules, adding them to the real
  `app/benches/resize_rgb/main.rs`/`resize_rgba/main.rs` permanently is a separate, later
  decision for whoever reads this report.
- `patch.diff` here is not a candidate for actual application - it exists only to construct the
  SSE4.1-forced comparison variant. There is no "apply this patch" action implied by this
  candidate, unlike most others in this workspace.
