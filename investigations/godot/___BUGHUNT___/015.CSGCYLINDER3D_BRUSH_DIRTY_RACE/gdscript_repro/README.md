# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting `CSGShape3D::_get_brush()`/`bake_collision_shape()`
(`modules/csg/csg_shape.cpp`, reached via the `CSGCylinder3D` subclass) through nothing but the
public GDScript API. This started from an external sweep's hypothesis
(`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md`): "`CSGShape3D`'s plain `bool dirty` +
`_make_dirty()`, rebuilt lazily by the main thread, no `Mutex` anywhere in the class" - confirmed
here with a real repro and a fully-symbolized crash, unlike candidate 014's disproven
enter-tree-race hypothesis for `NavigationRegion2D`.

`main.tscn`'s inline script instantiates one `CSGCylinder3D`, `add_child()`s it (main thread, in
`_ready()`), then starts 6 `Thread`s that each randomly either write `radius`/`height` (triggering
`_make_dirty()`) or call `bake_collision_shape()`/`get_meshes()` (triggering `_get_brush()` if
dirty) for 15 seconds - a mix of writer and reader pressure on the same lazily-rebuilt `brush`/
`dirty` state, modeled on the external sweep's own random-method-hammering methodology.

## Result: confirmed, reliably crashing unpatched, all clean patched

Unpatched, this reproduced on the **very first run** (and all 3 runs tried), with a real,
fully-symbolized ASan heap-use-after-free:

```
==1841803==ERROR: AddressSanitizer: heap-use-after-free on address 0x6cb1d6129328 ...
READ of size 8 at 0x6cb1d6129328 thread T31
    #0 CowData<CSGBrush::Face>::size() const core/templates/cowdata.h:185
    #1 Vector<CSGBrush::Face>::size() const core/templates/vector.h:93
    #2 CSGShape3D::_get_brush_collision_faces() modules/csg/csg_shape.cpp:895
    #3 CSGShape3D::bake_collision_shape() modules/csg/csg_shape.cpp:930
    ...
    #14 CoreBind::Thread::_start_func(void*) core/core_bind.cpp:1493

freed by thread T33 here:
    #0 free ...
    #1 memdelete<CSGBrush>(CSGBrush*) core/os/memory.h:169
    #2 CSGShape3D::_get_brush() modules/csg/csg_shape.cpp:491
```

Full output: `gdscript_output_crash.txt` (trimmed excerpt - the raw run is ~65k lines, almost
entirely repeated `ERR_READ_THREAD_GUARD`/`ERR_THREAD_GUARD` spam from the racing threads calling
`Node`/`Node3D` methods like `get_child_count()`/`update_gizmos()`/`update_configuration_warnings()`
off the main thread - expected noise from this same class of issue elsewhere in the engine, not a
separate bug, and not this candidate's own finding).

With the complete `../patch.diff` applied (adds a per-instance `brush_mutex`, held across
`_get_brush()`, `_make_dirty()`'s own `dirty` write, and - critically - the *entire* body of
`_get_brush_collision_faces()`, not just its inner call to `_get_brush()`) and the engine rebuilt:
**8/8 clean runs**, see `gdscript_output_fixed.txt`.

**A first fix attempt was incomplete** - see `gdscript_output_fixed.txt` and `../RESULT.md`'s "The
fix" section for what was missed and why (the returned `CSGBrush*` stays valid only as long as the
lock that protected its creation is still held; a caller reading from it after `_get_brush()` has
already returned - and released the lock - can still race a second thread's rebuild).

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/015.CSGCYLINDER3D_BRUSH_DIRTY_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
Expect an ASan heap-use-after-free within a few seconds on an unpatched checkout; a clean ~15s run
every time with the complete `../patch.diff` applied.
