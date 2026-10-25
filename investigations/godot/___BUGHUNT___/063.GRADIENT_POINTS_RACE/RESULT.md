# `Gradient::points` - plain unsynchronized `LocalVector`, no guards at all in this class

Fresh discovery (fallback to SKILL.md's normal process after the coordinator's full priority list
- priority 1/2 candidates, the IDEAS.md backlog, and the `_THREAD_SAFE_CLASS_` file sweep - was
exhausted). `Gradient` has **zero** existing thread guards
(`grep -c THREAD_GUARD scene/resources/gradient.{h,cpp}` = 0) and stores its color-stop data in a
plain `LocalVector<Point> points` - the same "plain unsynchronized container, GDScript-reachable
get/set pair" shape that's been this session's single most productive pattern (FileDialog/051,
MultiplayerSpawner/055, CollisionShape3D/056, CodeEdit/057, Image/061).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/gradient.h`/`.cpp` - `points` is a `LocalVector<Point>` (`Point` = `{float offset; Color color;}`), mutated by `add_point()`/`remove_point()`/`set_offsets()`/`set_colors()`/`reverse()`, read by `get_point_count()`/`get_offset()`/`get_color()`/`get_offsets()`/`get_colors()`/the inline `get_color_at_offset()` (bound as `sample`) - all with zero synchronization | Static read | Identified the exact shape and confirmed every method is `ClassDB`-bound (publicly GDScript-reachable) |
| GDScript-level repro (`repro/`: one `Gradient` with an initial point, 3 writer threads repeatedly `add_point()`-ing random offsets/colors and occasionally `remove_point(0)` once the list grows past 20 entries, 3 reader threads reading `get_point_count()` then indexing every entry via `get_color()`/`get_offset()`), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly, no rebuild needed for the unfixed run) in an isolated git worktree (`.worktrees/063`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - a `FATAL: Index p_index = N is out of bounds` bounds-check abort (SIGILL, exit 132), a TOCTOU between the reader's `get_point_count()` snapshot and the writer's concurrent `push_back()`/`remove_at()` shrinking the vector before the reader's subsequent indexed access lands |
| Fix: added `_THREAD_SAFE_CLASS_` to `Gradient`, wrapped every method touching `points` (`add_point`, `remove_point`, `reverse`, `set_offset`, `get_offset`, `set_color`, `get_color`, `set_offsets`, `get_offsets`, `set_colors`, `get_colors`, `get_point_count`, and the inline `get_color_at_offset`/`sample`) in `_THREAD_SAFE_METHOD_` - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes, 10/10 clean with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `Gradient`'s public GDScript API (`add_point`/`remove_point`/
`get_point_count`/`get_color`/`get_offset`).

## Class

`memory-safety` (a plain unsynchronized `LocalVector` buffer race - concurrent `push_back()`/
`remove_at()` reallocating/shrinking the backing array while a reader thread indexes into a
since-stale `size()`, producing an out-of-bounds access that `ERR_FAIL_UNSIGNED_INDEX_V`'s own
bounds check can still catch *if* the size it reads is also stale, but which in this repro manifests
as a `FATAL` abort when the actual current size has shrunk further than the reader's own earlier
snapshot accounted for - same general TOCTOU shape as every other `LocalVector`/`Vector` race this
session, just caught here by Godot's own `CRASH_COND`-style bounds check rather than ASan directly).

## Root cause

`Gradient::add_point()`/`remove_point()`/`set_offsets()`/`set_colors()`/`reverse()`
(`scene/resources/gradient.cpp`) mutate `points` with zero lock. `get_point_count()`/`get_offset()`/
`get_color()`/`get_offsets()`/`get_colors()`/`get_color_at_offset()` (the latter defined inline in
`gradient.h`, bound to GDScript as `sample()`) read it, also with zero lock. A reader thread's
`get_point_count()` → loop-indexing pattern races a writer thread's concurrent `push_back()`/
`remove_at()`: by the time the reader's loop reaches a later index, the vector may have already
shrunk (or had its backing buffer reallocated) under a concurrent writer, producing the `FATAL`
bounds-check abort observed.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `points` was introduced.

## The fix

`+23/-0` in `scene/resources/gradient.cpp`, `+5/-0` in `scene/resources/gradient.h` (see
`patch.diff`):

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `Gradient`.
- `_THREAD_SAFE_METHOD_` as the first statement in `add_point()`, `remove_point()`, `reverse()`,
  `set_offset()`, `get_offset()`, `set_color()`, `get_color()`, `set_offsets()`, `get_offsets()`,
  `set_colors()`, `get_colors()`, `get_point_count()`, and the inline `get_color_at_offset()`
  (bound as `sample()`) - every public method that reads or mutates `points`.

Deliberately **not** locked: `set_interpolation_mode()`/`get_interpolation_mode()`/
`set_interpolation_color_space()`/`get_interpolation_color_space()` (plain enum fields, not
`points`-backed, same low-risk category as other plain-scalar accessors left alone elsewhere this
session) and the private `transform_color_space()`/`inv_transform_color_space()`/`_update_sorting()`
helpers (only ever called from within an already-locked public entry point, recursive `Mutex` makes
this safe).

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- `_update_sorting()`'s `points.sort()` call (triggered lazily from several of the now-locked
  methods) was not given independent analysis beyond "it only runs inside an already-locked call" -
  consistent with how this session has treated similar lazy-cache-rebuild helpers elsewhere (e.g.
  `TreeItem::_create_children_cache()`).
- This was found via fresh discovery (SKILL.md's normal process) after the coordinator's full
  priority list was exhausted - a reasonable next step for continued breadth would be to apply the
  same heuristic (search for `Vector`/`LocalVector`/`HashMap`-backed `Resource`/`Node` classes with
  zero `_THREAD_SAFE_CLASS_`/guard infrastructure and a public get/set pair) to more classes not yet
  covered this session.
