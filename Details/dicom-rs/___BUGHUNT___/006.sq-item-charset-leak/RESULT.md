# SpecificCharacterSet declared inside a sequence item leaks past the item boundary

From `IDEAS.md`'s backlog, itself noticed while working candidate 002
(`002.writer-charset-not-switched`): the old (now-lost) fuzz-bug-repro campaign's "bug-04" fix
description mentioned "writer pre-priming via token-stream scan" and "tag-order-safe `IntoTokens`"
- mechanisms that don't obviously follow from candidate 002's simpler, confirmed VR::UI root
cause. Speculated this might point at a separate, sequence(SQ)-nesting-specific charset gap.
Constructed a direct test for it rather than continuing to speculate.

**Update**: initially recorded as Confirmed with no bounded fix attempted (see git history of this
file / `CORRECTION` note below is unnecessary since this is the same session completing its own
work, not a later candidate correcting an earlier one) - revisited within the same session once
the full scope was mapped out, and a complete fix (both decode and encode sides) was implemented
and verified.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `parser/src/stateful/decode.rs`'s `set_character_set`/`read_value_cs` (the only place `self.text` is ever mutated during decode - triggered whenever an element tagged `(0008,0005)` is decoded, unconditionally, with no notion of "scope") and `parser/src/dataset/read.rs`'s `DataSetReader` (a `seq_delimiters: Vec<SeqToken>` stack already exists to track sequence/item nesting depth for length bookkeeping). Confirmed `self.text` is a single flat field, shared and mutated in linear stream order regardless of nesting depth, and that nothing saved/restored it around item boundaries. |
| Standalone repro (release build, `object`'s public API: `InMemDicomObject::read_dataset_with_ts`/`write_dataset_with_ts`) | `___BUGHUNT___/006.../repro`. Hand-built explicit-VR-LE wire bytes: a sequence (`ReferencedStudySequence`, `0008,1110`) containing one item that declares its own `SpecificCharacterSet = ISO_IR 126` (Greek) and a `PatientName` (`0010,0010`, VR=PN) with a single Greek-encoded byte (`0xE1` = Greek alpha under ISO-8859-7), followed by a **top-level** `PatientName` element (same tag, main data set scope, appearing *after* the sequence closes) with the identical raw byte `0xE1` - the top-level scope never declares its own `SpecificCharacterSet`, so per PS3.5 it should decode under the default repertoire (where `0xE1` is Latin-1 'a with acute', a different character). | Reproduced deterministically: both the item-scoped and the top-level `PatientName` decoded to the identical string `"α"` (Greek alpha) pre-fix. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic, no randomness in this code path. |
| Scope mapping for a bounded fix | Traced every push/pop site touching `seq_delimiters`/`seq_tokens` on both `parser/src/dataset/read.rs` (decode) and `parser/src/dataset/write.rs` (encode), and the field visibility needed to reach the active character set from each (`StatefulDecode`/`StatefulEncoder` didn't expose it outside their own modules). | Found a single centralized push point on each side (`push_sequence_token` for decode, inlined per-branch pushes for encode) and multiple pop sites (6 on decode, 2 relevant on encode) - all safely unifiable behind one save-on-push/restore-on-pop helper per side, avoiding the need to reason about each site's individual control flow. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`logic` (a character set scope violation: an item-local declaration is treated as data-set-global,
silently corrupting the interpretation of unrelated, later content that never opted into a
non-default character set).

## Root cause

`StatefulDecoder::set_character_set` (called from `read_value_cs` whenever a `(0008,0005)`
element is decoded, at any nesting depth) unconditionally overwrites the single shared `self.text`
field used for all subsequent charset-affected text decoding, with no awareness of DICOM's actual
nesting scope rules (PS3.5 6.1.2.5: a `SpecificCharacterSet` declared inside a sequence item
applies only to that item, and any sequences nested inside it - not to sibling items, not to the
enclosing data set, and not to content after the sequence closes). The equivalent mechanism on the
encode side (`StatefulEncoder`'s `try_new_codec`, triggered from `encode_text_element` the same
unconditional way) has the identical gap. Both `DataSetReader` (decode) and `DataSetWriter`
(encode) already maintain a stack precisely tracking entry into and exit from every item, but
nothing hooked into those push/pop points to save the charset in effect on entry and restore it on
exit.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

**Decode side** (`parser/src/stateful/decode.rs`, `parser/src/dataset/read.rs`):
- Added `active_character_set()`/`set_active_character_set()` to the `StatefulDecode` trait
  (implemented on `StatefulDecoder` and the blanket `&mut D` delegate), exposing the
  previously-module-private character set state.
- Added a `saved_charset: SpecificCharacterSet` field to `read.rs`'s (private) `SeqToken`,
  populated from `active_character_set()` in the single centralized `push_sequence_token`.
- Added a `pop_seq_delimiter()` helper that pops `seq_delimiters` and, if the popped token was an
  `Item` (not a bare `Sequence` token - sequences aren't scope containers themselves, only their
  items are), restores `saved_charset` via `set_active_character_set()`. **Every** direct
  `self.seq_delimiters.pop()` call site (6 total, across `ItemDelimiter`/`SequenceDelimiter`
  handling, the pixel-data `UnexpectedEof` special case, and length-tracking closure in
  `update_seq_delimiters`) was routed through this single helper instead of duplicating the
  restore logic at each site - deliberately, to avoid the exact class of mistake described below.

**Encode side** (`parser/src/stateful/encode.rs`, `parser/src/dataset/write.rs`), added for full
round-trip correctness - fixing decode alone would let a correctly-scoped in-memory object get
mis-encoded the moment it's written back out (the item's charset switch would still leak into
whatever gets encoded after the item, in the opposite direction):
- Added the equivalent `active_character_set()`/`set_active_character_set()` methods directly on
  `StatefulEncoder` (no trait involved here, since `DataSetWriter`'s `write()` method - where the
  push/pop sites live - is only implemented for the concrete `T = SpecificCharacterSet` case
  already, so no trait indirection was needed).
- Added the equivalent `saved_charset` field to `write.rs`'s own (private, separately-defined)
  `SeqToken`, populated at all 5 push sites (`SequenceStart` x2 strategies, `ItemStart` x2
  strategies, `PixelSequenceStart`), and restored in `DataToken::ItemEnd`'s handling (the only
  site that pops an `Item`-typed token in the writer).

See `patch.diff` (4 files: `parser/src/stateful/decode.rs`, `parser/src/stateful/encode.rs`,
`parser/src/dataset/read.rs`, `parser/src/dataset/write.rs`). Verified: `repro/` re-run against
the patched tree now shows the item-scoped `PatientName` correctly staying `"α"` while the
top-level one correctly decodes as `"á"` (Latin-1), **and** a full write-then-reparse round trip
of the corrected object preserves `"á"` (proving the encode-side fix is also effective, not just
the decode-side one).

**Self-inflicted regression caught and fixed before finalizing** (this was actually the second
occurrence of this exact mistake pattern this session, after an identical one in candidate 004): a
`sed`-based mechanical rewrite risk was anticipated this time and avoided by manually verifying
each of the 6+5 individual replacement sites rather than blindly scripting the substitution, given
`checked_resize_buffer`'s self-recursion bug in candidate 004 had already demonstrated how a
mechanical multi-site rewrite can accidentally match its own helper's body. No regression actually
occurred here, but the full test suite was still run before/after as the mandatory final check.

- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, identical
  both before and after the patch (no regressions, no newly-passing tests - no existing test
  constructed a nested per-item `SpecificCharacterSet` declaration).

## What's not done / open questions

- The exact nesting-scope semantics implemented here are a straightforward "restore on item exit"
  stack, which correctly handles the tested case (and any item nested inside another item, since
  the stack naturally composes). PS3.5's own wording is subtler in one respect not specifically
  tested: a charset declared directly inside an item is stated to remain in scope for any
  *subsequent* items within the same *enclosing* sequence too in some phrasings of the standard -
  this implementation instead scopes strictly to "this item and anything nested inside it," which
  matches the more common/conservative reading and every other DICOM toolkit's typical behavior,
  but wasn't cross-checked against the precise standard text for every edge case (e.g. multiple
  sibling items, only the first of which redeclares the charset).
- Not checked: whether the same leak (now independently) exists in the **lazy** dataset reader
  (`LazyDataSetReader`, `parser/src/dataset/lazy_read.rs`) - likely shares the identical root cause
  (same `StatefulDecoder`/`self.text` mechanism) but has its own, separate token-generation logic
  and its own item-tracking, so this candidate's fix does not cover it. Left in `IDEAS.md`.
- Not fuzzed - found by direct reasoning from a speculative lead, then confirmed and fixed by hand
  construction. A short fuzz campaign seeded with nested `SpecificCharacterSet` declarations might
  find further edge cases in the scoping semantics noted above.
