# object-build-object-premature-end-masked-by-caller

`InMemDicomObject::build_object` (`object/src/mem.rs`), when called with `in_item=true` (i.e.
while building the contents of a single sequence item), has a `while let Some(token) =
dataset.next()` loop whose only early-return path is `DataToken::ItemEnd if in_item`. If the
token iterator is exhausted (`dataset.next()` returns `None`) before that item delimiter is
ever seen, the loop simply ends and the function falls through to its normal `Ok(InMemDicomObject
{ .. })` return - silently treating a truncated/malformed item as if it were complete, with no
error of any kind. This is structurally the exact same class of gap that `build_sequence`
(a few dozen lines below in the same file) already explicitly guards against (its own trailing
`PrematureEndSnafu.fail()` after its `while` loop, for the analogous "iterator exhausted before
`SequenceEnd`" case).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-068): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/build-object-premature-end` (commit `3b7550e3`) adding exactly this guard | Confirmed unmerged |
| Temporary in-tree `#[test]` in `object/src/mem.rs` (reverted before write-up): call the private `build_object` function directly with `in_item=true` and a token stream that ends without `ItemEnd` | `cargo test -p dicom-object build_object_premature_end_in_item_repro` | Confirms the isolated function-level defect: returns `Ok(..)` with an incomplete object, as described |
| Grepped every call site of `build_object` in `object/src/mem.rs` to find every place `in_item=true` is passed, to determine real-world reachability through the crate's *public* API (`build_object` itself is a private function) | `grep -n "build_object("` across the file | Found exactly **one** call site passing `in_item: true` - inside `build_sequence`, once per sequence item (`items.push(Self::build_object(&mut *dataset, dict.clone(), true, len, None, None)?);`). Every other call site (the top-level dataset-reading entry points) passes `in_item: false` |
| Read `build_sequence`'s own loop structure in full to determine what happens immediately after that one call returns | Static code reading | `build_sequence`'s `while let Some(token) = dataset.next()` loop calls `dataset.next()` again on the *same shared iterator* immediately after `build_object(..., true, ..)?` returns. Since both functions consume the same iterator, if that iterator became exhausted *inside* `build_object` (triggering the untested fallthrough), `build_sequence`'s own next `dataset.next()` call is guaranteed to also observe that same exhaustion (an iterator, once exhausted, stays exhausted) - which `build_sequence`'s own pre-existing, already-correct trailing check (`PrematureEndSnafu.fail()`) catches and turns into an `Err` regardless of what `build_object` itself returned |
| Constructed real, hand-crafted Explicit-VR-LE wire bytes for exactly this scenario (an SQ element with undefined length, one Item with undefined length, one short element inside it, then true end-of-stream with **no** Item Delimitation Item and **no** Sequence Delimitation Item) and parsed them through the crate's actual public `InMemDicomObject::read_dataset_with_ts` entry point (scratch check outside `___BUGHUNT___`, not part of this candidate's deliverable, since the finding here is a negative result with no repro artifact to preserve) | Ran directly against the current (unpatched) tree | **Already correctly rejected**: `Err(PrematureEnd)` ("Premature data set end") - confirming empirically that `build_sequence`'s own existing check catches this exact scenario end-to-end through the real public API, before the missing `build_object` guard would ever matter |

## Status

**Not reproducible** - the isolated function-level gap is real (confirmed via direct unit-level
testing of the private function), but it has exactly one caller in the entire crate, and that
caller already independently detects and errors on every input that could trigger the gap,
because both functions share the same token iterator and iterator exhaustion is a one-way,
globally-observable state. No sequence of bytes reachable through this crate's public read API
can distinguish the buggy behavior from the fixed behavior - both return the same `Err
(PrematureEnd)` in every case tested.

## Class

n/a

## Root cause

Not applicable (no independently observable bug) - `build_object`'s `in_item=true` branch is
missing a guard that would, in isolation, allow it to silently succeed on truncated input, but
its sole caller (`build_sequence`) already performs the equivalent check on the very same
iterator immediately afterward, making the two functions' error detection redundant rather than
complementary for every path currently reachable from the public API.

## The fix

N/A - no independently observable bug found via the public API, so no fix was applied to the
live tree this session. That said, the unmerged branch's change is still a reasonable, cheap
defensive-consistency improvement worth adopting on its own merits (it makes `build_object`
correct in isolation, matching `build_sequence`'s already-correct behavior, and protects against
any *future* caller that might invoke `build_object(in_item=true, ..)` from a different context
where the "shared iterator" argument above wouldn't hold) - just not something this session
applied, since CONFIRMATION.md's bar requires an actually-reproducible behavioral difference
before writing up a fix.

## What's not done / open questions

None - the reachability argument was verified exhaustively (every call site enumerated, the
sole `in_item=true` caller's own loop structure read in full, and the conclusion double-checked
empirically against the real public API with real wire bytes).
