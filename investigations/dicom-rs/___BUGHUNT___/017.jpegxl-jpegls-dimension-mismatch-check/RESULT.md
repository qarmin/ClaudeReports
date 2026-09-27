# jpegxl.rs/jpegls.rs do not share candidate 016's dimension-mismatch buffer-sizing bug

From `IDEAS.md`'s backlog, itself noticed while working candidate 016
(`016.jpeg2000-dimension-mismatch-panic`): checked whether the same "pre-size the output buffer
from DICOM metadata, then write the external codec's own decoded sample count into it without
cross-checking" pattern is repeated in `jpegxl.rs`/`jpegls.rs`, the codebase's other two
JPEG-family adapters.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `JpegLsAdapter::decode_frame` (`jpegls.rs`): decodes via `CharLS::default().decode(&frame_data)` and simply `dst.append(&mut decoded)` - no pre-sized buffer, no offset-based indexed writes at all; whatever CharLS decodes is appended directly. There is no indexing operation here that could panic from a size mismatch - the worst a dimension disagreement could cause is `dst` ending up a different length than a caller expected (a possible downstream logic concern, not examined further, but categorically not the crash class candidate 016 found). | Ruled out - structurally does not have the vulnerable pattern. |
| Code reading | `JpegXlAdapter::decode_frame` (`jpegxl.rs`, all three bit-depth branches: 8-bit, 16-bit, 24-bit/float): each computes its buffer size as `stream.channels() * stream.width() * stream.height()` - **the decoded JPEG XL stream's own actual dimensions**, not the DICOM object's declared `Rows`/`Columns`/`SamplesPerPixel` (those DICOM-declared values are only used earlier for an unrelated `dst.reserve_exact(...)` capacity hint, which affects performance at most, never correctness). Since the buffer is sized from the same source that determines how much will actually be written (`stream.write_to_buffer(&mut dst[offset..])`, which is itself bounded by the destination slice's own length per ordinary Rust slice-write semantics), there is no cross-boundary trust gap for this write to exceed - self-consistent by construction. | Ruled out - already written more defensively than `jpeg2k.rs` was; uses the actual decoded stream's own dimensions rather than trusting DICOM metadata for the allocation that matters. |

## Status

**Not reproducible** - ruled out by direct code reading; no live repro was constructed since the
analysis is conclusive from the source alone (both files' logic was read completely for the
relevant code paths, not just spot-checked) - matches this skill's convention (see candidate 005)
of a code-reading-only ruling when the mechanism itself is clearly absent, rather than spending a
repro's worth of effort disproving something the code already shows isn't there.

## Class

N/A (not confirmed).

## Root cause

N/A - the suspected pattern (buffer sized from DICOM metadata, written into using an independent,
uncorrelated external decoder's own dimension count) genuinely isn't present in either file:
`jpegls.rs` has no pre-sized buffer at all, and `jpegxl.rs` sizes its buffer from the same
decoded-stream dimensions that also determine how much gets written, not from DICOM metadata.

## The fix

N/A - no fix needed, nothing to fix.

## What's not done / open questions

- `jpeg.rs` (the plain/baseline JPEG adapter, distinct from JPEG 2000/XL/LS) was not checked for
  this pattern in this session - it uses the `jpeg-decoder` crate and wasn't part of the
  `IDEAS.md` lead this candidate was resolving (that lead specifically named `jpegxl.rs`/
  `jpegls.rs`), but the same investigation approach could be applied to it too if pursued later.
- `JpegXlAdapter`'s `24`-bit (float) branch converts `f32` samples to 24-bit integers via
  `(sample * 16777215.) as u32` with no explicit clamp before the cast - a sample outside the
  `[0.0, 1.0]` range (which a decoder bug or malformed input could in principle produce) would
  produce a nonsensical but not out-of-bounds/crashing result (Rust's `as` float-to-int cast
  saturates rather than panicking or wrapping since Rust 1.45). Noticed in passing while reading
  this code but not investigated further, as it's a "possibly wrong output" logic concern rather
  than the crash-class pattern this candidate was specifically checking for.
