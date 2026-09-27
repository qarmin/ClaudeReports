# RESIZE_NO_ALPHA_TYPES_RANDOMIZED_UNIFORM_COLOR_SWEEP

Backlog item from `IDEAS.md` (top row, sourced from candidate 010): candidate 010 already swept
`DynamicImage::resize`/`resize_exact`/`resize_to_fill`/`imageops::thumbnail`/`thumbnail_exact` for
uniform-color-input preservation across a fixed grid of 5 filters x 6 sizes x u8 RGBA
(opaque+alpha)/f32 RGBA/u16 RGBA/LumaA8, in-tree and via a standalone `repro/`, without finding a
live bug, and explicitly left two things unchecked in its own "What's not done": the no-alpha
color types (`Luma8`/`Luma16`/`Rgb16`/`Luma32F`/`Rgb32F` - the trivial `has_constant_alpha=>true`
branch in `src/imageops/resize.rs`, never premultiplied) and a randomized (rather than
fixed-grid-only) sweep. This candidate closes both gaps.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `src/imageops/resize.rs`'s `resize_impl`/`premultiply_alpha`/`has_constant_alpha` dispatch to confirm which color types take the untested no-alpha branch, and reread `has_constant_alpha_integer`'s XOR-sum trick (candidate 010 didn't examine it) to rule out a false "constant alpha" classification | Manual code read | `Luma8`/`Rgb8`/`Luma16`/`Rgb16`/`Luma32F`/`Rgb32F` unconditionally report `has_constant_alpha() == true` (no per-pixel channel to vary) and never call `premultiply_alpha`/`unpremultiply_alpha` - these 5 types were genuinely unswept by candidate 010 (which only covered u8/f32/u16 RGBA and LumaA8). `has_constant_alpha_integer`'s row-summed-XOR-into-`u64` check is sound: XOR of unsigned subpixels is always non-negative, so a non-zero per-pixel divergence can never cancel to a zero sum - not a source of false negatives. |
| Manual boundary-value/differential-invariant construction (per `HEURISTICS.md`'s "Logic bugs" guidance): temporary `#[test]` fns added to `src/imageops/resize.rs`'s `mod tests`, exercising `DynamicImage::resize_exact` for `Luma8`/`Luma16`/`Rgb16`/`Luma32F`/`Rgb32F` on a uniform-color source across 5 `FilterType`s x 6 size/aspect-ratio combinations (same grid candidate 010 used for the RGBA sweep), in-process | `cargo test --lib scratch_uniform_color_probe_no_alpha_types`, debug build, 3 repeat runs | 0/150 mismatches, 3/3 identical runs - uniform color preserved exactly for every combination. |
| Randomized property sweep (escalation beyond the fixed grid): 500 iterations of random source/destination size (1-40), random filter, and random `Rgba8` color through `resize_exact`, via a temporary `#[test]` fn using a seeded xorshift32 PRNG (no external RNG dependency) | `cargo test --lib scratch_uniform_color_probe_randomized`, debug build, 3 repeat runs | 0/500 mismatches, 3/3 identical runs. |
| Standalone `repro/` (own `Cargo.toml`, real public API `image::{DynamicImage, imageops::FilterType}`, mirrors both probes above: 30 fixed-grid no-alpha-type combinations + 500 randomized `Rgba8` combinations) run via `just test` end-to-end against a pristine local clone pinned at `base_commit` | `just test` (clone -> build+run release), then a second bare run of the same binary | Matches the in-tree runs: `Not reproduced` both times, 0 mismatches out of 530 combinations. See `output.txt`. |

This is a manual boundary-value/differential-invariant check plus a short randomized property
sweep (per `HEURISTICS.md`'s "Logic bugs" guidance - no sanitizer/miri/fuzzing needed, same tool
class as candidates 005/010). The soft cap of 3 escalating approaches was reached (code read to
scope the untested branch, fixed-grid no-alpha-type sweep, randomized sweep) with zero signal at
every step, so digging stopped there per `SKILL.md`'s "3 clean negatives means move on" - combined
with candidate 010's own independent 3-approach sweep of the alpha-bearing types, this is the
second consecutive clean sweep of this same suspect.

## Status

**Not reproducible.** The two gaps candidate 010 left open (no-alpha color types, randomized
rather than fixed-grid-only coverage) are now both checked and remain clean: 150 fixed-grid
combinations across `Luma8`/`Luma16`/`Rgb16`/`Luma32F`/`Rgb32F` and 500 randomized `Rgba8`
combinations all preserved a uniform source color exactly through `resize_exact`, reproducibly
(3/3 in-tree runs for each of the two new probe tests, plus 2/2 runs of the standalone `repro/`
against a pristine clone). Per `SKILL.md`'s candidate loop step 3, a candidate with no
reproduction is recorded as `Not reproducible`.

## Root cause

N/A (nothing reproduced). The no-alpha color types take a strictly simpler path than the
alpha-bearing types candidate 010 already cleared: `has_constant_alpha` returns `true`
unconditionally for them (no per-pixel alpha channel to compare), so `resize_impl` never calls
`premultiply_alpha`/`unpremultiply_alpha` for these types and forwards the raw samples straight to
`pic-scale-safe`'s `resize_plane8`/`resize_plane16`/`resize_rgb16`/`resize_plane_f32`/
`resize_rgb_f32` with no extra accumulation step of this repo's own that could introduce an
analogous seed-value bias - consistent with candidate 010's finding for the alpha-bearing
siblings that share the same `resize_impl` dispatch.

## The fix

No fix - nothing to fix, since no bug reproduced. As candidate 010 already noted, a permanent
`test_resize_uniform_color_preserved`-style regression test covering these APIs (now including the
no-alpha types checked here) would still be worthwhile purely as prevention, but per `SKILL.md`'s
candidate loop a fix/test is only written up and kept as a lasting artifact when a candidate is
`Confirmed` against a real bug - this candidate is `Not reproducible`, so the temporary probe
`#[test]` fns used during digging were reverted rather than kept. The main project's working tree
was confirmed clean (`git status`/`git diff --stat`) before this file was written.

- Project test suite: not run before/after, since no change was made to the main project's working
  tree (temporary probe tests were added and reverted within the same digging session - N/A, no
  patch to verify).

## What's not done / open questions

- **Recommendation, not a finding** (repeated from candidate 010, now covering the full pixel-type
  matrix): a permanent uniform-color-preservation regression test for
  `DynamicImage::resize`/`resize_exact`/`resize_to_fill`/`imageops::thumbnail`/`thumbnail_exact`
  across all 12 `DynamicImage` color-type variants (mirroring candidate 009's
  `test_resize_uniform_color_preserved`) would close the *coverage* gap itself and guard against a
  future regression in this repo's glue code or a future `pic-scale-safe` version bump. Not added
  this session for the same reason as candidate 010: no currently-failing case to justify a
  `Confirmed` candidate with a `patch.diff`.
- Not checked: `pic-scale-safe`'s own internal implementation (out of this repo's own source sweep
  by design, same scoping as candidates 008/010) for input/parameter combinations well outside
  this candidate's random-size range (1-40) - e.g. very large images, dimensions adjacent to
  `usize`/`u32` limits, or other `ResamplingFunction` internals.
- Not checked: `imageops::thumbnail`/`thumbnail_exact` for the no-alpha types specifically (this
  candidate focused the new coverage on `resize_exact`, since `thumbnail`'s own averaging path
  was already checked for the accumulator-seed bug class by candidate 005 and, for `LumaA8`, by
  candidate 010) - a `Luma8`/`Rgb16`/etc. sweep through `thumbnail` specifically would be a small,
  low-priority follow-up rather than a new suspect.
- Not checked: a spatially-*varying*-alpha invariant (as opposed to uniform color under constant
  alpha) - e.g. whether a uniform RGB color with per-pixel-varying alpha round-trips through
  premultiply/resize/unpremultiply without RGB drift near low-alpha regions. This is a distinct,
  weaker invariant (blending with varying alpha can legitimately shift intermediate values even in
  a correct implementation) and wasn't swept by either this candidate or candidate 010; noted here
  as a possible future suspect rather than investigated.
