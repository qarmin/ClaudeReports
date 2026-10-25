# `SpriteFrames::animations` - plain unsynchronized `HashMap<StringName, Anim>`, no guards at all in this class

Fresh discovery (continuing the same heuristic that found candidate 063's `Gradient`). `SpriteFrames`
has **zero** existing thread guards (`grep -c THREAD_GUARD scene/resources/sprite_frames.{h,cpp}` =
0) and stores its animation data in a `HashMap<StringName, Anim>`, where each `Anim` holds its own
`Vector<Frame>` - a nested container race, same general shape as `CodeEdit`/057's `HashMap` and
`Image`/`Gradient`'s `Vector`, just one level deeper.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/sprite_frames.h`/`.cpp` - `animations` is mutated by `add_animation()`/`remove_animation()`/`rename_animation()`/`duplicate_animation()`/`clear_all()`, and each entry's `frames: Vector<Frame>` is separately mutated by `add_frame()`/`remove_frame()`/`set_frame()`/`clear()`. Read by `has_animation()`/`get_animation_list()`/`get_animation_names()`/`get_frame_count()`/the inline `get_frame_texture()`/`get_frame_duration()` (both defined in the header). All with zero synchronization anywhere | Static read | Identified the exact shape and confirmed every method is `ClassDB`-bound |
| GDScript-level repro (`repro/`: one `SpriteFrames` with one animation `"anim"` and one initial (null-texture) frame, 3 writer threads repeatedly `add_frame("anim", null)` and occasionally `remove_frame("anim", 0)` once the list grows past 20 entries, 3 reader threads reading `get_frame_count("anim")` then indexing every entry via `get_frame_texture()`/`get_frame_duration()`), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly, no rebuild needed for the unfixed run) in an isolated git worktree (`.worktrees/064`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - 1 clean ASan report (`heap-use-after-free scene/resources/sprite_frames.h:113 in SpriteFrames::get_frame_duration()`), 2 `negative-size-param`/`memmove` ASan aborts (a corrupted size value fed into a `memmove` during `Vector`'s internal reallocation) |
| Fix: added `_THREAD_SAFE_CLASS_` to `SpriteFrames`, wrapped every method touching `animations` or any entry's `frames` (`add_frame`, `set_frame`, `remove_frame`, `clear`, `clear_all`, `add_animation`, `has_animation`, `duplicate_animation`, `remove_animation`, `rename_animation`, `get_animation_list`, `get_animation_names`, `set_animation_speed`, `get_animation_speed`, `set_animation_loop_mode`, `get_animation_loop_mode`, `get_frame_count`, and the two inline header methods `get_frame_texture`/`get_frame_duration`) in `_THREAD_SAFE_METHOD_` - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes (2 distinct ASan signatures), 10/10 clean with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `SpriteFrames`'s public GDScript API (`add_frame`/`remove_frame`/
`get_frame_count`/`get_frame_texture`/`get_frame_duration`).

## Class

`memory-safety` (a plain unsynchronized nested-container race - `animations` itself is a
`HashMap<StringName, Anim>`, and each `Anim`'s own `frames: Vector<Frame>` is independently
unsynchronized; this candidate's repro specifically exercises the inner `Vector<Frame>` race by
keeping the animation key constant and only churning frames within it).

## Root cause

`SpriteFrames::add_frame()`/`remove_frame()`/`set_frame()`/`clear()` (`scene/resources/
sprite_frames.cpp`) mutate a given animation's `frames: Vector<Frame>` with zero lock.
`get_frame_count()` and the inline `get_frame_texture()`/`get_frame_duration()`
(`scene/resources/sprite_frames.h`) read it, also with zero lock. A reader thread's
`get_frame_count()` → loop-indexing pattern races a writer thread's concurrent `push_back()`/
`insert()`/`remove_at()`: the vector's backing buffer can be reallocated or shrunk between the
reader's size snapshot and its later indexed access, producing a heap-use-after-free or a corrupted
size value fed into the vector's internal `memmove` during reallocation.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `animations`/`Anim::frames` were
introduced.

## The fix

`+37/-0` in `scene/resources/sprite_frames.cpp`, `+8/-0` in `scene/resources/sprite_frames.h` (see
`patch.diff`):

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `SpriteFrames`.
- `_THREAD_SAFE_METHOD_` as the first statement in every public method that reads or mutates
  `animations` or any entry's `frames`: `add_frame`, `set_frame`, `remove_frame`, `clear`,
  `clear_all`, `add_animation`, `has_animation`, `duplicate_animation`, `remove_animation`,
  `rename_animation`, `get_animation_list`, `get_animation_names`, `set_animation_speed`,
  `get_animation_speed`, `set_animation_loop_mode`, `get_animation_loop_mode`, `get_frame_count`,
  and the two inline header accessors `get_frame_texture`/`get_frame_duration`.

Deliberately **not** independently locked: `set_animation_loop()`/`get_animation_loop()` (deprecated
wrappers that delegate to the now-locked `set_animation_loop_mode()`/`get_animation_loop_mode()`,
covered transitively) and `_get_animations()`/`_set_animations()` (internal serialization helpers
that call the now-locked `get_animation_list()`/`add_animation()`/etc., also covered transitively -
`Mutex` is recursive so this nesting is safe).

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed, 2 distinct
ASan signatures). Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied -
identical to the established baseline.

## What's not done / open questions

- This candidate's repro only exercises the inner `frames: Vector<Frame>` race (one animation key,
  churning only frames). The outer `animations: HashMap` race (concurrent `add_animation()`/
  `remove_animation()`/`rename_animation()` racing `has_animation()`/`get_animation_list()`) is
  covered by the same fix but was not independently confirmed with its own dedicated repro - the fix
  locks the same mutex around both, so a single `_THREAD_SAFE_CLASS_` closes both races at once, but
  a maximally rigorous write-up would test them separately.
- Found via fresh discovery (SKILL.md's normal process) after the coordinator's full priority list
  was exhausted. Same heuristic as candidate 063 (`Gradient`) - worth continuing to apply to more
  classes for the rest of the budget.
