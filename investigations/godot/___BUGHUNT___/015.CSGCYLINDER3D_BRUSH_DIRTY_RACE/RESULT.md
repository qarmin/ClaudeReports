# `CSGShape3D`'s `dirty`/`brush`/`node_aabb` have no locking anywhere in the class

Started from an external sweep's hypothesis (`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md`,
2026-10-05): "`CSGShape3D`'s plain `bool dirty` + `_make_dirty()`, rebuilt lazily by the main
thread's `_process()`, no `Mutex` anywhere in the class." Unlike candidate 014's investigation of a
similar-sounding hypothesis for `NavigationRegion2D` (which turned out to be a different,
narrower bug), this one is confirmed largely as originally hypothesized: `CSGShape3D::_get_brush()`
(`modules/csg/csg_shape.cpp`) does a check-then-act on `dirty`, then unconditionally
`memdelete(brush)`/rebuilds/reassigns `brush`/`node_aabb`/`dirty` with **zero** synchronization.
Reached here via `CSGCylinder3D` (a `CSGPrimitive3D`/`CSGShape3D` subclass named in the external
sweep's own finding), through the public, GDScript-bound `bake_collision_shape()` entry point.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `csg_shape.h`/`.cpp` lock-usage audit (`grep -n "Mutex"`) | n/a | Zero matches - `dirty`/`brush`/`node_aabb` have no synchronization primitive of any kind |
| GDScript-level repro (`gdscript_repro/`, modeled on the external sweep's random-method-hammering methodology: instantiate `CSGCylinder3D`, `add_child`, spawn 6 worker `Thread`s mixing `radius`/`height` writes with `bake_collision_shape()`/`get_meshes()` calls), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes` (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb), `--headless --path gdscript_repro` under `timeout -k 5 25` | **Crashed on the first run and all 3 runs tried** - a real, fully-symbolized ASan heap-use-after-free: `CowData<CSGBrush::Face>::size()` read from `CSGShape3D::_get_brush_collision_faces()` (`csg_shape.cpp:895`) on memory `memdelete`d by a concurrent `CSGShape3D::_get_brush()` (`csg_shape.cpp:491`) on a different thread |
| First fix attempt (`brush_mutex` added only inside `_get_brush()` itself) | Same build/run, 8 runs | **Still crashed 8/8** - identical signature. The lock correctly protected `_get_brush()`'s own rebuild, but the external caller (`_get_brush_collision_faces()`) kept reading from the *returned raw pointer* after `_get_brush()` had already returned (and released the lock) - a second thread's concurrent rebuild could still free it out from under the first caller's still-in-progress read |
| Complete fix (`brush_mutex` additionally held for the entire body of `_get_brush_collision_faces()`, not just its inner `_get_brush()` call), same build/run, 8 runs | Same build/run | **Clean 8/8 runs** |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the complete fix applied - identical to the established baseline from candidates 002-014 (same pre-existing `test_ip.cpp` IPv6 environment failure), including the module's own `tests/test_csg.h` suite passing |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried) and the incomplete first fix's
own 8/8 re-crash, then 8/8 clean runs with the complete fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `CSGCylinder3D` class through nothing but the
public GDScript API (`CSGCylinder3D.radius`/`.height` property writes, `.bake_collision_shape()`,
`.get_meshes()`). No second participant is involved (not a peer/wire-input finding) - this is a
same-process data race between GDScript `Thread`s.

## Class

`memory-safety` (concurrent unsynchronized free/rebuild of a `CSGBrush*` and its backing
`Vector<CSGBrush::Face>` `CowData` - same general mechanism as this session's other `Vector`/
`HashMap` races, but compounded by a raw-pointer API that outlives the lock protecting its
creation, which is what made the first fix attempt insufficient)

## Root cause

`CSGShape3D::_get_brush()` (`csg_shape.cpp:498`) does `if (!dirty) { return brush; }` then,
when dirty, `memdelete(brush); brush = nullptr;` followed by a full rebuild (calling
`_build_brush()`, `_pack_manifold()`, manifold boolean CSG operations, and recursively
`child->_get_brush()` for any CSG children) before reassigning `brush`/`node_aabb` and clearing
`dirty` - all with zero locking. `CSGShape3D::_make_dirty()` sets `dirty = true` (also unlocked)
whenever a shape property changes, and queues a `call_deferred()`'d `update_shape()` - but
`_get_brush()` itself is *also* directly reachable off the main thread via
`bake_collision_shape()` -> `_get_brush_collision_faces()` -> `_get_brush()`, a fully public,
GDScript-bound call chain. Two threads calling into this chain concurrently (or one thread calling
it while the main thread's own deferred `update_shape()` is mid-rebuild) race `memdelete(brush)`
against another thread still reading `brush->faces`, producing the heap-use-after-free seen here.

The **first fix attempt's own gap** is itself worth recording as part of the root-cause picture:
locking `_get_brush()`'s own body is necessary but not sufficient, because `_get_brush()` *returns
a raw pointer* (`CSGBrush *`) whose valid lifetime is tied to `dirty` not flipping true again - a
lock scoped only to `_get_brush()`'s own function body protects the *creation* of that pointer, not
a caller's subsequent *use* of it after the function has already returned and released the lock.
Any caller that holds onto the returned pointer across the `_get_brush()` call boundary needs to
keep holding the *same* lock for as long as it keeps using that pointer - exactly the gap between
`_get_brush_collision_faces()` getting `n = _get_brush()` and finishing its loop over `n->faces`.

Reachability: `CSGCylinder3D`/`CSGShape3D` is an ordinary, GDScript-instantiable node -
`bake_collision_shape()` is bound and callable from any thread, with no special setup (camera
hardware, editor context, etc.) needed to reach this race.

## Introduced by

8d199a9b2c "CSG Support for Godot!" (2018-04-27) - the commit that introduced CSG support
(`dirty`/`_get_brush()`/`_make_dirty()`) to the engine. No locking was ever added to this
mechanism at any point since (`git log -S"void CSGShape3D::_make_dirty"` shows only a later
rename commit touching this function's signature, no synchronization-adding patch) - a gap present
since the feature's original introduction, not a regression.

## The fix

Add a per-instance `mutable Mutex brush_mutex` (`csg_shape.h`) and hold it:

1. For the entire body of `_get_brush()` (`csg_shape.cpp`), including its recursive
   `child->_get_brush()` calls - safe because each child locks its *own*, different mutex
   instance, never the parent's.
2. For `_make_dirty()`'s own `dirty = true` write and read, **released before** propagating to
   `parent_shape->_make_dirty()` - mirroring candidates 012/013's identical lesson on cross-object
   lock ordering: never hold *this* instance's lock while asking a *different* instance (the
   parent) for *its* lock, or a downward `_get_brush()` tree-walk (parent-then-child locking order)
   could deadlock against an upward `_make_dirty()` propagation if it held the reverse order.
3. For the **entire body** of `_get_brush_collision_faces()`, not just its inner call to
   `_get_brush()` - the fix step that was missing from the first attempt (`+8/-3` of this
   candidate's `+29/-10` total came from this one correction after the first attempt still
   crashed 8/8).

```cpp
// csg_shape.h
mutable Mutex brush_mutex;

// csg_shape.cpp
CSGBrush *CSGShape3D::_get_brush() {
	MutexLock lock(brush_mutex);
	if (!dirty) {
		return brush;
	}
	... // full rebuild, unchanged
}

Vector<Vector3> CSGShape3D::_get_brush_collision_faces() {
	MutexLock lock(brush_mutex);  // covers the whole function, not just _get_brush()
	Vector<Vector3> collision_faces;
	CSGBrush *n = _get_brush();   // recursive re-lock of the same (recursive) mutex - safe
	...
}
```

`Mutex` is recursive (`core/os/mutex.h`, `MutexImpl<std::recursive_mutex>`), which is load-bearing:
`_get_brush_collision_faces()` takes the lock, then calls `_get_brush()` which takes the *same*
lock again on the same thread - a non-recursive `BinaryMutex` would self-deadlock here.

Verified: 8/8 clean runs of `gdscript_repro/` against the complete fix (vs. crashing on 3/3 unfixed
runs, and the incomplete first attempt's own re-crash on all 8/8 of its own verification runs).

- **Applied for real** to `modules/csg/csg_shape.h`/`.cpp`, rebuilt the ASan editor binary twice
  (first attempt ~72s, corrected attempt ~43s - both single/few-TU incremental builds), ran
  `gdscript_repro/` and the test suite against the complete fix, then reverted (`git checkout --
  modules/csg/csg_shape.h modules/csg/csg_shape.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan +
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed | 3
  skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 failure,
  unrelated to this change), including the module's own CSG test suite. No new failures.

## What's not done / open questions

- **A second, related raw-pointer-lifetime gap was found but not independently reproduced or
  fixed**: inside `_get_brush()`'s own body, the recursive-child loop does
  `CSGBrush *child_brush = child->_get_brush();` then immediately uses `child_brush` (`copy_from`,
  `_pack_manifold`) *after* the child's own `_get_brush()` call has already returned and released
  the *child's* lock - the exact same "caller outlives the lock" shape that bit the first fix
  attempt, just one level removed (parent reading a child's brush instead of an external caller
  reading `this`'s own brush). **Not independently confirmed**: this candidate's repro uses a
  single, childless `CSGCylinder3D` (`get_child_count() == 0` in the harness, since `add_child()`
  attaches it to the scene root, not as a CSG child of itself), so the recursive-child code path
  inside `_get_brush()` never actually executes in this session's testing. A future candidate
  should build a repro with an actual parent/child CSG tree (e.g. `CSGCombiner3D` with a
  `CSGCylinder3D` child) and worker threads racing property writes on the *child* against
  `_get_brush()` calls on the *parent* to confirm whether this second gap is independently
  triggerable, and if so, extend the parent's hold of its own lock to also cover its use of each
  `child_brush` pointer (likely requiring the parent to hold the child's lock too for that specific
  window, which needs care to avoid reintroducing a lock-ordering issue).
- `get_brush_faces()` (`csg_shape.h:134`, the public virtual used by editor gizmo code,
  `modules/csg/editor/csg_gizmos.cpp:386`) has the identical unprotected-pointer-after-`_get_brush()`
  shape as the now-fixed `_get_brush_collision_faces()`, but is not reachable from GDScript
  (no `ClassDB::bind_method`) - only called from main-thread-only editor tooling, so not
  independently confirmed as reachable off the main thread, and left unfixed.
- `get_aabb()` (returns `node_aabb` unlocked) and `get_configuration_warnings()` (walks
  `current_shape->brush` unlocked up the parent chain) were not given the same lock - both are
  lower-severity (a torn read of a POD `AABB`, or a read racing a concurrent `memdelete` on a
  `const` method not reachable the same way `bake_collision_shape()` is) and weren't independently
  confirmed as crashing this session; noted rather than guessed at.
