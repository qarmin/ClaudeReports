# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting both `Curve2D` and `Curve3D` (`scene/resources/curve.h`/
`.cpp`) through nothing but their public GDScript API. Before this candidate's fix, both classes had
**zero** locking anywhere - confirmed via `grep -n "Mutex" scene/resources/curve.h` returning no
matches for either class - despite being ordinary `Resource`s that can be (and commonly are) shared
and concurrently accessed (e.g. a `Path2D`/`Path3D`'s `curve` resource read by a physics/navigation
thread while an editor or procedural-generation thread mutates it). Built and actually run this
session against a real, self-built Godot editor binary (`scons platform=linuxbsd target=editor
use_asan=yes tests=yes debug_symbols=yes cpp_compiler_launcher=ccache c_compiler_launcher=ccache`,
commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb).

`main.tscn`'s inline script creates one `Curve2D` (2 points) and one `Curve3D` (2 points), then
starts 3+3 `Thread`s: 3 randomly call `add_point`/`remove_point`/`samplef`/`sample_baked`/
`get_point_position` on the shared `Curve2D`, the other 3 do the same on the shared `Curve3D`, all
for 15 seconds - modeled on candidate 016's own random-method-hammering methodology for the sibling
`Curve` class.

## Result: confirmed, reliably crashing unpatched, all clean patched

Unpatched, this reproduced reliably (5/5 runs) as a mix of fatal, intentional bounds-check traps
(`SIGILL`, Godot's own `ERR_FAIL_INDEX`-style crash) and one real ASan
`attempting double-free` heap-corruption report (run 1) - both are the same underlying `points`/
`baked_point_cache`/etc. corruption, just caught by different mechanisms depending on timing. Both
the `Curve2D` code path (`sample`/`sample_baked` around `curve.cpp:768`/`1026`) and the `Curve3D`
code path (`sample`/`sample_baked` around `curve.cpp:1476`/`1937`) crashed across the 5 runs - see
`gdscript_output_crash.txt` for one full captured run showing both.

With `../patch.diff` applied (`_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` across effectively all of
`Curve2D`'s and `Curve3D`'s public/`ClassDB`-bound methods touching `points`/`baked_cache_dirty`/
`baked_point_cache`/`baked_dist_cache`/etc.) and the engine rebuilt: **8/8 clean runs**, see
`gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/027.CURVE2D_CURVE3D_UNGUARDED_POINTS_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
Expect a crash (a fatal bounds-check trap or a real ASan heap-corruption report, both reachable
depending on timing) within a few seconds to ~15s on an unpatched checkout; a clean ~15s run every
time with `../patch.diff` applied.
