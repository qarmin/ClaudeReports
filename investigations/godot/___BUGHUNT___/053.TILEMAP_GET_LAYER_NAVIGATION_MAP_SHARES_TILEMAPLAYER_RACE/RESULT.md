# `TileMap::get_layer_navigation_map()` - shares candidate 032's `TileMapLayer` root cause exactly

From the coordinator's priority list (`TileMap` as a sibling of candidate 032's `TileMapLayer`).
Since Godot 4.3's refactor, `TileMap` is a thin compatibility wrapper that holds a
`Vector<TileMapLayer *> layers` and delegates nearly every call straight through via the
`TILEMAP_CALL_FOR_LAYER_V` macro - including `get_layer_navigation_map()`, which expands to
`return layers[layer]->get_navigation_map();`, i.e. **exactly** candidate 032's already-confirmed
(and fixed) `TileMapLayer::get_navigation_map()`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `modules/tilemap/tile_map.cpp` - `TileMap::get_layer_navigation_map(p_layer)` expands via `TILEMAP_CALL_FOR_LAYER_V` to `return layers[p_layer]->get_navigation_map();` with no guard of its own | Static read | Confirmed the crash path is identical to candidate 032's, just one call frame deeper (through the macro-generated wrapper and the `layers` vector indirection) |
| GDScript-level repro (`repro/`: one `TileMap` added to the tree - which always has at least 1 default layer from its constructor - 4 threads randomly calling `get_layer_navigation_map(0)`/`is_layer_navigation_enabled(0)`/`is_layer_enabled(0)` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/053`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 20 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - a deterministic plain segfault (not an ASan-reported heap corruption, since this is a null-pointer dereference, not a data race in the classic sense), always immediately preceded by the same `ERROR: .../Layer0: The function 'get_world_2d()' on this node can only be accessed from either the main thread or a thread group` message candidate 032 also saw - `TileMap`'s internal `Layer0` child node (a real `TileMapLayer` instance) hitting the identical guarded-then-null-dereferenced path |
| **Verification that candidate 032's existing patch alone fixes this** - applied `032.TILEMAPLAYER_GET_NAVIGATION_MAP_THREAD_GUARD/patch.diff` (unmodified, touches only `modules/tilemap/tile_map_layer.cpp` - zero changes to `tile_map.cpp`) to the same worktree, rebuilt | Same build/run, 10 attempts | **Clean 10/10 runs**, "survived" every time - confirms the fix lives entirely in `TileMapLayer`, nothing `TileMap`-specific needs to change |
| Project test suite with the patch applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed** - but **not a distinct bug**: this is candidate 032's `TileMapLayer::get_navigation_map()`
race (really a deterministic thread-guard-then-null-deref, not a true data race), reachable through
`TileMap`'s macro-generated per-layer delegation wrappers. 3/3 unfixed crashes (identical mechanism
to candidate 032), 10/10 clean using candidate 032's own unmodified patch.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `TileMap`'s public GDScript API.

## Class

`crash` (identical to candidate 032 - a guarded accessor, `CanvasItem::get_world_2d()`, returning a
null `Ref<World2D>` when called off the main thread, immediately dereferenced unconditionally one
call-frame up).

## Root cause

Identical to candidate 032 - see that candidate's `RESULT.md` for the full mechanism
(`TileMapLayer::get_navigation_map()`'s `is_inside_tree()`-guarded fallback branch unconditionally
dereferences `get_world_2d()`'s result, which is a thread-guarded method that returns null off the
main thread). The only difference here is the call path: `TileMap::get_layer_navigation_map(p_layer)`
is a macro-generated (`TILEMAP_CALL_FOR_LAYER_V`) thin wrapper with no state or logic of its own,
indexing into the `layers` vector and calling straight into the same unguarded
`TileMapLayer::get_navigation_map()` that candidate 032 already found and fixed.

## Introduced by

Same as candidate 032 - the bug lives in `TileMapLayer`, not in `tile_map.cpp`'s delegation macro.
`TileMap::get_layer_navigation_map()`'s macro-generated pass-through form predates this specific bug
(the `TILEMAP_CALL_FOR_LAYER_V` delegation pattern itself is part of the same 2024-02-28 refactor
candidate 032 traced the actual bug to - the macro was *already* in place, decoupling `TileMapLayer`
from `TileMap`, when the unguarded `get_world_2d()->` fallback was added to `TileMapLayer` itself).

## The fix

**No new fix - this candidate's `patch.diff` is an unmodified copy of candidate 032's own
`patch.diff`** (`+5/-1` against `modules/tilemap/tile_map_layer.cpp`). Nothing in `tile_map.cpp`
needs to change; fixing `TileMapLayer` (already done) transitively fixes every caller that forwards
to it, including every `TileMap::get_layer_*()`/`is_layer_*()` wrapper that delegates to a
`TileMapLayer` method touching `get_world_2d()`.

Verified: 10/10 clean runs of `repro/` with candidate 032's patch applied (vs. 3/3 crashing
unfixed). Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the patch applied - same
baseline already established and verified in candidate 032 itself.

## What's not done / open questions

- `Node` (the base `Node` class itself) is the next item on the coordinator's priority list - a
  genuinely different kind of check than this one, since `Node` isn't a thin wrapper around
  anything and would need its own from-scratch investigation rather than a fast-confirmation check.
- This confirms, for the third time this session (after candidate 050's `ConfirmationDialog`→`Label`
  and this one), that the coordinator's hypothesis about the external sweep's "Group A/B" list is
  largely made up of thin-delegation classes whose crashes trace back to a handful of already-fixed
  base/child classes - worth keeping in mind for `MultiplayerSpawner`/`CollisionShape3D` too (check
  for delegation to an already-fixed sibling before assuming a novel mechanism, but don't assume it
  either - candidate 052's `CSGSphere3D` just showed a "looks like a sibling" case can still hide a
  genuinely new, second bug).
