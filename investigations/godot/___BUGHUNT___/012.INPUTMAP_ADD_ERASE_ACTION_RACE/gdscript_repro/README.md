# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting `InputMap::add_action()`/`erase_action()`
(`core/input/input_map.cpp`) through nothing but the public GDScript API. Unlike `AudioServer`
(candidate 011) or `CameraServer` (candidate 009), `InputMap` had **zero** locking anywhere in the
class before this candidate's fix - no `_THREAD_SAFE_CLASS_`, no ad hoc `lock()`/`unlock()`, no
mutex of any kind guarding `input_map` (`HashMap<StringName, Action>`). Built and actually run this
session against a real, self-built Godot editor binary (`scons platform=linuxbsd target=editor
use_asan=yes tests=yes debug_symbols=yes`, commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb).

Adapted from a ready-made minimal reproduction project already produced by an external sweep
(`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md` and its
`problems/crashes/20261005_173454_crashes_InputMap_add_erase_action_race/` folder) - the GDScript
logic in `main.tscn` is the same script, just repackaged with this workspace's own `main.tscn`/
`project.godot` naming convention (see candidates 009/011) instead of that project's `Node.gd`/
`Node.tscn`.

`main.tscn`'s inline script starts 6 `Thread`s that each spam `InputMap.add_action("race_action")`
then `InputMap.erase_action("race_action")` back and forth for 15 seconds, all racing on the same
action name - no interaction with any other singleton is required.

## Result: confirmed, reliably crashing unpatched, all clean patched

Unpatched, this reproduced on the **very first run** (and all 3 runs tried), with a
fully-symbolized real-engine ASan report:

```
==1775==ERROR: AddressSanitizer: heap-use-after-free on address 0x73647c27e180 ...
READ of size 8 at 0x73647c27e180 thread T31
    #0 StringName::operator==(StringName const&) const core/string/string_name.h:120
    #1 HashMapComparatorDefault<StringName, void>::compare(...) core/templates/hashfuncs.h:306
    #2 HashMap<...>::_lookup_idx_unchecked(...) core/templates/hash_map.h:134
    #3 HashMap<...>::_lookup_idx(...) core/templates/hash_map.h:107
    #4 HashMap<...>::has(StringName const&) const core/templates/hash_map.h:332
    #5 InputMap::add_action(StringName const&, float) core/input/input_map.cpp:119
    ...
    #13 CoreBind::Thread::_start_func(void*) core/core_bind.cpp:1493
```

Full output: `gdscript_output_crash.txt` (the full raw run, only 214 lines - much shorter than
candidate 011's, since this race crashes almost immediately rather than needing many iterations to
line up).

With `../patch.diff` applied (adds `_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` to all 18 public
`InputMap` methods touching `input_map`/`default_builtin_cache`/
`default_builtin_with_overrides_cache`, matching the convention already established by candidates
008/009/010) and the engine rebuilt (full rebuild of dependents, ~9m26s - `input_map.h` is a widely
included header): **8/8 clean runs**, see `gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/012.INPUTMAP_ADD_ERASE_ACTION_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
Expect an ASan heap-use-after-free within a few seconds on an unpatched checkout (crashed well
before the 15s loop budget elapsed in all 3 runs tried); a clean ~15s run every time with
`../patch.diff` applied.
