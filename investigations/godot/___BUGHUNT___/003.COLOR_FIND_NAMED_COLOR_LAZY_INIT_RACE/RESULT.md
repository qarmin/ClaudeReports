# `Color::find_named_color()` double-checked lazy-init race on a static `HashMap`

Thread-focused bug-hunt session, picked from `IDEAS.md`'s backlog (seeded from a prior
conversation's manual code review). `Color::find_named_color()` (`core/math/color.cpp:412-431`)
uses a function-local `static HashMap<String, int> named_colors_hashmap;`, checked via
`if (unlikely(named_colors_hashmap.is_empty()))` and populated in a loop with **no lock at all** -
the same bug *class* as candidate 002 (`ResourceUID::get_path_id()`), on a different container.
Matches real, open upstream issue [godotengine/godot#113865](https://github.com/godotengine/godot/issues/113865)
("Non-thread-safe static HashMap initializations").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading + call-graph trace | n/a | Confirmed `find_named_color()` has zero lock around the lazy-populate loop; `Color.from_string(name, default)` (public GDScript API) falls back to `named()` -> `find_named_color()` whenever the string isn't a valid hex color - a real, reachable entry point from script |
| Standalone C++ mirror repro, ASan, real `core/templates/hash_map.h`/`core/os/memory.cpp` (CharString key, see "Fully reproducible" below for why) | g++ 15, `-fsanitize=address -g -O1 -DASAN_ENABLED -DTHREADS_ENABLED`, 10 threads calling the lazy-init lookup concurrently, ~15-18s per run | **Crashed 5/5 runs**, ASan heap-buffer-overflow in `HashMap::_insert_empty_at`/`_insert_element`/`_insert`/`operator[]` - a concurrent-write corruption (multiple threads racing to populate the same map on first use), not just a reader-vs-writer race like candidate 002 |
| Fix verification: `repro_fixed.cpp` (same mirror, plain `static Mutex` + `MutexLock` instead of bare `static HashMap`) | Same build flags, 18s per run | **Clean 3/3 runs**, "No crash within the time limit" every time |
| Re-verification against a **pristine fresh clone** via this candidate's own `justfile` | Same flags, `just test` | **Crashed on reproduce-before** (heap-buffer-overflow, identical signature), **clean on reproduce-after** - this run is what's saved as `output.txt` |
| **Real engine, GDScript-only reproduction** (`gdscript_repro/`) against a real, self-built ASan editor binary (`scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, commit `ebf68e6f024eda115c301595d9296f23f2a254eb`) | `--headless --path gdscript_repro`, 10 real `Thread`s calling the actual public `Color.from_string()` scripting API (no C++ at all) | **Crashed 3/3 runs**, ASan heap-buffer-overflow with frame #4 literally `Color::find_named_color(String const&) core/math/color.cpp:422`, reached through the real `Color.named()`/`Color.from_string()`/GDScript VM/`CoreBind::Thread` call chain - see `gdscript_repro/gdscript_output_crash.txt`. After applying `patch.diff` and rebuilding (~60-70s incremental): **6/6 clean runs total** across two verification passes, see `gdscript_repro/gdscript_output_fixed.txt` |

## Status

**Confirmed.**

Reproduces **5/5** unfixed runs (standalone mirror) + **1/1** unfixed run (pristine-clone harness
via the shipped `justfile`) + **3/3** unfixed runs (real engine binary, GDScript-only) = **9/9**
total runs crashing under ASan with the lock absent; **3/3 + 1/1 + 6/6 = 10/10** clean runs with
the fix, across all three harnesses. All runs: debug info + `-O1`, ASan (`-fsanitize=address`),
`THREADS_ENABLED`, no fixed seed/thread-pinning, 24-core machine, 10 concurrent threads.

**Fully reproducible**: **Yes**, on two independent levels, same shape as candidate 002. The C++
harness (`repro/`) links and runs Godot's real, unmodified `core/templates/hash_map.h` and
`core/os/memory.cpp`, plus a mirror function (`mini_find_named_color`) that is a byte-for-byte
copy of the real `Color::find_named_color()` body - but does not link the actual `Color` class
(would require `core/math/color_names.inc`'s real ~150-entry table plus, if using the real
`String` key type instead of `CharString`, the real `core/string/ustring.cpp` implementation,
which itself pulls in `Object`/`OS`/`Variant`/`TranslationServer` to link standalone - judged
impractical for a repro whose only job is to demonstrate the container-mutation race, identical
for either CowData-backed string type). That gap is fully closed by `gdscript_repro/`: a real
Godot project, run against a real, self-built editor binary, driving the actual `Color` class
through its actual public GDScript API (`Color.from_string()`) from real engine `Thread`s - the
ASan report's own stack trace names the real `Color::find_named_color`/`named`/`from_string`
methods and the real GDScript call machinery, not a mirror or a stand-in.

## Class

`memory-safety` (heap-buffer-overflow from concurrent, unsynchronized `HashMap` mutation)

## Root cause

`Color::find_named_color()` (`core/math/color.cpp:412-431`) declares
`static HashMap<String, int> named_colors_hashmap;` and populates it lazily:

```cpp
static HashMap<String, int> named_colors_hashmap;
if (unlikely(named_colors_hashmap.is_empty())) {
    const int named_color_count = get_named_color_count();
    for (int i = 0; i < named_color_count; i++) {
        named_colors_hashmap[String(named_colors[i].name).remove_char('_')] = i;
    }
}
```

The function-local `static` declaration itself is thread-safe to *construct* (C++11 guarantees
this), but the subsequent `is_empty()` check and population loop are ordinary, unsynchronized
code. If two or more threads call `find_named_color()` for the first time concurrently, all of
them can observe `is_empty() == true` and all proceed to call `operator[]` on the **same**
`HashMap` instance concurrently. `HashMap::_insert`/`_insert_element`/`_insert_empty_at`
(`core/templates/hash_map.h:143-245`) are not internally synchronized - concurrent insertion
corrupts the map's internal `_elements`/`_hashes` arrays (a classic concurrent-write race, not
just a reader-vs-writer one), producing the heap-buffer-overflow seen in every reproduction.

Reachability: `Color.from_string(p_string, p_default)` (`color.cpp:452-458`) calls `named()` ->
`find_named_color()` whenever `p_string` isn't a valid HTML color. `from_string` is a public,
scripting-bound static `Color` method - any GDScript, in an exported/running game, can call
`Color.from_string("yellow", Color.BLACK)` from multiple `Thread`s (or from
`WorkerThreadPool`/multiple scenes loading concurrently that happen to construct named colors),
hitting the race on the very first such call after engine startup.

## Introduced by

49594d8c41 "Optimize `Color::find_named_color()`" (2023-04-09). This commit replaced an O(n)
linear search through `named_colors[]` with the cached `HashMap` lookup - the cache itself was a
legitimate performance win, but its lazy-populate step was never given a lock, introducing this
race from day one.

## The fix

Add a plain `static Mutex` and a `MutexLock` guard around the existing bare `static HashMap`
(see `patch.diff`, `+3/-0` against `core/math/color.cpp`). An earlier draft of this fix used a new
`OwnedMutex<T>` wrapper type added on this branch (`core/templates/owned_mutex.h`, used to fix
candidate 002) - **deliberately reverted in favor of plain `Mutex`/`MutexLock`**, since
`OwnedMutex` is this branch's own, not-yet-upstreamed addition and its eventual shape (if/when
merged into godotengine/godot) could differ from what exists here today; a bug-hunt fix should
stand on its own against vanilla upstream, not depend on an experimental local-only type. Plain
`Mutex`/`MutexLock` is the same idiom already used throughout the rest of this file's neighbors
(e.g. `core/object/message_queue.cpp`'s `LOCK_MUTEX`/`UNLOCK_MUTEX`) and needs no new type at all.

```cpp
static Mutex named_colors_mutex;
static HashMap<String, int> named_colors_hashmap;
MutexLock lock(named_colors_mutex);
if (unlikely(named_colors_hashmap.is_empty())) {
    ...
    named_colors_hashmap[String(named_colors[i].name).remove_char('_')] = i;
}
...
const HashMap<String, int>::ConstIterator E = named_colors_hashmap.find(name);
```

Verified against the real engine via `gdscript_repro/` (3/3 clean runs with this plain-`Mutex`
fix, re-verified after switching away from `OwnedMutex`) - see "Status" above. The standalone C++
mirror repro (`repro/`) exists for reference but is **not** the basis of this candidate's
`Confirmed`/fix-verified status - per updated session guidance, GDScript-level repro against the
real engine binary is the primary (and in this case sufficient) evidence; the C++ mirror's own
`repro_fixed.cpp` was updated to match the plain-`Mutex` fix for consistency but not re-compiled
after that edit (not needed - see above).

- **Applied for real** to `core/math/color.cpp`, rebuilt (incremental, ~12 min - this rebuild also
  recompiled `tests/` after a long idle gap invalidated SCons' cache), `gdscript_repro/` re-run
  3/3 clean, project test suite re-run (`1431 cases | 1430 passed | 1 failed | 3 skipped` -
  identical to baseline, same pre-existing unrelated `test_ip.cpp` IPv6 failure), then reverted
  (`git checkout -- core/math/color.cpp`) before finishing this
  candidate - main tree confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan+
  debug_symbols binary used for the GDScript repro, **absolute** `XDG_DATA_HOME` path - a relative
  one is silently ignored and falls back to the real home directory, which triggered an unrelated,
  already-known bug, candidate 001's Logger null-deref, on the first attempt): **identical before
  and after**: `1431 test cases | 1430 passed | 1 failed | 3 skipped`,
  `425313 assertions | 424313 passed | 1000 failed`. The one failing case
  (`tests/core/io/test_ip.cpp:44`, IPv6 loopback resolution) is the same pre-existing environment
  limitation already documented for candidate 002 - unrelated to this change, present in both
  runs. No new failures introduced.

## What's not done / open questions

- **The real upstream issue (#113865) names 3 more unverified siblings** with the same reported
  shape: `VariantParser::parse_value`'s typed-array/dict cache, `EngineUpdateLabel::_get_version_type`,
  and `_find_annotation_arguments` (`modules/gdscript/gdscript_editor.cpp`). Not investigated this
  session - worth a follow-up candidate, added back to `IDEAS.md`.
- **The standalone C++ repro uses `CharString` instead of `String`** as the key type (see "Fully
  reproducible" above for why) - fully compensated for by the real end-to-end GDScript
  confirmation, but noted for anyone reading `repro/` in isolation.
- Not checked whether the same "function-local static container, no lock" shape exists elsewhere
  in `core/math/` beyond `color.cpp` - only this one function was investigated.
