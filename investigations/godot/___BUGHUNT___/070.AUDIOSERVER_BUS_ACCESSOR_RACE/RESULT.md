# `AudioServer`'s bus accessors - lock-free fast path races its own structural mutators and its own per-frame `update()`

Fresh discovery, but driven by a lead the risk map had flagged since an earlier session and left
unconfirmed: "several bus setters (`set_bus_volume_db`, `set_bus_send`, `set_bus_solo`,
`set_bus_mute`, `set_bus_bypass_effects`, `set_bus_effect_enabled`) mutate `buses[p_bus]->*` with no
lock while the audio thread reads the same `Bus` structs under lock elsewhere - flagged as a possible
intentional lock-free fast path rather than confirmed as a bug." This candidate empirically tested
that hypothesis and confirmed it as a real, fixable bug - but the fix took three attempts to actually
close, because the same ordering bug recurs independently in several different places in this file.

`AudioServer` doesn't use the `_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` macro convention used by
every other candidate this session. It has its own hand-rolled `lock()`/`unlock()` pair that delegates
to `AudioDriver::get_singleton()->lock()`/`unlock()` - the same mutex the real-time audio-processing
thread uses. `add_bus`/`remove_bus`/`add_bus_effect`/`remove_bus_effect`/`set_bus_count` already called
it; roughly 20 other bus accessors (`set_bus_volume_db`/`get_bus_volume_db`/`set_bus_mute`/etc.) did
not call it at all.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `servers/audio/audio_server.h`/`.cpp` - confirmed `Vector<Bus *> buses` with zero `Mutex`/`_THREAD_SAFE_CLASS_` in the header, `lock()`/`unlock()` declared non-const and delegating to `AudioDriver` | Static read | Identified ~20 unlocked accessors matching the risk-map lead |
| GDScript-level repro (`repro/`: 3 writer threads doing `AudioServer.add_bus(1)` and conditionally `remove_bus(...)`, 3 reader threads looping `set_bus_volume_db`/`get_bus_volume_db`/`set_bus_mute` over every current bus index, 3 seconds, against the unpatched ASan editor binary) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot...san --headless --path repro` | **3/3 crashed** - `double-free` in `Memory::realloc_static()` and `heap-use-after-free` in `CowData<AudioServer::Bus*>::size()` |
| Fix attempt 1: wrap each of the ~20 unlocked accessors in `lock()`/`unlock()` (or `const_cast<AudioServer*>(this)->lock()/unlock()` for const getters), but leave the pre-existing `ERR_FAIL_INDEX(p_bus, buses.size())` bounds check *before* `lock()`, matching this file's own established (but broken) convention | Same build/run, 10 attempts | **10/10 still crashed** - including a clean hit inside `AudioServer::set_bus_mute()` itself. Root cause: the bounds check reads `buses.size()` unsynchronized, so even once the lock is acquired the already-validated index can be stale relative to a concurrent resize |
| Fix attempt 2: move the bounds check to *after* `lock()` in all ~23 touched accessors (manually replicate `ERR_FAIL_INDEX`/`ERR_FAIL_INDEX_V` as `if (...) { unlock(); ERR_FAIL_...; }` inside the locked region) | Same build/run, 10 attempts | **10/10 still crashed** - `double-free` in `Memory::realloc_static()`, 2 SIGABRTs. The repro's own writer threads call `add_bus()`/`remove_bus()` directly, and those two structural mutators were deliberately left untouched (assumed "already correctly locked" since they call `lock()` somewhere) |
| Investigation: read `add_bus()`/`remove_bus()`/`move_bus()` bodies directly | Static read | `remove_bus()` had the identical bounds-check-before-lock bug (`ERR_FAIL_INDEX(p_index, buses.size());` then `lock();`). `add_bus()` had **no locking at all** anywhere in its body - it reads `buses.size()`, loops over `buses[j]->name`, and does `buses.push_back()`/`buses.insert()`/`bus_map[attempt]=bus` completely unsynchronized. `move_bus()` had the same unlocked-bounds-check pattern and no locking in its body either (not reached by this repro, fixed anyway for consistency) |
| Fix attempt 3: lock `remove_bus()`'s bounds check inside the lock, wrap `add_bus()`'s entire body in `lock()`/`unlock()`, same for `move_bus()` | Same build/run, 10 attempts | **9/10 crashed, 1/10 survived** - `heap-use-after-free servers/audio/audio_server.cpp:1532 in AudioServer::update()` |
| Investigation: traced `AudioServer::update()`'s caller | `grep -rn "AudioServer::get_singleton()->update()"` | Called once per frame from `main/main.cpp:5143` (main thread). Its `#ifdef DEBUG_ENABLED` block iterates `buses` and dereferences `bus->bypass`/`bus->effects` with **zero locking**, racing the now-fixed `add_bus()`/`remove_bus()` |
| Fix attempt 4 (final): add `lock()`/`unlock()` around `update()`'s two buses-iterating loops, held across both (since `update()` runs on the main thread, not the audio-mixing thread, this cannot self-deadlock against the same `lock()`) | Same build/run, 10 attempts | **Clean 10/10** - every run printed `"survived"` |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes, 10/10 clean runs with the complete fix (after three insufficient
intermediate attempts, each of which genuinely reduced but did not eliminate the race).

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built ASan
editor binary, driving `AudioServer`'s public GDScript API (`add_bus`/`remove_bus`/
`set_bus_volume_db`/`get_bus_volume_db`/`set_bus_mute`) plus the engine's own per-frame `update()`
call, with no test-only instrumentation.

## Class

`memory-safety` - a `Vector<Bus *>`-backed structure racing across three independent code paths that
each needed their own fix: (1) ~20 "simple" accessor methods with no lock at all, (2) two structural
mutators (`remove_bus`, and separately `add_bus` with zero locking whatsoever) whose own bounds checks
or entire bodies raced the vector's own resize/reallocation, and (3) the engine's own per-frame
`AudioServer::update()`, which is not part of the public scripting API surface most candidates in this
session have focused on, but still races the same unsynchronized `Vector<Bus *>`.

## Root cause

`AudioServer` predates (or was simply never migrated to) the `_THREAD_SAFE_CLASS_`/
`_THREAD_SAFE_METHOD_` convention used elsewhere, instead hand-rolling `lock()`/`unlock()` calls
method-by-method. This produced three independent gaps:

1. ~20 accessors (`set_bus_volume_db`, `get_bus_volume_db`, `set_bus_mute`, etc.) simply never called
   `lock()`/`unlock()` at all.
2. Several methods that *did* call `lock()` (`set_bus_name`, `add_bus_effect`, `remove_bus`, and the
   ~20 from (1) in their pre-existing `ERR_FAIL_INDEX` guards) performed their bounds check against
   `buses.size()` *before* acquiring the lock - so even a "fixed" method could still validate an index
   against a size that a concurrent `add_bus`/`remove_bus`/`set_bus_count` was in the middle of
   changing, and then dereference `buses[p_bus]` once inside the lock using the now-stale,
   possibly-out-of-range index.
3. `add_bus()` itself had no lock in its body at all, and `AudioServer::update()` (an internal,
   non-`ClassDB`-bound method invoked once per frame by `main/main.cpp`) iterated `buses` under
   `#ifdef DEBUG_ENABLED` with zero locking - neither of these was part of the original "bus setters"
   risk-map lead, and both were only found by following the crash signatures after the first two fix
   attempts still failed.

## Introduced by

Not independently dated - `AudioServer`'s locking has apparently been incomplete/inconsistent since
the bus system was introduced; this is a pre-existing, systemic gap rather than a recent regression.

## The fix

See `patch.diff` (`+197/-72` in `servers/audio/audio_server.cpp`):

- `remove_bus()`: bounds check moved from before `lock()` to inside it.
- `add_bus()`: entire body now wrapped in `lock()`/`unlock()` (previously had none).
- `move_bus()`: entire body now wrapped in `lock()`/`unlock()`, bounds check moved inside the lock
  (not reached by this repro, fixed for consistency since it has the identical shape).
- `get_bus_count()`, `set_bus_name()`, `get_bus_name()`, `get_bus_index()`, `set_bus_volume_db()`,
  `get_bus_volume_db()`, `get_bus_channels()`, `set_bus_send()`, `get_bus_send()`, `set_bus_solo()`,
  `is_bus_solo()`, `set_bus_mute()`, `is_bus_mute()`, `set_bus_bypass_effects()`,
  `is_bus_bypassing_effects()`, `add_bus_effect()`, `remove_bus_effect()`, `get_bus_effect_count()`,
  `get_bus_effect_instance()`, `get_bus_effect()`, `swap_bus_effects()`, `set_bus_effect_enabled()`,
  `is_bus_effect_enabled()`, `get_bus_peak_volume_left_db()`, `get_bus_peak_volume_right_db()`,
  `is_bus_channel_active()`: each now calls `lock()` (or `const_cast<AudioServer*>(this)->lock()` for
  `const` getters, since `lock()`/`unlock()` are non-const and reached via `AudioDriver`, not a
  `mutable` member) as the first statement, with any `ERR_FAIL_INDEX`/`ERR_FAIL_INDEX_V` bounds check
  replaced by an equivalent manual `if (...) { unlock(); ERR_FAIL_...; }` performed *after* the lock is
  held, and `unlock()` before every return path.
- `AudioServer::update()`: `lock()`/`unlock()` added around the two `buses`-iterating loops inside its
  `#ifdef DEBUG_ENABLED` block (both loops covered by a single lock/unlock pair, held across both,
  since this runs on the main thread, never the audio-mixing thread, so it cannot self-deadlock).

`set_bus_volume_linear()`/`get_bus_volume_linear()` were deliberately **not** independently locked -
they delegate to the now-locked `set_bus_volume_db()`/`get_bus_volume_db()`, covered transitively.

Verified: 10/10 clean runs of `repro/` with the complete fix applied (vs. 3/3 crashing unfixed, and
10/10 and 9/10 still crashing after the first two intermediate, insufficient fix attempts). Test
suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the established
baseline.

## What's not done / open questions

- This is the first candidate this session where a naive "add a lock around the unlocked body" fix
  was insufficient on the *first* retry - two additional, genuinely distinct gaps (`add_bus()`'s total
  absence of locking, and `AudioServer::update()`'s own unlocked per-frame iteration) had to be found
  by reading the ASan signature of the *still-failing* "fixed" build each time, rather than being
  visible from a single static read of the file up front. Worth remembering as a general lesson: when
  a fix to the obviously-flagged methods doesn't close a race, check (a) whether the bounds check
  guarding the fixed methods' own bodies happens before or after the lock, and (b) whether any
  *internal*, non-`ClassDB`-bound method reachable from a timer/frame callback touches the same
  container without ever having been in scope for the original lead.
- `AudioDriver`'s own mixing-thread code path (the actual real-time audio callback) was not separately
  audited in this candidate - the fix only closes races between script-driven threads and the main
  thread's `update()`; it does not independently re-verify that the audio-mixing thread's own reads of
  `buses` happen under the same lock (though since `lock()`/`unlock()` here directly delegate to
  `AudioDriver`'s own lock, this was presumably already the intended synchronization point for that
  thread).
- Found via fresh discovery following up on a risk-map lead flagged-but-not-investigated in an earlier
  session (candidates 001-069), the first candidate this session targeting a `servers/` singleton
  rather than a `scene/resources/` or `core/` container type.
