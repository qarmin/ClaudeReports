# `NavigationLink2D::get_navigation_map()` dereferences `get_world_2d()`'s thread-guard-null result unchecked

Third follow-up from `IDEAS.md`'s `get_world_2d()->`/`get_world_3d()->` sweep (candidates
014/019/020): `NavigationLink2D::get_navigation_map()`
(`scene/2d/navigation/navigation_link_2d.cpp:179-186`) has byte-for-byte the same body shape as its
already-confirmed siblings (`NavigationRegion2D` in 014, `NavigationRegion3D` in 019,
`NavigationLink3D` in 020 - the last of which was added by the exact same commit that added this
2D class). Built and ran its own independent repro + backtrace per candidate 014's own lesson
rather than assuming the pattern purely from code reading.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| GDScript-level repro (`repro/`, modeled on candidates 014/017/019/020's harness: instantiate `NavigationLink2D`, `add_child`, spawn 6 worker `Thread`s hammering a mix of setters/getters including `get_navigation_map()`), against the real, already-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on all 3/3 runs tried**, each within a few seconds |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary/repro | `World2D::get_navigation_map (this=0x0)` at `world_2d.cpp:52`, called from `NavigationLink2D::get_navigation_map` at `navigation_link_2d.cpp:183`, from a `Node::_spam` worker thread - confirms the exact mechanism, identical to candidates 014/019/020 |
| Fix (`ERR_READ_THREAD_GUARD_V(RID())` + explicit `Ref<World2D>::is_valid()` check before dereferencing), incremental rebuild (ccache-warm, ~50s), 8 runs | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried, each crashing within a few
seconds, deterministic not timing-dependent), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `NavigationLink2D` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from a single
GDScript `Thread` calling one public method off the main thread.

## Class

`crash` (null-pointer dereference - a single worker thread calling `get_navigation_map()` crashes
deterministically every time; the other 5 threads only exercise sibling setters, not required to
trigger this specific crash).

## Root cause

`NavigationLink2D::get_navigation_map()`:

```cpp
RID NavigationLink2D::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (is_inside_tree()) {
		return get_world_2d()->get_navigation_map();          // <-- unconditional dereference
	}
	return RID();
}
```

Identical mechanism to candidates 014/019/020: `CanvasItem::get_world_2d()` begins with
`ERR_READ_THREAD_GUARD_V(Ref<World2D>());`, returning a null `Ref<World2D>` when called off the
main thread/a thread group. `get_navigation_map()` dereferences that result unconditionally,
crashing inside `World2D::get_navigation_map()` (`scene/resources/world_2d.cpp:52`) on `this ==
nullptr`.

Reachability: the bound, GDScript-callable public method
(`ClassDB::bind_method(D_METHOD("get_navigation_map"), ...)`, `navigation_link_2d.cpp:46`) is
reachable from any thread with no special setup - a deterministic crash on every off-main-thread
call (as long as `map_override` isn't set, the common default configuration).

## Introduced by

e6ff4e56e30a93b1d25b90982ac3b344e228f4a9 "Add NavigationLink function to change navigation map"
(2024-08-13) - the same commit that added `NavigationLink3D`'s equivalent method (candidate 020),
adding both 2D and 3D `get_navigation_map()` getters with the same unguarded body. Neither was
ever retrofitted with the `ERR_READ_THREAD_GUARD_V` idiom `get_world_2d()`/`get_world_3d()`
themselves already use.

## The fix

Same shape as candidates 014/019/020 (`+5/-1` against
`scene/2d/navigation/navigation_link_2d.cpp`, see `patch.diff`):

```cpp
RID NavigationLink2D::get_navigation_map() const {
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

Verified: 8/8 clean runs of `repro/` against the fix (vs. crashing on all 3/3 unfixed runs tried).

- **Applied for real** to `scene/2d/navigation/navigation_link_2d.cpp`, rebuilt the ASan editor
  binary, ran `repro/` and the full test suite against the fix, then reverted (`git checkout --
  scene/2d/navigation/navigation_link_2d.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before this
  session's work and after the fix - identical to the established baseline (same pre-existing
  `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `NavigationLink2D` has two other internal call sites of `get_world_2d()->` in the same file
  (`:137` in `_edit_get_rect()`'s radius lookup, `:346` in `_link_enter_navigation_map()`, `:388`
  in `_update_debug_mesh()`) - not touched. `:137` is editor-gizmo-only (`DEBUG_ENABLED`, called
  from the editor's main-thread selection/gizmo code), `:346` only runs from
  `NOTIFICATION_ENTER_TREE`, and `:388` only from `NOTIFICATION_DRAW`'s debug-mesh update - all
  three already restricted to the main thread by the engine's own contract, not independently
  confirmed reachable off the main thread, left alone per the same discipline as 014/019/020.
- The broader sweep in `IDEAS.md` still has unconfirmed siblings: `NavigationObstacle3D`,
  `NavigationAgent3D`, plus non-navigation classes (`CollisionObject3D`, `VisualInstance3D`,
  `Path3D`, `RayCast3D`, `ShapeCast3D`, `SpringArm3D`, `SoftBody3D`). This candidate is the third
  independent confirmation of the exact same mechanism (4/4 for the pattern so far: 014, 019, 020,
  021) - very high confidence the remaining list shares it, but each still needs its own repro
  before being written up. Given the hit rate, a future session might reasonably treat 2-3 more
  confirmations as sufficient and then batch the obviously-identical remaining ones with lighter
  per-class verification (still a real run, just less exploratory) - a judgment call for whoever
  picks this up next, not decided here.
