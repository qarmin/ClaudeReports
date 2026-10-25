# `TreeItem::create_child()`/`remove_child()` race on the unguarded `children_cache`

Continues the external thread-safety sweep's `Control` widget lead (candidates 039 `Label`, 040
`LineEdit`, 041 `ItemList`). `Tree`/`TreeItem` use an intrusive doubly-linked list
(`parent`/`prev`/`next`/`first_child`/`last_child`, all plain `TreeItem*`) plus a lazily-built
`LocalVector<TreeItem*> children_cache` for fast indexed access - all completely unguarded, zero
`Mutex`/`_THREAD_SAFE_CLASS_` anywhere in either class.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/tree.h`/`.cpp` - confirmed zero `Mutex` anywhere in `TreeItem`; `create_child()` pushes the new item into `children_cache` (`LocalVector<TreeItem*>`) and relinks the intrusive `prev`/`next`/`first_child`/`last_child` pointers, `remove_child()` does the inverse, `get_child_count()`/`get_first_child()`/`get_last_child()` read the same fields, all with no lock | Static read | Confirmed `create_item`/`remove_child`/`get_child_count`/`get_first_child` are all `ClassDB`-bound, reachable from GDScript |
| GDScript-level repro (`repro/`: a `Tree` with one root `TreeItem`, 3 threads repeatedly calling `tree.create_item(root)` then conditionally `root.remove_child(root.get_first_child())`, 3 threads repeatedly reading `root.get_child_count()`/`root.get_first_child()`/`.get_next()` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/042`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - 3 real ASan reports (2 `heap-buffer-overflow`, 1 `heap-use-after-free`), all inside `LocalVector<TreeItem*>::push_back()`/`::remove_at()` - concurrent `create_child()`/`remove_child()` calls on the same parent racing each other's `children_cache` mutation |
| Fix (`_THREAD_SAFE_CLASS_` on `TreeItem`'s header + `_THREAD_SAFE_METHOD_` on `create_child()`, `remove_child()`, `get_child_count()`, `get_first_child()`, `get_last_child()` - the methods that mutate/read `children_cache`/`first_child`/`last_child` on `this` as the parent), incremental rebuild (ccache-warm hardlinked worktree, ~2m54s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~2m29s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (3 distinct real ASan crashes, all inside the same
`children_cache` push_back/remove_at code path), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `Tree`/`TreeItem` classes through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from GDScript
`Thread`s calling `create_item()`/`remove_child()` concurrently on a shared parent `TreeItem`.

## Class

`memory-safety` (heap corruption on a `LocalVector<TreeItem*>` - concurrent `create_child()`/
`remove_child()` calls on the same parent race each other's resize of the shared `children_cache`,
manifesting as either a heap-buffer-overflow (push past the allocated capacity mid-resize) or a
heap-use-after-free (reading/freeing a slot another thread's concurrent `remove_at()` already
invalidated) depending on timing).

## Root cause

```cpp
TreeItem *TreeItem::create_child(int p_index) {
	TreeItem *ti = memnew(TreeItem(tree));
	...
	TreeItem *item_prev = nullptr;
	TreeItem *item_next = first_child;
	if (p_index < 0 && last_child) {
		item_prev = last_child;
	} else {
		if (!children_cache.is_empty()) {
			...
			item_next = children_cache[idx];   // <-- unguarded read
			...
		}
		...
	}
	...
}
```

`children_cache` (a lazily-built `LocalVector<TreeItem*>` mirroring the intrusive linked list for
fast indexed access) and the intrusive `first_child`/`last_child`/`prev`/`next` pointers are all
plain members with zero synchronization anywhere in either class. Two threads calling
`create_child()`/`remove_child()` concurrently on the same parent race each other's
`children_cache.push_back()`/`.remove_at()` - confirmed via ASan: `heap-buffer-overflow ... in
LocalVector<TreeItem*, ...>::push_back()`, with a full stack through `TreeItem::create_child() <-
Tree::create_item()`, and a second, independently-triggered `heap-use-after-free ... in
LocalVector<TreeItem*, ...>::remove_at()`.

Reachability: `create_item`/`remove_child`/`get_child_count`/`get_first_child` are all bound,
GDScript-callable public methods - `Tree`/`TreeItem` are ordinary scene nodes/objects, reachable
from any thread with no special setup once the `Tree` is added to the scene tree.

## Introduced by

bca0d36fe662477eb787b4ddc9b0b69ab4603393 "Improve TreeItem API and allow to move nodes"
(2021-03-07) - the commit that introduced `children_cache` itself (confirmed via `git log
-S"children_cache"`); a later commit (1ad3b9986ed76b2ffc83f84319db4921722e1a42 "Speed up deletion
in large trees via the Scene Tree Dock", 2025-08-10) changed its container type from `Vector` to
`LocalVector` but didn't change the locking situation either way - it was never guarded.

## The fix

`+10/-0` against `scene/gui/tree.cpp`, `+3/-0` against `scene/gui/tree.h` (see `patch.diff`): add
`_THREAD_SAFE_CLASS_` next to `TreeItem`'s private members, and `_THREAD_SAFE_METHOD_` to
`create_child()`, `remove_child()`, `get_child_count()`, `get_first_child()`, `get_last_child()` -
the methods that directly mutate/read `children_cache`/`first_child`/`last_child` when called on
`this` as the parent.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed, with both a
heap-buffer-overflow and a heap-use-after-free observed).

- **Applied for real** in an isolated worktree (`.worktrees/042`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- **A real, documented architectural limitation, not an oversight**: `_THREAD_SAFE_CLASS_` creates
  a *per-instance* mutex. `get_next()`/`get_prev()` (reading `this->next`/`this->prev` on a
  *child*) are **not** locked by this fix, because the actual writes to a child's `next`/`prev`
  fields happen inside the *parent*'s `create_child()`/`remove_child()`, under the *parent's*
  mutex - a different object, a different lock. Locking `get_next()` with the child's own mutex
  would not provide mutual exclusion against the parent's concurrent write. Properly fixing this
  would need a single shared lock across an entire tree (e.g. stored on the `Tree` object itself,
  reachable from every `TreeItem` via its `tree` back-pointer) rather than per-`TreeItem` locks -
  a larger architectural change out of scope for this candidate. This session's repro's reader
  threads did call `get_next()` but the 3/3 crashes observed all landed in the parent-side
  `children_cache` push_back/remove_at path, not in a `get_next()`-specific failure mode - the
  cross-object gap remains real but wasn't what this specific repro happened to hit.
- `Tree::create_item(nullptr, ...)` (creating a *root* item) touches `Tree`'s own `root` field,
  not `TreeItem`'s - a different race on a different object, not covered by this fix and not
  exercised by this candidate's repro (which always passes a non-null parent).
- `Tree` itself has 161 methods, `TreeItem` has 146 - the vast majority of both remain completely
  unguarded (selection state, column/cell data, scrolling, drag-and-drop, etc.).
- **6 more `Control` widgets remain from the same external sweep**, not yet independently
  confirmed: `CodeEdit`, `GraphEdit`, `OptionButton`, `SplitContainer`, `TabBar`, `TextEdit`.
