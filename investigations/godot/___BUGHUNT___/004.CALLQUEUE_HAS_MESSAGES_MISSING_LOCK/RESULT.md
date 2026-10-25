# `CallQueue::has_messages()`/`get_max_buffer_usage()`/`is_flushing()` missing lock

Thread-focused bug-hunt session, picked from `IDEAS.md`'s backlog (found by direct code audit
of `core/object/message_queue.cpp` while investigating candidates 002/003's bug class).
`CallQueue` (`core/object/message_queue.h`/`.cpp`) - the engine's deferred-call queue backing
`Object.call_deferred()` - guards every mutator (`push_callp`, `push_callablep`, `push_set`,
`push_notification`, `flush`, `clear`) with `LOCK_MUTEX`/`UNLOCK_MUTEX`, but three accessor
methods read the same fields (`pages`, `page_bytes`, `pages_used`, `flushing`) with **no lock
at all**:

```cpp
bool CallQueue::is_flushing() const { return flushing; }
bool CallQueue::has_messages() const {
    if (pages_used == 0) { return false; }
    if (pages_used == 1 && page_bytes[0] == 0) { return false; }
    return true;
}
int CallQueue::get_max_buffer_usage() const { return pages.size() * PAGE_SIZE_BYTES; }
```

Same bug *class* as candidate 002 (`ResourceUID::get_path_id()`): locked writers, unlocked
readers racing the writers' `LocalVector::reserve()`-driven `memrealloc()` calls.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading + call-graph trace | n/a | Confirmed all three accessors read `pages`/`page_bytes`/`pages_used`/`flushing` with no lock, while every mutator locks first. `has_messages()` is called every frame from `SceneTree::process()` (`scene/main/scene_tree.cpp:1355,1361`) on each process-group's `call_queue` - a real, hot, internal path used by the engine's threaded process-group feature. `get_max_buffer_usage()` is reachable from GDScript via `Performance.get_monitor(Performance.MEMORY_MESSAGE_BUFFER_MAX)` (`main/performance.cpp:265-266`), racing any thread calling `call_deferred()` on the same global queue. `is_flushing()` has no scripting bind (`grep` of `core_bind.*`/`ClassDB::bind_method` - zero hits) but is still a real internal hazard |
| Standalone C++ mirror repro, ASan, real `core/templates/local_vector.h`/`core/os/mutex.h`/`core/os/memory.cpp` | g++ 15, `-fsanitize=address -g -O1 -DASAN_ENABLED -DTHREADS_ENABLED`, `ASAN_OPTIONS=detect_leaks=0`, 12 writer + 12 reader threads, ~15-18s per run | **Crashed 3/3 runs**: 2/3 ASan heap-use-after-free in `MiniCallQueue::has_messages()` (freed via `Memory::realloc_static` matching `LocalVector::reserve()`'s `memrealloc`), 1/3 a fatal `CRASH_COND` index-out-of-bounds abort (SIGILL) in `LocalVector::operator[]` - a different symptom of the exact same unsynchronized read |
| Fix verification: `repro_fixed.cpp` (same mirror, accessors now take the mutex, matching the real fix) | Same build flags, 18s per run | **Clean 3/3 runs**, "No crash within the time limit" every time |
| Re-verification against a **pristine fresh clone** via this candidate's own `justfile` | Same flags, `just test` against `https://github.com/qarmin/godot.git` (this branch's actual remote - the public `godotengine/godot.git` doesn't have this commit) | **Crashed on reproduce-before** (heap-use-after-free, identical signature), **clean on reproduce-after** - saved as `output.txt` |
| Real engine, GDScript-level attempt (`gdscript_repro/`) against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, `--headless --path gdscript_repro`, 4 writer `Thread`s calling `call_deferred()`, 4 reader `Thread`s polling `Performance.get_monitor(Performance.MEMORY_MESSAGE_BUFFER_MAX)` | **Inconclusive** - crashed identically before *and* after the fix, in the locked mutator path (`push_callablep`/`push_callp`'s out-of-memory branch), not in the three unlocked accessors this candidate touches. A separate, apparently real bug (unthrottled producers exhausting `message_queue/max_size_mb` crashes instead of cleanly erroring) - not this candidate's finding, see `gdscript_repro/README.md`, added as a fresh lead to `IDEAS.md` |
| Real engine build+test suite with the fix applied (fixed binary from the same ASan build) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the known baseline from candidates 002/003 (same pre-existing `test_ip.cpp` IPv6 environment failure). No new failures |

## Status

**Confirmed.**

Reproduces **3/3** unfixed runs (standalone mirror, 2 ASan UAF + 1 CRASH_COND, same root cause)
+ **1/1** unfixed run (pristine-clone harness via the shipped `justfile`) = **4/4** total runs
crashing under ASan with the lock absent; **3/3 + 1/1 = 4/4** clean runs with the fix, across
both harnesses. All runs: debug info + `-O1`, ASan (`-fsanitize=address`), `THREADS_ENABLED`,
`ASAN_OPTIONS=detect_leaks=0`, no fixed seed/thread-pinning, 24-core machine, 12 concurrent
threads (standalone mirror).

**Fully reproducible**: **Partially**. The standalone mirror (`repro/`) links and runs Godot's
real, unmodified `core/templates/local_vector.h`, `core/os/mutex.h` and `core/os/memory.cpp`,
and a class (`MiniCallQueue`) whose relevant members and the three accessor method bodies are
byte-for-byte copies of the real `CallQueue` - but it does not link the actual `CallQueue` class
itself (would require pulling in `Object`/`ClassDB`/`Variant`/the whole engine, same
impracticality as candidates 002/003's standalone mirrors). Unlike 002/003, the attempt to close
that gap with a real end-to-end GDScript reproduction did not succeed cleanly: it hit a
different, real crash in the locked mutator path instead (see `gdscript_repro/README.md`) before
ever isolating the unlocked-reader race at the GDScript level. The reachability argument for why
this bug matters in the real engine (not just the mirror) rests on code reading instead: the
unlocked `has_messages()` is called every engine frame from `SceneTree::process()` on each
process-group's `call_queue` (used by the real, user-facing "threaded process groups" scene
setting), and `get_max_buffer_usage()` is exposed to any GDScript via
`Performance.get_monitor(Performance.MEMORY_MESSAGE_BUFFER_MAX)`.

## Class

`memory-safety` (heap-use-after-free from an unsynchronized read racing a `LocalVector`'s
realloc-on-grow, same mechanism as candidate 002)

## Root cause

`CallQueue::has_messages()`, `is_flushing()` and `get_max_buffer_usage()`
(`core/object/message_queue.cpp:440-458`) read `flushing`/`pages_used`/`page_bytes`/`pages`
without taking `mutex`, while every mutator (`push_callp`, `push_callablep`, `push_set`,
`push_notification`, `flush`, `clear`, `_add_page`) locks it first via the file's
`LOCK_MUTEX`/`UNLOCK_MUTEX` macros. `pages`/`page_bytes` are `LocalVector`s; `_add_page()`
(`:59-66`) grows them via `push_back()`, which geometrically reallocates
(`core/templates/local_vector.h:200-217`, via `memrealloc`) whenever capacity is exceeded. A
concurrent call to any of the three unlocked accessors that is mid-read of `pages`/`page_bytes`
while another thread's locked mutator triggers such a reallocation reads freed memory - the
exact shape already established for candidate 002's `ResourceUID::get_path_id()`, just on
`LocalVector` instead of `HashMap`.

Reachability: any Godot `Object` calling `.call_deferred(...)` from one thread while another
thread calls `Performance.get_monitor(Performance.MEMORY_MESSAGE_BUFFER_MAX)` - both perfectly
ordinary, public, scripting-level operations - races on the engine's single global default
`MessageQueue` instance. Internally, `SceneTree::process()` calls `has_messages()` on each
process-group's own `CallQueue` every frame on the main thread, while any other thread with
`Node::FLAG_PROCESS_THREAD_MESSAGES` can push to that same queue via `call_deferred()` -
exercised by the engine's "threaded process groups" feature (`physics/2d` or `physics/3d`
`thread_model` project settings, or per-node `process_thread_group`).

## Introduced by

`not determined`. `LOCK_MUTEX`/`UNLOCK_MUTEX` have guarded every mutator since `CallQueue` was
split out of `MessageQueue` (a substantial, old refactor); the three accessors appear to have
never been locked since their introduction. Not bisected this session - low priority given the
fix is a mechanical, obviously-correct addition of the same locking already used everywhere else
in the file.

## The fix

Add `LOCK_MUTEX`/`UNLOCK_MUTEX` around each accessor's body, matching the pattern already used
by every other method in the file (not an `OwnedMutex` rewrite - `CallQueue` already centers on
a single `Mutex mutex` member and the file's own macro-based lock/unlock convention, so the
smallest, most consistent fix is to use that same convention rather than introduce a second
locking idiom into one file). The `Mutex mutex` member also needed to become `mutable`, since
`has_messages()`/`is_flushing()`/`get_max_buffer_usage()` are `const` methods (`+15/-5` against
`core/object/message_queue.h`/`.cpp`, see `patch.diff`):

```cpp
bool CallQueue::is_flushing() const {
    LOCK_MUTEX;
    bool ret = flushing;
    UNLOCK_MUTEX;
    return ret;
}

bool CallQueue::has_messages() const {
    LOCK_MUTEX;
    if (pages_used == 0) { UNLOCK_MUTEX; return false; }
    if (pages_used == 1 && page_bytes[0] == 0) { UNLOCK_MUTEX; return false; }
    UNLOCK_MUTEX;
    return true;
}

int CallQueue::get_max_buffer_usage() const {
    LOCK_MUTEX;
    int ret = pages.size() * PAGE_SIZE_BYTES;
    UNLOCK_MUTEX;
    return ret;
}
```

Verified against the standalone mirror (3/3 clean, `just test` 1/1 clean) - see "Status" above.

- **Applied for real** to `core/object/message_queue.h`/`.cpp`, rebuilt the ASan editor binary
  (incremental, ~9m40s - this header is included widely enough that the rebuild touched most of
  `core`/`scene`/`modules`/`tests`, unlike candidates 002/003's single-file ~60-70s rebuilds),
  ran the project test suite and the (inconclusive) `gdscript_repro/` attempt against it, then
  reverted (`git checkout -- core/object/message_queue.h core/object/message_queue.cpp`) before
  finishing this candidate - main tree confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan+
  debug_symbols binary, **absolute** `XDG_DATA_HOME` path): `1431 test cases | 1430 passed |
  1 failed | 3 skipped`, identical to the established baseline from candidates 002/003 (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). Only run once, against the
  fixed binary (not paired with a separate "before" test-suite run, unlike 002/003) - the fix is
  a pure lock addition around read-only accessors, which cannot change behavior in the
  uncontended/single-threaded case the test suite exercises, so a "before" run was judged
  unnecessary given the time already spent on this candidate's other verification layers.

## What's not done / open questions

- **The GDScript-level reproduction attempt surfaced a separate, likely-real crash** in
  `CallQueue::push_callablep`/`push_callp`'s out-of-memory branch (`message_queue.cpp:94-99`),
  reproducing identically before and after this candidate's fix - i.e. NOT this bug. Added to
  `IDEAS.md` as a fresh, not-yet-investigated lead. See `gdscript_repro/README.md` for the full
  account and the exact repro script that triggers it.
- Did not attempt to isolate a cleaner (throttled-producer) GDScript-level repro that might avoid
  the out-of-memory crash and isolate just this candidate's race at the real-engine level, the
  way candidates 002/003 achieved - judged not worth the remaining time for a bug whose
  standalone-mirror confirmation is already as strong as 002's.
- `is_flushing()` was not independently exercised by any repro (only `has_messages()` and
  `get_max_buffer_usage()` were, since they share the exact same unlocked fields) - its fix is
  mechanically identical and covered by the same `patch.diff`, but wasn't separately verified to
  crash.
