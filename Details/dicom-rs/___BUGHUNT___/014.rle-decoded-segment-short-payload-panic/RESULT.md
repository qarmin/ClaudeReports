# RLE Lossless decoder panics when a segment's compressed payload decodes short

From `IDEAS.md`'s backlog, noticed while designing candidate 013's fix
(`013.rle-header-untrusted-nr-segments`): candidate 013 fixed the RLE segment *offset header*
validation, but flagged `decoded_segment[decoded_index]` (used right after, to copy a segment's
decoded bytes into the output buffer) as a separate, unvalidated indexing point that trusts a
segment's *compressed payload* decodes to at least as many bytes as the pixel-interleaving loop
expects - a different attack surface than the header.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `PackBitsReader::new` (decodes a segment's PackBits-compressed bytes eagerly, into an in-memory buffer, stopping whenever the compressed input is exhausted - no guarantee about how many *output* bytes that produces) and the `decoder.take(rows*cols).read_to_end(&mut decoded_segment)` call that follows it in both `decode()`/`decode_frame()`. Confirmed `decoded_segment.len()` can legitimately be smaller than `rows * cols` if the segment's compressed content is short, truncated, or simply doesn't encode enough pixels - and the interleaving loop right after unconditionally indexes `decoded_segment[decoded_index]` up to `rows * cols - 1` with no bounds check. |
| Standalone repro (release build, `dicom-encoding`+`dicom-transfer-syntax-registry`'s public API, same harness shape as candidate 013: a minimal `PixelDataObject` + `TransferSyntax::pixel_data_reader()`) | `___BUGHUNT___/014.../repro`. A fully well-formed RLE fragment (1 segment, matching `BitsAllocated=8`/`SamplesPerPixel=1`, offsets internally consistent and within bounds - passes every check candidate 013 added) whose single segment's PackBits content is just `[0x00, 0xAB]` ("copy 1 literal byte, value 0xAB") - a valid, 1-byte-decoding segment, for a 4x4 (`rows*cols=16`) image expecting 16 bytes from it. | Reproduced deterministically: `index out of bounds: the len is 1 but the index is 1` panicking in `decode()` at the interleaving loop, on a 10-byte, entirely well-formed-per-candidate-013's-checks input. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic, no randomness in this code path. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`crash` (an unguarded `Vec` index panic, reachable from a 10-byte crafted RLE fragment with an
otherwise entirely valid, spec-conformant header - this is specifically the gap candidate 013's
header validation could not close, since the header itself is genuinely fine here; only the
segment's compressed *content* is short).

## Root cause

`PackBitsReader::new` decodes a segment's PackBits-compressed bytes into an in-memory buffer by
running its decode loop until the compressed input is exhausted (`while bytes_read < length`) -
there is no guarantee, and no check anywhere, that the resulting buffer actually contains `rows *
cols` bytes (the number every pixel-interleaving loop in both `decode()` and `decode_frame()`
unconditionally assumes when it indexes `decoded_segment[decoded_index]` for `decoded_index` up to
`rows * cols - 1`). A segment can be well-formed by every check candidate 013 added (correct
offset bounds, consistent with the declared segment count) while still compressing to far fewer
output bytes than the image dimensions require - DICOM RLE doesn't guarantee a 1:1 relationship
between "segment passes header validation" and "segment decodes to the expected pixel count";
that's a property of the segment's own compressed content, checked nowhere in the original code.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replaced the direct `decoded_segment[decoded_index]` indexing in both `decode()` and
`decode_frame()` with `decoded_segment.get(decoded_index)`, returning a clear `Result::Err`
(`whatever!`) instead of panicking when a segment's decoded content runs out before the
interleaving loop expects it to. See `patch.diff` - **bundled as the full combined diff** (both
candidate 013's original fix and this candidate's addition), for the same self-containment reason
as candidates 007/010: this fix depends on candidate 013's already having been applied to the same
file, and per the skill's rule a candidate folder may not reference a sibling folder.

Verified: `repro/` re-run against the patched tree now reports a clear error
(`"RLE segment decoded to only 1 byte(s), expected at least 2"`) instead of panicking.

- Project test suite: `cargo test -p dicom-transfer-syntax-registry -p dicom-pixeldata --lib
  --features dicom-transfer-syntax-registry/rle` and `cargo test --workspace --lib` - 351 passed /
  0 failed / 2 ignored, identical both before and after the patch (no regressions, no
  newly-passing tests - no existing test, including the real RLE sample files exercised by
  `dicom-pixeldata`'s own integration tests, hits a segment that decodes short).

## What's not done / open questions

- The error message reports "expected at least N" using only the *current* `decoded_index` at the
  point of failure, not the full expected count (`rows * cols`) - a minor diagnostic-completeness
  gap, not a correctness issue (the `Err` itself is still always correct and always returned
  before any out-of-bounds access happens).
- This closes the two specific gaps identified while investigating RLE Lossless decoding this
  session (header validation in 013, payload-length validation here) - no further systematic sweep
  of the function was done beyond what these two candidates' repros specifically targeted. A
  broader fuzz campaign against this adapter (seeded with the real RLE sample files already used
  by `dicom-pixeldata`'s tests) would be a reasonable next step to look for further edge cases in
  the same area, not attempted this session.
