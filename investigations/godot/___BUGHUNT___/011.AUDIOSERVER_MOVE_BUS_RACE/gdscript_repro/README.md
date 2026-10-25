# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting `AudioServer::add_bus()`/`move_bus()`
(`servers/audio/audio_server.cpp`) through nothing but the public GDScript API. Unlike
`set_bus_count()`/`remove_bus()` in the same file, these two mutate the `buses`/`bus_map`
`Vector`/`HashMap` without ever calling `lock()`/`unlock()`. Built and actually run this session
against a real, self-built Godot editor binary (`scons platform=linuxbsd target=editor use_asan=yes
tests=yes debug_symbols=yes`, commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb).

Adapted from a ready-made minimal reproduction project already produced by an external sweep
(`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md` and its
`problems/crashes/20261005_163945_crashes_AudioServer_move_bus_race/` folder) - the GDScript logic
in `main.tscn` is the same script, just repackaged with this workspace's own `main.tscn`/
`project.godot` naming convention (see candidate 009) instead of that project's `Node.gd`/
`Node.tscn`.

`main.tscn`'s inline script sets up 3 buses, then starts 4 `Thread`s that each spam
`AudioServer.move_bus(1, 2)` / `move_bus(2, 1)` back and forth for 15 seconds - no interaction
with the real audio-mixer thread is required for this to race, though `AudioDriverDummy`'s own
background thread (which takes the *same* lock while mixing, see `servers/audio/audio_driver_dummy.cpp`)
is also running concurrently the whole time.

## Result: confirmed, reliably crashing unpatched, all clean patched

Unpatched, this reproduced on the **very first run** (and all 3 runs tried), with a
fully-symbolized real-engine ASan report:

```
==449==ERROR: AddressSanitizer: unknown-crash on address 0x75e8f18ddd70 ...
READ of size 8 at 0x75e8f18ddd70 thread T31
    #0 CowData<AudioServer::Bus*>::size() const core/templates/cowdata.h:185
    #1 Vector<AudioServer::Bus*>::size() const core/templates/vector.h:93
    #2 AudioServer::move_bus(int, int) servers/audio/audio_server.cpp:734
    ...
    #11 CoreBind::Thread::_start_func(void*) core/core_bind.cpp:1493
```

Full output: `gdscript_output_crash.txt` (trimmed excerpt - the raw run is ~119k lines, almost
entirely repeated `ERR_FAIL_COND` spam from the 4 racing threads calling `move_bus()` with
transiently out-of-range indices while another thread's `remove_at()`/`insert()` is mid-flight;
this is expected noise from the race itself, not a separate bug).

With `../patch.diff` applied (adds `lock()`/`unlock()` to both `add_bus()` and `move_bus()`,
matching `set_bus_count()`/`remove_bus()`'s existing convention in the same file) and the engine
rebuilt (incremental, ~82s - only `audio_server.cpp` and one unrelated already-stale TU changed):
**8/8 clean runs**, see `gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/011.AUDIOSERVER_MOVE_BUS_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
Expect an ASan abort within a few seconds to ~15s on an unpatched checkout; a clean ~15s run every
time with `../patch.diff` applied.

Only `move_bus()` was independently reproduced this session - `add_bus()` was fixed alongside it
because it has the identical missing-lock shape in the same file (confirmed by code reading, not
by its own dedicated repro). See `../RESULT.md`'s "What's not done" section.
