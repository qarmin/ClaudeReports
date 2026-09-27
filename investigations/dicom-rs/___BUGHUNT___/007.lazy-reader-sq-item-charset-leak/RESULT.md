# SpecificCharacterSet item-scope leak also present in LazyDataSetReader

From `IDEAS.md`'s backlog, itself noticed while working candidate 006
(`006.sq-item-charset-leak`): candidate 006 fixed the flat/eager `DataSetReader` and
`DataSetWriter` but explicitly did not check whether `LazyDataSetReader`
(`parser/src/dataset/lazy_read.rs`) - used internally by `object::collector::DicomCollector`,
a separate, independent token-generation implementation from `DataSetReader` - shares the same
root cause. It does; same class of bug (per `HEURISTICS.md`/`SKILL.md` pitfall #7, "the same
underlying bug can live in more than one place"), fixed the same way.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `parser/src/dataset/lazy_read.rs`'s `LazyDataSetReader` - confirmed it has its own, separately-defined `SeqToken` struct and `seq_delimiters: Vec<SeqToken>` stack (structurally near-identical to `dataset::read::DataSetReader`'s, but a distinct type in a distinct module, not shared code) and its own `push_sequence_token`, with no charset save/restore around item boundaries - the identical gap as candidate 006, just duplicated into a second implementation. |
| Standalone repro (release build, `object`'s public API: `DicomCollector::open_file`, `read_file_meta`, `read_dataset_to_end`) | `___BUGHUNT___/007.../repro`. Built a complete, valid DICOM file (128-byte preamble + `DICM` magic + a minimal file meta group via `FileMetaTableBuilder`, transfer syntax Explicit VR LE) whose data set is the exact same nested-SQ-with-item-charset construction as candidate 006's repro (a `ReferencedStudySequence` item declaring `SpecificCharacterSet = ISO_IR 126`, followed by a top-level `PatientName` with the identical raw byte `0xE1` and no charset declaration of its own), written to a temp file and read back via `DicomCollector` (which uses `LazyDataSetReader` internally, unlike the dataset-level API candidate 006 used directly). | Reproduced deterministically: both the item-scoped and the top-level `PatientName` decoded to the identical string `"α"` (Greek alpha) pre-fix - the exact same symptom as candidate 006, via the separate lazy-reading code path. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic, no randomness in this code path. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`logic` (same class as candidate 006: a character set scope violation, item-local treated as
data-set-global, silently corrupting later content).

## Root cause

Identical mechanism to candidate 006, duplicated into a second, independent implementation:
`LazyDataSetReader`'s own `push_sequence_token`/`seq_delimiters` stack (a separate `SeqToken`
struct in `lazy_read.rs`, not shared with `dataset::read::DataSetReader`'s) never saved or
restored the decoder's active character set around item boundaries, so a `SpecificCharacterSet`
declared inside an item leaked into everything decoded afterward, exactly as in candidate 006 -
just via `LazyDataSetReader`'s own, separately-implemented token generation logic (used by
`object::collector::DicomCollector`, distinct from the plain `InMemDicomObject::read_dataset_*`
path candidate 006 exercised).

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Applied the exact same pattern established in candidate 006, to `lazy_read.rs`'s
`LazyDataSetReader`:
- Added a `saved_charset: SpecificCharacterSet` field to `lazy_read.rs`'s own `SeqToken` (a
  distinct struct from `dataset::read::SeqToken`, requiring its own, separate edit - not shared
  code), populated from the `StatefulDecode::active_character_set()` trait method candidate 006
  already added, in `push_sequence_token`.
- Added an equivalent `pop_seq_delimiter()` helper restoring `saved_charset` for popped `Item`
  tokens, and routed all 5 direct `self.seq_delimiters.pop()` call sites in `lazy_read.rs` through
  it (the same discipline as candidate 006: one centralized helper rather than duplicating restore
  logic at each site).

No new trait/encoder-side changes were needed here - `LazyDataSetReader` only reads (there is no
"lazy writer"; encoding always goes through the already-fixed `DataSetWriter`), and the
`StatefulDecode::active_character_set()`/`set_active_character_set()` trait methods candidate 006
added are already usable from `lazy_read.rs` since `LazyDataSetReader<S>` is generic over
`S: StatefulDecode`.

See `patch.diff` - **bundled as the full combined diff** (both candidate 006's original 4-file fix
and this candidate's `lazy_read.rs` addition), rather than referencing candidate 006's own
`patch.diff` from this folder's `justfile`, since this candidate's fix depends on the trait methods
006 introduced and per the skill's "why per-candidate folders clone instead of referencing the main
project" rule, a candidate folder may not reference anything outside itself (including a sibling
candidate folder) - this keeps `007.../patch.diff` fully self-contained and appliable to a bare
clone on its own.

Verified: `repro/` re-run against the patched tree shows the item-scoped `PatientName` correctly
staying `"α"` while the top-level one (read via `DicomCollector`/`LazyDataSetReader`) correctly
decodes as `"á"` (Latin-1).

- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, identical
  both before and after the patch (no regressions, no newly-passing tests).

## What's not done / open questions

- Same nesting-scope caveat as candidate 006: the implemented semantics are "restore on item exit"
  (correct for the tested case and any deeper nesting), not independently cross-checked against
  every subtlety of PS3.5's wording for sibling items within the same sequence.
- Not fuzzed - found by directly porting a known-real bug's construction to a second code path,
  per `HEURISTICS.md` pitfall #7's guidance to check whether a confirmed bug class repeats
  elsewhere in the codebase once one instance is found.
