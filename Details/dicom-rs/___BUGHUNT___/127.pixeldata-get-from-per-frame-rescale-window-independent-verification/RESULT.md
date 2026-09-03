# pixeldata-get-from-per-frame-rescale-window-independent-verification

Follow-up to candidate 114's own `IDEAS.md` note: `pixeldata/src/attribute.rs`'s
`get_from_per_frame` helper is shared by 6 accessors (`voi_lut_sequence`/`voi_lut_function`/
`rescale_intercept`/`rescale_slope`/`window_center`/`window_width`), but candidate 114 only built
a dedicated repro for `voi_lut_sequence` - the other four (rescale/window) got the exact same fix
but were never independently exercised with their own repro. This candidate closes that gap.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `pixeldata/src/lib.rs`'s public `DecodedPixelData::rescale()`/`.window()` accessors (lines 665/727) to confirm they share the exact same "count check without a corresponding index bound, then silent fallback to element 0 on mismatch" pattern `voi_lut_sequence()` uses | - | Confirmed: both accessors have their own `enforce_frame_fg_vm_match`-gated length-mismatch fallback, structurally identical to `voi_lut_sequence()`'s |
| Built a real end-to-end repro (`repro/src/main.rs`) via the public `dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point on a genuinely constructed 3-frame `InMemDicomObject` with a real `PerFrameFunctionalGroupsSequence` (frame 0 and frame 2 carry real, distinct `RescaleIntercept`/`RescaleSlope` via `PixelValueTransformationSequence` and `WindowCenter`/`WindowWidth` via `FrameVOILUTSequence`; frame 1's item has neither group at all - the gap), then called the public `.rescale()`/`.window()` accessors | release build, pristine `df2b9e42` | `decoded.rescale()` = `[Rescale { slope: 1.0, intercept: 10.0 }]` (frame 0's real value, silently applied to all 3 frames); `decoded.window()` = `Some([WindowLevel { width: 50.0, center: 100.0 }])` (same silent misapplication) - both wrong, both deterministic, 3/3 runs |
| Applied candidate 114's own existing `patch.diff` (verbatim, `114-fix.patch` in this folder) and re-ran the same repro | same repro binary, no changes | `decoded.rescale()` = `[Rescale { slope: 1.0, intercept: 0.0 }]` (falls through to the documented safe default, since no per-frame OR shared source remains valid after the fix makes `get_from_per_frame` return `None` outright for a gapped sequence); `decoded.window()` = `None` (correctly reports "no valid window level" instead of fabricating one) - fixed, 3/3 runs, and standalone-verified via a fresh-clone `just test` |
| Full workspace test suite before/after | `cargo test --workspace --exclude dcms --lib` | 351 passed / 0 failed / 2 ignored, identical before and after (114's patch was already verified clean by candidate 114 itself; re-confirmed here) |

## Status

**Confirmed** - as an independent verification of already-Confirmed candidate 114's fix, not a
distinct new bug (candidate 114's own root cause and fix already fully cover this; see its
`RESULT.md`). Reproduces deterministically 3/3 runs both in-tree and via a standalone fresh-clone
`just test`.

**Fully reproducible**: **Yes**. The repro drives the real public `PixelDecoder::decode_pixel_data()`
entry point on a genuinely constructed `InMemDicomObject` (built entirely through the crate's own
public `put_element`/`DataElement::new` API), then calls the real public `.rescale()`/`.window()`
accessors - not a hand-implemented stand-in. Single external artifact (a real, though internally
gapped, `PerFrameFunctionalGroupsSequence` - the same real-world-plausible shape candidate 114
already established), no second participant to fake, so `CONFIRMATION.md` section 11 does not
apply.

## Class

logic

## Root cause

Identical to candidate 114's: `get_from_per_frame` (`pixeldata/src/attribute.rs`) used
`.iter().filter_map(...)` over `PerFrameFunctionalGroupsSequence`'s items, silently *skipping* any
item missing the requested attribute instead of preserving its position - producing a shorter,
index-misaligned `Vec` whenever any per-frame item in the middle of the sequence lacks the
attribute. `DecodedPixelData::rescale()`/`.window()` (and `.voi_lut_function()`, not independently
re-verified here but structurally identical) both have their own length-mismatch fallback
(triggered whenever the per-frame array's length doesn't match `NumberOfFrames`) that - since
`enforce_frame_fg_vm_match` is always `false` for the default `decode_pixel_data()` path - silently
returns element 0 for use on *every* frame, rather than erroring or falling back to the documented
safe default. This makes a real, present value from frame 0 look like universally-valid data,
which is more misleading than the honest "no valid data" default these accessors otherwise use
when genuinely nothing is present.

## Introduced by

Same commit as candidate 114's own finding (see that candidate's `RESULT.md`/`SUMMARY.md` row) -
not independently re-derived here, since this candidate verifies the same code path, not a
distinct defect with its own introduction point.

## The fix

No new fix authored - candidate 114's existing `patch.diff` (included verbatim here as
`114-fix.patch`) already covers this. Re-verified against a scenario 114 itself didn't build a
repro for (`rescale_intercept`/`rescale_slope`/`window_center`/`window_width` specifically, not
`voi_lut_sequence`), confirming the shared-helper fix transparently covers all 6 accessors that
call it, not just the one 114 happened to test.

- Project test suite: `cargo test --workspace --exclude dcms --lib`: 351 passed / 0 failed / 2
  ignored both before and after (identical), no regressions.

## What's not done / open questions

- `voi_lut_function` (the 6th accessor sharing `get_from_per_frame`) was not independently
  exercised with its own dedicated repro by either candidate 114 or this one - structurally
  identical to the other 5, considered low-risk, but not empirically re-verified.
- The still-open sibling item from candidate 114's own backlog (independently verifying this fix
  against the `gdcm`-feature decode path, `pixeldata/src/gdcm.rs`) is a **separate** open backlog
  row, not addressed by this candidate - the `gdcm` feature requires the vendored GDCM C++ library,
  which this candidate did not build.
