# `MultiplayerSpawner::spawnable_scenes` - plain unsynchronized `LocalVector`, no guards at all in this class

From the coordinator's priority list. `MultiplayerSpawner` has **zero** existing thread guards
(`grep -c THREAD_GUARD modules/multiplayer/multiplayer_spawner.{h,cpp}` = 0) and holds its
`spawnable_scenes` list in a plain `LocalVector<SpawnableScene>` - mutated by
`add_spawnable_scene()`/`clear_spawnable_scenes()`, read by `get_spawnable_scene_count()`/
`get_spawnable_scene()`, with no synchronization anywhere - the same "plain unsynchronized
container" shape as candidates 016/027/035/036/037/051 this session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `modules/multiplayer/multiplayer_spawner.h`/`.cpp` - `spawnable_scenes` is a `LocalVector<SpawnableScene>` (not `CowData`-backed, but still a plain heap-allocated growable buffer with zero synchronization of its own); `add_spawnable_scene()`'s `ResourceLoader::exists()` check only runs when `Engine::is_editor_hint()` is true, which is false for a plain `--headless` run - meaning fake, non-existent scene paths can be pushed freely from a GDScript repro with no real `.tscn` files needed | Static read | Identified the exact shape and a convenient way to drive it purely from GDScript without needing real resource files |
| GDScript-level repro (`repro/`: one `MultiplayerSpawner` added to the tree, 3 writer threads calling `add_spawnable_scene()` with fresh fake paths (occasionally calling `clear_spawnable_scenes()` once the list gets long), 3 reader threads calling `get_spawnable_scene_count()` then indexing every entry via `get_spawnable_scene(i)`), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/055`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - 1 clean ASan report (`heap-use-after-free core/templates/cowdata.h:525 in CowData<char32_t>::_ref()`, reached through `MultiplayerSpawner::get_spawnable_scene()` at `multiplayer_spawner.cpp:124`), 2 plain segfaults with no report printed before the process died (consistent with how severely-corrupted-heap crashes have behaved in a couple of other candidates this session) |
| Fix: added `_THREAD_SAFE_CLASS_` to the class, wrapped `spawnable_scenes`' mutation sites (`add_spawnable_scene()`'s `push_back()`, `clear_spawnable_scenes()`'s `clear()`) and read sites (`get_spawnable_scene_count()`, `get_spawnable_scene()`, the internal `_get_spawnable_scenes()` property getter) each in their own `MutexLock(_thread_safe_)` scope - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes, 10/10 clean with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `MultiplayerSpawner`'s public GDScript API. No real networking/peer
participant needed - this is a same-process, same-instance data race, independent of whether any
actual multiplayer session is active.

## Class

`memory-safety` (a plain unsynchronized `LocalVector` buffer race - concurrent `push_back()`/
`clear()` reallocating or freeing the backing array while a reader thread indexes into a
since-stale `size()`/pointer, producing a heap-use-after-free on the `String` path field it
eventually tries to copy out of a freed/reallocated slot).

## Root cause

`MultiplayerSpawner::add_spawnable_scene()` (`modules/multiplayer/multiplayer_spawner.cpp:100`)
calls `spawnable_scenes.push_back(sc)` and `clear_spawnable_scenes()` calls
`spawnable_scenes.clear()`, both with zero synchronization. `get_spawnable_scene_count()`/
`get_spawnable_scene(idx)` read `spawnable_scenes.size()`/index into it, also with zero
synchronization. A writer thread's `push_back()` (which can reallocate the backing buffer once
capacity is exceeded) or `clear()` (which frees the backing buffer outright) racing against a reader
thread's concurrent `size()`/`operator[]` is a classic unsynchronized-container race: the reader can
read a stale size, dereference a pointer into memory that's mid-reallocation or already freed, and
by the time it copies out the `String path` field, that read lands on freed/stale memory - caught by
ASan as a `CowData<char32_t>` UAF inside the `String` copy itself.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `spawnable_scenes` was introduced.

## The fix

`+16/-3` in `modules/multiplayer/multiplayer_spawner.cpp`, `+2/-0` in
`modules/multiplayer/multiplayer_spawner.h` (see `patch.diff`):

- Added `_THREAD_SAFE_CLASS_` to `MultiplayerSpawner` (the header already transitively includes
  `core/os/thread_safe.h` via `scene/main/node.h`, no new `#include` needed).
- `add_spawnable_scene()`: wrapped the `push_back()` call and the `size()` read used for the
  "first scene added" connection check in one `MutexLock` scope, capturing the post-push size into
  a local (`new_size`) so the rest of the function (which touches the node tree, not the vector)
  doesn't need to stay inside the lock.
- `get_spawnable_scene_count()`, `get_spawnable_scene()`, and the internal `_get_spawnable_scenes()`
  property getter: each wrapped in its own `MutexLock` scope around the existing body.
- `clear_spawnable_scenes()`: wrapped just the `clear()` call, leaving the node
  connect/disconnect logic (main-thread-only UI/tree bookkeeping, not touching the vector) outside
  the lock.

Deliberately **not** locked: `_set`/`_get`/`_get_property_list` (editor-only, `TOOLS_ENABLED`-gated,
main-thread-only property-inspector code paths) and `_set_spawnable_scenes()` (calls the
now-individually-locked `clear_spawnable_scenes()`/`add_spawnable_scene()` in a loop, so it's
transitively covered without needing its own lock).

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- `tracked_nodes` (a `HashMap<ObjectID, SpawnInfo>`) and `spawn_node`/`spawn_limit`/`spawn_function`
  were not audited - this candidate focused on `spawnable_scenes` specifically, since that's the
  field with a clean, directly GDScript-reachable read/write pair. `tracked_nodes` is mutated by
  `_track()`/`_node_exit()`, which look like they're driven by tree-signal callbacks
  (`child_entered_tree`) rather than directly public API, so likely main-thread-only in practice -
  not independently confirmed either way.
- `CollisionShape3D` is the next item on the coordinator's priority list.
