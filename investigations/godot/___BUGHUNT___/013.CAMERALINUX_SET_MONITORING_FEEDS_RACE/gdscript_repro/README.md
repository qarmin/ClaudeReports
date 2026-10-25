# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting `CameraLinux::set_monitoring_feeds()`
(`modules/camera/camera_linux.cpp`) through nothing but the public GDScript API
(`CameraServer.set_monitoring_feeds()`). Unlike already-confirmed candidate 009
(`CameraServer::feeds` Vector corruption, `servers/camera/camera_server.cpp`), this is a
**different** unguarded member one directory over in the Linux platform backend: a plain
`bool monitoring_feeds` (not atomic) check-then-act, followed by unsynchronized
`camera_thread.start()`/`.wait_to_finish()` calls. Built and actually run this session against a
real, self-built Godot editor binary (`scons platform=linuxbsd target=editor use_asan=yes
tests=yes debug_symbols=yes`, commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb).

Adapted from a ready-made minimal reproduction project already produced by an external sweep
(`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md` and its
`problems/timeouts/20261005_173227_timeouts_CameraServer_set_monitoring_feeds_race/` folder) -
the GDScript logic in `main.tscn` is the same script, just repackaged with this workspace's own
`main.tscn`/`project.godot` naming convention (see candidates 009/011/012) instead of that
project's `Node.gd`/`Node.tscn`.

`main.tscn`'s inline script starts 6 `Thread`s that each spam
`CameraServer.set_monitoring_feeds(true/false)` with a random bool for 15 seconds, all racing on
the same `CameraLinux` singleton instance's thread-lifecycle state.

## Result: confirmed, reliably crashing unpatched, all clean patched

Unpatched, this reproduced on **every run tried** (3/3), first as a stream of Godot's own
assertion errors, then escalating to a real process abort:

```
ERROR: A Thread object has been re-started without wait_to_finish() having been called on it.
   at: start (core/os/thread.cpp:73)
...
terminate called after throwing an instance of 'std::system_error'
  what():  Invalid argument
```

Full output: `gdscript_output_crash.txt`. The `ERR_FAIL_COND_V_MSG` in `Thread::start()`
(`core/os/thread.cpp:73`) fires repeatedly first (two threads both observe the stale,
unsynchronized `monitoring_feeds` bool as `false` and both call `camera_thread.start()` on the
same already-started `Thread` object), then a concurrent `start()`/`wait_to_finish()` pair racing
on the same `Thread`'s internal `id`/`thread` state corrupts it badly enough that a later
`thread.join()` (inside `wait_to_finish()`) throws `std::system_error` ("Invalid argument" -
`EINVAL` from the underlying `pthread_join`), uncaught, aborting the whole process via
`std::terminate()`. This matches the external sweep's own description ("hangs or aborts").

With `../patch.diff` applied (adds a dedicated `monitoring_feeds_mutex` wrapping the entire
function body of `set_monitoring_feeds()`, serializing concurrent callers around the
check-then-act and the `start()`/`wait_to_finish()` lifecycle) and the engine rebuilt (incremental,
~2.5min - only `camera_linux.cpp`/`.h` changed): **8/8 clean runs**, see `gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/013.CAMERALINUX_SET_MONITORING_FEEDS_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
Expect the `ERR_FAIL_COND_V_MSG` spam and/or a `std::terminate` abort within a few seconds on an
unpatched checkout; a clean ~15s run every time with `../patch.diff` applied.
