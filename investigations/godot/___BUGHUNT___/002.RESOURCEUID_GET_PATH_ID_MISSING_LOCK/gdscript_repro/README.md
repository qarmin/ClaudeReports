# GDScript-level reproduction (no C++ needed) - actually run, real crash captured

This is a real, minimal Godot project that hits the exact same bug as `../repro/` -
`ResourceUID::get_path_id()`'s missing lock (`core/io/resource_uid.cpp:222-228`) - using only
documented, public GDScript API:

- Writer thread: `ResourceUID.add_id()` / `ResourceUID.remove_id()` (locked, real engine code path)
- Reader thread: `ResourceLoader.get_resource_uid()` -> internally calls `ResourceUID.get_path_id()`
  (**unlocked** - the bug)

Both calls are plain, documented, scriptable engine API - nothing here requires reading engine
source or writing C++. This was built and actually run this session against a real, self-built
Godot editor binary (`scons platform=linuxbsd target=editor use_asan=yes tests=yes
debug_symbols=yes`, commit `e7cfa294a0b81bed7986be04a848cc1832a3f083`).

## Result: confirmed, 3/3 crashing runs unpatched, 3/3 clean runs patched

Unpatched, 3/3 runs crashed with AddressSanitizer reporting a heap-use-after-free **inside the
real engine**, not a stand-in:

```
==14631==ERROR: AddressSanitizer: heap-use-after-free on address 0x76c71611e5b8 ...
READ of size 8 at 0x76c71611e5b8 thread T29
    #0 ResourceUID::get_path_id(String const&) const core/io/resource_uid.cpp:225
    #1 ResourceLoader::get_resource_uid(String const&) core/io/resource_loader.cpp:1415
    #2 ... (GDScript call machinery: MethodBindTR, GDScriptFunction::call, GDScriptInstance::callp)
    #10 CoreBind::Thread::_start_func core/core_bind.cpp:1493

0x76c71611e5b8 is located 40 bytes inside of 48-byte region [...]
freed by thread T28 here:
    #0 ... in void memdelete<HashMapElement<CharStringT<char>, long> >(...) core/os/memory.h:169
    #3 HashMap<...>::erase(CharStringT<char> const&) core/templates/hash_map.h:373
    #4 ResourceUID::remove_id(long) core/io/resource_uid.cpp:234

SUMMARY: AddressSanitizer: heap-use-after-free core/io/resource_uid.cpp:225 in ResourceUID::get_path_id(String const&) const
```

Full output: `gdscript_output_crash.txt` (frame #0 is `resource_uid.cpp:225` - the exact unlocked
`reverse_cache.getptr()` line; the freeing thread is a real `ResourceUID::remove_id()` call,
frame #4, triggered from GDScript's own `Thread` via `CoreBind::Thread::_start_func` -> `Callable`
-> `GDScriptInstance::callp` -> the actual `main.gd` script). Reproduced identically 3/3 runs.

With `../patch.diff` applied (adds `MutexLock l(mutex);` to `get_path_id()`) and the engine
rebuilt (`scons`, incremental, ~35-38s since only `core/io/resource_uid.cpp` changed): 3/3 runs
completed the full ~20s with "No crash within the time limit" and no ASan report. Full output:
`gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/002.RESOURCEUID_GET_PATH_ID_MISSING_LOCK/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes`; `.san` suffix is this
project's own naming for the ASan-instrumented binary). Expect an ASan abort within a few seconds
on an unpatched checkout, a clean ~20s run with `../patch.diff` applied.
