# FileMetaTable's generic attribute API panics reading absent Implementation Version Name

Found during a fresh discovery round (no seeded lead): after finding a stack-overflow class of
bug in `022` by asking "does resource usage scale with untrusted *structure*", I went back to
asking "does an `.unwrap()` on an `Option` populated straight from an optional DICOM attribute
ever get called without checking `is_some()` first" - the same class of question that produced
`009`/`010`/`021`, but applied to a part of the codebase not yet swept: `object/src/meta.rs`'s
`impl DicomObject for FileMetaTable` (the generic attribute-access trait implementation for the
file meta group, used transitively by `FileDicomObject`'s own `DicomObject` impl for any tag in
group `0002`).

`FileMetaTable::attr_opt` first classifies a requested tag as either present-unconditionally
("mandatory") or present-only-if-its-`Option`-field-`is_some()` ("optional"), then, if classified
as present, hands back a `FileMetaAttribute` that the caller can later call
`.to_primitive_value()`/`.to_str()`/etc. on. _Implementation Version Name_ (0002,0013) is Type 3
(optional) per PS3.10, and is modeled correctly as `Option<String>` in the `FileMetaTable` struct,
and `to_str()`'s handling of this tag correctly treats it as optional
(`.as_deref().map(Cow::Borrowed).unwrap_or_default()`) - but `attr_opt`'s own classification
mistakenly lumps `IMPLEMENTATION_VERSION_NAME` in with the *mandatory* group (unconditional
`true`, no `.is_some()` guard), while `to_primitive_value()`'s handling of the same tag does
`self.meta.implementation_version_name.clone().unwrap()` - assuming it's always present. The
result: for any `FileMetaTable` that has an Implementation Class UID but no Implementation
Version Name (an entirely ordinary, spec-legal combination - the DICOM standard makes this
attribute optional independently of Implementation Class UID, and this exact combination is
common in real-world files from implementations that skip the optional field), the standard
generic accessor sequence `.attr(tag)` -> `.to_primitive_value()` panics.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `object/src/meta.rs`'s `FileMetaTable::attr_opt` (tag classification match) cross-referenced against `<FileMetaAttribute as DicomAttribute>::to_primitive_value` (which `.unwrap()`s six `Option`-typed fields) and `<FileMetaAttribute as DicomAttribute>::to_str` (which handles the same six fields gracefully via `.unwrap_or_default()`). Found that five of the six optional fields (`source_application_entity_title`, `sending_application_entity_title`, `receiving_application_entity_title`, `private_information_creator_uid`, `private_information`) are correctly gated behind `.is_some()` in `attr_opt`'s match, but `implementation_version_name` is not - it is grouped with the genuinely mandatory attributes. | Identified the mismatch: `to_primitive_value()`'s `.unwrap()` for this one field is unreachable-by-design for the other five (since `attr_opt` filters them out when absent), but is reachable for `IMPLEMENTATION_VERSION_NAME`. |
| Standalone repro (release build, public API: `FileMetaTableBuilder`, `FileDicomObject`, `DicomObject`/`DicomAttribute` traits) | `___BUGHUNT___/023.../repro`. Built a `FileMetaTable` via the builder with `media_storage_sop_class_uid`, `media_storage_sop_instance_uid`, `transfer_syntax`, and `implementation_class_uid` all explicitly set (which avoids the builder's own fallback that would otherwise auto-populate `implementation_version_name` when *both* Implementation Class UID and Version Name are left unset), but deliberately never calling `.implementation_version_name(...)`. Wrapped it in a `FileDicomObject`, then called `file.attr(tags::IMPLEMENTATION_VERSION_NAME)?.to_primitive_value()` - the ordinary documented way to fetch an attribute generically by tag. | Reproduced deterministically: panicked with `called \`Option::unwrap()\` on a \`None\` value` at `object/src/meta.rs:1015:84`, inside `<FileMetaAttribute as DicomAttribute>::to_primitive_value`. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically (fully deterministic - no randomness or timing involved). |
| Regression check (attribute present) | Same repro shape, but with `.implementation_version_name("MY_IMPL_1")` set on the builder | `attr()` + `to_primitive_value()` succeed and return the expected `Str("MY_IMPL_1 ")` value, confirming the normal (attribute-present) case was never broken and the fix doesn't change it. |
| Reachability check | `FileDicomObject<InMemDicomObject>::attr_opt` (`object/src/lib.rs`) tries `self.meta.attr_opt(tag)` first, falling back to the data set proper only if the file meta group doesn't claim the tag - so this is reachable on the standard, most commonly used `FileDicomObject` returned by `dicom_object::open_file`, for any file lacking this one optional attribute, via the crate's own documented generic-attribute-access pattern (`file.attr_opt(tag)?`/`file.attr(tag)?`, as shown in this crate's own top-level doc example). | N/A (confirms broad reachability: this is not an obscure internal-only code path). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build.

## Class

`crash` (unhandled `Option::unwrap()` panic, reachable through the crate's own documented generic
attribute-access API on an entirely ordinary, spec-compliant DICOM file that merely omits one
Type-3/optional file meta attribute).

## Root cause

`FileMetaTable::attr_opt` (`object/src/meta.rs`) misclassifies `IMPLEMENTATION_VERSION_NAME` as
unconditionally present ("mandatory") instead of conditionally present on
`self.implementation_version_name.is_some()` (as done correctly for five sibling optional
attributes), while `<FileMetaAttribute as DicomAttribute>::to_primitive_value` unconditionally
`.unwrap()`s the same `Option` field. The mismatch between the two functions is what turns an
ordinary absent-optional-attribute situation into a panic.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Moved `tags::IMPLEMENTATION_VERSION_NAME` out of the unconditional "mandatory attributes" match
arm and into its own guarded arm (`if self.implementation_version_name.is_some()`), matching the
pattern already used for the other five optional fields. See `patch.diff`
(`object/src/meta.rs`, one match arm).

Verified:
- `repro/` re-run against the patched tree no longer panics: `attr()` now correctly reports the
  attribute as absent (`No such data element with tag (0002,0013)`), matching the behavior any
  other absent optional attribute already has.
- Regression check: when the attribute *is* present, `attr()` + `to_primitive_value()` still
  succeed and return the correct value, unaffected by the fix.
- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, identical
  both before and after the patch (no regressions; no existing test exercised
  `attr()`/`attr_opt()` on this specific tag while it was absent).

## What's not done / open questions

- Did not check whether any other `DicomObject`/`DicomAttribute` trait implementations in this
  codebase (e.g. any future custom `DicomObject` impls outside this crate) have a similar
  present/mandatory classification mismatch - this was found by direct code reading of the one
  concrete implementation in `object/src/meta.rs`, not through a systematic macro/derive-level
  audit.
- Not fuzzed - found by cross-referencing two hand-written match statements
  (`attr_opt`'s classification vs. `to_primitive_value`'s field access) over the same six optional
  fields and noticing the one inconsistency, the same "diff structurally-parallel sibling
  branches" technique that has repeatedly paid off this session.
