# json-asnumbers-vr-value-mismatch-panic

`json/src/ser/value.rs`'s `AsNumbers` serializer (used by `json/src/ser/mod.rs`'s `DicomJson<&
InMemElement<D>>::serialize` for VR::DS/FL/FD/IS/SL/SS/SV/UL/US/UV) `panic!()`s for
`PrimitiveValue::Date`/`DateTime`/`Time`/`Tags` instead of returning a serialization error. Found
by reading `json/src/ser/` right after noticing the freshest commit on this branch (`df2b9e42`,
merge of PR #821/#822, "youdie006/fix/812-json-sq-primitive-panic") had *just* fixed the
structurally identical bug for one specific case (`VR::SQ` with a primitive value, in the same
`serialize` function this `AsNumbers` helper is called from) - a strong, explicit signal that the
crate's own `DataElement`-VR-vs-stored-value-variant mismatch class (already documented in
`projects/dicom-rs.md` from the 2026-08-24 session's candidates `025`/`026`) likely still has
unfixed siblings nearby, per `SKILL.md` "critical pitfalls" #7 ("the same underlying bug can live
in more than one place").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: read the newly-landed `serialize_primitive_with_sq_vr_is_error` test and its accompanying `VR::SQ => { return Err(serde::ser::Error::custom(...)) }` fix in `json/src/ser/mod.rs`, then grepped the same crate for other `panic!`/`unreachable!` sites reachable from the same "VR determines which serializer wrapper is used, but nothing enforces the wrapper matches the stored `PrimitiveValue` variant" pattern | Static code reading, no build needed | Found four more panic sites in `json/src/ser/value.rs`'s `AsNumbers::serialize` (the wrapper used for `VR::DS`/`FL`/`FD`/`IS`/`SL`/`SS`/`SV`/`UL`/`US`/`UV`), for `PrimitiveValue::Date`/`DateTime`/`Time`/`Tags` - reachable the same way the just-fixed `SQ` case was: a `DataElement` constructed directly via `put_element`/`from_element_iter` with a VR that doesn't match its stored value's actual variant, or via the documented `AttributeAction::SetVr` (changes VR without touching the stored value, per candidate `025` from the prior session) |
| Hand-built input: a `DataElement` tagged `VR::DS` (dispatches to `AsNumbers`) whose stored value is `PrimitiveValue::Date`, debug build, `cargo test` (temporary unit test added to `json/src/ser/mod.rs`, reverted before this record was written) | `dicom_json::to_string(&obj)` via the public API | Panics: `wrong impl: cannot encode Date as numbers` at `json/src/ser/value.rs:64`. Reproduced 3/3 in-tree unit-test runs, then 2/2 more via the standalone `repro/` built against a fresh clone of pristine upstream `df2b9e42` (5/5 total) |
| Second boundary input in the same class (`VR::US` with a stored `PrimitiveValue::Tags` value - a different `AsNumbers`-dispatched VR and a different non-numeric variant) | Same technique, temporary unit test | Also panics before the fix (`wrong impl: cannot encode Tags as numbers`), also returns a graceful `Err` after - confirms the fix isn't narrowly tailored to just the `Date` case |

## Status

**Confirmed** - deterministic, pure-logic bug with no layout/seed/timing dependency; reproduces
5/5 runs across two distinct triggering inputs (`Date`+`VR::DS`, `Tags`+`VR::US`) - 3 in-tree
unit-test runs of the primary input + 2 standalone `repro/` runs against a fresh clone of pristine
upstream `df2b9e42`, plus the second boundary input confirmed independently panicking before the
fix and erroring gracefully after.

## Class

`crash` (a `panic!()` reachable from the public `dicom_json::to_string`/`to_value`/etc.
serialization API on an object that is fully constructible via this crate's own public,
documented API surface - no `unsafe`, no memory-safety implication).

## Root cause

Same root cause already documented in `projects/dicom-rs.md` from the prior session ("A
`DataElement`'s VR and its stored `PrimitiveValue`/`Value` variant are never kept in sync by the
type system"), now confirmed present a third and fourth time (after `025`/`026`), on the DICOM+JSON
*serialization* side rather than the wire encoder. `json/src/ser/mod.rs`'s
`DicomJson<&InMemElement<D>>::serialize` dispatches which value-serializing wrapper to use
(`AsStrings`, `AsPersonNames`, `AsNumbers`, `AsAttributeTags`, `InlineBinary`) purely from the
element's declared `vr()`, under the documented assumption (stated in each wrapper's own doc
comment, e.g. "Should be used for the value representations DS, FL, FD, ...") that the *caller*
guarantees the stored value actually matches one of the VRs the wrapper is meant for. Nothing
enforces that guarantee - `AttributeAction::SetVr` changes only the VR field, and direct
construction via `put_element`/`from_element_iter` never validates VR-vs-value consistency at all.
`AsNumbers` was written to assume its input is always numeric-or-stringly, and `panic!()`s for the
four variants it doesn't expect (`Date`/`DateTime`/`Time`/`Tags`) rather than treating a
mismatch as the same kind of "inconsistent state" the `VR::SQ` case (fixed one commit earlier in
this same file's `mod.rs`) already learned to handle gracefully.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed all four `panic!()` calls in `AsNumbers::serialize` to `return Err(serde::ser::Error::
custom(...))`, following the exact pattern the immediately-preceding commit (`df2b9e42`, PR #821/
#822) already established for the `VR::SQ` case in the same file's `mod.rs`:

```
-            PrimitiveValue::Date(_) => panic!("wrong impl: cannot encode Date as numbers"),
-            PrimitiveValue::DateTime(_) => panic!("wrong impl: cannot encode DateTime as numbers"),
-            PrimitiveValue::Time(_) => panic!("wrong impl: cannot encode Time as numbers"),
-            PrimitiveValue::Tags(_) => panic!("wrong impl: cannot encode Tags as numbers"),
+            PrimitiveValue::Date(_) => {
+                return Err(serde::ser::Error::custom("cannot encode Date as numbers"));
+            }
+            PrimitiveValue::DateTime(_) => {
+                return Err(serde::ser::Error::custom("cannot encode DateTime as numbers"));
+            }
+            PrimitiveValue::Time(_) => {
+                return Err(serde::ser::Error::custom("cannot encode Time as numbers"));
+            }
+            PrimitiveValue::Tags(_) => {
+                return Err(serde::ser::Error::custom("cannot encode Tags as numbers"));
+            }
```

Verified against two inputs in the same class (CONFIRMATION.md section 7): both `Date`+`VR::DS`
and `Tags`+`VR::US` now return a proper `serde_json` `Err` instead of panicking.

- Project test suite (`cargo test -p dicom-json --lib`): **12 passed, 2 failed (both this
  candidate's own temporary repro tests, reverted before writing this record) -> 14 passed, 0
  failed** after the fix, with both temporary tests included. No other test in the `dicom-json`
  crate's existing suite (including the just-landed `serialize_primitive_with_sq_vr_is_error` test
  from the prior commit) was affected.

## What's not done / open questions

- `AsStrings`, `AsPersonNames`, and `AsAttributeTags` (the other value-serializing wrappers
  dispatched by the same `vr()`-keyed match) were read and appear to use `.to_multi_str()` /
  similar generic, non-panicking conversions for whatever `PrimitiveValue` variant they receive -
  not independently stress-tested with every possible VR/value mismatch combination this session,
  only confirmed to not contain an obvious `panic!()`/`unreachable!()` by reading.
- The read/deserialize side of this exact concern (whether `json/src/de/mod.rs` can ever construct
  an `InMemDicomObject` with a VR/value mismatch that later trips one of these serializer panics
  when the object is round-tripped back to JSON) was not investigated - only the direct
  construction and `SetVr` paths were confirmed as reachable triggers.
- Per `SKILL.md`'s "critical pitfalls" #7, the *same* underlying "VR-vs-value drift" root cause is
  already known to also affect `parser/src/stateful/encode.rs` (candidate `025`, wire encoding) and
  `parser/src/dataset/mod.rs` (candidate `026`, token classification) from the prior session - this
  candidate is the third and fourth confirmed instance of the same class in this codebase, all in
  different modules. A dedicated future session sweeping every place that dispatches on `vr()`
  without also checking the stored value's actual variant (the technique that found `023`/`024`
  originally) could plausibly find more.
