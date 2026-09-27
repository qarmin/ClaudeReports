# overlay-attr-unwrap-or-before-validate

`pixeldata/src/overlay.rs` reads three attributes with `.unwrap_or(<DICOM spec default>)` and then,
on the very next line, validates the resulting value against the value the spec mandates:

- `decode_overlay_group`: `attribute::overlay_bits_allocated(obj, group).unwrap_or(1)` followed by
  `if bits_allocated != 1 { return Err(UnsupportedBitsAllocated) }`
- `decode_overlay_group`: `attribute::overlay_bit_position(obj, group).unwrap_or(0)` followed by
  `if bit_position != 0 { return Err(UnsupportedBitPosition) }`
- `extract_embedded_overlay_bits`: `attribute::samples_per_pixel(obj).unwrap_or(1)` followed by
  `if samples_per_pixel != 1 { return Err(EmbeddedSamplesPerPixel) }`

Flagged by a `HEURISTICS.md` pattern sweep ("error deliberately discarded right before the code
that was supposed to act on it"). This is a different defect from candidate `037`, which fixed an
arithmetic overflow further down in the same `extract_embedded_overlay_bits` and did not touch the
error swallowing.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `attribute::retrieve_required_u16` (the shared body of all three helpers) to work out which failures `.unwrap_or` is actually swallowing | Static reading of `pixeldata/src/attribute.rs:360-373` and `GetAttributeError` | The helper can fail three distinct ways: `MissingRequired` (element absent - the only case a spec default is appropriate for), `Retrieve` (an `AccessError` from `element_opt`), and `CastValue` (the element IS present but `Value::uint16()` rejected it - a strict variant match, so anything that did not parse into `PrimitiveValue::U16`, i.e. any other VR, fails). `.unwrap_or` collapses all three into "absent" |
| Temporary in-crate unit test: build an object with `(6000,0100)` present and encoded with VR `SS`, serialize it with `FileDicomObject::write_all` and read it back with `dicom_object::from_reader`, then call `PixelDecoder::decode_overlay` | debug build, `cargo test -p dicom-pixeldata --lib` | Element round-trips as `vr=SS value=Primitive(I16([16]))`; `attribute::overlay_bits_allocated` returns `Err(CastValue { requested: "uint16", got: I16 })`; `decode_overlay(0)` returns `Ok(Some(..))`. Test reverted afterwards |
| Same technique extended to all three sites, each paired with a baseline object carrying the same non-conformant value under the correct VR `US` | debug build, `cargo test -p dicom-pixeldata --lib` | All three diverge: correct VR -> `UnsupportedBitsAllocated` / `UnsupportedBitPosition` / `EmbeddedSamplesPerPixel`; wrong VR -> `Ok(Some(plane))` |
| Standalone `repro/` against a fresh clone of pristine upstream `df2b9e42`, driving the same three scenarios plus a control (attributes absent entirely) through `write_all` -> `from_reader` -> `PixelDecoder::decode_overlay` | dev profile, `cd repro && cargo run`, 3 consecutive runs | 3/3 runs, bit-for-bit identical output (md5 `6c4ae51359cbd2a185b8587038d407d1`), exit 1: "3 of 3 sites silently accepted a malformed element as the spec default". Control case still decodes normally |
| Sweep for the same `.unwrap_or(default)`-on-an-attribute-read pattern elsewhere in `dicom-pixeldata` | `grep` over `pixeldata/src/` | Only `pixeldata/src/gdcm.rs:225` (`el.uint16().unwrap_or(0)` for Planar Configuration, gdcm feature only). Not the same defect - no "must equal the default" validation follows it - but the same swallowing shape; recorded as a new suspect, not fixed here |

## Status

**Confirmed** - reproduces 3/3 runs of the standalone `repro/` against a fresh clone of pristine
upstream `df2b9e42` (dev profile), with byte-identical output every time, plus the earlier in-tree
unit-test runs. Not build-profile dependent: this is a plain control-flow/logic divergence with no
arithmetic or UB involved.

**Fully reproducible**: **Yes**. Every scenario is written out as a real Explicit VR Little Endian
DICOM stream by `FileDicomObject::write_all` and read back with the crate's own parser
(`dicom_object::from_reader`) before `PixelDecoder::decode_overlay` is called, so the object state
that triggers the bug is genuinely produced by the real DICOM reader from real bytes rather than
hand-assembled in memory. The affected entry point is the crate's own public API.

## Class

`logic` - a non-conformant, explicitly-rejected attribute value is silently accepted (and the
overlay plane is then decoded under an assumption the file itself contradicts), instead of the
conversion failure being surfaced.

## Root cause

`retrieve_required_u16` returns `Err` for three unrelated reasons, only one of which ("element
absent", `MissingRequired`) justifies substituting the DICOM standard's default value.
`.unwrap_or(default)` cannot tell them apart, so an element that is *present with a value that
could not be read* - most realistically a non-conformant file that encodes `(60xx,0100)`,
`(60xx,0102)` or `(0028,0002)` with the wrong VR, which this crate does not cross-check against the
data dictionary - is indistinguishable from an absent element.

That would be merely lossy if the value were then just used. What makes it a real divergence is
that the very next statement in each case is a conformance check *against that same default*:
`if bits_allocated != 1`, `if bit_position != 0`, `if samples_per_pixel != 1`. The substituted
default is by construction the one value that passes, so the check that exists specifically to
reject a non-conformant value is guaranteed to pass for exactly the inputs that are least
trustworthy. Concretely, in the repro: an object declaring Overlay Bits Allocated = 16 is rejected
with `UnsupportedBitsAllocated` when it is encoded conformantly, but is decoded as a 32-pixel
1-bit plane when the identical value is encoded with VR `SS`. The same applies to a declared
Overlay Bit Position of 3, and to an embedded overlay in a 3-samples-per-pixel image, where the
result is an overlay bitmap read out of the wrong sample stride.

## Introduced by

897c3d47 "[pixeldata] Add overlay plane extraction" (Stephan Vedder, 2026-08-05) - all three
`.unwrap_or` sites are present verbatim in the commit that first added `pixeldata/src/overlay.rs`;
the only later commit touching the file, a4bf81f6 "[pixeldata] add overlay error type", did not
change them.

## The fix

Added a small private helper in `pixeldata/src/overlay.rs` which keeps the lenient "absent means the
spec default" behaviour but stops it from covering up a genuine read failure:

```
fn u16_or_default<D>(
    obj: &FileDicomObject<InMemDicomObject<D>>,
    tag: Tag,
    read: std::result::Result<u16, attribute::GetAttributeError>,
    default: u16,
) -> Result<u16>
where
    D: DataDictionary + Clone,
{
    match read {
        Ok(value) => Ok(value),
        Err(e) => match obj.element_opt(tag) {
            Ok(Some(elem)) if !elem.is_empty() => Err(e.into()),
            Ok(_) => Ok(default),
            Err(_) => Err(e.into()),
        },
    }
}
```

and routed all three call sites through it. Deciding on the element rather than on the error
variant also keeps a *present but value-less* element (zero length) falling back to the default,
which matches what `attribute::number_of_frames_in_overlay` already does for the same situation -
so the only behaviour that changes is the one that was wrong.

Checked against more than the one triggering input (CONFIRMATION.md section 7): the repro covers
all three sites independently, each against a same-value/correct-VR baseline, plus a control where
the attributes are absent entirely; the added unit test additionally covers the zero-length-element
case. `repro/` output before the patch: `3 of 3 sites silently accepted a malformed element as the
spec default` (exit 1); after: `0 of 3 sites ...` with the control still decoding (exit 0). Full
before/after in `output.txt`; `just test` runs the whole cycle from a fresh clone.

- `cargo test -p dicom-pixeldata --features image --lib`: **58 passed, 0 failed, 2 ignored ->
  59 passed, 0 failed, 2 ignored** (the extra test is the regression test added by `patch.diff`).
- `cargo test --workspace`: **537 passed, 0 failed, 6 ignored -> 538 passed, 0 failed, 6 ignored**.
  No test changed from passing to failing.

## What's not done / open questions

- `pixeldata/src/gdcm.rs:225` does `el.uint16().unwrap_or(0)` for Planar Configuration under the
  `gdcm` feature, swallowing the same class of cast failure. It is *not* the same defect (no
  "must equal the default" validation follows, and both 0 and 1 are legal values), and the
  non-gdcm `attribute::planar_configuration` handles absent-vs-unreadable correctly, so it was
  left alone - but it is the one other place in this crate with the same shape. Recorded as a new
  suspect rather than bundled into this patch.
- The broader question of whether this crate should reject an element whose wire VR contradicts the
  data dictionary (rather than each reader coping individually) is a design decision for the
  maintainers; this patch deliberately does not change VR handling, only stops the error from being
  discarded. Candidate `037`'s RESULT.md raised the same underlying observation from a different
  angle.
- Only the wrong-VR route into `CastValue` was exercised. `GetAttributeError::Retrieve` (an
  `AccessError` out of `element_opt`) is now also propagated rather than defaulted; no input was
  constructed that makes `element_opt` fail on an `InMemDicomObject`, so that branch is reasoned
  about but not exercised by the repro.
- No fuzzing was pointed at the overlay decoder in this session; a `fuzz-bug-repro` campaign over
  `decode_overlays` would be the natural way to look for further attribute-validation gaps in the
  same file.
