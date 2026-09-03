# gdcm-feature-verification-of-candidate-114-fix

Follow-up to candidate 114's own backlog note (and candidate 111's earlier finding that
`pixeldata/src/gdcm.rs`'s `decode_pixel_data`/`decode_pixel_data_frame` call the exact same
`rescale_intercept`/`rescale_slope`/`window_center`/`window_width`/`voi_lut_function`/
`voi_lut_sequence` free functions from `pixeldata/src/attribute.rs`, not a separate gdcm-specific
implementation): candidate 114's fix was never independently exercised against the `gdcm`-feature
decode path, since that feature requires building the vendored GDCM C++ library, which wasn't
attempted in that session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `pixeldata/src/gdcm.rs` (lines 21-73, 209-238) to confirm both `decode_pixel_data` and `decode_pixel_data_frame` call the identical, non-gdcm-specific `attribute.rs` free functions (`rescale_intercept(self)`, `window_center(&self)`, etc. - literally the same functions candidate 114 patched), and that these calls happen *before* the actual GDCM C++ decode call in the function body | - | Confirmed structurally: there is no separate/duplicate implementation for the gdcm feature to diverge from - candidate 114's fix (in `attribute.rs`, not feature-gated at all) necessarily applies identically regardless of the `gdcm` feature flag |
| Built and ran a real end-to-end repro (`repro/src/main.rs`, adapted from candidate 127's own repro with `dicom-pixeldata`'s `gdcm` Cargo feature enabled instead of the default) via the public `dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point (now dispatching to `gdcm.rs`'s implementation instead of `lib.rs`'s) on the same 3-frame object with a gap in `PerFrameFunctionalGroupsSequence` | release build, pristine `df2b9e42`, real vendored GDCM C++ library (built via `gdcm-rs`'s `cmake`-driven `build.rs`) actually decoding the constructed object's native uncompressed pixel data | `decoded.rescale()` = `[Rescale { slope: 1.0, intercept: 10.0 }]`, `decoded.window()` = `Some([WindowLevel { width: 50.0, center: 100.0 }])` - identical wrong output to candidate 127's non-gdcm repro, confirming the bug reproduces via the gdcm feature path too, 3/3 runs |
| Applied candidate 114's own existing `patch.diff` (verbatim, `114-fix.patch`) and re-ran the same repro | same repro binary | `decoded.rescale()` = `[Rescale { slope: 1.0, intercept: 0.0 }]`, `decoded.window()` = `None` - fixed, identical to candidate 127's non-gdcm result, 3/3 runs, and standalone-verified via a fresh-clone `just test` (including a genuine from-scratch GDCM C++ build, ~29s wall-clock on this machine thanks to parallel compilation) |
| `gdcm`-feature and full workspace test suites before/after | `cargo test -p dicom-pixeldata --features "gdcm image" --lib` and `cargo test --workspace --exclude dcms --lib` | gdcm-feature suite: 81 passed / 2 failed / 3 ignored, byte-for-byte identical before and after this patch - the 2 failures (`test_1bit_image_decoding_data`, `test_1bit_image_decoding_data_frame`) are the already-documented, unrelated pre-existing bugs from candidates 097/098 (independently confirmed present on pristine `df2b9e42` with the patch NOT applied too, via `git stash`). Full workspace suite: 351 passed / 0 failed / 2 ignored, identical before/after |

## Status

**Confirmed** - as an independent verification of already-Confirmed candidate 114's fix under the
`gdcm` Cargo feature specifically, not a distinct new bug (candidate 114's own root cause and fix
in `attribute.rs` already fully cover this, since `gdcm.rs` calls the identical functions).
Reproduces deterministically 3/3 runs both in-tree and via a standalone fresh-clone `just test`.

**Fully reproducible**: **Yes**. The repro drives the real public `PixelDecoder::decode_pixel_data()`
entry point (with the real vendored GDCM C++ library actually decoding the constructed object's
pixel data, not bypassed or mocked) on a genuinely constructed `InMemDicomObject`. Single external
artifact (the same real-world-plausible gapped `PerFrameFunctionalGroupsSequence` shape candidate
114 already established), no second participant to fake, so `CONFIRMATION.md` section 11 does not
apply.

## Class

logic

## Root cause

Identical to candidate 114's - see that candidate's `RESULT.md`. `pixeldata/src/gdcm.rs`'s
`decode_pixel_data`/`decode_pixel_data_frame` do not implement their own rescale/window/VOI-LUT
attribute reading; they call the exact same `attribute.rs` free functions the default (non-gdcm)
path uses, before ever invoking GDCM's own C++ decode routines. There is no gdcm-specific code
path for this particular bug class to hide in - the fix in the shared helper (`get_from_per_frame`)
transparently covers both.

## Introduced by

Same commit as candidate 114's own finding (see that candidate's `RESULT.md`/`SUMMARY.md` row) -
not independently re-derived here, since this candidate verifies the same code path under a
different Cargo feature, not a distinct defect with its own introduction point.

## The fix

No new fix authored - candidate 114's existing `patch.diff` (included verbatim here as
`114-fix.patch`) already covers this, confirmed empirically rather than only by code-reading
inference.

- Project test suite: `cargo test -p dicom-pixeldata --features "gdcm image" --lib`: 81 passed / 2
  failed / 3 ignored both before and after (the 2 failures are pre-existing and unrelated, see
  above). `cargo test --workspace --exclude dcms --lib`: 351 passed / 0 failed / 2 ignored both
  before and after. No regressions either way.

## What's not done / open questions

- This closes the last open item in the `get_from_per_frame`/candidates-107/112/113/114/126/127
  family of findings that `IDEAS.md` was tracking - no further open backlog rows remain for this
  specific bug class as of this session.
