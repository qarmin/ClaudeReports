# `GridMap::get_navigation_map()` dereferences `get_world_3d()`'s thread-guard-null result unchecked

Flagged by candidate 026's own follow-up notes as a very high-confidence unconfirmed sibling of the
already-8/8-confirmed `get_world_2d()->`/`get_world_3d()->` unchecked-dereference family
(candidates 014, 019-024, 026). Same body shape, 3D variant, on a different class
(`GridMap`, `modules/gridmap/grid_map.cpp`) not covered by that sweep.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `modules/gridmap/grid_map.cpp:309-316` (`GridMap::get_navigation_map()`) and `scene/3d/node_3d.cpp:1074-1079` (`Node3D::get_world_3d()`) | Static read | Byte-for-byte the same shape as candidates 014/019-024/026: `is_inside_tree()` guarded, but `get_world_3d()`'s own `ERR_READ_THREAD_GUARD_V(Ref<World3D>())` result is dereferenced unconditionally one line later. Confirmed `get_navigation_map()` is `ClassDB`-bound (`modules/gridmap/grid_map.cpp:1423`), reachable from GDScript |
| GDScript-level repro (`repro/`: instantiate `GridMap`, `add_child`, spawn 4 worker `Thread`s calling a mix of `get_navigation_map()`/`get_collision_layer()`/`get_cell_size()` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/031`, with `bin/`/`.sconsign5.dblite` hardlinked from the main tree per `projects/godot.md`'s documented technique) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 25 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on all 3 attempts** |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary/repro | `World3D::get_navigation_map (this=0x0)` at `world_3d.cpp:72`, called from `GridMap::get_navigation_map` at `grid_map.cpp:313`, from a `Node::_spam` worker thread - confirms the exact mechanism, identical shape to candidates 014/019-024/026 |
| Fix (`ERR_READ_THREAD_GUARD_V(RID())` + explicit `Ref<World3D>::is_valid()` check before dereferencing), incremental rebuild (ccache-warm hardlinked worktree, ~83s first rebuild after the edit) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild, run; reapply fix, rebuild, run) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (deterministic - every attempt crashed within a few
seconds of starting the worker threads, not timing-dependent), and 8/8 clean runs with the fix
applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `GridMap` class through nothing but the public GDScript API.
No second participant is involved - a same-process crash reachable from a single GDScript `Thread`
calling one public method (`get_navigation_map()`) off the main thread.

## Class

`crash` (null-pointer dereference - a single worker thread calling `get_navigation_map()` crashes
deterministically every time it's reached off the main thread, while the node is inside the tree
and no `navigation_map` override has been set - the common default configuration).

## Root cause

`GridMap::get_navigation_map()`:

```cpp
RID GridMap::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (is_inside_tree()) {
		return get_world_3d()->get_navigation_map();          // <-- unconditional dereference
	}
	return RID();
}
```

Identical mechanism to candidates 014/019-024/026: `Node3D::get_world_3d()`
(`scene/3d/node_3d.cpp:1074-1079`) begins with `ERR_READ_THREAD_GUARD_V(Ref<World3D>());`
(the thread guard fires *before* even checking `is_inside_world()`), returning a null
`Ref<World3D>` when called off the main thread/a thread group. `get_navigation_map()` dereferences
that result unconditionally, crashing inside `World3D::get_navigation_map()`
(`scene/resources/3d/world_3d.cpp:72`) on `this == nullptr` - confirmed via gdb:
`World3D::get_navigation_map (this=0x0) at scene/resources/3d/world_3d.cpp:72`, the very first
line (`if (navigation_map.is_null())`) reads a member through the null `this` pointer.

Reachability: the bound, GDScript-callable public method
(`ClassDB::bind_method(D_METHOD("get_navigation_map"), ...)`, `grid_map.cpp:1423`) is reachable
from any thread with no special setup - a deterministic crash on every off-main-thread call, as
long as `map_override` isn't set (the default).

## Introduced by

41c529a94d7c690dca127bbc4f37c7f65d040f6b "Add GridMap function to change navigation map for baked
navigation regions" (2022-09-07) - this commit added `GridMap::get_navigation_map()` itself (and
its `ClassDB` binding) in exactly the unguarded shape shown above; never retrofitted with the
`ERR_READ_THREAD_GUARD_V` idiom `get_world_3d()` itself already uses.

## The fix

Same shape as candidates 014/019-024/026 (`+5/-1` against `modules/gridmap/grid_map.cpp`, see
`patch.diff`):

```cpp
RID GridMap::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (is_inside_tree()) {
		ERR_READ_THREAD_GUARD_V(RID());
		Ref<World3D> world_3d = get_world_3d();
		if (world_3d.is_valid()) {
			return world_3d->get_navigation_map();
		}
	}
	return RID();
}
```

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed).

- **Applied for real** in an isolated worktree (`.worktrees/031`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end (only the pre-existing `___BUGHUNT___/`/
  `___OPTIMIZATIONS___/` untracked dirs and the pre-existing `.gitignore` modification, both
  predating this candidate).
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `TileMapLayer::get_navigation_map()` (`modules/tilemap/tile_map_layer.cpp:3473-3480`) has the
  exact same bug shape (flagged in candidate 026's notes, still unconfirmed) - left for its own
  candidate, not pursued under this one's number per the one-bug-per-candidate convention.
- `CanvasLayer::set_follow_viewport()` (`scene/main/canvas_layer.cpp:276-306`) has the related
  setter-shaped variant (also flagged in candidate 026's notes) - also left for its own candidate.
- Other `GridMap` call sites of `get_world_3d()->` in the same file (`:843`, `:1056`, `:2102`,
  `:2160`) were not touched - `:843`/`:1056` are inside `_octant_update()`/octant rebuild paths
  reached only from main-thread scene operations per the engine's own threading contract; `:2102`/
  `:2160` are editor-only gizmo/debug code. Not independently verified as main-thread-only this
  session - a future pass could check each explicitly rather than assuming.
