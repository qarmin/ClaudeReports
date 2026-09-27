# pixeldata-gdcm-decode-frame-bitsallocated1-chunks-exact-panic

From `IDEAS.md`'s backlog (2026-08-25 entry, noticed as a byproduct of candidate `096`'s own
regression testing): `pixeldata/src/gdcm.rs`'s `decode_pixel_data_frame` (the `gdcm`-feature
`PixelDecoder` impl) computes `frame_size = cols * rows * samples_per_pixel * (bits_allocated / 8)`
in two spots and then calls `.chunks_exact(frame_size)`. `BitsAllocated = 1` is an ordinary,
DICOM-legal attribute value (a real fixture, `pydicom/liver.dcm`, uses it) and integer division
makes `1 / 8 = 0`, so `frame_size = 0` and `chunks_exact(0)` panics with "chunk size must be
non-zero". Candidate `096`'s own fix explicitly allows `BitsAllocated = 1` (it validates against
the DICOM-legal set `{1, 8, 16, 32}`), so it does not touch this separate defect.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug build, real project test suite (`cargo test -p dicom-pixeldata --features "gdcm image" --lib test_1bit_image_decoding`) against the real fixture `pydicom/liver.dcm` (`BitsAllocated=1`, 3 frames) | 1 run | `tests::test_1bit_image_decoding_data_frame` panics immediately: `pixeldata/src/gdcm.rs:293:22: chunk size must be non-zero` - the exact predicted location/message |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`, real public `dicom_pixeldata::PixelDecoder::decode_pixel_data_frame` API via the `gdcm` feature) on a hand-built, well-formed, native/uncompressed (`ImplicitVRLittleEndian`) `FileDicomObject<InMemDicomObject>`: 8x8 pixels, `SamplesPerPixel=1`, `BitsAllocated=1`, `NumberOfFrames=2`, one well-formed 16-byte packed-bit `PixelData` buffer (8 bytes/frame, exact length, nothing malformed) - calling `decode_pixel_data_frame(1)` (the second frame) | in-tree, 3 separate runs | `SIGABRT`-free Rust panic every run, identical message/location, 3/3 |
| Same standalone `repro/`, via `just test`'s `clone`/`reproduce-before` against a fresh clone of pristine upstream `df2b9e42` | 3 separate manual re-runs after `just clone` (the `just test` chain itself stops at the first non-zero exit, same accepted pattern as candidate `096`'s justfile) | Reproduced identically 3/3: same panic message/location (path differs only by the clone's own absolute path) |

## Status

**Confirmed** - deterministic (no randomness anywhere in the input or the code path), reproduces
3/3 in-tree runs and 3/3 standalone `repro/` runs against a fresh clone of pristine upstream
`df2b9e42`, identical panic message and source location every time.

**Fully reproducible**: **Yes**. `repro/` drives the real, public
`dicom_pixeldata::PixelDecoder::decode_pixel_data_frame()` entry point (the `gdcm` feature, i.e.
the exact code path in `pixeldata/src/gdcm.rs` that any real caller enabling that feature would
use) on a `FileDicomObject<InMemDicomObject>` built entirely through this crate's own public
element-insertion API (`put_element`) - the same public entry point a real DICOM file parse would
populate. There is no second "peer" process to run here: the untrusted input this bug is about is
the DICOM object's own attribute values (`BitsAllocated=1`, an individually legal value), supplied
through the exact same public path a real file parse would populate them through - per
`CONFIRMATION.md` section 11's carve-out, a single malformed/inconsistent input handed to a
parser's real public entry point needs no second participant to impersonate.

## Class

`crash` (an ordinary Rust panic - `Vec::chunks_exact` rejecting a zero chunk size - triggered by
nothing more than an ordinary, individually-legal DICOM attribute value, `BitsAllocated = 1`, that
this crate's own attribute reader accepts without complaint; a real-world PACS/viewer decoding any
untrusted 1-bit-per-pixel image - e.g. a bitmap overlay/mask frame, as in the real
`pydicom/liver.dcm` fixture - through the `gdcm` feature and asking for an individual frame hits
this unconditionally).

## Root cause

`pixeldata/src/gdcm.rs`'s `decode_pixel_data_frame` computed the per-frame byte size in two
places using plain integer division by 8:

- the `DicomValue::PixelSequence` branch's `ImplicitVRLittleEndian`/`ExplicitVRLittleEndian` arm
  (labelled "just in case of encapsulated uncompressed data"): `frame_size = cols * rows *
  samples_per_pixel * (bits_allocated / 8)`
- the `DicomValue::Primitive` branch (native, non-encapsulated uncompressed data): the same
  formula, in `usize`

For `bits_allocated = 1` (a real, DICOM-legal value - PS3.5 explicitly allows `BitsAllocated = 1`
for single-bit/bitmap pixel data, packed 8 samples per byte), `1 / 8` truncates to `0`, so
`frame_size` becomes `0` in both spots. The subsequent `data.chunks_exact(frame_size)` /
`p.to_bytes().chunks_exact(frame_size)` call panics unconditionally with "chunk size must be
non-zero" - `Vec::chunks_exact`'s own documented precondition - regardless of how much actual
pixel data is present or which frame is requested.

This is the same general "divide bits_allocated by 8 without accounting for `bits_allocated < 8`"
shape already fixed once in this exact file's sibling native (non-`gdcm`) implementation
(`pixeldata/src/lib.rs`'s own `decode_pixel_data`/`decode_pixel_data_frame`, which already special-
case `bits_allocated == 1` with a `frame_samples / 8` packed-size computation plus bit-unpacking)
- the `gdcm.rs` file's own `decode_pixel_data_frame` was simply never brought in line with that
fix.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Mirrored the native (non-`gdcm`) implementation's already-correct pattern into both spots in
`pixeldata/src/gdcm.rs`'s `decode_pixel_data_frame`: special-case `bits_allocated == 1` by
computing the packed frame size as `frame_pixels.div_ceil(8)` (never zero for any `frame_pixels >
0`) and unpacking each bit of the resulting chunk into a full byte (`0` or `255`), matching the
data representation `DecodedPixelData::frame_data`/`to_dynamic_image` already expect for 1-bit
images (as established by the native implementation and by `DecodedPixelData::frame_data`'s own
`bits_allocated.div_ceil(8)`-based slicing). For `bits_allocated != 8` the existing multiply-based
computation is kept, but changed from `bits_allocated / 8` to `bits_allocated.div_ceil(8)` for
consistency (a no-op for the now GDCM-legal set `{8, 16, 32}` established by candidate `096`'s own
fix, since those all divide evenly by 8).

See `patch.diff` for the exact diff (scoped to `pixeldata/src/gdcm.rs` only, both spots inside
`decode_pixel_data_frame`).

Verified:
- Re-running `repro/` against the patched clone (`just apply`, then `cargo run` again): no longer
  panics, 3/3 runs; prints `did not panic - decode_pixel_data_frame(1) returned Ok: cols=8 rows=8
  number_of_frames=1 data.len()=64` (the correct bit-unpacked 64-byte second frame) every time.
  Re-ran the same way 3/3 times before the fix (deterministic panic every time) and confirmed the
  graceful/correct behavior after applying the patch. See `output.txt` for the exact captured
  output.
- `cargo test -p dicom-pixeldata --features "gdcm image" --lib` (main working tree): **before**
  fix, 81 passed / 2 failed (`tests::test_1bit_image_decoding_data`,
  `tests::test_1bit_image_decoding_data_frame`) / 3 ignored; **after** fix, 82 passed / 1 failed
  (`tests::test_1bit_image_decoding_data` only - a separate, pre-existing bug, see below) / 3
  ignored - the exact target test (`test_1bit_image_decoding_data_frame`) now passes, zero new
  failures introduced.
- `cargo test --workspace --lib` (no `gdcm` feature, default workspace run): **351 passed, 0
  failed, 2 ignored**, identical before and after - this fix is entirely inside a
  `#[cfg(feature = "gdcm")]`-gated module, so the default workspace test run is unaffected either
  way, as expected.

## What's not done / open questions

- `tests::test_1bit_image_decoding_data` (the non-frame `decode_pixel_data`, as opposed to
  `decode_pixel_data_frame`) still fails after this fix, with a *different* symptom
  (`FrameOutOfRange` from `DecodedPixelData::frame_data`, not a panic in `gdcm.rs` itself):
  `gdcm.rs`'s `decode_pixel_data`'s own `DicomValue::Primitive` branch (`p.to_bytes().to_vec()`,
  around line 128) never performs the 1-bit-to-byte expansion at all, unlike its `lib.rs` sibling
  and unlike this candidate's now-fixed `decode_pixel_data_frame`. The returned `DecodedPixelData`
  ends up with `bits_allocated = 1` but a data buffer 8x smaller than
  `frame_data`/`to_dynamic_image` expect, which is a distinct, non-panicking data-integrity bug in
  the same file - not investigated as its own candidate this session (added to `IDEAS.md`).
- Only `pixeldata/src/gdcm.rs`'s `decode_pixel_data_frame` was checked. Whether any other codec
  adapter or decode path (e.g. `transfer-syntax-registry`'s own adapters, or GDCM's own internal
  1-bit handling for compressed transfer syntaxes) has the same `bits_allocated / 8` shape for
  `bits_allocated == 1` was not swept this session.
