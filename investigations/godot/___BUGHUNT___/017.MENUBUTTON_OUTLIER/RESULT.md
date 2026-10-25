# `MenuButton::show_popup()` dereferences `get_window()`'s thread-guard-null result unchecked

Flagged by an external 6-thread-bombardment sweep (`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md`,
2026-10-05) as an "outlier": unlike the other `Control` widgets in the same sweep, `MenuButton`
crashed even at `threads=1` and has no `NOTIFICATION_ENTER_TREE`-based `READY` handler, so the
scene-tree-main-thread-servicing-race hypothesis that explained the rest of that sweep's group
didn't apply here - it needed its own dedicated look.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| GDScript-level repro (6 worker `Thread`s hammering `MenuButton.get_popup`/`.show_popup`/`.set_switch_on_hover`/`.is_switch_on_hover`/`.set_item_count`/`.get_item_count`), against the project's existing self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes` (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb), `--headless --path gdscript_repro` under `timeout -k 5 25` | **Crashed 3/3 runs**, plain SIGSEGV (exit 139), no ASan/Godot-symbolized report in the raw run |
| Same repro under `gdb -batch -ex run -ex "thread apply all bt"` to get a real backtrace | Same build | All 6 worker threads independently faulted at the identical frame: `Window::get_current_screen(this=0x0)` called from `MenuButton::show_popup()` at `menu_button.cpp:93`, `this=0x0` inside `Node::is_readable_from_caller_thread` (`node.h:711`) - a null-pointer dereference, not a sanitizer-flagged memory error, which is why the plain run above produced no ASan report |
| Narrowed repro (`repro/`, item-count methods removed - see "What's not done" below) against a rebuilt binary with the fix applied, 6 repeat runs | Same build, fix applied | **Clean 6/6 runs**, exit 0, "survived" printed each time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, `XDG_DATA_HOME` set | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-016 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** The crash is a deterministic null-pointer dereference, not a timing-dependent race:
every one of the 6 worker threads independently hit the identical fault in the single `gdb` capture
(not "N out of 6 got lucky"), and the process-level repro crashed 3/3 runs. With the fix applied,
the narrowed repro (isolating this specific bug from the separate issue noted below) ran clean 6/6.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `MenuButton` through nothing but its public GDScript API
(`MenuButton.get_popup()`/`.show_popup()`/`.set_switch_on_hover()`/`.is_switch_on_hover()`). No
second participant is involved - this is a same-process bug between GDScript `Thread`s and one
shared `MenuButton` node.

## Class

`crash` (deterministic null-pointer dereference off the main thread - same general shape as
candidate 014's `NavigationRegion2D::get_navigation_map()` finding: an existing thread guard
returns a null/safe sentinel for the *wrong* thread, and the caller dereferences it unconditionally
a few lines later, rather than two threads racing each other for a shared resource)

## Root cause

`MenuButton::show_popup()` (`scene/gui/menu_button.cpp:81`) calls `get_window()` without any thread
guard of its own. `Node::get_window()` has `ERR_READ_THREAD_GUARD_V(nullptr)` internally and returns
a null `Window*` when called off the main thread (logging an error, visible in the preceding log
lines in `output.txt`) - but `show_popup()`'s very next line immediately calls
`->get_current_screen()` on that null pointer (line 93:
`DisplayServer::get_singleton()->screen_get_usable_rect(get_window()->get_current_screen())`).
`Window::get_current_screen()` in turn starts with `ERR_READ_THREAD_GUARD_V(...)`, which calls
`is_readable_from_caller_thread()` - a non-virtual method invoked through the null `this`, which
immediately segfaults at `node.h:711` before the guard macro even gets a chance to report the real
problem. This happens on **any** call to `show_popup()` from a non-main thread, every time -
there's no concurrent-access window to miss, which is why all 6 independent worker threads in the
single `gdb` capture hit the exact same line.

Unlike the `get_world_2d()`/`get_world_3d()` family (candidate 014 and its still-unswept siblings in
`IDEAS.md`), this crash route goes through `get_window()` specifically, and `show_popup()` was the
only call site in `MenuButton` reaching it from the exercised public API.

`show_popup()`'s own thread-guard macros (`ERR_THREAD_GUARD`/`ERR_MAIN_THREAD_GUARD`/etc., defined
in `scene/main/node.h:933-948`) are all compiled to no-ops unless `DEBUG_ENABLED` - this build has it
on (editor target), so the fix is effective here, but a release build would still crash on this
exact input; that's a pre-existing, engine-wide limitation shared by every other thread-guard-based
fix in this workspace, not unique to this candidate.

## Introduced by

272c297931 "Improve MenuButton and OptionButton" (2022-10-02) - the commit that introduced
`show_popup()` itself. The method never had a thread guard at any point in its history (checked via
`git log --follow -p` on `scene/gui/menu_button.cpp`); this isn't a regression from a later edit.

## The fix

Add `ERR_THREAD_GUARD` as the first statement of `show_popup()`, matching the convention already
used by sibling guarded methods elsewhere in the engine (e.g. candidate 014's fix, and the dozens of
existing `ERR_THREAD_GUARD`/`ERR_READ_THREAD_GUARD` call sites already in `scene/`). This makes
`show_popup()` log the same "caller thread can't call this function" error and return immediately
when called off the main thread, instead of falling through to the unconditional null dereference
two lines later (`+2/-0`, see `patch.diff`).

Verified sufficient against the narrowed repro (6/6 clean runs) - see "What's not done" below for
why the fix was verified against a narrowed rather than the original full repro.

- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before this
  candidate's work and after the fix - no new failures (the 1 failure is the pre-existing
  `test_ip.cpp` IPv6 environment issue already present in the baseline from candidates 002-016).

## What's not done / open questions

- **The original repro (including `set_item_count`/`get_item_count`) still crashes even with this
  fix applied** - but from a *different, independent* bug: `PopupMenu::get_item_count()` (via
  `MenuButton::get_item_count()`/`set_item_count()`) races `PopupMenu`'s `Vector<Item> items` with
  no locking, caught as a real ASan `unknown-crash` (`READ of size 8` in
  `CowData<PopupMenu::Item>::size()`, `core/templates/cowdata.h:185`, called from
  `PopupMenu::get_item_count()` at `scene/gui/popup_menu.cpp:3082`). This candidate's `repro/` was
  deliberately narrowed to exclude `set_item_count`/`get_item_count` so this fix could be verified
  in isolation against the specific bug it actually addresses - the `PopupMenu::items` race is
  tracked as its own backlog entry in `IDEAS.md` for a dedicated candidate.
- `get_popup()` itself (returning a raw `PopupMenu*` with no guard at all) wasn't independently
  implicated in any observed crash this session, but reading `PopupMenu`'s own public API for the
  same missing-lock pattern as the `items` race above (not just `get_item_count()`) wasn't done
  exhaustively - worth checking alongside the new `PopupMenu::items` candidate.
- `OptionButton` shares `show_popup()`'s exact origin commit (272c297931 added both in one commit)
  and very likely the identical missing-guard shape - not checked this session, worth a quick
  follow-up given how directly the same commit introduced both.
