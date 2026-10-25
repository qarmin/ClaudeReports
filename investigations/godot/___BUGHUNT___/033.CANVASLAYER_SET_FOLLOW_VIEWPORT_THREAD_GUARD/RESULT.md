# `CanvasLayer::set_follow_viewport()` dereferences `vp->get_world_2d()`'s thread-guard-null result unchecked

Flagged by candidate 026's own follow-up notes as a structurally distinct ("setter-shaped", like
candidate 024's `CollisionObject3D::set_disable_mode()`) unconfirmed sibling of the already-9/9-
confirmed `get_world_2d()->`/`get_world_3d()->` unchecked-dereference family (candidates 014,
019-024, 026, 031, 032). Here the thread-guarded getter is called through a cached `Viewport *vp`
member rather than on `this`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/main/canvas_layer.cpp:276-307` (`CanvasLayer::set_follow_viewport()` / `_update_follow_viewport()`) and `scene/main/viewport.cpp:1401-1404` (`Viewport::get_world_2d()`) | Static read | `set_follow_viewport(bool)` is a bound public setter (`ClassDB::bind_method(D_METHOD("set_follow_viewport", "enable"), ...)`, `canvas_layer.cpp:331`) that, when the value actually changes, calls `_update_follow_viewport()`, whose non-`force_exit` branch does `vp->get_world_2d()->get_canvas()` unconditionally; `Viewport::get_world_2d()` has `ERR_READ_THREAD_GUARD_V(Ref<World2D>())` at its very first line. `vp` itself is only ever assigned during `NOTIFICATION_ENTER_TREE` (main thread), so it's already valid by the time a worker thread could call the setter - the null-deref risk is entirely in the `get_world_2d()` return value, not `vp` itself |
| GDScript-level repro (`repro/`: instantiate `CanvasLayer`, `add_child`, spawn 4 worker `Thread`s each alternately calling `set_follow_viewport(true)`/`set_follow_viewport(false)` for 3s - the value-changed check means only the (contended) transitions where a thread's view of `follow_viewport` actually differs take the live code path, but with 4 threads racing the same unguarded `bool` this happens immediately), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/033`, `bin/`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 25 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Segfaulted (SIGSEGV, exit 139) on all 3 attempts** |
| Re-run under `gdb -batch -ex run -ex "thread apply all bt"` for a symbolized backtrace | Same binary/repro | `World2D::get_canvas (this=0x0)` at `world_2d.cpp:47`, called from `CanvasLayer::_update_follow_viewport` at `canvas_layer.cpp:305`, from a `Node::_spam` worker thread - confirms the exact mechanism |
| Fix (`ERR_READ_THREAD_GUARD` + explicit `Ref<World2D>::is_valid()` check before dereferencing), incremental rebuild (ccache-warm hardlinked worktree, ~59s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (deterministic once the first value-changing call
hits the live branch - every attempt crashed within a few seconds), and 8/8 clean runs with the
fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `CanvasLayer` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from a single
GDScript `Thread` calling one public method (`set_follow_viewport()`) off the main thread.

## Class

`crash` (null-pointer dereference - a single worker thread calling `set_follow_viewport()` with a
value different from the node's current `follow_viewport` state crashes deterministically while
the node is inside the tree, through the already-correctly-thread-guarded `Viewport::get_world_2d()`
whose result is then dereferenced unconditionally).

## Root cause

`CanvasLayer::_update_follow_viewport()` (called from the public setters `set_follow_viewport()`
and `set_follow_viewport_scale()`):

```cpp
void CanvasLayer::_update_follow_viewport(bool p_force_exit) {
	if (!is_inside_tree()) {
		return;
	}
	if (p_force_exit || !follow_viewport) {
		RS::get_singleton()->canvas_set_parent(canvas, RID(), 1.0);
	} else {
		RS::get_singleton()->canvas_set_parent(canvas, vp->get_world_2d()->get_canvas(), follow_viewport_scale); // <-- unconditional dereference
	}
}
```

Same mechanism as candidates 014/019-024/026/031/032, one level removed through a cached pointer:
`Viewport::get_world_2d()` (`scene/main/viewport.cpp:1401-1404`) begins with
`ERR_READ_THREAD_GUARD_V(Ref<World2D>());`, returning a null `Ref<World2D>` when called off the
main thread/a thread group. `_update_follow_viewport()` dereferences that result unconditionally,
crashing inside `World2D::get_canvas()` (`scene/resources/world_2d.cpp:47`) on `this == nullptr` -
confirmed via gdb: `World2D::get_canvas (this=0x0) at scene/resources/world_2d.cpp:47`.

Reachability: `set_follow_viewport(bool)` is a bound, GDScript-callable public setter
(`ClassDB::bind_method(D_METHOD("set_follow_viewport", "enable"), ...)`, `canvas_layer.cpp:331`);
`set_follow_viewport_scale(float)` reaches the identical unconditional-dereference branch
unconditionally (it doesn't even have a value-changed guard) and is a second, independent entry
point into the same bug.

## Introduced by

19a6a6286a73ed2e95a00f92994851c817d21401 "Ability to make CanvasLayers have pseudo 3D depth."
(2019-04-05) - `git log -S"vp->get_world_2d()->get_canvas()"` on this file shows this is the commit
that added the `vp->get_world_2d()->get_canvas()` call in `_update_follow_viewport()`. Never
retrofitted with the `ERR_READ_THREAD_GUARD` idiom `Viewport::get_world_2d()` itself already uses.

## The fix

`+5/-1` against `scene/main/canvas_layer.cpp` (see `patch.diff`):

```cpp
	if (p_force_exit || !follow_viewport) {
		RS::get_singleton()->canvas_set_parent(canvas, RID(), 1.0);
	} else {
		ERR_READ_THREAD_GUARD;
		Ref<World2D> world_2d = vp->get_world_2d();
		if (world_2d.is_valid()) {
			RS::get_singleton()->canvas_set_parent(canvas, world_2d->get_canvas(), follow_viewport_scale);
		}
	}
```

`ERR_READ_THREAD_GUARD` (the void-returning sibling of `ERR_READ_THREAD_GUARD_V`, `scene/main/node.h:939`)
is used here rather than the `_V` form since `_update_follow_viewport()` returns `void`; `CanvasLayer`
inherits `is_readable_from_caller_thread()` from `Node` the same way every other fixed class in this
sweep does. Fixing the shared private helper covers both of its callers
(`set_follow_viewport()`/`set_follow_viewport_scale()`) in one patch.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed).

- **Applied for real** in an isolated worktree (`.worktrees/033`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `set_follow_viewport_scale()`'s own independent reachability into the same bug (noted above) was
  not given its own separate repro - the fix covers it via the shared helper, and the mechanism is
  identical, but it wasn't independently exercised this session.
- The underlying `bool follow_viewport` itself is still a plain unguarded data race (concurrent
  `set_follow_viewport()` calls can race each other's read-modify-write of the field, independent
  of the null-deref fixed here) - out of scope for this candidate, which targets the deterministic
  crash specifically; a future candidate could look at whether `CanvasLayer` needs a real mutex for
  its mutable state more broadly, in the spirit of candidates 008-018/027/029/030's full-class
  treatments.
- This closes out every lead named in candidate 026's own follow-up list (`GridMap`/031,
  `TileMapLayer`/032, `CanvasLayer`/033 all now confirmed-and-fixed; `Window`/`MultiplayerSpawner`
  were already read-and-ruled-out as genuinely clean in candidate 026 itself).
