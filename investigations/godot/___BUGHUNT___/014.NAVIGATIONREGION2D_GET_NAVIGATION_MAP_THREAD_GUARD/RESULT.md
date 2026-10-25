# `NavigationRegion2D::get_navigation_map()` dereferences `get_world_2d()`'s thread-guard-null result unchecked

Started from an external sweep's broader hypothesis
(`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md`, 2026-10-05): "`NavigationRegion2D`/
`Link2D`/`Obstacle2D` (+3D siblings), `GridMap`, `CanvasLayer`, `TileMapLayer`, `Window`,
`MultiplayerSpawner` all register with a server from a `NOTIFICATION_ENTER_TREE` handler that only
the main thread runs - a worker thread touching the node before that finishes races it." Building an
actual repro modeled on the sweep's own methodology (instantiate -> `add_child` -> hammer random
bound methods from worker threads) and reading the resulting crash backtrace revealed a **narrower
and more precise** mechanism than the enter-tree-timing hypothesis: `add_child()` processes
`NOTIFICATION_ENTER_TREE` **synchronously** (`Node::_propagate_enter_tree()`, called directly from
`Node::_set_tree()`, itself called synchronously as part of `add_child()`'s own call chain) - so by
the time worker threads are spawned, `_region_enter_navigation_map()` has already run. The actual
crash is unrelated to that timing: `NavigationRegion2D::get_navigation_map()`
(`scene/2d/navigation/navigation_region_2d.cpp:224`) calls `get_world_2d()` (`CanvasItem`'s own
`ERR_READ_THREAD_GUARD_V(Ref<World2D>())`-guarded getter, which logs an error and returns a null
`Ref<World2D>()` when called off the main thread/a thread group) and **immediately dereferences the
result unconditionally**, with no null check of its own.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| GDScript-level repro (`gdscript_repro/`, modeled on the external sweep's own harness methodology: instantiate, `add_child`, spawn 6 worker `Thread`s hammering a mix of `NavigationRegion2D` setters/getters including `get_navigation_map()`), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes` (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb), `--headless --path gdscript_repro` under `timeout -k 5 25` | **Segfaulted (SIGSEGV) on the first run and all 3 runs tried**, each within a couple seconds - not an ASan-reported heap/stack overflow, a plain null-pointer dereference |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` to get a symbolized backtrace | Same binary/repro | Fully symbolized: `World2D::get_navigation_map (this=0x0)` called from `NavigationRegion2D::get_navigation_map` at `navigation_region_2d.cpp:228`, from a `CoreBind::Thread` worker - confirms the exact null-deref mechanism |
| Fix (`ERR_READ_THREAD_GUARD_V(RID())` + explicit `Ref<World2D>::is_valid()` check before dereferencing), same build/run, 8 runs | Same build/run | **Clean 8/8 runs** |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-013 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried, each crashing within a couple
seconds), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `NavigationRegion2D` class through nothing but
the public GDScript API (`NavigationRegion2D.new()`, `add_child()`, `get_navigation_map()` and
sibling setters/getters). No second participant is involved (not a peer/wire-input finding) - this
is a same-process crash reachable from a single GDScript `Thread` calling one public method off
the main thread.

## Class

`crash` (null-pointer dereference - distinct from this session's other `memory-safety`/`data-race`
findings, since no concurrent mutation or corruption is required at all: a *single* worker thread
calling `get_navigation_map()` crashes deterministically every time, with or without any other
thread involved)

## Root cause

`NavigationRegion2D::get_navigation_map()`:

```cpp
RID NavigationRegion2D::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (is_inside_tree()) {
		return get_world_2d()->get_navigation_map();          // <-- unconditional dereference
	}
	return RID();
}
```

`CanvasItem::get_world_2d()` (`scene/main/canvas_item.cpp:1252-1254`) begins with
`ERR_READ_THREAD_GUARD_V(Ref<World2D>());` - when called from any thread other than the main
thread (or a registered "thread group"), this macro logs the error seen in this candidate's repro
output and returns a default-constructed (null) `Ref<World2D>`. This is Godot's own documented,
intentional safety idiom for methods that aren't safe to call off the main thread: fail softly with
a clear error and a safe default, rather than let undefined behavior happen. `get_navigation_map()`
calls this guarded getter but was never given a matching guard (or even a null check) of its own -
`get_world_2d()` fails exactly as designed, and the very next expression crashes anyway. The bound,
GDScript-callable public method (`ClassDB::bind_method(D_METHOD("get_navigation_map"), ...)`,
`navigation_region_2d.cpp:322`) is reachable from any thread with no special setup - this is not a
race in the TOCTOU sense at all, it's a deterministic crash on every off-main-thread call (as long
as `map_override` isn't set, which is the common case for the default/no-override configuration).

Reachability: any GDScript code (a background worker thread doing pathfinding queries, for example
- a plausible real use case, not a contrived one) calling `NavigationRegion2D.get_navigation_map()`
off the main thread crashes the engine, with zero concurrency/timing required.

## Introduced by

a9dc53d152 "Remove Navigation2D/3D nodes, and move the navigation map to the world resource"
(2021-03-08) - this is the commit that gave `NavigationRegion2D::get_navigation_map()` its current
`get_world_2d()->get_navigation_map()` body. `ERR_READ_THREAD_GUARD_V` itself was added to the
engine later as part of Godot 4.x's broader thread-safety-guard infrastructure, so this call site
simply predates that convention and was never retrofitted - not a regression introduced by a later
change, a pre-existing gap that the rest of the engine's safety idiom grew up around without ever
reaching this spot.

## The fix

Add the same `ERR_READ_THREAD_GUARD_V` idiom `CanvasItem::get_world_2d()` itself already uses, plus
an explicit null check before dereferencing (`+5/-1` against
`scene/2d/navigation/navigation_region_2d.cpp`, see `patch.diff`):

```cpp
RID NavigationRegion2D::get_navigation_map() const {
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

The explicit `ERR_READ_THREAD_GUARD_V(RID())` (rather than relying solely on the null check) gives
callers one clear, specific error naming `get_navigation_map()` itself - matching the engine's own
idiom used throughout `CanvasItem`/`Node` - instead of the more confusing indirect message about
`get_world_2d()` that a caller never directly invoked. The `Ref<World2D>::is_valid()` check is kept
as defense in depth for the (rarer) case `get_world_2d()` returns null for a different reason (e.g.
`tl->get_viewport()` being null), not just the thread-guard path.

Verified: 8/8 clean runs of `gdscript_repro/` against the fix (vs. crashing on all 3/3 unfixed runs
tried).

- **Applied for real** to `scene/2d/navigation/navigation_region_2d.cpp`, rebuilt the ASan editor
  binary (incremental, ~53s - single `.cpp` changed), ran `gdscript_repro/` and the test suite
  against the fix, then reverted (`git checkout --
  scene/2d/navigation/navigation_region_2d.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan +
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed | 3
  skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 failure,
  unrelated to this change). No new failures.

## What's not done / open questions

- **This exact same pattern (`get_world_2d()->`/`get_world_3d()->` dereferenced unconditionally,
  with no thread guard or null check) is extremely widespread** - confirmed via `grep -rn
  "get_world_2d()->\|get_world_3d()->" scene/` to appear in at least `NavigationRegion3D`
  (`:217,358,381,648,672`), `NavigationLink2D` (`:137,183,346,388`), `NavigationLink3D`
  (`:66,344,511`), `NavigationObstacle3D` (`:101,307`), `NavigationAgent3D` (several sites via
  `agent_parent->get_world_3d()->`), plus unrelated non-navigation classes
  (`CollisionObject3D`/`VisualInstance3D`/`Path3D`/`RayCast3D`/`ShapeCast3D`/`SpringArm3D`/
  `SoftBody3D`). Every one of these is a same-shape candidate for an identical crash, not
  independently confirmed this session - only `NavigationRegion2D::get_navigation_map()` was built,
  reproduced, and fixed. Added to `IDEAS.md`'s backlog as a dedicated follow-up item rather than
  bundled into this one candidate (per "one bug per candidate folder").
- `NavigationRegion2D` has **other** internal call sites of `get_world_2d()->get_navigation_map()`
  (`:286` in `_navigation_map_changed()`, `:420` in `_region_enter_navigation_map()`, `:604` in a
  debug-only method) that were not touched - `:420`/`:604` are only ever invoked from
  `NOTIFICATION_ENTER_TREE`/debug-draw paths that the engine's own contract already restricts to
  the main thread, and `:286`'s `_navigation_map_changed()` is a private signal callback not
  directly reachable from arbitrary GDScript, so none of these three were independently confirmed
  as reachable off the main thread the way the public, bound `get_navigation_map()` getter is -
  left alone rather than guessed at.
- The external sweep's own broader "scene-tree main-thread-only servicing race" hypothesis (the
  `NOTIFICATION_ENTER_TREE`-timing mechanism) may still be real for *other* classes in its list
  (`GridMap`, `CanvasLayer`, `TileMapLayer`, `Window`, `MultiplayerSpawner`, the Navigation
  `Obstacle`/`Link` family) - this candidate only disproves it as the mechanism for this one
  specific `NavigationRegion2D` crash, it doesn't rule out the broader pattern existing elsewhere.
  A future candidate should verify the actual mechanism per-class with a real repro + backtrace
  rather than assuming the sweep's original hypothesis transfers unchanged, the same lesson this
  candidate learned here.
