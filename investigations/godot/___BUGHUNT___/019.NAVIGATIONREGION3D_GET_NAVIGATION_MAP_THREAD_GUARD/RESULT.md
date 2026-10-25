# `NavigationRegion3D::get_navigation_map()` dereferences `get_world_3d()`'s thread-guard-null result unchecked

Follow-up from `IDEAS.md` on candidate 014's confirmed `NavigationRegion2D::get_navigation_map()`
null-deref: `grep -rn "get_world_3d()->" scene/` flagged `NavigationRegion3D::get_navigation_map()`
(`scene/3d/navigation/navigation_region_3d.cpp:217`) as the most direct 3D sibling of the same
shape. Built and ran a real repro rather than assuming the pattern transfers unchanged (per
candidate 014's own lesson) - it does transfer, with an identical mechanism.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| GDScript-level repro (`repro/`, modeled directly on candidate 014's/017's harness: instantiate `NavigationRegion3D`, `add_child`, spawn 6 worker `Thread`s hammering a mix of setters/getters including `get_navigation_map()`), against the real, already-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on all 3/3 runs tried**, each within a few seconds - a plain null-pointer dereference, not an ASan-reported heap/stack issue |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary/repro | `World3D::get_navigation_map (this=0x0)` at `world_3d.cpp:72`, called from `NavigationRegion3D::get_navigation_map` at `navigation_region_3d.cpp:217`, from a `Node::_spam` (`CoreBind::Thread`) worker - confirms the exact mechanism |
| Fix (`ERR_READ_THREAD_GUARD_V(RID())` + explicit `Ref<World3D>::is_valid()` check before dereferencing), same build/run, 8 runs | Rebuilt incrementally (`scons ... use_asan=yes tests=yes debug_symbols=yes cpp_compiler_launcher=ccache c_compiler_launcher=ccache -j12`), then 8x repro run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-014 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried, each crashing within a few
seconds, deterministic not timing-dependent), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `NavigationRegion3D` class through nothing but the public
GDScript API (`NavigationRegion3D.new()`, `add_child()`, `get_navigation_map()` and sibling
setters/getters). No second participant is involved - this is a same-process crash reachable from
a single GDScript `Thread` calling one public method off the main thread.

## Class

`crash` (null-pointer dereference - a *single* worker thread calling `get_navigation_map()`
crashes deterministically every time, with or without any other thread involved; the other 5
threads in the repro are only there to also exercise the sibling setters, not required to trigger
this specific crash).

## Root cause

`NavigationRegion3D::get_navigation_map()`:

```cpp
RID NavigationRegion3D::get_navigation_map() const {
	if (map_override.is_valid()) {
		return map_override;
	} else if (is_inside_tree()) {
		return get_world_3d()->get_navigation_map();          // <-- unconditional dereference
	}
	return RID();
}
```

`Node3D::get_world_3d()` (`scene/3d/node_3d.cpp:1074-1080`) begins with
`ERR_READ_THREAD_GUARD_V(Ref<World3D>());` - when called from any thread other than the main
thread (or a registered "thread group"), this macro logs the error seen in this candidate's repro
output and returns a default-constructed (null) `Ref<World3D>`, exactly as its own comment says
("World3D can only be set from main thread, so it's safe to obtain on threads" - true for the
getter itself, not for an unguarded caller). `get_navigation_map()` calls this guarded getter but
has no guard or null check of its own - `get_world_3d()` fails exactly as designed, and the very
next expression (`->get_navigation_map()`) crashes anyway inside `World3D::get_navigation_map()`
(`scene/resources/3d/world_3d.cpp:72`) dereferencing `this == nullptr`.

This is the exact same shape as candidate 014's confirmed 2D finding, in the 3D sibling class, as
`IDEAS.md` predicted - but independently confirmed with its own repro and backtrace rather than
assumed, per candidate 014's own stated lesson (don't reuse a hypothesis across sibling classes
uncritically).

Reachability: the bound, GDScript-callable public method
(`ClassDB::bind_method(D_METHOD("get_navigation_map"), ...)`, `navigation_region_3d.cpp:273`) is
reachable from any thread with no special setup - not a race in the TOCTOU sense, a deterministic
crash on every off-main-thread call (as long as `map_override` isn't set, the common default
configuration).

## Introduced by

84647ab09eba232ec7466dda9ce5fa269aad8bf3 "Add NavigationRegion function to change navigation map"
(2023-05-17) - this is the commit that added `NavigationRegion3D::get_navigation_map()` with its
current `get_world_3d()->get_navigation_map()` body. `ERR_READ_THREAD_GUARD_V` itself was added to
the engine as part of Godot 4.x's broader thread-safety-guard infrastructure; this call site was
never retrofitted with it, the same situation as candidate 014's 2D counterpart.

## The fix

Add the same `ERR_READ_THREAD_GUARD_V` idiom `Node3D::get_world_3d()` itself already uses, plus an
explicit null check before dereferencing (`+5/-1` against
`scene/3d/navigation/navigation_region_3d.cpp`, see `patch.diff`):

```cpp
RID NavigationRegion3D::get_navigation_map() const {
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

The explicit `ERR_READ_THREAD_GUARD_V(RID())` gives callers one clear, specific error naming
`get_navigation_map()` itself (matching the engine's own idiom), instead of the more confusing
indirect message about `get_world_3d()` that a caller never directly invoked. The
`Ref<World3D>::is_valid()` check is kept as defense in depth for the case `get_world_3d()` returns
null for a different reason (e.g. `data.viewport` being null), not just the thread-guard path.

Verified: 8/8 clean runs of `repro/` against the fix (vs. crashing on all 3/3 unfixed runs tried).

- **Applied for real** to `scene/3d/navigation/navigation_region_3d.cpp`, rebuilt the ASan editor
  binary, ran `repro/` and the full test suite against the fix, then reverted (`git checkout --
  scene/3d/navigation/navigation_region_3d.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan +
  debug_symbols binary): `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before this
  session's work and after the fix - identical to the established baseline (same pre-existing
  `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `NavigationRegion3D` has other internal call sites of `get_world_3d()->...` in the same file
  (`:358` in `set_navigation_map()`'s equality check, `:381` in `_region_enter_navigation_map()`,
  `:648`/`:672` in edge-connection debug/baking helpers, `:609`/`:728` reading `get_scenario()`) -
  not touched. `:381`/`:609`/`:728` are only reachable from `NOTIFICATION_ENTER_TREE`/debug-draw
  paths the engine's own contract already restricts to the main thread; `:358`'s check only runs
  inside `set_navigation_map()`, which per the class's own doc comments is meant to be called from
  the main thread (it mutates `map_override` with no lock); `:648`/`:672` are reached from
  `_update_meshes_in_navigation_map()`/baking flows that are themselves main-thread-only in
  practice. None of these were independently confirmed reachable off the main thread the way the
  public, bound `get_navigation_map()` getter is - left alone rather than guessed at, same
  discipline candidate 014 used for `NavigationRegion2D`'s equivalent internal call sites.
- The broader `get_world_2d()->`/`get_world_3d()->` sweep in `IDEAS.md` still has unconfirmed
  siblings: `NavigationLink2D`, `NavigationLink3D`, `NavigationObstacle3D`, `NavigationAgent3D`,
  plus non-navigation classes (`CollisionObject3D`, `VisualInstance3D`, `Path3D`, `RayCast3D`,
  `ShapeCast3D`, `SpringArm3D`, `SoftBody3D`). This candidate confirms the pattern repeats exactly
  as predicted for one more class, raising confidence in the rest, but each still needs its own
  repro before being written up - not done this session, re-added to `IDEAS.md` with this
  candidate's confirmation as supporting evidence.
