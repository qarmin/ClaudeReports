# `FileDialog` - two distinct unguarded fields race: `full_dir` and `local_history`/`local_history_pos`

From the coordinator's priority list (the `Window`-family dialogs flagged in the external sweep).
Unlike candidates 049 (`Window`, ruled not reproducible) and 050 (`ConfirmationDialog`, shares a
sibling's already-fixed race), `FileDialog` has **zero** existing thread-guard macros of its own
(`grep -c THREAD_GUARD scene/gui/file_dialog.{h,cpp}` = 0) and turned out to hold **two separate,
genuinely new unguarded members** that race under concurrent `set_current_dir()`/`get_current_dir()`
calls from GDScript - not a pass-through to an already-fixed child `Control`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/file_dialog.h`/`.cpp` - `get_current_dir()` returns the plain `String full_dir` field directly; `update_dir()` writes it unconditionally (`full_dir = dir_access->get_current_dir();`) before any thread-guard-tripping UI calls later in the same function | Static read | Identified `full_dir` as an unguarded `CowData`-backed `String` field, same shape as every other confirmed candidate this session |
| GDScript-level repro (`repro/`: one `FileDialog` added to the tree with `access = ACCESS_RESOURCES`, 3 threads alternately setting `.current_dir` to `res://a`/`res://b`, 3 threads reading `.current_dir`/`.length()`, for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/051`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 4/5 unfixed attempts** (1 plain segfault, no ASan report) - `heap-use-after-free core/templates/safe_refcount.h:87 in SafeNumeric<unsigned long>::decrement()`, reached through `String::operator=`/`CowData<char32_t>::_ref()`/`_unref()` - the classic `CowData` refcount race, matching `full_dir`'s unguarded concurrent read ( `get_current_dir()`) vs. write (`update_dir()`) |
| First fix attempt: `_THREAD_SAFE_CLASS_` added to `FileDialog`, `_THREAD_SAFE_METHOD_` added to `update_dir()`'s `full_dir` write and to `get_current_dir()`/`get_current_path()`'s reads only (narrowest possible scope, matching candidate 039's style) - rebuilt, reran | Same build/run, 10 attempts | **Still crashed, ~6/10** - but with a *different* signature: `heap-use-after-free core/templates/list.h:312 in List<LineEdit::TextOperation...>::push_back()`, inside `LineEdit::_clear_undo_stack()`/`clear_internal()`, reached from `directory_edit->set_text()` called inside `FileDialog::_change_dir()`/`update_dir()` |
| Investigated the second signature: is it a *different* bug in `LineEdit` (candidate 040's fix not covering this internal path), or something else? Traced the ASan "freed by thread T28" / "previously allocated by thread T29" pair - both are `_writer_thread` instances, i.e. **two different FileDialog-API-calling threads**, not a LineEdit-internal issue. Re-read `FileDialog::set_current_dir()`'s full body (not just `_change_dir()`/`update_dir()`) and found `_push_history()` called unconditionally at the end, **completely unguarded**, mutating `local_history` (`Vector<String>`, also `CowData`-backed) via `local_history.resize()`/`.push_back()` on every call, from every thread, with zero lock | Static read + cross-reference with the first fix's remaining crash | **Second, independent unguarded field found**: `local_history`/`local_history_pos`, a `CowData<String>` buffer race exactly the same shape as `full_dir`'s, just a different member. Corrupting this buffer's heap metadata is what was producing the misleading "it's inside `LineEdit`" signature - heap corruption from one `Vector`'s freed/reallocated backing store can plausibly manifest as a UAF in an unrelated nearby allocation (`LineEdit`'s undo-stack `List` nodes), which is exactly what was observed |
| Second fix: moved `_THREAD_SAFE_METHOD_` up to the very top of `set_current_dir()` itself (recursive `Mutex`, safe to re-enter from the already-locked `update_dir()`/`get_current_dir()` calls it makes internally), covering **both** `_change_dir()`→`update_dir()`'s `full_dir` write and `_push_history()`'s `local_history` mutation in one lock scope - rebuilt, reran | Same build/run, 12 attempts | **Clean 12/12**, "survived" every time |
| Project test suite with the full fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline (pre-existing `test_ip.cpp` IPv6 environment failure), no new failures |

## Status

**Confirmed** - two distinct unguarded `CowData`-backed fields (`full_dir`, `local_history`/
`local_history_pos`) in `FileDialog` itself, not inherited from any child `Control`. 4/5 unfixed
crashes (various ASan signatures depending on which field's corruption surfaces first), 12/12 clean
with both fields covered by a single lock at `set_current_dir()`'s entry point (plus narrower locks
on the two direct getters).

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `FileDialog`'s public GDScript API (`current_dir` property).

## Class

`memory-safety` (`String`/`Vector`'s `CowData` refcount/buffer races - same family as nearly every
other confirmed candidate this session).

## Root cause

`FileDialog` has no thread guards of its own anywhere in the class. Two of its private fields are
mutated by the public `set_current_dir()` entry point and read by other public accessors, with zero
synchronization:

1. **`full_dir`** (`String`) - written unconditionally as the very first statement of `update_dir()`
   (`full_dir = dir_access->get_current_dir();`), called via `set_current_dir() -> _change_dir() ->
   update_dir()`. Read directly by `get_current_dir()` and `get_current_path()`.
2. **`local_history`/`local_history_pos`** (`Vector<String>`/`int`) - mutated unconditionally by
   `_push_history()` (`local_history.resize(...)`, `.push_back(...)`), called directly at the end of
   `set_current_dir()`'s body, every single call, with no guard.

Both are classic `CowData` (`String`'s and `Vector`'s shared copy-on-write backing buffer) refcount/
buffer races: concurrent writers corrupt the refcount or the buffer itself, and ASan catches it as a
heap-use-after-free, sometimes in the `SafeNumeric` refcount path, sometimes manifesting as apparent
corruption in a completely unrelated nearby heap allocation (as seen with the first, narrower fix
attempt, where fixing only `full_dir` left `local_history`'s corruption to resurface as a confusing
`LineEdit`-internal crash).

**Lesson for this session's notes**: when a narrow fix stops reproducing the *original* crash
signature but a *different* crash signature appears in an unrelated-looking class, don't assume a
brand new bug in that unrelated class first - re-check the original class for a **second, sibling
unguarded field** that the same calling code path also touches. Heap corruption from one `CowData`
buffer race can surface anywhere nearby, not just at its own write/read site.

## Introduced by

Not independently dated - `FileDialog` has never had any thread-safety pass applied to it (0
existing guard macros, vs. 147 in the sibling `Window` base class checked in candidate 049), so both
races have existed since `full_dir`/`local_history` were introduced to the class.

## The fix

`+9/-1` in `scene/gui/file_dialog.cpp`, `+3/-0` in `scene/gui/file_dialog.h` (see `patch.diff`):

- Added `_THREAD_SAFE_CLASS_` to `FileDialog` (`#include "core/os/thread_safe.h"` + the macro in the
  class body), giving the class its own recursive per-instance `Mutex`.
- `_THREAD_SAFE_METHOD_` at the very top of `set_current_dir()` - covers both `_change_dir()`'s
  `update_dir()` write to `full_dir` and the trailing `_push_history()`'s mutation of
  `local_history`/`local_history_pos`, in a single lock scope (safe to re-enter because `Mutex` is
  recursive, and `update_dir()`/`get_current_dir()` carry their own redundant-but-harmless locks too).
- `_THREAD_SAFE_METHOD_` in `update_dir()` (wrapping just the `full_dir` write), `get_current_dir()`,
  and `get_current_path()` - narrower locks for direct callers of those specific accessors that don't
  go through `set_current_dir()`.

Deliberately **not** locked: the rest of `set_current_dir()`'s internal callees (`_change_dir()`,
`invalidate()`, and all of `update_dir()`'s UI-side-effect code like `directory_edit->set_text()`,
`drives->...`) - those are either already covered transitively by the outer lock, already guarded
internally (an already-fixed `LineEdit::set_text()` from candidate 040), or are main-thread-only UI
code whose own `ERR_MAIN_THREAD_GUARD`s already reject off-thread calls (logged as errors, observed
throughout this repro's output, not a crash in themselves).

Verified: 12/12 clean runs of `repro/` with the fix applied (vs. 4/5 crashing unfixed, with two
distinct ASan signatures depending on which field's corruption happened to surface). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- `_go_back()`/`_go_forward()`/`update_filters()` and other internal callers of `_change_dir()` that
  *don't* go through `set_current_dir()` are all driven by UI signal callbacks (button presses),
  always main-thread in practice - not independently covered by a lock, consistent with this
  session's "lock at the public entry point, not every internal helper" scope convention used
  throughout (same as `Label`/`LineEdit`/etc.).
- `CSGSphere3D` (sibling of CSGCylinder3D/015) is the next item on the coordinator's priority list.
