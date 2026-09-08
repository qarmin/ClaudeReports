# What does CRC32 verification cost on the decode path? (measurement only, no patch)

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

Every IDAT byte and every chunk's bytes are hashed with `crc32fast` as they're read
(`src/decoder/stream.rs:807-809, 848-850, 940-942, 1057-1059`), and the running hash is checked
against the chunk's trailing 4-byte CRC (`:989`, `self.current_chunk.crc.clone().finalize()`).
`DecodeOptions::ignore_crc` already exists and skips all of this, which makes "what does CRC
verification cost" a clean A/B with **no patch needed at all** - this candidate is a pure
measurement/characterization, not a code change.

## What was tried

Standalone bench (`bench/` - no existing harness reaches this specific toggle) decodes every real
file in `tests/benches/` (15 files, the repo's own bench corpus) fully (`Reader::next_frame` into a
preallocated buffer, not just headers) with `ignore_crc=false` (default) and `ignore_crc=true`,
**alternating which one runs first every iteration** (not two separate blocks - this cancels
per-run system drift within a single process invocation, the same discipline `tools/paired.sh` uses
across binaries, applied here across a boolean flag instead). 54-90 samples per file after 10%
warmup discard, `taskset -c 0`. Two independent runs (60 and 100 iterations) to check
reproducibility given this session's earlier finding that timing can be session-unstable on this
machine (see `001.ENCODER_OUTPUT_VEC_CAPACITY/RESULT.md`).

| File | img | crc_on p50 (run 1 / run 2) | crc_off p50 (run 1 / run 2) | delta (run 1 / run 2) |
|---|---|---|---|---|
| Exoplanet_Phase_Curve_(Diagram).png | 3840x2160 | 14.681 / 14.685 ms | 14.549 / 14.601 ms | -0.9% / -0.6% |
| Exoplanet...indexed_gimp.png | 3840x2160 | 1.989 ms | 1.956 ms | -1.6% |
| Fantasy_Digital_Painting.png | 1488x1486 | 12.015 ms | 11.960 ms | -0.5% |
| Lohengrin...png | 2812x3982 | 13.802 / 13.699 ms | 13.651 / 13.523 ms | -1.1% / -1.3% |
| Transparency.png | 300x300 | 0.049 ms | 0.048 ms | -1.5% |
| kodim02.png | 768x512 | 2.059 / 2.042 ms | 2.046 / 2.028 ms | -0.7% / -0.7% |
| kodim07.png | 768x512 | 2.473 ms | 2.458 ms | -0.6% |
| kodim17.png | 512x768 | 2.084 ms | 2.064 ms | -1.0% |
| kodim23.png | 768x512 | 1.989 ms | 1.980 ms | -0.4% |
| lorem_ipsum_oxipng.png | 935x534 | 0.219 ms | 0.217 ms | -1.0% |
| lorem_ipsum_screenshot.png | 935x534 | 0.653 ms | 0.643 ms | -1.4% |
| paletted-zune.png | 2048x2253 | 4.241 ms | 4.197 ms | -1.0% |
| tango-...-128-rsvg-convert.png | 128x128 | 0.053 ms | 0.053 ms | -1.1% |
| tango-...-16.png | 16x16 | 0.0045 ms | 0.0043 ms | -2.5% |
| tango-...-32.png | 32x32 | 0.0077 ms | 0.0076 ms | -1.2% |

(run 2 only re-measured 3 representative files - Exoplanet/Lohengrin/kodim02 - to check
reproducibility across the size range; the deltas agree with run 1 within ~0.3 percentage points.)

## Result

**Time.** CRC verification costs a small, real, consistent **~0.4-2.5% of total decode time**
across all 15 real files, with no correlation to image size or color type (the smallest file,
16x16, shows the *largest* relative delta at -2.5%, likely because fixed per-call overhead is a
bigger fraction of an already-tiny total). This is measured via within-process alternation (not a
cross-process paired comparison), so it does not carry the same session-to-session instability risk
flagged for candidate 001 - and the two independent runs on overlapping files agree closely,
supporting that this small delta is real rather than noise. No allocation-count/memory dimension
applies here (CRC computation doesn't allocate).

## Does it change behavior/output?

- N/A - measured/quantified only, no patch applied to real source. `DecodeOptions::ignore_crc`
  already exists in shipped code and is a public, documented opt-out; nothing about this candidate
  changes it.
- Correctness of the *measurement itself*: decoded pixel buffers are asserted byte-identical
  between `ignore_crc=true` and `ignore_crc=false` for every file before any timing is recorded
  (`verify=OK` on all 15) - confirms CRC checking only validates, never affects decoded output, as
  expected.
- Project test suite: not run - no source file was changed by this candidate (pure measurement),
  so a before/after comparison would be identical by construction.

## Why the result looks the way it does

CRC32 (via `crc32fast`, which uses hardware CRC32 instructions when available) is cheap per byte
relative to the rest of the decode pipeline (inflate/unfilter/color conversion), so its total
contribution stays in the low single-digit percent even though every single IDAT byte passes
through it. This matches the shape of the backlog entry that proposed this measurement: it doesn't
point at a large win, but it does answer the question cleanly enough that nobody needs to re-ask it
without first checking here.

## What's not done / open questions

- Not measured: whether removing/relaxing CRC checking specifically for `IDAT`/`fdAT` (the largest
  volume of hashed bytes) versus small ancillary chunks contributes differently - this candidate
  measured the combined effect only, per-chunk-type breakdown would need per-callsite
  instrumentation, not attempted here (diminishing returns given the total is already only ~1%).
- Not measured: whether `crc32fast`'s hardware-accelerated path is actually active on this machine
  (it auto-detects at runtime) - if it weren't, this ~1% figure could be an overestimate of the
  fully-optimized cost on a typical deployment target. Worth a quick check (`crc32fast` exposes how
  it detects capability) before treating ~1% as a hard ceiling.
- This confirms the backlog's own framing (do this measurement before touching CRC handling) - the
  answer is "not worth touching," so no follow-up candidate is queued for CRC handling itself.
