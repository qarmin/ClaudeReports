# `CollisionObject3D::set_disable_mode()` dereferences `get_world_3d()`'s thread-guard-null result unchecked

First non-navigation follow-up from `IDEAS.md`'s `get_world_2d()->`/`get_world_3d()->` sweep.
Unlike candidates 014/019-023 (all a `get_navigation_map()`-shaped *getter*), reading
`CollisionObject3D` (`scene/3d/physics/collision_object_3d.cpp`) first (per this workspace's own
closing note on candidate 023 - don't assume the non-navigation siblings share the exact same
getter shape) found a structurally different but mechanically identical bug in a public *setter*:
`set_disable_mode()` → `_apply_enabled()` → `get_world_3d()->get_space()`, unconditionally
dereferenced.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: grepped `get_world_2d()->`/`get_world_3d()->` in `collision_object_3d.cpp`, `visual_instance_3d.cpp`, `path_3d.cpp`, `ray_cast_3d.cpp`, `shape_cast_3d.cpp`, `spring_arm_3d.cpp`, `soft_body_3d.cpp` and read each call site's context | n/a | Most hits (`spring_arm_3d.cpp`'s `process_spring()`, `soft_body_3d.cpp`'s `NOTIFICATION_ENTER_WORLD` handler, the various debug-mesh `get_scenario()` calls) are only reachable from `NOTIFICATION_*` handlers or internal physics-process callbacks already restricted to the main thread - not independently reproducible off-thread. `CollisionObject3D::_apply_enabled()`'s `DISABLE_MODE_REMOVE` case stood out: reachable via the **public, bound** `set_disable_mode()` setter |
| GDScript-level repro (`repro/`: a `Node3D` root with an `Area3D` child, `process_mode` set to `PROCESS_MODE_DISABLED` on the main thread first (safe - this only triggers `NOTIFICATION_DISABLED`'s `_apply_disabled()`, not `_apply_enabled()`), then 6 worker `Thread`s calling `set_disable_mode()` alternating between `DISABLE_MODE_MAKE_STATIC`/`DISABLE_MODE_KEEP_ACTIVE`), against the real, already-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on all 3/3 runs tried**, each on the very first worker-thread call (since `disable_mode` starts at its default `DISABLE_MODE_REMOVE`, the very first `set_disable_mode()` call from any thread hits the buggy branch before updating the field) |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary/repro | `World3D::get_space (this=0x0)` at `world_3d.cpp:58`, called from `CollisionObject3D::_apply_enabled` at `collision_object_3d.cpp:276`, called from `CollisionObject3D::set_disable_mode` at `collision_object_3d.cpp:228`, from a `Node3D::_spam` worker thread - confirms the exact mechanism, same thread-guard-null-deref family as 014/019-023 |
| Fix (`ERR_THREAD_GUARD` at the top of `set_disable_mode()`), incremental rebuild (ccache-warm, ~46s), 8 runs | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried, each crashing within a few
seconds, deterministic not timing-dependent given the default `disable_mode`), and 8/8 clean runs
with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `CollisionObject3D` (via `Area3D`) class through nothing but
the public GDScript API (`process_mode` property, `set_disable_mode()`). No second participant is
involved - a same-process crash reachable from a single GDScript `Thread` calling one public
method off the main thread.

## Class

`crash` (null-pointer dereference - any single worker thread calling `set_disable_mode()` with a
mode different from the node's current `disable_mode`, while the node is inside the tree and
disabled via `process_mode`, crashes deterministically; the other 5 threads in the repro only add
contention on the same call, not required to trigger the crash).

## Root cause

`CollisionObject3D::set_disable_mode()`:

```cpp
void CollisionObject3D::set_disable_mode(DisableMode p_mode) {
	if (disable_mode == p_mode) {
		return;
	}
	bool disabled = is_inside_tree() && !is_enabled();
	if (disabled) {
		// Cancel previous disable mode.
		_apply_enabled();
	}
	...
}
```

`_apply_enabled()`'s `DISABLE_MODE_REMOVE` case:

```cpp
case DISABLE_MODE_REMOVE: {
	if (is_inside_tree()) {
		RID space = get_world_3d()->get_space();          // <-- unconditional dereference
		...
	}
} break;
```

`Node3D::get_world_3d()` begins with `ERR_READ_THREAD_GUARD_V(Ref<World3D>());`, returning a null
`Ref<World3D>` when called off the main thread/a thread group - the same guard candidates
014/019-023 already found unchecked in various getters, here unchecked in a setter's internal
helper instead. The crash happens inside `World3D::get_space()` (`scene/resources/3d/world_3d.cpp:58`)
dereferencing `this == nullptr`.

Reachability: `set_disable_mode()` is a public, bound, GDScript-callable setter
(`ClassDB::bind_method(D_METHOD("set_disable_mode", "mode"), ...)`,
`collision_object_3d.cpp:489`) reachable from any thread with no special setup, as long as the node
is inside the tree, currently disabled (`process_mode == PROCESS_MODE_DISABLED`, or any other
condition `is_enabled()` treats as disabled), and the requested mode differs from the node's
current `disable_mode` - all easily satisfied from ordinary GDScript, not a contrived edge case.

## Introduced by

5cbdc7a0acbefcc3aa3cb0eb0fe05b5b10fe7370 "Add support for controlling physics nodes' behavior when
disabled" (2021-06-17) - the commit that added `set_disable_mode()`/`_apply_enabled()` with this
`get_world_3d()->get_space()` body. Never retrofitted with the `ERR_THREAD_GUARD` idiom other
unsafe setters/methods in the engine already use (the same gap candidate 017 found and fixed in
`MenuButton::show_popup()`).

## The fix

A single `ERR_THREAD_GUARD` at the top of the public entry point, matching candidate 017's fix
shape for a void bound method (`+2/-0` against
`scene/3d/physics/collision_object_3d.cpp`, see `patch.diff`):

```cpp
void CollisionObject3D::set_disable_mode(DisableMode p_mode) {
	ERR_THREAD_GUARD

	if (disable_mode == p_mode) {
		return;
	}
	...
}
```

Guarding the public entry point (rather than adding a null check inside `_apply_enabled()`) also
protects `_apply_enabled()`'s other, currently-safe-looking call sites reached through
`set_disable_mode()` from ever being called off-thread in the future, consistent with how
candidate 017 guarded `MenuButton::show_popup()` itself rather than patching
`get_window()`'s caller alone.

Verified: 8/8 clean runs of `repro/` against the fix (vs. crashing on all 3/3 unfixed runs tried).

- **Applied for real** to `scene/3d/physics/collision_object_3d.cpp`, rebuilt the ASan editor
  binary, ran `repro/` and the full test suite against the fix, then reverted (`git checkout --
  scene/3d/physics/collision_object_3d.cpp`) before finishing this candidate - main tree confirmed
  clean (`git status`) afterward.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before this
  session's work and after the fix - identical to the established baseline (same pre-existing
  `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `_apply_enabled()` is also called directly from `_notification()`'s `NOTIFICATION_ENABLED`/
  `NOTIFICATION_EXIT_WORLD` handlers - both already restricted to the main thread by the engine's
  own scene-tree notification contract, not independently confirmed reachable off-thread, left
  alone.
- The remaining non-navigation siblings from the original sweep - `VisualInstance3D`, `Path3D`,
  `RayCast3D`, `ShapeCast3D`, `SpringArm3D`, `SoftBody3D` - were read this session (see "What was
  tried" above) but their `get_world_2d()->`/`get_world_3d()->` call sites all turned out to be
  inside notification handlers or internal process callbacks already restricted to the main
  thread, **not** behind a public bound method the way `CollisionObject3D::set_disable_mode()` or
  the earlier `get_navigation_map()` getters were. None of them produced an independently
  confirmable candidate this session - this is a genuine negative result for those six classes
  specifically (not "not yet checked"), distinct from the six confirmed hits in the navigation
  family. A future session should still double check `RayCast3D`/`ShapeCast3D`'s own bound
  methods more thoroughly (their `.h` files define their own `is_enabled()`, suggesting a possibly
  analogous disable/enable path not yet traced to a `get_world_3d()` call) before fully closing
  this out.
