# json-value-inline-binary-conflict-check-wrong-var

`json/src/de/mod.rs`'s `DataElementVisitor::visit_map` (the DICOM+JSON data-element deserializer,
reachable from the public `dicom_json::from_str`/`from_reader`/`from_slice`/`from_value`) has a
final `match (values, inline_binary) { ... _ => unreachable!() }` with no arm for the case where
both are `Some`. This was on the project's own `projects/dicom-rs.md` leftover list of
`unreachable!()` sites not yet checked from the prior session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning ("state-machine invariant that isn't checked, only assumed" per HEURISTICS.md) | Read the field-conflict guards for the `"Value"`/`"InlineBinary"`/`"BulkDataURI"` JSON keys | Found the conflict guards for `"InlineBinary"` and `"BulkDataURI"` check `values.is_some()` (the *plural*, deserialized-`PrimitiveValue` variable, only ever assigned *after* the whole `while let Some(key) = map.next_key()` loop finishes) instead of `value.is_some()` (the *singular* raw-JSON variable actually set by the `"Value"` key, mid-loop) - so `values` is always `None` at the time these guards run, and the guard can never fire |
| Hand-built boundary input, debug build, `cargo test` (temporary unit test added to `json/src/de/mod.rs`, reverted before this record was written) | `{"00100010":{"vr":"SH","Value":["abc"],"InlineBinary":"AAAA"}}` via `dicom_json::from_str::<InMemDicomObject>` | Panics: `internal error: entered unreachable code` at `json/src/de/mod.rs:365:18`. Reproduced 3/3 runs in the in-tree unit test, then 2/2 more via the standalone `repro/` built against a fresh clone of pristine upstream `df2b9e42` (5/5 total) |
| Same technique, second input in the same class | `{"00100010":{"vr":"SH","Value":["abc"],"BulkDataURI":"http://x/y"}}` (the `"BulkDataURI"` guard has the identical `values.is_some()` vs `value.is_some()` mistake) | Confirms the same root cause a second way: with the unfixed code this does **not** panic (the final match only inspects `values`/`inline_binary`, not `bulk_data_uri`), but it also does not error - a spec-violating "Value" + "BulkDataURI" combination is silently accepted, using the `"Value"` data and quietly carrying `bulk_data_uri` along unused. With the fix applied, this now correctly returns `Err("\"BulkDataURI\" conflicts with \"Value\"")`, same as `"InlineBinary"`. |

## Status

**Confirmed** - deterministic, pure-logic bug with no layout/seed/timing dependency; reproduces
5/5 runs (3 in-tree unit-test runs + 2 standalone `repro/` runs against a fresh clone of pristine
upstream `df2b9e42`), same panic location and message every time. Debug and release builds both
show it identically (this is ordinary control flow, not something instrumentation-only reveals).

## Class

`crash` (a `panic!`/`unreachable!()` reachable from ordinary use of a public deserialization
entry point on attacker/user-supplied JSON text - no `unsafe`, no memory-safety implication).

## Root cause

`visit_map` tracks two separate variables for the same JSON `"Value"` field across the loop:
`value: Option<serde_json::Value>` (the raw JSON payload, set when the `"Value"` key is seen) and
`values: Option<Value<...>>` (the fully-deserialized `PrimitiveValue`/`Value::Sequence`, computed
*after* the loop from `value` based on the element's declared VR - see lines 214-352). The
`"InlineBinary"` and `"BulkDataURI"` branches' conflict guards were written to check `values`
(plural) instead of `value` (singular) - almost certainly a copy/paste of the `"Value"` branch's
own guards (which correctly check `inline_binary`/`bulk_data_uri`, the siblings that *are* set
mid-loop) without swapping in the right variable name for the field actually being checked against.
Because `values` never becomes `Some` until after the loop ends, these two guards are permanently
dead code - a JSON object that supplies `"Value"` first and then `"InlineBinary"` (or
`"BulkDataURI"`) sails through both guards unchallenged. After the loop, `value` (`Some`) feeds
into `values` (`Some`) via the VR-dependent conversion, while `inline_binary` (also `Some`) was
never cleared - so the closing `match (values, inline_binary)` lands on `(Some(_), Some(_))`, an
arm nobody wrote, and hits `unreachable!()`.

The guard ordering in the *other* direction (`"InlineBinary"` first, then `"Value"`) is not
affected: the `"Value"` branch's own guards correctly check `inline_binary.is_some()`/
`bulk_data_uri.is_some()`, which *are* set mid-loop, so that ordering already errors out
correctly. The bug is one-directional, keyed on which of the two conflicting JSON keys appears
first in the object.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed both guards from `values.is_some()` to `value.is_some()`, matching what they were
actually meant to check (the same singular `value` variable the `"Value"` branch's own guards
already reference correctly):

```
-                "InlineBinary" => {
-                    if values.is_some() {
+                "InlineBinary" => {
+                    if value.is_some() {
@@
-                "BulkDataURI" => {
-                    if values.is_some() {
+                "BulkDataURI" => {
+                    if value.is_some() {
```

Verified against two inputs in the same class (CONFIRMATION.md section 7): both the original
`"Value"`+`"InlineBinary"` ordering and the previously-silently-accepted `"Value"`+`"BulkDataURI"`
ordering now return a proper `serde_json` deserialization `Err` (`"\"InlineBinary\" conflicts with
\"Value\""` / `"\"BulkDataURI\" conflicts with \"Value\""`) instead of panicking or silently
succeeding. Re-ran the standalone `repro/` against the patched clone (via `just test`'s
`apply`/`reproduce-after` steps) - exit code 0, no panic, the expected `Err(...)` printed.

- Project test suite (`cargo test -p dicom-json --lib`): **12 passed, 2 failed (both this
  candidate's own temporary repro tests, reverted before writing this record) -> 14 passed, 0
  failed** after the fix, with the temporary tests included. The full `dicom-json` crate test
  suite (including doctests) passes cleanly on the patched tree: 14 lib tests + 5 doctests, all
  passing.

## What's not done / open questions

- Only the `"Value"`-vs-`"InlineBinary"`/`"BulkDataURI"` ordering was checked; the guard on the
  `"Value"` branch itself (checking `inline_binary`/`bulk_data_uri`) was not modified and was
  already correct - not re-derived from scratch, just confirmed by reading.
- This is the same "parallel checks on the same closed set of fields drift out of sync" root-cause
  shape noted in `projects/dicom-rs.md` for the `FileMetaAttribute` bugs (023/024) from the prior
  session, just in JSON deserialization rather than file-meta attribute matching - worth keeping
  as a general pattern to watch for in this codebase (any spot with more than one guard/branch
  referencing "the other" field of a small mutually-exclusive set).
- Not checked: whether `dicom_json::from_reader`/`from_slice` (which share the same deserializer
  code path via `DicomJson<T>: Deserialize`) have any other independent entry point that could
  reach this differently - unlikely, since they all funnel through the same `serde_json::from_*`
  call into the same `Deserialize` impl, but not explicitly exercised here beyond `from_str`.
