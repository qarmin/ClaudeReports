# Re-measuring candidate 005's AVX2 CbCr16 LB horizontal win under a genuinely wide (PreferQuality-scale, 16+ tap) Lanczos3 filter

Candidate 005 added `src/avx2/cbcr_u16_lb.rs` (an AVX2 tier for `src/factory/cbcr16.rs`'s
`bit_depth <= 12` x86_64 path) and measured a large win (-39% to -46%) on the existing
`app/benches/resize_cbcr/main.rs` `"Pic scale CbCr10: Lanczos 3"` case, which only resizes to 1/4
size - a filter width (`kernel_size`) that, per this same math (`generate_weights` in
`src/math/weights.rs`, confirmed directly this session with a throwaway `#[cfg(test)]` probe on
`generate_weights`, reverted before this candidate's own measurement) tops out around 24 taps for
a 4x downscale, so the AVX2 kernel's new 16-tap main loop fires at most once per output pixel
there. `IDEAS.md` flagged this as untested for CbCr specifically, after candidate 007 found the
*structurally different* plane (CN=1) `u16` 16-tap widening (candidate 006) to be a genuine null
result once actually re-measured under a wide filter (`kernel_size=96`) - but noted CbCr's kernel
has a different shape (an interleave-shuffle split the win was tied to) so that null result does
not necessarily transfer. This candidate closes that gap: it adds a new criterion case
(`"Pic scale CbCr10 wide filter: Lanczos 3"`, same as the existing `"Pic scale CbCr10: Lanczos 3"`
case but downscaling to 1/16 instead of 1/4) and re-measures candidate 005's unmodified patch
against it.

Confirmed directly this session (throwaway probe on `generate_weights` with `in_size=4928,
out_size=4928/16`): `kernel_size=96`, `bounds[0].size=56`, last row's `bounds.size=56` - i.e. the
16-tap AVX2 main loop now fires ~6 times per output pixel, the same magnitude candidate 007 used
to settle the plane case.

## What was tried

Only one variant: candidate 005's existing AVX2 kernel + factory wiring, unchanged
(`patch.diff`, byte-identical to `005.AVX2_CBCR16_LB_HORIZONTAL/patch.diff` - includes candidate
002's still-unmerged SSE4.1 kernel as a prerequisite, same as 005), measured against a new bench
case (`bench_addition.diff`, adds `"Pic scale CbCr10 wide filter: Lanczos 3"` to
`app/benches/resize_cbcr/main.rs` - `bit_depth=10`, `WorkloadStrategy::PreferSpeed`,
`ThreadingPolicy::Single`, nasa asset resized to 1/16 in both dimensions, otherwise identical to
the existing `"Pic scale CbCr10: Lanczos 3"` case). `app/Cargo.toml`'s `pic-scale` feature list
was given `"sse"` only for the baseline build (SSE4.1-only, isolating this candidate's own AVX2
contribution the same way candidate 005 did) and `"sse", "avx"` for the patched build. Two
prebuilt `resize_cbcr` bench binaries were compared via `tools/paired.sh` (`RUNS=9`, alternating
A/B, point estimate = criterion's reported mean), run from `app/` (the bench opens
`../assets/...` relative to cwd).

| Variant | single-core (`taskset -c 3`), median of 9 | multi-core (unpinned), median of 9 | notes |
|---|---|---|---|
| baseline (SSE4.1 only) vs. AVX2 (candidate 005's patch), pinned repeat 1 | 20.161 -> 11.152 ms (**-44.7%**), 9/9 | - | "consistent" |
| same, pinned repeat 2 | 40.303 -> 21.901 ms (**-45.7%**), 9/9 | - | "consistent" |
| same, unpinned repeat 1 | - | 20.634 -> 10.935 ms (**-47.0%**), 9/9 | "consistent" |
| same, unpinned repeat 2 | - | 21.351 -> 11.223 ms (**-47.4%**), 9/9 | "consistent" |

`uptime` load ranged ~2-11 across the 4 runs (this box's already-documented noise for
`resize_cbcr` - see `README.md`); pinned repeat 2's `A` samples bimodally split between a ~21ms
and a ~41ms mode (visible in the raw pairs below), the same host-level clustering behavior
`IDEAS.md` already flags for this bench - it did not affect the paired win/loss count (still
9/9).

Raw pairs (ms), pinned repeat 1:
`A: 20.022 20.161 19.757 19.919 21.135 19.816 41.024 40.878 41.166` /
`B: 10.694 10.694 10.82 10.778 11.152 18.906 22.284 20.106 22.348`

Raw pairs (ms), pinned repeat 2:
`A: 43.049 40.303 31.639 35.963 41.351 42.727 41.246 20.619 20.878` /
`B: 21.901 22.134 22.102 22.549 19.674 22.046 19.058 10.868 11.467`

Raw pairs (ms), unpinned repeat 1:
`A: 20.797 20.49 20.248 20.483 20.607 21.437 21.458 20.634 20.711` /
`B: 10.791 10.814 10.851 10.894 10.957 10.935 11.038 10.955 10.996`

Raw pairs (ms), unpinned repeat 2:
`A: 20.952 21.504 23.185 20.322 27.279 20.55 19.889 21.351 21.762` /
`B: 11.223 12.369 10.985 15.153 14.328 10.693 12.044 10.656 11.097`

## Result

The win holds, and is if anything slightly larger than under the narrow filter: **-44.7% to
-47.4% median wall-clock** across all 4 `paired.sh` runs (2 pinned, 2 unpinned), every single one
landing 9/9 "consistent" - a tighter, more decisive spread than candidate 005's own original
narrow-filter measurement (-39% to -46%, with one pinned run only reaching 8/9). This settles the
open question from `IDEAS.md`: unlike candidate 007's plane (CN=1) `u16` case, where widening the
filter to the same `kernel_size=96` magnitude made no measurable difference to an already-null
result, CbCr (2-channel) `u16`'s AVX2 win is not an artifact of the original narrow-filter bench
under-exercising the new loop - it holds at essentially the same (slightly larger) magnitude once
the 16-tap loop dominates the work.

## Does it change behavior/output?

- **Bit-exact?** Yes. Candidate 005's own `bit_exact_check/` crate (copied unchanged into this
  candidate's folder, `Cargo.toml`'s `pic-scale` path-dependency repointed at this checkout),
  which already sweeps `bounds_size` past 16 taps/output pixel via its own extreme-downscale
  synthetic cases (independent of this candidate's new criterion bench case), built against the
  current working tree (patch applied, plus this candidate's own bench addition, which does not
  affect the library itself) and printed `FINAL_HASH=12406396843747482019` - the identical hash
  candidate 005 recorded.
- **Is this a breaking change?** No - internal dispatch only, no public API change. The new
  criterion case is bench-only, not part of the library's public surface.
- Project test suite (`cargo test -p pic-scale --features avx,sse,colorspaces,rdm --lib`): **66
  passed, 0 failed** with candidate 005's patch applied, and **66 passed, 0 failed** with it
  reverted (pristine `cbcr16.rs`/`avx2/mod.rs`/`sse/mod.rs`, no `cbcr_u16_lb.rs` files) - identical
  counts, no new failures either way.

## Why the result looks the way it does

This is the mirror image of candidate 007's finding for plane (CN=1) `u16`, and for the same
underlying reason candidate 007 already identified: candidate 005's AVX2 win was tied to
*removing* CbCr's existing interleave-shuffle split (the SSE4.1 kernel's 8-tap step was
artificially split into two separate 128-bit shuffle+madd pairs purely because SSE registers are
128 bits, not because the 2-channel data needed splitting - AVX2 collapses that into one
256-bit shuffle+madd pair), not from SIMD width alone. That instruction-count halving happens
once per 8-tap step regardless of how many 8-tap (or 16-tap) steps a given output pixel needs -
so widening the filter from `kernel_size~24` to `kernel_size~96` just means the same per-step win
gets multiplied by roughly 4x as many steps, not that a new effect kicks in. Plane's `u16` case
had no such split to remove (its existing SSE kernel already used one un-split 128-bit
`_mm_dot16_avx_epi32` call), so its AVX2 widening was a pure register-width doubling with no
instruction-count reduction underneath - and that class of change apparently doesn't help on this
CPU/kernel shape (candidate 007), while CbCr's actually-fewer-instructions-per-tap change does,
consistently, at both filter widths (candidates 005 and this one).

## What's not done / open questions

- This closes the specific gap flagged in `IDEAS.md` (CbCr u16's `PreferQuality`-scale/wide-filter
  case) - both `resize_plane` (candidate 007) and `resize_cbcr` (this candidate) now have a real
  criterion-measured answer for their respective AVX2 16-tap widenings under a genuinely wide
  filter, and the two answers differ (null for plane, still-strong-win for CbCr) for a reason now
  tied to kernel shape, not filter width, on both sides.
- Not measured on hardware without AVX2 (dispatch already falls back to SSE4.1/scalar correctly
  there; no behavior change expected, not specifically re-verified this session).
- The new `"Pic scale CbCr10 wide filter: Lanczos 3"` bench case itself remains uncommitted
  (`bench_addition.diff` in this folder, reapplied by this candidate's own `justfile`) - per this
  workspace's rules, adding it to the real `app/benches/resize_cbcr/main.rs` permanently is a
  separate, later decision for whoever reads this report.
- Only checked the `bit_depth<=12` ("LB") path specifically (same path candidate 005's kernel
  targets) - the existing `"Pic scale CbCr16: Lanczos 3"` bench case (`bit_depth=16`, a different,
  floating-point horizontal path with no AVX2 kernel of its own) was not touched by this candidate
  and remains out of scope, same as it was for candidate 005.
