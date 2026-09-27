# filemeta-group-length-stale-after-sop-uid-inference

`object/src/mem.rs`'s file meta enrichment step - which infers a missing Media Storage SOP
Class/Instance UID from the main data set's own SOP Class/Instance UID (see candidate 053) -
mutates `FileMetaTable` fields *after* `FileMetaTable::build()` has already computed and stored
`information_group_length` from whatever was actually present in the file's own meta group. Since
nothing recomputes the group length afterward, it stays stale (too short) whenever either
inference actually changes something. The writer then serializes this stale, too-short group
length verbatim - truncating the meta group at write time and spilling its last element(s) into
the main data set on the very next read.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Stacked-fuzzing technique (as used for candidates 049-053): apply candidates 001, 002, 008, 048-053's already-confirmed patches to the real tree, then re-run the existing `object/fuzz/fuzz_targets/open_file.rs` | `ASAN_OPTIONS=symbolize=0 cargo +nightly fuzz run open_file -- -max_total_time=150` (150s) | A new crash: a parse/write/re-parse/compare round trip reported `NOT EQUAL`, with `Tag(0x0002, 0x0013)` (Implementation Version Name - a file meta tag) appearing as an ordinary top-level `DataElement` in the *main data set* after the round trip |
| `cargo +nightly fuzz tmin` to minimize the crashing input | Default tmin settings | Minimized to 340 bytes |
| Standalone diagnostic (dedicated `dicom-core`/`dicom-object` path-dep binary) parsing, writing, and manually byte-walking the actually-written file meta group (ignoring the written file's own declared `information_group_length`, to see independently where each element really starts/ends) | debug build, with all 9 patches above applied | Found the writer declared `information_group_length = 134`, but a full byte-accurate walk of the meta elements it actually wrote (Information Version, Media Storage SOP Class UID, SOP Instance UID, Transfer Syntax, Implementation Class UID, Implementation Version Name) totaled 176 bytes - the declared value was 42 bytes short, exactly matching the byte contribution of the (non-empty, correctly-padded-by-053) inferred Media Storage SOP Class UID value |
| Code reading: traced every call site of `FileMetaTable::update_information_group_length()` across the crate | `grep -n update_information_group_length object/src/*.rs` | Confirmed it is called from `FileMetaTableBuilder::build()` (once, at construction time) and from `set_transfer_syntax()`/`apply()` (immediately after each of *those* functions mutates a meta field directly) - but **not** from `mem.rs`'s SOP UID inference step, the only other place in the crate that mutates a `FileMetaTable` field directly after it has already been built |
| Independent, minimal, hand-built reproduction: a file meta group with only the mandatory Transfer Syntax UID (Media Storage SOP Class UID deliberately omitted), plus a data set containing just SOPClassUID = `"1.2.3.4\0"` | Constructed directly via the public API (`dicom_object::from_reader`), no fuzzer involved. Tested against a **pristine** tree with *only* this candidate's fix applied (candidate 053's fix deliberately *not* applied), to confirm this defect and its fix are fully independent of 053 | Confirmed: `information_group_length` was `134` after the first parse but `142` after a write/re-read round trip - i.e., already wrong on the very first parse, before any round trip even happened. Reproduced deterministically: in-tree checks during investigation + 2 independent `just test` runs against a fresh clone of pristine upstream `df2b9e42` (see `backtrace.txt`) |

## Status

**Confirmed**, with a bounded fix applied and verified. Deterministic - reproduces on every run,
for any file missing Media Storage SOP Class UID or SOP Instance UID in its file meta group, whose
corresponding data set SOP Class/Instance UID element is non-empty (the ordinary case for any
real, well-formed DICOM file lacking those two specific meta fields - not just fuzzer-crafted
input).

## Class

`logic` (a stale cached value gets propagated to the wire, corrupting the boundary between the
file meta group and the main data set on the very next read - no crash, but silent structural
corruption). Distinct from, but discovered immediately after and closely related to, candidate 053
(which made the *inferred value itself* correct in isolation, but did not address this separate
group-length bookkeeping gap). Confirmed independent of 053 by testing with only this candidate's
patch applied to a pristine tree.

## Root cause

`object/src/meta.rs`'s `FileMetaTableBuilder::build()` computes `information_group_length` exactly
once, via `update_information_group_length()`, using whichever fields were populated purely from
parsing the file's own on-wire meta group. `object/src/mem.rs`'s
`FileDicomObject::from_reader_with_all_options` then runs its own post-processing step - after
`build()` has already returned a complete `FileMetaTable` - to infer Media Storage SOP
Class/Instance UID from the main data set when either is empty, mutating those two fields directly
on the already-built table. This mirrors two other places in the crate that mutate a
`FileMetaTable` field after construction - `set_transfer_syntax()` and the generic attribute
`apply()` - both of which already call `self.update_information_group_length()` immediately after
their own mutation, precisely to keep the cached group length in sync. The SOP UID inference step
was the one place that mutates fields directly without following this same, already-established
convention.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a single call to `meta.update_information_group_length()` right after both inference blocks
in `object/src/mem.rs`, matching the pattern already used by `set_transfer_syntax`/`apply`:

```rust
// `FileMetaTable::build()` already computed `information_group_length` from
// whatever was actually present in the file's own meta group, before either
// inference above had a chance to run - so it must be recomputed here if either
// one just changed a field, the same way `set_transfer_syntax`/`apply` already do
// immediately after mutating a meta field. Otherwise the now-stale, too-short
// group length gets written out verbatim, truncating the meta group and
// spilling its last element(s) into the main data set on the next read.
meta.update_information_group_length();

Ok(FileDicomObject { meta, obj })
```

See `patch.diff` for the exact diff (fix only; the accompanying unit test added during
investigation - `inferred_sop_class_uid_keeps_group_length_in_sync` - is not included, matching
this session's convention of keeping `patch.diff` minimal).

- `cargo test -p dicom-object --lib`: baseline (test present, fix removed) - the new test
  **FAILED** with exactly the predicted stale/refreshed mismatch (`left: 134, right: 142`) -> with
  the fix restored - **76 passed, 0 failed**.
- Full workspace (`cargo test --workspace --lib`, all 12 crates with a `#[lib]` target):
  **351 passed / 0 failed / 2 ignored** (pristine, no candidate changes) -> **352 passed / 0
  failed / 2 ignored** with this fix + its 1 test applied - zero regressions anywhere in the
  existing suite.
- Confirmed independent of candidate 053: reproduced and fixed against a pristine tree with
  *only* this candidate's patch applied (053's SOP-UID-padding fix deliberately absent).

## What's not done / open questions

- This fix recomputes the group length unconditionally after the inference step, regardless of
  whether either inference block actually changed anything - a cheap, purely computed operation
  with no observable cost difference from conditionally guarding it, so no attempt was made to
  skip it when both fields were already non-empty.
- No other post-construction `FileMetaTable` mutation sites beyond the two already checked
  (`set_transfer_syntax`, `apply()`, both already correct, and this inference step, now fixed)
  were found during this investigation to have the same gap.
