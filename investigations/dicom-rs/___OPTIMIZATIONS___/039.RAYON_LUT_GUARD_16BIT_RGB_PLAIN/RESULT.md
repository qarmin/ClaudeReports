# Candidate 006's rayon guard, measured for the 16-bit-plain non-monochrome RGB site (call site #6)

Candidate 006's rayon threshold+guard covers 7 dispatch sites in `pixeldata/src/lib.rs`; one
(`convert_pixel_slice`'s 16-bit plain-numeric-conversion path for non-monochrome images, used for
16-bit `PhotometricInterpretation::Rgb`/`YbrFull` pixel data with no Modality LUT applied) was
confirmed bit-exact but never independently timed - the only real `dicom-test-files` fixture with
this shape, `pydicom/SC_rgb_16bit.dcm`, is only 100x100x3=30,000 pixels, too small to be sure of a
timing read either way. `IDEAS.md` flagged this as the lowest-priority open measurement gap in
candidate 006's own site coverage.

## What was tried

Reused the existing `gen-ybr` mode (tiles a `PlanarConfiguration::Standard` source's pixel data
vertically) directly against `pydicom/SC_rgb_16bit.dcm` as source - no new harness mode needed,
since `gen-ybr` already produces exactly the right shape (it only assumes `SamplesPerPixel=3`,
`PlanarConfiguration::Standard`, doesn't care about `PhotometricInterpretation` or
`BitsAllocated`). Important prerequisite check: `convert_pixel_slice` explicitly rejects
`PlanarConfiguration != Standard` when `samples_per_pixel > 1` (`pixeldata/src/lib.rs:1516-1523`,
"TODO #129") - so candidate 034's own 16-bit `PlanarConfiguration::PixelFirst` fixtures
(`planar16_*.dcm`) are NOT usable here; `tovec` needs `PlanarConfiguration::Standard` specifically,
which `SC_rgb_16bit.dcm` already is. Built `rgb16_below.dcm` (100x100=10,000px, below the
16,384px threshold) and `rgb16_above.dcm` (10,000x100=1,000,000px, above threshold).

Two binaries (baseline, patched with candidate 006's `patch.diff`) compared via `tools/paired.sh`
(`RUNS=9`) on `tovec` (`decode_pixel_data().to_vec_frame::<f32>(0)`, isolating
`convert_pixel_slice` directly).

| Variant | below threshold (10,000px), round 1 | below threshold, round 2 (re-check) | below threshold, round 3 (fresh `just test` clone) | above threshold (1,000,000px), round 1 | above threshold, round 2 (fresh clone) |
|---|---|---|---|---|---|
| baseline | 0.405 ms p50 | 0.409 ms p50 | 0.401 ms p50 | 4.186 ms p50 | 3.867 ms p50 |
| patched | 0.427 ms p50 | 0.382 ms p50 | 0.417 ms p50 | 4.079 ms p50 | 3.981 ms p50 |

## Result

**No consistent change at either scale** - a clean, unambiguous null result for this specific
site, closing the gap `IDEAS.md` flagged:

- Below threshold, round 1: +5.4%, 4/9 - not consistent.
- Below threshold, round 2 (re-check): -6.5%, 6/9 - not consistent, and the direction even
  flipped between rounds - clear noise, not a real effect either way.
- Below threshold, round 3 (fresh-clone `just test`): +3.8%, 4/9 - not consistent, a third
  independent confirmation.
- Above threshold, round 1: -2.6%, 4/9 - not consistent, as expected (same parallel code path
  either way).
- Above threshold, round 2 (fresh-clone `just test`): +2.9%, 4/9 - not consistent, reproducing
  round 1's null result independently.

This is a materially cleaner result than several of candidate 006's other sites (the 16-bit LUT
site showed a real, reproducible below-threshold regression; the monochrome sites showed a
narrower, harder-to-pin-down regression zone) - this particular site shows no measurable effect
in either direction at either scale on this box.

## Does it change behavior/output?

- **Bit-exact?** Yes - `sha256sum` of `dump-image` output identical
  (`3408ffd908039a3e31d9f00c6fc778e879d9d9727867dea5d8f3d2d71e8a2c17`) baseline vs. patched on
  `rgb16_above.dcm`.
- **Is this a breaking change?** No - same private `pixeldata` crate change as candidate 006.
- Project test suite: `cargo test --release -p dicom-pixeldata`: **39 + 1 + 1 + 10 = 51 passed /
  0 failed** (patch applied), matching candidate 006's own recorded count exactly. Reverted-tree
  (baseline) count is identical.

## Why the result looks the way it does

Unlike the 16-bit LUT site (which showed a real below-threshold regression candidate 006 itself
couldn't fully explain), this site's guarded `if` sits in front of a numerically simpler
conversion (a plain cast/rescale, no LUT table construction or lookup) - plausibly small enough
per-element work, and few enough total instructions either side of the branch, that the guard
check's own overhead genuinely doesn't register above this box's noise floor at either the
below-threshold (guard overhead only) or above-threshold (same parallel path both builds) case.
Not confirmed via disassembly - offered as the most likely explanation given the site's relative
simplicity, not a traced fact.

## What's not done / open questions

- Only the **unsigned** 16-bit plain site was tested (`SC_rgb_16bit.dcm`'s pixel representation is
  unsigned) - the **signed** 16-bit plain sibling site remains completely unmeasured, no signed
  16-bit RGB/YbrFull fixture was available or built. Lower priority per the original `IDEAS.md`
  entry, but still an open gap.
- This closes the specific `IDEAS.md` item asking for this measurement (unsigned site, timing) -
  no further action needed for that exact question.
