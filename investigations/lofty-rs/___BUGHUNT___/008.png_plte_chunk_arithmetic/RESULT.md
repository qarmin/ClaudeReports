# png_plte_chunk_arithmetic

`lofty/src/picture/mod.rs`, `PictureInformation::from_png` (~line 348-422): flagged in `IDEAS.md`
while working `002.jpeg_precision_components_overflow/` as sharing the same cross-format trigger
path (`PictureInformation::from_picture`, reachable from `From<Tag> for VorbisComments` and
`OggPictureStorage::insert_picture` when carrying an ID3v2/APE/MP4-sourced picture into a
FLAC/OGG/`VorbisComments` file) - the suspicion being that `from_png`'s own arithmetic
(`color_depth *= 3` / `*= 4`, the PLTE-chunk `size.overflowing_add(4)`) might hide a variant of the
`002` JPEG `u8`-before-widen overflow, but this had only been read, not independently boundary-tested
or fuzzed.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual arithmetic trace of every operation in `from_png` | static read | `color_depth` is `u32::from(reader.read_u8()?)` before any multiplication (unlike JPEG's `precision * components` which multiplies two `u8`s before widening) - max value `255 * 4 = 1020`, nowhere near `u32::MAX`, so no overflow is possible regardless of input. The PLTE chunk's `size.overflowing_add(4)` is already explicitly checked and `break`s the loop on overflow. `size / 3` for `num_colors` has a constant non-zero divisor, so it can't divide-by-zero. `reader.seek(SeekFrom::Current(i64::from(content_size)))` can't overflow `i64` since `content_size: u32` always fits. |
| Debug-assertions build (`cargo test`, default dev profile has `overflow-checks` on), a standalone integration test (`lofty/tests/scratch_png_boundary.rs`, deleted after the session) driving the real end-to-end trigger path noted in the candidate (`Picture::unchecked(..).build()` -> `Tag::push_picture` -> `VorbisComments::from(tag)`, i.e. the real `From<Tag> for VorbisComments` -> `PictureInformation::from_picture` -> `from_png` path) with a crafted color-type-3 PNG whose PLTE chunk size was set to `u32::MAX`, `u32::MAX - 1`, `u32::MAX - 3`, `u32::MAX - 4` (straddling the `overflowing_add(4)` boundary), and `0`/`1`/`2` (straddling `size / 3`'s smallest values), wrapped in `std::panic::catch_unwind` | debug build, `cargo test -p lofty --test scratch_png_boundary -- --nocapture`, 7 boundary variants, reverted afterward | All 7 variants completed without panicking; every one converted exactly 1 picture through the real `From<Tag> for VorbisComments` path. |
| Short one-off fuzz loop, same test file, calling the public `PictureInformation::from_png` directly with 200,000 pseudo-random byte strings (length 8..71, `xorshift64`-generated content) prefixed with the correct 8-byte PNG signature | debug build, same `cargo test` invocation, 200,000 iterations, `catch_unwind` around each call, reverted afterward | 0 panics out of 200,000 iterations. |

## Status

**Not reproducible.** No arithmetic overflow, divide-by-zero, or panic was found in `from_png`
across manual boundary-value construction on the specific arithmetic sites flagged as suspicious,
nor across a 200,000-iteration random fuzz sweep of arbitrary post-signature bytes, both run under
a debug-assertions (`overflow-checks`) build and both driven through the real end-to-end trigger
path (`From<Tag> for VorbisComments`) noted in the candidate.

## Class

N/A (not reproducible)

## Root cause

Not applicable in the "confirmed bug" sense. The suspicion did not hold up: unlike
`from_jpeg`'s `precision * components` (confirmed vulnerable in `002`, multiplies two `u8`s before
widening to `u32`), `from_png`'s `color_depth` is already widened to `u32` via `u32::from(..)`
*before* being multiplied by the small constants `3`/`4`, so the maximum possible product (`1020`)
never approaches `u32::MAX`. The only other arithmetic in the function
(`size.overflowing_add(4)`) is already explicitly checked in the source, and the remaining
operations (`size / 3`, `i64::from(content_size)`) have no way to overflow or divide by zero given
`size`/`content_size` are `u32`.

## The fix

No fix attempted - no reproducible incorrect behavior was found to fix.

- Project test suite: not run for this candidate (no fix was made; the temporary
  `lofty/tests/scratch_png_boundary.rs` investigation file was deleted and `git status` verified
  clean after the session).

## What's not done / open questions

- Only `from_png`'s own arithmetic was in scope for this candidate, per the `002` follow-up note.
  `from_jpeg` itself was already confirmed vulnerable and fixed in `002`; `from_reader`,
  `from_flac_bytes`/`from_flac_bytes_inner`, and `from_ape_bytes` (other `PictureInformation`/
  `Picture` entry points in the same file) were not independently re-checked this session.
- The 200,000-iteration fuzz sweep was a short, targeted one-off (a few seconds of wall time), not
  a real fuzzing campaign with a persistent corpus - a longer run via `fuzz-bug-repro` was not
  attempted and might still be worth pointing at `from_png`/`from_jpeg` together given they share
  the same untrusted-picture-bytes attack surface.
