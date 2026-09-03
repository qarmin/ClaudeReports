# gdcm-jpeg2000-jpegls-declared-dims-mismatch-heap-overflow

From `IDEAS.md`'s backlog (2026-08-25 entry, a follow-on from candidate `096`): GDCM's own
`Bitmap::GetBufferLength()` computes a declared pixel-buffer length purely from the DICOM
header's own `Rows`/`Columns`/`BitsAllocated`/`SamplesPerPixel`, entirely independent of
whatever the *actual* embedded compressed codestream really encodes. Candidate `096` checked
the RLE/JPEG `Try*Codec` buffer-copy paths and found both consistently reused the *same*
(possibly wrapped) length for both allocation and the final copy, so no provable overflow was
found there. This candidate reads the JPEG 2000 (`gdcmJPEG2000Codec.cxx`) and JPEG-LS
(`gdcmJPEGLSCodec.cxx`) codec internals `096` explicitly left unread, looking for the same
"declared header size disagrees with actual decoded size, and the mismatch isn't bounds-checked"
pattern.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the real vendored GDCM 3.1 C++ sources bundled inside the `gdcm-rs` crate (`gdcmBitmap.cxx`'s `Bitmap::GetBufferInternal`/`TryJPEG2000Codec`/`TryJPEGLSCodec`/`TryJPEGCodec`/`TryRLECodec`, `gdcmJPEG2000Codec.cxx`'s `Decode`/`DecodeByStreamsCommon`, `gdcmJPEGLSCodec.cxx`'s `Decode`/`DecodeByStreamsCommon`) | Traced how each codec's decoded output size relates to the declared `len = GetBufferLength()` used for the final `memcpy`/buffer allocation | Found two distinct, real mismatches (see "Root cause") - both different from `096`'s RLE/JPEG finding, and different from each other in direction (over-write vs over-read) |
| Debug build, real public API (`cargo test -p dicom-pixeldata --features gdcm --lib`), temporary in-tree `#[test]`s (reverted before this write-up): opened the real, unmodified `pydicom/MR_small_jp2klossless.dcm` and `pydicom/MR_small_jpeg_ls_lossless.dcm` fixtures (genuine 64x64 embedded codestreams) via `dicom_object::open_file`, then patched only `Rows`/`Columns` via the public `put_element` API (JPEG2000: down to 32x32; JPEG-LS: up to 256x256), then called `.decode_pixel_data()` | `cargo test -p dicom-pixeldata --features gdcm --lib bughunt_jpeg2000_declared_dims_larger_than_actual_codestream -- --nocapture` / `bughunt_jpegls_...` , 3 runs each | JPEG2000: crashed 3/3 with varying glibc heap-corruption symptoms (`double free or corruption (out)`, `corrupted size vs. prev_size`, `SIGSEGV`) - the classic non-deterministic-symptom signature of a real heap-buffer-overflow **write**. JPEG-LS: crashed 3/3 deterministically with `gdcmBitmap.cxx:731: ... Assertion 'len <= outbv->GetLength()' failed` (SIGABRT) |
| Same two temporary tests, `--release` Cargo profile (GDCM built CMake-Release/`NDEBUG`, a pristine build already cached in `target/release` from an earlier session) | `cargo test --release -p dicom-pixeldata --features gdcm --lib ... -- --nocapture`, 3 runs each | JPEG2000: crashed 3/3 again (`double free or corruption (out)` x2, `munmap_chunk(): invalid pointer` x1) - the real heap-write overflow doesn't need the assertion, it corrupts memory regardless of `NDEBUG`. JPEG-LS: crashed 3/3 with `SIGSEGV` (the guarding `assert()` is compiled out under `NDEBUG`, so the out-of-bounds `memcpy` read itself now runs far enough to hit an unmapped page) |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`, real public `dicom_pixeldata::PixelDecoder::decode_pixel_data()` API, `dicom-test-files` crate for the real fixtures) against a fresh clone of pristine upstream `df2b9e42`, via `just test` | Debug and Release Cargo builds, 3 runs each, both variants (12 runs total) | Reproduced identically in every run: non-zero exit (signal 6 or 11), all 12/12 crashed. Full output captured in `output.txt` |

## Status

**Confirmed** - deterministic in the sense that every single run crashes (12/12 across debug
Cargo build/release Cargo build x JPEG2000/JPEG-LS, both in-tree and in the standalone clone);
the exact *symptom* (SIGSEGV vs. which glibc heap-corruption message vs. a clean assertion
SIGABRT) varies run to run for the JPEG2000 write-overflow, which is expected and itself
characteristic of genuine heap corruption (the corrupted allocator metadata is only actually
*detected* whenever the next `malloc`/`free` call happens to inspect the corrupted region) - see
`CONFIRMATION.md`'s note that a reproduction is still real when it fires every time even if the
specific failure mode isn't identical each time.

**Fully reproducible**: **Yes**. `repro/` drives the real, public
`dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point (the `gdcm` feature, i.e. the
exact FFI path `gdcm-rs`/vendored GDCM uses) on real, unmodified `pydicom` DICOM fixtures
(genuine JPEG 2000/JPEG-LS-encoded pixel data) opened through this crate's own public
`dicom_object::open_file`, with only the `Rows`/`Columns` header attributes subsequently patched
through the crate's own public `put_element` API - the same public entry point any real caller
(a DICOM viewer, a PACS client parsing an incoming file) would use, and the same kind of
"attacker/producer-controlled header field disagrees with the actual embedded data" input a real
malformed or adversarially-crafted DICOM file would supply. There is no second "peer" process
needed here, per `CONFIRMATION.md` section 11's carve-out for a single malformed/inconsistent
input handed straight to a parser's own public entry point.

## Class

`memory-safety` (two distinct real heap-buffer-overflow bugs - one write, one read - inside a
third-party C++ library reached via FFI, both triggered by nothing more than an ordinary DICOM
file whose header `Rows`/`Columns` don't match its own embedded compressed pixel data; a natural
crash/DoS vector, and for the write-overflow case a plausible route to worse-than-DoS memory
corruption, for any service decoding untrusted DICOM pixel data with the `gdcm` feature enabled).

## Root cause

Two independent, differently-shaped bugs, both stemming from the same underlying design flaw:
GDCM's `Bitmap::GetBufferLength()` (`gdcmBitmap.cxx:250`) computes a "declared" buffer length
purely from the DICOM header's own `Rows`/`Columns`/`BitsAllocated`/`SamplesPerPixel`
(propagated from this crate's own `pixeldata/src/gdcm.rs`, which reads them straight from the
DICOM object with no cross-check against the pixel data itself), while the JPEG 2000 and
JPEG-LS codecs each decode the *actual* embedded compressed codestream, which carries its own,
completely independent width/height/component-count baked into the compressed format itself.
Nothing on this path ever checks that the two agree before trusting both:

1. **JPEG 2000 heap-buffer-overflow write** - `JPEG2000Codec::DecodeByStreamsCommon`
   (`gdcmJPEG2000Codec.cxx:812-814`):
   ```cpp
   // Copy buffer
   unsigned long len = Dimensions[0]*Dimensions[1] * (PF.GetBitsAllocated() / 8) * image->numcomps;
   char *raw = new char[len];
   ```
   `Dimensions` here is the **declared** header size (set via `Bitmap::TryJPEG2000Codec`'s
   `codec.SetDimensions(GetDimensions())`, i.e. attacker/producer-controlled `Rows`/`Columns`).
   The very next loop (`gdcmJPEG2000Codec.cxx:816-885`) fills `raw` using `wr`/`hr` -
   `image->comps[compno].w`/`h`, the **actual** dimensions OpenJPEG decoded from the real
   embedded codestream, entirely unrelated to `Dimensions`. If the declared header dimensions
   are *smaller* than what's actually embedded, this loop writes past the end of the
   `len`-byte `raw` buffer - a straightforward heap-buffer-overflow **write**, inside GDCM's
   own decode function, with no guard of any kind (not even a compiled-out `assert`).
2. **JPEG-LS heap-buffer-overflow read** - `Bitmap::TryJPEGLSCodec` (`gdcmBitmap.cxx:716-734`):
   ```cpp
   unsigned long len = GetBufferLength();
   ...
   bool r = codec.Decode(PixelData, out);
   ...
   const ByteValue *outbv = out.GetByteValue();
   assert( len <= outbv->GetLength() );
   memcpy(buffer, outbv->GetPointer(), len /*outbv->GetLength()*/ );  // FIXME
   ```
   `len` is again the **declared** header size. `outbv` is the JPEG-LS codec's own decoded
   output, which - unlike the JPEG 2000 case above - *is* self-consistently sized from the
   actual codestream's own embedded header inside `JPEGLSCodec::DecodeByStreamsCommon`
   (`gdcmJPEGLSCodec.cxx:170`: `rgbyteOut.resize(params.height * params.width * ...)`, using
   `params` read directly from the codestream by `JpegLsReadHeader`) - so `outbv`'s real size
   is independent of `len`. The only guard against `len` exceeding `outbv`'s real size is an
   `assert()`, compiled out under `NDEBUG` (an ordinary Release build of GDCM, i.e. what a real
   `cargo build --release` deployment's `cmake`-crate invocation would normally produce). This
   is the same shape of bug already present *and already runtime-guarded* one function up in
   this very file: `Bitmap::TryJPEGCodec` (`gdcmBitmap.cxx:538-542`) has a real, always-compiled
   `if (len > outbv->GetLength()) { gdcmErrorMacro(...); return false; }` check immediately
   before its own analogous `memcpy` - `TryJPEGLSCodec` (and, by the same pattern,
   `TryPVRGCodec`/`TryKAKADUCodec`, not independently confirmed this session) never got the
   equivalent runtime check, only the `assert()` that only fires in a Debug build. If the
   declared header dimensions are *larger* than what's actually embedded, this is a
   heap-buffer-overflow **read** past the end of `outbv`'s real allocation (into `buffer`,
   which is correctly sized to `len` bytes on the caller/Rust side, so the read is the unsafe
   part here, not the write).

Both are reachable from `pixeldata/src/gdcm.rs`'s `decode_pixel_data`/`decode_pixel_data_frame`,
which read `Rows`/`Columns`/`BitsAllocated`/`SamplesPerPixel` straight from the DICOM object with
no validation against the actual pixel data, and hand them into `gdcm_rs::decode_single_frame_compressed`/
`decode_multi_frame_compressed` unchanged.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

**No bounded fix found this session.** The actual defect in both cases lives entirely inside
vendored, third-party GDCM 3.1 C++ source bundled by the `gdcm-rs` crate
(`~/.cargo/registry/.../gdcm-rs-0.6.0/GDCM/Source/MediaStorageAndFileFormat/gdcmJPEG2000Codec.cxx`
and `gdcmBitmap.cxx`) - outside this repository's own git-tracked working tree, so there is no
"real source" inside the `dicom-rs` project itself to patch the way candidate `096`'s
`BitsAllocated`-range check could be added on the Rust side.

A Rust-side mitigation in `pixeldata/src/gdcm.rs` (the same file candidate `096` fixed) is not a
simple range-check this time: unlike `096`'s `BitsAllocated` (where a single fixed legal-value
set - `1`/`8`/`16`/`32` - could be checked with no knowledge of the pixel data itself), the defect
here is a *mismatch* between two numbers that are only both known once one of them (the actual
embedded codestream's own width/height) has been independently parsed out of the compressed
JPEG 2000/JPEG-LS bytes *before* ever calling into `gdcm_rs`. Doing that correctly would mean
re-implementing a real (if partial) JPEG 2000 SIZ-marker parser and a JPEG-LS SOF55-marker parser
directly in this crate - substantial new parsing logic, itself a new attack surface if done
hastily, and not something that can be soundly bounded and verified within one session. (Candidate
`091`'s existing `jpegls.rs` fix took exactly this approach for the *native*, non-`gdcm` JPEG-LS
decoder path, where it could reuse the already-integrated `charls` crate's own header-parsing
entry point rather than writing a new parser from scratch - no equivalent shortcut exists here for
the `gdcm` FFI path.)

- Project test suite: not run before/after, since no code was changed - N/A (no fix attempted).

## What's not done / open questions

- `Bitmap::TryPVRGCodec` and `Bitmap::TryKAKADUCodec` (`gdcmBitmap.cxx`) have the exact same
  shape as `TryJPEGLSCodec`'s bug (`assert(len <= outbv->GetLength())` with no compiled-in
  runtime check) - not independently confirmed this session since neither PVRG nor KAKADU
  appears reachable from this crate's own transfer-syntax dispatch (`gdcm_rs`'s
  `GDCMTransferSyntax` mapping would need to be checked to be sure), but worth a follow-up read
  if either turns out to be reachable.
- The JPEG 2000 write-overflow's exact overflow amount is bounded by how far the declared
  `Rows`/`Columns` fall short of the real embedded codestream's dimensions - not fuzzed for a
  worst-case/most-exploitable size relationship (e.g. declaring `1x1` against a very large real
  embedded image) this session; the two example sizes used here (32x32 declared vs. real 64x64)
  were chosen only to demonstrate the mismatch cleanly, not to maximize the overflow.
  Multi-frame (`NumberOfDimensions == 3`) JPEG 2000/JPEG-LS decoding was not separately checked
  for the same class of mismatch (both have their own, differently-shaped 3-D decode branches
  not read in depth this session).
- Not reported upstream to the GDCM project - out of scope for this session, but see
  candidate `091`'s `IDEAS.md` note for the same "not reported upstream" caveat on a different
  third-party dependency (`charls`); this one likely deserves the same treatment eventually.
