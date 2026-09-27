# Writing a textual value under a binary-VR header silently corrupts the encoded bytes instead of erroring

`parser/src/stateful/encode.rs`'s `StatefulEncoder::encode_primitive_element` intercepts
`PrimitiveValue::Str`/`Strs` unconditionally at the very top of its `match`, before any VR check,
and always routes them to the text codec (`encode_text_element`/`encode_texts_element`), which
writes the header using whatever `VR` the caller declared. Nothing in the public
`DataElement`/`InMemDicomObject` API keeps a `DataElementHeader`'s `vr` field in sync with the
actual `PrimitiveValue` variant stored alongside it - `DataElement::new`'s own doc comment says
outright "This method will not check whether the value representation is compatible with the
given value" - so a caller can freely construct (and `put`/`update_value`) an element whose VR
says "this is binary" (e.g. `VR::US`) while its value is a `PrimitiveValue::Str`/`Strs`. This is
the mirror direction of candidate `092` (a binary `PrimitiveValue` under a textual VR), explicitly
scoped out and left open in `092`'s own `RESULT.md`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Traced `encode_primitive_element`'s `match value { ... }` dispatch: `PrimitiveValue::Str`/`Strs` are matched first, unconditionally, calling `encode_text_element`/`encode_texts_element` with the caller-declared `de.vr` passed straight through with no check against it. Confirmed `encode_element_header` (`encoding/src/encode/explicit_le.rs`) picks the header's length-field width (short 2-byte vs. long 4-byte form) purely from `de.vr`, independent of what was actually written as the value. | Identified the concrete corruption path: any VR whose value is defined as binary per PS3.5 Table 6.2-1 (e.g. `US`, `UL`, `SS`, `SL`, `FL`, `FD`, `AT`, `OB`, `OW`, `OD`, `OF`, `OL`, `OV`, `SV`, `UV`, `UN`), paired with a `PrimitiveValue::Str`/`Strs`, reaches the text-codec branch with no validation. |
| Manual boundary-value construction, in-tree | A one-off `object/examples/quick_check.rs` (removed after use, working tree left clean) built a `VR::US` element holding `PrimitiveValue::Str("hello".to_string())` via `DataElement::new`, wrote it with `write_dataset_with_ts`, and round-tripped it through `FileDicomObject::from_reader`. | `write_dataset_with_ts` returned `Ok(())`. The wire bytes for the 6-byte "US" value field were the literal ASCII text `"hello "` (space-padded), not binary `u16`s. Round-tripping through the crate's own reader produced `Primitive(U16([25960, 27756, 8303]))` - the original text silently replaced by nonsense numbers, no error anywhere in the write-then-read cycle. |
| Standalone repro (release build, public API: `DataElement::new`, `InMemDicomObject::put_element`, `write_dataset_with_ts`, `FileDicomObject::from_reader`) | `___BUGHUNT___/093.../repro`, run via `just test` against a fresh clone of the base commit | Reproduced identically to the in-tree check above; see `output.txt`. |
| Repeat-run check | `cargo run --release` x3 (in-tree quick check, pre-fix) + the standalone `repro/` run x3 more (pre-fix, via a scratch copy pointed at the in-tree crates) | 3/3 (in-tree) + 3/3 (standalone `repro/`) runs reproduced identically (fully deterministic, no layout/seed dependence - a pure logic gap). |
| Second input in the same bug class (CONFIRMATION.md section 7) | Same repro, edited to use `VR::UL` + `PrimitiveValue::Strs(["world", "again"])` instead of `VR::US` + `Str` | Also correctly rejected with the new `VrValueMismatch` error after the fix - the fix isn't narrow to the one VR/variant pair that first triggered it. |
| Full project test suite, pristine tree (baseline) | `cargo test --workspace --lib` | 351 passed / 0 failed / 2 ignored. |

## Status

**Confirmed** - reproduces 3/3 (in-tree quick check) and separately 3/3 (standalone `repro/`)
runs, release build, deterministic (pure logic gap, no memory/layout dependence).

**Fully reproducible**: **Yes**. The repro drives the real, public entry points a genuine caller
would use end to end: `DataElement::new` (documented as intentionally not validating VR/value
compatibility) -> `InMemDicomObject::put_element` -> `FileDicomObject::write_dataset_with_ts` (the
real encoder) -> `FileDicomObject::from_reader` (the real decoder), all in one process, exactly
mirroring how candidate `092`'s repro was judged. This is a single-participant finding (one caller
misusing one library's own public construction API against its own writer/reader, not one network
peer reacting to another's wire input), so `CONFIRMATION.md` section 11's two-participant
requirement does not apply here.

## Class

`logic` (no crash; the writer silently serializes a textual value under a binary-VR header the two
disagree with, instead of returning an error - producing a spec-violating on-disk element that its
own reader then silently misinterprets as native-endian numbers/tags on the way back in).

## Root cause

`StatefulEncoder::encode_primitive_element` matches `PrimitiveValue::Str`/`Strs` first and
unconditionally, before any VR check, and routes them straight to `encode_text_element`/
`encode_texts_element`, which write the value's UTF-8/charset-converted bytes under whatever
`de.vr` the caller declared. Any VR whose value is defined as binary by PS3.5 Table 6.2-1
(`AT`, `FL`, `FD`, `OB`, `OD`, `OF`, `OL`, `OV`, `OW`, `SL`, `SS`, `SV`, `UL`, `UN`, `US`, `UV`)
paired with a `PrimitiveValue::Str`/`Strs` therefore gets its text bytes written as-is under a
binary-VR header, with the header's length-field format (`encode_element_header`) chosen purely
from `de.vr`. No layer between the public mutation APIs (`DataElement::new`, `put`,
`update_value`) and the encoder enforces that a `DataElementHeader`'s VR and its paired
`PrimitiveValue`'s variant actually agree - the same gap `092` found in the opposite direction.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a `VrValueMismatch` error variant to `parser::stateful::encode::Error`, and a check at the
top of `encode_primitive_element`'s `match value { ... }`: if the value is `PrimitiveValue::Str`
or `Strs` and the declared VR is one of the binary-only VRs listed above (`DS`/`IS` excluded -
those are genuinely textual VRs that intentionally accept `Str`/`Strs`; `SQ` excluded - sequences
never carry a `PrimitiveValue` at this call site), return `Err(Error::VrValueMismatch)` instead of
writing the mismatched bytes. See `patch.diff` (`parser/src/stateful/encode.rs`).

Before drafting the final VR list, grepped the whole workspace (including all test modules) for
any existing `Str`/`Strs` value paired with one of the candidate binary VRs, to check for a
legitimate use case the fix might break (the exact failure mode that hit `092`'s first attempt,
which broke a legitimate `DA`/`DT`/`TM` + `Date`/`Time`/`DateTime` test) - found none.

Verified:
- The standalone `repro/` re-run against the patched tree (via `just test`, a fresh clone from
  `https://github.com/Enet4/dicom-rs.git` at the base commit) no longer reproduces: the write call
  now returns a graceful `Err(VrValueMismatch)` instead of silently emitting text bytes under a
  binary-VR header. See `output.txt` for the full captured `just test` run (before: reproduces;
  after: does not).
- A second VR/value pair in the same bug class (`VR::UL` + `PrimitiveValue::Strs`) is also
  rejected by the fix, not just the exact pair that first triggered it.
- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, both
  before and after the patch - no regressions.

## What's not done / open questions

- Did not extend validation to `DataElement::new`/`put`/`update_value` themselves (i.e. rejecting
  the mismatch at construction time rather than at encode time), for the same reason `092` chose
  not to: `DataElement::new`'s doc comment explicitly documents that it will not perform this
  check, so changing that would be an API contract change beyond this candidate's bounded scope;
  encode-time rejection is the point where the mismatch actually causes externally-visible harm (a
  corrupted file on disk).
- Did not check whether any *decoder* path (as opposed to the public mutation APIs checked here)
  can produce this same VR/value mismatch on its own while parsing untrusted input - this
  candidate's confirmed reachability path is exclusively through the public construction/mutation
  APIs, matching `092`'s own scoping.
- Between `092` and this candidate, both directions of the VR/`PrimitiveValue` mismatch bug class
  identified in `092`'s "What's not done" section are now covered by a fix in
  `encode_primitive_element` (though the two candidates' fixes are independent, unapplied patches
  against the same base commit - they were not combined/tested together in one patch this
  session, since `092`'s patch was reverted from the tree per this skill's workflow before this
  candidate started).
