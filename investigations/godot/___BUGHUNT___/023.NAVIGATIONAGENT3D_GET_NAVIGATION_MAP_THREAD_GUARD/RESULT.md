# `NavigationAgent3D::get_navigation_map()` dereferences `agent_parent->get_world_3d()`'s thread-guard-null result unchecked

Fifth follow-up from `IDEAS.md`'s `get_world_2d()->`/`get_world_3d()->` sweep (candidates
014/019/020/021/022), and the last unconfirmed navigation sibling. `NavigationAgent3D` is a plain
`Node` (not `Node3D`) that tracks a `Node3D` parent (`agent_parent`) and calls
`agent_parent->get_world_3d()` rather than its own `get_world_3d()` - a slightly different shape
from the previous four candidates, so this one was independently repro'd rather than assumed to
transfer unchanged, per candidate 014's own lesson.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| GDScript-level repro (`repro/`: a `Node3D` root with a `NavigationAgent3D` child so `agent_parent` gets set via the entry-tree notification, then 6 worker `Thread`s hammering a mix of setters/getters including `get_navigation_map()`), against the real, already-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on all 3/3 runs tried**, each within a few seconds |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary/repro | `World3D::get_navigation_map (this=0x0)` at `world_3d.cpp:72`, called from `NavigationAgent3D::get_navigation_map` at `navigation_agent_3d.cpp:591`, from a `Node3D::_spam` worker thread - confirms the exact mechanism |
| Fix (`ERR_READ_THREAD_GUARD_V(RID())` on `this`, plus `Ref<World3D>::is_valid()` check before dereferencing `agent_parent->get_world_3d()`), incremental rebuild (ccache-warm, ~50s), 8 runs | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried, each crashing within a few
seconds, deterministic not timing-dependent), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `NavigationAgent3D` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from a single
GDScript `Thread` calling one public method off the main thread.

## Class

`crash` (null-pointer dereference - a single worker thread calling `get_navigation_map()` crashes
deterministically every time once `agent_parent` is set, which happens automatically as soon as
the agent is added as a child of a `Node3D`; the other 5 threads only exercise sibling
setters/getters, not required to trigger this specific crash).

## Root cause

`NavigationAgent3D::get_navigation_map()`:

```cpp
RID NavigationAgent3D::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (agent_parent != nullptr) {
		return agent_parent->get_world_3d()->get_navigation_map();          // <-- unconditional dereference
	}
	return RID();
}
```

Same underlying mechanism as candidates 014/019/020/021/022, but through one extra level of
indirection: `NavigationAgent3D` is a plain `Node` that tracks its `Node3D` parent in
`agent_parent` (set via `set_agent_parent()` during the enter-tree notification), and calls
`agent_parent->get_world_3d()` instead of a `get_world_3d()` of its own. `Node3D::get_world_3d()`
still begins with `ERR_READ_THREAD_GUARD_V(Ref<World3D>());`, returning null off the main
thread/a thread group - the guard's `is_readable_from_caller_thread()` check reflects the calling
thread's own state (main thread vs. a registered thread group), not anything specific to which
node instance it's invoked on, so it fires identically whether checked on `agent_parent` or on
`this` (`NavigationAgent3D` is itself inside the tree whenever `agent_parent` is set, since both
become true together during the same enter-tree notification). `get_navigation_map()`
dereferences the result unconditionally, crashing inside `World3D::get_navigation_map()`
(`scene/resources/3d/world_3d.cpp:72`) on `this == nullptr`.

Reachability: the bound, GDScript-callable public method
(`ClassDB::bind_method(D_METHOD("get_navigation_map"), ...)`, `navigation_agent_3d.cpp:103`) is
reachable from any thread with no special setup - a deterministic crash on every off-main-thread
call (as long as `map_override` isn't set, the common default configuration, and the agent has
been added to a scene tree so `agent_parent` is non-null - also the common case).

## Introduced by

f10ff0efda43d4856ddea5a778e39c114fe6ce80 "Add NavigationAgent set_navigation_map() function"
(2022-06-14) - the commit that added `NavigationAgent3D::get_navigation_map()` with its current
`agent_parent->get_world_3d()->get_navigation_map()` body. Never retrofitted with the
`ERR_READ_THREAD_GUARD_V` idiom `get_world_3d()` itself already uses.

## The fix

Same shape as candidates 014/019/020/021/022, guarding on `this` (since `this` and `agent_parent`
are both inside the tree together whenever `agent_parent` is non-null) rather than on
`agent_parent` directly (`+5/-1` against `scene/3d/navigation/navigation_agent_3d.cpp`, see
`patch.diff`):

```cpp
RID NavigationAgent3D::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (agent_parent != nullptr) {
		ERR_READ_THREAD_GUARD_V(RID());
		Ref<World3D> world_3d = agent_parent->get_world_3d();
		if (world_3d.is_valid()) {
			return world_3d->get_navigation_map();
		}
	}
	return RID();
}
```

Verified: 8/8 clean runs of `repro/` against the fix (vs. crashing on all 3/3 unfixed runs tried).

- **Applied for real** to `scene/3d/navigation/navigation_agent_3d.cpp`, rebuilt the ASan editor
  binary, ran `repro/` and the full test suite against the fix, then reverted (`git checkout --
  scene/3d/navigation/navigation_agent_3d.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before this
  session's work and after the fix - identical to the established baseline (same pre-existing
  `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `NavigationAgent3D` has another internal call site of `agent_parent->get_world_3d()->` in the
  same file (`:849`, inside the pathfinding query setup) - not touched; this runs from
  `_update_navigation()`, which per the class's own threading contract (an internal method driven
  by the agent's own physics-process callback) is already main-thread-only, not independently
  confirmed reachable off the main thread.
- This closes out the navigation-class portion of `IDEAS.md`'s `get_world_2d()->`/
  `get_world_3d()->` sweep: all five flagged navigation siblings (`NavigationRegion2D`/3D,
  `NavigationLink2D`/3D, `NavigationObstacle3D`, `NavigationAgent3D`) are now independently
  confirmed and fixed (014, 019, 020, 021, 022, 023 - 6/6 hit rate on this exact pattern). The
  remaining items from the original sweep are non-navigation classes
  (`CollisionObject3D`, `VisualInstance3D`, `Path3D`, `RayCast3D`, `ShapeCast3D`, `SpringArm3D`,
  `SoftBody3D`) - a distinct follow-up, not yet attempted at all, and not guaranteed to share the
  exact same getter-shape (each needs to be read first to confirm it even has an unguarded public
  getter like `get_navigation_map()`, rather than just an internal use).
