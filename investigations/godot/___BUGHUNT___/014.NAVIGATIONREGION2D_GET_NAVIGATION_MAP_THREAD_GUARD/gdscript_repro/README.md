# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting `NavigationRegion2D::get_navigation_map()`
(`scene/2d/navigation/navigation_region_2d.cpp:224`) through nothing but the public GDScript API.
This candidate started from an external sweep's broader hypothesis ("a worker thread touching a
Navigation node before its `NOTIFICATION_ENTER_TREE`-driven main-thread registration finishes races
it") but the **actual** confirmed mechanism, found by building a real repro and reading the resulting
crash backtrace rather than guessing, is narrower and simpler: `get_navigation_map()` calls
`get_world_2d()` (which already has its own `ERR_READ_THREAD_GUARD_V` - it logs an error and
returns a null `Ref<World2D>()` when called off the main thread/a thread group) and then
**immediately dereferences that return value unconditionally** (`get_world_2d()->get_navigation_map()`)
with no null check - turning a gracefully-guarded error into a real null-pointer-dereference
segfault. Built and actually run this session against a real, self-built Godot editor binary
(`scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, commit
b8420aa3a60689572a745c0f8a85c9df62d4cbfb).

`main.tscn`'s inline script instantiates one `NavigationRegion2D`, `add_child()`s it (on the main
thread, in `_ready()`), then immediately starts 6 `Thread`s that each call a random mix of
`set_enabled`/`set_use_edge_connections`/`set_navigation_layers`/`set_enter_cost`/
`set_travel_cost`/`get_navigation_map`/`is_enabled`/`get_navigation_layers` for 15 seconds -
modeled on the external sweep's own methodology (`~/Projekty/Godot/Qarminer/tools/thread/thread.gd`:
instantiate -> `add_child` -> spawn N worker threads hammering random bound methods), not a
hand-built stand-in. `map_override` is never set in this repro, so `get_navigation_map()` always
takes the `get_world_2d()->...` branch when called.

## Result: confirmed, reliably crashing unpatched, all clean patched

Unpatched, this reproduced on **every run tried** (3/3) as a real SIGSEGV (not caught by ASan's own
error reporting - a plain null-pointer dereference, not a heap/stack overflow ASan detects):

```
ERROR: ...@NavigationRegion2D@...: The function `get_world_2d()` on this node can only be accessed
from either the main thread or a thread group. Use `call_deferred()` instead.
   at: get_world_2d (scene/main/canvas_item.cpp:1254)
[... process then segfaults ...]
```

Re-run once under `gdb -batch -ex run -ex "thread apply all bt"` to get a real, fully-symbolized
crash backtrace (`gdb_backtrace.txt`):

```
Thread 29 "Node::_spam" received signal SIGSEGV, Segmentation fault.
0x0000555567efdf1e in World2D::get_navigation_map (this=0x0) at scene/resources/world_2d.cpp:52
#0  World2D::get_navigation_map (this=0x0) at scene/resources/world_2d.cpp:52
#1  NavigationRegion2D::get_navigation_map (this=<optimized out>) at scene/2d/navigation/navigation_region_2d.cpp:228
#2  call_with_validated_variant_args_retc_helper<...> (...) at ./core/variant/binder_common.h:133
...
#12 CoreBind::Thread::_start_func (...) at core/core_bind.cpp:1493
```

`this=0x0` on `World2D::get_navigation_map` is the smoking gun: `get_world_2d()`'s own thread guard
fired (logged the `ERROR:` line above) and returned a null `Ref<World2D>()`, and the caller
dereferenced it anyway. Full raw output: `gdscript_output_crash.txt`.

With `../patch.diff` applied (adds `ERR_READ_THREAD_GUARD_V(RID())` plus an explicit
`Ref<World2D>::is_valid()` check before dereferencing) and the engine rebuilt (incremental, ~53s -
single `.cpp` changed): **8/8 clean runs**, see `gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/014.NAVIGATIONREGION2D_GET_NAVIGATION_MAP_THREAD_GUARD/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
Expect a SIGSEGV within a few seconds on an unpatched checkout (reproduced on the very first run
every time this session); a clean ~15s run every time with `../patch.diff` applied. For a
symbolized backtrace instead of a bare exit code, run under `gdb -batch -ex run -ex "thread apply
all bt" --args <binary> --headless --path <this dir>`.
