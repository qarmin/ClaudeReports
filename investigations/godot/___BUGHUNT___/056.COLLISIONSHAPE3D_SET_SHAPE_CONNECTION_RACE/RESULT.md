# `CollisionShape3D::set_shape()` - concurrent calls race each other's signal connect/disconnect on the shared `Shape3D`

From the coordinator's priority list. `CollisionShape3D` has **zero** existing thread guards
(`grep -c THREAD_GUARD scene/3d/physics/collision_shape_3d.{h,cpp}` = 0). Unlike most of this
session's "plain field read/write" candidates, the crash here isn't a torn read of the `shape`
field itself (`Ref<Shape3D>`'s copy-assignment is reasonably well-behaved against a single
concurrent reader) - it's **two writer threads calling `set_shape()` concurrently**, each
connecting/disconnecting the same `callable_mp` signal handlers on the same shared `Shape3D`
resource instances, racing each other's mutation of that resource's own signal-connection list.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/3d/physics/collision_shape_3d.cpp` - `set_shape()` calls `shape->disconnect_changed(...)` on the *old* shape and `shape->connect_changed(...)` on the *new* shape, with zero guard; `get_shape()` is a bare `return shape;` | Static read | Identified the exact shape: no field is `CowData`-backed here, but `connect_changed()`/`disconnect_changed()` mutate the target `Shape3D`'s own `Object::connections` list, and this session's repro drives *multiple writer threads* calling `set_shape()` on the same `CollisionShape3D` with the same two shared `Shape3D` resources - exposing both threads to the *same* resources' connection lists concurrently |
| GDScript-level repro (`repro/`: one `CollisionShape3D` added to the tree as a plain child (no `CollisionObject3D` parent needed to exercise this), one shared `BoxShape3D` and one shared `SphereShape3D`, 3 writer threads alternating `cs.shape = box`/`cs.shape = sphere`, 3 reader threads reading `cs.shape` and calling `.get_class()` on the result), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/056`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - 2 clean ASan reports (`heap-use-after-free core/templates/list.h:239 in List<Object::Connection,...>::_Data::erase()`), 1 abort with a partial backtrace showing the same path (`Ref<Shape3D>::unref()` → the shared shape's `Connection` list) |
| Fix: added `_THREAD_SAFE_CLASS_` to `CollisionShape3D`, wrapped `set_shape()`'s *entire* body (not just the field assignment - the whole connect/disconnect/gizmo/collision-object-update sequence) in one `MutexLock(_thread_safe_)` scope, and `get_shape()` in its own - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes, 10/10 clean with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `CollisionShape3D`'s public GDScript API (`.shape` property). The node
didn't even need a `CollisionObject3D` parent (`StaticBody3D`/`Area3D`/etc.) to exercise this -
`set_shape()`'s signal connect/disconnect logic runs regardless of `collision_object` being null.

## Class

`memory-safety` (a signal-connection-list race on the shared `Shape3D` resource, triggered by
concurrent calls to the *same* `CollisionShape3D` instance's `set_shape()` - structurally closer to
candidate 011's `AudioServer::move_bus()`-style "multiple writers racing each other" shape than the
more common single-writer-vs-reader `CowData` races elsewhere this session).

## Root cause

`CollisionShape3D::set_shape()` (`scene/3d/physics/collision_shape_3d.cpp:195`) has no guard of any
kind. When called concurrently from two threads (both targeting the same `CollisionShape3D`
instance, and in this repro's case, cycling between the same two shared `Shape3D` resources), both
threads can simultaneously:
- read `shape.is_valid()` and call `shape->disconnect_changed(...)` on the *same* old shape, or
- call `shape->connect_changed(...)` on the *same* new shape,

both of which mutate that `Shape3D` object's own `Object::connections` list
(`core/templates/list.h`'s `List<Object::Connection>`). Two threads concurrently inserting into or
erasing from the same unsynchronized `List` is a textbook race - `_Data::erase()` freeing a node
that another thread's concurrent `erase()`/`add()` is still touching, caught by ASan as a
heap-use-after-free.

This is a different mechanism from "plain field torn read" races seen elsewhere this session: the
`shape` field assignment itself (`Ref<Shape3D>::operator=`) isn't the primary danger here (copy
assignment writes the new pointer before releasing the old one, and `Ref`'s own refcounting is
atomic) - the real damage is `connect_changed()`/`disconnect_changed()`'s unsynchronized mutation of
the target object's signal list, reached from two concurrently-executing, fully-unguarded
`set_shape()` calls.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `set_shape()`'s connect/disconnect logic
was introduced.

## The fix

`+3/-0` in `scene/3d/physics/collision_shape_3d.cpp`, `+2/-0` in
`scene/3d/physics/collision_shape_3d.h` (see `patch.diff`):

- Added `_THREAD_SAFE_CLASS_` to `CollisionShape3D` (the header already transitively includes
  `core/os/thread_safe.h` via `scene/3d/node_3d.h` → `scene/main/node.h`, no new `#include` needed).
- `set_shape()`: locked the *entire* function body, not just the field assignment - the
  connect/disconnect calls and the `collision_object`/gizmo updates that follow all need to stay
  serialized relative to a concurrent `set_shape()` call on the same instance, since they all key
  off the same `shape` field's current value.
- `get_shape()`: locked the existing one-line body.

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed). Test suite:
`1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- `set_disabled()`/`is_disabled()`, `set_debug_color()`/`get_debug_color()`, and
  `set_debug_fill_enabled()`/`get_debug_fill_enabled()` were not audited - these touch plain scalar/
  `Color` fields (not `CowData`/connection-list-backed), lower suspicion, but not independently
  confirmed either way.
- `make_convex_from_siblings()` (editor-only, `METHOD_FLAG_EDITOR`) calls `set_shape()` internally,
  so it's transitively covered by this fix without needing its own lock.
- `CodeEdit` (lower priority per the coordinator's list) is the last priority-1 item, if time
  allows; otherwise `CryptoKey` (priority-2) is next.
