# datatoken-header-branch-sequence-value-under-non-sq-vr-unreachable

Found via the same fresh `unreachable!()` sweep that led to candidate 120, while reading the
surrounding context of `parser/src/dataset/mod.rs`'s `DataElementTokens::next` state machine (the
same file/function family as already-Confirmed candidate 026, which fixed the neighboring
`PixelSequenceStart` branch's own `unreachable!()` for the identical VR/value-mismatch class - a
different, still-unfixed sibling gap in the immediately following `DataElementTokens::Header`
branch, at what is currently line 656).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading of `impl From<DataElementHeader> for DataToken` and `DataElementTokens::next`'s `Header` state | n/a | Confirmed `DataToken::from(header)` classifies purely from `(header.vr(), header.tag)` - any VR other than `SQ` (and not the pixel-data-undefined-length `OB` case) classifies as an ordinary `ElementHeader`, transitioning to the `DataElementTokens::Header` state. That state's own `match elem.into_value()` only has arms for `Value::Primitive` (returns it) and a catch-all `Value::Sequence \| Value::PixelSequence => unreachable!()` - nothing prevents a caller from constructing exactly that combination (a non-SQ VR with a `Value::Sequence`), the same recurring class as candidates 025/026/092/093/094/095/120 |
| Standalone repro building a real `InMemDicomObject` (via the public `DataElement::new`/`put_element` API) with one element declared `VR::PN` but holding a `Value::Sequence` (an empty item, via `DataSetSequence::new`), written via the real public `write_all` entry point | debug build, 3 repeat runs, against unpatched `df2b9e42` | Reproduced 3/3: `thread 'main' panicked at .../mod.rs:656:77: internal error: entered unreachable code` |

No further tools were needed - same well-established bug class as six prior candidates; a single
targeted repro was sufficient.

## Status

**Confirmed** - deterministic, pure logic/panic bug with no layout/seed/timing dependency:
reproduces 3/3 runs.

**Fully reproducible**: Yes. The repro drives the real, public `write_all` entry point on a real
`InMemDicomObject` built entirely through the crate's own public element-insertion API - the same
path any real caller would use. There is no second "peer" process here: the untrusted input is the
object's own in-memory state, constructed through the exact same public API a real caller would use
to (mistakenly or not) create a VR/value mismatch - not a hand-constructed stand-in for another
process's behavior.

## Class

crash (a bare `unreachable!()` panic reachable from ordinary use of the public `write_all`/dataset-
writing API on an object whose VR/value pairing the type system never enforced to be consistent).

## Root cause

Same underlying defect class as candidate 026, in the immediately following branch of the same
state machine: `impl From<DataElementHeader> for DataToken` classifies a `DataToken` purely from
`(vr, tag)`, with no dependency on the element's actual stored `Value`. Any VR other than `SQ`
(and not the special-cased undefined-length `OB` pixel-data tag) is classified as an ordinary
element, transitioning the writer's state machine into `DataElementTokens::Header`, which then
unconditionally expects a `Value::Primitive` - its `unreachable!()` catch-all assumed a
`Value::Sequence`/`Value::PixelSequence` could never reach this state, which is only true if every
`DataElement`'s VR and stored value are always kept in sync - something the public API never
enforces (the same root cause already documented for candidates 025/026/092/093/094/095/120).

## Introduced by

60ec6d03 "[core/object] in-memory PixelSequence repr" - the commit that introduced this exact
`unreachable!()` catch-all arm alongside the `Value::PixelSequence` variant it was written to
exclude.

## The fix

Mirrored the already-established pattern from the neighboring `SequenceStart` branch's own
mismatch handling (a few lines above in the same function, added when a different mismatch shape
was fixed there): captured the element's tag before consuming it, then replaced the
`unreachable!()` arm with a `tracing::warn!` (naming the tag) followed by emitting no value token
(`(None, DataElementTokens::End)`) - the same "too risky to synthesize a substitute token, so skip
the value entirely" resolution the `SequenceStart` branch already uses for its own
undefined-length primitive-value mismatch. Unlike candidate 026's sibling fix (which could
re-synthesize an `ElementHeader`+`PrimitiveValue` pair for its own mismatch shape), there is no
reasonable primitive fallback for a `Sequence`/`PixelSequence` value once the header token has
already gone out as an ordinary (non-SQ) element - skipping the value avoids both the panic and
inventing a nonsensical substitute value.

Verified against the repro: before the fix, the write call panics (3/3 runs); after the fix, it
completes successfully (writing 348 bytes, with the mismatched value silently - but audibly, via
the warning - dropped) with no panic - see `output.txt` for the full before/after capture.

- Project test suite: `cargo test -p dicom-parser -p dicom-object` - 75+9+49+22+0 passed, 0 failed
  across all reported groups, before AND after (identical counts, zero regressions - no existing
  test constructs this exact VR/value mismatch). Full workspace `cargo test --workspace --exclude
  dcms` (dcms excluded: untracked scratch crate belonging to a different session, not part of this
  workspace's tracked tree) - 0 failures anywhere before or after.

## What's not done / open questions

- Same open question already recorded by candidate 026 for its own sibling branch: whether the
  equivalent read/decode path (`parser/src/dataset/read.rs`) can ever itself produce this exact
  mismatch when parsing a crafted file (as opposed to this candidate's write-path repro, which
  relies on constructing the mismatch directly via the public API) - not independently checked
  this session either.
- This candidate and candidate 026 patch two different match arms in the same function
  (`DataElementTokens::next`'s `SequenceStart`/`PixelSequenceStart` branch vs. its `Header`
  branch) - both are structurally independent hunks and expected to apply together without
  conflict, but (per this skill's "always revert" rule) this was not independently verified this
  session, following the same follow-up pattern already recorded for other candidate pairs sharing
  a file (e.g. 004/080/081, 092/093, 068/108).
- The remaining `unreachable!()` sites found by this session's fresh sweep
  (`encoding/src/adapters.rs`'s 8 occurrences, `encoding/src/transfer_syntax/mod.rs`'s 2,
  `findscu/src/main.rs`'s 1) are still unchecked - left in `IDEAS.md`'s backlog.
