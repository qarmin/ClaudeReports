# `RayCast3D::set_enabled()` / `ShapeCast3D::set_enabled()` dereference `get_world_3d()`'s thread-guard-null result unchecked

Follow-up on candidate 024's own closing note: `RayCast3D`/`ShapeCast3D` declare their own
`is_enabled()` rather than inheriting `Node::is_enabled()`, and were flagged as worth checking for
a `CollisionObject3D::set_disable_mode()`-shaped setter reaching `get_world_3d()` through a
differently-named helper the plain `get_world_2d()->`/`get_world_3d()->` text grep had already
read but, in that earlier session, attributed only to `NOTIFICATION_*` handlers. Re-reading both
classes' `set_enabled()` found a second, previously-missed call path into the same already-found
`_update_debug_shape()` helper (which does contain the literal `get_world_3d()->` text, and was
read during the original sweep) - the earlier session traced `_update_debug_shape()`'s callers from
`NOTIFICATION_ENTER_TREE`/`NOTIFICATION_INTERNAL_PHYSICS_PROCESS` (both main-thread-restricted) but
missed that the public, bound `set_enabled(bool)` setter also calls it directly, gated only on
`is_inside_tree() && PhysicsServer3D::debug_is_enabled()` - neither of which restricts the caller's
thread.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: re-read `scene/3d/physics/ray_cast_3d.cpp`/`shape_cast_3d.cpp` end to end, tracing every caller of `_update_debug_shape()` (not just the `NOTIFICATION_*` ones read previously) | n/a | `RayCast3D::set_enabled()`/`ShapeCast3D::set_enabled()` (public, bound, `ADD_PROPERTY`-exposed as the `enabled` property's setter) both call `_update_debug_shape()` directly when `is_inside_tree() && PhysicsServer3D::get_singleton()->debug_is_enabled()` - no thread restriction. `_update_debug_shape()` ends with `RenderingServer::get_singleton()->instance_set_scenario(debug_instance, get_world_3d()->get_scenario())` inside `if (is_inside_tree())`, unconditionally dereferencing `get_world_3d()`'s result. (`RayCast3D::set_target_position()` and `ShapeCast3D::set_target_position()` have an analogous, separate reachable path via `debug_instance.is_valid()`/`debug_is_enabled()` - noted but not pursued this session, see "What's not done" below) |
| GDScript-level repro (`repro/`: a `Node3D` root that calls `get_tree().set_debug_collisions_hint(true)` on the main thread *before* adding a `RayCast3D` child (safe - `NOTIFICATION_ENTER_TREE`'s own `_update_debug_shape()` call then runs on the main thread, priming `debug_instance`), then spawns 6 worker `Thread`s calling `ray.set_enabled(true)` in a tight loop), run against the real, already-built ASan editor binary rebuilt from this session's own worktree at commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb (hardlinked `bin/`/`.sconsign5.dblite` from the main tree's already-built artifacts into an isolated git worktree, `cp -al`, to avoid a from-scratch sanitizer rebuild - incremental rebuild confirmed to only recompile the two touched `.cpp` files and relink, ~50s-1m30s per rebuild) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on all 3/3 unfixed runs**, each within well under a second of the first worker thread's first `set_enabled(true)` call |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary/repro | `World3D::get_scenario (this=0x0)` at `world_3d.cpp:88`, called from `RayCast3D::_update_debug_shape` at `ray_cast_3d.cpp:548`, called (via the engine's `MethodBindT`/`call_with_validated_object_instance_args` dispatch) from a `Node3D::_spam` worker thread executing the GDScript `ray.set_enabled(true)` call - confirms the exact mechanism, same thread-guard-null-deref family as 014/019-024 |
| Fix (`ERR_THREAD_GUARD` at the top of `RayCast3D::set_enabled()` only), incremental rebuild (ccache-warm + hardlinked baseline, ~50s), 5 runs of `repro/` | Same build/run | **Clean 5/5 runs**, "survived" printed every time, exit 0 |
| Second, independent GDScript repro (`repro_shapecast/`: identical shape, `ShapeCast3D` in place of `RayCast3D`, no `Shape3D` resource needed - `_update_debug_shape_vertices()` already guards the shape-dependent part behind `shape.is_valid()`), run against the unfixed binary (reverted `ray_cast_3d.cpp`/`shape_cast_3d.cpp` back to baseline via `git checkout --`, confirmed via `grep ERR_THREAD_GUARD` returning nothing, rebuilt - scons reported `.` up to date, i.e. content-hash-identical to the original baseline build) | Same command, `--path repro_shapecast` | **Segfaulted (SIGSEGV, exit 139) on 3/3 unfixed runs**, symbolized backtrace: `World3D::get_scenario (this=0x0)` at `world_3d.cpp:88`, called from `ShapeCast3D::_update_debug_shape` at `shape_cast_3d.cpp:634` - textually identical mechanism to `RayCast3D`'s |
| Fix re-applied to both `RayCast3D::set_enabled()` and `ShapeCast3D::set_enabled()`, rebuilt, 3/3 runs of `repro_shapecast/` plus a re-check of `repro/` against this same combined-fix build | Same commands | **Clean 3/3 `ShapeCast3D` runs** ("survived", exit 0) and **clean `RayCast3D` re-check** against the same binary |
| Project test suite before/after (run twice: once with only `RayCast3D`'s fix, once more with both fixes applied together) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` in every run (unfixed, `RayCast3D`-only fix, and both-fixed) - identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 environment failure), no new failures at any point |

## Status

**Confirmed**, independently for both classes. `RayCast3D`: 3/3 unfixed runs crash, 5/5 fixed runs
clean. `ShapeCast3D`: 3/3 unfixed runs crash (separate repro, `repro_shapecast/`), 3/3 fixed runs
clean. Both deterministic given each repro's setup - not timing-dependent, since `debug_instance` is
primed on the main thread before any worker thread runs, and the very first `set_enabled(true)`
call from any worker thread hits the buggy branch.

**Fully reproducible**: **Yes**, for both classes - two real Godot projects (`repro/` for
`RayCast3D`, `repro_shapecast/` for `ShapeCast3D`), each run against a real, self-built ASan editor
binary, driving the actual class through nothing but the public GDScript API
(`SceneTree.set_debug_collisions_hint()`, the `enabled` property/`set_enabled()`). No second
participant is involved in either - a same-process crash reachable from a single GDScript `Thread`
calling one public method off the main thread.

## Class

`crash` (null-pointer dereference - any single worker thread calling `set_enabled(true)` while the
node is inside the tree and physics debug-shape drawing is on, crashes deterministically once
`debug_instance` has been primed on the main thread; the other 5 threads in each repro only add
contention on the same call, not required to trigger the crash). `ShapeCast3D::set_enabled()` has
the textually identical bug, independently confirmed with its own repro (`repro_shapecast/`) and
fixed alongside `RayCast3D`'s.

## Root cause

`RayCast3D::set_enabled()`:

```cpp
void RayCast3D::set_enabled(bool p_enabled) {
	enabled = p_enabled;
	update_gizmos();
	...
	if (is_inside_tree() && PhysicsServer3D::get_singleton()->debug_is_enabled()) {
		if (p_enabled) {
			_update_debug_shape();          // <-- no thread restriction
		} else {
			_clear_debug_shape();
		}
	}
}
```

`_update_debug_shape()`'s tail:

```cpp
	RenderingServer::get_singleton()->instance_set_base(debug_instance, debug_mesh->get_rid());
	if (is_inside_tree()) {
		RenderingServer::get_singleton()->instance_set_scenario(debug_instance, get_world_3d()->get_scenario());   // <-- unconditional dereference
		...
	}
```

`Node3D::get_world_3d()` begins with `ERR_READ_THREAD_GUARD_V(Ref<World3D>());`, returning a null
`Ref<World3D>` when called off the main thread/a thread group - the same guard candidates
014/019-024 already found unchecked in various getters/setters, here unchecked inside a private
helper (`_update_debug_shape()`) reached from a public setter (`set_enabled()`) rather than
directly inside the public entry point itself. `ShapeCast3D::set_enabled()`/
`ShapeCast3D::_update_debug_shape()` have the textually identical shape
(`scene/3d/physics/shape_cast_3d.cpp:200-218` and `:591-634`).

Reachability: `set_enabled()` is a public, bound, GDScript-callable setter (`enabled` property,
`ClassDB::bind_method(D_METHOD("set_enabled", "enabled"), ...)`) reachable from any thread with no
special setup beyond the node being inside the tree and physics debug-shape drawing being turned
on (`SceneTree.set_debug_collisions_hint(true)`, itself a public, bound, GDScript-callable method
with no thread restriction of its own - the ordinary scripting entry point for turning on collision
-shape visualization, in the editor or at runtime) - not a contrived edge case.

## Introduced by

f556d7a54b40fd9b3e4adb59dce5cdffbdbbfd0c "Change ShapeCast3D and RayCast3D debug to use
RenderingServer directly" (2024-02-27) - the commit that rewrote both classes' debug-shape drawing
from a child `MeshInstance3D` node to a `RenderingServer`-owned RID, introducing the literal
`get_world_3d()->get_scenario()` dereference at the end of `_update_debug_shape()`. This postdates
0a9f72d5a80c8957ef5172f546c9076089862cef "Make more base nodes thread safe" (2023-05-15), the
commit that added `Node3D::get_world_3d()`'s `ERR_READ_THREAD_GUARD_V` - so the thread guard
already existed engine-wide when this 2024 refactor added a new unconditional dereference of its
return value, the same "caller added after the guard, not audited against it" shape as candidates
014/019-024. The call path into this helper from `set_enabled()` itself is older still (originally
added by 93d6003aa8602262fb4878620178f51e828e5b0c, "Show 3D raycasts when debugging collisions",
2017-03-21, long before thread groups existed) and was never itself the unsafe part - it only
became unsafe once `_update_debug_shape()`'s 2024 rewrite added a thread-guarded call inside it.

## The fix

A single `ERR_THREAD_GUARD` at the top of each class's own `set_enabled()`, matching candidate
024's fix shape for a void bound method (+2/-0 per file, `+4/-0` total across
`scene/3d/physics/ray_cast_3d.cpp` and `scene/3d/physics/shape_cast_3d.cpp`, see `patch.diff`):

```cpp
void RayCast3D::set_enabled(bool p_enabled) {
	ERR_THREAD_GUARD

	enabled = p_enabled;
	...
```

(and identically for `ShapeCast3D::set_enabled()`). This makes an off-thread call print a single
descriptive error ("The caller thread can't call the function `set_enabled()` on this node...")
and return without touching any state, instead of segfaulting - the same trade-off already
accepted engine-wide for every other `ERR_THREAD_GUARD`-protected setter.

- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` before the fix, same
  `1431 | 1430 passed | 1 failed | 3 skipped` after - identical counts, same single pre-existing
  `tests/core/io/test_ip.cpp` IPv6 failure (an environment limitation unrelated to this change, not
  a regression). No new failures introduced by the fix.

## What's not done / open questions

- `_update_debug_shape()` is also called from both classes' `_notification()`'s
  `NOTIFICATION_ENTER_TREE`/`NOTIFICATION_INTERNAL_PHYSICS_PROCESS` handlers - already restricted to
  the main thread by the engine's own scene-tree notification contract, not independently confirmed
  reachable off-thread, left alone (guarding `set_enabled()` itself is sufficient; it returns before
  reaching `_update_debug_shape()` at all when called off-thread).
- `RayCast3D::set_target_position()` and `ShapeCast3D::set_target_position()` have a second,
  structurally distinct reachable path into the same unguarded `_update_debug_shape()` helper
  (`RayCast3D`'s via `else if (debug_instance.is_valid()) { _update_debug_shape(); }`, no
  `debug_is_enabled()` check needed at all once `debug_instance` has ever been primed;
  `ShapeCast3D`'s via its own explicit `is_inside_tree() && debug_is_enabled()` check, same shape as
  `set_enabled()`) - **not fixed this session**, and not independently confirmed with its own
  repro (high confidence it reproduces identically, per the same mechanism, but per this
  workspace's own policy of confirming each reachable path with its own repro before fixing it,
  this is a new, separate suspect, not folded into this candidate's fix). Left in `IDEAS.md`.
