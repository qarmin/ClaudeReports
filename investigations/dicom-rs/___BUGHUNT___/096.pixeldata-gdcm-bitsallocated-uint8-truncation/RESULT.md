# pixeldata-gdcm-bitsallocated-uint8-truncation

From `IDEAS.md`'s backlog (2026-08-25 entry): candidate `085` confirmed and fixed the
"infallible allocation sized straight from untrusted declared header fields (Rows/Columns/
SamplesPerPixel/BitsAllocated), aborting on allocation failure" bug class in
`transfer-syntax-registry/src/adapters/*.rs`, but explicitly left `pixeldata/src/` unswept for
the same pattern. `pixeldata/src/gdcm.rs`'s `decode_pixel_data`/`decode_pixel_data_frame` (the
`PixelDecoder` impl used whenever the `gdcm` feature is enabled) build `dims`/pixel-format
values straight from the DICOM header's own declared `Rows`/`Columns`/`SamplesPerPixel`/
`BitsAllocated`/`NumberOfFrames` and hand them to `gdcm_rs::decode_multi_frame_compressed`/
`decode_single_frame_compressed` - an FFI call into the vendored GDCM 3.1 C++ library - with no
cross-check against the actual encapsulated fragment size.

Reading the real vendored GDCM source (`~/.cargo/registry/.../gdcm-rs-0.6.0/GDCM/Source/...`)
found the actual mechanism: `gdcm::PixelFormat::GetPixelSize()` computes
`pixelsize = (uint8_t)(BitsAllocated / 8); pixelsize *= SamplesPerPixel;` - an **8-bit
narrowing cast** applied to an ordinary, individually-legal 16-bit (`US`) `BitsAllocated`
attribute value that this crate's own `pixeldata::attribute::bits_allocated()` reads and
forwards completely unchecked. `BitsAllocated = 2048` (2048 / 8 = 256, which truncates to 0 as
a `uint8_t`) makes GDCM's own `Bitmap::GetBufferLength()` (`Rows * Columns * PixelSize`) return
`0` regardless of the declared `Rows`/`Columns`/`SamplesPerPixel` - a different, and more
directly demonstrable, consequence than the originally-suspected `u32` `Rows*Columns*Frames`
wraparound (that wraparound is real too - see "What's not done" - but turned out to be
self-consistently handled inside GDCM's own RLE/JPEG `Try*Codec` buffer-copy logic, so it did
not itself produce a provable crash this session).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the real vendored GDCM 3.1 C++ sources bundled inside the `gdcm-rs` crate (`gdcmBitmap.cxx`, `gdcmPixelFormat.cxx`, `gdcmRLECodec.cxx`) reachable from `pixeldata/src/gdcm.rs`'s FFI call | Traced `Bitmap::GetBufferLength()` -> `PixelFormat::GetPixelSize()` (returns `uint8_t`) -> `gdcm_wrapper.cc`'s `c_decode_frames` (`malloc(length)`, no null-check) -> `RLECodec::Decode`'s `NumberOfDimensions==3` branch (`char *buffer = new char[len]; llen = len/nframes;`) | Found: (1) `GetPixelSize()`'s 8-bit narrowing cast lets an ordinary `BitsAllocated` value (e.g. `2048`, legal for the attribute's own `US`/16-bit range) truncate GDCM's own declared-size computation to `0`; (2) `gdcm_wrapper.cc` never checks `malloc()`'s return for `NULL` before use (mitigated on the Rust side only by `gdcm-rs`'s own `NonNull` check in `PixelData::new`); (3) the RLE/JPEG `Try*Codec` functions in `gdcmBitmap.cxx` consistently reuse the *same* (possibly wrapped/truncated) `GetBufferLength()` value for both the `malloc`/`new` size and the final `memcpy` length, so the originally-suspected `Rows*Columns*NumberOfFrames` 32-bit wraparound (found via a similar `unsigned int mul` computation) did not, on this specific codepath, produce a provable buffer overflow this session - it degrades to a silent, self-consistent 0-or-small buffer instead |
| Debug build, real public API (`cargo test -p dicom-pixeldata --features gdcm --lib`), temporary in-tree `#[test]` (reverted before this write-up): a `FileDicomObject<InMemDicomObject>` built entirely via `put_element` - RLE Lossless transfer syntax, `Rows=Columns=64`, `SamplesPerPixel=1`, `BitsAllocated=2048`, one well-formed 64-byte RLE fragment (a bare header: `NumberOfSegments=1`, `Offset[0]=64`, no payload) - calling `.decode_pixel_data()` | `cargo test -p dicom-pixeldata --features gdcm --lib bughunt_gdcm_buffer_length_u32_wraparound -- --nocapture`, 3 separate runs | `SIGABRT` every run: `gdcmRLECodec.cxx:777: ... Assertion 'length' failed` (this sandbox's cached `gdcm-rs` build is CMake `Debug`, i.e. assertions enabled - see `README.md`'s tooling notes) |
| Same input, `--release` Cargo profile (a pristine `gdcm-rs` Release/CMake build was already cached in `target/release` from an earlier session) | `cargo test --release -p dicom-pixeldata --features gdcm --lib ... -- --nocapture` | No abort (GDCM's assertions are compiled out under `NDEBUG`): `decode_pixel_data()` returns `Ok(DecodedPixelData)` with `cols=64 rows=64 number_of_frames=1` but `data.len()=0` - a silent declared-vs-actual-size mismatch, the exact precondition candidates `086`/`087` (this project's own `___BUGHUNT___/`) already showed panics a caller's later `frame_data`/`to_dynamic_image` call under 64-bit `usize` multiply overflow |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`, real public `dicom_pixeldata::PixelDecoder::decode_pixel_data()` API) against a fresh clone of pristine upstream `df2b9e42`, via `just test` | Debug build (`cargo run`), 3 separate runs | Reproduced identically: `SIGABRT`/exit `134`, same assertion message/location, 3/3 |

## Status

**Confirmed** - deterministic, reproduces 3/3 in-tree test runs plus 3/3 standalone `repro/` runs
against a fresh clone of pristine upstream `df2b9e42`, identical assertion message and location
every time. Reproduces specifically in a **CMake-Debug build of the vendored GDCM library**
(this sandbox's default `cargo build`/`cargo test`, and therefore also what any `cargo test`-based
CI for this crate would produce). In a CMake-Release build of GDCM (`NDEBUG`, matching what a
`cargo build --release` deployment's `cmake`-crate invocation would normally produce), the same
root cause does not abort but instead makes `decode_pixel_data()` return `Ok(..)` with a
`DecodedPixelData` whose declared dimensions don't match its actual (empty) data - not itself
re-confirmed this session as leading to a further crash (that would require chaining into
candidates `086`/`087`'s already-confirmed downstream overflow, which this session did not
separately re-verify for this specific input).

**Fully reproducible**: **Yes**. `repro/` drives the real, public
`dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point (the `gdcm` feature, i.e. the
exact FFI path `gdcm-rs`/vendored GDCM uses) on a `FileDicomObject<InMemDicomObject>` built
entirely through this crate's own public element-insertion API (`put_element`) - the same public
entry point any real caller (a DICOM viewer, a PACS client parsing an incoming file) would use.
There is no second "peer" process to run here: the untrusted input this bug is about is the
DICOM object's own attribute values (`BitsAllocated`), which this repro supplies through the
exact same public path a real file parse would populate them through - per `CONFIRMATION.md`
section 11's carve-out, a single malformed/inconsistent input handed to a parser's real public
entry point needs no second participant to impersonate.

## Class

`crash` (an uncatchable `SIGABRT` from a debug assertion inside a third-party C++ library reached
via FFI, triggered by nothing more than an ordinary, individually-legal 16-bit DICOM attribute
value (`BitsAllocated`) that this crate's own attribute reader accepts without validation - a
natural DoS vector for any service decoding untrusted DICOM pixel data with the `gdcm` feature
enabled; in a Release build of the vendored dependency, the same input instead causes silent data
corruption rather than a crash, i.e. also `security`/`logic`-adjacent).

## Root cause

`gdcm::PixelFormat::GetPixelSize()` (vendored inside `gdcm-rs`, `GDCM/Source/MediaStorageAndFileFormat/gdcmPixelFormat.cxx:206-220`)
computes the per-pixel byte size as a **`uint8_t`**:
```cpp
uint8_t pixelsize = (uint8_t)(BitsAllocated / 8);
...
pixelsize *= SamplesPerPixel;
return pixelsize;
```
`pixeldata/src/gdcm.rs` reads `BitsAllocated` via `pixeldata::attribute::bits_allocated()`
(`pixeldata/src/attribute.rs:149-153`), which does nothing beyond parsing it as a plain `u16` -
no range check against the DICOM standard's own legal values (`1`, `8`, `16`, or `32`) - and
passes it straight through to `gdcm_rs::decode_multi_frame_compressed`/
`decode_single_frame_compressed`. A value like `2048` (still a perfectly legal 16-bit integer)
makes `BitsAllocated / 8 = 256` truncate to `0` when narrowed to `uint8_t`, so
`GetPixelSize()` returns `0` and GDCM's own `Bitmap::GetBufferLength()` (`Rows * Columns *
PixelSize`) becomes `0` regardless of the declared image dimensions. `gdcm::RLECodec::Decode`'s
3-D branch (`gdcmRLECodec.cxx:635-650`) allocates `new char[len]` from that (now zero) length and
divides it by the fragment count (`llen = len / nframes`), then `RLECodec::DecodeByStreams`
(`gdcmRLECodec.cxx:762-778`) hits `unsigned long length = Length; assert(length);` - `length` is
`0`, so the assertion fails and the whole process aborts. This is the same general "attacker-
controlled header field feeds straight into an allocation/size computation with no validation"
class already confirmed in this project three times over in `parser/src/stateful/decode.rs`
(candidates `004`/`080`/`081`) and once in the codec adapters (candidate `085`) - here crossing
into a **third-party C++ dependency via FFI**, with the specific narrowing happening inside GDCM's
own code rather than this repository's.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a validation of `bits_allocated` against the DICOM standard's own legal set (`1`, `8`,
`16`, `32`) in both `decode_pixel_data` and `decode_pixel_data_frame` in
`pixeldata/src/gdcm.rs`, immediately after the attribute is read and before it is ever passed
into `gdcm_rs`, reusing this crate's existing generic `UnsupportedOtherSnafu` error variant
(already used elsewhere in this crate for exactly this kind of "individually-legal-typed but
semantically-out-of-range attribute" rejection):

```rust
let bits_allocated = bits_allocated(self)?;
ensure!(
    matches!(bits_allocated, 1 | 8 | 16 | 32),
    UnsupportedOtherSnafu {
        name: "BitsAllocated",
        value: bits_allocated.to_string(),
    }
);
```

This is a fix in this repository's own `pixeldata/src/gdcm.rs`, not in the vendored/external
GDCM C++ source or the `gdcm-rs` crate (both out of scope for a same-session fix here) - it stops
the dangerous value from ever reaching GDCM's FFI boundary at all, which resolves both
manifestations observed this session (the Debug-GDCM abort and the Release-GDCM silent
size-mismatch) at the same single point, rather than patching either symptom separately.

See `patch.diff` for the exact diff (scoped to `pixeldata/src/gdcm.rs` only).

Verified:
- Re-running `repro/` against the patched clone (`just test`, `df2b9e42` + `patch.diff`): no
  longer aborts - prints a normal `Err(UnsupportedOther { name: "BitsAllocated", value: "2048" })`
  and exits `0`. Re-ran the same way 3/3 times before the fix (deterministic `SIGABRT`/exit `134`
  every time) and confirmed the graceful-error behavior after applying the patch.
- `cargo test -p dicom-pixeldata --features "gdcm image" --lib` (main working tree, both before
  and after applying the fix, and independently in the standalone clone): **81 passed; 2 failed
  (pre-existing, unrelated); 3 ignored** in both cases - identical pass/fail counts before and
  after, zero regressions. The 2 pre-existing failures
  (`tests::test_1bit_image_decoding_data`/`_frame`) are a distinct, already-present bug in this
  same file (`BitsAllocated=1` on a real fixture hits a division-by-zero-shaped
  `chunks_exact(0)` panic in `decode_pixel_data_frame`) - `BitsAllocated=1` is explicitly allowed
  by this fix's `1 | 8 | 16 | 32` check, so it is untouched by this patch; see "What's not done".
- `cargo test -p dicom-pixeldata --lib` (default features, no `gdcm`): unaffected, **39 passed;
  0 failed; 2 ignored** (this module is `#[cfg(feature = "gdcm")]`-gated entirely).

## What's not done / open questions

- The originally-suspected `Rows * Columns * NumberOfFrames` 32-bit (`unsigned int`) wraparound
  in GDCM's own `Bitmap::GetBufferLength()` is real (confirmed by code reading, not just
  theorized) but turned out to be self-consistently bounded inside the RLE/JPEG `Try*Codec`
  buffer-copy logic this session checked - it was not separately proven to cause a heap overflow
  (as opposed to silent truncated/wrong decoded output) via any concrete repro. The JPEG2000 and
  JPEG-LS codec paths inside vendored GDCM were not read this session and could plausibly handle
  this differently; a future session could specifically target those.
- The two pre-existing, unrelated failures noted above
  (`tests::test_1bit_image_decoding_data`/`_frame`, a `BitsAllocated=1`-triggered
  division-by-zero-shaped panic in `pixeldata/src/gdcm.rs`'s `decode_pixel_data_frame`) were
  noticed as a byproduct of this session's regression check and were added to `IDEAS.md`'s
  backlog rather than investigated here (out of scope for this candidate, which is about
  `BitsAllocated` values *outside* the legal set, not `BitsAllocated=1` itself).
- This fix only covers `pixeldata/src/gdcm.rs`'s two `PixelDecoder` methods. Whether the same
  "individually-legal-but-nonsensical `BitsAllocated`" class reaches GDCM through any other path
  (e.g. via `dicom-transcode`'s use of the same crate) was not separately checked.
