# `NavigationLink3D::get_navigation_map()` dereferences `get_world_3d()`'s thread-guard-null result unchecked

Second follow-up from `IDEAS.md`'s `get_world_2d()->`/`get_world_3d()->` sweep (candidates 014/019):
`NavigationLink3D::get_navigation_map()` (`scene/3d/navigation/navigation_link_3d.cpp:340-347`) has
byte-for-byte the same body shape as `NavigationRegion2D`'s (014) and `NavigationRegion3D`'s (019)
already-confirmed getters. Built and ran its own independent repro + backtrace per candidate 014's
own lesson (don't assume a sibling transfers unchanged) rather than assuming the pattern purely from
code reading.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| GDScript-level repro (`repro/`, modeled on candidates 014/017/019's harness: instantiate `NavigationLink3D`, `add_child`, spawn 6 worker `Thread`s hammering a mix of setters/getters including `get_navigation_map()`), against the real, already-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on all 3/3 runs tried**, each within a few seconds |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary/repro | `World3D::get_navigation_map (this=0x0)` at `world_3d.cpp:72`, called from `NavigationLink3D::get_navigation_map` at `navigation_link_3d.cpp:344`, from a `Node::_spam` worker thread - confirms the exact mechanism, identical to candidates 014/019 |
| Fix (`ERR_READ_THREAD_GUARD_V(RID())` + explicit `Ref<World3D>::is_valid()` check before dereferencing), incremental rebuild (`scons ... cpp_compiler_launcher=ccache c_compiler_launcher=ccache -j12`, ~44s thanks to a now-warm ccache), 8 runs | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried, each crashing within a few
seconds, deterministic not timing-dependent), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `NavigationLink3D` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from a single
GDScript `Thread` calling one public method off the main thread.

## Class

`crash` (null-pointer dereference - a single worker thread calling `get_navigation_map()` crashes
deterministically every time; the other 5 threads only exercise sibling setters, not required to
trigger this specific crash).

## Root cause

`NavigationLink3D::get_navigation_map()`:

```cpp
RID NavigationLink3D::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (is_inside_tree()) {
		return get_world_3d()->get_navigation_map();          // <-- unconditional dereference
	}
	return RID();
}
```

Identical mechanism to candidates 014 (`NavigationRegion2D`) and 019 (`NavigationRegion3D`):
`Node3D::get_world_3d()` begins with `ERR_READ_THREAD_GUARD_V(Ref<World3D>());`, returning a null
`Ref<World3D>` when called off the main thread/a thread group. `get_navigation_map()` dereferences
that result unconditionally with no guard or null check of its own, crashing inside
`World3D::get_navigation_map()` (`scene/resources/3d/world_3d.cpp:72`) on `this == nullptr`.

Reachability: the bound, GDScript-callable public method
(`ClassDB::bind_method(D_METHOD("get_navigation_map"), ...)`, `navigation_link_3d.cpp:187`) is
reachable from any thread with no special setup - a deterministic crash on every off-main-thread
call (as long as `map_override` isn't set, the common default configuration).

## Introduced by

e6ff4e56e30a93b1d25b90982ac3b344e228f4a9 "Add NavigationLink function to change navigation map"
(2024-08-13) - the commit that added `NavigationLink3D::get_navigation_map()` with its current
`get_world_3d()->get_navigation_map()` body, mirroring `NavigationRegion3D`'s equivalent method
(candidate 019, added by a near-identically-named commit a year earlier). Neither was ever
retrofitted with the `ERR_READ_THREAD_GUARD_V` idiom `get_world_3d()` itself already uses.

## The fix

Same shape as candidates 014/019 (`+5/-1` against
`scene/3d/navigation/navigation_link_3d.cpp`, see `patch.diff`):

```cpp
RID NavigationLink3D::get_navigation_map() const {
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

Verified: 8/8 clean runs of `repro/` against the fix (vs. crashing on all 3/3 unfixed runs tried).

- **Applied for real** to `scene/3d/navigation/navigation_link_3d.cpp`, rebuilt the ASan editor
  binary, ran `repro/` and the full test suite against the fix, then reverted (`git checkout --
  scene/3d/navigation/navigation_link_3d.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before this
  session's work and after the fix - identical to the established baseline (same pre-existing
  `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `NavigationLink3D` has two other internal call sites of `get_world_3d()->` in the same file
  (`:66` in `_update_debug_mesh()`'s nav-map lookup, `:164` reading `get_scenario()`, `:511` in
  `_link_enter_navigation_map()`) - not touched. `:66`/`:164` only run from debug-draw paths and
  `:511` only from `NOTIFICATION_ENTER_TREE`, both already restricted to the main thread by the
  engine's own contract - not independently confirmed reachable off the main thread, left alone
  rather than guessed at, same discipline as candidates 014/019.
- The broader sweep in `IDEAS.md` still has unconfirmed siblings: `NavigationLink2D`,
  `NavigationObstacle3D`, `NavigationAgent3D`, plus non-navigation classes (`CollisionObject3D`,
  `VisualInstance3D`, `Path3D`, `RayCast3D`, `ShapeCast3D`, `SpringArm3D`, `SoftBody3D`). This
  candidate is the second independent confirmation of the exact same mechanism (3/3 for the
  pattern so far: 014, 019, 020), which raises confidence further, but each remaining class still
  needs its own repro before being written up.
