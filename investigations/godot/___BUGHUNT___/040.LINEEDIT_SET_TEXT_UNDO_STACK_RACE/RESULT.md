# `LineEdit::set_text()`/`get_text()` have no locking anywhere in the class

Continues the external thread-safety sweep's `Control` widget lead (candidate 039 confirmed
`Label`; same sweep flagged `LineEdit` as one of the other 9 widgets sharing the "crashes under
6-thread concurrency, survives at `threads=1`" signature). `LineEdit` is a much larger class than
`Label` (159 `LineEdit::` method definitions vs. `Label`'s 69), so this candidate deliberately
scoped straight to reproducing and fixing the specific crash exercised, rather than auditing every
method up front.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/line_edit.h`/`.cpp` - confirmed zero `Mutex`/`_THREAD_SAFE_CLASS_` anywhere; `set_text()` has no thread guard and calls `_set_text()` -> `clear_internal()` -> `_clear_undo_stack()` -> `_create_undo_state()`, which directly mutates a `List<TextOperation> undo_stack`-shaped member with zero synchronization | Static read | Confirmed `set_text`/`get_text` are `ClassDB`-bound, reachable from GDScript; unlike `Label` (whose crash needed `get_minimum_size()` to trigger a lazy rebuild), `LineEdit`'s crash here comes from **concurrent `set_text()` calls racing each other directly** - no reader thread is even required to trigger it |
| GDScript-level repro (`repro/`: shared `LineEdit`, added to tree, 3 threads alternating `set_text()` between two different-length strings, 3 threads calling `get_minimum_size()` + reading `.text` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/040`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - 2 real ASan heap-use-after-frees inside `SafeNumeric<unsigned long>::decrement()` (`String`'s `CowData` refcount, reached through a `TextOperation`'s destructor during `List::erase()`), and 1 inside `List<LineEdit::TextOperation>::push_back()` itself, with a full stack through `_create_undo_state() <- _clear_undo_stack() <- clear_internal() <- _set_text() <- set_text()` - all three manifestations of the same unguarded `undo_stack` racing itself across the 3 `_set_spam` threads |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on `set_text()` and `get_text()` only - the two confirmed entry points), incremental rebuild (ccache-warm hardlinked worktree, ~3m56s - `line_edit.h` is widely included, similar to `label.h`/`image.h`) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~5m24s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (3 distinct real ASan crashes, all inside the same
unguarded `undo_stack` list), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `LineEdit` class through nothing but the public GDScript
API. No second participant is involved - a same-process crash reachable from GDScript `Thread`s
calling `set_text()` concurrently on a shared `LineEdit` (the reader threads calling
`get_minimum_size()`/reading `.text` in the repro were not actually necessary to trigger this
specific crash - the 3 writer threads racing each other was sufficient).

## Class

`memory-safety` (heap corruption on a `List<LineEdit::TextOperation>` - concurrent `set_text()`
calls both clear and rebuild the same undo-history list without any lock, corrupting its internal
linked-list allocator state; manifests as a heap-use-after-free either inside the list's own
`push_back()`/`erase()`, or inside a `TextOperation`'s `String` member's `CowData` refcount during
destruction, depending on timing).

## Root cause

```cpp
void LineEdit::set_text(String p_text) {
	_set_text(p_text);                 // <-- no lock anywhere in this call chain
}
```

`_set_text()` -> `clear_internal()` -> `_clear_undo_stack()` -> `_create_undo_state()` ultimately
calls `undo_stack.push_back(...)`/`undo_stack.clear()` on a plain `List<TextOperation>` member with
zero synchronization. Two threads both calling `set_text()` concurrently race each other's
modifications to this same list - confirmed via ASan: `heap-use-after-free ... in
List<LineEdit::TextOperation, DefaultAllocator>::push_back()`, with a full stack showing the exact
call chain from `set_text()`.

Reachability: `set_text`/`get_text` are bound, GDScript-callable public methods - `LineEdit` is an
ordinary `Control`-derived node, reachable from any thread with no special setup once added to the
tree.

## Introduced by

not determined for the class overall - `scene/gui/line_edit.cpp` has existed since the initial
open-source commit (0b806ee0fc9097fa7bda7ac0109191c9c5e0a1ac "GODOT IS OPEN SOURCE", 2014-02-09).
The specific `undo_stack` mechanism that this crash routes through was added later by
9dddce75d0e27558372efdc0a3e8e79051cfb87c "Added multi level undo redo to line edit" (2017-10-29),
already in this same unguarded shape.

## The fix

`+4/-0` against `scene/gui/line_edit.cpp`, `+3/-0` against `scene/gui/line_edit.h` (see
`patch.diff`): add `_THREAD_SAFE_CLASS_` next to the private members, and `_THREAD_SAFE_METHOD_` to
`set_text()` and `get_text()` only - the two confirmed entry points. This is a much narrower scope
than candidate 039's `Label` fix (which also covered the internal shaping chokepoint) - see "What's
not done" below for why.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed).

- **Applied for real** in an isolated worktree (`.worktrees/040`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- **This fix is deliberately much narrower than candidate 039's `Label` fix.** `LineEdit` has 159
  methods (more than double `Label`'s 69) and `_shape()` (the text-rid shaping rebuild, analogous
  to `Label`'s own) has over 20 call sites in this file alone - locking only `set_text`/`get_text`
  fixes the specific confirmed crash (the undo-stack race) but leaves the `_shape()`/`text_rid`
  path, the selection state, the IME composition state, and the context-menu/caret-handling methods
  all completely unguarded. A full-class treatment would need significantly more time than this
  session allotted to this single candidate.
- Not independently confirmed whether `get_minimum_size()` (which calls
  `TS->shaped_text_get_size(text_rid)`) has its own separate race against `_shape()`'s
  `TS->shaped_text_clear(text_rid)`/rebuild - the repro's reader threads calling
  `get_minimum_size()` never manifested as the crash in any of the 3 unfixed runs (all 3 landed in
  the undo-stack path instead), so this specific sub-mechanism remains unconfirmed either way.
- **7 more `Control` widgets remain from the same external sweep**, not yet independently
  confirmed: `CodeEdit`, `GraphEdit`, `ItemList`, `OptionButton`, `SplitContainer`, `TabBar`,
  `TextEdit`, `Tree` (originally 9, now 7 after `Label`/039 and `LineEdit`/040) - per candidate
  039's own lesson, each likely has its own distinct unguarded mechanism, not a single shared
  fix - confirm each independently.
