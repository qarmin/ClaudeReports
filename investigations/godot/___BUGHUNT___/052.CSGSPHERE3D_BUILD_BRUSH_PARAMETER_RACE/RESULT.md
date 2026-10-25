# `CSGSphere3D::_build_brush()` races against its own setters - candidate 015's fix is necessary but not sufficient for CSG primitives

From the coordinator's priority list (`CSGSphere3D` as a sibling of candidate 015's `CSGCylinder3D`).
Started as an expected "fast confirmation" (apply 015's unmodified `patch.diff`, confirm it fixes
the sibling class, done) - but it **didn't fully fix it**, revealing a second, distinct race that
015's narrower repro (a plain `CSGCylinder3D`) never happened to trigger.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| GDScript-level repro (`repro/`, modeled directly on candidate 015's own `gdscript_repro/main.tscn`: one `CSGSphere3D` added to the tree, 6 threads for 15s each randomly choosing between writing `.radius`, writing `.rings`, calling `.bake_collision_shape()`, or calling `.get_meshes()`), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly from the main tree's own `bin/` via worktree hardlinking, no rebuild needed) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 20 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 unfixed attempts** - `heap-buffer-overflow`/`heap-use-after-free`, both inside `CSGSphere3D::_build_brush()` (`csg_shape.cpp:1583`) and `CowData<CSGBrush::Face>::size()` - looked at first glance like the exact same `brush`/`dirty` pointer-lifetime race candidate 015 already fixed |
| **Fast-confirmation attempt**: applied candidate 015's own unmodified `patch.diff` (the `CSGShape3D::_make_dirty()`/`_get_brush()`/`_get_brush_collision_faces()` `brush_mutex` fix) to the worktree, rebuilt, reran | Same build/run, 8 attempts | **Still crashed 8/8** - same function (`CSGSphere3D::_build_brush()`, `csg_shape.cpp:1602`, line shifted by the patch's own added lines), same general shape (`heap-buffer-overflow`, `WRITE of size 12`), meaning 015's fix did *not* resolve this class's crash, just failed to change the underlying problem |
| Investigated why the "obviously-the-same-bug" fast-confirmation didn't work: read `CSGSphere3D::set_radius()`/`set_rings()`/`set_radial_segments()`/`set_smooth_faces()`/`set_material()` - each writes its own scalar field (`radius`, `rings`, `smooth_faces`, etc.) **directly, with no lock**, and only calls the now-locked `_make_dirty()` *afterward*. Meanwhile `_build_brush()` (called from inside the now-locked `_get_brush()`) reads those same fields - `rings`/`radial_segments` in particular determine the *size* of the output vertex/UV arrays it allocates up front, then writes into using per-element indices computed from (potentially re-read) `rings`/`radial_segments` values later in the same function | Static read | Identified the real mechanism: candidate 015's `brush_mutex` only ever serializes *calls into* `_get_brush()`/`_build_brush()` against *each other* - it does nothing to stop a concurrent, totally unguarded **setter** from mutating `rings`/`radial_segments`/`smooth_faces`/`material` out from under an in-progress `_build_brush()` call that's reading those same fields to size and fill its arrays. A classic TOCTOU: the array is allocated using one value of `rings`, then (if `rings` changes mid-build on another thread) written into using a different value, causing an out-of-bounds write |
| Complete fix: moved `brush_mutex` from `private` to `protected` in `CSGShape3D` (so subclasses can use it directly), and wrapped each of `CSGSphere3D`'s shape-parameter field writes (`set_radius()`, `set_radial_segments()`, `set_rings()`, `set_smooth_faces()`, `set_material()`) in their own `MutexLock(brush_mutex)` scope, immediately before calling `_make_dirty()` (safe to do since `Mutex` is recursive and `_get_brush()`/`_build_brush()` already run under the same lock) - rebuilt, reran | Same build/run, 12 attempts | **Clean 12/12** |
| Project test suite with the complete fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline (pre-existing `test_ip.cpp` IPv6 environment failure), no new failures |

## Status

**Confirmed** - and a meaningfully different finding than a simple "fast confirmation" would have
produced. 3/3 unfixed crashes, 8/8 *still* crashing with candidate 015's patch alone, 12/12 clean
with the complete fix (015's fix + this candidate's additional setter-locking).

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `CSGSphere3D`'s public GDScript API.

## Class

`memory-safety` (a TOCTOU race between an unlocked scalar-field setter and a locked-but-insufficient
reader, producing an out-of-bounds heap write when the array-sizing field changes mid-build - a
different mechanism from the `CowData` refcount races that make up most of this session's other
candidates, though the root *cause* - "a public setter mutates state that another method reads
without equivalent synchronization" - is the same family).

## Root cause

Candidate 015 fixed `CSGShape3D::_get_brush()`/`_get_brush_collision_faces()`'s races against *each
other* (concurrent rebuild-vs-read of the `brush`/`dirty`/`node_aabb` trio) by adding `brush_mutex`,
taken inside those two methods. That fix is real and necessary, but it only serializes concurrent
*callers of `_get_brush()`* against each other - it does nothing about a concurrent call to a
**primitive subclass's own property setter**, because those setters (`CSGSphere3D::set_radius()`
and siblings) never took `brush_mutex` themselves; they wrote their field directly, then called
`_make_dirty()` (which itself briefly takes `brush_mutex`, but only to touch the `dirty` flag and
propagate to the parent - by the time `_make_dirty()`'s lock is taken, the field write has *already
happened*, unsynchronized, and may have happened concurrently with an in-progress `_build_brush()`
read of the same field on another thread).

`CSGSphere3D::_build_brush()` is particularly exposed because `rings`/`radial_segments` don't just
affect vertex *values*, they determine array *sizes*: the function computes a face count from
`rings`/`radial_segments` near the top, allocates `Vector<Vector3>`/`Vector<Vector2>` arrays sized
accordingly, then loops writing into them, re-reading the member fields (not a cached local copy)
at each iteration. If another thread's `set_rings()`/`set_radial_segments()` call races in in the
middle of that loop, the loop can write past the end of an array sized for the *old* ring count -
exactly the `heap-buffer-overflow` ASan caught.

This is why candidate 015's `CSGCylinder3D` repro (which only exercised `radius`/`height`/
`bake_collision_shape()`/`get_meshes()`) didn't surface this: a quick read of
`CSGCylinder3D::_build_brush()` shows its own geometry parameters (`sides`) are read in a way that
doesn't produce an out-of-bounds write as readily as `CSGSphere3D`'s two-dimensional (`rings` ×
`radial_segments`) array sizing does - the underlying field-write race almost certainly still exists
there too (unverified, see below), just with a smaller/harder-to-hit blast radius.

## Introduced by

Same era as candidate 015's finding - `brush_mutex` didn't exist before that candidate's fix, so
there's no prior point at which this race could have been "introduced" by this candidate's own
changes; it's a gap in the original fix's *scope*, not a separately-introduced regression.

## The fix

`patch.diff` is a complete, standalone patch (includes candidate 015's own changes plus this
candidate's additions) - applying it alone to unpatched HEAD fixes both the `brush`/`dirty` race
*and* this candidate's setter race. Diffstat: `+30/-12` in `modules/csg/csg_shape.cpp`,
`+18/-1` in `modules/csg/csg_shape.h`.

On top of candidate 015's changes:
- Moved `brush_mutex`'s declaration from `private` to `protected` in `CSGShape3D` (a 3-line change:
  close the existing `private:` block before it, open `protected:`, reopen `private:` after) so
  subclasses can take the lock directly.
- Wrapped the single field-assignment statement in each of `CSGSphere3D::set_radius()`,
  `set_radial_segments()`, `set_rings()`, `set_smooth_faces()`, and `set_material()` in its own
  `{ MutexLock lock(brush_mutex); ...; }` scope, immediately before (not instead of) the existing
  `_make_dirty()` call.

Verified: 12/12 clean runs of `repro/` with the complete fix applied (vs. 3/3 crashing with nothing
applied, and 8/8 *still* crashing with only candidate 015's patch). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- **This same gap likely affects every other `CSGPrimitive3D` subclass** (`CSGCylinder3D`,
  `CSGBox3D`, `CSGTorus3D`, `CSGMesh3D`, `CSGPolygon3D`) - none of their setters were audited or
  fixed in this candidate, only `CSGSphere3D`'s. Candidate 015's own `CSGCylinder3D` repro didn't
  surface it empirically (likely a smaller/harder-to-hit blast radius, not an actual absence of the
  bug - `CSGCylinder3D::set_sides()`/`set_radius()`/`set_height()` have the exact same unlocked
  direct-field-write shape by inspection). A thorough closure of this family would need the same
  `MutexLock(brush_mutex)` treatment applied to every primitive subclass's setters, not just
  `CSGSphere3D`'s - worth a dedicated follow-up candidate or a scripted sweep across
  `modules/csg/csg_shape.cpp`'s primitive subclasses, similar to candidate 030's `PopupMenu` sweep.
- `TileMap` (sibling of `TileMapLayer`/032) is the next item on the coordinator's priority list.
