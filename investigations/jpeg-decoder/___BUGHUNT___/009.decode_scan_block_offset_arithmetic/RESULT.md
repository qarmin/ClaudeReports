# decode_scan_block_offset_arithmetic

`decode_scan`'s progressive/non-interleaved/restart-interval block-offset computation
(`block_y`/`block_x`/`block_offset` feeding `self.coefficients[...][block_offset..block_offset+64]`
and `mcu_row_coefficients[i][block_offset..block_offset+64]`, `src/decoder.rs:910-1061`, especially
958-1054) is dense arithmetic on input-derived sizes in an area with two prior crash fixes for
similar edge cases (a25b43a "Fix MCU reference in odd decoding order", 6d9fd3e "The decoder now
treats non-interleaved streams as blocks"). The plain non-interleaved case was manually traced and
checked out in a prior review; the full cross product of progressive x interleaved x
restart-interval x multiple-scans-per-component had not been fully verified by hand, making it a
candidate for a fuzz/boundary-value pass.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual algebraic trace | Re-derived `block_offset`/`coefficients_per_mcu_row` bounds by hand for all 4 combinations (progressive x interleaved, progressive x non-interleaved, baseline x interleaved, baseline x non-interleaved), using `update_component_sizes` (`src/parser.rs:292-310`) to confirm `block_size.width/height` is always frame-global (`mcu_size` from `h_max`/`v_max` across *all* frame components), not scan-subset-derived. Algebra showed `block_offset + 64` always lands exactly on the allocated buffer size in every combination, including progressive interleaved DC scans over a subset of frame components (`spectral_selection_start == 0`, `component_count > 1`, allowed by `parser.rs:414-421`) and progressive non-interleaved AC scans (`component_count == 1`, enforced by the same parser check). | No arithmetic mismatch found by hand. |
| Combinatorial boundary-value construction + differential test | 576 real progressive JPEGs generated via ImageMagick/libjpeg-turbo (`convert -interlace Plane`), spanning: image dimensions 1x1 up to 200x150 (many not multiples of 8/16, several as small as a single padded MCU), per-component sampling factors from 1x1 to 4x4 (symmetric and asymmetric, e.g. `2x1`,`1x2`,`4x1`,`2x3`,`3x2`), restart intervals in {1,2,3,4,5,8,1000} (including "reset every single block" and "never reset"). Each file decoded via the crate's real public `Decoder::decode()` API (debug build, `overflow-checks=true`/`debug-assertions=true`) and diffed byte-for-byte against libjpeg-turbo's own decode of the same file (`convert file.jpg -depth 8 rgb:-`). | 0/576 panics. 6/576 flagged for max-abs-pixel-diff > 15 (see root-cause dig below) - all 6 root-caused to an unrelated chroma-upsampling filter difference, not block-offset arithmetic. |
| Mutation fuzzing | Custom `examples/fuzz_mutate.rs` harness (xorshift64 PRNG, no external corpus) starting from 5 hand-verified base files that combine progressive + interleaved (multi-component DC scan, 4:2:0/4:1:1/2x1/1x2 sampling) + restart interval + multiple scans per component (10-11 SOS markers, incl. successive-approximation refinement scans): 1-6 random byte mutations per iteration, applied anywhere in the file after SOI. 30000 iterations each on 4 small files (64x48, 65x49, 33x17, 129x65), 5000 iterations on one larger file (200x150), all run through `std::panic::catch_unwind` against the real `Decoder::decode()` entry point, debug build with overflow-checks. | 0 crashes across 125000 total mutation trials. |

## Status

**Not reproducible.** Covered: a full manual algebraic re-derivation of the offset/bounds formulas
for all 4 progressive/interleaved combinations; 576 structurally-distinct real progressive JPEGs
(boundary sampling factors, boundary restart intervals, boundary/off-alignment dimensions) decoded
and diffed against a reference decoder with zero panics and zero unexplained large diffs; 125000
random-byte-mutation trials across 5 base files that each already exercise
progressive+interleaved+restart+multi-scan simultaneously, with zero panics under a
debug-assertions/overflow-checks build. Two of the differential runs did surface real pixel-value
differences (max abs diff 48-55 on 8-bit channels) on 6 files, all sharing the same shape (a 3x3
image with one 2x2-or-asymmetric chroma-subsampled component, i.e. a single 16x16 MCU heavily
padded around a 3-pixel image) - these were isolated and root-caused (see below) to a
chroma-upsampling filter difference unrelated to restart interval or progressive multi-scan
structure, and unrelated to `decode_scan`'s block-offset arithmetic specifically.

**Fully reproducible**: N/A (no bug reproduced; nothing to mark Confirmed).

## Class

N/A - not reproduced.

## Root cause

No bug found in `decode_scan`'s block-offset arithmetic. The formulas hold algebraically because
`component.block_size.width`/`.height` (`src/parser.rs:305-306`) are always derived from the
frame-global `mcu_size` (itself from `h_max`/`v_max` across *all* frame components, not the
current scan's subset, `src/parser.rs:292-299`), while `block_y`/`block_x` in `decode_scan` are
always derived from `mcu_y`/`mcu_x` ranging over the same frame-global `mcu_size` (`max_mcu_x`/
`max_mcu_y` at `src/decoder.rs:901-908`) times each component's own sampling factor. Multiplying
these out, `block_offset + 64` lands exactly on `component.block_size.width * .height * 64` (the
allocated buffer size, `src/decoder.rs:403-411`) at the maximum-index iteration in every one of the
4 progressive/interleaved combinations - there is no rounding mismatch between how the buffer is
sized and how it's indexed, because both sides ultimately key off the same frame-level `mcu_size`
regardless of which subset of components a given scan lists in its own SOS header (which the
parser enforces is 1 component for any scan with `spectral_selection_start != 0`, and up to 4 for
DC-only scans, `src/parser.rs:414-421`, `390-392`). Restart-interval handling
(`src/decoder.rs:920-956`) only resets Huffman-decoder/predictor/EOB-run state and never
participates in the offset arithmetic at all, so it cannot desynchronize it either.

The 6 flagged differential-test outliers were isolated by re-testing the same 3x3/2x2-subsampled
image (a) with subsampling removed (4:4:4) - diff dropped to 0 - and (b) with the restart interval
removed entirely while keeping 2x2 subsampling - diff stayed at 55, unchanged. This pins the cause
on chroma-upsampling behavior for a component subsampled 2x2 inside a single, heavily-padded 16x16
MCU around a 3-pixel image, independent of restart interval and independent of the progressive
multi-scan structure being investigated here - i.e., an upsampling-filter difference from the
reference decoder (this crate's own upsampler vs. libjpeg-turbo's "fancy" upsampling), not an
indexing defect in `decode_scan`. See `output.txt` for the isolating re-runs and the raw per-pixel
dump.

## Introduced by

N/A - no bug found to attribute.

## The fix

N/A - nothing to fix; candidate ruled out.

- Project test suite: not run before/after, since no source change was made this session (nothing
  in `src/` was touched or reverted).

## What's not done / open questions

- The chroma-upsampling-filter difference noticed as a side effect (6/576 files, isolated to tiny
  heavily-padded 2x2-subsampled images) was not investigated further - it's a value-accuracy
  question about the upsampling algorithm, not the block-offset-arithmetic class this candidate
  targeted, and no external reference decoder was checked for which behavior (if either) is
  "more correct" on such an extreme edge case. Worth a fresh `IDEAS.md` entry if someone wants to
  pursue it specifically.
- Coverage here used only libjpeg-turbo (via ImageMagick) as the structural-diversity generator and
  reference oracle; no independently-crafted (non-libjpeg-turbo-encoded) malformed progressive
  streams with deliberately-inconsistent structural fields (e.g. a DC scan claiming a component
  subset inconsistent with a later AC scan's assumptions, or spectral-selection ranges chosen to
  stress `eob_run` carry-over across restart boundaries) were hand-authored - a dedicated
  boundary-value construction pass targeting *hand-crafted* (not real-encoder-produced) header
  combinations was not attempted this session and remains open ground for a future pass, per the
  soft 3-approach cap in `SKILL.md`.
- Not run under `miri` or ASAN/UBSAN this session - the suspected bug class (arithmetic on
  input-derived `usize`/`u16` sizes) is exactly the class `HEURISTICS.md` says a
  debug-assertions build already covers for free, so miri/ASAN were judged lower-yield than the
  boundary-value/fuzzing/differential combination actually used.
