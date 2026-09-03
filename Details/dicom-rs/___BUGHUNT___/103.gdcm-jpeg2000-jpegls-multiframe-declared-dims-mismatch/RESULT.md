# gdcm-jpeg2000-jpegls-multiframe-declared-dims-mismatch

From `IDEAS.md`'s backlog (2026-08-25 entry, a follow-on from candidate `100`): candidate `100`
confirmed a real heap-buffer-overflow in vendored GDCM's JPEG 2000 (`gdcmJPEG2000Codec.cxx`) and
JPEG-LS (`gdcmBitmap.cxx`/`gdcmJPEGLSCodec.cxx`) decode paths, where the output buffer is sized
from the DICOM header's *declared* `Rows`/`Columns` but filled/compared against the *actual*
decoded codestream size. That candidate's repro only used single-fragment files. This candidate
reads GDCM's separate `NumberOfDimensions == 3` (multi-frame) branch in both codecs, which it
left unread, for the same pattern, and attempts a real end-to-end repro with genuine multi-frame
`pydicom` fixtures.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the real vendored GDCM 3.1 C++ sources (`gdcmJPEG2000Codec.cxx::Decode`/`DecodeByStreamsCommon`, `gdcmJPEGLSCodec.cxx::Decode`, `gdcmBitmap.cxx::TryJPEG2000Codec`/`TryJPEGLSCodec`/`GetBufferLength`) | Traced the `NumberOfDimensions == 3` branch of both codecs' `Decode()`, and dicom-rs's own `gdcm-rs` C++ wrapper (`gdcm_wrapper.cc`) | Found that the 3-D branch in both codecs is a per-fragment loop that calls the *exact same* per-frame decode primitives already implicated by candidate 100 (`JPEG2000Codec::DecodeByStreamsCommon`'s `raw = new char[Dimensions[0]*Dimensions[1]*...]` write; `JPEGLSCodec`'s self-sized-from-codestream `rgbyteOut`/outer `TryJPEGLSCodec`'s `assert(len <= outbv->GetLength())` read guard) - once per fragment/frame instead of once. **Also found: dicom-rs's own `gdcm_wrapper.cc::c_decode_frames` always calls `image.SetNumberOfDimensions(3)` unconditionally**, for both `decode_single_frame_compressed` (`dims = [w, h, 1]`) and `decode_multi_frame_compressed` (`dims = [w, h, N]`) - see "Correction to candidate 100" below |
| Debug build, real public API (`cargo test -p dicom-pixeldata --features gdcm,image --lib`), temporary in-tree `#[test]`s (reverted before this write-up): opened real, unmodified, genuinely multi-frame `pydicom/emri_small_jpeg_2k_lossless.dcm` and `pydicom/emri_small_jpeg_ls_lossless.dcm` fixtures (10 real frames, confirmed via a temporary `#[test]` printing `SequenceOfFragments::fragments().len() == 10` for both) via `dicom_object::open_file`, patched only `Rows`/`Columns` via the public `put_element` API (JPEG2000: down to 32x32; JPEG-LS: up to 256x256; real per-frame codestream is 64x64), then called `.decode_pixel_data()` | `cargo test -p dicom-pixeldata --features gdcm,image --lib bughunt_multiframe_declared_dims_mismatch -- --nocapture --test-threads=1`, 3 runs each variant | JPEG2000: crashed 3/3 (2x SIGSEGV, 1x SIGABRT with a varying glibc heap-corruption message) - same non-deterministic-symptom signature as candidate 100's write-overflow. JPEG-LS: crashed 3/3 deterministically with the identical `gdcmBitmap.cxx:731: ... Assertion 'len <= outbv->GetLength()' failed` (SIGABRT) candidate 100 found |
| Same two temporary tests, `--release` Cargo profile (triggers GDCM's own cmake-crate Release build) | `cargo test --release -p dicom-pixeldata --features gdcm,image --lib ... -- --nocapture`, 3 runs each | JPEG2000: crashed 3/3 (SIGABRT x2 with different glibc messages, SIGSEGV x1). JPEG-LS: crashed 3/3 with SIGSEGV (assert compiled out under this GDCM build's NDEBUG, same as candidate 100's release finding) |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`, real public `dicom_pixeldata::PixelDecoder::decode_pixel_data()` API, `dicom-test-files` crate for the real multi-frame fixtures) against a fresh clone of pristine upstream `df2b9e42`, via `just test` | `just clone && just reproduce` (default/dev cargo profile) plus 3 additional direct `cargo run` repeats per variant | Reproduced in every run: jpeg2000 variant crashed 5/5 (1 via `just reproduce`, 2 earlier ad hoc, 3 in the final batch - SIGSEGV / 3 differently-worded glibc heap-corruption SIGABRTs); jpegls variant crashed 5/5, identical assertion-failure SIGABRT every time. Full output captured in `output.txt` |

## Status

**Confirmed** - deterministic in the sense that every run crashes (6/6 in-tree debug+release x 2
variants, plus 10/10 in the standalone `repro/` clone = 16/16 total across both variants); the
exact *symptom* for the JPEG2000 write-overflow varies run to run (SIGSEGV vs. which glibc
heap-corruption message), which is expected and characteristic of genuine heap corruption (see
`CONFIRMATION.md` section 3) - the JPEG-LS read-overflow's *assertion* symptom (debug builds) is
fully deterministic every run, and turns into SIGSEGV once GDCM's own assertions are compiled out
(release builds), matching candidate 100's exact finding.

**Fully reproducible**: **Yes**. `repro/` drives the real, public
`dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point (the `gdcm` feature) on real,
unmodified, genuinely multi-frame `pydicom` fixtures (10 real embedded JPEG 2000/JPEG-LS frames,
one per fragment), with only the `Rows`/`Columns` header attributes patched through this crate's
own public `put_element` API - the same public entry point any real caller (a DICOM viewer, a
PACS client) would use, and the same "declared header disagrees with actual embedded data per
frame" input a real malformed/adversarial multi-frame DICOM file would supply. No second "peer"
process is needed, per `CONFIRMATION.md` section 11's carve-out for a malformed/inconsistent
input handed straight to a parser's own public entry point.

## Class

`memory-safety` (the same two heap-buffer-overflow bugs candidate 100 found - one write, one
read - inside third-party GDCM reached via FFI, now confirmed to additionally fire when
processing a genuine multi-frame file's per-frame decode loop, not just a single frame).

## Root cause

Same underlying defect as candidate `100`: GDCM's `Bitmap::GetBufferLength()` computes a
declared total buffer length purely from the DICOM header's own
`Rows`/`Columns`/`NumberOfFrames`/`BitsAllocated`/`SamplesPerPixel`, independent of what the
*actual* embedded compressed codestream(s) encode. What this candidate adds:

1. **The `NumberOfDimensions == 3` branch is not a differently-shaped, separately-guarded code
   path - it is a per-fragment loop around the exact same unguarded primitives candidate 100
   already found.** `JPEG2000Codec::Decode`'s 3-D branch (`gdcmJPEG2000Codec.cxx:560-598`) calls
   `DecodeByStreams`/`DecodeByStreamsCommon` once per fragment; each call independently allocates
   `raw = new char[Dimensions[0]*Dimensions[1]*(BitsAllocated/8)*numcomps]` (declared per-frame
   size) and fills it from the actual per-frame codestream's own `wr`/`hr` dimensions
   (`gdcmJPEG2000Codec.cxx:812-885`) - so the same write-overflow now fires once per frame instead
   of once. `JPEGLSCodec::Decode`'s 3-D branch (`gdcmJPEGLSCodec.cxx:207-265`) similarly decodes
   each fragment into a `rgbyteOut` self-sized from that fragment's own JPEG-LS header
   (`params.height`/`params.width`), concatenates all frames into one `DataElement`, and the
   outer `Bitmap::TryJPEGLSCodec` (`gdcmBitmap.cxx:716-734`) still only guards the final
   declared-vs-actual comparison with a compiled-out `assert(len <= outbv->GetLength())` before
   `memcpy(buffer, outbv->GetPointer(), len)` - now comparing the declared *total* (`Rows *
   Columns * NumberOfFrames * BytesPerPixel`) against the *sum* of the actual per-frame decoded
   sizes, but the missing-runtime-check defect is identical.
2. **Correction to candidate 100's "2-D" framing**: candidate 100's `RESULT.md` describes its
   finding as being in GDCM's `NumberOfDimensions == 2` branch. That is inaccurate. dicom-rs's own
   `gdcm-rs` C++ wrapper (`gdcm_wrapper.cc::c_decode_frames`, called by both
   `decode_single_frame_compressed` and `decode_multi_frame_compressed` in `gdcm-rs`'s
   `src/lib.rs`) unconditionally calls `image.SetNumberOfDimensions(3)`, regardless of frame
   count (`dims[2]` is `1` for the single-frame case, `NumberOfFrames` otherwise). There is no
   code path from this crate's `gdcm` feature that ever reaches GDCM's true
   `NumberOfDimensions == 2` branch - candidate 100's single-fragment repro already went through
   the `NumberOfDimensions == 3` branch with `Dimensions[2] == 1` (a one-iteration loop), not a
   genuinely different "2-D" branch. This candidate's contribution is confirming the *same* branch
   also crashes with `Dimensions[2] > 1` (a genuine multi-iteration loop over real per-frame
   fragments), which was the open question left by candidate 100 - not a structurally distinct new
   bug. See `100/CORRECTION.md` for the note added to that candidate's own record.

Both are reachable from `pixeldata/src/gdcm.rs`'s `decode_pixel_data`/`decode_pixel_data_frame`,
which read `Rows`/`Columns`/`NumberOfFrames`/`BitsAllocated`/`SamplesPerPixel` straight from the
DICOM object with no validation against the actual pixel data, and hand them into
`gdcm_rs::decode_multi_frame_compressed` unchanged whenever a file has more than one fragment.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

**No bounded fix found this session** - same reasoning as candidate 100. The defect lives
entirely inside vendored, third-party GDCM 3.1 C++ source bundled by the `gdcm-rs` crate
(`~/.cargo/registry/.../gdcm-rs-0.6.0/GDCM/Source/MediaStorageAndFileFormat/gdcmJPEG2000Codec.cxx`
and `gdcmBitmap.cxx`/`gdcmJPEGLSCodec.cxx`), outside this repository's own git-tracked working
tree. A Rust-side mitigation would require independently parsing each fragment's JPEG
2000/JPEG-LS codestream header (to learn its real per-frame width/height) before ever calling into
`gdcm_rs` - the same scope candidate 100 judged out of reach for one session, now additionally
needing to be done per-fragment rather than once.

- Project test suite: not run before/after, since no code was changed - N/A (no fix attempted).

## What's not done / open questions

- Same open items as candidate 100: `Bitmap::TryPVRGCodec`/`TryKAKADUCodec` not checked (not
  reachable from this crate's transfer-syntax dispatch as far as established); not reported
  upstream to the GDCM project.
- The multi-frame overflow's worst-case severity (how far a maximally-mismatched multi-frame file
  could push the per-frame write past its buffer, summed over many frames) was not explored beyond
  the two example fixtures used here (10 frames, 32x32/256x256 declared vs. 64x64 real).
- `Bitmap::TryJPEGCodec2`/`TryJPEG2000Codec2` (`gdcmBitmap.cxx`, `std::ostream`-based variants,
  apparently unused by the `#if 0`-guarded code around them) were not read - likely dead code, not
  confirmed.
