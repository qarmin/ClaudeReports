# filemeta-sop-uid-inference-unpadded

`object/src/mem.rs`'s file meta table enrichment step - which, when Media Storage SOP Class UID
or Media Storage SOP Instance UID is missing from the file meta group, infers them from the main
data set's own SOP Class UID / SOP Instance UID elements - stores the inferred value **without
re-padding it to even length**, even though every other way of setting these fields (the builder
setters `FileMetaTableBuilder::media_storage_sop_class_uid`/`..._instance_uid`, via `ui_padded()`)
always does. The same defect, with the same underlying cause, also exists in
`object/src/meta.rs`'s `FileMetaTableBuilder::build()`: when Implementation Class UID is missing,
it falls back to this crate's own hardcoded `IMPLEMENTATION_CLASS_UID`/`IMPLEMENTATION_VERSION_NAME`
constants directly, also without padding - and `IMPLEMENTATION_VERSION_NAME` ("DICOM-rs 0.10.0")
is itself 15 characters, already odd-length on its own. Since DICOM requires every value to have
even length, a field stored this way is invalid the moment it's set; the crate's own writer must
pad it when serializing, and since this parser (unlike the main data set) always stores whatever
raw bytes it reads back verbatim with no trimming, that added padding becomes real, permanent
content on the very next read - so the field never compares equal to itself across a write/re-read
round trip.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Stacked-fuzzing technique (as used for candidates 049-052): apply candidates 001, 002, 008, 048, 049, 050, 051, 052's already-confirmed patches to the real tree, then re-run the existing `object/fuzz/fuzz_targets/open_file.rs` | `ASAN_OPTIONS=symbolize=0 cargo +nightly fuzz run open_file -- -max_total_time=150` (150s) | A new crash: a parse/write/re-parse/compare round trip reported `NOT EQUAL` |
| `cargo +nightly fuzz tmin` to minimize the crashing input | Default tmin settings | 367-byte minimized input, could not be reduced further by tmin |
| Standalone diagnostic (dedicated `dicom-object` path-dep binary, with temporary `eprintln!` tracing added directly to `object/src/meta.rs`'s parsing loop and `read_str_body`) parsing, writing, and re-parsing the minimized input field-by-field | debug build, with all 8 patches above applied | Traced the discrepancy precisely to `meta.media_storage_sop_class_uid`: the fuzzed file's own (0002,0002) element was never actually present at that tag in the real byte stream (an earlier malformed/off-group tag had already desynced the parser's position), so `FileMetaTable::build()` correctly left the field empty via its own "missing -> empty string" fallback; `mem.rs`'s "infer from the data set's own SOPClassUID" enrichment step then ran and set it from the main data set's (0008,0016) value ("1.2.840.10008.5.\0...2.1\0", 28 bytes/chars) via `Value::to_str()`, which trims the trailing pad character - producing an odd-length (27-char) string stored as-is, with no re-padding |
| Confirming which of the stacked patches were actually relevant, by testing with each combination | Re-ran the diagnostic with only subsets of the 8 patches applied | Confirmed this defect is fully independent of all 8 stacked patches - none of them touch this code path. A leftover hypothesis from earlier in this same investigation (that file-meta string reads should reject odd *wire-declared* lengths outright) was tested and found **not** to be the actual mechanism for this crash - the wire-declared lengths for every meta element in this input were already even; the corruption is introduced entirely by the inference fallback's un-padded `to_str()` copy. That hypothesis's speculative code changes were reverted before finalizing this candidate, keeping the fix scoped to the actual, verified root cause |
| Independent, minimal, hand-built reproduction: a file meta group with only the mandatory Transfer Syntax UID (Media Storage SOP Class UID deliberately omitted), plus a data set containing just SOPClassUID = `"1.2.3.4\0"` (8 bytes, already correctly padded) | Constructed directly via the public API (`dicom_object::from_reader`), no fuzzer involved | Confirmed the same defect independently: the inferred `media_storage_sop_class_uid` came out as `"1.2.3.4"` (7 chars, odd), and a write/re-read round trip produced `"1.2.3.4\0"` (8 chars) - not equal to the first parse's value. Reproduced deterministically: in-tree checks during investigation + 2 independent `just test` runs against a fresh clone of pristine upstream `df2b9e42` (see `backtrace.txt`) |

## Status

**Confirmed**, with a bounded fix applied and verified. Deterministic - reproduces on every run,
for:
1. Any file missing Media Storage SOP Class UID or SOP Instance UID in its file meta group, whose
   corresponding main data set SOP Class/Instance UID value happens to already be even-length
   (the ordinary, correct encoding) - the inference fallback still breaks it by trimming and not
   re-padding.
2. Any file missing Implementation Class UID in its file meta group at all - this affects
   `IMPLEMENTATION_VERSION_NAME`, this crate's own hardcoded default, unconditionally, regardless
   of the input file's own content.

## Class

`logic` (silent, one-way corruption: an odd-length value is stored where DICOM permits only even
lengths, without any error, and only becomes visible as a mismatch on a subsequent round trip).
Distinct from every previously-stacked candidate this session (001, 002, 008, 048-052) - confirmed
independent by testing with those patches both present and absent, with identical results either
way.

## Root cause

Two call sites share the identical defect: they bypass the padding normalization that every other
assignment path for these fields already applies (`ui_padded`/`txt_padded`, used by the
`FileMetaTableBuilder`'s own setters), assigning a possibly-odd-length string directly to the
`FileMetaTable` struct field instead.

1. `object/src/mem.rs` (`FileDicomObject::from_reader_with_all_options`), when Media Storage SOP
   Class/Instance UID is empty after parsing the file meta group:
   ```rust
   meta.media_storage_sop_class_uid = elem.value().to_str().context(ParseSopAttributeSnafu)?.to_string();
   ```
   `PrimitiveValue::to_str()` trims a `Strs` value's trailing pad character (`' '`/`'\0'`) when
   there's exactly one item - by design, for display/comparison purposes - but this call site then
   stores that *trimmed* value directly into a field that is expected to always be wire-ready
   (even-length), with no re-padding step afterward.

2. `object/src/meta.rs` (`FileMetaTableBuilder::build()`), when Implementation Class UID is
   missing:
   ```rust
   let implementation_class_uid = self.implementation_class_uid.unwrap_or_else(|| {
       implementation_version_name = Some(IMPLEMENTATION_VERSION_NAME.to_string());
       IMPLEMENTATION_CLASS_UID.to_string()
   });
   ```
   Both `IMPLEMENTATION_CLASS_UID` (44 chars, already even - harmless by coincidence) and
   `IMPLEMENTATION_VERSION_NAME` (15 chars, **odd**) are crate-level constants (`object/src/lib.rs`)
   assigned directly with no padding.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Both call sites now go through `ui_padded`/`txt_padded` - the same padding functions the builder's
own setters already use for these exact fields, matching each field's established padding
character (`'\0'` for UID-like fields via `ui_padded`, `' '` for the version name via
`txt_padded`). `ui_padded` was made `pub(crate)` (it was private to `meta.rs`) so `mem.rs` could
reuse it instead of duplicating the logic:

```rust
// object/src/meta.rs
pub(crate) fn ui_padded<T>(s: T) -> String { ... }
```
```rust
// object/src/mem.rs
meta.media_storage_sop_class_uid =
    crate::meta::ui_padded(elem.value().to_str().context(ParseSopAttributeSnafu)?);
// ... and the identical change for media_storage_sop_instance_uid
```
```rust
// object/src/meta.rs, FileMetaTableBuilder::build()
let implementation_class_uid = self.implementation_class_uid.unwrap_or_else(|| {
    implementation_version_name = Some(txt_padded(IMPLEMENTATION_VERSION_NAME));
    ui_padded(IMPLEMENTATION_CLASS_UID)
});
```

See `patch.diff` for the exact diff (fix only; the two accompanying unit tests added during
investigation -
`mem::tests::inferred_media_storage_sop_class_uid_is_padded_and_stable` and
`meta::tests::build_default_implementation_class_uid_and_version_name_are_even_length` - are not
included, matching this session's convention of keeping `patch.diff` minimal).

- `cargo test -p dicom-object --lib`: baseline (both tests present, fix removed) - both **FAILED**
  with exactly the predicted odd lengths (`"1.2.3.4"` len 7; `"DICOM-rs 0.10.0"` len 15) -> with
  the fix restored - **77 passed, 0 failed**.
- Full workspace (`cargo test --workspace --lib`, all 12 crates with a `#[lib]` target):
  **351 passed / 0 failed / 2 ignored** (pristine, no candidate changes) -> **353 passed / 0
  failed / 2 ignored** with this fix + its 2 tests applied - zero regressions anywhere in the
  existing suite.

## What's not done / open questions

- The original 367-byte fuzz input that led to this candidate still doesn't round-trip to a fully
  `EQUAL` object even with this fix applied, because its `information_group_length` field differs
  between the first parse (236, taken verbatim from the malformed file's own declaration) and the
  second (264, freshly recomputed by the writer from the now-corrected field contents). This was
  investigated and is **not** considered a bug: `information_group_length` is never round-tripped
  byte-for-byte by design - it's always recomputed on write to match the actually-written content
  - so a self-inconsistent (fuzzer-generated) file that declares a group length not matching its
    own real content will never produce a matching group length after a round trip, regardless of
  any other fix. A well-formed input file (self-consistent group length) is unaffected.
- Other file meta fields inferred or defaulted elsewhere in this crate were not exhaustively
  audited for the same missing-padding pattern beyond the two sites fixed here; these were the
  only two call sites found to directly assign to a `FileMetaTable` string field without going
  through a padding function.
