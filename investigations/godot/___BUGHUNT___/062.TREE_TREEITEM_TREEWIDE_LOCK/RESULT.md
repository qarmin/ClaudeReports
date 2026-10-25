# `Tree`/`TreeItem`'s cross-object lock gap - closed with a tree-wide mutex instead of per-`TreeItem` ones

From the coordinator's priority-3 list: the `get_next()`/`get_prev()` gap candidate 042 explicitly
documented as "needs a tree-wide shared lock design, not a quick fix... an architectural change, not
a quick follow-up fix." Investigated whether that architectural change is actually implementable in
bounded scope - it is, and a working fix was found and verified, not just designed on paper.

## The gap, restated

Candidate 042 fixed `TreeItem::create_child()`/`remove_child()`/`get_child_count()`/
`get_first_child()`/`get_last_child()` with a **per-`TreeItem`** `_THREAD_SAFE_CLASS_` mutex. But
`get_next()`/`get_prev()` read `this->next`/`this->prev` - fields that are *written* by the
**parent's** `create_child()`/`remove_child()`, under the **parent's own, separate** mutex instance.
A per-child mutex can never provide mutual exclusion against a different object's (the parent's)
concurrent write. Candidate 042 left this explicitly undone and called it an architectural gap.

## The fix idea

`TreeItem` already has a `Tree *tree` back-pointer (`tree.h:166`), and `Tree` already declares
`friend class TreeItem;` (`tree.h:495`, pre-existing, not added by this candidate). This makes a
**single, tree-wide lock** straightforward: move the mutex from `TreeItem` to `Tree` itself
(`_THREAD_SAFE_CLASS_` on `Tree`), and have every structural `TreeItem` method lock
`tree->_thread_safe_` (accessible via friendship) instead of its own instance's mutex. Since every
`TreeItem` that's actually part of a `Tree` shares the *same* `tree` pointer, this correctly
serializes a parent's writes against a (different object) child's reads - the exact thing a
per-instance design structurally cannot do. A small fallback mutex (`TreeItem::_fallback_mutex`)
covers the rare case of a standalone `TreeItem` not yet attached to any `Tree` (`tree == nullptr`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: confirmed `TreeItem::tree` and `Tree`'s `friend class TreeItem;` already exist, unused by any locking scheme today - the plumbing for a tree-wide lock was already present, just not wired up | Static read | Design is implementable without restructuring the classes' relationship, just moving where the mutex lives and which object's lock each method takes |
| GDScript-level repro (`repro/`: a `Tree` with a root `TreeItem` and 5 stable children, one of them (`middle_child`) held onto separately; 6 writer threads repeatedly `create_child(0)`+`remove_child()` on the root, 6 reader threads calling `middle_child.get_next()`/`get_prev()` plus `root.get_child_count()`/`get_first_child()`/`get_last_child()`, for 8s), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly, no rebuild needed for the unfixed run) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 20 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | A smaller-scale first attempt (3+3 threads/4s) saw **0/3 crashes** - not aggressive enough. Escalating to 6+6 threads/8s got **5/6 crashes**, three distinct signatures: `heap-use-after-free` in `LocalVector<TreeItem*>::remove_at()`, in `Span<TreeItem*>::find()` (reached through `_unlink_from_tree()`'s `parent->children_cache.erase(this)`), confirming real, severe corruption on vanilla HEAD (candidate 042's own fix was never merged, same fix-verify-revert workflow as every other candidate - this tree has *zero* locking in `Tree`/`TreeItem` today) |
| Fix: moved the mutex to `Tree` (`_THREAD_SAFE_CLASS_`), added `TreeItem::_fallback_mutex` for the no-tree case, and added `MutexLock lock(tree ? tree->_thread_safe_ : _fallback_mutex);` to `create_child()`, `remove_child()`, `get_child_count()`, `get_first_child()`, `get_last_child()`, `get_next()`, `get_prev()`, `get_parent()` - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed, and a real architectural fix found** - not just an investigation documenting "no bounded
fix found." 5/6 unfixed crashes (3 distinct signatures) at sufficient contention, 10/10 clean with
the tree-wide lock.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `Tree`/`TreeItem`'s public GDScript API.

## Class

`memory-safety` (concurrent unsynchronized `LocalVector<TreeItem*>` mutation/traversal on
`children_cache`, and unsynchronized raw-pointer linked-list mutation/reads on `first_child`/
`last_child`/`next`/`prev` - same underlying shape as candidate 042's own finding, this candidate
closes the specific gap 042 identified rather than re-finding the same crash).

## Root cause

See candidate 042's own `RESULT.md` for the base mechanism (concurrent `create_child()`/
`remove_child()` racing `children_cache`'s `push_back()`/`remove_at()`). This candidate's addition:
`get_next()`/`get_prev()` read `this->next`/`this->prev`, fields mutated by a *different* object's
(the parent's) `create_child()`/`remove_child()` - a cross-object race no per-instance mutex design
can close, confirmed both by code reading (candidate 042's own analysis) and by this candidate's own
repro crashing on vanilla HEAD under exactly this shape of concurrent access.

## Introduced by

Same as candidate 042 - not independently dated, the class has never had any locking at any point in
its tracked history.

## The fix

`+16/-0` in `scene/gui/tree.cpp`, `+9/-0` in `scene/gui/tree.h` (see `patch.diff`) - a complete,
standalone patch (does not depend on candidate 042's own patch, supersedes it):

- `Tree` (`tree.h`): added `_THREAD_SAFE_CLASS_` as a private member, next to the existing
  `friend class TreeItem;`.
- `TreeItem` (`tree.h`): added `mutable Mutex _fallback_mutex;` for the standalone (`tree == nullptr`)
  case.
- `TreeItem::create_child()`, `remove_child()`, `get_child_count()`, `get_first_child()`,
  `get_last_child()`, `get_next()`, `get_prev()`, `get_parent()` (`tree.cpp`): each now starts with
  `MutexLock lock(tree ? tree->_thread_safe_ : _fallback_mutex);` - locking the *tree's* mutex when
  attached to one, falling back to a per-item mutex only when standalone. `Mutex` is recursive, so
  `remove_child()` calling `_unlink_from_tree()` (which calls the now-locked `get_prev()`) and
  `create_child()`/`remove_child()` being called repeatedly from the same thread via
  `clear_children()`/`move_before()`/`move_after()` don't deadlock.

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 5/6 crashing unfixed at sufficient
contention). Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied -
identical to the established baseline.

## What's not done / open questions

- **Scope was deliberately limited to the 8 methods above** (the ones candidate 042 already
  identified plus the `get_next()`/`get_prev()`/`get_parent()` gap). `move_before()`, `move_after()`,
  `clear_children()`, `_change_tree()`, `validate_cache()`, `get_children()`, `get_child()`,
  `get_visible_child_count()`, `_get_prev_in_tree()`/`_get_next_in_tree()`, and `set_collapsed()`/
  `uncollapse_tree()` all touch the same structural fields (confirmed by a body-scanning script
  during investigation) but were **not** given the new lock in this candidate - a natural next step
  for whoever picks this class back up, now that the tree-wide mutex exists and the pattern is
  established (`MutexLock lock(tree ? tree->_thread_safe_ : _fallback_mutex);`).
- `Tree::create_item(nullptr, ...)` (creating a *root* item) touches `Tree::root` directly, not
  through any of the now-locked `TreeItem` methods - not covered, not exercised by this candidate's
  repro (which always creates a root once at setup, not concurrently).
- `Tree` itself has 161 methods, `TreeItem` has 146 - selection state, column/cell data, scrolling,
  drag-and-drop, etc. remain completely unaudited.
- Also found (while reviewing candidate 061's `Image::copy_internals_from()`) a conceptually
  identical cross-object gap in that method - flagged there, not fixed, same underlying lesson: a
  per-instance lock never protects against a *different* instance's concurrent mutation of the
  fields being read.
- Next per the coordinator's priority-3 list: the ~22 remaining unaudited `_THREAD_SAFE_CLASS_`
  files, with `scene_tree.h`/`rendering_device.h` flagged as highest-value.
