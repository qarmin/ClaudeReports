# json-multivalue-string-middle-item-over-trimmed

`json/src/ser/value.rs`'s `AsStrings` - the JSON serializer type used for generic (non-numeric)
text VRs such as LO, SH, CS, PN's component, UI, etc. - serializes a multi-valued `PrimitiveValue::Strs`
via `PrimitiveValue::to_multi_str()`, which trims trailing space/NUL padding from **every** item
independently. DICOM only ever pads the **last** item of a joined multi-valued string (to make the
whole wire-encoded, backslash-joined representation even-length) - trimming every item
independently silently destroys legitimate content in any earlier item, including turning a
middle item that consists **entirely** of whitespace into an empty string.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Short, ad-hoc, throwaway JSON round-trip fuzz harness (built outside this workspace, in `/tmp`, discarded after use) generating arbitrary `InMemDicomObject`s (multi-valued strings, numbers, tags, dates, nested sequences) and round-tripping them through `dicom_json::to_string`/`from_str` | `cargo +nightly fuzz run` against the harness, ~80s per round, with candidates 001/002/008/048-054's already-confirmed patches stacked onto the tree first | Found 4 crashes total. 2 were harness bugs (fixed in the harness, not reported - see `backtrace.txt`'s "Provenance" section). 1 was a rediscovery of already-known, already-fixed candidate 011 (confirmed by re-testing with 011's patch applied, which made the crash disappear). The 4th - reproduced here - persisted after fixing the harness and applying every other already-known patch |
| Manual root-cause analysis of the persisting crash, tracing `AsStrings::serialize` -> `PrimitiveValue::to_multi_str()` | Code reading | Found `to_multi_str()`'s `Strs` arm trims `[' ', '\0']` from **every** item via `.trim_end_matches(...)`, added by candidate 008 earlier this session to fix a different bug (backslash re-splitting) - candidate 008's own fix, while correctly fixing its target bug, introduced this new one as an unintended side effect |
| First fix attempt: changing `to_multi_str()` itself to only trim the last item | Direct edit + `cargo test --workspace --lib` | Fixed this repro, but **broke** the existing, deliberate test `json::ser::value::tests::serialize_primitive_value_as_numbers`, which requires every item of e.g. `["5 ", "6 ", "-7"]` to have its trailing space trimmed so each substring parses as a number in `AsNumbers` (used for numeric VRs that may hold string-typed values). Reverted this approach once the test failure was understood - `to_multi_str()` is shared by both the plain-string (`AsStrings`) and numeric (`AsNumbers`) JSON serializers, and each needs different trimming semantics |
| Second fix attempt: change only `AsStrings::serialize` to stop using the shared `to_multi_str()` and trim just the last item directly, leaving `to_multi_str()`/`AsNumbers` untouched | Direct edit + targeted tests + `cargo test --workspace --lib` | Fixed the repro without touching `to_multi_str()` at all; all pre-existing tests (including `serialize_primitive_value_as_numbers`) continued to pass. Reproduced deterministically: in-tree checks during investigation + 2 independent `just test` runs against a fresh clone of pristine upstream `df2b9e42` (see `backtrace.txt`) |

## Status

**Confirmed**, with a bounded fix applied and verified. Deterministic - reproduces on every run,
for any multi-valued text-VR element (serialized via `AsStrings`, i.e. any VR other than the
numeric-capable ones routed through `AsNumbers`) where a non-last item ends in a space or NUL
character, including the case where the item consists entirely of such characters.

## Class

`logic` (silent data loss during JSON serialization - a legitimate value is turned into a
different, shorter one with no error raised). A direct, previously-unknown follow-up defect in
candidate 008's own fix from earlier in this session - confirmed independent of every other
candidate applied this session by testing this repro against a pristine tree with no other patches
present.

## Root cause

`core/src/value/primitive.rs`'s `PrimitiveValue::to_multi_str()` (as fixed by candidate 008)
trims `[' ', '\u{0}']` from every item of a `Strs` value independently:

```rust
PrimitiveValue::Strs(values) => Cow::Owned(
    values
        .iter()
        .map(|s| s.trim_end_matches([' ', '\u{0}']).to_string())
        .collect(),
),
```

This function is shared by two JSON serializer types with different, incompatible needs:

- `AsStrings` (`json/src/ser/value.rs`) uses it to produce the plain string array for text VRs.
  For this purpose, trimming every item is wrong: DICOM's wire format only ever pads the whole
  joined multi-value string once, at the very end, to reach an even total length - individual,
  non-final items are delimited by explicit `\` separators and their exact content (including any
  literal trailing whitespace) is not itself DICOM padding and must be preserved.
- `AsNumbers` (same file) also uses it, for numeric VRs (IS/DS) that may hold string-typed values
  needing to be parsed into JSON numbers. Here, trimming every item **is** necessary: a
  space-padded numeric substring like `"6 "` must have its trailing space removed before it can be
  parsed as the number `6` - and unlike `AsStrings`, this consumer doesn't need to preserve the
  original text verbatim, only to extract a number from it.

Because both consumers shared the same helper, fixing `to_multi_str()` for one need necessarily
broke the other - `AsStrings` needed "trim only the last item" while `AsNumbers` needed "trim every
item," and no single trimming rule inside `to_multi_str()` alone can satisfy both.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed only `AsStrings::serialize` to stop calling `to_multi_str()` for the `Strs` case, matching
each item directly and trimming trailing space/NUL only on the last one:

```rust
impl Serialize for AsStrings<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            PrimitiveValue::Strs(values) => {
                let last = values.len().saturating_sub(1);
                serializer.collect_seq(values.iter().enumerate().map(|(i, s)| {
                    if i == last {
                        s.trim_end_matches([' ', '\u{0}'])
                    } else {
                        s.as_str()
                    }
                }))
            }
            _ => serializer.collect_seq(&*self.0.to_multi_str()),
        }
    }
}
```

`to_multi_str()` itself, and `AsNumbers`'s own use of it, are left completely unchanged - the fix
is scoped exactly to the one consumer that had the wrong trimming semantics.

See `patch.diff` for the exact diff (fix only; the accompanying unit test added during
investigation - `serialize_primitive_value_as_strings_only_trims_last_item` - is not included,
matching this session's convention of keeping `patch.diff` minimal).

- `cargo test -p dicom-json --lib`: baseline (test present, fix removed) - the new test
  **FAILED**, showing the middle item `" "` turned into `""` (exactly the predicted corruption)
  -> with the fix restored - **13 passed, 0 failed** (including the pre-existing
  `serialize_primitive_value_as_numbers` and `serialize_primitive_value_as_strings` tests, both
  still passing unmodified).
- Full workspace (`cargo test --workspace --lib`, all 12 crates with a `#[lib]` target):
  **351 passed / 0 failed / 2 ignored** (pristine, no candidate changes) -> **352 passed / 0
  failed / 2 ignored** with this fix + its 1 test applied - zero regressions anywhere in the
  existing suite.

## What's not done / open questions

- `AsPersonNames` (same file, used for VR PN) also calls `to_multi_str()` directly
  (`let strings = self.0.to_multi_str();`) - PN values are typically single-valued (VM=1) in
  practice, in which case "trim every item" and "trim only the last item" are equivalent (there's
  only one item), so this was not considered a live instance of the same bug and was left
  unmodified. Whether a multi-valued PN element (VM>1, permitted by the standard for some
  attributes) could exhibit the same defect through this path was not specifically tested this
  session.
- The two harness-only false positives (see `backtrace.txt`) were not written up as their own
  candidates, per this session's established practice of only recording genuine, verified library
  defects - not exploratory-tooling mistakes or rediscoveries of already-documented bugs.
