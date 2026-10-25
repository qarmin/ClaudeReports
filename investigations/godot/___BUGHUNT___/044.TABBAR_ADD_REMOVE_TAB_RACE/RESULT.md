# `TabBar` has no locking anywhere in the class

Continues the external thread-safety sweep's `Control` widget lead (candidates 039 `Label`, 040
`LineEdit`, 041 `ItemList`, 042 `Tree`, 043 `SplitContainer`). `TabBar` keeps its tab storage
directly in `Vector<Tab> tabs` (`scene/gui/tab_bar.h:119`), touched by ~80 public methods, with
zero `Mutex`/`_THREAD_SAFE_CLASS_` anywhere in the header - the exact same shape as candidates
018/030/041's confirmed `PopupMenu`/`ItemList` bugs.

(Note: `OptionButton`, also on the original sweep list, was checked first and found to forward all
item storage directly into its own internal `PopupMenu* popup` - already fully thread-safe since
candidate 030's fix. `OptionButton` itself has no local `CowData`-backed container (`_cached_size`
is a plain `Vector2` written only via `call_deferred()` from the main thread), so no candidate was
opened for it; removed from the backlog.)

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/tab_bar.h`/`.cpp` - confirmed zero `Mutex` anywhere in the header; `add_tab()`/`remove_tab()`/`clear_tabs()` mutate `tabs` (`Vector<Tab>`) directly, `get_tab_count()`/`get_tab_title()`/etc. read it directly, all with no lock | Static read | Confirmed `add_tab`/`remove_tab`/`get_tab_title`/`tab_count` (property) are all `ClassDB`-bound, reachable from GDScript |
| GDScript-level repro (`repro/`: shared `TabBar` pre-populated with 10 tabs, added to tree, 3 threads looping `add_tab()` + conditional `remove_tab(0)`, 3 threads looping `tab_count` read + `get_tab_title(0)` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/044`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - a real ASan heap-use-after-free (`READ of size 8`, in a worker thread) plus `FATAL: Index p_index = N is out of bounds` traps in all 3 runs (a stale index read against a shrinking `tabs` array) |
| Fix: counted the method surface first (81 public methods, per the project's own "count first, >30-40 needs a scripted pass" policy) - reused the same scripted-insertion technique as candidate 041's `ItemList` to insert `_THREAD_SAFE_METHOD_` after the opening brace of every matching `TabBar::<method>` definition (79 of 80 matched; the remaining 1, `set_switch_on_release`, is a trivial inline bool setter defined in the header - left unguarded), plus `_THREAD_SAFE_CLASS_` on the header. Incremental rebuild (ccache-warm hardlinked worktree, ~2m56s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~1m52s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (1 real ASan heap-use-after-free plus `FATAL`
bounds-check traps across all 3 runs), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `TabBar` class through nothing but the public GDScript API.
No second participant is involved - a same-process crash reachable from GDScript `Thread`s calling
`add_tab()`/`remove_tab()`/`get_tab_title()` concurrently on a shared `TabBar`.

## Class

`memory-safety` (heap corruption on a `Vector<Tab>` - concurrent `add_tab()`/`remove_tab()` calls
race each other's resize of the shared backing `CowData`, and reader threads calling
`get_tab_count()`/`get_tab_title()` can observe a stale size and then index past the actual
(already-shrunk) array; manifests as either an ASan heap-use-after-free or a `FATAL`
bounds-check trap depending on timing - same bimodal signature already documented for candidates
018/027/030/037/041's confirmed container races).

## Root cause

`tabs` (`Vector<Tab>`, `CowData`-backed) is touched directly by ~80 public methods with zero
synchronization anywhere in the class:

```cpp
void TabBar::add_tab(const String &p_str, const Ref<Texture2D> &p_icon) {
	Tab t;
	...
	tabs.push_back(t);              // <-- unguarded resize
	...
}

int TabBar::get_tab_count() const {
	return tabs.size();              // <-- unguarded read
}
```

Two threads concurrently calling `add_tab()`/`remove_tab()` race each other's `CowData` resize, and
a reader thread calling `get_tab_count()` followed by `get_tab_title(idx)` can read a now-stale
`idx` against an array that another thread has since shrunk - confirmed via a real ASan
heap-use-after-free and Godot's own bounds check (`FATAL: Index p_index = N is out of bounds`),
same underlying race, two different ways of catching it.

Reachability: `add_tab`/`remove_tab`/`get_tab_title`/`tab_count` (and ~76 other methods) are all
bound, GDScript-callable public methods - `TabBar` is an ordinary `Control`-derived node, reachable
from any thread with no special setup once added to the tree.

## Introduced by

not determined - `scene/gui/tab_bar.cpp` has existed since the initial open-source commit
(0b806ee0fc9097fa7bda7ac0109191c9c5e0a1ac "GODOT IS OPEN SOURCE", 2014-02-09; confirmed via `git
log --follow`) and has never had any locking since.

## The fix

`+158/-0` against `scene/gui/tab_bar.cpp`, `+3/-0` against `scene/gui/tab_bar.h` (see
`patch.diff`): add `_THREAD_SAFE_CLASS_` next to the private members, and `_THREAD_SAFE_METHOD_` to
all 79 public `TabBar::` methods that have an out-of-line definition in the `.cpp` (found by name
from the header's `public:` sections, inserted via the same scripted pass candidate 041 used for
`ItemList`). The 1 unmatched method (`set_switch_on_release`) is a trivial inline bool setter in
the header, left unguarded by the same convention used throughout this workspace for plain-field
accessors that don't touch `tabs`.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed, with both an ASan
and a FATAL-trap failure mode observed).

- **Applied for real** in an isolated worktree (`.worktrees/044`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- Like candidate 041's `ItemList`, this candidate did the **full** method sweep in one pass - no
  known remaining gap in `TabBar` itself from this session's perspective, beyond the 1 trivial
  inline setter noted above.
- **4 widgets remain** from the original external-sweep list: `CodeEdit`, `GraphEdit`, `TextEdit`
  (3 actually - `OptionButton` was checked and ruled out, see note above; `SplitContainer`/043 and
  `TabBar`/044 are now closed, leaving `CodeEdit`, `GraphEdit`, `TextEdit`).
