# `CodeEdit::breakpointed_lines` - plain unsynchronized `HashMap`, independent of `TextEdit`'s own (narrower) fix

From the coordinator's priority list ("`CodeEdit` (inherits TextEdit/045's fix, lower priority per
IDEAS.md - only if time allows)"). Checked directly since the session's remaining budget was ample.
`CodeEdit` does inherit `TextEdit`'s `_thread_safe_` mutex protection for `set_text()`/`get_text()`/
`get_line()`/etc. (candidate 045), but that mutex is **private** to `TextEdit`, so `CodeEdit` (a
subclass) cannot reuse it - and `CodeEdit` has its own, entirely separate unguarded state: the
breakpoint-tracking `HashMap<int, bool> breakpointed_lines`, mutated directly by
`set_line_as_breakpoint()` with zero guard of any kind, independent of anything `TextEdit`'s own fix
covers.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/code_edit.h`/`.cpp` - `grep -c THREAD_GUARD` = 0 in this class specifically (separate from `TextEdit`'s own count). `CodeEdit::set_line_as_breakpoint()` mutates `breakpointed_lines[p_line]`/`.erase(p_line)` directly; `CodeEdit::_text_changed()` (called from the `text_changed` signal after any edit) also iterates and mutates the same map, also unguarded. Neither `is_line_breakpointed()` nor `get_breakpointed_lines()` actually read `breakpointed_lines` (they go through `get_line_gutter_metadata()` instead) - so the exposed race is a pure **writer-vs-writer** race, not the more common writer-vs-reader shape | Static read | Identified `breakpointed_lines` as a second, independent unguarded field, requiring `CodeEdit`'s own separate mutex (not reusable from `TextEdit`, since that one is private) |
| GDScript-level repro (`repro/`: one `CodeEdit` with 100 lines of text pre-populated via `.text =` (covered by `TextEdit`'s existing lock, used only for setup here), 3 writer threads each repeatedly toggling `set_line_as_breakpoint(random_line, true)` then `(..., false)` on random lines, 3 reader threads calling `get_breakpointed_lines()`), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/057`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts**, identical signature every time: `heap-use-after-free core/templates/hash_map.h:366 in HashMap<int,bool,...>::erase()`, directly inside `CodeEdit::set_line_as_breakpoint()` (`code_edit.cpp:1651`) - confirms the writer-vs-writer race on `breakpointed_lines` itself |
| Fix: added `_THREAD_SAFE_CLASS_` to `CodeEdit` (its own, separate mutex from `TextEdit`'s), wrapped `set_line_as_breakpoint()`'s `breakpointed_lines` mutation in a `MutexLock` scope, and (for completeness, since it's the same field) also wrapped the three mutation points inside `_text_changed()`'s edit-shift bookkeeping loop - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes (identical signature each time), 10/10 clean with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `CodeEdit`'s public GDScript API (`set_line_as_breakpoint()`/
`get_breakpointed_lines()`).

## Class

`memory-safety` (a plain unsynchronized `HashMap<int, bool>` race - two threads concurrently
inserting/erasing into the same hash table, corrupting its internal bucket/element bookkeeping,
caught by ASan as a heap-use-after-free inside `erase()`).

## Root cause

`CodeEdit::set_line_as_breakpoint()` (`scene/gui/code_edit.cpp:1643`) directly mutates
`breakpointed_lines[p_line] = true;` / `breakpointed_lines.erase(p_line);` with no guard. This is a
**different field from anything `TextEdit`'s candidate 045 fix touches** - `TextEdit`'s `_thread_safe_`
mutex is a `private` member of `TextEdit`, not accessible to `CodeEdit` (a subclass), so inheriting
"the fix" in the inheritance sense does nothing for `CodeEdit`'s own, independently-added mutable
state. Multiple threads calling `set_line_as_breakpoint()` concurrently race each other's
insert/erase on the same `HashMap`, corrupting it.

A second mutation site exists in `CodeEdit::_text_changed()` (triggered by `TextEdit`'s own
`text_changed` signal after any edit), which iterates and erases/inserts into the same
`breakpointed_lines` map to keep breakpoints aligned with line-number shifts after edits - also
unguarded, fixed as part of the same patch for consistency even though this session's specific
repro didn't need to exercise it to get a clean crash (my repro didn't perform any text edits
concurrently with breakpoint toggles).

## Introduced by

Not independently dated - `CodeEdit` has never had any thread-safety pass applied to it as a class
in its own right (0 pre-existing guard macros *in this file*, separate from whatever `TextEdit`
itself has), so the race has existed since `breakpointed_lines` was introduced.

## The fix

`+16/-6` in `scene/gui/code_edit.cpp`, `+2/-0` in `scene/gui/code_edit.h` (see `patch.diff`):

- Added `_THREAD_SAFE_CLASS_` to `CodeEdit` (a separate, `CodeEdit`-owned mutex - `TextEdit`'s own is
  private and unreachable from here; `scene/gui/text_edit.h` is already included, which transitively
  pulls in `core/os/thread_safe.h`, no new `#include` needed).
- `set_line_as_breakpoint()`: wrapped just the `breakpointed_lines` mutation block (not the gutter
  metadata call before it, nor the signal emission/`queue_redraw()` after it).
- `_text_changed()`: wrapped each of the three places that read/mutate `breakpointed_lines`
  (building the snapshot list, the `erase()`, and the `[next_line] = true` insert) in their own
  small `MutexLock` scopes, keeping the signal emissions and `is_line_breakpointed()` calls between
  them outside the lock.

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed, identical
signature every time). Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied -
identical to the established baseline.

## What's not done / open questions

- This closes out the entire priority-1 list from the coordinator's instructions
  (`Window`/049, `ConfirmationDialog`/050, `FileDialog`/051, `CSGSphere3D`/052, `TileMap`/053,
  `Node`/054, `MultiplayerSpawner`/055, `CollisionShape3D`/056, `CodeEdit`/057).
- `CryptoKey` (priority-2) is next, followed by the IDEAS.md backlog (priority-3).
- `CodeEdit`'s other `HashMap`/`Vector` fields (`line_number_text_cache`, `delimiters`,
  `delimiter_cache`, `code_completion_options`, `auto_brace_completion_pairs`, etc.) were not
  audited - this candidate focused specifically on `breakpointed_lines` since it was the first and
  most directly GDScript-reachable unguarded field found. Worth a dedicated follow-up scan if this
  class is revisited.
