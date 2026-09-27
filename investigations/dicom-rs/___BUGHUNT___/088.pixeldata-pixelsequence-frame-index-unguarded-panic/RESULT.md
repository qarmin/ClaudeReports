# pixeldata-pixelsequence-frame-index-unguarded-panic

`pixeldata/src/lib.rs`'s `FileDicomObject<InMemDicomObject<D>>::decode_pixel_data_frame` (the
crate's real override of the public `PixelDecoder` trait method, ~line 2407-2548) validates that
`number_of_frames as usize == fragments.len()` for its `DicomValue::PixelSequence` branch, but
never validates the caller-supplied `frame: u32` argument itself against `fragments.len()` before
indexing with `fragments[frame as usize]` (line 2511) - a plain out-of-bounds slice-index panic,
unlike the sibling `DicomValue::Primitive` branch a few lines below in the same match, which uses
`.get(frame_offset..frame_offset + frame_size).context(FrameOutOfRangeSnafu { .. })` for the exact
same kind of out-of-range request. Flagged in candidate `038`'s own "What's not done" section as a
plausible additional bug in this same file, not exercised that session for lack of a local sample
file with "Encapsulated Uncompressed" transfer syntax; also the top unchecked backlog entry in
`IDEAS.md`, noted while working candidate `087`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reading: read `decode_pixel_data_frame`'s `DicomValue::PixelSequence` branch against its own sibling `DicomValue::Primitive` branch in the same `match`, and against `is_encapsulated_pixeldata()`'s definition (`core/src/header.rs`, tag == `(7FE0,0010)` + undefined length, independent of the declared transfer syntax) and `can_decode_all()`'s definition (`encoding/src/transfer_syntax/mod.rs`) | Static code reading, no build needed | Confirmed the exact reachability path candidate `038` left unchecked does not require an exotic "Encapsulated Uncompressed" transfer-syntax stub at all (those stubs' `Codec::EncapsulatedPixelData(None, None)` already fails `can_decode_all()` and is rejected earlier in the function with `UnsupportedTransferSyntaxSnafu`) - it only requires an ordinary, codec-free transfer syntax (`Codec::None`, e.g. plain Explicit VR Little Endian) whose Pixel Data element happens to be encoded with undefined length + item fragments, which the parser classifies purely by tag+length, not by the declared transfer syntax's codec |
| Debug-assertions build + a temporary in-tree `#[test]` (`pixeldata/src/lib.rs`, added and reverted before this record was written): built a `FileDicomObject` via the crate's own public `FileMetaTableBuilder`/`InMemDicomObject`/`DataElement` API, transfer syntax Explicit VR Little Endian, Pixel Data set via `DataElement::new_with_len(.., Length::UNDEFINED, PixelFragmentSequence::new(vec![], vec![frag0, frag1]))` (2 fragments), `NumberOfFrames` = 2 (matching), then called `decode_pixel_data_frame(2)` | `cargo test -p dicom-pixeldata --lib`, debug build | Panics: `index out of bounds: the len is 2 but the index is 2` at `pixeldata/src/lib.rs:2511:21`. Reproduced 3/3 debug-build unit-test runs. Also confirmed frame `0`/`1` (in range) decode successfully both before and after the fix - the bug is specific to out-of-range indices |
| Standalone `repro/` against a fresh clone of pristine upstream `df2b9e42`: same object construction, but round-tripped through the real file writer/reader (`write_to_file` then `dicom_object::open_file`) instead of staying purely in-memory, then calling the real public `PixelDecoder::decode_pixel_data_frame(2)` | `just test` (`cargo run --release`), fresh clone, `RUST_BACKTRACE=1` for one run | Same panic, same line, this time with a full stack backtrace confirming the call path through the real `PixelDecoder` trait impl (see `output.txt`). Reproduced in both debug and release profiles |

## Status

**Confirmed** - deterministic, pure out-of-bounds-slice panic with no layout/seed/timing
dependency; reproduces 3/3 in-tree debug-build unit-test runs plus the standalone `repro/`'s
release-build run against a fresh clone of pristine upstream `df2b9e42` (identical panic message
and location every time).

**Fully reproducible**: **Yes**. `repro/` builds the DICOM object entirely through this crate's
own public API (`FileMetaTableBuilder`, `InMemDicomObject`, `DataElement`, `PixelFragmentSequence`)
and then round-trips it through the real file writer (`write_to_file`) and the real public
`dicom_object::open_file` entry point before calling the real public
`PixelDecoder::decode_pixel_data_frame` - this is a single external artifact (one malformed-but-
well-formed-at-the-byte-level DICOM file) handed to the library's real public entry point, with an
ordinary out-of-range `u32` argument a caller could easily pass by mistake (e.g. iterating
`0..=number_of_frames`); there is no second participant to fake here (see `CONFIRMATION.md`
section 11's exception for a single external artifact fed to a parser's real public entry point).

## Class

`crash` (an out-of-bounds slice index panic reachable via the documented public
`PixelDecoder::decode_pixel_data_frame` trait method on an ordinary object obtained from
`dicom_object::open_file` - no `unsafe`, no memory-safety implication beyond the panic itself).

## Root cause

The `DicomValue::PixelSequence` branch checks that the *count* of frames matches the *count* of
fragments (`number_of_frames as usize == fragments.len()`), which only guarantees the two counts
agree with each other - it says nothing about whether the specific `frame` index the caller passed
in is actually within that count. The very next branch in the same `match` (`DicomValue::Primitive`)
handles the equivalent situation correctly, using `.get(..).context(FrameOutOfRangeSnafu)` instead
of direct indexing - the two branches drifted out of sync on how they validate an out-of-range
`frame` argument, the same "structurally parallel implementations of the same operation, one
fixed, one not" shape already seen in candidates `023`/`024`/`038` in this codebase. Reachability
does not require an exotic transfer syntax: any codec-free transfer syntax (`Codec::None`) whose
Pixel Data happens to carry undefined length + item fragments reaches this exact branch, because
`is_encapsulated_pixeldata()` classifies elements by tag + length alone, not by the declared
transfer syntax.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed the `DicomValue::PixelSequence` branch to use `fragments.get(frame as usize).context(FrameOutOfRangeSnafu { frame_number: frame })?.to_vec()`
instead of the direct `fragments[frame as usize]` index, matching the sibling `Primitive` branch's
existing error-handling pattern exactly (same `FrameOutOfRangeSnafu` error type, already used
elsewhere in this same function).

Verified against both boundary inputs (`CONFIRMATION.md` section 7): frame `0`/`1` (valid, in a
2-fragment/2-frame object) still succeed identically before and after the fix; frame `2` (out of
range) now returns `Err(FrameOutOfRange { frame_number: 2, .. })` instead of panicking - confirmed
both via the temporary in-tree unit test and via `repro/`'s `just apply && just reproduce-after`
against the patched fresh clone (see `output.txt` for the real captured before/after output).

- Project test suite (`cargo test -p dicom-pixeldata --lib`): **39 passed, 1 failed (this
  candidate's own temporary repro test, reverted before writing this record), 2 ignored -> 40
  passed, 0 failed, 2 ignored** after the fix. No other test in the crate's existing suite was
  affected.

## What's not done / open questions

- Only the `FileDicomObject<InMemDicomObject<D>>` override (the crate's only real implementor of
  `PixelDecoder`) was checked here; candidate `038` already separately covers the *default* trait
  method's own unrelated unguarded-arithmetic issue in the same file.
- The sibling `DicomValue::Primitive` branch and the earlier `Codec::EncapsulatedPixelData(Some(decoder), _)`
  fast path (which delegates to per-codec `decode_frame` implementations) were read and already
  use graceful bounds checking - not independently re-verified with a fresh boundary input this
  session beyond what candidate `038` already did for the RLE codec case, since this candidate's
  scope was specifically the unguarded `PixelSequence` branch.
- Whether any other public entry point in this crate performs a similar count-matches-count check
  without separately validating the specific index derived from a caller argument was not
  systematically re-swept beyond this one spot.
