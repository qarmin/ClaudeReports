# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting the missing lock in `Input::action_press()`/
`action_release()` (`core/input/input.cpp:1515-1543` before the fix) and their unlocked sibling
readers (`is_action_pressed()`, `get_action_strength()`, etc., `:415-580`) through nothing but
the public GDScript `Input` singleton API. Built and actually run this session against a real,
self-built Godot editor binary (`scons platform=linuxbsd target=editor use_asan=yes tests=yes
debug_symbols=yes`, commit `b8420aa3a60689572a745c0f8a85c9df62d4cbfb`).

`main.gd` starts 16 `Thread`s, each looping `Input.action_press(action)` /
`Input.action_release(action)` over 10 built-in input actions (`ui_accept`, `ui_cancel`, etc. -
no custom input map needed) - both calls mutate the engine's single
`HashMap<StringName, ActionState> action_states` with **zero** locking before the fix, so 16
threads concurrently calling `HashMap::operator[]` (which can trigger an insert/rehash) on the
same map race each other directly - no interaction with the engine's own main-thread input
processing is even required to trigger this.

## Result: confirmed, crashing runs unpatched, all clean runs patched

Unpatched, out of 14 total runs across this session's testing (8 in the first batch, 6 more with
core dumps enabled for analysis), **4 runs crashed** (SIGSEGV). Unlike every other candidate this
session, these crashes did **not** produce a usable ASan report or backtrace - neither ASan's own
signal handler nor `gdb` analyzing a core dump could unwind a sane call stack:

```
$ gdb -batch -ex "bt" bin/godot.linuxbsd.editor.x86_64.san core.14487
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007d2dbdf1bebb in ?? ()
#1  0x0000603580b53050 in ?? ()
#2  0x00007ffc9931d640 in ?? ()
...
```

This is itself informative, not just an inconvenience: a completely garbage, non-symbolizable
stack (rather than a clean single-pointer fault landing in a recognizable function, as every other
candidate this session produced) is a signature of **severe concurrent corruption** - 16 threads
simultaneously calling `HashMap::operator[]` on the same map, several possibly mid-rehash at
once, can scribble over far more of the heap (and, via `ActionState`'s own fields, potentially
unrelated adjacent allocations) than a single reader-vs-writer race does, eventually crashing
somewhere unrelated to any of the actual corrupting writes. Both the first batch's bare
`Segmentation fault` output (`gdscript_output_crash_bare_sigsegv.txt`) and the later
core-dump-enabled runs showed the identical signature.

With `../patch.diff` applied (`_THREAD_SAFE_METHOD_` added to all 9 previously-unlocked
`action_states`-touching methods) and the engine rebuilt (incremental, ~44s - only `input.cpp`
changed): **10/10 clean runs**, see `gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/008.INPUT_ACTION_PRESS_RELEASE_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
This is a narrow-ish race (roughly 2-4 crashes per 10 runs on an unpatched checkout in this
session's testing) - run it several times if the first attempt doesn't crash. Expect a clean
~15s run every time with `../patch.diff` applied.

Note: pass an **absolute** path for `XDG_DATA_HOME` if you set it - a relative path is silently
ignored by Godot (falls back to `$HOME/.local/share`), which doesn't affect this particular race
but did cause a separate, already-known, already-recorded bug (candidate 001, Logger null-deref
on an unwritable user data dir) to fire during this session's earlier test-suite runs.
