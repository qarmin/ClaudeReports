# Re-measuring candidate 006's AVX2 16-tap widening for planar `u16` under a genuinely wide Lanczos3 filter window

Candidate 006 widened `src/avx2/plane_u16_lb.rs`'s horizontal-convolution kernel (`Row4ExecutionHandler`
and `OneRowExecutionUnit`, `bit_depth <= 12`) from an 8-tap 128-bit `madd_epi16` step to a 16-tap
256-bit one, but measured a null result on the only existing criterion case that reaches this path
(`"Pic scale Plane10: Lanczos 3"`, `PreferSpeed`, 1/4 downscale). `IDEAS.md` flagged that this case's
filter width (`bounds_size`, computed as `min_kernel_size(3) * 2 * scale` for Lanczos3, where
`scale = in_size/out_size`) tops out at `kernel_size = 24` for a 4x downscale - confirmed directly
this session with a throwaway unit test on `generate_weights`: `kernel_size=24`, per-row `bounds.size`
mostly 24 (min 14 at the edges). That means the new 16-tap loop fires at most once per output pixel
(`jx=0..16`), with the remaining 0-8 taps handled by the unchanged 8-tap tail loop - not enough to
tell whether the widening is a real win once it actually dominates the work. This candidate adds a
new criterion case with a much larger downscale ratio (1/16 instead of 1/4) to push `kernel_size` to
96 (confirmed via the same throwaway probe: `kernel_size=96`, most rows at 96, edges as low as 56/57)
- the 16-tap loop now fires ~6 times per output pixel - and re-measures candidate 006's unmodified
patch (`patch.diff`, byte-identical to `006.AVX2_PLANE_U16_LB_HORIZONTAL/patch.diff`) against it.

## What was tried

Only one variant: candidate 006's existing AVX2 16-tap widening, unchanged, measured against a new
bench case (`bench_addition.diff`, adds `"Pic scale Plane10 wide filter: Lanczos 3"` to
`app/benches/resize_plane/main.rs` - `bit_depth=10`, `WorkloadStrategy::PreferSpeed`,
`ThreadingPolicy::Single`, nasa asset resized to 1/16 in both dimensions). `app/Cargo.toml`'s
`pic-scale` feature list was given `"avx", "sse"` (not part of either diff, reverted after
measurement, same as every other candidate in this workspace - see `README.md`). Two prebuilt
`resize_plane` bench binaries (baseline = pristine `src/avx2/plane_u16_lb.rs` + this candidate's
new bench case; patched = baseline + candidate 006's `plane_u16_lb.rs` patch) were compared via
`tools/paired.sh` (`RUNS=9`, alternating A/B, point estimate = criterion's reported mean), run from
`app/` (the bench opens `../assets/...` relative to cwd).

| Variant | single-core (`taskset -c 3`), median of 9 | multi-core (unpinned), median of 9 | notes |
|---|---|---|---|
| baseline vs. AVX2-16-tap (candidate 006's patch), pinned repeat 1 | 4.110 -> 3.890 ms (**-5.3%**), B faster in 7/9 | - | "NOT consistent" |
| same, pinned repeat 2 | 8.160 -> 8.063 ms (**-1.2%**), B faster in 4/9 | - | "NOT consistent" |
| same, unpinned repeat 1 | - | 4.006 -> 4.025 ms (**+0.5%**), B faster in 5/9 | "NOT consistent" |
| same, unpinned repeat 2 | - | 4.097 -> 4.011 ms (**-2.1%**), B faster in 6/9 | "NOT consistent" |

`uptime` load ranged ~2.5-4.6 across the 4 runs. Pinned repeat 2 landed mostly in this box's
already-documented ~8ms bimodal noise mode (see `README.md`/candidate 006's `RESULT.md`); the other
3 runs stayed mostly in the ~4ms mode - visible directly in the raw pairs below.

Raw pairs (ms), pinned repeat 1:
`A: 4.0337 3.9901 4.0364 4.1152 4.1795 4.1096 4.0347 4.1238 4.4009` /
`B: 3.8132 3.9113 3.8916 3.7964 4.944 3.8898 3.8408 3.8173 5.1999`

Raw pairs (ms), pinned repeat 2:
`A: 4.2124 8.6725 8.1604 8.2854 8.8899 8.0413 8.2332 3.8468 3.8698` /
`B: 7.9978 8.5283 8.4913 3.7204 8.3382 8.1562 8.0627 3.8598 4.4339`

Raw pairs (ms), unpinned repeat 1:
`A: 3.7452 4.5593 4.0435 3.8273 4.0592 4.0058 3.7967 4.3509 3.9136` /
`B: 4.2508 3.8545 3.8944 4.0254 3.8922 3.9456 4.6941 4.2006 4.1169`

Raw pairs (ms), unpinned repeat 2:
`A: 4.2178 4.0669 3.9325 4.0974 4.0102 4.3195 3.9992 4.1473 4.4232` /
`B: 4.0084 4.0302 3.9097 3.8684 4.0106 4.7915 4.0038 4.0705 4.4047`

A single non-alternating `cargo bench` invocation of the patched binary right after building it
(criterion comparing against its own saved sample from the immediately preceding baseline run, not
a paired A/B run) reported "change: -8.79%, p=0.00, Performance has improved" - included here for
completeness, but per `MEASUREMENT.md`'s paired-comparison discipline this single unpaired
before/after is not treated as evidence on its own; the 4 `paired.sh` runs above are the actual
measurement.

## Result

No measurable change, even with the filter window now wide enough (`kernel_size=96`, the 16-tap
loop firing ~6 times per output pixel) to settle the open question from candidate 006/`IDEAS.md`.
All 4 `paired.sh` runs (2 pinned, 2 unpinned) reported "NOT consistent" (B faster in 4-7 of 9 pairs,
i.e. not a decisive majority either way), point-estimate deltas ranging from -5.3% to +0.5% - all
within this box's documented noise floor for this bench (medians alone swung between the ~4ms and
~8ms noise modes across repeats of the *same* binary, same as candidates 002/004/006 saw on
`resize_cbcr`/`resize_plane`). This settles candidate 006's open question: the widening is a genuine
null result for CN=1 `u16` low-bit-depth, not an artifact of the original bench under-exercising the
new loop.

## Does it change behavior/output?

- **Bit-exact?** Yes. Candidate 006's own `bit_exact_check/` crate (copied unchanged into this
  candidate's folder), pointed at this session's checkout: built once against pristine
  `plane_u16_lb.rs` and once with candidate 006's patch applied (same checkout, file swapped via
  `git checkout`/`git apply` in between, `--release`), both runs printed the identical
  `FINAL_HASH=10119632667376043265` (the same hash candidate 006 recorded) and identical per-case
  `first8=[...]` samples across all 15 synthetic size/scale/bit-depth/strategy combinations.
- **Is this a breaking change?** No - internal dispatch only, no public API change. The new
  criterion case is bench-only, not part of the library's public surface.
- Project test suite (`cargo test -p pic-scale --features avx,sse,colorspaces,rdm --lib`): **66
  passed, 0 failed** before (pristine `plane_u16_lb.rs`) and **66 passed, 0 failed** after
  (candidate 006's patch applied) - identical counts, no new failures.

## Why the result looks the way it does

Confirmed directly this session (a throwaway `#[cfg(test)]` probe on `generate_weights`, reverted
before this candidate's own measurement): the existing `"Pic scale Plane10: Lanczos 3"` case's
`kernel_size` is 24 (4x downscale, Lanczos3's `min_kernel_size=3` support radius scaled by the
downscale ratio), so the new 16-tap loop fires 0-1 times per output pixel there - matching candidate
006's suspicion exactly. This candidate's new case (`1/16` downscale) pushes `kernel_size` to 96, so
the loop now fires ~6 times per pixel - a genuinely dominant share of the horizontal convolution's
work - and *still* shows no measurable difference. This confirms candidate 006's own explanation for
its null result was about the *shape* of the kernel, not just how often it fires: planar (1-channel)
`u16`'s existing 8-tap SSE step was already a single un-split 128-bit `_mm_dot16_avx_epi32` call (no
interleave shuffle to collapse, unlike CbCr's 2-channel case in candidate 005), so doubling it to a
256-bit 16-tap step is a pure width doubling with no instruction-count reduction underneath - and a
pure width doubling with the same total scalar work apparently doesn't move the needle on this
CPU/kernel shape even when it dominates the loop, likely because the same number of taps still needs
the same number of source loads/multiplies/adds either way (256-bit vs. two 128-bit lanes), and
`_mm256_dot16_avx_epi32`'s internal shuffle-and-widen cost may offset whatever's saved from fewer
loop-overhead instructions.

## What's not done / open questions

- This closes the specific gap flagged in `IDEAS.md` for `resize_plane`/CN=1 `u16` - the same gap
  is still open for `resize_cbcr` per that file's other still-backlogged row about CbCr u16's
  `PreferQuality`-scale case (candidate 005 only checked correctness there via ad hoc
  `bit_exact_check` cases, not a criterion case), though CbCr's kernel shape is different (has the
  interleave-shuffle split candidate 005's win came from) so this candidate's null result doesn't
  necessarily transfer.
- Not measured on hardware without AVX2 (dispatch already falls back correctly there; no behavior
  change expected, not specifically re-verified this session).
- The new `"Pic scale Plane10 wide filter: Lanczos 3"` bench case itself remains uncommitted
  (`bench_addition.diff` in this folder, reapplied by this candidate's own `justfile`) - per this
  workspace's rules, adding it to the real `app/benches/resize_plane/main.rs` permanently is a
  separate, later decision for whoever reads this report.
