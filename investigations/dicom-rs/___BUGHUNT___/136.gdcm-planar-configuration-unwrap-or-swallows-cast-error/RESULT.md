# gdcm-planar-configuration-unwrap-or-swallows-cast-error

`pixeldata/src/gdcm.rs`'s `decode_pixel_data_frame` reads the Planar Configuration attribute
inline with `self.element(tags::PLANAR_CONFIGURATION).ok().map(|el| el.uint16().unwrap_or(0)).unwrap_or(0)`
(effectively: any error, whether the element is absent or present-but-unreadable, becomes `0`),
instead of calling the shared `attribute::planar_configuration()` helper that every other
attribute read in the same function uses (`samples_per_pixel`, `bits_allocated`, `bits_stored`,
`high_bit`, `pixel_representation` are all read via that shared, validated module). The shared
helper existed all along but was gated `#[cfg(not(feature = "gdcm"))]`, so it was never even
compiled into a `gdcm`-feature build - `gdcm.rs` had no way to call it even if it wanted to.
Noticed independently while working candidates 134 and 135 as the one other place in
`pixeldata/src/gdcm.rs` with the same "swallow a `Result`-returning attribute read" shape.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `gdcm.rs`'s two `PixelDecoder` methods and compare against `attribute::planar_configuration()` | df2b9e42 | Confirmed `decode_pixel_data_frame` is the only one of the two methods that reads Planar Configuration at all (`decode_pixel_data` always hardcodes `PlanarConfiguration::Standard` in its output, since GDCM's own multi-fragment decode path already normalizes planar layout internally). The shared helper validates the value is exactly 0 or 1 and propagates a genuine conversion failure; the ad-hoc `gdcm.rs` code does neither - it never checks the value is 0/1 at all (any other `u16` silently behaves like Standard, since the only later use is `== 1`), and treats a present-but-unreadable element identically to an absent one |
| Standalone `repro/` via the crate's public `PixelDecoder::decode_pixel_data_frame()`, a real 1x1 RGB (SamplesPerPixel=3) object round-tripped through `FileDicomObject::write_to_file` + `dicom_object::open_file` | release build, `gdcm` + `image` features, PlanarConfiguration declared with VR=CS and a non-numeric text value ("YES") - a wire encoding no conformant writer would produce, but one this crate does not itself validate against the dictionary | Reproduced: `decode_pixel_data_frame(0)` returns `Ok(..)` on the pristine tree, silently treating the malformed element as PlanarConfiguration=Standard. `just test` runs the full before/apply/after cycle from a fresh clone of `df2b9e42` |
| Repeat-run check | `cargo run --release` re-run multiple times on the pristine and patched trees | Deterministic - no randomness/I/O timing involved in this code path |

## Status

**Confirmed** - deterministic, reproduces on every run, release build.

**Fully reproducible**: **Yes**. The repro builds a real DICOM file through the crate's public
object API, writes it to disk, reads it back through `dicom_object::open_file` (so the value under
test comes out of the real parser) and calls the real public
`dicom_pixeldata::PixelDecoder::decode_pixel_data_frame()` entry point. Single external artifact (a
DICOM file handed to a decoder), no second participant to fake - `CONFIRMATION.md` section 11 does
not apply.

## Class

logic

## Root cause

`attribute::planar_configuration()` (before this candidate) was gated `#[cfg(not(feature =
"gdcm"))]`, so `pixeldata/src/gdcm.rs` had no access to it and reimplemented the read ad-hoc:
`self.element(tags::PLANAR_CONFIGURATION).ok().map(|el| el.uint16().unwrap_or(0)).unwrap_or(0)`.
This collapses three distinct situations into the same "0" result: the element is genuinely
absent (the only case where defaulting to Standard is correct), the element is present but not
representable as a `u16` (a genuine conversion error the shared helper would report), and - unlike
the shared helper - a present element whose numeric value is neither 0 nor 1 (never validated at
all, silently treated as Standard since the only later use is `planar_configuration == 1`).

The only consumer of this value, `decode_pixel_data_frame`'s `interleave_planes()` call for a
3-samples-per-pixel image, exists specifically to undo PixelFirst (per-plane) encoding back into
Standard (per-pixel interleaved) layout. A file whose PlanarConfiguration is genuinely `1` but
encoded under a non-conformant wire VR silently skips that conversion instead of reporting the
malformed element, corrupting the returned pixel data's channel order with no diagnostic.

## Introduced by

fea825f7 "Improve the memory usage" (2023-10-12) - this commit is where the ad-hoc
`el.uint16().unwrap_or(0)` first appears in `gdcm.rs` (a `git log -S` on the exact snippet finds
no earlier occurrence). The shared `attribute::planar_configuration()` helper predates it, so this
was already a missed opportunity to reuse existing, correct code at the time.

## The fix

`patch.diff` (+4/-8 across two files):

- `pixeldata/src/attribute.rs`: dropped the `#[cfg(not(feature = "gdcm"))]` gate from
  `planar_configuration()` - it has no gdcm-specific dependencies, so there was never a technical
  reason it couldn't be compiled unconditionally.
- `pixeldata/src/gdcm.rs`: `decode_pixel_data_frame` now calls `planar_configuration(self)?`,
  exactly the pattern already used by every sibling attribute read in the same function, and
  compares the result against `PlanarConfiguration::PixelFirst` instead of the raw `u16` literal
  `1`. Removed the now-unused `dicom_dictionary_std::tags` import.

Verified against the same input class the bug was found with (a non-numeric wire value), and
checked that a *correctly*-encoded PlanarConfiguration (VR=US, value 1) still triggers the
interleave conversion as before (unchanged code path once the value is read successfully) -
`repro/`'s only scenario needs no variation beyond the one malformed case since the fix routes
through the same already-validated shared helper the non-gdcm path has used all along.

- `cargo test -p dicom-pixeldata --features "gdcm image" --lib`: 81 passed / 2 failed / 3 ignored,
  identical before and after. The 2 failures (`tests::test_1bit_image_decoding_data`,
  `tests::test_1bit_image_decoding_data_frame`) are the already-documented pre-existing failures
  from candidates 097/098, confirmed present on the unpatched tree in this session too.
- `cargo test --workspace --exclude dcms --lib`: 351 passed / 0 failed / 2 ignored, identical
  before and after.

## What's not done / open questions

- `decode_pixel_data` (the non-frame-indexed method) never reads PlanarConfiguration at all -
  always reports `PlanarConfiguration::Standard` in its output regardless of the actual file. This
  was not changed: unlike `decode_pixel_data_frame`, that method's own pixel bytes come from
  `decode_multi_frame_compressed`/`decode_single_frame_compressed` (GDCM's own C++ decode, which
  normalizes planar layout internally before returning), so reporting `Standard` there is already
  correct - not the same bug, not touched by this patch.
- Whether a genuinely out-of-spec PlanarConfiguration value (neither 0 nor 1) should be an error
  (as the shared helper now enforces for the `gdcm` build too) or silently tolerated is a
  standards-conformance question already settled by the pre-existing non-gdcm behavior; this patch
  only brings the `gdcm` build into agreement with it, not a new policy decision.
