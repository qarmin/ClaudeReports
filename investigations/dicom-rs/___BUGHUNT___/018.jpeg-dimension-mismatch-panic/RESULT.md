# Baseline JPEG decoder panics when declared dimensions don't match the embedded JPEG's own

From `IDEAS.md`'s backlog, itself noticed while working candidate 016
(`016.jpeg2000-dimension-mismatch-panic`): checked whether the plain/baseline `jpeg.rs` adapter
(distinct from JPEG 2000/XL/LS, uses the `jpeg-decoder` crate) shares the same "pre-size the
output buffer from DICOM metadata, then write the external codec's own decoded byte count into it
without cross-checking" pattern. It does, in both of its entry points (`decode()` and
`decode_frame()`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `JpegAdapter::decode`/`decode_frame` (`transfer-syntax-registry/src/adapters/jpeg.rs`) - both pre-size `dst` from the DICOM object's declared `Rows`/`Columns`/`SamplesPerPixel`/`BitsAllocated` (`dst.resize(base_offset + samples_per_pixel * stride * nr_frames, 0)` in `decode()`, similarly in `decode_frame()`), then decode each JPEG fragment via `jpeg_decoder::Decoder::decode()` and write `decoded.len()` bytes at `dst[dst_offset..(dst_offset + decoded_len)]` with no check that `decoded_len` fits in the remaining pre-sized space - exactly candidate 016's pattern, independently present in a second adapter. |
| Standalone repro (release build, `dicom-object`+`dicom-pixeldata`'s public API: `open_file`, `InMemDicomObject::put_element`, `PixelDecoder::decode_pixel_data`, same shape as candidate 016) | `___BUGHUNT___/018.../repro`. A real JPEG-encoded DICOM file (`pydicom/SC_rgb_jpeg_gdcm.dcm` via `dicom-test-files`, a genuine 100x100, 3-samples-per-pixel image) with its `Rows`/`Columns` overwritten to `4`/`4` via the public element-mutation API - the embedded JPEG codestream itself is untouched and still genuinely encodes a 100x100 image. | Reproduced deterministically: `range end index 30000 out of range for slice of length 48` panicking in `decode()` (the multi-frame entry point, called by `decode_pixel_data()`) - a real, unmodified JPEG codestream, only paired with disagreeing DICOM metadata. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic (a real, static JPEG codestream; no randomness). |
| Regression check | Ran `cargo test -p dicom-transfer-syntax-registry -p dicom-pixeldata --lib --features dicom-transfer-syntax-registry/jpeg` before and after the fix - this includes `dicom-pixeldata`'s own real-JPEG-file tests (`pydicom/color3d_jpeg_baseline.dcm`, `SC_rgb_jpeg_lossy_gdcm.dcm`, `SC_rgb_jpeg_gdcm.dcm`, including a multi-frame case). | 39 + 3 passed / 0 failed / 2 ignored, identical both before and after - confirms the fix doesn't reject legitimately consistent real-world JPEG data, single- or multi-frame. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`crash` (an unguarded slice-range panic, reachable without any malformed/crafted JPEG codestream -
only a mismatch between a DICOM file's own `Rows`/`Columns` metadata and its embedded JPEG's own
dimensions, the same class as candidate 016, now confirmed present in a second, independent
adapter implementation).

## Root cause

Identical mechanism to candidate 016, independently present in `jpeg.rs`'s own separate
implementation (not shared code with `jpeg2k.rs`): `dst` is pre-sized from the DICOM object's
declared dimensions, but the actual number of bytes copied into it (`decoded_len`, from the
`jpeg-decoder` crate's own decode of the embedded JPEG's real width/height/component count) is
never checked against the remaining space before the copy. A JPEG codestream's own SOF
(Start-Of-Frame) marker carries its true dimensions completely independently of the surrounding
DICOM object's `Rows`/`Columns` attributes - nothing in this adapter cross-checks the two, in
either the single/multi-fragment `decode()` path or the per-frame `decode_frame()` path.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a bounds check (`ensure_whatever!`) immediately before each of the two
`dst[dst_offset..(dst_offset + decoded_len)].copy_from_slice(&decoded)` call sites (`decode()`'s
per-frame loop and `decode_frame()`'s single-frame path), verifying `dst_offset + decoded_len <=
dst.len()` and returning a clear `Result::Err` identifying the mismatch instead of indexing past
the buffer. See `patch.diff` (`transfer-syntax-registry/src/adapters/jpeg.rs`, one file, both call
sites).

Verified: `repro/` re-run against the patched tree no longer panics - `decode_pixel_data()` now
returns a clear error (`"JPEG frame 0 decoded to 30000 bytes, more than the space remaining (48)
... possible dimension mismatch"`) instead of panicking.

- Project test suite: `cargo test -p dicom-transfer-syntax-registry -p dicom-pixeldata --lib
  --features dicom-transfer-syntax-registry/jpeg` - 39 + 3 passed / 0 failed / 2 ignored,
  identical both before and after the patch. `cargo test --workspace --lib` - 351 passed / 0
  failed / 2 ignored, identical both before and after (no regressions, no newly-passing tests - no
  existing test constructs a dimension mismatch between DICOM metadata and an embedded JPEG
  codestream).

## What's not done / open questions

- This is the second of two now-confirmed instances of the same root pattern (candidate 016 was
  the first, in `jpeg2k.rs`). `jpegls.rs`/`jpegxl.rs` were already checked and ruled out for this
  exact pattern in candidate 017. No further JPEG-family adapters remain unchecked in this
  codebase as of this session.
- Not fuzzed - found by direct code reading (recognizing a pattern already confirmed once this
  session) then confirmed using a real, unmodified JPEG file with only its DICOM-level metadata
  altered via the public API, mirroring candidate 016's approach exactly.
