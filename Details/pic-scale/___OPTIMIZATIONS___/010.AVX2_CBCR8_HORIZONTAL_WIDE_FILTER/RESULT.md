# Re-measuring candidate 004's AVX2 horizontal kernel for CbCr (2-channel) u8 under a genuinely wide Lanczos3 filter

Candidate 004 added `src/avx2/cbcr8.rs` (an AVX2 tier for `src/factory/cbcr8.rs`'s x86
horizontal-convolution dispatch, widening `src/sse/cbcr8.rs`'s "reorder taps into per-channel
groups via `shuffle_epi8`, then `madd_epi16`" trick from 8 taps/iteration to 16 via
`_mm256_shuffle_epi8`) and measured a real but modest win (-4% to -7%) on a newly-added
`app/benches/resize_cbcr/main.rs` `"Pic scale CbCr8: Lanczos 3"` case, which only resizes to 1/4
size - a filter width that (same math candidates 006/007/008/009 already probed) tops out around
24 taps for a 4x downscale, so the new 16-tap main loop fires at most once per output pixel there.
Candidate 008 already found that the *structurally* related CbCr `u16` AVX2 win (candidate 005,
also built around removing an interleave-shuffle split) not only held but grew slightly larger
under a wide (`kernel_size~96`) filter. `IDEAS.md` flagged the CbCr u8 case as the remaining
untested cell after 007/008/009. This candidate closes that gap: it adds a new criterion case
(`"Pic scale CbCr8 wide filter: Lanczos 3"`, same shape as candidate 004's own `"Pic scale CbCr8:
Lanczos 3"` case but downscaling to 1/16 instead of 1/4) and re-measures candidate 004's
unmodified patch against it.

## What was tried

Only one variant: candidate 004's existing AVX2 kernel + factory wiring, unchanged (`patch.diff`,
byte-identical to `004.AVX2_CBCR8_HORIZONTAL/patch.diff`), measured against a new bench case
(`bench_addition.diff`, adds `"Pic scale CbCr8 wide filter: Lanczos 3"` to
`app/benches/resize_cbcr/main.rs` - `u8`/CN=2, `WorkloadStrategy::PreferSpeed` (same strategy
candidate 004's own case used), `ThreadingPolicy::Single`, nasa asset resized to 1/16 in both
dimensions instead of 1/4, using `img.to_luma_alpha8()` as the stand-in 2-channel u8 source, same
as candidate 004). `app/Cargo.toml`'s `pic-scale` feature list was given `"avx", "sse"` for both
binaries (not part of either diff, reverted after measurement). Two prebuilt `resize_cbcr` bench
binaries (baseline = pristine `cbcr8.rs`/`factory/cbcr8.rs`/`avx2/mod.rs` + this candidate's new
bench case; patched = baseline + candidate 004's patch) were compared with `tools/paired.sh`
(`RUNS=9`, alternating A/B, point estimate = criterion's reported mean), run from `app/`.
`resize_cbcr` was already flagged in this workspace as the noisiest of the four benches
(candidates 002/004), so extra repeats were budgeted here too.

| Variant | median (`taskset -c 3`) | median (unpinned) | notes |
|---|---|---|---|
| baseline vs. AVX2 (candidate 004's patch), pinned repeat 1 | 9.353 -> 8.703 ms (**-6.9%**), 7/9 | - | "NOT consistent" (just under the 8/9 bar) |
| same, pinned repeat 2 | 9.892 -> 9.520 ms (**-3.8%**), 7/9 | - | "NOT consistent" (same, and same sign) |
| same, unpinned repeat 1 | - | 4.906 -> 4.481 ms (**-8.7%**), 9/9 | "consistent" |
| same, unpinned repeat 2 | - | 4.407 -> 4.155 ms (**-5.7%**), 8/9 | "consistent" |

`uptime` load stayed ~3-5 across these 4 runs (quieter than the session's earlier ~11 peak). Raw
pairs show the same bimodal per-sample clustering already documented for `resize_cbcr` in this
workspace's `README.md`/`projects/pic-scale.md` (values splitting into a ~9ms and a ~14ms mode on
pinned core 3; a ~4.1-4.9ms and a ~6.8-8.2ms mode unpinned) for both baseline and patched binaries
alike.

Raw pairs (ms), pinned repeat 1:
`A: 9.8426 10.189 8.9007 4.4263 9.207 9.6067 10.434 9.353 9.2564` /
`B: 13.151 9.0485 8.7015 4.2852 9.5981 8.9567 8.3263 8.6764 8.7031`

Raw pairs (ms), pinned repeat 2:
`A: 10.276 14.121 14.6 9.363 9.8917 14.345 9.197 9.1952 9.5737` /
`B: 8.988 14.119 9.625 8.8807 12.03 12.149 9.52 9.1533 4.5047`

Raw pairs (ms), unpinned repeat 1:
`A: 5.0463 5.2461 4.9981 4.3924 4.9058 4.6626 4.7756 4.9774 4.3884` /
`B: 4.7078 5.185 4.1966 4.1012 4.6286 4.4806 4.59 4.0964 4.3241`

Raw pairs (ms), unpinned repeat 2:
`A: 4.4415 4.5351 4.3713 4.3768 4.3579 4.3603 4.4074 4.7721 8.2227` /
`B: 4.1554 4.1388 4.1002 4.1649 4.1375 4.1565 4.1524 6.8372 7.747`

## Result

The win holds at roughly the same magnitude as candidate 004's original narrow-filter
measurement (-4% to -7%): both unpinned repeats were decisively "consistent" (**-8.7% and -5.7%**
median wall-clock, 9/9 and 8/9). Both pinned (`taskset -c 3`) repeats agreed in direction and
similar magnitude (-6.9% and -3.8%) but landed just short of `paired.sh`'s strict 8/9-consistency
bar (7/9 both times) - read as the same host-level `resize_cbcr` noise this workspace has
documented for three prior candidates (002, 004, and now this one) rather than a real
regression, given both unpinned repeats and both pinned repeats agree on direction. Unlike
candidate 009's plane-u8 case (where pinned measurements were essentially directionless, -2.9% to
+7.7% across 4 repeats), this candidate's pinned repeats stayed consistently negative just below
the strictness bar - a weaker but not contradictory signal.

## Does it change behavior/output?

- **Bit-exact?** Yes. Candidate 004's own `bit_exact_check/` crate (copied unchanged into this
  candidate's folder, `Cargo.toml`'s `pic-scale` path-dependency repointed at this checkout - 13
  size/scale/strategy combinations sweeping tap counts through 1/2/4/8/16/17+ per output pixel),
  built once against pristine `cbcr8.rs`/`factory/cbcr8.rs`/`avx2/mod.rs` and once with candidate
  004's patch applied (same checkout, patch applied/reverted via `git apply`/`git apply -R` in
  between, `--release`): both runs printed the identical `FINAL_HASH=10798212667270458715` - the
  same hash candidate 004 recorded - and identical per-case `first8=[...]` samples.
- **Is this a breaking change?** No - internal dispatch only, no public API change. The new
  criterion case is bench-only, not part of the library's public surface.
- Project test suite (`cargo test -p pic-scale --features avx,sse,colorspaces,rdm --lib`): **66
  passed, 0 failed** before (pristine `cbcr8.rs`) and **66 passed, 0 failed** after (candidate
  004's patch applied) - identical counts, no new failures.

## Why the result looks the way it does

Same explanation candidate 008 already gave for the structurally related CbCr `u16` case: this
kernel's AVX2 win comes from *removing* an existing interleave-shuffle split (the SSE4.1 kernel's
8-tap step needed two separate 128-bit `shuffle_epi8`+`madd_epi16` pairs purely because of
128-bit register width, not because the 2-channel data needed splitting for correctness; AVX2
collapses that into one 256-bit shuffle+madd pair via lane-replicated control masks). That
instruction-count reduction happens once per 8-tap step regardless of how many such steps a given
output pixel needs, so widening the filter from `kernel_size~24` to `kernel_size~96` mostly means
the same per-step win applies ~4x as often, not that a new effect appears or disappears - matching
what both this candidate and candidate 008 (u16) observed, and unlike candidate 007/009's
`plane`/pure-width-doubling cases where pinned measurements were flat-out inconclusive rather than
just under the strictness bar.

## What's not done / open questions

- This closes the specific gap flagged in `IDEAS.md` for `resize_cbcr`/CN=2 u8 - all four CN x
  bit-depth-class combinations candidates 002-005 (u16 tiers) and 003/004 (u8 tiers) originally
  covered now also have a wide-filter recheck (007 plane-u16 null, 008 CbCr-u16 win-holds, 009
  plane-u8 mixed/unpinned-win, 010 this one CbCr-u8 win-holds).
- Not measured on hardware without AVX2 (dispatch already falls back to SSE4.1/scalar correctly
  there; no behavior change expected, not specifically re-verified this session).
- The new `"Pic scale CbCr8 wide filter: Lanczos 3"` bench case itself remains uncommitted
  (`bench_addition.diff` in this folder, reapplied by this candidate's own `justfile`) - per this
  workspace's rules, adding it (and candidate 004's still-uncommitted narrow-filter `"Pic scale
  CbCr8: Lanczos 3"` case) to the real `app/benches/resize_cbcr/main.rs` permanently is a
  separate, later decision for whoever reads this report.
- Neither pinned repeat cleared the strict 8/9 bar; a third or fourth pinned repeat (as candidate
  004 needed) might tip one over 8/9, but the two repeats already run agree closely enough in
  direction and magnitude with each other and with the unpinned result that a third repeat was not
  run this session given time budget.
