# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting `Curve` (`scene/resources/curve.cpp`) through nothing but the
public GDScript API. Before this candidate's fix, `Curve` had **zero** locking anywhere in the
class - confirmed via `grep -n "Mutex" scene/resources/curve.h` returning no matches - despite being
an ordinary `Resource` that can be (and commonly is) shared and concurrently accessed by multiple
owners (e.g. an `AnimationCurve`/gradient resource referenced from several nodes, or accessed from a
background thread doing procedural generation). Built and actually run this session against a real,
self-built Godot editor binary (`scons platform=linuxbsd target=editor use_asan=yes tests=yes
debug_symbols=yes`, commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb).

`main.tscn`'s inline script creates one `Curve` with 2 points, then starts 6 `Thread`s that each
randomly call `add_point()`/`remove_point()`/`sample()`/`sample_baked()`/`get_point_position()` for
15 seconds, all racing on the same `_points`/`_baked_cache` state - modeled on the external sweep's
own random-method-hammering methodology.

## Result: confirmed, reliably crashing unpatched, all clean patched

Unpatched, this reproduced reliably (5/5 runs with the corrected repro script; see below) as either
a real ASan heap-use-after-free or a fatal, intentional bounds-check trap - both are the same
underlying `_points`/`_baked_cache` corruption, just caught by different mechanisms depending on
timing:

```
[3] LocalVector<Curve::Point, ...>::operator[](unsigned int) const at core/templates/local_vector.h:238
[4] CowData<float>::size() const at core/templates/cowdata.h:185
[5] Curve::sample_baked(float) const at scene/resources/curve.cpp:560
...
[13] CoreBind::Thread::_start_func(void*) at core/core_bind.cpp:1496
```

Full output: `gdscript_output_crash.txt`. **Note on repro script history**: the first draft of
`main.tscn`'s script called `curve.get_point(...)` in one of its random branches, which produced
harmless `SCRIPT ERROR: Invalid call. Nonexistent function 'get_point'` noise (`Curve::get_point()`
exists in C++ but, unlike `get_point_position()`/`get_point_left_tangent()`/etc., is not
`ClassDB`-bound) - fixed by switching that branch to `get_point_position()` before measuring the
5/5 repeat-run rate quoted above; this did not affect whether the real race reproduced, only added
script-level noise alongside it.

With `../patch.diff` applied (adds `_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` across effectively
`Curve`'s whole public API touching `_points`/`_baked_cache`/`_min_value`/`_max_value`/
`_min_domain`/`_max_domain`/`_bake_resolution`) and the engine rebuilt: **8/8 clean runs**, see
`gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/016.CURVE_ADD_REMOVE_POINT_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
Expect a crash (ASan heap-use-after-free, or a fatal bounds-check trap - both reachable depending on
timing) within a few seconds to ~15s on an unpatched checkout; a clean ~15s run every time with
`../patch.diff` applied.
