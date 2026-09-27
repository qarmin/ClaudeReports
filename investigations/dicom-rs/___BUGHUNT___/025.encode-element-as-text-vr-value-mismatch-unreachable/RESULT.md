# Writing a DS/IS-VR element panics if its stored value isn't numeric/textual

Found via the same "audit every `unreachable!()` in the codebase" sweep that ruled out several
false positives in `object/src/mem.rs` (all of those turned out to be genuinely unreachable,
protected by `AttributeSelector`'s "last step is always a tag" construction invariant).
`parser/src/stateful/encode.rs`'s `StatefulEncoder::encode_element_as_text` - an "edge case
method for encoding data elements with IS and VR values" (i.e. VR::DS/VR::IS elements whose
stored value is *not* already textual) - handles `PrimitiveValue::Empty` and the nine numeric
variants (`U8`..`F64`), then falls through to `unreachable!()` for `Date`/`DateTime`/`Time`/
`Tags`/`Strs`/`Str`. The comment implies this can't happen because `Str`/`Strs` are intercepted
earlier by the caller (true), but nothing rules out `Date`/`DateTime`/`Time`/`Tags` - a
`DataElement`'s VR and its `PrimitiveValue` variant are independent fields, not kept in sync by
the type system, and `dicom_core::ops::AttributeAction::SetVr` is a *documented, public*
operation whose doc comment explicitly says "the underlying value is not modified" - i.e. it is
designed to let exactly this combination exist (change the VR, keep the value as-is).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Grepped the whole workspace for `unreachable!()` (23 occurrences) after two of them turned into confirmed bugs in `object/src/meta.rs` (`023`/`024`). Checked each of `object/src/mem.rs`'s four (`value_at`, `value_at_mut`-equivalents, `apply`'s selector-walk) and confirmed all four are genuinely guarded by `AttributeSelector`'s invariant that its last step is always `AttributeSelectorStep::Tag` (enforced by a private field + guarded constructors, `core/src/ops.rs`). Checked `parser/src/stateful/encode.rs:414` next: traced `encode_primitive_element`'s dispatch (intercepts `Str`/`Strs` first, routes everything else with VR DS/IS to `encode_element_as_text`) and found `Date`/`DateTime`/`Time`/`Tags` are NOT excluded by that interception, and are reachable if the corresponding `DataElement`'s VR is DS/IS. | Identified `AttributeAction::SetVr` as the natural, documented way to produce this VR/value combination. |
| Standalone repro (release build, public API: `DataElement::new`, `dicom_core::ops::{ApplyOp, AttributeAction, AttributeOp}`, `write_dataset_with_ts`) | `___BUGHUNT___/025.../repro`. Created an ordinary element with VR `DA` holding a `PrimitiveValue::Date`, applied `AttributeOp::new(tag, AttributeAction::SetVr(VR::DS))` (succeeds, per its own contract), then called `write_dataset_with_ts`. | Reproduced deterministically: panicked with `internal error: entered unreachable code` at `parser/src/stateful/encode.rs:414:41`. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically (fully deterministic). |
| Reachability check | `encode_primitive_element`'s dispatch (`parser/src/stateful/encode.rs`) intercepts only `PrimitiveValue::Str`/`Strs` before routing to `encode_element_as_text` for VR DS/IS - so `Date`/`DateTime`/`Time`/`Tags` values genuinely do reach the buggy function whenever an element's VR is DS/IS and its value is one of those four variants. `SetVr`'s own doc comment ("the underlying value is not modified... implementations are free to ignore this request if it cannot be done or it does not make sense") confirms this VR/value split is an intended, supported capability rather than an internal invariant violation. | N/A (confirms this isn't a contrived state achievable only through unsafe/internal APIs - it's the direct, documented result of one public operation). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build.

## Class

`crash` (unhandled `unreachable!()` panic when serializing an ordinary in-memory object whose
element VR was changed independently of its stored value, via the crate's own public,
documented `SetVr` operation).

## Root cause

`StatefulEncoder::encode_element_as_text` (`parser/src/stateful/encode.rs`) assumes a VR::DS/IS
element's `PrimitiveValue` is always `Empty` or one of the nine numeric variants, having
implicitly relied on its caller (`encode_primitive_element`) to rule out `Str`/`Strs` - but
`encode_primitive_element` never rules out `Date`/`DateTime`/`Time`/`Tags`, and nothing in the
`DataElement`/`AttributeAction` API keeps a value's variant in sync with its VR (indeed,
`AttributeAction::SetVr` exists specifically to change one independently of the other).

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Merged `Date`/`DateTime`/`Time`/`Tags`/`Strs`/`Str` into the same match arm as the numeric
variants - `PrimitiveValue::to_str()` already produces a sensible textual representation for
every variant (used elsewhere for `Display`), so the existing text-encoding logic handles them
correctly with no further changes needed. This makes `encode_element_as_text` total over all
`PrimitiveValue` variants instead of relying on an unenforced caller invariant. See `patch.diff`
(`parser/src/stateful/encode.rs`, one match arm).

Verified:
- `repro/` re-run against the patched tree no longer panics: the DS-VR-with-Date-value element
  now writes successfully (as its textual date representation, per `to_str()`).
- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, identical
  both before and after the patch (no regressions; no existing test exercised this VR/value
  combination).

## What's not done / open questions

- Did not check whether the analogous *decoding* path can ever produce this same VR/value
  mismatch on its own (without `SetVr`) - e.g. whether any transfer syntax's decoder could tag an
  element as DS/IS while still parsing its bytes as a Date/Time/DateTime/Tags value. This seems
  unlikely (decoders pick the `PrimitiveValue` variant based on the VR they just read, so a DS/IS
  header should always produce a numeric-or-string variant), but was not exhaustively verified;
  the confirmed repro path here is exclusively through `SetVr`.
- Checked all `unreachable!()` sites in `object/src/`, `parser/src/stateful/encode.rs`, and
  `core/src/ops.rs`'s `AttributeSelector` invariant as part of this sweep; did not exhaustively
  check the remaining ~15 occurrences elsewhere in the workspace (`encoding/src/adapters.rs`,
  `parser/src/dataset/mod.rs`, `transfer-syntax-registry/src/adapters/jpegxl.rs`,
  `json/src/de/mod.rs`, `core/src/value/range.rs`, `core/src/header.rs`,
  `object/src/collector.rs`) due to running low on session time - these remain unaudited leads
  for a future session.
- Not fuzzed - found by systematically auditing every `unreachable!()` call site in the
  workspace after two hits in a row in `object/src/meta.rs` suggested the pattern was worth
  sweeping broadly rather than opportunistically.
