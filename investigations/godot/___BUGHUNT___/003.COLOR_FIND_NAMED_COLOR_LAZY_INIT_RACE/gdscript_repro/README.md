# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting `Color::find_named_color()`'s missing lock
(`core/math/color.cpp:412-431`) through the actual public GDScript API -
`Color.from_string(name, default)` falls back to `Color.named()` ->
`find_named_color()` whenever the string isn't a valid hex color. Built and
actually run this session against a real, self-built Godot editor binary
(`scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`,
commit `ebf68e6f024eda115c301595d9296f23f2a254eb`).

## Result: confirmed, 3/3 crashing runs unpatched, 6/6 clean runs patched

Unpatched, 3/3 runs crashed with AddressSanitizer reporting a heap-buffer-overflow
**inside the real engine**, not a stand-in:

```
==69205==ERROR: AddressSanitizer: heap-buffer-overflow ...
WRITE of size 8 at 0x7bdb0b0300c8 thread T29
    #0 HashMap<String, int, ...>::_insert_empty_at(...) core/templates/hash_map.h:143
    #1 HashMap<String, int, ...>::_insert_element(...) core/templates/hash_map.h:159
    #2 HashMap<String, int, ...>::_insert(...) core/templates/hash_map.h:245
    #3 HashMap<String, int, ...>::operator[](...) core/templates/hash_map.h:577
    #4 Color::find_named_color(String const&) core/math/color.cpp:422
    #5 Color::named(String const&, Color const&) core/math/color.cpp:405
    #6 Color::from_string(String const&, Color const&) core/math/color.cpp:454
    #7-#11 (Variant static-method call machinery)
    #12 GDScriptFunction::call(...) modules/gdscript/gdscript_vm.cpp:2166
    #13 GDScriptInstance::callp(...) modules/gdscript/gdscript.cpp:1976
    ...
    #16 CoreBind::Thread::_start_func(void*) core/core_bind.cpp:1493
```

Full output: `gdscript_output_crash.txt`. Frame #4 is `color.cpp:422` - the exact
unlocked `named_colors_hashmap[...]` insertion line - reached from real GDScript
`Thread`s via `CoreBind::Thread::_start_func` -> `Callable` -> `GDScriptFunction::call`
-> the actual `main.gd` script's `Color.from_string()` calls. Two of the three runs
manifested as a bare segfault (exit 139) instead of a clean ASan report (the
third shown above) - both are the same underlying corruption, ASan's own reporting
machinery can itself be hit by the race under heavy contention; all three are
real crashes of the real binary.

With `../patch.diff` applied (`OwnedMutex<HashMap<String,int>>` instead of a bare
`static HashMap`) and the engine rebuilt (incremental, ~60-70s since only
`core/math/color.cpp` changed): 6/6 total clean runs (3 right after applying the
fix, 3 more after a second rebuild cycle during verification) - see
`gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/003.COLOR_FIND_NAMED_COLOR_LAZY_INIT_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes`).
Expect an ASan abort (or bare segfault) within a few seconds on an unpatched
checkout, a clean ~15s run with `../patch.diff` applied.

Note: pass an **absolute** path for `XDG_DATA_HOME` if you set it - a relative
path is silently ignored by Godot (falls back to `$HOME/.local/share`), which
doesn't affect this particular race but did cause a separate, already-known,
already-recorded bug (candidate 001, Logger null-deref on an unwritable user
data dir) to fire during this session's own test-suite runs before that was
noticed.
