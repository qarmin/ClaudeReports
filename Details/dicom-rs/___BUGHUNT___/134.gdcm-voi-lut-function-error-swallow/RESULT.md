# gdcm-voi-lut-function-error-swallow

`pixeldata/src/gdcm.rs`'s two `PixelDecoder` methods read the VOI LUT Function attribute with
`voi_lut_function(self).unwrap_or(None)` (lines 68 and 233 at df2b9e42), discarding any
`Result::Err` from `pixeldata/src/attribute.rs`'s `voi_lut_function()` helper and treating it
exactly like "attribute not specified". The non-`gdcm` sibling implementation of the same logical
step (`pixeldata/src/lib.rs:2213`, `ImagingProperties::from_obj`) propagates the same error with
`?`. Suspect found by the "parallel implementations of the same operation drifting out of sync"
heuristic, which the project risk map flags as unusually high-yield for exactly this `gdcm.rs` vs
`lib.rs` pair (candidates 023/024/026/027, 038, 097/098/107/112-114).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read both `gdcm.rs` call sites and `lib.rs:2213`'s `ImagingProperties::from_obj` | df2b9e42 | Confirmed structurally equivalent: both compute the same `Option<Vec<VoiLutFunction>>` from the same `attribute::voi_lut_function()` helper, feed it into the same `DecodedPixelData.voi_lut_function` field, and it is consumed by the same `DecodedPixelData::voi_lut_function()` accessor, whose `None` case means "use `VoiLutFunction::Linear`" in `build_monochrome_image`/`to_dynamic_image`. Only the error handling differs |
| Read `attribute::voi_lut_function()` and `PrimitiveValue::string()` to enumerate which real inputs make the helper return `Err` | - | Two distinct input classes: (a) the element is present with a **zero-length value** (`PrimitiveValue::Empty` -> `CastValueError{requested:"Str", got:Empty}`), which is a perfectly legal way of leaving this Type 3 attribute unspecified; (b) the element is present with a **non-textual value**, e.g. a file declaring VR `US` on the wire for (0028,1056) (`got: U16`), which is genuinely malformed |
| Scratch probe binary (`tmp/probe`, deleted afterwards) over 4 inputs (absent / zero-length / VR=US numeric / valid `SIGMOID`), each written to a real `.dcm` file and re-read via `dicom_object::open_file`, then `decode_pixel_data()`, built twice (with and without the `gdcm` feature) | release build, pristine df2b9e42, real vendored GDCM C++ library | Divergence confirmed in **both** directions: zero-length -> non-gdcm `Err`, gdcm `Ok`/`None`; VR=US numeric -> non-gdcm `Err`, gdcm `Ok`/`None` (error swallowed). Absent and valid inputs behave identically on both paths |
| Standalone `repro/` driving the public `PixelDecoder::decode_pixel_data()` entry point on two real, parser-round-tripped DICOM files, run once per Cargo feature configuration | release build, real GDCM C++ decode path when the feature is on | Reproduces deterministically; `just test` from a genuine fresh `git clone` runs the whole before/apply/after cycle (see `output.txt`) |
| Repeatability re-runs of the pristine (unpatched) clone | release build | 6/6 (3 runs x 2 decode paths), identical output every time |
| `git log -S` on both call sites | - | The swallowing was introduced wholesale by ee7fe612 (PR #514), see "Introduced by" |
| Test suites before/after the fix | `cargo test -p dicom-pixeldata --features "gdcm image" --no-fail-fast` and `cargo test --workspace --lib --no-fail-fast` | Byte-identical before and after, see "The fix" |

## Status

**Confirmed** - deterministic, 6/6 repeat runs on the pristine clone (3 runs x 2 Cargo feature
configurations), release build, plus one full `just test` cycle from a fresh clone.

**Fully reproducible**: **Yes**. The repro builds two real DICOM files through the crate's public
object API, writes them to disk, reads them back through `dicom_object::open_file` (so the values
under test come out of the real parser) and calls the real public
`dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point, with the real vendored GDCM C++
library actually decoding the pixel data in the `gdcm` configuration. The input is a single
external artifact (a DICOM file handed to a decoder), with no second participant to fake, so
`CONFIRMATION.md` section 11 does not apply.

## Class

logic

## Root cause

`attribute::voi_lut_function()` conflates two different situations into the same
`GetAttributeError::CastValue`: an element that is present but *empty* (legal "unspecified"), and
an element whose value genuinely is not text (malformed). Neither of the crate's two `PixelDecoder`
implementations handles that conflation correctly, and they fail in opposite directions:

- `pixeldata/src/gdcm.rs` (`unwrap_or(None)`, both `decode_pixel_data` and
  `decode_pixel_data_frame`): every error becomes "attribute not specified". A file whose
  VOILUTFunction element carries a non-string value decodes successfully and is then rendered with
  the implicit `VoiLutFunction::Linear` fallback (`lib.rs`'s `build_monochrome_image` /
  `to_dynamic_image` `None` arm) - a silently wrong windowing function, with no diagnostic.
  The commit that introduced this (ee7fe612, "If VoiLut is not present on the dicom file, default
  to None") was aiming at the *empty-value* case; `unwrap_or(None)` also catches the malformed one.
- `pixeldata/src/lib.rs:2213` (`?`): every error, including the legal empty-value one, aborts the
  whole decode. A file with a zero-length (0028,1056) - valid DICOM for a Type 3 attribute - cannot
  be decoded at all on the default (non-`gdcm`) build.

The net observable effect is that the crate's public decoding behaviour for one and the same file
depends on which Cargo feature the consumer happened to enable.

## Introduced by

ee7fe612 "If VoiLut is not present on the dicom file, default to None" (2024-05-01, merged via
d5327438 / PR #514) replaced `voi_lut_function(self).context(GetAttributeSnafu)?` with
`voi_lut_function(self).unwrap_or(None)` at both `gdcm.rs` call sites and nowhere else, which is
where the two implementations diverged.

The other half of the finding - `voi_lut_function()` returning `Err` for a legal zero-length
element in the first place - predates that and goes back to the helper's own introduction,
5c685278 "ENH: Tests and integration with DecodedPixelData WIP" (2023-11-17), which is also where
`lib.rs`'s `?` has been ever since.

## The fix

`patch.diff` (+7/-2 across two files):

- `pixeldata/src/attribute.rs`: `voi_lut_function()` returns `Ok(None)` when any retrieved element
  has an empty value (`multiplicity() == 0`), instead of reporting a cast failure. This makes
  "present but empty" mean "unspecified" - which is what ee7fe612 was actually trying to achieve -
  while a genuine type mismatch still returns `Err`. Returning `None` for the whole attribute
  (rather than a shorter array) when *any* element is empty matches the convention candidate 114
  already established for `get_from_per_frame`: a per-frame array that cannot be filled for every
  frame is dropped entirely rather than silently misaligned.
- `pixeldata/src/gdcm.rs`: both call sites use `voi_lut_function(self)?`, exactly the form
  `lib.rs:2213` uses, so a real conversion error is reported instead of silently degraded.

Verified against both input classes and on both decode paths, not just the one that triggered the
suspicion: after the patch, the zero-length case decodes successfully with no VOI LUT function on
*both* paths, and the VR=US case errors out on *both* paths (`output.txt`, "AFTER" sections). The
absent-attribute and valid-`SIGMOID` cases were checked too and are unchanged by the patch.

- Project test suite:
  - `cargo test -p dicom-pixeldata --features "gdcm image" --no-fail-fast`: 94 passed / 2 failed /
    3 ignored, identical before and after. The 2 failures (`tests::test_1bit_image_decoding_data`,
    `tests::test_1bit_image_decoding_data_frame`) are the already-documented pre-existing failures
    from candidates 097/098, confirmed present on the unpatched tree in this session too.
  - `cargo test --workspace --lib --no-fail-fast`: 351 passed / 0 failed / 2 ignored, identical
    before and after.

## What's not done / open questions

- Whether erroring out on a malformed (non-textual) VOILUTFunction is the behaviour the maintainers
  want at all is a product decision: the alternative is to keep decoding but log a warning and fall
  back to `Linear`. This patch chose consistency with the existing, older `lib.rs` behaviour
  (report the error) rather than inventing a third behaviour; a maintainer who prefers leniency
  should apply it to *both* paths, not just the `gdcm` one.
- Other `attribute.rs` accessors that use `.string()` (`photometric_interpretation`) would also
  return `Err` for a zero-length element, but that one is a Type 1 (always required) attribute, so
  an error there is defensible; it was not changed. The `to_float64`/`to_int`-based accessors
  (`rescale_*`, `window_*`) do not return `Result` at all and silently drop unconvertible values -
  a different, already partly-documented shape (candidates 107/112/113/114), not re-examined here.
- `gdcm.rs` was checked for other `unwrap_or`/`.ok()` uses of a `Result`-returning `attribute.rs`
  helper while working on this: the remaining one (`el.uint16().unwrap_or(0)` for Planar
  Configuration in `decode_pixel_data_frame`) reads an element directly rather than through a
  helper that the `lib.rs` path also uses, so it is not the same divergence; it was not
  investigated further.
