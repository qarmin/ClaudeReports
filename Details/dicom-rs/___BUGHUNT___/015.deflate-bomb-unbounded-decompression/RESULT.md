# Deflated Image Frame Compression decoder has no decompression-bomb protection

Found during a discovery round (no seeded lead): after finding two real bugs in
`rle_lossless.rs` (candidates 013/014), checked the sibling pixel data adapters in the same
directory for the same "under-tested, handles untrusted compressed data" risk profile - none of
`deflated.rs`, `jpeg.rs`, `jpeg2k.rs`, `jpegls.rs`, `jpegxl.rs`, `uncompressed.rs`, or `mod.rs`
have a `#[cfg(test)] mod test` at all (only `rle_lossless.rs` does). `deflated.rs` stood out
immediately: it pipes attacker-controlled compressed bytes straight into `std::io::copy` with
absolutely no size limit anywhere - the textbook shape of a decompression-bomb (CWE-409)
vulnerability, a well-known class this project's own `flate2` dependency exists specifically to
guard against if used carelessly.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `DeflatedImageFrameAdapter::decode`/`decode_frame` (`transfer-syntax-registry/src/adapters/deflated.rs`) - both call `std::io::copy(&mut decoder, dst)` (a `flate2::read::DeflateDecoder` wrapping raw fragment bytes) with no `.take()`, no size check, and no relationship at all to the object's own declared `Rows`/`Columns`/`SamplesPerPixel`/`BitsAllocated` (which `encode_frame`, in the very same file, *does* compute and use - the decode side simply never adopted the equivalent check). |
| Standalone repro (release build, `dicom-encoding`+`dicom-transfer-syntax-registry`'s public API: a minimal `PixelDataObject` + `TransferSyntax::pixel_data_reader()`, same harness shape as candidates 013/014) | `___BUGHUNT___/015.../repro`. A `PixelDataObject` declaring a 4x4, 1-sample, 8-bit image (16 bytes expected) whose single fragment is a real, valid DEFLATE stream of 260,735 bytes compressing 256 MiB of all-zero data (a ~1029x amplification ratio, well within what plain single-stream DEFLATE can achieve, no nested/exotic construction needed) built with the project's own `flate2` dependency. Called `.decode()` directly - the exact code path `pixeldata`'s own `decode_pixel_data()` uses for this transfer syntax. | Reproduced deterministically: `decode()` happily produced the full 268,435,456-byte (256 MiB) output in ~140ms, for an image whose own declared dimensions call for exactly 16 bytes - no size check, no cap, no error, anywhere in the call chain. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic (a real, static compressed payload; no randomness). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`security` (a classic decompression-bomb / resource-exhaustion DoS: a modestly-sized, entirely
plausible-looking DICOM file - a few hundred KB - can force multiple hundred-MB-to-GB-scale
allocations and copies per fragment, for a transfer syntax with no inherent size relationship
enforced between "declared image dimensions" and "how much data the compressed stream is allowed
to expand into." Multiple fragments compound the effect, since each is decompressed and
accumulated into the same unbounded `dst` independently).

## Root cause

`decode()`/`decode_frame()` never establish any upper bound on how many bytes
`std::io::copy(&mut decoder, dst)` is allowed to write, despite the object itself already
providing everything needed to compute a sane, precise expected size
(`Rows * Columns * SamplesPerPixel * ceil(BitsAllocated / 8)`, exactly what `encode_frame` in the
very same file already computes for the opposite direction). A tiny, highly-compressible
fragment - trivially producible with any general-purpose DEFLATE encoder, no specialized
"bomb-crafting" tooling required - can therefore force the decoder to allocate and copy far more
data than the image could ever legitimately contain.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added an `expected_frame_size` helper (mirroring `encode_frame`'s existing `frame_size`
computation) and used it to bound decompression in both `decode()` and `decode_frame()`: each
`flate2::read::DeflateDecoder` is wrapped in `.take(limit)` before `std::io::copy`, where `limit`
is the expected total size (summed across all frames for `decode()`) plus one byte of slack - the
extra byte lets the code distinguish "decoded exactly the expected amount" (valid) from "there was
still more data past the limit" (oversized, likely a bomb), which is checked immediately after and
turned into a clear `Result::Err` rather than either a silent truncation or an unbounded copy. See
`patch.diff` (`transfer-syntax-registry/src/adapters/deflated.rs`, one file, both call sites plus
the new shared helper).

Verified: `repro/` re-run against the patched tree now rejects the 256 MiB bomb in ~120
microseconds (over 1000x faster than the original unbounded run, since the `.take()` cap stops
`io::copy` from doing any of the wasted decompression work in the first place) with a clear error
message identifying the expected size and possible decompression-bomb cause. A second check added
directly to this candidate's repro confirms the fix does **not** reject legitimately-sized,
correctly-compressed data - a fragment that decompresses to exactly the expected 16 bytes still
decodes successfully and byte-for-byte correctly.

- Project test suite: `cargo test -p dicom-transfer-syntax-registry -p dicom-pixeldata --lib
  --features dicom-transfer-syntax-registry/deflate` and `cargo test --workspace --lib` - 351
  passed / 0 failed / 2 ignored, identical both before and after the patch (no regressions, no
  newly-passing tests - this transfer syntax has no dedicated real-world `.dcm` sample file in the
  project's existing test suite as of this session, per `grep`, so this candidate's own
  legitimate-data sanity check is the only direct confirmation that valid data still round-trips
  correctly).

## What's not done / open questions

- The `+1` slack byte approach means a fragment that decompresses to *exactly* one byte more than
  expected is correctly rejected, but the error is only raised after that one extra byte has
  already been read and copied into `dst` - a negligible, bounded amount of wasted work (at most
  one byte over the limit), not a meaningful residual concern.
- Not fuzzed - found by direct code reading against a well-known vulnerability class
  (decompression bombs), then confirmed by constructing a real DEFLATE stream with the project's
  own `flate2` dependency. `pixeldata`'s own fuzz targets could plausibly be extended to include
  this transfer syntax in their corpus/target mix if not already covered, though decompression-bomb
  detection specifically benefits more from a deliberate test like this one than from
  general-purpose fuzzing (a fuzzer optimizing for code coverage has no particular incentive to
  discover *high compression ratios*, which is what actually triggers this class of bug).
- Did not check whether `jpeg2k.rs`/`jpegxl.rs`/`jpegls.rs` (which delegate to `jxl-oxide`,
  `jpeg-decoder`, etc.) have equivalent decompression-bomb exposure - those crates likely have
  their own internal size/memory safeguards (being general-purpose, widely-used image codec
  libraries), unlike this project's own hand-written `deflated.rs` glue code, but this wasn't
  independently verified this session.
