# `CameraLinux::set_monitoring_feeds()` races its own background thread's start/join lifecycle

`modules/camera/camera_linux.cpp`: `CameraLinux::set_monitoring_feeds()` does a check-then-act on
a plain `bool monitoring_feeds` (not atomic, not guarded by any lock), then calls
`camera_thread.start()`/`.wait_to_finish()` with **zero** synchronization. A separate `camera_mutex`
member exists on the same class (`modules/camera/camera_linux.h`), but it only guards the `feeds`
vector inside `_update_devices()` - it is never taken by `set_monitoring_feeds()`. Two threads
calling `set_monitoring_feeds()` concurrently can both pass the early-return check and both call
`start()` on the same already-started `Thread` object, or one can call `wait_to_finish()` (a join)
while another concurrently calls `start()` - both are undefined behavior at the OS thread-API
level. Distinct from already-confirmed candidate 009 (`CameraServer::feeds` `Vector` corruption in
`servers/camera/camera_server.cpp`) - same class family, a different unguarded member one directory
over in the Linux backend. Found via an external 6-thread-bombardment sweep
(`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md`, 2026-10-05) that already produced a
minimal reproduction project
(`~/Projekty/Godot/Qarminer/problems/timeouts/20261005_173227_timeouts_CameraServer_set_monitoring_feeds_race/`),
adapted into this candidate's `gdscript_repro/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `camera_linux.h`/`.cpp` lock-usage audit | n/a | `camera_mutex` exists and guards `feeds` in `_update_devices()`, but `set_monitoring_feeds()` - the function managing `exit_flag`/`camera_thread`'s start/stop lifecycle - takes no lock at all |
| GDScript-level repro (`gdscript_repro/`, adapted from the external sweep's ready-made MRP), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes` (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb), `--headless --path gdscript_repro` under an external `timeout -k 5 25`, 6 `Thread`s spamming `CameraServer.set_monitoring_feeds(<random bool>)` for 15s each | **Crashed (aborted) on the first run and all 3 runs tried** - `Thread::start()`'s own re-start-without-join detection logged repeatedly, followed by an uncaught `std::system_error` (`Invalid argument`) and `terminate()` |
| Fix (dedicated `monitoring_feeds_mutex` held across the whole function), same build/run, 8 runs | Same build/run | **Clean 8/8 runs**, each completing the full ~15s loop and printing `survived` |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-012 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (every attempt tried, each aborting well inside the
15-second budget), and 8/8 clean runs with the fix applied. This session only observed the "abort"
manifestation (`std::system_error`/`terminate`); the external sweep separately reports a genuine
hang/deadlock (confirmed via `/proc/<pid>/task/*/status` showing every thread blocked on a futex)
as an alternative outcome of the same race under different scheduling - not independently
reproduced here, but consistent with the same missing-lock mechanism (see "Root cause" below for
why both are plausible from the same bug).

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `CameraServer`/`CameraLinux` singleton through
nothing but the public GDScript API (`CameraServer.set_monitoring_feeds()`). No second participant
is involved (not a peer/wire-input finding) - this is a same-process data race between GDScript
`Thread`s calling the same unsynchronized accessor.

## Class

`data-race` (concurrent, unsynchronized `std::thread`-level `start()`/`join()`/re-assignment on the
same `Thread` object - a different mechanism from this session's other `memory-safety` candidates,
since the corruption here is at the OS thread-API level, not a C++ container)

## Root cause

`CameraLinux::set_monitoring_feeds()` (`modules/camera/camera_linux.cpp`) reads `monitoring_feeds`
and conditionally calls `camera_thread.start(...)` or `camera_thread.wait_to_finish()` with no lock
at all. Two concrete races follow directly from this:

1. **Concurrent `start()` on an already-started `Thread`**: if two threads both read
   `monitoring_feeds == false` before either writes it, both take the `if (p_monitoring_feeds)`
   branch and both call `camera_thread.start(...)`. Godot's own `Thread::start()`
   (`core/os/thread.cpp:73`) detects this specific case (a `std::thread` object already holding a
   live handle) and logs the "re-started without wait_to_finish()" error, but the underlying
   `std::thread` assignment itself still performs an OS-level `pthread_create`/handle-replacement
   that is not safe to race - the old handle can be clobbered while something else still expects to
   `join()` it, which is exactly the `EINVAL`/`std::system_error` this session's repro observed
   repeatedly before the final `terminate()`.
2. **Concurrent `start()` vs `wait_to_finish()` on the same object**: one thread calling
   `wait_to_finish()` (a blocking `join()`) can race a second thread's concurrent `start()`
   reassigning the same `Thread` member - `std::thread`'s own contract requires external
   synchronization for concurrent operations on one instance, which this function provided none of.

Both are real, OS-level undefined behavior from unsynchronized concurrent operations on the same
`std::thread`-backed object - not one deterministic bug, but the same missing-lock root cause
manifesting differently (abort here; a genuine deadlock in the external sweep's own testing)
depending on which specific interleaving the scheduler produces.

Reachability: `CameraServer`/`CameraLinux` is a globally accessible singleton, fully scriptable
from any thread via the `monitoring_feeds` property/`set_monitoring_feeds()` method - no camera
hardware or special setup is needed to reach this race on a Linux build.

## Introduced by

54685c3037 "Deactivate the `CameraServer` by default. Add `monitoring_feeds` property to
`CameraServer`. This saves resources for games that don't use a physical camera." (2025-03-16) -
this is the commit that introduced `monitoring_feeds` and `CameraLinux::set_monitoring_feeds()`'s
thread start/stop lifecycle management; it has never had any locking at any point since (`git log
-S"CameraLinux::set_monitoring_feeds"` shows only this one introducing commit, no later patch
touched its synchronization).

## The fix

Add a dedicated `Mutex monitoring_feeds_mutex` member (`camera_linux.h`) and hold it for the entire
body of `set_monitoring_feeds()` (`camera_linux.cpp`), serializing all calling threads around the
check-then-act and the `start()`/`wait_to_finish()` calls (`+9/-0` across `camera_linux.h`/`.cpp`,
see `patch.diff`):

```cpp
// camera_linux.h
Mutex camera_mutex;
Mutex monitoring_feeds_mutex;  // new, deliberately separate - see below

// camera_linux.cpp
inline void CameraLinux::set_monitoring_feeds(bool p_monitoring_feeds) {
	MutexLock lock(monitoring_feeds_mutex);

	if (p_monitoring_feeds == monitoring_feeds) {
		return;
	}
	...
}
```

**Deliberately a *separate* mutex from the existing `camera_mutex`, not a reuse of it** - this
mirrors the lock-ordering lesson from candidate 012 (`InputMap`): `wait_to_finish()` can block for
an arbitrary amount of time waiting for the background thread to notice `exit_flag` and return.
`_update_devices()` (the background thread's own loop) needs to keep taking `camera_mutex`
independently, on its own schedule, for it to ever reach the point of checking `exit_flag` and
returning. If `set_monitoring_feeds()` held *`camera_mutex`* across its `wait_to_finish()` call, a
background thread that happens to be about to enter its own `MutexLock lock(camera_mutex);` block
in `_update_devices()` would block waiting for the same mutex the caller holds while blocked in
`wait_to_finish()` waiting for that exact background thread to exit - a genuine deadlock introduced
by the "obvious" one-mutex fix. Using a separate, dedicated mutex that `_update_devices()` never
touches avoids this entirely: nothing the background thread needs to do in order to notice
`exit_flag` and exit depends on `monitoring_feeds_mutex` in any way.

Verified: 8/8 clean runs of `gdscript_repro/` against the fix (vs. aborting on all 3/3 unfixed runs
tried).

- **Applied for real** to `modules/camera/camera_linux.h`/`.cpp`, rebuilt the ASan editor binary
  (~9m - a fuller rebuild than a single-TU change, consistent with touching a module header), ran
  `gdscript_repro/` and the test suite against the fix, then reverted (`git checkout --
  modules/camera/camera_linux.h modules/camera/camera_linux.cpp`) before finishing this candidate -
  main tree confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan +
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed | 3
  skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 failure,
  unrelated to this change). No new failures.

## What's not done / open questions

- This session's repro only reproduced the "abort" manifestation (`std::system_error`/
  `terminate()`), not the hang/deadlock the external sweep also reports from the same root cause -
  not independently confirmed here, though the code-reading argument for why both are plausible
  outcomes of the same unsynchronized `start()`/`wait_to_finish()` race stands on its own.
- `CameraServer::is_monitoring_feeds()` (base class, `_FORCE_INLINE_ bool is_monitoring_feeds()
  const { return monitoring_feeds; }`) still reads `monitoring_feeds` without taking
  `monitoring_feeds_mutex` - a plain `bool` read/write race remains between this accessor and
  `set_monitoring_feeds()`'s now-locked write, though it's a far narrower hazard (a torn/stale bool
  read, not a `std::thread` lifecycle race) and wasn't in scope for this candidate's confirmed
  finding.
- Did not check whether `CameraAndroid`/`CameraApple` (the other two platform backends,
  `modules/camera/camera_android.cpp`/`camera_apple.mm`) have an analogous unguarded thread/lifecycle
  pattern - only the Linux backend (where the external sweep's repro actually runs, and where this
  session's own toolchain targets) was checked.
