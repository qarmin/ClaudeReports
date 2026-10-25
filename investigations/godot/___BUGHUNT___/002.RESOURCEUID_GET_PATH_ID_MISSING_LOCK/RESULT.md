# ResourceUID::get_path_id() missing lock - heap-use-after-free race

User-pointed target from a code-review conversation (not from `IDEAS.md`'s backlog): every
mutator of `ResourceUID`'s `reverse_cache` map (`add_id`, `set_id`, `remove_id`) takes
`MutexLock l(mutex);` before touching it, but `get_path_id()` reads the same map with no lock at
all. `add_id`/`set_id`/`remove_id` are bound to scripting (`ClassDB::bind_method`), and
`get_path_id()` is reachable from a background thread via `ResourceLoader::get_resource_uid()`
during threaded resource loading - so this looked like a real, reachable heap-use-after-free, not
just a style nit.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading + call-graph trace (`grep`/`git log -S`/`git blame`) | n/a | Confirmed `get_path_id()` (`core/io/resource_uid.cpp:222-228`) has no `MutexLock`, confirmed every other `reverse_cache` accessor does, confirmed `add_id`/`set_id`/`remove_id` are `ClassDB`-bound (`resource_uid.cpp:450-453`), confirmed `ResourceLoader::get_resource_uid()` (`resource_loader.cpp:1412-1415`) calls `get_path_id()` only when `!Engine::is_editor_hint()` |
| Standalone C++ repro, ASan, against the **live checkout's** headers (`-I` into the main tree directly, not a fresh clone) | g++ 15, `-fsanitize=address -g -O1 -DASAN_ENABLED -DTHREADS_ENABLED`, 1 writer thread (`add_id`/`remove_id`, locked) + 1 reader thread (`get_path_id`, unlocked), ~10-15s per run | **Crashed 3/3 runs** with ASan heap-use-after-free/heap-buffer-overflow. First attempt (2 writer + 2 reader threads, before adding `-DTHREADS_ENABLED`) crashed too, but on the wrong race (writer-vs-writer, because `Mutex`/`MutexLock` silently compile to no-ops without `THREADS_ENABLED` - see `core/os/mutex.h:41/100` - a real pitfall worth recording, see `projects/godot.md`) |
| Fix verification: `repro_fixed.cpp` (same mirror class, `MutexLock` added to `get_path_id`) against the same live-checkout headers | Same build flags, 10s per run | **Clean 5/5 runs**, "No crash within the time limit" every time |
| Re-verification against a **pristine fresh clone** (`git clone` at `base_commit`, no leftover live-tree build artifacts) via this candidate's own `justfile` | Same flags, `just test` (clone -> reproduce-before -> apply -> reproduce-after) | **Crashed on reproduce-before** (heap-use-after-free, `clone/core/templates/span.h:43` via `CharStringT::operator==` -> `memcmp`), **clean on reproduce-after** (ran the full 20s, "No crash within the time limit") - this run is what's saved as `output.txt`. This step mattered: the live-checkout run had silently depended on a stale, gitignored `core/profiling/profiling.gen.h` left over from an unrelated earlier session's SCons build; a pristine clone doesn't have it, so the justfile's `clone` recipe creates an empty stand-in (correct for a profiler-disabled build - see the justfile's own comment) |
| **Real engine, GDScript-only reproduction** (`gdscript_repro/`) against a real, self-built editor binary (`scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, ~17m23s clean build, commit `e7cfa294a0b81bed7986be04a848cc1832a3f083`) | `--headless --path gdscript_repro`, ~20-30s per run, real `Thread`s calling the actual public `ResourceUID.add_id`/`.remove_id` and `ResourceLoader.get_resource_uid` scripting API (no C++ at all) | **Crashed 3/3 runs**, ASan heap-use-after-free with frame #0 literally `ResourceUID::get_path_id(String const&) const core/io/resource_uid.cpp:225` and the freeing frame literally `ResourceUID::remove_id(long) core/io/resource_uid.cpp:234` - see `gdscript_repro/gdscript_output_crash.txt`. After applying `patch.diff` and an incremental rebuild (~35-38s, only `core/io/resource_uid.cpp` recompiled): **3/3 clean runs**, see `gdscript_repro/gdscript_output_fixed.txt` |

## Status

**Confirmed.**

Reproduces **3/3** unfixed runs (live-checkout harness) **+ 1/1** unfixed run (pristine-clone
harness via the shipped `justfile`) **+ 3/3** unfixed runs (real engine binary, GDScript-only,
`gdscript_repro/`) = **7/7** total runs crashing under ASan with the lock absent; **5/5 + 1/1 +
3/3 = 9/9** clean runs with the lock added, across all three harnesses. All runs: debug info +
`-O1`/normal optimization, ASan (`-fsanitize=address`), `THREADS_ENABLED` (real
`std::mutex`-backed `Mutex`, not the no-op no-threads stub), no fixed seed/thread-pinning -
ordinary default scheduling on a 24-core machine.

**Fully reproducible**: **Yes**, on two independent levels. The C++ harness (`repro/`) links and
runs Godot's real, unmodified `core/templates/hash_map.h`, `core/os/mutex.h`, and
`core/os/memory.cpp`, plus a class (`MiniResourceUID`) that is a byte-for-byte copy of the real
`ResourceUID::add_id`/`remove_id`/`get_path_id` method bodies - but does not link the actual
`ResourceUID`/`ResourceLoader` classes themselves. That gap is fully closed by `gdscript_repro/`:
a real Godot project, run against a real, self-built editor binary, driving the actual
`ResourceUID`/`ResourceLoader` singletons through their actual public GDScript API from two real
engine `Thread`s - the ASan report's own stack trace names the real `ResourceUID::get_path_id`/
`remove_id` methods and the real GDScript call machinery (`GDScriptFunction::call` ->
`GDScriptInstance::callp` -> `CoreBind::Thread::_start_func`), not a mirror or a stand-in.

## Class

`memory-safety` (heap-use-after-free / heap-buffer-overflow, depending on exact timing - both are
the same underlying race, see "Root cause")

## Root cause

`ResourceUID::get_path_id()` (`core/io/resource_uid.cpp:222-228`) reads
`reverse_cache.getptr(p_path.utf8())` without acquiring `mutex` (`core/io/resource_uid.h:49`).
Every mutator of the same `HashMap<CharString, ID> reverse_cache` (`add_id:156`, `set_id:170`,
`remove_id:230`, `load_from_cache:322`, `clear:419`) takes `MutexLock l(mutex);` first.

`HashMap::_resize_and_rehash()` (`core/templates/hash_map.h:181-213`) grows the table by
allocating new `_elements`/`_hashes` buffers and freeing the old ones (`Memory::free_static`,
`core/os/memory.cpp:204`) once occupancy crosses `MAX_OCCUPANCY`. A `CharString` key stored in a
freed-and-not-yet-reused bucket, or the `CowData` backing a `CharString` value that a concurrent
`remove_id()`/overwrite is destroying, can be read by `get_path_id()`'s unlocked `getptr()` at the
exact moment the owning thread frees it - the ASan traces show the free happening in the
writer thread (`Memory::free_static`) and the read happening in the reader thread
(`CowData<char>::size()` / `CharStringT::operator==` -> `memcmp`), on the same freed allocation.

Reachability in the real engine (not just the repro): `add_id`/`set_id`/`remove_id` are bound to
GDScript (`ClassDB::bind_method`, `resource_uid.cpp:450-453`) - any script, in an exported/running
game, not just the editor, can call them on the main thread. `get_path_id()` is reached from
`ResourceLoader::get_resource_uid()` (`resource_loader.cpp:1412-1415`), which calls it specifically
when `!Engine::get_singleton()->is_editor_hint()` - i.e. in the running game, not the editor - and
Godot's threaded resource loading (`ResourceLoader.load_threaded_request()`) can call
`get_resource_uid()` from a worker thread. A script calling `ResourceUID.add_id()`/`remove_id()`
on the main thread while a threaded load is in flight races exactly the pattern reproduced here.

## Introduced by

60591dc7e8 "Add reverse UID cache" (2025-09-12, Tomasz Chabora). This commit added
`reverse_cache`, `get_path_id()`, and the locked updates to `reverse_cache` in `add_id`/`set_id`/
`remove_id`/`load_from_cache`/`clear` all in the same diff - `get_path_id()` was added without a
lock from the very start, alongside (not after) the correctly-locked mutators.

## The fix

Add `MutexLock l(mutex);` as the first line of `get_path_id()`, matching every sibling accessor of
`reverse_cache`/`unique_ids` in this class (see `patch.diff`, `+1/-0` against
`core/io/resource_uid.cpp`). No other change needed - `mutex` is a plain (non-recursive-contending)
member already used this way throughout the file, and `MutexLock`'s constructor works from a
`const` method the same way `get_id_path()`'s existing `MutexLock l(mutex);` does two methods
above it.

Verified against `repro_fixed.cpp` (same `MiniResourceUID` mirror, lock added to `get_path_id`)
and, more importantly, against the real engine: 9/9 total clean runs across all three harnesses
(5 live-checkout C++ + 1 pristine-clone C++ + 3 real-engine GDScript), vs. 7/7 crashing runs
without it (see "Status" above).

- **Applied for real** to `core/io/resource_uid.cpp`, rebuilt (incremental, ~35-38s - only that
  one file recompiled), `gdscript_repro/` re-run 3/3 clean (see above), then reverted
  (`git checkout -- core/io/resource_uid.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan+
  debug_symbols binary used for the GDScript repro): **identical before and after**: `1431 test
  cases | 1430 passed | 1 failed | 3 skipped`, `425313 assertions | 424313 passed | 1000 failed`.
  The one failing case (`tests/core/io/test_ip.cpp:44`, `CHECK("0:0:0:0:0:0:0:1" ==
  String(IPV6))`) is a pre-existing environment limitation (this sandbox has no IPv6 loopback
  resolution for "localhost") unrelated to this change - same failure present in both the
  before and after runs, in a test file this patch never touches. No new failures introduced.

## What's not done / open questions

- **The standalone C++ repro (`repro/`) does not link against the real `ResourceUID`/
  `ResourceLoader` classes** - doing so would require pulling in `Object`/`ClassDB`/the whole
  engine binding machinery (see `projects/godot.md`), judged impractical for a repro whose only
  job is to show the container+lock interaction. This gap is closed by `gdscript_repro/`, which
  **was** actually run this session against a real self-built ASan editor binary and hit the
  identical bug through the real `ResourceUID`/`ResourceLoader` singletons (see its own
  `README.md` and the captured `gdscript_output_crash.txt`/`gdscript_output_fixed.txt`).
- **Other `reverse_cache`/`unique_ids` access patterns were not re-audited for the same class of
  bug beyond what the original code-review pass found.** `ResourceUID::update_cache()`
  (`resource_uid.cpp:354-361`) reads `changed`/`cache_entries` *before* acquiring `mutex` (a
  check-then-lock race) - torn-read risk on plain scalars, not a use-after-free, and not
  investigated as its own candidate this session. Noted in `IDEAS.md` for a future session.
- **The same "lock only guards the write/init path, not every read" pattern exists in
  `scene/resources/material.cpp`** (`ShaderMaterial`/`BaseMaterial3D::_check_material_rid()` takes
  `material_rid_mutex`, but `_get_material()`/`_set_material()` - the accessors used everywhere
  else in that file - never do). Not investigated as its own candidate this session; noted in
  `IDEAS.md`.
