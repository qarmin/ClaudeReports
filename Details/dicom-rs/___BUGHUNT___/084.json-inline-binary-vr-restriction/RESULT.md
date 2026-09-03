# json-inline-binary-vr-restriction

`json/src/de/mod.rs`'s `DataElementVisitor::visit_map` (backing the public
`dicom_json::from_str`/`from_reader`/`from_slice`/`from_value` DICOM+JSON deserializers, e.g. a
DICOMweb QIDO-RS/STOW-RS payload) accepts the `"InlineBinary"` JSON field for *any* declared VR,
with no check against PS3.19 Table A.1.5-2's whitelist of VRs actually allowed to carry
`InlineBinary` (OB/OD/OF/OL/OV/OW/UN). Carried over from a prior session's `IDEAS.md` backlog
entry; an unmerged upstream fix already exists at `3af9da79` (`upstream/bug/json/binary-inline`).
Note: that same upstream commit's diff also independently fixes already-covered candidate 027 (the
`values.is_some()` vs `value.is_some()` typo in the `InlineBinary`/`BulkDataURI` conflict guards) -
only the VR-restriction half is new/reported here; `patch.diff` in this folder deliberately
contains only that half, left untangled from 027's own fix.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction against the real public API (debug build) | `{"00100010":{"vr":"DS","InlineBinary":"AAE="}}"` via `dicom_json::from_str::<InMemDicomObject>` | Deserializes successfully; the resulting element's declared VR is `DS` (text) but its actual `PrimitiveValue` is `U8([0, 1])` - a type-confused, spec-violating in-memory element with nothing in the public API rejecting it |
| Downstream-impact check: real write/read round trip through the crate's own public `write_to_file`/`open_file`, VR `DS` (checked whether re-encoding a mismatched-variant value panics anywhere) | Same object as above, written via `with_meta(...).write_to_file(...)`, read back via `open_file(...)` | No panic. `parser/src/stateful/encode.rs`'s `encode_element_as_text` has a pre-existing special case for `VR::DS \| VR::IS` with a non-string `PrimitiveValue`: it calls `value.to_str()` and writes *that* as text instead of the raw bytes. The file ends up containing the literal bytes `"0\1 "` (the decimal/backslash rendering of `[0, 1]`), not `[0, 1]` itself - silently wrong content, not a crash |
| Same downstream-impact check with a VR that has no such special case (`LO`, invalid-UTF-8 raw bytes `[0xFF, 0xFE, 0x00, 0x41]` via InlineBinary) | Same write/read round trip, VR `LO` | Still no panic. The generic (non-DS/IS) branch of `encode_primitive_element` writes the raw `U8` bytes verbatim under the `LO` header (no VR-specific text conversion outside DS/IS), producing an on-disk "Long String" element containing genuinely non-text, non-UTF-8 bytes - invalid per PS3.5, but the decoder on read-back treats it as Latin-1-ish text and returns a `Strs` value with no panic either |
| Code reading of `dicom_json`'s own re-serializer (`json/src/ser/value.rs`'s `AsStrings`/`AsNumbers`/`AsAttributeTags`), to check whether re-serializing one of these type-confused elements back to JSON panics | Static read of all match arms reachable from a `PrimitiveValue::U8` payload (the only variant `InlineBinary` can actually produce) | No panic reachable via this bug: `AsStrings`/`AsNumbers` both have `U8` arms that serialize the raw bytes as numbers/strings without panicking; `AsAttributeTags` returns a serde error (not a panic) for any non-`Tags` value. `AsNumbers`'s `panic!` arms are for `Date`/`DateTime`/`Time`/`Tags`, none of which `InlineBinary` can ever produce (it always yields `PrimitiveValue::from(bytes)`, i.e. `U8`) |

## Status

**Confirmed** - deterministic, pure-logic bug with no layout/seed/timing dependency. The core
deserialization type-confusion reproduces 2/2 direct runs plus a further 2/2 standalone `repro/`
runs (once ad hoc against the in-tree checkout, once via a genuinely fresh clone through
`just test`), all against the debug/release-agnostic control flow (this is ordinary branching, not
something instrumentation-only reveals). The downstream-impact checks (write/read round trip,
JSON re-serialization) were code-read/spot-checked once each and consistently found "no panic,
silently wrong content instead" - recorded above as what was actually covered, not as an
exhaustive sweep of every VR/encoder combination.

**Fully reproducible: Yes.** This is a single external artifact (a malformed/spec-violating
DICOM+JSON document) handed to the crate's own real public deserialization entry point
(`dicom_json::from_str`) - the same bytes a real DICOMweb client/attacker would submit to a
STOW-RS endpoint built on this crate. There is no second participant to fake here (see
`CONFIRMATION.md` section 11's own carve-out for exactly this shape of finding).

## Class

`logic` (a spec-conformance / data-integrity bug reachable from ordinary use of a public
deserialization entry point on attacker/user-supplied JSON text; not a crash - the round-trip
checks above found the type-confused value gets silently reinterpreted rather than causing UB or
a panic anywhere exercised).

## Root cause

`DataElementVisitor::visit_map`'s closing `match (values, inline_binary)` has exactly two "real"
outcomes for `InlineBinary`: `(None, None)` (empty) and `(None, Some(inline_binary))` (decode
base64 unconditionally, regardless of the already-known `vr`). PS3.19 Table A.1.5-2 restricts
`InlineBinary` to VRs OB/OD/OF/OL/OV/OW/UN specifically because those are the only VRs whose
`PrimitiveValue` representation is naturally "a bunch of raw bytes" - every other VR (text,
numeric-as-text, dates, tags, person names, sequences) has its own dedicated JSON `"Value"`
encoding earlier in the same function (the big VR match at lines 217-351) precisely because a
literal byte dump isn't a meaningful encoding for those VRs. By never consulting `vr` in the
`InlineBinary` arm, the code lets a caller declare any VR at all and get a raw-byte
`PrimitiveValue::U8` back regardless, silently producing an element whose declared VR and actual
value representation disagree - an invariant every other part of the crate (VR-driven decoding,
VR-driven typed accessors, VR-driven encoding) otherwise assumes holds.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Ported the VR-restriction half of upstream commit `3af9da79` (`upstream/bug/json/binary-inline`):
changed the closing match to also inspect `vr`, keeping the base64-decode arm only for
`VR::OB | VR::OD | VR::OF | VR::OL | VR::OV | VR::OW | VR::UN` (PS3.19 Table A.1.5-2), and added a
new arm that returns a proper `serde_json`/`serde::de::Error::custom` for any other VR paired with
`InlineBinary`, instead of silently accepting it. Deliberately left the pre-existing
`values.is_some()`/`value.is_some()` typo in the surrounding conflict guards untouched, since
that's candidate 027's own already-recorded fix, not part of this candidate.

Checked against a second input in the same class, not just the one that first triggered it (per
`CONFIRMATION.md` section 7): re-ran the patched deserializer against a *whitelisted* VR
(`{"7FE00010":{"vr":"OB","InlineBinary":"AAE="}}"`) to confirm the fix doesn't also break the
legitimate case - it still deserializes to `Primitive(U8([0, 1]))` as expected, while the original
`DS` input is now rejected with `"value of VR DS should not be encoded in InlineBinary"`.

- Project test suite: `cargo test -p dicom-json --lib`: 12 passed before -> 12 passed after (no
  new failures). `cargo test --workspace --lib`: 351 passed / 2 ignored before -> 351 passed / 2
  ignored after (identical, summed across all 12 crates' lib test binaries). No new tests were
  added to the in-tree suite as part of this session (the upstream fix commit's own test coverage
  was left unapplied, to keep `patch.diff` minimal and isolated to the VR-restriction change
  itself); `repro/` plus this session's own ad hoc probes serve as the verification instead.

## What's not done / open questions

- The downstream-impact sweep only checked two VR classes (`DS`/`IS`'s special-cased text
  conversion, and one representative "plain" text VR, `LO`) plus a static read of the JSON
  re-serializer's match arms - not every VR/encoder/typed-accessor combination that could receive
  a type-confused value from this bug. A maintainer wanting full confidence that *no* downstream
  code path can be made to panic via this exact input class would need a broader sweep (e.g. a
  short fuzz harness feeding `InlineBinary` + every VR through both the JSON deserializer and the
  file writer, via `fuzz-bug-repro`, run separately).
- Even with this fix applied, the *encoder*'s own DS/IS-specific "write mismatched value as text"
  fallback (`parser/src/stateful/encode.rs`'s `encode_element_as_text`) and the *generic*
  "write whatever bytes are actually stored, regardless of VR" behavior for other VRs remain
  unchanged - they were only exercised here as evidence of "no panic", not flagged as their own
  bug, since they only need to reproduce their present code intent under a value/VR mismatch that,
  after this session's fix, can no longer reach them via the JSON deserializer specifically. A
  mismatch could still reach them via the object crate's other public mutation APIs
  (`DataElement::new`, `put`, `update_value`) with a hand-constructed `PrimitiveValue`/`VR` pair
  that disagrees on purpose - not checked this session, and out of scope for the JSON-specific
  finding here.
