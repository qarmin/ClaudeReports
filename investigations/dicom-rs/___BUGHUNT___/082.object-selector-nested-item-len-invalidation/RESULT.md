# object-selector-nested-item-len-invalidation

`object/src/mem.rs`'s `entry_at_mut` (used by `update_value_at`) walks an `AttributeSelector`
down into nested sequence items by reassigning a local `obj` binding at each `Nested` step, but
never touches that intermediate item's own cached `len: Length` field - only `update_value_at`'s
caller-level `self.len = Length::UNDEFINED` at the very end invalidates the *root* object's cache.
Source hint: `origin/fix/selector-ancestor-len-invalidation` (commit 3181d529), carried over from
prior git-archaeology code reading recorded in `IDEAS.md`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `object/src/mem.rs` (`entry_at_mut`/`update_value_at`), `object/src/tokens.rs`, `parser/src/dataset/mod.rs` (`ItemTokens::new`, `DataElementTokens`), `parser/src/dataset/write.rs` (`DataSetWriter::write`), `parser/src/dataset/read.rs` (`update_seq_delimiters`) | Confirmed the full mechanism: `entry_at_mut` never invalidates an intermediate item's `len`; `o.length()` (that stale `len`) is threaded verbatim into `ItemTokens::new`'s `len` parameter; `DataSetWriter::write`'s `ExplicitLengthSqItemStrategy::NoChange` branch writes that `len` into the wire `ItemStart` header with no recomputation from actual bytes written; on re-read, `update_seq_delimiters` compares `base_offset + len` against the real cumulative position and fails hard (`InconsistentSequenceEndSnafu`) once the mutated leaf's real size overruns the stale boundary |
| Direct reproduction (debug build) | An `object/examples/selector_bug.rs` scratch binary (removed before finishing this candidate) built a well-formed Explicit VR Little Endian data set by hand: one SQ element (undefined length) containing one item with an EXPLICIT length of 10 bytes, whose sole content is `PatientName="AB"` (2-byte value, exactly filling the declared 10-byte item). Read via the real public `InMemDicomObject::read_dataset_with_ts`, then `update_value_at` the leaf to a 16-byte string, then `write_dataset_with_ts_options` with `ExplicitLengthSqItemStrategy::NoChange`, then re-read the output via the same real reader | Reproduced deterministically 3/3 runs: writer emits an `ItemStart{len: Length(10)}` verbatim (stale) while writing 24 real bytes of item content; re-parsing the writer's own output fails with `InconsistentSequenceEnd { end_of_sequence: 30, bytes_read: 44 }` |

## Status

**Confirmed**. Reproduces deterministically 3/3 runs (debug build) and 3/3 runs of the standalone
`repro/` (release build) - no randomness or timing involved anywhere in the mechanism, so it is
expected to reproduce 100% of the time given the same input and the same
`ExplicitLengthSqItemStrategy::NoChange` write option.

**Fully reproducible**: **Yes**. The finding is about a single participant's own internal
cache-invalidation bug, not about how it reacts to another participant's input - the "attacker"
here is the caller's own prior sequence of API calls (`read_dataset_with_ts` -> `update_value_at`
-> `write_dataset_with_ts_options` -> `read_dataset_with_ts` again), all through this crate's own
real public API, with no second participant to fake. The one hand-constructed artifact (the
initial 38-byte input) is an ordinary, fully well-formed DICOM data set using a legal (if less
common) encoding choice (explicit, non-undefined item length) - equivalent to a real file handed
to `read_dataset_with_ts`, not a stand-in for another participant's behavior.

## Class

logic

## Root cause

`InMemDicomObject`'s `len` field caches the object's own encoded byte length, used only when the
object is a sequence item read with an explicit (non-undefined) length. `entry_at_mut`'s selector
walk descends into nested items by simply reassigning `obj = items.get_mut(*item as usize)?` at
each `Nested` step - it never marks that intermediate item's own `len` as stale. `update_value_at`
mutates the leaf in place via `entry.update_value(f)` and then sets only `self.len = UNDEFINED`
on the *outermost* object it was called on - never on any ancestor item traversed on the way to
the leaf. On write, `object/src/tokens.rs`'s `IntoTokens` impls map each sequence item to
`AsItem(o.length(), o)` (`parser/src/dataset/mod.rs`), so the stale `len` flows straight into
`ItemTokens::new`'s `len` parameter and becomes the `ItemStart{len}` token emitted for that item.
`DataSetWriter::write`'s `ExplicitLengthSqItemStrategy::NoChange` branch (the *non-default*
strategy; the crate's default, `SetUndefined`, discards all cached item/sequence lengths on write
specifically to avoid this class of bug, per its own doc comment) writes that `len` byte-for-byte
into the wire header with no recomputation from what is actually written afterward. If the
mutated leaf's real encoded size differs from what the stale length promised, the produced item
boundary no longer matches the item's real content, and any later "encode -> decode" round trip
using `NoChange` will either fail outright (as reproduced here, via
`update_seq_delimiters`'s `InconsistentSequenceEnd` check when the new content overruns the
promised boundary) or silently truncate/misalign the following bytes (when the new content is
smaller instead of larger).

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Invalidate each intermediate item's own `len` (set it to `Length::UNDEFINED`) as `entry_at_mut`
descends past it, mirroring what the outer `update_value_at` already does for the root:

```rust
// object/src/mem.rs, in entry_at_mut's Nested-step branch, right after re-binding `obj`:
obj.len = Length::UNDEFINED;
```

This is sufficient because it invalidates *every* ancestor item along the selector path (not just
the immediate parent), since the loop invalidates `obj.len` at every `Nested` step before
descending further - a leaf nested three levels deep correctly invalidates all three ancestor
items' cached lengths, not just the nearest one. It only affects the mutation path
(`entry_at_mut` has exactly one caller, `update_value_at`), so it does not change behavior for any
read-only access (`entry_at`/`value_at`, which have their own separate, unmodified traversal
loops).

Verified by re-running `repro/`'s `just test` against the patched clone: the writer now emits
`ItemStart{len: Length(Undefined)}` for the mutated item (plus the matching `ItemEnd` delimiter,
per `DataSetWriter`'s existing undefined-length handling), and re-parsing the output succeeds.
Re-ran 3/3 times with identical (fixed) output.

- Project test suite: `cargo test -p dicom-object --lib`: 75 passed / 0 failed, both before and
  after the fix (no pre-existing test covered this path). `cargo test --workspace --lib`: 351
  passed, 2 ignored, identical before and after (zero regressions).

## What's not done / open questions

- The same staleness class could in principle also apply to a sequence's own outer
  `DataSetSequence::length` (a separate cached field from any individual item's `len`) if a
  similar selector-based API existed for inserting/removing whole items rather than mutating a
  leaf value in place; not checked here, since `update_value_at`/`entry_at_mut` only ever mutate
  an existing leaf's value, never the sequence's own item count or its own explicit length field.
- Not checked whether `dicom_core::ops`-based mutation (`ApplyOp`/`AttributeOp`, e.g.
  `InMemDicomObject::apply`) has an equivalent gap - it uses its own separate traversal, not
  `entry_at_mut`, and was out of scope for this candidate (it was not part of the seeded lead).
- As the seeded lead already noted, this bug's *reachability* is opt-in: the crate's default
  write strategy (`ExplicitLengthSqItemStrategy::SetUndefined`) already discards all explicit
  item/sequence lengths on write, sidestepping this entirely. It only bites a caller that
  explicitly requests `NoChange` via `write_dataset_with_ts_options`/`write_dataset_with_ts_cs_options`
  and then also uses `update_value_at` on a leaf nested inside an item whose own length was
  explicit (defined) at read time.
