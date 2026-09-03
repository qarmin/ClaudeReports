# gdcm-bitmap-tryjpegcodec2-tryjpeg2000codec2-unreachable

From `IDEAS.md`'s backlog (2026-08-27 entry, noticed while working candidate `103`): vendored
GDCM's `gdcmBitmap.cxx` declares `std::ostream`-based `Bitmap::TryJPEGCodec2`/`TryJPEG2000Codec2`,
noted as unread siblings of the already-confirmed-vulnerable `TryJPEGCodec`/`TryJPEG2000Codec`
(candidates `085`/`096`/`100`/`103`'s declared-size-vs-actual-decoded-size heap-overflow class).
Candidate `103` flagged the surrounding call site as "apparently `#if 0`-guarded, not confirmed."
This candidate reads both functions and traces their real reachability from this crate's `gdcm`
feature.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `gdcmBitmap.cxx`'s only two call sites for `TryJPEGCodec2`/`TryJPEG2000Codec2` (`Bitmap::GetBuffer2`, lines 1020-1034 of the pinned `gdcm-rs-0.6.0` vendored source) | vendored GDCM 3.1 sources bundled inside `gdcm-rs-0.6.0` (pinned exact version, checked against this repo's own `Cargo.lock`) | `GetBuffer2` is explicitly commented `// Compress the raw data` - it is the *encode* counterpart of the *decode* entry point `Bitmap::GetBuffer`/`GetBufferInternal` (lines 993-1017) that candidates 085/096/100/103 already traced. Inside `GetBuffer2`: `TryJPEGCodec2(os)` is called live (line 1024); `TryJPEG2000Codec2(os)` is source-commented-out (line 1025: `//if( !success ) success = TryJPEG2000Codec2(os);`) - unconditionally dead regardless of build config, same shape as candidate `102`'s `TryKAKADUCodec` finding |
| `grep -rn "GetBuffer2\b"` across the *entire* vendored `gdcm-rs-0.6.0/` tree (not just `Source/`, to also catch `Applications/`/`Examples/`/`Testing/`) | one-shot, deterministic | Exactly three hits total: `gdcmBitmap.h`'s declaration, `gdcmBitmap.cxx`'s own definition, and one more inside `GDCM/Examples/Cxx/CompressImage.cxx:71` - which is itself source-commented-out (`//image.GetBuffer2(out);`). **No live caller of `GetBuffer2` exists anywhere in the vendored tree** - it is unconditionally dead code, one level up from the already-dead `TryJPEG2000Codec2` inside it |
| Cross-check against `gdcm-rs-0.6.0/build.rs` and this crate's own `gdcm_wrapper.cc` (the only C++ code dicom-rs's `pixeldata` crate actually compiles/calls into GDCM through) | `grep -n "GDCM_BUILD_EXAMPLES" build.rs`; `grep -n "GetBuffer\|TryJPEG" gdcm_wrapper.cc` | `build.rs` sets `GDCM_BUILD_EXAMPLES = "OFF"`, so even the one other (dead) reference inside `Examples/Cxx/CompressImage.cxx` is never compiled into this crate's build in the first place. `gdcm_wrapper.cc` calls only `image.GetBufferLength()` and `image.GetBuffer(...)` (the decode path) - it never references `GetBuffer2`, `TryJPEGCodec2`, or `TryJPEG2000Codec2` |
| Search of dicom-rs's own `pixeldata/src/gdcm.rs` (the public `decode_pixel_data()` entry point's Rust-side wrapper) for any encode-shaped call | `grep -n "GetBuffer2\|TryJPEGCodec2\|TryJPEG2000Codec2\|encode" pixeldata/src/gdcm.rs` | No match beyond an unrelated code comment; this file only calls into `gdcm_rs::decode_single_frame_compressed`/`decode_multi_frame_compressed`, which only reach `GetBuffer`/`GetBufferInternal` as established above - there is no encode path exposed by this crate's `gdcm` feature at all |

## Status

**Not reproducible.** Neither `Bitmap::TryJPEGCodec2` nor `Bitmap::TryJPEG2000Codec2` is reachable
from this crate's `gdcm` feature, in the exact pinned `gdcm-rs-0.6.0` build this repository
produces:

- Both functions are only ever called (or would be called) from `Bitmap::GetBuffer2`, GDCM's
  *encoding* ("Compress the raw data") entry point - not the *decoding* entry point
  (`GetBuffer`/`GetBufferInternal`) that `gdcm_wrapper.cc` and `pixeldata/src/gdcm.rs`'s public
  `decode_pixel_data()` API actually use.
- `GetBuffer2` itself has zero live callers anywhere in the whole vendored `gdcm-rs-0.6.0` tree
  (confirmed by a full-tree `grep`, not just `Source/`) - the one other textual occurrence, in
  `GDCM/Examples/Cxx/CompressImage.cxx`, is itself source-commented-out, and `Examples/` isn't
  even compiled by `build.rs` (`GDCM_BUILD_EXAMPLES=OFF`).
- Independently, `TryJPEG2000Codec2`'s own call site inside `GetBuffer2` is *also*
  source-commented-out, so even a hypothetical future caller of `GetBuffer2` would still never
  reach `TryJPEG2000Codec2` without a further source change.

This matches `CONFIRMATION.md`'s guidance (already applied identically in candidate `102`) that a
definitive negative from reading real, pinned source plus this project's own real build
configuration is a valid, complete "Not reproducible" result on its own, without needing a
dynamic/fuzz/sanitizer run against a path that provably cannot execute - here the reachability gap
is a plain call-graph fact (zero call sites, not even a preprocessor-gated one), stronger than
candidate `102`'s PVRG/KAKADU case which still depended on a build-time CMake flag.

**Fully reproducible**: n/a (Not reproducible - no bug to reproduce).

## Class

n/a (Not reproducible).

## Root cause

Not a reachable bug, for two independent reasons stacked on top of each other:

1. **Wrong direction.** `TryJPEGCodec2`/`TryJPEG2000Codec2` are GDCM's `std::ostream`-based
   *encode* helpers (`TryJPEGCodec2` calls `codec.Code(PixelData, out)`, GDCM's compression
   entry point), reached only through `Bitmap::GetBuffer2` ("Compress the raw data"). The
   already-confirmed sibling bugs (candidates 085/096/100/103) live in the *decode* helpers
   `TryJPEGCodec`/`TryJPEG2000Codec`/`TryJPEGLSCodec`, reached through the separate
   `Bitmap::GetBuffer`/`GetBufferInternal` decode entry point. dicom-rs's `pixeldata` crate (via
   `gdcm_wrapper.cc`) only ever calls the decode entry point - it has no encode path through GDCM
   at all, so `GetBuffer2` and everything under it are structurally out of scope regardless of any
   internal bug shape they might have.
2. **Dead code on top of that.** Even setting aside the direction mismatch, `GetBuffer2` has no
   live caller anywhere in the vendored source tree GDCM itself ships (its only other reference is
   commented out, in a file that isn't even built), and `TryJPEG2000Codec2`'s own call site inside
   `GetBuffer2` is independently commented out too - so this is unreachable "for its own sake",
   not just unreachable from this crate specifically. This is the same unconditional-dead-code
   pattern candidate `102` found for `TryKAKADUCodec`.

## The fix

No fix needed/attempted - there is nothing to patch on a path that cannot execute in this crate's
build (and would need to be reached through GDCM's *encode* direction even if it were reachable,
which this crate never exercises).

- Project test suite: not run before/after, since no code was changed - N/A (no fix attempted).

## What's not done / open questions

- This only settles reachability through `gdcm_wrapper.cc`/`pixeldata/src/gdcm.rs`'s actual usage
  of GDCM (the decode-only path this crate's public API exposes). It does not audit whether
  `TryJPEGCodec2`'s own body (which does call live, encode-direction GDCM codec code, unlike
  `TryJPEG2000Codec2`) has a latent bug of its own shape that would matter to a hypothetical future
  caller that adds pixel-data *encoding* support to this crate - out of scope since no such caller
  exists today.
- Not reported upstream to the GDCM project - same "not reported upstream" caveat already on
  record for candidates `091`/`100`/`102`.
- This closes the specific open item candidate `103` left ("`TryJPEGCodec2`/`TryJPEG2000Codec2` ...
  apparently unused ... not confirmed") for good.
