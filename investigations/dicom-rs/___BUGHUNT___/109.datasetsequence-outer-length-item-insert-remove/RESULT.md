# datasetsequence-outer-length-item-insert-remove

Follow-up to candidate 082/090's item-level `len` staleness class, one level up: `IDEAS.md` flagged
that a `DataSetSequence`'s own outer declared `length` field (`core/src/value/mod.rs`) could go
stale if some public `dicom-object` API let a caller insert/remove whole items from an existing
sequence in place, without recomputing that field - the note explicitly said no such API had been
found yet. This candidate went looking for one and checked whether it actually leaves stale state
reachable on the wire.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: grep `object/src/mem.rs`/`core/src/value/mod.rs`/`core/src/header.rs` for every public path that can reach `DataSetSequence::items_mut()` (push/remove/truncate) | Found `DataSetSequence::items_mut()` (`core/src/value/mod.rs:904`) and `DataSetSequence::truncate()` (:934) are public, and are reachable from `dicom-object`'s public API two ways: (1) `InMemDicomObject::update_value_at`/`update_value` hand the caller `&mut Value<...>`, which can be matched to `Value::Sequence(seq)` and then `seq.items_mut()`/`truncate()` called directly; (2) `apply()`'s `Nested`-step traversal and `entry_at_mut`'s `Nested`-step traversal both call `DataElement::items_mut()` (not `Value::items_mut()`) to reach an ancestor sequence's item list for constructive pushes. So, contrary to the note's assumption, such an API does exist | Traced whether either path leaves the sequence's own header length stale on write |
| Code reading: `DataElement::update_value` (`core/src/header.rs:262`) and `DataElement::items_mut`/`fragments_mut` (`core/src/header.rs:513-550`) | Both are unconditionally guarded: `update_value` runs the caller's closure against `&mut Value`, then - regardless of what the closure did internally - matches on the *resulting* value and, for `Value::Sequence(_)`/`Value::PixelSequence(_)`, forcibly sets `self.header.len = Length::UNDEFINED` (doc comment: "the header is automatically updated... length is reset to undefined"). `items_mut()`/`fragments_mut()` do the same reset inline, before returning the mutable items collection, so even a caller who skips `update_value` entirely and calls `DataElement::items_mut()` directly still can't leave a stale `header.len` behind. `InMemDicomObject::entries` (the map holding each tag's `DataElement`) is a private field with no public `get_mut`/`element_mut`/`IndexMut`/`iter_mut`, so there is no way to obtain a raw `&mut DataElement` or `&mut Value` that bypasses both guards | No public path found that reaches `DataSetSequence::items_mut()`/`truncate()` while skipping the `header.len` reset - the *element header's* declared length (what the writer actually serializes as `SequenceStart{ len }`, per `parser/src/dataset/mod.rs`'s `DataElementTokens::Start`, which reads `elem.header().len`, not `DataSetSequence`'s own private `length` field) is always forced to `Length::UNDEFINED` the moment an item is inserted/removed this way |
| Direct reproduction, debug build (`object/examples/scratch_109_seq_len.rs`, scratch file removed before finishing this candidate) - real end-to-end round trip: hand-built well-formed wire bytes for a top-level SQ element (tag 0008,1140) with an EXPLICIT (defined, 20-byte) length containing one item (`PatientID`="1234"), parsed via the crate's real `InMemDicomObject::read_dataset_with_ts`, mutated via the public `obj.update_value_at(tag, \|v\| { if let Value::Sequence(seq) = v { seq.items_mut().push(new_item); seq.items_mut().remove(0); } })` (both an insertion and a removal of items directly on the sequence's item list, no leaf value touched), then re-serialized via the real `write_dataset_with_ts_options` with `ExplicitLengthSqItemStrategy::NoChange` (the non-default strategy that writes cached explicit lengths verbatim - the one strategy that could expose this class of bug, since the default `SetUndefined` discards all cached sequence/item lengths on write regardless) and re-parsed via the same real `read_dataset_with_ts` | Did **not** reproduce: the writer emitted `SequenceStart{ len: Length(Undefined) }` (confirmed by the successful re-parse finding exactly 1 item after the push+remove, i.e. the sequence's own boundary was written as an undefined-length delimited sequence, not as a stale 20-byte explicit length that would no longer match the now-different item content) - see `output.txt` |

## Status

**Not reproducible.**

Code reading found the mutation API the original note assumed didn't exist yet
(`DataSetSequence::items_mut()`/`truncate()`, reachable via `update_value_at`/`update_value` and via
`apply`'s/`entry_at_mut`'s `Nested`-step traversal), but both public entry points that can reach it
(`DataElement::update_value` and `DataElement::items_mut()`/`fragments_mut()`) unconditionally reset
the owning `DataElement`'s own `header.len` to `Length::UNDEFINED` as part of the same call, before
control ever returns to the caller - this is exactly the field the writer serializes for
`SequenceStart`, not `DataSetSequence`'s own private `length` field (which does go stale internally,
but is never read again after being set at parse time in `InMemDicomObject::build_object`, per a
grep for `.length()`/`HasLength::length` calls on `Value::Sequence`/`DataSetSequence` values -
the sequence's own private `length` field is only reachable through `HasLength::length()`/
`DataSetSequence::length()`, and the only caller of those on this path is `DataElement::new()` at
construction time). Tried with both an insertion and a removal on the same sequence, on a real
parsed object with an explicit (non-undefined) declared length, written with the one writer
strategy (`ExplicitLengthSqItemStrategy::NoChange`) capable of exposing a stale explicit length -
round-tripped correctly both times (0 reproductions across both mutation kinds, debug build).

## Class

n/a (not reproducible)

## Root cause

Not applicable to a bug - the suspected gap doesn't exist. The class of bug candidate 082/090
confirmed (an *ancestor item's own* cached `InMemDicomObject::len`, used only when that item itself
was read with an explicit item length, going stale after a *leaf nested inside it* is mutated) is
structurally different from what this candidate checked (the *sequence element's own* declared
length, used when the sequence itself was read with an explicit length, going stale after an item is
inserted/removed *directly under that sequence*). The latter turns out to already be handled: every
public mutator that can reach a sequence's item list re-derives and overwrites the owning element's
`header.len` as an unconditional side effect of the same call, rather than leaving the caller to
remember to do so - the same invalidation discipline 082/090's *fix* introduces for `entry_at_mut`/
`apply`'s ancestor-item traversal already existed for this specific field before this session.

## The fix

Not applicable - nothing to fix.

## What's not done / open questions

- Only `update_value_at`/`update_value` and the two `Nested`-step traversal loops (`apply`,
  `entry_at_mut`) were checked as ways to reach `DataSetSequence::items_mut()`/`truncate()`; no
  other public API surface reaching a `DataSetSequence` by mutable reference was found in
  `dicom-object` (confirmed by grepping for `pub fn` returning `&mut` types touching `entries`,
  `Value`, or `DataElement` in `object/src/mem.rs` - none exist beyond the ones checked here).
- This closes the `IDEAS.md` backlog entry noted while working candidate 082 on 2026-08-25 - the
  entry's own premise (no such API exists) turned out to be only half right (the API exists) but
  its conclusion (nothing to check yet) turns out to still hold, for a different reason (the API
  is already safe by construction).
- While tracing every caller of `DataSetSequence::length()`/`HasLength::length()` on a sequence
  value, found that `apply_leaf`'s `AttributeAction::SetVr` arm (`object/src/mem.rs`) reconstructs
  the element via `DataElement::new(header.tag, new_vr, value)`, which re-derives `header.len` from
  `value.length()` - for a sequence, that reads `DataSetSequence`'s own private `length` field
  directly, bypassing the `update_value`/`items_mut()` guards this candidate found. If a caller
  first mutates a sequence's items via `update_value_at`/`items_mut()` (correctly resetting
  `header.len` to `Length::UNDEFINED`, per above) and *then* calls `SetVr` on that same tag, the
  stale pre-mutation `DataSetSequence.length` could resurface into the reconstructed `header.len`,
  potentially reintroducing this exact bug class through a second-order path. Not verified
  end-to-end this session (scope was the original single-mutation question); added to `IDEAS.md`'s
  backlog for a follow-up candidate.
