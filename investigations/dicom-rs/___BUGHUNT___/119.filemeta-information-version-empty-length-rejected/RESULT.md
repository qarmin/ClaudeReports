# filemeta-information-version-empty-length-rejected

Found via this session's GitHub issue raw-sync: upstream issue `Enet4/dicom-rs#565` (open, no
linked PR as of `df2b9e42`, filed 2024-10-04) reports a real file from a NewTom CT scanner that
dicom-rs entirely refuses to open (`UnexpectedDataValueLength { tag: Tag(0x0002, 0x0001), ... }`),
even though every other tag in the file is fine per the reporter's own description.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading of `object/src/meta.rs`'s file-meta-group parsing loop, specifically the `Tag(0x0002, 0x0001)` ("File Meta Information Version") match arm | n/a | Found the mechanism: this arm hard-rejects with `UnexpectedDataValueLength` unless the element's declared length is exactly `2`. Cross-checking `FileMetaTableBuilder::build()` (a few hundred lines below) found this field is otherwise entirely optional at the builder level - a genuinely *missing* element already falls back to a documented default (`[0, 1]`, with a comment citing issue #28/PR #29) - so a *present-but-wrong-length* element being treated more strictly than a *missing* one looked like an inconsistency, not an intentional strictness choice |
| Standalone repro via the real public `dicom_object::from_reader` entry point, on a minimal hand-encoded file meta group with (0002,0001) present but declared length `0` (matching the issue's own described shape) followed by a mandatory Transfer Syntax UID | debug build, 3 repeat runs, against unpatched `df2b9e42` | Reproduced 3/3: `parse error: Could not parse meta group data set` - the whole file fails to open |
| Same repro, 3 repeat runs, against a tree with this candidate's own fix applied | debug build | The file parses successfully every time, with `information_version` falling back to the same documented `[0, 1]` default a missing element would already get |

No further tools were needed - this is a narrow, deterministic parsing-strictness inconsistency
with a clean, already-precedented fix (reuse the existing "missing means default" fallback for
"present but not the expected length" too).

## Status

**Confirmed** - deterministic, pure parsing-logic bug with no layout/seed/timing dependency:
reproduces 3/3 runs both for the original (still-live) rejection and for this candidate's own fix
(3/3 successful parses).

**Fully reproducible**: Yes. The repro drives the real, public `dicom_object::from_reader` entry
point on a genuinely valid-shape file meta group - a single external artifact (the file itself)
with no second participant to fake; per `CONFIRMATION.md` section 11 this single-process shape is
the correct one here (same reasoning as candidates 053/108/118's own meta-parsing repros).

## Class

logic

## Root cause

`object/src/meta.rs`'s file-meta-group parsing loop requires the File Meta Information Version
element (0002,0001) to have a declared length of exactly 2 bytes, failing the entire file's parse
with `UnexpectedDataValueLength` otherwise. This element's actual semantic content (a single bit
identifying protocol version 1) is not load-bearing for correctly reading the rest of the file, and
the project's own code already acknowledges this: `FileMetaTableBuilder::build()` treats a
genuinely *missing* File Meta Information Version as entirely fine, silently defaulting to `[0, 1]`
(a fix already made for a different, older issue, #28/PR #29). The length-validation check for a
*present* element never received the same leniency - so a real-world device (per the issue, a
NewTom CT scanner) that writes this element present-but-empty is treated more strictly than a
device that omits it entirely, even though both convey the same practical information ("we don't
know/didn't set a meaningful version").

## Introduced by

Not determined precisely - the strict `length() != Length(2)` check has existed since very early in
the project's history (`git log -S` on the check's current phrasing only turns up a later type
refactor, `cf02290e` "[core] Refactor DICOM data length retrieval"; `git blame`/`log --follow`
further back lands on generic early commit messages, "Progress"/"Progressing..."/"First commit",
that don't meaningfully pin down original intent). Not worth a deeper `git bisect` this session -
the check's *existence* isn't in question, only that it was never relaxed to match the leniency
`build()` already grants a missing element.

## The fix

Changed the `Tag(0x0002, 0x0001)` match arm to read whatever length was actually declared (keeping
the stream in sync, exactly as every other element in this loop already does via its own `elem_len`)
and only call `builder.information_version(..)` when that turns out to be exactly 2 bytes;
otherwise, log a `tracing::warn!` (naming the tag and the unexpected length) and leave the field
unset, letting `build()`'s pre-existing `unwrap_or([0, 1])` default apply - the exact same path a
genuinely missing element already takes. No new fallback logic was introduced; this simply extends
the *existing* leniency to one more way of "not usefully specifying the version."

Verified against the repro: before the fix, the file fails to parse entirely (3/3 runs, "Could not
parse meta group data set"); after the fix, the file parses successfully with `information_version
== [0, 1]` (3/3 runs) - see `output.txt` for the full before/after capture, including the emitted
warning.

- Project test suite: `cargo test -p dicom-object` - 75+9+22 passed, 0 failed across all reported
  groups, before AND after (identical counts, zero regressions - no existing test constructs a
  File Meta Information Version element with a non-2 length). Full workspace `cargo test
  --workspace --exclude dcms` (dcms excluded: untracked scratch crate belonging to a different
  session, not part of this workspace's tracked tree) - 0 failures anywhere before or after.

## What's not done / open questions

- The specific real file attached to issue #565 was not obtained/tested directly this session (no
  network access) - this candidate's repro instead hand-constructs a minimal file meta group with
  the exact declared-length shape described in the issue's own error message (length 0), which is
  sufficient to confirm the mechanism and the fix without needing the original file.
- Not checked: whether any *other* file-meta element in this same parsing loop has a similarly
  strict length check that could benefit from the same "present-but-wrong-length treated like
  missing" leniency (e.g. Implementation Class UID's own length handling was not re-examined this
  session beyond what candidates 053/054 already covered, which was about padding, not length
  strictness) - not swept exhaustively.
- This fix only relaxes the *length* check for this one element; it does not attempt to validate
  or otherwise interpret whatever non-2-byte content might actually be present when the length is
  merely small-but-nonzero (e.g. a hypothetical length-1 value) - such content is silently
  discarded (consumed from the stream, then ignored) exactly the same way a length-0 value is,
  which matches the "we don't have a useful version" interpretation but was not separately called
  out as its own tested case.
