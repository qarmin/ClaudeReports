# Lut::new_rescale_and_normalize: fuse the two-pass min/max scan into one

`Lut::new_rescale_and_normalize` (`pixeldata/src/lut.rs`, used whenever `VoiLutOption::Normalize`
is requested explicitly, or implicitly whenever a file has no VOI LUT/window center-width at all
and the default `VoiLutOption::Default` path is taken) scans every decoded pixel sample **twice**:
once via `.clone().fold(..)` for the min, once via `.fold(..)` for the max, over the same
`filter_map`'d `f64` iterator. Noticed as an unaddressed spillover from candidate 012 (already
flagged in `IDEAS.md`), which only fixed `Lut::new_with_fn`'s table-construction dispatch, not
this separate, unguarded, sequential-only per-*sample* fold in the same file.

## What was tried

Single change: replace the two separate `.fold()` calls with one `.fold()` producing a `(min,
max)` tuple in a single pass over `samples_f64`, removing the need for `.clone()` on the iterator
entirely.

Harness: added a new `colorspace-normalize <file> <iters>` mode to
`___OPTIMIZATIONS___/harness/bench_pipeline` (`ConvertOptions::new().with_voi_lut(VoiLutOption::
Normalize)` + `to_dynamic_image_with_options`), since none of the existing modes forced this VOI
LUT path for a fixture that already has its own window center/width. Fixture: `testdata/
mono_above.dcm` (reused from candidate 006 - MONOCHROME2, 2048x512 = 1,048,576 pixels, no window
tags of its own).

| Variant | single-core p50 (`taskset -c 0`) | multi-core p50 | notes |
|---|---|---|---|
| baseline (two-pass fold) | 1.839 ms | 1.830 ms / 1.697 ms (2 rounds) | |
| patched (single-pass fold) | 1.555 ms | 1.499 ms / 1.384 ms (2 rounds) | |

## Result

**-15.4% single-core, -18.1%/-18.4% multi-core across two independent rounds, 9/9 paired wins in
all three rounds, fully consistent.** `uptime` load average was 2.1-2.2 throughout (some
background load present, but the paired alternating-run discipline cancels it out - see the 9/9
consistency).

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum=` (FNV-1a of the decoded+converted image bytes) is identical
  between baseline and patched builds on `mono_above.dcm`. This is expected, not just observed:
  min/max reduction over a fixed set of `f64` values is invariant to how the fold is split or
  ordered (unlike a sum, which can accumulate different rounding depending on association order) -
  fusing the two passes changes nothing about which value ends up as the min or the max.
- **Is this a breaking change?** No public API/type changes. `new_rescale_and_normalize` is
  `pub(crate)`, not exposed outside the `dicom-pixeldata` crate.
- Project test suite: `cargo test --release -p dicom-pixeldata`: **51 passed / 0 failed / 2
  ignored, both before and after** (39+1+1+10 lib/doctest split, unchanged). `cargo test --release
  --workspace`: **0 failures anywhere, both before and after** (patched build checked; baseline
  confirmed separately via the same suite before the patch was applied).

## Why the result looks the way it does

Halving the number of passes over `samples_f64` halves the number of `filter_map`/`NumCast`/
`ToPrimitive` conversions and comparisons done per pixel - for ~1M samples that's ~1M fewer
`to_f64()` calls and ~1M fewer branch/compare pairs. The relative win (-15 to -18%) is larger than
just "half the loop iterations" would suggest because `new_rescale_and_normalize`'s own cost is a
significant fraction of the whole `colorspace-normalize` pipeline for this fixture (decode + LUT
construction + windowing + pixel mapping) - the old two-pass scan was walking the full sample
count twice for a value (min/max) that's a pure reduction, no reason to ever require two passes.

## What's not done / open questions

- Not attempted: parallelizing this fold with rayon (a separate idea already in `IDEAS.md`,
  "Add the same rayon threshold+guard to `Lut::new_rescale_and_normalize`'s min/max-scanning fold"
  - out of scope for this candidate, which is a pure algorithmic fix, not a dispatch change; the
  two could be combined in a future candidate).
- Not measured: a fixture below the pixel count where this matters (a small image's normalize
  cost is presumably already negligible either way, same reasoning as candidate 003/012's
  below-threshold cases, but not directly quantified here).
- This does not touch `Lut::new_with_fn` (candidate 012's own target) or any other part of the
  LUT/windowing pipeline - single, isolated fix per this workspace's "one fix per candidate" rule.
