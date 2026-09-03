# gdcm-decode-pixel-data-bitsallocated1-missing-expansion

From `IDEAS.md`'s backlog (2026-08-25 entry, noticed as a byproduct of candidate `097`'s own
regression testing): `pixeldata/src/gdcm.rs`'s `decode_pixel_data` (the non-frame `gdcm`-feature
`PixelDecoder` method, as opposed to `decode_pixel_data_frame`, which candidate `097` already
fixed) never performs 1-bit-to-byte expansion in its `DicomValue::Primitive` branch - it just does
`p.to_bytes().to_vec()`, unlike its `lib.rs` sibling (the default non-`gdcm` implementation) and
unlike this same file's own now-fixed `decode_pixel_data_frame`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug build, real project test suite (`cargo test -p dicom-pixeldata --features "gdcm image" --lib test_1bit_image_decoding_data`) against the real fixture `pydicom/liver.dcm` (`BitsAllocated=1`, 3 frames) | 1 run | `tests::test_1bit_image_decoding_data` fails: `called Result::unwrap() on an Err value: Error(FrameOutOfRange { frame_number: 0, ... })` from `DecodedPixelData::frame_data`/`to_dynamic_image`, exactly matching the diagnosed symptom |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`, real public `dicom_pixeldata::PixelDecoder::decode_pixel_data`/`DecodedPixelData::frame_data` API via the `gdcm` feature) on a hand-built, well-formed, native/uncompressed (`ImplicitVRLittleEndian`) `FileDicomObject<InMemDicomObject>`: 8x8 pixels, `SamplesPerPixel=1`, `BitsAllocated=1`, `NumberOfFrames=2`, one well-formed 16-byte packed-bit `PixelData` buffer (8 bytes/frame, exact length, nothing malformed) | in-tree, 3 separate runs | Deterministic 3/3: `decode_pixel_data()` returns `Ok` with a 16-byte `data` buffer (8x smaller than the 128 bytes implied by `rows*cols*number_of_frames`), and the subsequent `frame_data(1)` call fails with `Frame #1 is out of range`, every run |
| Same standalone `repro/`, via `just test`'s `clone`/`reproduce-before` against a fresh clone of pristine upstream `df2b9e42` | `just clone` then `just reproduce-before` | Reproduced identically: same 16-byte (not 128-byte) buffer and the same `Frame #1 is out of range` error |

## Status

**Confirmed** - deterministic (no randomness anywhere in the input or the code path), reproduces
3/3 in-tree runs and again via a fresh clone of pristine upstream `df2b9e42` through the
candidate's own `just test` chain, identical wrong buffer size and downstream error every time.

**Fully reproducible**: **Yes**. `repro/` drives the real, public
`dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point (the `gdcm` feature, i.e. the
exact code path in `pixeldata/src/gdcm.rs` that any real caller enabling that feature would use)
on a `FileDicomObject<InMemDicomObject>` built entirely through this crate's own public
element-insertion API (`put_element`) - the same public entry point a real DICOM file parse would
populate. There is no second "peer" process to run here: the untrusted input this bug is about is
the DICOM object's own attribute values (`BitsAllocated=1`, an individually legal value), supplied
through the exact same public path a real file parse would populate them through - per
`CONFIRMATION.md` section 11's carve-out, a single malformed/inconsistent input handed to a
parser's real public entry point needs no second participant to impersonate.

## Class

`logic` (a non-panicking data-integrity bug: `decode_pixel_data()` itself returns `Ok`, but the
`DecodedPixelData` it returns has a `data` buffer 8x smaller than what its own `bits_allocated`/
`cols`/`rows`/`number_of_frames` fields imply, which then surfaces downstream as a graceful but
wrong `FrameOutOfRange` error the moment any caller tries to use the declared dimensions to slice
into it - e.g. `frame_data`/`to_dynamic_image`, or any other consumer trusting the returned
metadata).

## Root cause

`pixeldata/src/gdcm.rs`'s `decode_pixel_data`, in its `DicomValue::Primitive` branch (native,
non-encapsulated uncompressed pixel data), simply returned `p.to_bytes().to_vec()` verbatim, with
no awareness of `BitsAllocated`. For `bits_allocated = 8/16/32` this happens to be correct (each
sample already occupies whole bytes on the wire), but for `bits_allocated = 1` - a real,
DICOM-legal value (PS3.5 explicitly allows single-bit/bitmap pixel data, packed 8 samples per byte,
and the real fixture `pydicom/liver.dcm` uses it) - the wire bytes are still bit-packed, so the
returned buffer is exactly `rows * cols * number_of_frames / 8` bytes instead of the one-byte-per-
pixel representation `DecodedPixelData::frame_data`/`to_dynamic_image` (and this same file's now-
fixed `decode_pixel_data_frame`, and its `lib.rs` sibling) all expect.

This is the same general "no `bits_allocated == 1` packed-bit special case" shape as candidate
`097`'s finding in this exact file's `decode_pixel_data_frame` (and the same shape already fixed
once in `pixeldata/src/lib.rs`'s own non-`gdcm` `decode_pixel_data`) - `decode_pixel_data`'s
`gdcm`-feature implementation was simply never brought in line with either of those.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Mirrored the already-correct pattern (from `pixeldata/src/lib.rs`'s own `decode_pixel_data`, and
from this same file's `decode_pixel_data_frame` after candidate `097`'s fix) into
`decode_pixel_data`'s `DicomValue::Primitive` branch: special-case `bits_allocated == 1` by
computing the packed size across every frame (`frame_samples.div_ceil(8) * number_of_frames`),
bounds-checking it against the actual buffer via `.get(..).context(FrameOutOfRangeSnafu)` (instead
of an unchecked slice), then unpacking each bit into a full byte (`0` or `255`) and truncating to
exactly `frame_pixels * number_of_frames` output bytes - matching the data representation
`DecodedPixelData::frame_data`/`to_dynamic_image` already expect. For `bits_allocated != 1` the
original unconditional `.to_vec()` is kept unchanged.

See `patch.diff` for the exact diff (scoped to `pixeldata/src/gdcm.rs`'s `decode_pixel_data`
only - a new import, `FrameOutOfRangeSnafu`, plus the branch itself).

Verified:
- Re-running `repro/` against the patched clone (`just apply`, then `cargo run` again): no longer
  returns an undersized buffer, 3/3 runs; prints `decode_pixel_data() returned Ok: cols=8 rows=8
  number_of_frames=2 data.len()=128 (expected 128)` followed by `frame_data(1) succeeded (no bug
  observed)` every time. Re-ran the same way 3/3 times before the fix (deterministic wrong-size
  buffer + `FrameOutOfRange` every time) and confirmed the correct behavior after applying the
  patch. See `output.txt` for the exact captured output (both before and after).
- `cargo test -p dicom-pixeldata --features "gdcm image" --lib` (main working tree): **before**
  fix, 81 passed / 2 failed (`tests::test_1bit_image_decoding_data`,
  `tests::test_1bit_image_decoding_data_frame`) / 3 ignored; **after** fix, 82 passed / 1 failed
  (`tests::test_1bit_image_decoding_data_frame` only - a separate, pre-existing bug already fixed
  by candidate `097`, not present here since `097`'s own fix was reverted per this workspace's
  "always revert" rule before this candidate was investigated in isolation) / 3 ignored - the
  exact target test (`test_1bit_image_decoding_data`) now passes, zero new failures introduced.
- `cargo test --workspace --lib` (no `gdcm` feature, default workspace run): **351 passed, 0
  failed, 2 ignored**, identical before and after - this fix is entirely inside a
  `#[cfg(feature = "gdcm")]`-gated module, so the default workspace test run is unaffected either
  way, as expected.
- `just test` (the candidate's own self-contained justfile, fresh clone of pristine upstream
  `df2b9e42`, `gdcm`+`image` features via the repro crate): `clone` -> `reproduce-before`
  (reproduces the wrong-size buffer + `FrameOutOfRange`) -> `apply` -> `reproduce-after` (correct
  128-byte buffer + successful `frame_data(1)`), run end-to-end this session.

## What's not done / open questions

- Between candidates `097` and `098`, both `gdcm.rs` methods (`decode_pixel_data_frame` and
  `decode_pixel_data`) now special-case `bits_allocated == 1`; the `DicomValue::PixelSequence`
  branch of `decode_pixel_data` (encapsulated/compressed pixel data) was not touched by this
  candidate and was not checked for the same `bits_allocated == 1` shape - GDCM's own C++ codec
  paths are invoked there instead of a plain Rust slice, so any equivalent defect (if one exists)
  would live inside vendored GDCM rather than in this Rust source, which is a different
  investigation (see the still-open `IDEAS.md` entry about GDCM's own JPEG2000/JPEG-LS internals).
- Only this exact spot was checked; whether any other codec adapter or decode path (e.g.
  `transfer-syntax-registry`'s own adapters) has the same "assume `bits_allocated` is byte-aligned"
  shape for `bits_allocated == 1` was not swept this session.
