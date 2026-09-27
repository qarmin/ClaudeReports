# Writing a mismatched VR/PrimitiveValue element silently corrupts the encoded bytes instead of erroring

`parser/src/stateful/encode.rs`'s `StatefulEncoder::encode_primitive_element` intercepts
`PrimitiveValue::Str`/`Strs` to use the text codec, special-cases `VR::DS`/`VR::IS` to render a
binary value via `to_str()`, but otherwise falls straight through to
`self.encoder.encode_primitive(...)`, which writes the value's bytes according to its own Rust
variant (native-endian numbers, raw tag words) with no awareness of the declared `VR` at all.
Nothing in the public `DataElement`/`InMemDicomObject` API keeps a `DataElementHeader`'s `vr`
field in sync with the actual `PrimitiveValue` variant stored alongside it - `DataElement::new`'s
own doc comment says outright "This method will not check whether the value representation is
compatible with the given value" - so a caller can freely construct (and `put`/`update_value`) an
element whose VR says "this is text" while its value is raw binary. Flagged in
`___BUGHUNT___/IDEAS.md` as the downstream-impact check for candidate `084` (which fixed the one
producer of this mismatch reachable from the JSON deserializer), noting other public mutation
APIs can still reach it directly.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Traced `encode_primitive_element`'s dispatch and the concrete `encode_primitive` impl used by all transfer syntaxes (`encoding/src/encode/mod.rs`'s `BasicEncode::encode_primitive` default method) to see exactly which `PrimitiveValue` variants get real textual conversion (`Str`/`Strs`/`Date`/`Time`/`DateTime`, all rendered as ASCII text unconditionally) versus which are written as raw native-endian bytes/tag words with zero conversion (`U8`/`I16`/`U16`/`I32`/`U32`/`I64`/`U64`/`F32`/`F64`/`Tags`). Confirmed `encode_element_header` (`encoding/src/encode/explicit_le.rs`) picks the header's length-field width purely from `de.vr`, independent of what `encode_primitive` is about to write. | Identified the concrete corruption path: a "genuinely binary" `PrimitiveValue` variant paired with a VR whose value is defined as always-textual per PS3.5 Table 6.2-1 (excluding `DS`/`IS`, already special-cased) reaches the raw-byte-write branch with no validation. |
| Standalone repro (release build, public API: `DataElement::new`, `InMemDicomObject::put_element`, `write_dataset_with_ts`, `FileDicomObject::from_reader`) | `___BUGHUNT___/092.../repro`. Built an ordinary element with declared VR `UI` (a textual UID VR) holding `PrimitiveValue::U32([0xDEADBEEF, 1, 0xFFFFFFFF])`, wrote it with `write_dataset_with_ts`, inspected the raw output bytes, and round-tripped the whole file through the crate's own `from_reader`. | `write_dataset_with_ts` returned `Ok(())` (no error, no panic). The 12-byte "UI" value field on the wire was the raw little-endian `u32` bytes (`ef be ad de 01 00 00 00 ff ff ff ff`), not a valid DICOM UID text string (0x00/space-padded printable ASCII). Round-tripping through the crate's own reader produced `Strs(["ï¾\u{ad}Þ\u{1}\0\0\0ÿÿÿÿ"])` - a nonsense nul-containing string silently reconstructed from the original numbers, with no error anywhere in the write-then-read cycle. |
| Repeat-run check | `./target/release/repro` x3 | 3/3 runs reproduced identically (fully deterministic, no layout/seed dependence - purely a logic gap). |
| Second input in the same bug class (CONFIRMATION.md section 7) | Same repro, edited to use `VR::PN` + `PrimitiveValue::F64([1.5, -2.25])` instead of `VR::UI` + `U32`, run against the patched tree in a throwaway `/tmp` copy | Also correctly rejected with the new `VrValueMismatch` error - the fix isn't narrow to the one VR/variant pair that first triggered it. |
| Full project test suite, pristine tree (baseline) | `cargo test --workspace --lib` | 351 passed / 0 failed / 2 ignored. |

## Status

**Confirmed** - reproduces 3/3 runs, release build, deterministic (pure logic gap, no
memory/layout dependence).

**Fully reproducible**: **Yes**. The repro drives the real, public entry points a genuine caller
would use end to end: `DataElement::new` (documented as intentionally not validating VR/value
compatibility) -> `InMemDicomObject::put_element` -> `FileDicomObject::write_dataset_with_ts` (the
real encoder) -> `FileDicomObject::from_reader` (the real decoder), all in one process. This is a
single-participant finding (one caller misusing one library's own public construction API against
its own writer/reader, not one network peer reacting to another's wire input), so `CONFIRMATION.md`
section 11's two-participant requirement does not apply here.

## Class

`logic` (no crash; the writer silently serializes a value under a value-representation header the
two disagree with, instead of returning an error - producing a spec-violating on-disk element that
its own reader then silently misinterprets on the way back in).

## Root cause

`StatefulEncoder::encode_primitive_element` only special-cases `VR::DS`/`VR::IS` (rendering a
binary value as decimal text via `to_str()`) and unconditionally special-cases
`PrimitiveValue::Str`/`Strs` (routed to the text codec regardless of VR). Every other
combination - specifically, any VR whose value is defined as always-textual by PS3.5 (`AE`, `AS`,
`CS`, `DA`, `DT`, `LO`, `LT`, `PN`, `SH`, `ST`, `TM`, `UC`, `UI`, `UR`, `UT`) paired with a
genuinely binary `PrimitiveValue` variant (`U8`/`I16`/`U16`/`I32`/`U32`/`I64`/`U64`/`F32`/`F64`/
`Tags`) - falls through to `self.encoder.encode_primitive`, which encodes strictly by the value's
own Rust variant with no reference to the declared VR at all. The header's length-field format is
chosen from `de.vr` alone (`encode_element_header`), so the result looks like a syntactically
well-formed short-form text element on the wire while its value bytes are raw binary. No layer
between the public mutation APIs (`DataElement::new`, `put`, `update_value`) and the encoder
enforces that a `DataElementHeader`'s VR and its paired `PrimitiveValue`'s variant actually agree.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a `VrValueMismatch` error variant to `parser::stateful::encode::Error`, and a check in
`encode_primitive_element`'s fallback branch (after the existing `DS`/`IS` special case): if the
value is a "genuinely binary" variant (everything except `Empty`/`Date`/`Time`/`DateTime`, which
`encode_primitive` already renders correctly as text unconditionally) and the declared VR is one
of the always-textual VRs listed above, return `Err(Error::VrValueMismatch)` instead of writing
the mismatched bytes. See `patch.diff` (`parser/src/stateful/encode.rs`).

The first version of this fix incorrectly included `DA`/`DT`/`TM` unconditionally in the
"visibly wrong VR" branch without excluding `Date`/`Time`/`DateTime` values, which broke a
legitimate, pre-existing use case (`object::mem::tests::inmem_object_write_datetime_odd`, which
constructs a `VR::DT` element from a `PrimitiveValue::DateTime` - the correct, intended way to
populate that VR, already handled correctly and textually by `encode_primitive` itself). Caught by
running the full test suite before finalizing, not by the repro alone - the repro only exercises
the one mismatched pair it was built around, and does not exercise the codebase's legitimate
Date/Time/DateTime-VR pairings, which is exactly the gap `CONFIRMATION.md` section 7 warns about.

Verified:
- `repro/` re-run against the patched tree (via `just test`, a fresh clone from
  `https://github.com/Enet4/dicom-rs.git` at the base commit) no longer reproduces: the write call
  now returns a graceful `Err(VrValueMismatch)` instead of silently emitting corrupted bytes.
  Confirmed 3/3 runs.
- A second VR/value pair in the same bug class (`VR::PN` + `PrimitiveValue::F64`) is also rejected
  by the fix, not just the exact pair that first triggered it.
- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, both
  before and after the (corrected) patch - no regressions, including the `DA`/`DT`/`TM` +
  `Date`/`Time`/`DateTime` legitimate-use test that the first (incorrect) version of the fix broke.

## What's not done / open questions

- The fix only covers the direction demonstrated here: a "binary" `PrimitiveValue` variant paired
  with an always-textual VR. The mirror direction - `PrimitiveValue::Str`/`Strs` paired with a VR
  that expects binary data (e.g. `VR::US` with a `Str` value) - is unconditionally intercepted
  earlier in `encode_primitive_element` and routed to the text codec without any VR check either;
  this candidate did not build a repro for that direction or extend the fix to cover it. Added to
  `IDEAS.md` as a follow-up lead.
- Did not extend validation to `DataElement::new`/`put`/`update_value` themselves (i.e. rejecting
  the mismatch at construction time rather than at encode time) - `DataElement::new`'s doc comment
  explicitly documents that it will not perform this check, so changing that would be an API
  contract change beyond this candidate's bounded scope; encode-time rejection was chosen instead
  since it's the point where the mismatch actually causes externally-visible harm (a corrupted
  file on disk).
- Did not check whether any *decoder* path (as opposed to the public mutation APIs checked here)
  can produce this same VR/value mismatch on its own while parsing untrusted input - this
  candidate's confirmed reachability path is exclusively through the public construction/mutation
  APIs, matching what `IDEAS.md`'s entry for this candidate already narrowed it down to.
