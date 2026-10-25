# `AudioServer::add_bus()`/`move_bus()` mutate `buses`/`bus_map` without taking the class's own lock

`servers/audio/audio_server.cpp`: `AudioServer::move_bus()` (:744 onward) and `add_bus()` (:677
onward) mutate the `buses` `Vector<Bus *>` (and `add_bus()` also `bus_map`) with no
`lock()`/`unlock()` call, while their siblings `set_bus_count()` (:598) and `remove_bus()` (:660)
in the same file correctly bracket the same kind of mutation in `lock()`/`unlock()` - a missed-lock
asymmetry within one class, the same shape as candidates 002/004/009/010. Found via an external
6-thread-bombardment sweep (`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md`, 2026-10-05)
that already produced a minimal reproduction project for `move_bus()`
(`~/Projekty/Godot/Qarminer/problems/crashes/20261005_163945_crashes_AudioServer_move_bus_race/`),
adapted into this candidate's `gdscript_repro/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `audio_server.cpp` lock-usage audit across all 4 bus-mutating entry points | n/a | `set_bus_count()`/`remove_bus()` call `lock()`/`unlock()` around their `buses`/`bus_map` mutation; `add_bus()`/`move_bus()` do not, at all |
| GDScript-level repro (`gdscript_repro/`, adapted from the external sweep's ready-made MRP), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes` (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb), `--headless --path gdscript_repro`, 4 `Thread`s spamming `AudioServer.move_bus(1,2)`/`move_bus(2,1)` for 15s each, against 3 buses | **Crashed on the first run and all 3 runs tried** - real, fully-symbolized ASan `unknown-crash` (`READ of size 8`) inside `CowData<AudioServer::Bus*>::size()` / `Vector<AudioServer::Bus*>::size()`, called from `AudioServer::move_bus` at `audio_server.cpp:734`, from a `CoreBind::Thread` worker |
| Fix (lock()/unlock() added to both `add_bus()` and `move_bus()`), same build/run, 8 runs | Same build/run | **Clean 8/8 runs** |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-010 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried), 0/3 crashes with `add_bus()`
also exercised indirectly via `set_bus_count(3)` in setup (no crash observed there this session -
see "What's not done"), and 8/8 clean runs with the complete fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `AudioServer` singleton through nothing but the
public GDScript API (`AudioServer.set_bus_count()`, `.move_bus()`). No second participant is
involved (not a peer/wire-input finding) - this is a same-process data race between GDScript
`Thread`s and (incidentally, per the engine's own `AudioDriverDummy` background thread) the audio
mixer, both calling into the same unsynchronized class.

## Class

`memory-safety` (concurrent unsynchronized `Vector`/`CowData` mutation - same mechanism as
candidates 004's `LocalVector` race and 009's `CameraServer::feeds`, on `AudioServer::buses`
instead)

## Root cause

`AudioServer::buses` (`Vector<Bus *>`) and `bus_map` (`HashMap<StringName, Bus *>`,
`servers/audio/audio_server.h`) are mutated by four entry points in `audio_server.cpp`:
`set_bus_count()` (:598), `remove_bus()` (:660), `add_bus()` (:677), `move_bus()` (:733). The first
two correctly bracket their mutation in `lock()`/`unlock()` (the class's own `AudioServer::lock()`/
`unlock()` wrap a `Mutex audio_server_lock` the class declares for exactly this purpose - also
taken by `AudioDriverDummy::thread_func()`, `servers/audio/audio_driver_dummy.cpp`, before every
mix). `add_bus()` and `move_bus()` were never given the same treatment - `move_bus()`'s
`buses.remove_at(p_bus)` followed by `buses.push_back(bus)`/`buses.insert(...)` runs completely
unguarded, so a second thread's concurrent call (another `move_bus()`, or the mixer thread's locked
`AudioServer::audio_server_process()` reading `buses.size()`/indexing into it) can observe the
`Vector`'s backing `CowData` mid-`memmove`/mid-reallocation, producing the heap corruption ASan
caught here. This reproduces with just GDScript `Thread`s calling `move_bus()` against each other -
no interaction with the real mixer thread is required, though it's also racing concurrently in the
background the whole time per `AudioDriverDummy`'s own locked access.

Reachability: `AudioServer` is a globally accessible singleton, fully scriptable from any thread -
no special setup beyond `AudioServer.set_bus_count()` is needed to reach `move_bus()`/`add_bus()`.

## Introduced by

3b019bf644 "Ability to delete, drag and drop audio buses!" (2017-01-23) - this is the commit that
first gave `move_bus()` a real body (`buses.remove(p_bus)` then insert/push_back; the function had
existed as an empty stub since 0aa7242624 two days earlier). No `lock()`/`unlock()` call was ever
present in `move_bus()` at any point in its history (`git log -S"void AudioServer::move_bus"` shows
only this one body-adding change and no match for a later patch removing a lock) - so this has been
a plain missed lock since the feature's introduction, not a regression from a later refactor.

## The fix

Add `lock()`/`unlock()` around `add_bus()`'s and `move_bus()`'s existing `buses`/`bus_map`
mutation, matching `set_bus_count()`/`remove_bus()`'s exact convention in the same file (`+8/-0`
against `servers/audio/audio_server.cpp`, see `patch.diff`):

```cpp
void AudioServer::move_bus(int p_bus, int p_to_pos) {
	ERR_FAIL_COND(p_bus < 1 || p_bus >= buses.size());
	ERR_FAIL_COND(p_to_pos != -1 && (p_to_pos < 1 || p_to_pos > buses.size()));

	MARK_EDITED

	if (p_bus == p_to_pos) {
		return;
	}

	lock();

	Bus *bus = buses[p_bus];
	buses.remove_at(p_bus);
	...
	unlock();

	AudioDriver::get_singleton()->move_sample_bus(p_bus, p_to_pos);
	...
}
```
(and identically wrapping `add_bus()`'s naming-loop-through-insertion block). `AudioServer::lock()`
wraps a plain, non-recursive `Mutex` (`audio_server.h`) - neither function calls any other
lock-taking `AudioServer` method while holding the lock, so no re-entrancy concern, matching the
existing `set_bus_count()`/`remove_bus()` precedent exactly.

Verified: 8/8 clean runs of `gdscript_repro/` against the fix (vs. crashing on all 3/3 unfixed
runs tried).

- **Applied for real** to `servers/audio/audio_server.cpp`, rebuilt the ASan editor binary
  (incremental, ~82s), ran `gdscript_repro/` and the test suite against the fix, then reverted
  (`git checkout -- servers/audio/audio_server.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan +
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed | 3
  skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 failure,
  unrelated to this change). No new failures.

## What's not done / open questions

- **`add_bus()`'s fix was not independently confirmed by its own repro this session** - it was
  fixed because it has the identical missing-lock shape as `move_bus()` in the same file (same
  mutation of `buses`/`bus_map`, same sibling functions correctly locking), not because a dedicated
  concurrent-`add_bus()` reproduction was built and run. A future session could build one (spam
  `AudioServer.add_bus()`/`remove_bus()` from multiple threads) to independently verify this half
  of the fix the same way `move_bus()` was verified here.
- Did not check whether other `AudioServer` methods beyond these four touch `buses`/`bus_map`
  without locking (e.g. bus-effect accessors) - only the four bus-count/order mutators named in the
  external sweep's finding were audited this session.
- Did not attempt to resolve the raw unfixed-run backtrace beyond the one captured in
  `gdscript_output_crash.txt` - the full raw log is ~119k lines of repeated `ERR_FAIL_COND` spam
  from the 4 racing threads (each racing against the others' concurrent `remove_at()`/`insert()`,
  which transiently puts `p_bus`/`p_to_pos` out of range from the caller's point of view) - this is
  expected noise from the race itself, not a second distinct bug, and wasn't further triaged line by
  line.
