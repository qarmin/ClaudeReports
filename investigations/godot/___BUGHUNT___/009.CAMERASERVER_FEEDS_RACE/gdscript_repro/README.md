# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting the fully-unlocked `CameraServer::feeds`
(`Vector<Ref<CameraFeed>>`, `servers/camera/camera_server.cpp`) through nothing but the public
GDScript API: `CameraFeed.new()`, `CameraServer.add_feed()`/`.remove_feed()` (writers),
`CameraServer.get_feed_count()`/`.feeds()` (readers). `CameraServer` declares a
`_THREAD_SAFE_CLASS_` mutex (`camera_server.h:50`) but - before this candidate's fix - never
actually uses it anywhere in `camera_server.cpp`. Built and actually run this session against a
real, self-built Godot editor binary (`scons platform=linuxbsd target=editor use_asan=yes
tests=yes debug_symbols=yes`, commit `b8420aa3a60689572a745c0f8a85c9df62d4cbfb`).

`main.gd` sets `CameraServer.monitoring_feeds = true`, then starts 8 writer `Thread`s (each
looping `CameraFeed.new()` → `add_feed()` → `remove_feed()`) and 8 reader `Thread`s (each
looping `get_feed_count()`/`feeds()`) - all racing on the same `Vector<Ref<CameraFeed>>`.

## Result: confirmed, reliably crashing unpatched, all clean patched

Unpatched, this reproduced on the **very first run** (and every subsequent run tried), with a
fully-symbolized real-engine ASan report:

```
==17150==ERROR: AddressSanitizer: unknown-crash on address 0x75067507c398 ...
READ of size 8 at 0x75067507c398 thread T33
    #0 CowData<Ref<CameraFeed> >::_insert_uninitialized(...) core/templates/cowdata.h:348
    #1 CowData<Ref<CameraFeed> >::_insert_uninitialized(...) core/templates/cowdata.h:154
    #2 CowData<Ref<CameraFeed> >::push_back(...) core/templates/cowdata.h:309
    #3 Vector<Ref<CameraFeed> >::push_back(...) core/templates/vector.h:75
    #4 CameraServer::add_feed(Ref<CameraFeed> const&) servers/camera/camera_server.cpp:121
    ...
    #13 CoreBind::Thread::_start_func(void*) core/core_bind.cpp:1493
```

Other runs crashed with a fatal, intentional `CRASH_BAD_UNSIGNED_INDEX`-style abort from
`Vector::operator[]`'s own bounds check instead (`core/templates/vector.h:47`) - both are real
manifestations of the same concurrent `Vector<Ref<CameraFeed>>` corruption. Full output:
`gdscript_output_crash.txt`.

**First fix attempt was incomplete**: adding `_THREAD_SAFE_METHOD_` to the 7 methods visible in
`camera_server.h`'s public interface (`add_feed`, `remove_feed`, `get_feed`, `get_feed_count`,
`get_feeds`, `get_feed_index`, `get_feed_by_id`) was not enough - the fixed binary still crashed,
now inside `CameraServer::get_free_id()` (`:75-91`), called from `CameraFeed`'s own constructor
(`camera_feed.cpp:156`) every time `CameraFeed.new()` runs, which *also* iterates `feeds` with no
lock and was missed in the first pass (it has no public binding, so didn't show up when auditing
`_bind_methods()`). Adding `_THREAD_SAFE_METHOD_` there too (the real `patch.diff`, `+16/-0`
total across 8 methods) finally eliminated the crash - a useful reminder that a method needing a
lock doesn't have to be itself script-bound to matter.

With the complete `../patch.diff` applied and the engine rebuilt (incremental, ~33-41s per
rebuild - only `camera_server.cpp` changed each time): **8/8 clean runs**, see
`gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/009.CAMERASERVER_FEEDS_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
Unlike candidates 005/008's narrower races, this one reproduced on every attempt during this
session's testing - expect an ASan abort (or a fatal bounds-check trap) within a few seconds on
an unpatched checkout, a clean ~15s run every time with `../patch.diff` applied.

Note: pass an **absolute** path for `XDG_DATA_HOME` if you set it - a relative path is silently
ignored by Godot (falls back to `$HOME/.local/share`), which doesn't affect this particular race
but did cause a separate, already-known, already-recorded bug (candidate 001, Logger null-deref
on an unwritable user data dir) to fire during this session's earlier test-suite runs.
