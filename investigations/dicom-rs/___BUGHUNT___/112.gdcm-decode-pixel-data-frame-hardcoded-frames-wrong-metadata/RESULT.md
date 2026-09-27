# gdcm-decode-pixel-data-frame-hardcoded-frames-wrong-metadata

Follow-up to candidates 107/111 (per-frame metadata array indexing in `pixeldata`). Noticed while
working candidate 111: `pixeldata/src/gdcm.rs`'s `decode_pixel_data_frame` builds the full,
un-truncated `rescale_intercept`/`rescale_slope`/`window_center`/`window_width`/
`voi_lut_sequence`/`voi_lut_function` arrays via the same `pixeldata/src/attribute.rs` accessors
`decode_pixel_data` uses (which can genuinely differ per real DICOM frame, e.g. via
`PerFrameFunctionalGroupsSequence`), but hardcodes `number_of_frames: 1` on the returned
`DecodedPixelData`. A caller can only address that single already-decoded frame as frame 0
(`to_dynamic_image(0)`/`to_vec_frame(0)`), which then indexes element 0 of those metadata arrays -
frame 0's values - regardless of which real frame was actually requested and decoded.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `pixeldata/src/gdcm.rs::decode_pixel_data_frame` vs `pixeldata/src/lib.rs`'s `build_monochrome_image`/`convert_pixel_slice` indexing (`if array.len() > 1 { array[frame] } else { array[0] }`) | Confirmed the array-vs-hardcoded-`number_of_frames`/frame-index mismatch by inspection |
| Manual boundary-value construction: a real, public-API-built `FileDicomObject` with 2 native uncompressed frames (pixel values 10 and 200) and a real `PerFrameFunctionalGroupsSequence`/`FrameVOILUTSequence` giving frame 0 and frame 1 genuinely different WindowCenter/WindowWidth (50/100 vs 200/100), no top-level WindowCenter/WindowWidth | Debug+release build, `gdcm`+`image` features, in-tree first then via a standalone `repro/` against a fresh clone | Reproduced deterministically: `decode_pixel_data_frame(1).to_dynamic_image(0)` returns pixel value 255 (frame 0's window applied to frame 1's pixel data) instead of the correct 128 (matching `decode_pixel_data().to_dynamic_image(1)`, the reference/full-decode path for the same real frame), 3/3 runs |

## Status

**Confirmed**, reproduces 3/3 runs (release build, in-tree) and 1/1 via the standalone `repro/`
against a fresh clone of pristine upstream `df2b9e42` through `just reproduce-before` (deterministic
logic bug, not layout/seed-dependent - every run gives the identical wrong pixel value).

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

`decode_pixel_data_frame` (the `gdcm`-feature `PixelDecoder` method for decoding a single frame) at
`pixeldata/src/gdcm.rs:185` decodes only the requested frame's pixel bytes, but calls the exact same
`pixeldata/src/attribute.rs` accessors (`rescale_intercept`, `rescale_slope`, `window_center`,
`window_width`, `voi_lut_function`, `voi_lut_sequence`) as the full-object `decode_pixel_data`, which
build arrays covering *every* frame in the object (one entry per `PerFrameFunctionalGroupsSequence`
item when present). These full arrays are stored into the returned `DecodedPixelData` unmodified,
while `number_of_frames` is hardcoded to `1`. Downstream consumers
(`DecodedPixelData::to_dynamic_image`/`to_vec_frame` in `pixeldata/src/lib.rs`, via
`build_monochrome_image`/`convert_pixel_slice`) index these arrays using the `frame` argument the
*caller* passes in, which for a single-frame `DecodedPixelData` is always `0` - not the real frame
number that was actually decoded. So `array[0]` (frame 0's metadata) is always applied, no matter
which real frame `decode_pixel_data_frame(N)` was asked to decode, whenever the object's per-frame
metadata genuinely varies across frames.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a small helper `select_frame_metadata` in `pixeldata/src/gdcm.rs` and applied it, right before
constructing the returned `DecodedPixelData`, to each of the four per-frame-capable arrays
(`rescale`, `window`, `voi_lut_function`, `voi_lut_sequence`): if an array has more than one entry
(genuinely per-frame), it's narrowed down to just the single entry for the frame actually decoded
(falling back to entry 0 if `frame` is somehow out of bounds, matching candidate 107's existing
"fall back to first value" convention), keeping that one value at index 0. Arrays with 0 or 1 entries
(shared across all frames) are left untouched. This makes the returned `DecodedPixelData`
self-consistent: its single frame's pixel data and its single frame's metadata now both genuinely
describe the same real frame, so indexing them at 0 is correct again.

Verified: re-ran the same repro against the patched tree (in-tree, 3/3 runs, and via `repro/`'s
`just reproduce-after` against the applied clone) - `decode_pixel_data_frame(1).to_dynamic_image(0)`
now returns 128, matching the reference. Also checked frame 0 is still handled correctly (window
50/100 keeps applying to frame 0's own pixel value 10, unaffected since it's a separate code path
from the mismatch under test).

- Project test suite: `cargo test -p dicom-pixeldata --features "gdcm image" --lib`: 81 passed/2
  failed(pre-existing, unrelated - `tests::test_1bit_image_decoding_data` and
  `tests::test_1bit_image_decoding_data_frame`, the still-unfixed-on-master bugs candidates 097/098
  already found and fixed-then-reverted per this skill's protocol)/3 ignored, identical before and
  after this fix (confirmed via `git stash`-and-rerun on the exact same tree). `cargo test
  --workspace --lib`: 351 passed/0 failed/2 ignored, identical before and after (this fix is
  `gdcm`-feature-gated and the default workspace test run doesn't enable it).

## What's not done / open questions

- The non-`gdcm` default implementation (`pixeldata/src/lib.rs`'s own `decode_pixel_data_frame`,
  distinct from this `gdcm.rs` one) was checked by reading. It already narrows `rescale`/`window`/
  `voi_lut_function` down to the single entry for the requested `frame`
  (`.get(frame as usize).or(inner.first())...map(|el| vec![el])`) before returning - the exact
  pattern this candidate's fix adds to `gdcm.rs`. But its `voi_lut_sequence` field is passed straight
  through from `ImagingProperties::from_obj` completely unnarrowed in both of its return branches -
  the same bug class may still apply there, specifically for VOI LUT Sequence. Not built into a repro
  this session (added to `IDEAS.md`'s backlog).
- Only `WindowCenter`/`WindowWidth` was used to build a visible mismatch in the repro; `rescale`,
  `voi_lut_function`, and `voi_lut_sequence` share the exact same hardcoded-`number_of_frames: 1`
  mechanism and are fixed by the same `select_frame_metadata` calls, but weren't independently
  exercised with their own differing-per-frame repro object this session.
