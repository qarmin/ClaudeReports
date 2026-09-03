# RESIZE_THUMBNAIL_UNIFORM_COLOR_TEST_GAP

Backlog item from `IDEAS.md` (top row, sourced from candidate 009): candidate 009 added a
"uniform input stays uniform" regression test only for `imageops::resize`'s `vertical_sample`
path (`src/imageops/sample.rs`), where it caught candidate 003's confirmed `+1.0` accumulator
bias. This candidate checks whether the *other*, separate resize/thumbnail code paths -
`DynamicImage::resize`/`resize_exact`/`resize_to_fill` (delegated to the external `pic-scale-safe`
crate via `src/imageops/resize.rs`'s `resize_impl`) and `imageops::thumbnail`/
`DynamicImage::thumbnail_exact` (`src/imageops/sample.rs`'s own, third, independent
implementation) - have equivalent coverage, or would silently pass an analogous bug the way
`vertical_sample` did before candidate 003's fix.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grep for any existing "uniform"/invariant-style test covering these specific APIs in `src/imageops/*.rs` and `src/images/dynimage.rs` | `grep -rn uniform ...`, before any change | No hits besides candidate 009's own test on the unrelated `vertical_sample` path - confirms the coverage gap itself is real: no dedicated regression test exists for `resize_impl`, `resize_to_fill`, `thumbnail`, or `thumbnail_exact`. |
| Manual boundary-value/differential-invariant construction (per `HEURISTICS.md`'s "Logic bugs" guidance): added temporary `#[test]` fns directly to `src/imageops/resize.rs`'s `mod tests`, exercising `DynamicImage::resize`/`resize_exact`/`resize_to_fill`, `imageops::thumbnail`, and `DynamicImage::thumbnail_exact` on a uniform-color source across 5 `FilterType`s x 6 size/aspect-ratio combinations (upscale, downscale, extreme 2x200->50x50, 1x1->4x4, 4x4->1x1) x 2 alpha values (opaque and 128, to exercise the premultiply/unpremultiply path) for `Rgba<u8>`, in-process | `cargo test --lib scratch_uniform_color_probe`, debug build, 1 run | 0/many mismatches - every combination preserved the uniform color exactly. |
| Same approach extended to `Rgba<f32>` (the pixel type candidate 003's own `RESULT.md` flagged as affected *unconditionally and unclamped* by that bug class) and `Rgba<u16>` through `resize_exact` | `cargo test --lib scratch_uniform_color_probe_f32_u16`, debug build, 1 run | 0/many mismatches (f32 compared with a 1e-4 tolerance for float rounding; u16 compared exactly). |
| Same approach extended to `LumaA<u8>` (a different `has_constant_alpha`/premultiply branch than RGBA) through `resize_exact` and `thumbnail` | `cargo test --lib scratch_uniform_color_probe_luma`, debug build, 1 run | 0/many mismatches. |
| Standalone `repro/` (own `Cargo.toml`, real public API `image::{DynamicImage, imageops::thumbnail}`, mirrors all of the above checks in one binary: 30 filter/size combinations x the RGBA8/f32/u16/LumaA8 pixel-type sweep) run via `just test` end-to-end against a pristine local clone pinned at `base_commit` | `just test` (clone -> build+run release), then a second bare `cargo run --release` repeat inside the same clone | Matches the in-tree runs: `Not reproduced` both times, 0 mismatches out of 30 combinations x pixel-type sweep. See `output.txt`. |

This is a manual boundary-value/differential-invariant check (no sanitizer/miri/fuzzing needed -
same category as candidate 005's sibling-path sweep for the same bug class). The soft cap of 3
escalating approaches was reached (u8 RGBA incl. alpha/premultiply, f32/u16, LumaA8) with zero
signal at every step, so digging stopped there per `SKILL.md`'s "3 clean negatives means move on."

## Status

**Not reproducible.** The test-coverage gap itself is real and confirmed (no test in the current
suite asserts this invariant for `resize_impl`/`resize_to_fill`/`thumbnail`/`thumbnail_exact`),
but unlike candidate 009's `vertical_sample` case, digging found no live bug hiding behind that
gap in these specific code paths: a broad sweep (5 filters x 6 size/aspect-ratio combinations x
u8-opaque/u8-alpha/f32/u16/LumaA8 pixel types = well over 100 individual pixel/parameter
combinations checked across both the in-tree probe and the standalone `repro/`) found the
uniform-color invariant held in every case, reproducibly (2/2 repro runs, 3/3 in-tree test runs
across the three probe files added incrementally). Per `SKILL.md`'s candidate loop step 3, a
candidate with no reproduction is recorded as `Not reproducible`, not as a `Confirmed`
test-coverage finding the way candidate 009 was - 009's confirmation rested specifically on
demonstrating the gap already let a known, real bug through; this session found no equivalent bug
for these APIs to demonstrate the same way.

## Root cause

N/A (nothing reproduced). Why these paths turned out safe, read directly:

- `DynamicImage::resize`/`resize_exact` (`src/imageops/resize.rs::resize_impl`) delegates all the
  actual per-pixel weighted-average math to the external `pic-scale-safe` crate (out of this
  repo's own source, not swept here) - this repo's own glue code around it (dispatch by color
  type, `premultiply_alpha`/`unpremultiply_alpha`, `has_constant_alpha`) does no accumulation of
  its own that could introduce an analogous seed-value bug; it only forwards raw sub-pixel slices
  and reassembles the returned `Vec` into an `ImageBuffer`.
- `imageops::thumbnail` (`src/imageops/sample.rs`) was already checked for this exact bug class in
  candidate 005 (`ThumbnailSum::zeroed()` - correctly zero-seeded, confirmed via a separate
  differential check) - this candidate's own broader sweep (more filters/sizes/pixel types/alpha
  values, run through the public `DynamicImage`/`thumbnail_exact` entry points rather than the
  free function directly) is consistent with that earlier finding and extends it to
  `thumbnail_exact`, f32/u16/LumaA8, and the alpha-premultiply path, none of which candidate 005
  covered.
- `resize_to_fill` (`DynamicImage::resize_to_fill`) composes `resize_exact` + a crop; since
  `resize_exact` preserves uniform color and cropping a uniform-color image trivially preserves
  it, no separate accumulator exists in this path to seed wrong.

## The fix

No fix - nothing to fix, since no bug reproduced. A test-suite improvement (adding a permanent
`test_resize_thumbnail_uniform_color_preserved`-style regression test to `src/imageops/resize.rs`
and/or `src/imageops/sample.rs`, mirroring candidate 009's) would still be worthwhile purely as
*prevention* going forward, but per `SKILL.md`'s candidate loop a fix is only written up and kept
in `patch.diff` when Confirmed - this candidate is `Not reproducible`, so the temporary probe
`#[test]` fns used during digging were reverted rather than kept as a proposed patch. See "What's
not done" below for this as an explicit recommendation instead.

- Project test suite: not run before/after, since no change was made to the main project's working
  tree (temporary probe tests were added and reverted within the same digging session; `git
  status`/`git diff --stat` confirmed a clean tree before writing this file up - N/A, no patch to
  verify).

## What's not done / open questions

- **Recommendation, not a finding**: a permanent uniform-color-preservation regression test for
  `DynamicImage::resize`/`resize_exact`/`resize_to_fill`/`imageops::thumbnail`/`thumbnail_exact`
  (mirroring candidate 009's `test_resize_uniform_color_preserved`) would still close the
  *coverage* gap itself (distinct from "is there a bug today") and guard against a future
  regression in either this repo's glue code or a future `pic-scale-safe` version. Not added this
  session since there is no currently-failing case to justify writing it up as a `Confirmed`
  candidate with a `patch.diff`, per the distinction candidate 009 drew.
- Not checked: `pic-scale-safe`'s own internal test suite/source (the crate doing the actual
  per-pixel math for `resize_impl`) - out of this repo's own source sweep by design (see
  candidate 008's same scoping note), so this candidate only rules out a bug in *this repo's*
  glue code around it, not in `pic-scale-safe` itself for input/parameter combinations not tried
  here (e.g. much larger images, `usize`-adjacent dimensions, other `ResamplingFunction`
  internals not exercised by the 5 `FilterType` variants this crate exposes).
- Not checked: `Luma8`/`Luma16`/`Luma32F`/`Rgb16` single-channel-without-alpha pixel types through
  these same APIs (only `LumaA8`, `Rgba8`, `Rgba16`, `Rgba32F` were probed) - these all take the
  `has_constant_alpha => true` trivial branch in `resize.rs` (no premultiply ever attempted), so
  they were considered lower-risk and not included in the sweep to keep this candidate bounded.
