# pixeldata-transcode-unreachable-codec-safe

`pixeldata/src/transcode.rs` (the `Transcode` trait implementation, converting a DICOM object's
pixel data between transfer syntaxes - used by `storescu`'s `into_ts` and the pixeldata crate's
public API) has two `unreachable!("Unexpected codec from transfer syntax")` sites, each in a
`match ts.codec() { .. Codec::None => unreachable!(..) }` arm, reached from code paths that had
already branched on `ts.is_encapsulated_pixel_data()`/similar. Checked as part of continuing this
session's "diff a match statement's arms against the invariant the surrounding code actually
guarantees" technique (already high-yield this session and the prior one), since `transcode.rs`
itself had not been read at all yet despite `pixeldata/src/lib.rs` and `overlay.rs` (same crate)
each having a confirmed bug found this session (`037`, `038`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: traced both `Codec::None => unreachable!()` arms back to the branch condition that leads to each, and checked `TransferSyntax::is_encapsulated_pixel_data()`'s own definition | Static code reading, no build needed | Both sites are structurally guaranteed safe by Rust's enum exhaustiveness: `is_encapsulated_pixel_data()` (`encoding/src/transfer_syntax/mod.rs:520`) is defined as `matches!(self.codec, Codec::EncapsulatedPixelData(..))`. The first `unreachable!()` (line 170) is inside a match arm only reached when the outer `(current_ts.is_encapsulated_pixel_data(), ts.is_encapsulated_pixel_data()) == (true, true)`; the second (line 329, inside `decode_and_encode`) is called only from two sites, both reached only when `ts.is_encapsulated_pixel_data()` is `true` (one as a fallback within the same `(true, true)` arm, the other from a `(_, true)` arm). Since `Codec::EncapsulatedPixelData(..)` and `Codec::None` are mutually exclusive variants of the same `Codec` enum, `ts.is_encapsulated_pixel_data() == true` makes `ts.codec() == Codec::None` impossible by construction - both `unreachable!()` arms are correctly unreachable, matching their own inline comments ("already tested in `is_encapsulated_pixel_data`"/"already tested in `is_codec_free`", the latter comment using an imprecise but equivalent justification) |

## Status

**Not reproducible** - ruled out by tracing every call path to both sites back to the guard
condition that makes the `Codec::None` arm structurally impossible.

## Class

n/a

## Root cause

Not applicable (no bug) - both sites are guarded by prior enum-variant checks that are mutually
exclusive with the arm containing `unreachable!()`, verified directly against the referenced
predicate methods' own definitions rather than just trusting the inline comments' claims.

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

- Only the two `unreachable!()` sites and their direct call graph were checked - the rest of
  `transcode.rs` (726 lines, including `decode_inline`, the `EncodeOptions` handling, and the
  frame/offset-table bookkeeping around `PixelFragmentSequence::new`) was read in the course of
  tracing these two sites but not exhaustively re-audited for unrelated issues (e.g. the
  `total_pixeldata_len: u64 = fragments.iter().map(|f| f.len() as u64).sum()` computation nearby
  was noticed but not separately investigated - summing real fragment lengths already held in
  memory, unlikely to overflow `u64` in practice, judged low-priority relative to time remaining).
