# `TextEdit::set_text()`/`insert_text_at_caret()` race on the unguarded document model

Continues the external thread-safety sweep's `Control` widget lead (candidates 039 `Label`, 040
`LineEdit`, 041 `ItemList`, 042 `Tree`, 043 `SplitContainer`, 044 `TabBar`). `TextEdit` keeps its
document in a nested `Text` class wrapping `LocalVector<Line> text` (`scene/gui/text_edit.h:175`),
with zero `Mutex`/`_THREAD_SAFE_CLASS_` anywhere in the header. `TextEdit` is also `CodeEdit`'s
direct base class, so this candidate was picked to cover both widgets named in the external sweep
in one investigation.

`TextEdit` is one of Godot's largest GUI classes (hundreds of methods: carets, selections, syntax
highlighting, IME, undo/redo, gutters, folding, etc.) - this candidate's fix is **deliberately
scoped** to the confirmed crash path, not a full-class sweep (unlike candidates 030/041/044's
full-sweep treatment of smaller classes).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/text_edit.h`/`.cpp` - confirmed zero `Mutex` anywhere in the header; `set_text()` -> `_set_text()` -> `insert_text_at_caret()` -> `_insert_text()` -> `_base_insert_text()` -> `Text::insert()` -> `Text::set()` -> `Text::invalidate_cache()` all operate on the unguarded `text` member with no lock; `get_text()`/`get_line_count()`/`get_line()` read it directly, also unguarded | Static read | Confirmed `set_text`/`get_text`/`get_line_count`/`get_line`/`insert_text_at_caret` are all `ClassDB`-bound, reachable from GDScript |
| GDScript-level repro (`repro/`: shared `TextEdit`, added to tree, 3 threads alternating `.text` assignment between a 100-line string and a short one, 3 threads reading `get_line_count()`/`get_line(count-1)` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/045`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - real ASan heap-use-after-frees: 2 at the identical site (`TextEdit::Text::invalidate_cache()`, with a full stack through `Text::set() <- Text::insert() <- _base_insert_text() <- _insert_text() <- insert_text_at_caret() <- _set_text()`), 1 inside `CowData<char32_t>::_ref()` (a `String`'s own refcount, same family as candidates 038/040/043) |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on `set_text()`, `get_text()`, `get_line_count()`, `get_line()`, `insert_text_at_caret()` - the two confirmed entry points plus the 3 direct readers), incremental rebuild (ccache-warm hardlinked worktree, ~13m56s - `text_edit.h` is widely included and `tests/scene/test_text_edit.cpp` is this project's single slowest file to compile, per `projects/godot.md`'s own documented estimate) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~18m55s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (2 real ASan heap-use-after-frees at the identical
crash site, 1 at a second, independently-triggered site in `String`'s own `CowData` refcount), and
8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `TextEdit` class through nothing but the public GDScript
API. No second participant is involved - a same-process crash reachable from GDScript `Thread`s
calling `.text` assignment concurrently on a shared `TextEdit`.

## Class

`memory-safety` (heap-use-after-free on `TextEdit`'s internal `Text` document model - concurrent
`set_text()`/`insert_text_at_caret()` calls both rebuild the same unguarded `LocalVector<Line> text`
and its per-line cache fields, corrupting shared state; manifests at the cache-invalidation site or,
on a different timing, inside a line's own `String` refcount).

## Root cause

```cpp
void TextEdit::set_text(const String &p_text) {
	_set_text(p_text, false);           // <-- no lock anywhere in this call chain
}

int TextEdit::get_line_count() const {
	return text.size();                 // <-- unguarded read
}
```

`text` (`LocalVector<Line>`, wrapped in the nested `Text` helper class) and its per-line cache
fields (`max_line_width_dirty`, `max_line_width`, etc.) are plain members with zero synchronization
anywhere in the class. Two threads calling `set_text()`/`insert_text_at_caret()` concurrently race
each other through `Text::insert() <- Text::set() <- Text::invalidate_cache()` - confirmed via ASan:
`heap-use-after-free ... in TextEdit::Text::invalidate_cache(int, bool)`, with a full stack showing
the exact call chain from `_set_text()`.

Reachability: `set_text`/`get_text`/`get_line_count`/`get_line`/`insert_text_at_caret` are all
bound, GDScript-callable public methods - `TextEdit` is an ordinary `Control`-derived node,
reachable from any thread with no special setup once added to the tree. `CodeEdit` inherits this
exact code path unchanged (it doesn't override `set_text`/`get_line_count`), so this fix also
closes `CodeEdit`'s instance of the same external-sweep-flagged crash.

## Introduced by

not determined - `scene/gui/text_edit.cpp` has existed since the initial open-source commit
(0b806ee0fc9097fa7bda7ac0109191c9c5e0a1ac "GODOT IS OPEN SOURCE", 2014-02-09; confirmed via `git
log --follow`) and has never had any locking since.

## The fix

`+10/-0` against `scene/gui/text_edit.cpp`, `+3/-0` against `scene/gui/text_edit.h` (see
`patch.diff`): add `_THREAD_SAFE_CLASS_` next to the private members, and `_THREAD_SAFE_METHOD_` to
`set_text()`, `get_text()`, `get_line_count()`, `get_line()`, and `insert_text_at_caret()` (a
second, independently-bound entry point into the same mutation path).

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed, with 2 independent
crash sites observed).

- **Applied for real** in an isolated worktree (`.worktrees/045`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- **This fix is deliberately narrow**, covering only the specific confirmed crash path -
  `TextEdit` has hundreds of other public methods (caret/selection manipulation, syntax
  highlighting hooks, undo/redo, folding, gutters, IME composition, etc.) that remain completely
  unguarded. A full-class treatment is far out of scope for one session given the class's size.
- **`CodeEdit` was not independently given its own repro** - it inherits the fixed code path
  unchanged from `TextEdit` (doesn't override `set_text`/`get_line_count`/`get_line`), so this fix
  is expected to close `CodeEdit`'s instance of the same crash too, but this wasn't verified with a
  dedicated `CodeEdit`-specific repro this session.
- **1 widget remains** from the original external-sweep list: `GraphEdit` (now the only one not
  yet checked or ruled out).
