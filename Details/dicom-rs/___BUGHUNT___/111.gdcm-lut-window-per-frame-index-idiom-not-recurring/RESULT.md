# gdcm-lut-window-per-frame-index-idiom-not-recurring

Follow-up to candidate 107 ("pixeldata-per-frame-metadata-arrays-frame-index-oob"), from the
2026-08-25/27 `IDEAS.md` entry noted while working candidate 107: whether `pixeldata/src/gdcm.rs`'s
own `rescale_intercept`/`rescale_slope`/`voi_lut_sequence`/`voi_lut_function`/`window_center`/
`window_width` reads (via `pixeldata/src/attribute.rs`) share the same `if X.len() > 1 {
X[frame as usize] } else { X[0] }` unbounded-frame-index idiom just confirmed and fixed as
candidate 107 in `pixeldata/src/lib.rs`'s `build_monochrome_image`/`convert_pixel_slice`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Full read of `pixeldata/src/gdcm.rs`'s `decode_pixel_data`/`decode_pixel_data_frame` (both `PixelDecoder` methods active under the `gdcm` feature) | Static code reading, no build needed | Neither method indexes any of the four per-frame metadata arrays by `frame` at all. `decode_pixel_data` builds `rescale` via `zip(&rescale_intercept, &rescale_slope)` (a full-length, un-indexed pairing) and hands whole `Vec`s straight into the returned `DecodedPixelData` (`voi_lut_function`, `voi_lut_sequence`, `window`) - these are consumed later by GDCM's own C++ code via `decode_multi_frame_compressed`/`decode_single_frame_compressed` FFI calls for the actual pixel bytes, and the LUT/window arrays are never touched by GDCM at all, just passed through unindexed into the return value. `decode_pixel_data_frame` does the same - `rescale`/`window`/`voi_lut_sequence`/`voi_lut_function` are always full un-indexed arrays/`zip`s, never `X[frame]` |
| Full read of the six `pixeldata/src/attribute.rs` accessor functions gdcm.rs calls (`rescale_intercept`, `rescale_slope`, `voi_lut_function`, `voi_lut_sequence`, `window_center`, `window_width`) | Static code reading, no build needed | None of the six ever index by a `frame` argument (none of them even take one) - each one builds its whole result `Vec`/`Option<Vec<_>>` by iterating every item of `PerFrameFunctionalGroupsSequence`/`SharedFunctionalGroupsSequence` (via the shared `get_from_per_frame`/`get_from_shared` helpers) or falls back to a single top-level element, with no direct indexing operation anywhere in any of the six functions |
| Grep sweep of `pixeldata/src/gdcm.rs` for `\[frame`, `frame as usize\]`, `.len() >`, `.len() ==` to catch any indexing site not visible from the two read-throughs above | `grep -n "\[frame\|frame as usize\]\|\.len() >\|\.len() =="  pixeldata/src/gdcm.rs` | Exactly one direct-index-by-frame site in the whole file: `fragments[frame]` at line 252 (the raw `PixelSequence` fragment lookup inside `decode_pixel_data_frame`) - already `Confirmed` as a separate, already-known bug, candidate 060 (`pixeldata-gdcm-fragments-index-oob`), not a new instance and not re-reported here. The three `.len() ==`/`.len() >` sites found (`rescale_intercept.len() == rescale_slope.len()`, `wcs.len() == wws.len()` x2, `fragments.len() > 1`) are pairwise-consistency `ensure!()` guards feeding graceful `LengthMismatchRescaleSnafu`/`LengthMismatchWindowLevelSnafu` errors or a `zip()` (which silently truncates to the shorter side rather than panicking on a length mismatch) - none of them gate a later `X[frame]`-shaped index expression the way candidate 107's 18 spots did |

## Status

**Not reproducible.** The specific idiom this candidate was chasing (`if X.len() > 1 {
X[frame as usize] } else { X[0] }`, unbounded against the real pixel buffer's frame count) does
not exist anywhere in `pixeldata/src/gdcm.rs` or in the `pixeldata/src/attribute.rs` accessor
functions it calls. The `IDEAS.md` note's own hypothesis for *why* was correct: `gdcm.rs` hands
whole, un-indexed arrays through to its `DecodedPixelData` return value (for the LUT/window
metadata) and to GDCM's own C++ code via FFI (for the actual pixel bytes) rather than doing its
own per-frame indexing into these specific arrays in the same shape `lib.rs` does. This is now
independently verified by reading, not just assumed.

## Root cause

N/A (not reproducible) - the suspect pattern was checked and is genuinely absent. `gdcm.rs`'s only
"index an array by the caller-supplied `frame` argument without a bounds check" site is the
already-known-and-separately-tracked `fragments[frame]` at line 252 (candidate 060), which is
unrelated to the four per-frame metadata arrays (RescaleIntercept/RescaleSlope, VOI LUT Sequence,
VOI LUT Function, WindowCenter/WindowWidth) this candidate was specifically checking.

## What's not done / open questions

- While reading `decode_pixel_data_frame`, a different, structurally distinct issue was noticed
  (not investigated as part of this candidate, since it's a silent-wrong-value logic concern, not
  an instance of the panic-class idiom this candidate was chasing): the method builds `rescale`/
  `window`/`voi_lut_sequence`/`voi_lut_function` as the *entire* un-truncated per-frame array (via
  the same `attribute.rs` accessors used by `decode_pixel_data`), then returns a
  `DecodedPixelData` whose `number_of_frames` is hardcoded to `1`. Any downstream consumer calling
  `to_dynamic_image(0)`/`to_vec_frame(0)`/etc. on that single-frame result (the only valid frame
  index for a `number_of_frames: 1` object) will always pick index `0` of those arrays - i.e. the
  *first* frame's rescale/window/VOI-LUT metadata - regardless of which `frame` was actually
  requested from `decode_pixel_data_frame(frame)`. For a multi-frame image whose per-frame
  metadata genuinely varies frame-to-frame (a real, if less common, DICOM scenario -
  `PerFrameFunctionalGroupsSequence` exists precisely to allow this), requesting frame 5 of 10
  would silently render/convert using frame 0's window/rescale/VOI-LUT values instead of frame
  5's own. Logged as a new backlog entry in `IDEAS.md` for a future session (this is a candidate
  in its own right, a `logic`-class mismatch, not a crash - would need its own repro built from a
  DICOM object with genuinely differing `PerFrameFunctionalGroupsSequence` window/rescale values
  across frames and a value-correctness assertion, not just an out-of-bounds panic).
- This candidate only checked `pixeldata/src/gdcm.rs` and the specific `attribute.rs` accessors it
  calls for the exact panic-class idiom from candidate 107; it did not re-sweep the rest of
  `attribute.rs` (e.g. the overlay-related accessors) for the same idiom, since none of those are
  reached from `gdcm.rs`'s LUT/window handling and were already out of this candidate's stated
  scope.
