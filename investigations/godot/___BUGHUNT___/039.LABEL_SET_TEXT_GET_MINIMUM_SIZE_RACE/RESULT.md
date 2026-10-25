# `Label::get_minimum_size()`/`set_text()` have no locking anywhere in the class

An external thread-safety sweep (`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md`, run
against Godot 4.7.stable with a generic random-method-call harness) found 10 of 11 tested `Control`
widgets (`CodeEdit`, `GraphEdit`, `ItemList`, `Label`, `LineEdit`, `OptionButton`,
`SplitContainer`, `TabBar`, `TextEdit`, `Tree`) crash under 6-thread concurrent bombardment but
survive at 1 thread, and attributed it to "`Control`/`CanvasItem`: no `Mutex` anywhere... theme/
minimum-size caches and redraw queuing touched by every `Control` widget unguarded" without
pinning down the exact mechanism or building a symbolized repro (their binary was release-stripped,
no debug symbols). This candidate picks `Label` (the simplest of the 10) and root-causes it
directly against this branch's own ASan-instrumented build.

Reading confirmed the sweep's "no `Mutex`" claim for `Control`/`CanvasItem` themselves, **but**
also found that `Control`'s own `get_combined_minimum_size()` *does* have
`ERR_READ_THREAD_GUARD_V` (so the crash isn't in `Control`'s own minimum-size cache path) -
the actual unguarded mechanism is one level down, in `Label`'s **own** lazily-rebuilt text-shaping
cache (`paragraphs`, a `Vector<Paragraph>` holding shaped-text `RID`s), reached through `Label`'s
own `get_minimum_size()` override, which has **no thread guard of any kind** (unlike `Control`'s).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/label.h`/`.cpp` - confirmed zero `Mutex`/`_THREAD_SAFE_CLASS_` anywhere; `Label::get_minimum_size()` (overriding `Control::get_minimum_size()`) has no thread guard and calls `_ensure_shaped()` -> `_shape()` -> `_shape_internal()`, which rebuilds `mutable Vector<Paragraph> paragraphs` (each holding shaped-text `RID`s) completely unguarded; `set_text()` also has no thread guard and directly mutates `text`/`xl_text` (`String`, `CowData`-backed) and `text_dirty` | Static read | Confirmed `set_text`/`get_text`/`get_minimum_size` are all `ClassDB`-bound, reachable from GDScript; matches the sweep's "Label crashes under concurrency, survives at threads=1" classification exactly - the mechanism needs two threads actually racing the shape-rebuild, not a single-thread bug |
| GDScript-level repro (`repro/`: shared `Label`, added to tree, 3 threads alternating `set_text()` between two different-length strings, 3 threads calling `get_minimum_size()` + reading `.text` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/039`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - 3 distinct real ASan reports (an `unknown-crash` in `CowData<RID>::_unref()` with a full stack through `Vector<RID>::clear()` -> `Label::_shape_internal()` -> `_shape()` -> `_ensure_shaped()` -> `get_minimum_size()`; two further `heap-use-after-free`s in `RID::get_id()`/`RID::operator==()`) - all three inside the `paragraphs` rebuild racing itself across threads |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on `_ensure_shaped()` (the single chokepoint for all 6 of its call sites), `get_minimum_size()` (needs its own lock too, not just the one inside `_ensure_shaped()` - see Root cause below), `set_text()`, `get_text()`), incremental rebuild (ccache-warm hardlinked worktree, ~3m40s - `label.h` is widely included, similar to `image.h`) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~5m30s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (3 distinct, independently-triggered real ASan
crashes, all inside the same `paragraphs`-rebuild code path), and 8/8 clean runs with the fix
applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `Label` class through nothing but the public GDScript API.
No second participant is involved - a same-process crash reachable from GDScript `Thread`s calling
`set_text()`/`get_minimum_size()` concurrently on a shared `Label`.

## Class

`memory-safety` (heap corruption on a `Vector<RID>` - two threads concurrently entering
`Label::_shape_internal()` (reached via `get_minimum_size()`, with no guard at all) both call
`Vector<RID>::clear()` on the same `paragraphs[i].glyphs`/shaped-line RID vector, corrupting its
`CowData`; manifests as an `unknown-crash`/heap-use-after-free depending on timing, same family as
candidates 016/027/035/036/037/038's confirmed buffer/refcount races, just one level deeper inside
a widget's own lazy-shaping cache rather than a `Resource`'s plain container).

## Root cause

```cpp
Size2 Label::get_minimum_size() const {
	_ensure_shaped();                      // <-- no lock anywhere in this call chain
	...
}

void Label::_ensure_shaped() const {
	if (dirty || font_dirty || text_dirty) {
		_shape();                           // <-- rebuilds `paragraphs` unguarded
	} else {
		for (const Paragraph &para : paragraphs) { ... }
	}
}
```

`Label::get_minimum_size()` (the `ClassDB`-bound override that answers for this widget's layout
size) has **no thread guard of any kind** - unlike `Control::get_combined_minimum_size()` (the base
class's own equivalent), which has `ERR_READ_THREAD_GUARD_V`. Calling it off the main thread
reaches `_ensure_shaped()` -> `_shape()` -> `_shape_internal()`, which does
`paragraphs[i].lines.clear()`/rebuilds the `Vector<RID>` of shaped-line handles with zero
synchronization. Two threads both calling `get_minimum_size()` (or one calling it while another
calls `set_text()`, which flips `text_dirty` and triggers the same rebuild on the next shape check)
race the same `Vector<RID>`'s `CowData`, corrupting it - confirmed via ASan: `unknown-crash ... in
CowData<RID>::_unref()`, reached through `Vector<RID>::clear() <- Label::_shape_internal() <-
Label::_shape() <- Label::_ensure_shaped() <- Label::get_minimum_size()`.

A subtlety that shaped the fix: adding `_THREAD_SAFE_METHOD_` only inside `_ensure_shaped()` is
**not sufficient on its own** - the `MutexLock` it creates is scoped to `_ensure_shaped()`'s own
function body and releases the instant that function returns, so `get_minimum_size()`'s own
subsequent reads of `minsize`/`theme_cache`/etc. after the `_ensure_shaped()` call would still run
unguarded (the same "caller outlives the lock" shape candidate 015's `CSGShape3D` hit). The fix
below locks `get_minimum_size()` itself too, so the lock stays held across the whole function
(`Mutex` is recursive, so the nested lock inside `_ensure_shaped()` is free).

Reachability: `set_text`/`get_text`/`get_minimum_size` are bound, GDScript-callable public methods
- `Label` is an ordinary `Control`-derived node, reachable from any thread with no special setup
once added to the tree.

## Introduced by

not determined - `scene/gui/label.cpp` has existed since the initial open-source commit
(0b806ee0fc9097fa7bda7ac0109191c9c5e0a1ac "GODOT IS OPEN SOURCE", 2014-02-09; confirmed via `git
log --follow`) and has never had any locking since - `Control`/`CanvasItem` (the base classes) have
no thread-safety infrastructure at all, so this was never a regression, it's foundational.

## The fix

`+8/-0` against `scene/gui/label.cpp`, `+3/-0` against `scene/gui/label.h` (see `patch.diff`): add
`_THREAD_SAFE_CLASS_` next to the private members, and `_THREAD_SAFE_METHOD_` to `_ensure_shaped()`
(the single chokepoint reached by all 6 of its call sites in the file), `get_minimum_size()` (to
keep the lock held across its own post-`_ensure_shaped()` reads, per the subtlety above), and
`set_text()`/`get_text()` (the direct `String` mutator/reader).

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed, with 3 distinct ASan
failure signatures across the attempts).

- **Applied for real** in an isolated worktree (`.worktrees/039`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- **This fix is deliberately narrow, covering only the specific confirmed crash path** - `Label`
  has ~69 public methods, and the vast majority (every other property setter/getter, `draw`,
  `_notification`, accessibility handling, etc.) remain completely unguarded. A full-class sweep
  (same scripted-insertion approach candidate 030 used for `PopupMenu`) is a natural follow-up but
  out of scope for this session.
- **This is representative of a much larger, architectural gap, not a `Label`-specific bug.** The
  external sweep found the *identical* crash-under-concurrency/survive-at-one-thread signature in 9
  other `Control` widgets (`CodeEdit`, `GraphEdit`, `ItemList`, `LineEdit`, `OptionButton`,
  `SplitContainer`, `TabBar`, `TextEdit`, `Tree`) - `Control`/`CanvasItem` themselves have no
  thread-safety infrastructure at all (no `_THREAD_SAFE_CLASS_`, no base-level `Mutex`), so every
  one of Godot's ~800+ `Control`-derived widget classes is a potential instance of this same
  "unguarded lazy-rebuild cache" shape, not just these 10. Properly closing this out would mean
  deciding on an architectural answer (a `Control`/`CanvasItem`-level lock? per-widget opt-in?) -
  a decision for the project maintainers, not something this one candidate's narrow fix settles.
  `MenuButton` was separately flagged by the sweep as *not* fitting this pattern (candidate 017
  already confirmed and fixed a distinct, unrelated `get_window()`-null-deref bug in it).
