# object-apply-nested-item-len-invalidation

`InMemDicomObject::apply` (`object/src/mem.rs`, the `dicom_core::ops::ApplyOp`/`AttributeOp`-based
mutation entry point) has its own, separate tree-traversal loop for walking an `AttributeSelector`
down into nested sequence items - distinct from `entry_at_mut`/`update_value_at`, which candidate
082 (`object-selector-nested-item-len-invalidation`) already confirmed and fixed for exactly this
defect class. This candidate checks whether `apply`'s own, unrelated loop has the same gap.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `object/src/mem.rs`'s `apply` (the `Nested`-step branch, ~line 1532-1580) vs. its sibling `entry_at_mut` (already fixed by 082) | Confirmed structurally identical gap: `apply`'s loop rebinds `obj` at each `Nested` step via `items.get_mut(*item as usize)` (or `push`/`last_mut()` for constructive actions) but never invalidates the just-left-behind object's own `len`; only the innermost (leaf-owning) object gets `self.len = Length::UNDEFINED` set, and only by some of `apply_leaf`'s mutating arms (e.g. `Empty`, `apply_change_value_impl`) |
| Direct reproduction, single level of nesting (debug build, `object/examples/apply_bug.rs` scratch binary, removed before finishing this candidate) | One SQ element (undefined length) containing one item with an EXPLICIT length of 10 bytes, whose sole content is `PatientName="AB"`; mutated via `obj.apply(AttributeOp::new((tag, 0, leaf_tag), AttributeAction::SetStr(...)))` to a 16-byte string, then written with `ExplicitLengthSqItemStrategy::NoChange` and re-read | **Did not reproduce** - re-parsed successfully. Root cause: with only one level of nesting, the object bound to `obj` after the sole `Nested` step *is* the leaf-owning item itself, so `apply_leaf`'s own `self.len = Length::UNDEFINED` already covers it. The bug only manifests with an *ancestor* item strictly above the leaf-owning one. |
| Direct reproduction, two levels of nesting (debug build, same scratch binary, revised) | Outer SQ (0008,1140, undefined length) -> item0 (EXPLICIT length = 38) -> inner SQ (0008,1141, undefined length) -> item1 (EXPLICIT length = 10) -> leaf `PatientName` (0010,0010) = "AB"; mutated via `obj.apply(AttributeOp::new((outer_tag, 0, inner_tag, 0, leaf_tag), AttributeAction::SetStr("ABCDEFGHIJKLMNOP".into())))`, then written with `ExplicitLengthSqItemStrategy::NoChange` and re-read via the same real `read_dataset_with_ts` | Reproduced deterministically 3/3 runs (debug build): writer emits `item0`'s `ItemStart{len: Length(38)}` verbatim (stale - item0's real encoded content grew from 38 to 46 bytes because the leaf inside item1 grew), producing 88 bytes of output; re-parsing that output fails with `Error::CouldNotReadDataSetToken`, caused by `InconsistentSequenceEnd { end_of_sequence: 58, bytes_read: 80 }` |

## Status

**Confirmed**. Reproduces deterministically 3/3 runs (debug build, main project working tree) and
3/3 runs total including the standalone `repro/`'s release-build run via `just test` against a
fresh clone at the base commit - no randomness or timing involved anywhere in the mechanism
(purely a cache-invalidation gap in a data structure's own field), so it is expected to reproduce
100% of the time given the same input and the same `ExplicitLengthSqItemStrategy::NoChange` write
option.

**Fully reproducible**: **Yes**. As with candidate 082, this is a single participant's own
internal cache-invalidation bug, not a reaction to another participant's input - the "attacker" is
the caller's own prior sequence of real public API calls
(`read_dataset_with_ts` -> `apply` -> `write_dataset_with_ts_options` -> `read_dataset_with_ts`
again), with no second participant to fake. The one hand-constructed artifact (the initial 66-byte
input) is an ordinary, fully well-formed DICOM data set using a legal (if less common) encoding
choice (explicit, non-undefined item lengths at two nesting levels) - equivalent to a real file
handed to `read_dataset_with_ts`, not a stand-in for another participant's behavior.

## Class

logic

## Root cause

Same underlying cache-invalidation gap as candidate 082, in a structurally separate code path.
`InMemDicomObject::len` caches the object's own encoded byte length, used only when the object was
read as a sequence item with an explicit (non-undefined) length. `apply`'s `Nested`-step branch
descends into nested items by reassigning `obj = items.get_mut(*item as usize)?` (or
`push`/`last_mut()` for a constructive action) - it never marks the item being left behind as
having a stale `len`. Once the loop reaches the last `AttributeSelectorStep::Tag`, it calls
`obj.apply_leaf(*tag, action)` on the then-current (innermost) object; that call's mutating arms
set `self.len = Length::UNDEFINED` only on *that* object, not on any ancestor item traversed on the
way there. With one level of nesting this happens to be harmless, because the leaf-owning object
*is* the one bound to `obj` when `apply_leaf` runs. With two or more levels, every ancestor item
strictly above the leaf-owning one is left with its original, now-stale `len`.

On write, `object/src/tokens.rs`'s `IntoTokens` impls map each sequence item to
`AsItem(o.length(), o)`, so the stale `len` flows straight into the `ItemStart` token emitted for
that item. `DataSetWriter::write`'s `ExplicitLengthSqItemStrategy::NoChange` branch (the
non-default strategy; the crate's default, `SetUndefined`, discards all cached item/sequence
lengths on write specifically to avoid this class of bug) writes that stale `len` byte-for-byte
into the wire header with no recomputation from what is actually written afterward. Because the
leaf's real encoded size grew, item0's real encoded content also grew (46 bytes vs. the stale 38),
so the produced item boundary no longer matches its real content; re-parsing then fails with
`InconsistentSequenceEnd` once `update_seq_delimiters` compares the promised boundary against the
real cumulative read position.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Mirror 082's fix inside `apply`'s own traversal: invalidate the just-entered item's own `len`
immediately after rebinding `obj` at each `Nested` step, right after the `push`/`get_mut` branch:

```rust
// object/src/mem.rs, in apply's Nested-step branch, right after re-binding `obj`:
obj.len = Length::UNDEFINED;
```

This invalidates every item along the selector path as it's traversed (not just the immediate
parent), the same way 082's fix does for `entry_at_mut` - a leaf nested three or more levels deep
correctly invalidates every ancestor item's cached length, not just the nearest one. It is
additive to what `apply_leaf`'s own arms already do for the leaf-owning object, so no existing
behavior changes for single-level selectors (verified: the original one-level repro above still
round-trips correctly with the fix applied, exactly as it did without it).

Verified by re-running the two-level repro (debug build, in-tree) and the standalone `repro/`'s
`just test` (release build, against a fresh clone of the base commit) against the patched source:
the writer now emits `ItemStart{len: Length(Undefined)}` for item0 (with a matching `ItemEnd`
delimiter), and re-parsing the output succeeds, reconstructing the mutated value correctly. Re-ran
3/3 times with identical (fixed) output both in-tree and via `just test`.

- Project test suite (`cargo test -p dicom-object --lib`): 75 passed / 0 failed, both before and
  after the fix (no pre-existing test covered this path). `cargo test --workspace --lib`: 351
  passed, 2 ignored, 0 failed, identical before and after (zero regressions) - matches candidate
  082's own totals, since no other test in the workspace exercises `apply` with more than one
  level of sequence nesting.

## What's not done / open questions

- As with 082, a sequence's own outer `DataSetSequence::length` field (separate from any
  individual item's `len`) was not checked - `apply`'s constructive actions only ever push a new
  item or mutate an existing leaf, never change the sequence's own explicit length field directly.
- As with 082, this bug's reachability is opt-in: it only bites a caller that explicitly requests
  `ExplicitLengthSqItemStrategy::NoChange` on write and then also uses `apply`/`AttributeOp` on a
  leaf nested two or more sequence levels deep, where at least the outer item was read with an
  explicit (defined) length.
- Checked: `object/src/mem.rs` has exactly 4 loops matching on `AttributeSelectorStep::Nested`
  (`grep -n "AttributeSelectorStep::Nested"`) - the read-only `value_at`/`entry_at` (no mutation,
  not affected), `entry_at_mut` (082's target; its fix is recorded only in that candidate's own
  `patch.diff`, not applied to the tree, per this skill's "always revert" rule - so it is still
  present, unpatched, in the current codebase), and `apply` (this candidate). No other traversal
  loop of this shape exists in this file.
