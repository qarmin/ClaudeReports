# BMP `data_offset` (bfOffBits) general validation gap across all pixel-format readers

Given candidate: `BITMAPFILEHEADER.bfOffBits` (`self.data_offset`) is never validated against the
actual file/reader length anywhere in `read_metadata`/`read_headers`, for any BMP pixel format.
Candidate 029 already confirmed and fixed one specific consequence (`read_scanline`'s last-row
`end_pos - current_pos` underflow when `data_offset` points past EOF); candidate 030 already
grepped every `.seek(`/`.stream_position()` call site in the whole repo for the same
subtraction-of-two-`Seek`-positions shape and found no second instance. This candidate asks
whether *other* arithmetic assuming a seek to `data_offset` landed on real data (not necessarily
the `Seek`-subtraction shape 030 already swept for - e.g. `data_offset + row_offset` addition, or
row-size computation feeding that addition) hides an analogous bug in any of the six BMP
pixel-format readers (`read_palettized_pixel_data` for 1/2/4/8bpp, `read_16_bit_pixel_data`,
`read_32_bit_pixel_data`, `read_full_byte_pixel_data` for 24/32bpp, `read_rle_data` for RLE4/RLE8).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: read all six `file_offset = self.data_offset + (start_row as u64 * row_len as u64)` sites plus `read_rle_data`'s `start_pos = self.data_offset` seek, and every downstream use of the seeked-to position | N/A | Every non-RLE reader seeks to `file_offset` and then calls the shared `read_scanline` (candidate 029's already-known/fixed bug site, still present in the working tree per protocol - see below); `read_rle_data` seeks to `data_offset` directly and reads byte-by-byte via `RleReader::read_byte`/`read_exact`, which cleanly propagate `UnexpectedEof` on a too-short file with no arithmetic on the resulting position at all - ruled out as a distinct suspect. Also checked `read_palettized_pixel_data`'s own `row_byte_length = ((i32::from(self.bit_count) * self.width + 31) / 32 * 4) as usize` (i32 arithmetic on `self.width`, unrelated to `data_offset` but adjacent in the same function and reachable via the same "did the seek land on real data" reasoning) for a width large enough to overflow `i32` (needs `width >= 268_435_456` at `bit_count=8`) - ruled out because both `ParsedInfoHeader::parse` and `ParsedCoreHeader::parse` reject `width > MAX_WIDTH_HEIGHT` (`0xFFFF` = 65535) long before this line runs, so `bit_count(<=32) * width(<=65535)` can never approach `i32::MAX`. The final `u64` addition/multiplication in `file_offset` itself was also checked analytically: `data_offset` is bounded to `u32::MAX` (~4.3e9, it's parsed from a 32-bit field) and `start_row`/`row_len` are bounded by the same `MAX_WIDTH_HEIGHT` cap, so neither the multiplication nor the addition can reach `u64::MAX`. |
| Manual boundary-value construction: a temporary standalone probe (`examples/bmp_data_offset_probe.rs`, deleted before the tree was reverted, not committed) built minimal well-formed BMPs for every bit depth (1/4/8/16/24/32bpp BI_RGB, plus BI_RLE4/BI_RLE8), several width/height combinations (1x1, 4x1, 4x3, 17x5, 65535x1, and top-down/negative-height variants of each), and swept `data_offset` across 13 boundary values per case (0, 1, natural-offset-1/±0/+1, file-len-1/0/+1/+2, 2x file-len, `u32::MAX/2`, `u32::MAX-1`, `u32::MAX`) through the real public `BmpDecoder::new` -> `prepare_image` -> `read_image` entry path, under a debug-assertions (`overflow-checks = true`, Cargo's default `dev` profile) build, wrapped in `catch_unwind` to detect any panic. Candidate 029's already-known, already-verified fix (`patch.diff` from that candidate) was applied first to mask that already-documented bug so a second, distinct panic underneath it (if any) wouldn't be missed. | 760 cases total, `cargo run --example bmp_data_offset_probe`, dev profile (overflow-checks on) | **Zero panics** across all 760 cases - every case either decoded successfully or returned a clean `ImageError`/`io::Error`. |

Two approaches were used (code reading/analytical bounds-checking, then a targeted boundary-value
probe with the known bug pre-masked) per `SKILL.md`'s soft cap of 3 - both were clean negatives
with no partial signal, so a third tool (fuzzing) was not needed; this also directly follows on
from candidate 030's own exhaustive grep-based sweep of the `Seek`-arithmetic shape, which already
covered the closely related pattern across the whole repo.

## Status

**Not reproducible.** No new instance of a `data_offset`-driven arithmetic bug was found beyond the
one candidate 029 already confirmed and fixed (its fix was temporarily re-applied for this
candidate's own boundary-value sweep, then reverted per protocol - the underlying bug is still
present in the working tree and remains tracked only under candidate 029, not duplicated here).
Covered this session: analytical review of every `file_offset`/`start_pos` computation in all six
BMP pixel-format readers, and 760 boundary-value decode attempts spanning every bit depth,
compression type (BI_RGB/BI_RLE4/BI_RLE8), several width/height combinations including the
`MAX_WIDTH_HEIGHT` boundary, and the full range of `data_offset` values from 0 to `u32::MAX`.

## Class

n/a (not reproducible - no new bug found)

## Root cause

n/a - this candidate confirms the general "unchecked `data_offset`" concern does not manifest as
any *additional* crash beyond the single already-documented instance (candidate 029's
`read_scanline` underflow). The reason the general gap is largely benign: `Seek::seek` on both
`Cursor<&[u8]>` and `File` allows seeking past the actual end of the underlying data without
erroring, and every pixel-format reader's subsequent read (`read_exact` inside `read_scanline`, or
`RleReader::read_byte`/`read_exact` in the RLE path) then correctly surfaces a clean
`UnexpectedEof` `io::Error` when there isn't enough data at the seeked-to position - this is the
same fallback path candidate 029's fix routes into. The *only* place this fallback was bypassed by
an unchecked-subtraction shortcut was `read_scanline`'s already-fixed last-row special case; no
other reader has an equivalent shortcut. Width/height are independently bounded to `<= 0xFFFF` by
`MAX_WIDTH_HEIGHT`, which also rules out the adjacent `row_byte_length` `i32`-overflow concern
noted above.

## The fix

Not applicable - no new bug found to fix. The one known instance of this concern (BMP
`read_scanline`, candidate 029) already has a verified fix documented in that candidate's own
folder; it was not re-applied here since this candidate found nothing new to combine it with, and
per protocol the working tree was left exactly as it was before this candidate (029's fix reverted
again after use).

- Project test suite: not run for this candidate specifically (no source change was made to
  verify) - candidate 029's own record already states 331 passed / 0 failed before and after its
  fix, on the same tree this candidate started from.

As a side effect, this candidate's boundary-value probe also answers `IDEAS.md` row 43 (whether
candidate 029's fix, hand-verified only for a 2bpp input, holds for the other bit depths): the
probe applied candidate 029's `patch.diff` and exercised `data_offset` values exactly at
`file_len-1`/`file_len`/`file_len+1` (the past-EOF boundary the fix targets) across 1/4/8/16/24/32bpp
and BI_RLE4/BI_RLE8, with no panic in any case - the fix, which lives in the single shared
`read_scanline` function, does structurally cover every bit depth as expected. Both `IDEAS.md` rows
42 and 43 are removed from the backlog as a result of this candidate.

## What's not done / open questions

- As with candidate 030, TIFF IFD-offset and PNG chunk-offset handling live entirely inside the
  external `tiff`/`png` crates this project depends on, not in `image`'s own `src/` tree - neither
  this candidate nor 030 reached into either dependency's own source.
- The boundary-value probe used well-formed headers with only `data_offset` varied; it did not
  combine an out-of-range `data_offset` with other simultaneously-malformed fields (e.g. a bogus
  `colors_used`/palette size alongside an out-of-range `data_offset`) - a future session could
  fuzz the full header space jointly rather than varying `data_offset` in isolation, though nothing
  in the code read this session suggested the two interact in a way that would introduce a new bug
  class.
- ICO-embedded-BMP data reuses the same BMP decoder internals and was not separately re-probed
  here; candidate 030 already found ICO's own offset arithmetic to be purely additive with explicit
  `checked_add` guards, so it is not expected to introduce a new instance of this class, but that
  reasoning wasn't re-verified against a full ICO-wrapped boundary-value sweep this session.
