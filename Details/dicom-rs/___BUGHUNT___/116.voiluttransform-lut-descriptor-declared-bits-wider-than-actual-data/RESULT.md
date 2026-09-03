# voiluttransform-lut-descriptor-declared-bits-wider-than-actual-data

Found via this session's GitHub PR raw-sync: upstream `Enet4/dicom-rs#825` (open, unmerged as of
`df2b9e42`) reports that Agfa CR, Horos, and Cornerstone all emit a VOI LUT whose _LUT Descriptor_
third value declares more bits per entry than the _LUT Data_ entries actually use, and describes
the resulting rendered image as "almost uniformly flat". This directly answers an open question
this session's own candidate 115 (`WindowLevelTransform`, the sibling window-level transform in
the same file) raised in its own "what's not done" section: whether `VoiLutTransform` (the
explicit-LUT-table transform) has a comparable degenerate case - it does, via a different
mechanism (a wrong bit-depth assumption, not a degenerate denominator).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading of `VoiLutTransform::new`/`apply` (`pixeldata/src/transform.rs`) | n/a | Found the mechanism: `shift = next_pow_bits(bits_stored) - lut.bits_stored` (`lut.bits_stored` being the *declared* LUT Descriptor value, trusted verbatim), then `apply()` does `entry << shift` - if the declared depth is wider than what the data actually needs, `shift` comes out too small (down to `0`), so nowhere near the intended bit range is used |
| Real end-to-end repro via the crate's own public `PixelDecoder::decode_pixel_data()` + `to_dynamic_image_with_options()` API, on a constructed 16-bit-allocated/12-bit-stored monochrome object with a VOI LUT Sequence declaring `bits_stored=16` (LUT Descriptor) but whose LUT Data only ever holds values up to `4095` (needs 12 bits) - the exact declared-vs-actual mismatch PR #825 describes | release build, deterministic (pure function, no I/O/randomness) | Reproduced: a pixel selecting the LUT's `4095`-valued entry rendered as raw `4095` (about 6% of the full 16-bit output range) instead of the correctly-scaled `65520` (`4095 << 4`, matching the entry's own actual 12-bit span) |

No further tools were needed - like candidate 115, this is a pure, deterministic arithmetic
function with no adversarial-input/parsing surface (the mismatch is a *legitimate*, real-world
device quirk, not malicious input), so code reading plus one direct repro was sufficient.

## Status

**Confirmed** - deterministic, pure-function bug: reproduces on every run (no repeat-count
applies the way it would for a fuzz-found crash - there is no randomness or I/O anywhere in this
code path).

**Fully reproducible**: **Yes**. Not a "one participant reacts to another's wire input" finding
(`CONFIRMATION.md` section 11 doesn't apply) - it's a self-contained arithmetic defect in a public
library function, reproduced end-to-end through the crate's real, fully public entry point
(`decode_pixel_data()` + `to_dynamic_image_with_options()`, the same API a real DICOM viewer would
call) on a DICOM object whose VOI LUT Sequence shape matches the exact real-world device quirk
described in PR #825's own bug report, not synthesized to merely look plausible.

## Class

logic

## Root cause

`VoiLutTransform::new` computes how far to left-shift each raw LUT entry so that it fills the
*target* image's own bit depth (`next_pow_bits(bits_stored).next_power_of_two()`), by subtracting
the LUT's own declared bit depth (`lut.bits_stored`, taken straight from the _LUT Descriptor_'s
third value, PS3.3 C.11.2.1.5) - `shift = next_pow_bits - lut.bits_stored`. This assumes the LUT
Descriptor's declared depth always matches how many bits the _LUT Data_ entries actually occupy.
Real devices don't always honor that assumption: PR #825 documents Agfa CR (and observed
compatible behavior in Horos/Cornerstone) declaring 16 bits per entry while the entries themselves
only ever span 12 bits. Trusting the declared `16` computes `shift = 16 - 16 = 0` instead of the
correct `shift = 16 - 12 = 4`, so `apply()`'s `entry << shift` leaves the entry completely
unscaled - a LUT ramp meant to span the full output range instead only ever produces the bottom
~1/16th of it, rendering the whole image "almost uniformly flat" with no error, warning, or panic.

## Introduced by

9ff19ee3 "Implement LUT" (the original implementation trusting `lut.bits_stored` verbatim, same
originating commit as candidate 115's sibling bug - `VoiLutTransform` and `WindowLevelTransform`
were both added together).

## The fix

Added an `effective_bits_stored(lut)` helper that computes the number of bits actually needed to
represent the LUT's own maximum entry value (`u16::BITS - max.leading_zeros()`), and uses that
instead of the raw declared `lut.bits_stored` wherever the shift is computed - but only when it
would *reduce* the assumed depth (`used > declared` still falls back to `declared`, so a
legitimately-wider LUT is never assumed to need *more* shift than it declared, and an all-zero LUT,
which says nothing about real depth, also falls back to `declared`). This changes nothing for a
correctly-declared LUT (where `effective_bits_stored` always agrees with the declared value) and
fixes the mismatched case by deriving the shift from the data's own real range instead of blindly
trusting the header field describing it.

Verified against the repro: before the fix, the pixel value was `4095` (raw LUT entry, unscaled);
after the fix, `65520` (`4095 << 4`), matching the LUT ramp's intended full-range output - see
`output.txt` for both full runs.

- Project test suite: `cargo test -p dicom-pixeldata` - 39 passed, 0 failed, 2 ignored before AND
  after (identical counts, zero regressions). Full workspace `cargo test --workspace --exclude
  dcms` (dcms excluded: untracked scratch crate belonging to a different session, not part of this
  workspace's tracked tree) - 0 failures anywhere before or after.

## What's not done / open questions

- Upstream PR `Enet4/dicom-rs#825` independently arrived at the same fix (confirmed by reading its
  diff during discovery, including the same `effective_bits_stored` name and the same
  used-caps-at-declared / zero-falls-back-to-declared corner cases) - this candidate's fix was
  derived and verified independently by reading the code and the standard's own shift/scale logic,
  not copied, but the resulting code is substantively the same shape. If that PR is merged upstream
  first, this candidate becomes a duplicate-confirms-independently case, not a new finding.
- Not checked: whether `Lut::new_rescale_and_lut`'s own 8-bit rendering path (the `bits_allocated
  == 8` branch in `pixeldata/src/lib.rs`, distinct from the 16-bit branch this repro exercises) has
  the same reachability for a mismatched-depth VOI LUT - the fix lives in the shared
  `VoiLutTransform::new` constructor both paths call, so it should already be covered, but this
  wasn't independently exercised with an 8-bit-allocated repro this session.
- Not checked: the `gdcm`-feature decode path (`pixeldata/src/gdcm.rs`), which per candidate 111
  never constructs a `VoiLutTransform` of its own for its LUT/window handling (it hands the full,
  un-indexed arrays straight through) - this bug class is therefore expected not to apply there,
  but this wasn't independently re-verified this session.
