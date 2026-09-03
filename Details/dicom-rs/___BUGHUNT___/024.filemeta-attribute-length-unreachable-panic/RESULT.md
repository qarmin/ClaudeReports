# FileMetaAttribute::length() panics on the mandatory File Meta Information Version attribute (and two other tags)

Found immediately after `023`, while still looking at `object/src/meta.rs`'s `FileMetaAttribute`
trait implementations for further "sibling match arms that got out of sync" issues (the same
technique that found `023`). `023` fixed a mismatch between `attr_opt`'s presence classification
and `to_primitive_value`'s field access. Checking the sibling `<FileMetaAttribute as
HasLength>::length` match (also driven off `Tag(0x0002, self.tag_e)`) turned up a *different*,
unrelated defect in the same style of match statement: three tags that `attr_opt` (correctly,
after `023`'s fix) considers present - `FILE_META_INFORMATION_VERSION` (unconditionally, since
it's genuinely mandatory), `RECEIVING_APPLICATION_ENTITY_TITLE`, and `PRIVATE_INFORMATION` (both
correctly gated on `.is_some()`) - have **no corresponding arm at all** in `length()`'s match, so
they fall through to `_ => unreachable!()` and panic. This is worse than `023`'s bug:
`FILE_META_INFORMATION_VERSION` is a *mandatory* attribute, present in every single valid DICOM
file meta group, so calling `.length()` (or the default `HasLength::is_empty()`, which calls
`.length()` internally) on this attribute panics unconditionally, on any file.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `object/src/meta.rs`'s `<FileMetaAttribute as HasLength>::length`, diffed arm-by-arm against `<FileMetaAttribute as DicomAttribute>::to_primitive_value` and `attr_opt`'s classification (both already read closely for `023`). Found `FILE_META_INFORMATION_VERSION`, `RECEIVING_APPLICATION_ENTITY_TITLE`, and `PRIVATE_INFORMATION` present in the latter two but entirely absent from `length()`'s match. | Identified three tags that fall through to `length()`'s `unreachable!()` arm despite `attr_opt` legitimately returning `Some(...)` for them. |
| Standalone repro (release build, public API: `FileMetaTableBuilder`, `FileDicomObject`, `DicomObject`/`HasLength` traits) | `___BUGHUNT___/024.../repro`. Built an entirely ordinary `FileMetaTable` via the builder (only the standard mandatory fields set), wrapped it in a `FileDicomObject`, then called `file.attr(tags::FILE_META_INFORMATION_VERSION)?.length()` - the natural thing to do with any attribute handle. | Reproduced deterministically: panicked with `internal error: entered unreachable code` at `object/src/meta.rs:931:18`, inside `<FileMetaAttribute as HasLength>::length`. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically (fully deterministic). |
| Additional check (not part of the committed repro, done with a throwaway second binary) | Populated `receiving_application_entity_title` and `private_information` on the `FileMetaTable` directly, then called `.attr(...)?.length()` for each | Both panicked identically on the pristine tree (same `unreachable!()` arm), and both returned the correct length after the fix (`Length(7)` and `Length(5)` respectively, matching the input data). |
| Reachability check | `HasLength::is_empty()` (`core/src/header.rs`) has a default implementation `self.length() == Length(0)` - any generic code that checks `attr.is_empty()` before reading a file meta attribute's value (a common idiom used throughout this codebase, e.g. in `pixeldata/src/attribute.rs`'s accessors) would also panic transitively through this bug. | N/A (confirms `.length()` is not a rarely-called method - it backs the crate's own default `is_empty()` idiom). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build.

## Class

`crash` (unhandled `unreachable!()` panic reachable via ordinary use of the `HasLength` trait on
an attribute that `attr_opt` itself reports as present - notably including a *mandatory*
attribute present in every valid DICOM file).

## Root cause

`<FileMetaAttribute as HasLength>::length` (`object/src/meta.rs`) is missing match arms for three
tags that other methods on the same type (`attr_opt`, `to_primitive_value`, `to_str`) correctly
handle: `FILE_META_INFORMATION_VERSION`, `RECEIVING_APPLICATION_ENTITY_TITLE`, and
`PRIVATE_INFORMATION`. These three tags simply were not carried over when this match was written
or last extended, and fall through to the catch-all `unreachable!()` arm meant only for tags that
`attr_opt` would never hand out an attribute for in the first place.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added the three missing arms to `length()`'s match: `FILE_META_INFORMATION_VERSION` returns the
fixed 2-byte length of `information_version: [u8; 2]`; `RECEIVING_APPLICATION_ENTITY_TITLE` and
`PRIVATE_INFORMATION` follow the same `.as_ref().map(|x| x.len() as u32).unwrap_or(0)` pattern
already used for their sibling optional fields. See `patch.diff` (`object/src/meta.rs`, one match
statement, three new arms).

Verified:
- `repro/` re-run against the patched tree no longer panics: `length()` now returns `Length(2)`
  for File Meta Information Version, matching its actual 2-byte encoded size.
- Additional check: `RECEIVING_APPLICATION_ENTITY_TITLE` and `PRIVATE_INFORMATION` also return the
  correct lengths after the fix, when present.
- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, identical
  both before and after the patch (no regressions; no existing test called `.length()` on any of
  these three attributes).

## What's not done / open questions

- Did check the two remaining `Tag(0x0002, self.tag_e)`-keyed matches in this file after fixing
  this one: `value_type()` covers all 12 recognized tags with no gaps, and `cardinality()` uses a
  safe `_ => 1` fallback (never panics) rather than `unreachable!()`, so neither has a matching
  defect. Between `023` and this candidate, all five parallel matches in `FileMetaAttribute`'s
  trait impls (`attr_opt`, `to_primitive_value`, `to_str`, `length`, `value_type`; `cardinality`
  is panic-safe by construction) have now been cross-checked against each other.
- Not fuzzed - found by the same "diff structurally-parallel match arms" technique as `023`,
  applied to a sibling method on the same type immediately afterward.
