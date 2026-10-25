# `TileMapLayer::get_navigation_map()` dereferences `get_world_2d()`'s thread-guard-null result unchecked

Flagged by candidate 026's own follow-up notes as a very high-confidence unconfirmed sibling of the
already-9/9-confirmed `get_world_2d()->`/`get_world_3d()->` unchecked-dereference family
(candidates 014, 019-024, 026, 031). Same body shape as candidate 031's `GridMap` (2D variant this
time), on `modules/tilemap/tile_map_layer.cpp`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `modules/tilemap/tile_map_layer.cpp:3473-3480` (`TileMapLayer::get_navigation_map()`) and `scene/main/canvas_item.cpp:1253-1263` (`CanvasItem::get_world_2d()`) | Static read | Byte-for-byte the same shape as candidates 014/019-024/026/031: `is_inside_tree()` guarded, but `get_world_2d()`'s own `ERR_READ_THREAD_GUARD_V(Ref<World2D>())` result is dereferenced unconditionally one line later. Confirmed `get_navigation_map()` is `ClassDB`-bound (`modules/tilemap/tile_map_layer.cpp:2252`), reachable from GDScript |
| GDScript-level repro (`repro/`: instantiate `TileMapLayer`, `add_child`, spawn 4 worker `Thread`s calling a mix of `get_navigation_map()`/`is_navigation_enabled()`/`is_enabled()` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/032`, `bin/`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 25 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on all 3 attempts** |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary/repro | `World2D::get_navigation_map (this=0x0)` at `world_2d.cpp:52`, called from `TileMapLayer::get_navigation_map` at `tile_map_layer.cpp:3477`, from a `Node::_spam` worker thread - confirms the exact mechanism, identical shape to candidates 014/019-024/026/031 |
| Fix (`ERR_READ_THREAD_GUARD_V(RID())` + explicit `Ref<World2D>::is_valid()` check before dereferencing), incremental rebuild (ccache-warm hardlinked worktree, ~57s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (deterministic - every attempt crashed within a few
seconds of starting the worker threads), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `TileMapLayer` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from a single
GDScript `Thread` calling one public method (`get_navigation_map()`) off the main thread.

## Class

`crash` (null-pointer dereference - a single worker thread calling `get_navigation_map()` crashes
deterministically every time it's reached off the main thread, while the node is inside the tree
and no `navigation_map_override` has been set - the common default configuration).

## Root cause

`TileMapLayer::get_navigation_map()`:

```cpp
RID TileMapLayer::get_navigation_map() const {
	if (navigation_map_override.is_valid()) {
		return navigation_map_override;
	} else if (is_inside_tree()) {
		return get_world_2d()->get_navigation_map();          // <-- unconditional dereference
	}
	return RID();
}
```

Identical mechanism to candidates 014/019-024/026/031: `CanvasItem::get_world_2d()`
(`scene/main/canvas_item.cpp:1253-1263`) begins with `ERR_READ_THREAD_GUARD_V(Ref<World2D>());`,
returning a null `Ref<World2D>` when called off the main thread/a thread group.
`get_navigation_map()` dereferences that result unconditionally, crashing inside
`World2D::get_navigation_map()` (`scene/resources/world_2d.cpp:52`) on `this == nullptr` -
confirmed via gdb: `World2D::get_navigation_map (this=0x0) at scene/resources/world_2d.cpp:52`,
the very first line (`if (navigation_map.is_null())`) reads a member through the null `this`
pointer.

Reachability: the bound, GDScript-callable public method
(`ClassDB::bind_method(D_METHOD("get_navigation_map"), ...)`, `tile_map_layer.cpp:2252`) is
reachable from any thread with no special setup - a deterministic crash on every off-main-thread
call, as long as `navigation_map_override` isn't set (the default).

## Introduced by

787c784aca28f19841b2eb79b702e517007d9901 "Remove almost all remaining dependencies of TileMapLayer
on TileMap" (2024-02-28). Before this commit, `get_navigation_map()` only returned
`navigation_map`/the override, with no fallback at all; this commit added the
`is_inside_tree()`-guarded `return get_world_2d()->get_navigation_map();` fallback branch (part of
decoupling `TileMapLayer` from needing a parent `TileMap` node) in exactly the unguarded shape
shown above - `git log -S"get_world_2d()->get_navigation_map()"` and reading the commit's diff
directly confirm this is the exact hunk that added it.

## The fix

Same shape as candidates 014/019-024/026/031 (`+5/-1` against
`modules/tilemap/tile_map_layer.cpp`, see `patch.diff`):

```cpp
RID TileMapLayer::get_navigation_map() const {
	if (navigation_map_override.is_valid()) {
		return navigation_map_override;
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

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed).

- **Applied for real** in an isolated worktree (`.worktrees/032`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `CanvasLayer::set_follow_viewport()` (`scene/main/canvas_layer.cpp:276-306`) has the related
  setter-shaped variant flagged in candidate 026's notes - still not independently repro'd, left
  for its own candidate.
- Other call sites of `get_world_2d()`/`get_world_3d()` elsewhere in `tile_map_layer.cpp` were not
  audited this session - this candidate scoped strictly to `get_navigation_map()` per the
  one-bug-per-candidate convention.
