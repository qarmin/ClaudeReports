# `Window` (base class) - not reproducible

From the coordinator's priority list for this session: the original external fuzzer sweep's own
`SWEEP_RESULTS.md` listed `Window` in "Group A - crashes even at `threads=1`" (the same
scene-tree-main-thread-only-servicing-race family as the already-confirmed Navigation x6/`GridMap`/
`CanvasLayer`/`TileMapLayer`/`PopupMenu`/`CSGCylinder3D` candidates), but candidate 026's own
follow-up notes had already read-and-ruled-out `Window` at the code level ("neither calls
`get_world_2d()`/`get_world_3d()` anywhere in their own `.cpp`/`.h`"). This candidate gives `Window`
its own dedicated empirical repro rather than relying on that code-reading conclusion alone, per
this session's priority to actually confirm-or-rule-out every remaining class with its own repro.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/main/window.cpp` - confirmed `Window` already has extensive thread-guard coverage (147 `ERR_READ_THREAD_GUARD_V`/`ERR_MAIN_THREAD_GUARD`/`ERR_THREAD_GUARD` uses across the file) - every getter (`get_position`, `get_size`, `is_embedded`) and setter (`set_position`, `set_size`, `set_visible`, `grab_focus`) checked has its own guard | Static read | Unlike the already-confirmed siblings in this family, `Window` does not call an *unguarded* `get_world_2d()`/`get_world_3d()` anywhere, nor does it expose a plain unguarded container - the class appears to already be correctly guarded, contrary to the external sweep's "Group A" classification |
| GDScript-level repro, attempt 1 (`repro/`: construct a `Window` but do *not* add it to the tree yet; spawn 4 worker threads immediately, each looping `get_position()`/`get_size()`/`is_embedded()`; only then call `add_child(win)` from the main thread - designed to race the worker threads directly against the main thread's synchronous `NOTIFICATION_ENTER_TREE` handling), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/049`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro`, 3 runs | **3/3 clean** - "survived" every time, no crash, no ASan report |
| GDScript-level repro, attempt 2 (same setup, but the 4 worker threads call setters instead: `position =`, `size =`, `visible = true`, `grab_focus()`) | Same binary, 3 runs | **3/3 clean** |
| GDScript-level repro, attempt 3 (standard pattern used by every other confirmed candidate in this family: `add_child(win)` first, *then* spawn 4 worker threads mixing both getters and setters) | Same binary, 3 runs | **3/3 clean** |

## Status

**Not reproducible.** 9/9 runs clean across 3 distinct approaches (soft cap reached): racing the
`NOTIFICATION_ENTER_TREE` handoff with getters, racing it with setters, and the standard
post-add_child stress pattern used successfully by every other confirmed candidate in this same
sweep family. `Window`'s own thread-guard coverage (147 guard uses) appears to correctly prevent
the race that the external sweep's stripped, symbol-less binary attributed to this class.

Possible explanations for the discrepancy with the external sweep's own "Group A" classification
(not confirmed, offered for a future session's benefit): the external sweep ran against Godot
4.7.stable with a generic random-method-call harness with no debug symbols - it's plausible the
crash it observed and attributed to "Window" actually originated in a *different* class also under
test around the same time (misattribution in a stripped-binary crash dump), or in a Window-family
method this session's 3 targeted approaches didn't happen to exercise (the class has well over 100
public methods; only a representative handful of getters/setters were tried here, not an exhaustive
sweep).

## What's not done / open questions

- Not every one of `Window`'s 100+ public methods was exercised - only representative
  getters/setters directly related to position/size/visibility/focus and the enter-tree handoff.
  A method this session didn't try (e.g. something DisplayServer-driver-specific, or a dialog/
  popup-adjacent method) could still have an unguarded path.
- `ConfirmationDialog`/`FileDialog` (the other two `Window`-family classes from the same sweep
  entry, sharing this base class) are investigated separately in their own candidates this
  session - a finding there could retroactively suggest a code path in `Window` itself worth a
  second look.
