# filemeta-sop-uid-inference-no-character-validation

Found while stacked-fuzzing (a batch of 78 already-Confirmed patches applied, including candidate
077's file-meta UID character validation) `object/fuzz/fuzz_targets/open_file.rs`: a crash
reachable purely by applying candidate 077's own patch, tracing back to a call site 077 never
touched. `object/src/mem.rs`'s SOP-UID-inference fallback (already the subject of candidates
053/054, which fixed its missing *padding*) copies the main data set's own SOP Class/Instance UID
value verbatim into the file meta group's Media Storage SOP Class/Instance UID fields whenever the
latter are empty - with **no character-repertoire validation at all**, unlike every other way of
setting these fields. This is a real, independent gap left open by both 053/054 (padding-only) and
077 (validates a completely different call site, `object/src/meta.rs`'s own parsing loop).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Stacked-fuzzing: applied 78 cleanly-combinable already-Confirmed patches (candidates 001-126, skipping ones that conflicted) to the main tree, built, then fuzzed `object/fuzz/fuzz_targets/open_file.rs` with the existing 1555-file corpus plus the `dcms/` real-file corpus | `cargo +nightly fuzz run open_file corpus/open_file ../../dcms -- -max_total_time=600`, `DEBUGINFOD_URLS=` unset | Crashed almost immediately: `panicked at fuzz_targets/open_file.rs:30: serialized object should always deserialize: ParseMetaDataSet { source: InvalidUid { .. } }` - the harness's own second (post-write) read failed candidate 077's UID-character check, while the *first* read of the same raw bytes had succeeded |
| Standalone diagnostic (dedicated path-dep binary, temporary `eprintln!` tracing added to `object/src/meta.rs`'s parsing loop and `read_str_body`) tracing every tag visited and every `read_str_body` call during both reads of the crash artifact | debug build, only candidate 077's patch applied (isolated from the other 77) | Traced the discrepancy precisely: on the first read, the raw fuzzer bytes never actually contain a `(0002,0002)` element at all (a desynced/garbled tag appears in that position instead, unrelated to Media Storage SOP Class UID) - `read_str_body` is called exactly once, for Transfer Syntax. Yet the printed `FileMetaTable` already showed a *populated*, non-conformant `media_storage_sop_class_uid`, meaning it was never validated by `read_str_body` at all - it came from a different code path entirely |
| Read `object/src/mem.rs` around the SOP-UID-inference step (the same code candidates 053/054 already fixed for padding) | - | Found the root cause: this call site does `elem.value().to_str()?.to_string()` straight from the main data set's own `(0008,0016)` SOP Class UID element, with **zero character validation** - the fuzzer's garbled main-dataset SOP Class UID value gets copied verbatim into the file meta group by this inference step (not the file-meta-group parsing loop `read_str_body`/candidate 077 covers), producing a non-conformant on-disk UID field |
| Built a real, deterministic, non-fuzzer-dependent repro (`repro/src/main.rs`) via the real public `dicom_object::from_reader` entry point: a well-formed file whose meta group omits Media Storage SOP Class UID (triggering inference) and whose main data set's own SOP Class UID contains characters PS3.5 6.2's UI VR never permits ("1.2.NOT-A-REAL-UID!\0") | pristine `df2b9e42`, **no other candidate's patch needed** - this reproduces on totally clean master, independent of candidate 077 | Confirmed: `obj2.meta().media_storage_sop_class_uid()` = `"1.2.NOT-A-REAL-UID!"`, and the actual on-disk bytes for the file meta group's own `(0002,0002)` element contain the same non-conformant characters verbatim - deterministic, 3/3 runs, plus a standalone fresh-clone `just test` |

## Status

**Confirmed.** Reproduces deterministically (pure function over caller-supplied data, no
I/O/randomness) - 3/3 runs both in-tree and via a standalone fresh-clone `just test`.

**Fully reproducible**: **Yes**. The repro drives the real public `dicom_object::from_reader`
entry point (the same one any real caller uses to open a file) on a genuinely constructed object
built entirely through the crate's own public `FileMetaTableBuilder`/`put_element` API. Single
external artifact (a file whose own main-dataset SOP Class UID already contains non-conformant
characters - the same shape a malformed or simply differently-encoded real file could have), no
second participant to fake, so `CONFIRMATION.md` section 11 does not apply.

## Class

logic

## Root cause

`object/src/mem.rs`'s `read_parts_with_all_options_impl` (used by every public file-reading entry
point) infers Media Storage SOP Class/Instance UID from the main data set's own SOP Class/Instance
UID elements whenever the file meta group's own values are empty. This inference does
`elem.value().to_str()?.to_string()` with no check that the copied value is actually a conformant
UID (PS3.5 6.2 restricts the UI VR's character repertoire to digits, `.`, and padding) - unlike
every other way these fields get set (`FileMetaTableBuilder`'s own setters, which pad and are
typically set from already-validated sources) and unlike candidate 077's separate validation of
the file meta group's *own* wire-encoded UID elements. A main data set SOP Class/Instance UID
containing arbitrary characters (a malformed real file, or just a non-conformant one from a lax
encoder) is silently promoted, verbatim, into a formally UID-restricted file meta group field -
producing a file whose own file meta group is itself non-conformant, something no other assignment
path for these fields allows.

## Introduced by

c9160c80 "Infer Media Storage SOP Class UID and Media Storage SOP Instance UID from the SOP Class
UID and SOP Instance UID when missing" (2024-07-13) - the commit that introduced this inference
step; `git log -S` on its own `if let Some(elem) = obj.get(tags::SOP_CLASS_UID)` line confirms no
later commit touched this specific logic before the fix applied here (candidates 053/054's own fix
for the neighboring padding gap left this exact character-validation gap untouched).

## The fix

Added a small `is_uid_conformant` helper (mirroring candidate 077's own character set: `.`, ASCII
digit, NUL, ASCII whitespace) and gated both inference assignments on it - if the main data set's
own SOP Class/Instance UID value isn't UID-conformant, the inference is skipped and the field is
left empty, matching the existing fallback behavior when the data set has no SOP Class/Instance
UID at all (rather than propagating an error, which would break parsing of an otherwise-fine file
just because one attribute happens to be non-conformant). Verified against both a non-conformant
value (now correctly skipped, left empty) and the crate's own existing
`inmem_with_meta_infers_sop_instance_uid`/`infer_media_sop_from_dataset_sop_elements` tests (a
legitimate, conformant UID value, still correctly inferred - unaffected by this change).

- Project test suite: `cargo test -p dicom-object --lib`: 75 passed / 0 failed / 0 ignored both
  before and after (identical). `cargo test --workspace --exclude dcms --lib`: 351 passed / 0
  failed / 2 ignored both before and after (identical). No regressions.

## Addendum (same session): independently re-confirmed via a second fuzzer-found crash

A second, longer (5-minute) stacked-fuzzing round with this candidate's own fix included in the
78-patch stack found a second crash (`crash-12a24b987e11227b2461e52e2d237f3946e55f25`, a fuzzer
mutation of the same underlying corpus entry as the original crash) - but this one reached the
inference call site through a different shape: the main data set's `SOP_CLASS_UID` element was
declared with VR `SS` (a binary, signed-short VR) instead of a text VR, so `elem.value().to_str()`
produced a backslash-and-minus-sign-separated list of decimal numbers (e.g.
`"11825\-12750\13368\...\49"`) rather than garbled text - still clearly non-conformant, and still
caught by the same `is_uid_conformant` check. Verified with only candidate 077 applied (reproduces
the same `InvalidUid` round-trip failure) and with 077+this candidate's patch both applied (fixed,
second read succeeds, inferred field left empty) - confirming the fix's `char`-level check is
robust to the *source* value being an arbitrary non-UID `PrimitiveValue` representation, not just
arbitrary text. No new candidate needed; this is the same defect via a different trigger input.

## What's not done / open questions

- This fix only addresses the two inference call sites in `mem.rs` (the ones candidates 053/054
  already established as sharing the padding gap). No other post-construction `FileMetaTable`
  mutation site was re-audited for the same missing-validation pattern beyond what 053/054's own
  "what's not done" section already scoped.
- Whether other DICOM toolkits (dcmtk, GDCM, real PACS software) would actually reject a file with
  a non-conformant Media Storage SOP Class UID in practice - not verified against a real
  third-party implementation this session; the fix is scoped to PS3.5's own stated character
  repertoire restriction, which is authoritative regardless of any particular reader's actual
  strictness.
- The stacked-fuzzing session that led to this discovery (78-patch stack, ~10 minutes of fuzzing)
  was not continued further after this crash was found and root-caused - a longer run past this
  fix might surface additional new findings, not attempted this session.
