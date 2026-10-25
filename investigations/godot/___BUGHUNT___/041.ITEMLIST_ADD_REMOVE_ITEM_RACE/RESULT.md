# `ItemList` has no locking anywhere in the class

Continues the external thread-safety sweep's `Control` widget lead (candidates 039 `Label`, 040
`LineEdit`). `ItemList` keeps its item storage directly in `Vector<Item> items` (`scene/gui/
item_list.h:124`), touched by ~97 public methods, with zero `Mutex`/`_THREAD_SAFE_CLASS_` anywhere
in the header - the exact same "plain mutable container, zero locking, reachable off the main
thread" shape as candidates 018/030's confirmed `PopupMenu` bug, just a different container type
and class.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/item_list.h`/`.cpp` - confirmed zero `Mutex` anywhere in the header; `add_item()`/`remove_item()`/`clear()` mutate `items` (`Vector<Item>`) directly, `get_item_count()`/`get_item_text()`/etc. read it directly, all with no lock | Static read | Confirmed `add_item`/`remove_item`/`get_item_text`/`item_count` (property) are all `ClassDB`-bound, reachable from GDScript |
| GDScript-level repro (`repro/`: shared `ItemList` pre-populated with 10 items, added to tree, 3 threads looping `add_item()` + conditional `remove_item(0)`, 3 threads looping `item_count` read + `get_item_text(0)` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/041`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - 2 real ASan heap-use-after-frees inside `CowData<ItemList::Item>::size()`/a further unresolved frame (one with a full stack through `Vector<ItemList::Item>::size() <- ItemList::get_item_text()`), plus multiple `FATAL: Index p_index = N is out of bounds` traps (a stale index read against a shrinking `items` array) across all 3 runs |
| Fix: counted the method surface first (97 public methods touching `items` across the header's two `public:` sections, per the project's own "count first, >30-40 needs a scripted pass" policy) - wrote a small Python script to insert `_THREAD_SAFE_METHOD_` after the opening brace of every `ItemList::<method>` definition in the `.cpp` matching that list (94 of 96 matched; the remaining 2, `get_h_scroll_bar`/`get_v_scroll_bar`, are trivial pointer getters defined inline in the header - left unguarded, matching the project's convention for plain-pointer accessors), plus `_THREAD_SAFE_CLASS_` on the header. Incremental rebuild (ccache-warm hardlinked worktree, ~2m50s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~1m38s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (2 real ASan heap-use-after-frees plus multiple
`FATAL` bounds-check traps across all 3 runs), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `ItemList` class through nothing but the public GDScript
API. No second participant is involved - a same-process crash reachable from GDScript `Thread`s
calling `add_item()`/`remove_item()`/`get_item_text()` concurrently on a shared `ItemList`.

## Class

`memory-safety` (heap corruption on a `Vector<ItemList::Item>` - concurrent `add_item()`/
`remove_item()` calls race each other's resize of the shared backing `CowData`, and reader threads
calling `get_item_count()`/`get_item_text()` can observe a stale size and then index past the
actual (already-shrunk) array; manifests as either an ASan heap-use-after-free or a `FATAL`
bounds-check trap depending on timing - same bimodal signature already documented for candidates
018/027/030/037's confirmed container races).

## Root cause

```cpp
void ItemList::clear() {
	...
	items.clear();              // <-- unguarded resize
	...
}

int ItemList::get_item_count() const {
	return items.size();        // <-- unguarded read
}

Vector<int> ItemList::get_item(int p_idx);  // (illustrative - actual accessors index items[] directly)
```

`items` (`Vector<Item>`, `CowData`-backed) is touched directly by ~97 public methods with zero
synchronization anywhere in the class. Two threads concurrently calling `add_item()`/`remove_item()`
race each other's `CowData` resize, and a reader thread calling `get_item_count()` followed by
`get_item_text(idx)` can read a now-stale `idx` against an array that another thread has since
shrunk - confirmed via ASan (`heap-use-after-free ... in CowData<ItemList::Item>::size()`, reached
through `Vector<ItemList::Item>::size() <- ItemList::get_item_text()`) and via Godot's own bounds
check (`FATAL: Index p_index = N is out of bounds`, same underlying race, caught by the engine's
own `CRASH_BAD_INDEX`-style guard instead of ASan).

Reachability: `add_item`/`remove_item`/`get_item_text`/`item_count` (and ~93 other methods) are all
bound, GDScript-callable public methods - `ItemList` is an ordinary `Control`-derived node,
reachable from any thread with no special setup once added to the tree.

## Introduced by

not determined - `scene/gui/item_list.cpp` has existed since its own creation commit
(9acab32daaea38e09c4d74d5f0340479d3dd41bd "new file dialog!", 2015-06-06; confirmed via `git log
--follow`) and has never had any locking since.

## The fix

`+188/-0` against `scene/gui/item_list.cpp`, `+3/-0` against `scene/gui/item_list.h` (see
`patch.diff`): add `_THREAD_SAFE_CLASS_` next to the private members, and `_THREAD_SAFE_METHOD_` to
all 94 public `ItemList::` methods that have an out-of-line definition in the `.cpp` (found by name
from the header's `public:` sections, inserted via a small scripted pass rather than by hand -
same technique candidate 030 used for `PopupMenu`'s ~80 remaining methods). The 2 unmatched methods
(`get_h_scroll_bar`/`get_v_scroll_bar`) are trivial inline pointer getters in the header, left
unguarded by the same convention used throughout this workspace for plain-pointer accessors that
don't touch `items` at all.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed, with both an ASan
and a FATAL-trap failure mode observed).

- **Applied for real** in an isolated worktree (`.worktrees/041`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- Unlike candidates 039/040's deliberately narrow `Label`/`LineEdit` fixes, this candidate did the
  **full** 94-method sweep in one pass (matching `PopupMenu`'s eventual full treatment in candidate
  030) - no known remaining gap in `ItemList` itself from this session's perspective, beyond the 2
  trivial scroll-bar pointer getters noted above.
- **7 more `Control` widgets remain from the same external sweep**, not yet independently
  confirmed: `CodeEdit`, `GraphEdit`, `OptionButton`, `SplitContainer`, `TabBar`, `TextEdit`, `Tree`
  (originally 9 remaining after `Label`/039, now 7 after `LineEdit`/040 and `ItemList`/041 also
  closed). Per candidates 039/040's own lesson, each likely has its own distinct unguarded
  mechanism - confirm each independently rather than assuming this patch shape transfers. `Tree` is
  a reasonable next pick (shares no code with `ItemList` despite the similar name).
