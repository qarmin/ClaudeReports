# decode-pixel-data-frame-voi-lut-sequence-not-narrowed

Follow-up to candidate 112 (noted in its "What's not done" section and added to `IDEAS.md`'s
backlog): the default (non-`gdcm`) `PixelDecoder::decode_pixel_data_frame` in
`pixeldata/src/lib.rs` already narrows `rescale`/`window`/`voi_lut_function` down to the single
entry for the requested `frame` before returning, but its `voi_lut_sequence` field is passed
straight through from `ImagingProperties::from_obj` completely unnarrowed, in both the
encapsulated-decoder return branch and the native/`Primitive` return branch.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `pixeldata/src/lib.rs`'s default `decode_pixel_data_frame` (non-`gdcm`) vs `DecodedPixelData::voi_lut_sequence()`'s VM-mismatch fallback (`Ok(Some(&inner[0..1]))` when `enforce_frame_fg_vm_match` is `false`, which `decode_pixel_data_frame` always sets) and `build_monochrome_image`'s `voi_lut_sequence[frame as usize]` indexing | Confirmed the un-narrowed-array-vs-hardcoded-`number_of_frames: 1` mismatch by inspection; also found that the length-mismatch fallback path in `voi_lut_sequence()` (not the `len == number_of_frames` indexing path candidate 112 targeted) is what actually gets hit here, since a 1-frame `DecodedPixelData` never satisfies `inner.len() == 2 == number_of_frames` |
| Manual boundary-value construction: a real, public-API-built `FileDicomObject` with 2 native uncompressed frames (pixel bytes `[10,10,10,10]` and `[20,20,20,20]`) and a real `PerFrameFunctionalGroupsSequence` / `FrameVOILUTSequence` / nested `VOILUTSequence` giving frame 0 and frame 1 genuinely different single-entry VOI LUTs (frame 0 maps every input to output 0, frame 1 maps every input to output 255), no top-level `VOILUTSequence`/`WindowCenter`/`WindowWidth`, decoded via the **default (non-`gdcm`)** `decode_pixel_data_frame` | Debug+release build, default (no `gdcm`) `image` feature only, in-tree first then via a standalone `repro/` against a fresh clone | Reproduced deterministically: `decode_pixel_data_frame(1).to_dynamic_image(0)` returns pixel value `0` (frame 0's VOI LUT applied to frame 1's pixel data) instead of the correct `255` (matching `decode_pixel_data().to_dynamic_image(1)`, the reference/full-decode path for the same real frame), 3/3 runs. First construction attempt (VOI LUT entries nested one level too shallow, directly under `FrameVOILUTSequence` instead of under its nested `VOILUTSequence`) silently produced `voi_lut_sequence() == None` for both paths and no mismatch - corrected to match the real DICOM macro nesting `get_from_per_frame` expects before the bug reproduced |

## Status

**Confirmed**, reproduces 3/3 runs (release build, in-tree) and 1/1 via the standalone `repro/`
against a fresh clone of pristine upstream `df2b9e42` through `just clone && just reproduce-before`
(deterministic logic bug, not layout/seed-dependent - every run gives the identical wrong pixel
value).

**Fully reproducible**: **Yes**. This is a single-participant finding (one caller decoding one real
DICOM object through the crate's own public `PixelDecoder`/`DecodedPixelData` API, the same way the
crate's own `test_decode_pixel_data_individual_frames` test calls it -
`obj.decode_pixel_data_frame(frame).unwrap()` then `.to_dynamic_image(0)`), not an interaction
between two independent participants - see `CONFIRMATION.md` section 11's "a malformed file handed
to a parser's real public entry point" carve-out; there is no second participant to fake here. The
repro drives the real `decode_pixel_data_frame`/`to_dynamic_image` public entry points end-to-end on
a real (albeit hand-built, as any DICOM test fixture is) `FileDicomObject`, exactly as a real caller
would.

## Class

logic

## Root cause

`decode_pixel_data_frame` (the default, non-`gdcm` `PixelDecoder` method for decoding a single
frame, `pixeldata/src/lib.rs`) destructures `voi_lut_sequence` from `ImagingProperties::from_obj`,
which builds one `VoiLut` entry per real frame when the object carries a per-frame VOI LUT (via
`PerFrameFunctionalGroupsSequence`/`FrameVOILUTSequence`/`VOILUTSequence`). Unlike `rescale`,
`window`, and `voi_lut_function` (each narrowed via `.get(frame as usize).or(inner.first())...`
just above, in the same function), `voi_lut_sequence` is passed straight through unmodified into
both return branches, while `number_of_frames` is hardcoded to `1`.

Downstream, `DecodedPixelData::voi_lut_sequence()` compares the stored array's length against
`self.number_of_frames`: since the array still has 2 entries but `number_of_frames` is `1`, the
`len == number_of_frames` branch (candidate 112's original target) is never taken; instead the
length-mismatch fallback fires. Because `decode_pixel_data_frame` always sets
`enforce_frame_fg_vm_match: false`, that fallback doesn't error - it silently returns
`Ok(Some(&inner[0..1]))`, i.e. element **0** of the un-narrowed array (frame 0's real VOI LUT),
regardless of which frame was actually decoded. `build_monochrome_image`/`convert_pixel_slice` then
apply that (wrong) LUT to the (correct) requested frame's pixel bytes.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added the same narrowing pattern already used for `rescale`/`window`/`voi_lut_function` in this
function: before constructing the returned `DecodedPixelData` (in both the encapsulated-decoder and
native/`Primitive` branches), `voi_lut_sequence` is narrowed down to a single-element `Vec`
containing just the entry for the frame actually decoded (`.get(frame as usize).or(inner.first())`,
falling back to entry 0 if `frame` is somehow out of bounds, matching candidate 107's/112's existing
convention), cloned since `VoiLut` isn't `Copy`. Arrays with 0 entries stay `None`/empty as before.

Verified: re-ran the same repro against the patched tree (in-tree, 3/3 runs, and via `repro/`'s
`just apply && just reproduce-after` against the applied clone) -
`decode_pixel_data_frame(1).to_dynamic_image(0)` now returns `255`, matching the reference, and
`single.voi_lut_sequence()` reports a single entry holding frame 1's own LUT (`data=[255]`) instead
of frame 0's (`data=[0]`).

- Project test suite: `cargo test -p dicom-pixeldata --features image --lib`: 58 passed/0 failed/2
  ignored, identical before and after this fix (confirmed via `git stash`-and-rerun on the exact
  same tree). `cargo test --workspace --lib`: 351 passed/0 failed/2 ignored, identical before and
  after.

## What's not done / open questions

- Only checked the default (non-`gdcm`) `decode_pixel_data_frame` in `pixeldata/src/lib.rs`, as
  scoped by this candidate; `gdcm.rs`'s equivalent function was already fixed by candidate 112 for
  all four metadata groups (including `voi_lut_sequence`).
- `DecodedPixelData::voi_lut_sequence()`'s silent length-mismatch fallback (returning frame 0's
  entry with just a `tracing::warn!`, no error, whenever `enforce_frame_fg_vm_match` is `false`) is
  the same general shape as candidate 107's "fall back to first value" convention; this fix keeps
  relying on it being safe once the array is pre-narrowed to length 1 by the caller (matching
  `number_of_frames`), rather than changing that accessor's own fallback behavior, which is shared
  by other callers and out of scope here.
