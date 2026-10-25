# `NavigationObstacle2D::get_navigation_map()` dereferences `get_world_2d()`'s thread-guard-null result unchecked

Primary target of this sweep follow-up (candidate 026): the exact 2D counterpart of confirmed
candidate 022's `NavigationObstacle3D` - same `get_navigation_map()` body shape, same
`get_world_2d()`/`get_world_3d()` unchecked-dereference family already confirmed 7x (candidates
014, 019, 020, 021, 022, 023, 024). Built and ran its own independent repro + backtrace per
candidate 014's lesson rather than assuming the mechanism transfers unchanged. As part of the same
investigation, also read (not fully repro'd) the other still-unverified siblings named in the
original sweep - `GridMap`, `CanvasLayer`, `TileMapLayer`, `Window`, `MultiplayerSpawner` - see
"What's not done / open questions" below for what each turned up.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: grep `get_world_2d()->`/`get_world_3d()->` across `scene/`/`modules/` and inspect `NavigationObstacle2D::get_navigation_map()` (`scene/2d/navigation/navigation_obstacle_2d.cpp:242-249`) | Static read | Byte-for-byte the same shape as candidate 022's `NavigationObstacle3D::get_navigation_map()`: `is_inside_tree()` guarded, but `get_world_2d()`'s own thread guard result dereferenced unconditionally |
| GDScript-level repro (`repro/`, modeled on candidates 014/019-024's harness: instantiate `NavigationObstacle2D`, `add_child`, spawn 6 worker `Thread`s hammering a mix of setters/getters including `get_navigation_map()`), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/026`, with `bin/`/`.sconsign5.dblite` hardlinked from the main tree per `projects/godot.md`'s documented technique) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on the first try**; a follow-up 2/2 repeat also crashed (3/3 total) |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary, a simplified 3-op repro (see below) | `World2D::get_navigation_map (this=0x0)` at `world_2d.cpp:52`, called from `NavigationObstacle2D::get_navigation_map` at `navigation_obstacle_2d.cpp:246`, from a `Node::_spam` worker thread - confirms the exact mechanism, identical to candidate 022 |
| Simplified the repro script to only 3 read-only ops (`get_navigation_map`/`get_avoidance_enabled`/`get_radius`, 4 threads, 3s stress window) after the original 8-op/15s version produced tens-of-MB logs of unrelated `ERR_READ_THREAD_GUARD` messages (`get_global_scale`/`queue_redraw` from setter ops) that pushed wall-clock past the test timeout on the *fixed* binary (the pitfall `projects/godot.md` already documented for candidate 010) | Same ASan binary, unfixed and fixed | Simplified repro reproduces identically: **3/3 unfixed runs crash**, **8/8 fixed runs clean** ("survived" printed every time) |
| Fix (`ERR_READ_THREAD_GUARD_V(RID())` + explicit `Ref<World2D>::is_valid()` check before dereferencing), incremental rebuild (ccache-warm hardlinked worktree, ~53-90s per rebuild) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` both before (unfixed source, freshly rebuilt) and after the fix - identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs across two repro variants (deterministic, not
timing-dependent - every attempt crashed within a few seconds), and 8/8 clean runs with the fix
applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `NavigationObstacle2D` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from a single
GDScript `Thread` calling one public method (`get_navigation_map()`) off the main thread.

## Class

`crash` (null-pointer dereference - a single worker thread calling `get_navigation_map()` crashes
deterministically every time it's reached off the main thread).

## Root cause

`NavigationObstacle2D::get_navigation_map()`:

```cpp
RID NavigationObstacle2D::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (is_inside_tree()) {
		return get_world_2d()->get_navigation_map();          // <-- unconditional dereference
	}
	return RID();
}
```

Identical mechanism to candidates 014/019-024: `CanvasItem::get_world_2d()`
(`scene/main/canvas_item.cpp:1253-1263`) begins with `ERR_READ_THREAD_GUARD_V(Ref<World2D>());`,
returning a null `Ref<World2D>` when called off the main thread/a thread group. `get_navigation_map()`
dereferences that result unconditionally, crashing inside `World2D::get_navigation_map()`
(`scene/resources/world_2d.cpp:52`) on `this == nullptr` (the very first line, `if
(navigation_map.is_null())`, reads a member through the null `this`).

Reachability: the bound, GDScript-callable public method
(`ClassDB::bind_method(D_METHOD("get_navigation_map"), ...)`, `navigation_obstacle_2d.cpp:54`) is
reachable from any thread with no special setup - a deterministic crash on every off-main-thread
call (as long as `map_override` isn't set, the common default configuration).

## Introduced by

a6ac305f967a272c35f984b046517629a401b688 "Rework Navigation Avoidance" (2023-01-10) - the same
commit that introduced candidate 022's `NavigationObstacle3D::get_navigation_map()` bug; this
broader avoidance-system rework added the equivalent 2D body at the same time
(`git log --follow -p` on `scene/2d/navigation/navigation_obstacle_2d.cpp` shows the first
`+RID NavigationObstacle2D::get_navigation_map()` hunk inside this same commit). Never
retrofitted with the `ERR_READ_THREAD_GUARD_V` idiom `get_world_2d()` itself already uses.

## The fix

Same shape as candidates 014/019-024 (`+5/-1` against
`scene/2d/navigation/navigation_obstacle_2d.cpp`, see `patch.diff`):

```cpp
RID NavigationObstacle2D::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (is_inside_tree()) {
		ERR_READ_THREAD_GUARD_V(RID());
		Ref<World2D> world_2d = get_world_2d();
		if (world_2d.is_valid()) {
			return world_2d->get_navigation_map();
		}
	}
	return RID();
}
```

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed, across two repro
variants).

- **Applied for real** in an isolated worktree (`.worktrees/026`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end (only the pre-existing `___BUGHUNT___/`/`___OPTIMIZATIONS___/`
  untracked dirs and the pre-existing `.gitignore` modification, both predating this candidate).
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- The other call sites of `get_world_2d()->` in the same file (`:94`/`:104` in `_notification`'s
  `NOTIFICATION_POST_ENTER_TREE` handler) were not touched - already restricted to the main thread
  by the engine's own contract, same as the equivalent 3D file's `_notification` call site noted in
  candidate 022.
- **The other five classes named in this sweep step were read but not independently repro'd this
  session** (out of scope for this one candidate per the project's "one bug per candidate folder"
  convention - each would need its own numbered candidate and its own repro before being written
  up as Confirmed):
  - **`GridMap`** (`modules/gridmap/grid_map.cpp:309-316`) has the **exact same bug shape**:
    `RID GridMap::get_navigation_map() const` calls `get_world_3d()->get_navigation_map()`
    unconditionally when `is_inside_tree()` and `map_override` isn't set, and it's a bound public
    method (`ClassDB::bind_method(D_METHOD("get_navigation_map"), ...)`,
    `modules/gridmap/grid_map.cpp:1423`). Not yet repro'd - very high-confidence new suspect.
  - **`TileMapLayer`** (`modules/tilemap/tile_map_layer.cpp:3473-3480`) also has the **exact same
    bug shape**: `RID TileMapLayer::get_navigation_map() const` calls
    `get_world_2d()->get_navigation_map()` unconditionally, same `is_inside_tree()`/override guard
    structure, also a bound public method
    (`modules/tilemap/tile_map_layer.cpp:2252`). Not yet repro'd - very high-confidence new
    suspect.
  - **`CanvasLayer`** (`scene/main/canvas_layer.cpp:296-306`) has a related but structurally
    different shape: the public, unguarded setter `set_follow_viewport(bool)` calls
    `_update_follow_viewport()`, which (when `follow_viewport` is true and the node is inside the
    tree) calls `vp->get_world_2d()->get_canvas()` unconditionally, where `vp` is a cached
    `Viewport*` and `Viewport::get_world_2d()` (`scene/main/viewport.cpp:1401-1404`) has the same
    `ERR_READ_THREAD_GUARD_V(Ref<World2D>())` guard. This is the **setter-shaped** variant (like
    candidate 024's `CollisionObject3D::set_disable_mode()`), not yet repro'd - plausible new
    suspect, not confirmed.
  - **`Window`** and **`MultiplayerSpawner`**: read and ruled out at the code-reading level - grep
    for `get_world_2d(`/`get_world_3d(` across `scene/main/window.cpp`/`scene/main/window.h` and
    `modules/multiplayer/multiplayer_spawner.cpp` found **zero occurrences** in either file; neither
    class calls `get_world_2d()`/`get_world_3d()` anywhere, so this specific bug class doesn't
    apply to them at all.
  - These four leads (`GridMap`, `TileMapLayer`, `CanvasLayer`'s setter variant - `Window`/
    `MultiplayerSpawner` are genuinely clean) are left for a future candidate, not pursued further
    under this one's number per the one-bug-per-candidate convention.
