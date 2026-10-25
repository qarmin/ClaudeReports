# `Skin::binds_ptr` - a raw cached pointer into `binds`, refreshed only on resize, no guards at all

Fresh discovery (same heuristic as candidates 063-067, this time with a twist: the race isn't on the
container itself but on a raw pointer *cache* into it). `Skin` has **zero** existing thread guards
(`grep -c THREAD_GUARD scene/resources/3d/skin.{h,cpp}` = 0) and stores its bind poses in a
`Vector<Bind> binds`, but every accessor reads through a separately-cached raw pointer,
`Bind *binds_ptr`, refreshed only inside `set_bind_count()`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/3d/skin.h`/`.cpp` - `set_bind_count()` does `binds.resize(p_size); binds_ptr = binds.ptrw(); bind_count = p_size;`. Every other accessor (`set_bind_bone`/`set_bind_pose`/`set_bind_name`/the inline `get_bind_bone`/`get_bind_name`/`get_bind_pose`) indexes through `binds_ptr[p_index]` directly, **never through `binds` itself** - a textbook stale-pointer-cache shape, distinct from (but related to) this session's more common "plain container, no cache" races | Static read | Identified that a concurrent `set_bind_count()` (which can reallocate `binds`' backing buffer and reassign `binds_ptr`) racing any other accessor reading the *old* `binds_ptr` value is doubly dangerous: the pointer itself is read non-atomically, and even if read cleanly, may already point at freed memory |
| GDScript-level repro (`repro/`: one `Skin` with one initial bind, 3 writer threads repeatedly `add_bind()`-ing (which internally calls `set_bind_count()`, reallocating/re-caching `binds_ptr`) and occasionally resetting back down to 1 bind via `set_bind_count(1)`, 3 reader threads reading `get_bind_count()` then indexing every entry via `get_bind_bone()`/`get_bind_pose()`), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly, no rebuild needed for the unfixed run) in an isolated git worktree (`.worktrees/068`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts**, identical signature every time: `ASan double-free core/os/memory.cpp:160 in Memory::realloc_static()` - two threads concurrently calling `binds.resize()` (via `set_bind_count()`) both trying to reallocate/free the same underlying buffer |
| Fix: added `_THREAD_SAFE_CLASS_` to `Skin`, wrapped every method that reads or mutates `binds`/`binds_ptr`/`bind_count` (`set_bind_count`, `add_bind`, `add_named_bind`, `set_bind_name`, `set_bind_bone`, `set_bind_pose`, `clear_binds`, and the inline header accessors `get_bind_count`/`get_bind_bone`/`get_bind_name`/`get_bind_pose`) in `_THREAD_SAFE_METHOD_` - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes (identical signature every time), 10/10 clean with the fix.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `Skin`'s public GDScript API (`add_bind`/`set_bind_count`/
`get_bind_count`/`get_bind_bone`/`get_bind_pose`).

## Class

`memory-safety` (a raw-pointer-cache race: `binds_ptr` is a plain `Bind*` refreshed only inside
`set_bind_count()`'s `binds.resize()`+`binds.ptrw()` sequence, with zero synchronization against any
other method reading through the cached pointer - distinct from this session's more common "plain
unsynchronized container" shape in that the actual field at risk is a *cached pointer*, not the
container itself, though the underlying mechanism - two threads racing a `Vector`'s internal
reallocation - is the same).

## Root cause

`Skin::add_bind()`/`add_named_bind()` both call `set_bind_count(bind_count + 1)`, and multiple
threads calling either concurrently race each other's `binds.resize()` call directly - two
concurrent `Vector::resize()` calls on the same instance can both attempt to free/reallocate the
same backing buffer, which is exactly the ASan `double-free` caught here. Independently,
`set_bind_bone()`/`set_bind_pose()`/`set_bind_name()`/the inline getters all index through
`binds_ptr` without ever re-validating it against a concurrent `set_bind_count()` that may have
already reallocated `binds` and reassigned `binds_ptr` to a new value - a second, distinct
"use a stale cached pointer" race that this specific repro's dominant crash signature didn't
individually isolate, but which the complete fix (locking the whole class) closes along with the
confirmed one.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `binds_ptr` was introduced as a
performance cache over `binds`.

## The fix

`+36/-2` in `scene/resources/3d/skin.cpp`, `+12/-1` in `scene/resources/3d/skin.h` (see
`patch.diff`):

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `Skin`.
- `_THREAD_SAFE_METHOD_` as the first statement in `set_bind_count()`, `add_bind()`,
  `add_named_bind()`, `set_bind_name()`, `set_bind_bone()`, `set_bind_pose()`, `clear_binds()`, and
  the inline header accessors `get_bind_count()`, `get_bind_bone()`, `get_bind_name()`,
  `get_bind_pose()`.
- `add_bind()`/`add_named_bind()` needed the lock around their *entire* body, not just delegating to
  the now-locked `set_bind_count()`/`set_bind_bone()`/etc. individually - otherwise another thread
  could still resize the vector again between this thread's own `set_bind_count()` call and its
  subsequent `set_bind_bone()`/`set_bind_pose()` call, invalidating the `index` it had already
  computed. `Mutex` is recursive, so nesting the outer lock around calls to the now-also-locked
  inner setters is safe.

Verified: 10/10 clean runs of `repro/` with the fix applied (vs. 3/3 crashing unfixed, identical
signature every time). Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied
- identical to the established baseline.

## What's not done / open questions

- `_set()`/`_get()`/`_get_property_list()` (internal serialization helpers) were not independently
  locked - they call the now-locked public accessors (`set_bind_count`/`get_bind_count`/etc.)
  internally, covered transitively via the recursive `Mutex`.
- Found via fresh discovery (SKILL.md's normal process). This is the first candidate this session
  to confirm the "cached raw pointer into a container, refreshed only on resize" shape specifically
  (as opposed to reading the container directly) - worth checking other classes for the same
  `T *foo_ptr` + separate `Vector<T> foo` pairing pattern in future sweeps.
