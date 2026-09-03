# AdaptiveVRLittleEndianDecoder's Xs-override fix, exercised through its real entry point

Follow-up to candidate 095: `DataSetReaderOptions::flexible_decoding` (`parser/src/dataset/read.rs`)
is the actual, public entry point that selects `AdaptiveVRLittleEndianDecoder` - not
`dicom_object::open_file`/`from_reader`, nor the transfer-syntax registry from a UID alone, which
never call it. Candidate 095's fix (`DecodeFrom::is_implicit_vr()`, overridden by
`AdaptiveVRLittleEndianDecoder` to read its own `state: Cell<VrState>`) was verified there by code
reading only. This candidate builds and runs a repro through that real entry point directly.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `DataSetReader::new_with_ts_cs_options` (`parser/src/dataset/read.rs`) | n/a | Confirmed it selects `Box::<StandardAdaptiveVRLittleEndianDecoder>::default()` as a `DynDecoder<R>` (`Box<dyn DecodeFrom<R>>`) whenever `options.flexible_decoding` is `true` and the transfer syntax is little-endian - a plain public API (`DataSetReaderOptions::default().flexible_decoding(true)` + `DataSetReader::new_with_ts_options`), reachable without `dicom-object` or `dcms` at all |
| Manual boundary-value construction: built two raw little-endian data sets by hand (no full P10 file needed - `DataSetReader` operates on the data set directly) containing (0028,0103) Pixel Representation=1 (signed) followed by (0028,0106) SmallestImagePixelValue=0xFFFF (dictionary VR `Xs`) - one entirely in Implicit VR wire form (the non-conformant case this decoder exists for), one entirely in Explicit VR wire form (`US`) - fed both through `DataSetReader::new_with_ts_options` with the declared transfer syntax "1.2.840.10008.1.2.1" (Explicit VR Little Endian) and `flexible_decoding(true)`, on the current (095-patch-reverted) working tree | debug and release builds, `cargo run` against a standalone crate using `dicom-parser`'s own public API | Reproduced: the Explicit-VR-wire case decoded (0028,0106) as `VR::SS`/`I16([-1])` instead of the wire-declared `VR::US`/`U16([65535])` - i.e. the exact candidate-095 bug, now demonstrated through `AdaptiveVRLittleEndianDecoder` once it locks to `VrState::Explicit`. The Implicit-VR-wire case correctly decoded as `VR::SS`/`I16([-1])` (the legitimate use of the override). 3/3 repeat runs, deterministic |
| Re-ran the same two cases with candidate 095's `patch.diff` applied | same build | Explicit-VR-wire case now correctly decodes as `VR::US`/`U16([65535])`; Implicit-VR-wire case is unchanged (`VR::SS`/`I16([-1])`) - both cases correct after the fix |

Two escalating approaches were enough: code reading located the exact selection mechanism, and
manual boundary-value construction (matching `HEURISTICS.md`'s guidance for a logic bug with a
documented invariant to violate) reproduced it immediately and repeatably through the real entry
point; no third tool was needed.

## Status

**Confirmed**: reproduces 3/3 runs, deterministically, on every attempt - both directly against
the main project's working tree and against a fresh `git clone` of the real upstream repo at
`base_commit` via this candidate's own `repro/` (`just test`).

**Fully reproducible**: **Yes**. The repro drives the real, public, end-to-end entry point a real
caller would use - `dicom_transfer_syntax_registry::TransferSyntaxRegistry::get` to resolve a
transfer syntax UID string exactly as it would come from a file's own declared Transfer Syntax
UID, then `dicom_parser::dataset::read::DataSetReader::new_with_ts_options` with
`DataSetReaderOptions::default().flexible_decoding(true)`, feeding it a byte sequence with the
exact wire layout a real non-conformant (or, in the second case, perfectly conformant) file would
have. Per `CONFIRMATION.md` section 11, a data set handed to the parser's own public streaming
entry point is the genuine external input; there is no second participant being impersonated here.

## Class

logic

## Root cause

Same root cause as candidate 095, now confirmed to manifest through this second entry point too:
`StatefulDecoder::determine_vr_based_on_pixel_representation` applied its `Xs` -> `SS` override
regardless of which decoder produced the header. `AdaptiveVRLittleEndianDecoder` is reached via
`DynDecoder<R>` (`Box<dyn DecodeFrom<R>>`), and once it has probed the first non-delimiter element
and locked its internal `state: Cell<VrState>` to `Explicit`, it decodes subsequent headers by
reading the wire's own 2-character VR code directly (`decode_explicit_header`) - exactly as
authoritative as `ExplicitVRLittleEndianDecoder`'s own decoding, and just as wrongly overridden by
the unconditional pre-095 check.

Candidate 095's fix threading `is_implicit_vr()` through the `Box<T>`/`&T` forwarding impls of
`DecodeFrom` (not just the concrete decoder types) is exactly what makes this fix apply correctly
here: `DataSetReader`'s `StatefulDecoder<DynDecoder<R>, R>` only ever holds the decoder behind that
boxed trait object, so without those forwarding impls the fix would have silently fallen back to
the trait's default (`false`) for every call through this entry point, whatever the decoder's real
state - the fix as written already accounted for this, but candidate 095's own session never
verified it end-to-end through this specific path.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

No new fix was needed for this candidate; it confirms candidate 095's existing `patch.diff`
already covers this path correctly. Verified by applying that exact `patch.diff` (unmodified,
copied into this candidate's own folder for self-containment) to a fresh clone at `base_commit`
and re-running the same repro: the Explicit-VR-wire case is now correctly resolved as `VR::US` with
the correct unsigned value, while the Implicit-VR-wire case (the override's legitimate use) is
unaffected.

- Project test suite (`cargo test --workspace --no-fail-fast`): 537 passed, 0 failed, both before
  and after applying `patch.diff` on the main project's own working tree - no regressions, and
  (consistent with candidate 095's own finding) no pre-existing test in the suite already covered
  this path either way.

## What's not done / open questions

- This closes the specific open question left by candidate 095 ("no dedicated repro through
  whichever public entry point actually selects `AdaptiveVRLittleEndianDecoder`") - no new distinct
  bug was found; this is the same bug/fix, now verified through a second real trigger path.
  `IDEAS.md`'s row for this item is being removed as resolved.
- `DataSetReaderOptions::flexible_decoding` itself is not called anywhere in `dicom-object`'s or
  `dcms`'s own code (confirmed by repo-wide grep, excluding `target/`) - it is currently reachable
  only by an external caller using `dicom-parser` directly. Whether it's worth wiring up as an
  opt-in in a higher-level API (e.g. `OpenFileOptions`) is a product decision for the maintainers,
  not a bug.
