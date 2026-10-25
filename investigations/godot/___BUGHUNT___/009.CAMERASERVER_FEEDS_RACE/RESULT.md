# `CameraServer::feeds` fully unlocked despite declaring a dead `_THREAD_SAFE_CLASS_` mutex

Found during this session's sweep of `_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` users (the
same methodology that found candidate 008). `servers/camera/camera_server.h` declares
`_THREAD_SAFE_CLASS_` (a `mutable Mutex`), but `grep` of `camera_server.cpp` found **zero**
`_THREAD_SAFE_METHOD_`/`_THREAD_SAFE_LOCK_` usages anywhere in the file - the mutex is declared
and never used at all, a stronger version of candidate 008's "some methods lock, some don't"
asymmetry.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `camera_server.cpp` lock-usage audit | n/a | `feeds` (`Vector<Ref<CameraFeed>>`) is mutated by `add_feed()`/`remove_feed()` and read by `get_feed()`/`get_feed_count()`/`get_feeds()`/`get_feed_index()`/`get_feed_by_id()`/`get_free_id()` - all public or `ClassDB`-bound, all touching `feeds` with **zero** locking |
| GDScript-level repro (`gdscript_repro/`), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, `--headless --path gdscript_repro`, 8 writer `Thread`s (`CameraFeed.new()` → `add_feed()` → `remove_feed()`) + 8 reader `Thread`s (`get_feed_count()`/`feeds()`) | **Crashed on the first run and every run tried** - real ASan `unknown-crash` report with a fully-symbolized stack through `CameraServer::add_feed` → `Vector::push_back` → `CowData::_insert_uninitialized`, plus other runs hitting a fatal `Vector::operator[]` bounds-check trap - both are the same underlying corruption |
| First fix attempt (7 methods matching the public header's method list) | Same build/run | **Still crashed** - now inside `CameraServer::get_free_id()`, called unlocked from `CameraFeed`'s own constructor on every `CameraFeed.new()`, which has no `ClassDB` binding of its own and was missed when auditing `_bind_methods()` |
| Complete fix (8 methods, `get_free_id()` included) | Same build/run, 8 runs | **Clean 8/8 runs** |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the complete fix applied - identical to the established baseline from candidates 002-008 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.**

Unlike candidates 005/008's narrower, timing-dependent races, this one reproduced reliably on
essentially every attempt against the unfixed binary (both the initial 3/3 testing runs and
the incomplete-first-fix's own re-crash), and 8/8 clean runs with the complete fix. The crash
signature is consistent across runs: either an ASan `unknown-crash` (heap corruption detected,
but not cleanly classified as a UAF/overflow - consistent with concurrent `CowData` mutation
rather than a single freed-buffer dereference) inside `Vector<Ref<CameraFeed>>`'s `push_back`/
`CowData::_insert_uninitialized`, or a fatal `CRASH_BAD_UNSIGNED_INDEX`-style trap from
`Vector::operator[]`'s own bounds check - both are real manifestations of the same root cause.

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `CameraServer`/`CameraFeed` classes through
nothing but the public GDScript API (`CameraFeed.new()`, `CameraServer.add_feed()`/
`.remove_feed()`/`.get_feed_count()`/`.feeds()`). No standalone C++ mirror was built for this
candidate (per this project's current repro policy) - this bug has a trivial GDScript entry
point, even though `CameraServer` is nominally a platform-camera-integration class; nothing
about the race requires an actual camera device.

## Class

`memory-safety` (concurrent unsynchronized `Vector`/`CowData` mutation - same mechanism as
candidate 004's `LocalVector` race, on a `Vector<Ref<CameraFeed>>` instead)

## Root cause

`servers/camera/camera_server.h:50` declares `_THREAD_SAFE_CLASS_` (a `mutable Mutex
_thread_safe_`), the same RAII-mutex idiom used correctly throughout `Input`, `SceneTree`, and
most of this session's other `_THREAD_SAFE_CLASS_` users - but `camera_server.cpp` never calls
`_THREAD_SAFE_METHOD_`/`_THREAD_SAFE_LOCK_` anywhere. `feeds` (`Vector<Ref<CameraFeed>>`,
`camera_server.h`) is therefore mutated and read with zero synchronization by:

- **Writers**: `add_feed()` (`push_back`), `remove_feed()` (`remove_at`) - both public,
  `ClassDB`-bound (`camera_server.cpp:52-53`).
- **Readers**: `get_feed()`, `get_feed_count()`, `get_feeds()`, `get_feed_index()`,
  `get_feed_by_id()` - also public/bound - and `get_free_id()`, called internally from
  `CameraFeed::CameraFeed()` (`camera_feed.cpp:156`, to assign each new feed a unique ID) on
  every single `CameraFeed.new()`, regardless of whether that feed is ever added to the server.

`Vector<T>`'s backing `CowData<T>` reallocates on growth (`push_back`) and shifts elements on
removal (`remove_at`), the same `memrealloc`/pointer-invalidation mechanism already established
by candidate 004 on `LocalVector`. With 8 threads calling `add_feed`/`remove_feed` and 8 more
calling `get_feed_count`/`get_feeds` concurrently and completely unsynchronized, every one of
these operations can observe or mutate the vector mid-reallocation, producing the heap corruption
seen in every crashing run.

Reachability: `CameraServer`/`CameraFeed` are ordinary, `GDREGISTER_CLASS`-registered, fully
GDScript-instantiable/-callable classes (`servers/register_server_types.cpp:182,274`) - no actual
camera hardware or platform camera driver is required to trigger this; `CameraFeed.new()` and
`CameraServer.add_feed()`/`remove_feed()` work identically on any platform/headless build.

## Introduced by

`not determined`. The declared-but-dead `_THREAD_SAFE_CLASS_` suggests thread-safety was intended
from early on but never actually wired up to any method - not bisected this session.

## The fix

Add `_THREAD_SAFE_METHOD_` as the first statement of all 8 methods that touch `feeds`:
`get_free_id()`, `get_feed_index()`, `get_feed_by_id()`, `add_feed()`, `remove_feed()`,
`get_feed()`, `get_feed_count()`, `get_feeds()` - using the mutex the class already declares but
never used, matching the exact convention candidate 008 applied to `Input` (`+16/-0` against
`servers/camera/camera_server.cpp`, see `patch.diff`):

```cpp
int CameraServer::get_free_id() {
    _THREAD_SAFE_METHOD_

    bool id_exists = true;
    ...
}
```
(and identically for the other 7 methods). The class's `_thread_safe_` is a plain `Mutex`
(recursive, per `core/os/mutex.h:97`), so `get_feed_by_id()` calling `get_feed_index()`,
`get_feeds()` calling `get_feed_count()`/`get_feed()`, and `feed_texture()` calling
`get_feed_index()`/`get_feed()` (each independently taking the lock) remains safe.

**Caught an incomplete first attempt during this candidate's own verification**: adding the lock
only to the 7 methods visible in the public header's method list (matching `_bind_methods()`)
left `get_free_id()` - called unlocked from `CameraFeed`'s own constructor, with no `ClassDB`
binding of its own - still racing, and the rebuilt "fixed" binary still crashed, just in a
different place. This is recorded explicitly in `gdscript_repro/README.md` as a reminder that
auditing only script-bound methods can miss internally-called unlocked code paths.

Verified: 8/8 clean runs of `gdscript_repro/` against the complete fix (vs. crashing on
essentially every unfixed run), identical test-suite results (`1431 | 1430 | 1 | 3`, same
baseline).

- **Applied for real** to `servers/camera/camera_server.cpp`, rebuilt the ASan editor binary
  twice (incremental, ~33-41s each - single translation unit, no header touched), ran
  `gdscript_repro/` and the test suite against the complete fix, then reverted
  (`git checkout -- servers/camera/camera_server.cpp`) before finishing this candidate - main
  tree confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan+
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed |
  3 skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6
  failure, unrelated to this change). No new failures.

## What's not done / open questions

- Did not check `CameraFeed` itself (`servers/camera/camera_feed.h`/`.cpp`) for its own internal
  thread-safety gaps beyond the `get_free_id()` call path found here - only `CameraServer`'s
  `feeds` vector was audited.
- Did not check whether other `_THREAD_SAFE_CLASS_`-declaring files in this session's original
  44-file list have a similarly *fully* dead mutex (vs. `Input`'s partial/asymmetric usage in
  candidate 008) - only `camera_server.cpp` was checked this closely; a `grep -L
  "_THREAD_SAFE_METHOD_\|_THREAD_SAFE_LOCK_"` restricted to files that *do* have
  `_THREAD_SAFE_CLASS_` would find any other fully-dead mutexes quickly in a future session.
