# GDScript-level reproduction - actually run, real crash captured

A real, minimal Godot project hitting the `OPCODE_OPERATOR` first-run cache race
(`modules/gdscript/gdscript_vm.cpp:777-825`) through nothing but public GDScript syntax - every
GDScript binary operator expression (`a + b`) compiles to this opcode. Built and actually run
this session against a real, self-built Godot editor binary
(`scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`, commit
`b8420aa3a60689572a745c0f8a85c9df62d4cbfb`).

`main.gd` defines 300 near-identical methods (`op_0`...`op_299`, each `return a + b`) so there
are 300 independent "never run before" cache slots to try to race on. It then starts 32
`Thread`s and uses a `Mutex`+counter barrier so that, for each `op_i` in turn, **all 32 threads
call it for the first time at (as close as possible to) the same instant** - `Thread.start()`'s
own scheduling jitter alone was not enough to reliably land in the race window (see
`../RESULT.md`'s "What was tried" table for the naive version that got 0/5).

## Result: confirmed, 4/10 crashing runs unpatched, 8/8 clean runs patched

Unpatched, 4 of 10 runs crashed with a SIGSEGV inside the real engine:

```
handle_crash: Program crashed with signal 11
...
[1] ... handle_crash(int) at platform/linuxbsd/crash_handler_linuxbsd.cpp:95
[2] ... libc.so.6(+0x45cb0)
-- END OF C++ BACKTRACE --
GDScript backtrace (most recent call first):
    [0] op_96 (res://main.gd:203)
    [1] _runner (res://main.gd:622)
```

The short, non-symbolized C++ backtrace (nothing between `handle_crash` and raw libc) is itself
a signature of this bug: the crash happens *inside a call through a corrupted function pointer*,
which destroys the normal frame-pointer chain the unwinder relies on to produce a real
backtrace - unlike candidates 002-004's crashes, which always unwound cleanly into the exact
buggy function. The `op_N` named in the GDScript backtrace differs run to run (`op_96`, `op_21`,
`op_0`, `op_13` across the four crashing runs), consistent with the race being about *which*
expression's first run gets contended at the moment the barrier releases, not one fixed line.
Full output: `gdscript_output_crash.txt`.

With `../patch.diff` applied (acquire/release-ordered signature publish instead of plain `int`
reads/writes) and the engine rebuilt (incremental, ~48s - only `gdscript_vm.cpp` changed, by far
the fastest rebuild this session): 8/8 clean runs - see `gdscript_output_fixed.txt`.

## How to run it yourself

```sh
bin/godot.linuxbsd.editor.x86_64.san --headless --path ___BUGHUNT___/005.GDSCRIPT_OPCODE_OPERATOR_CACHE_RACE/gdscript_repro
```

(built with `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`).
This is a genuinely narrow race - expect roughly 2-4 crashes per 10 runs on an unpatched
checkout, not a guaranteed crash on every run - and a clean ~5-10s run every time with
`../patch.diff` applied.

Note: pass an **absolute** path for `XDG_DATA_HOME` if you set it - a relative path is silently
ignored by Godot (falls back to `$HOME/.local/share`), which doesn't affect this particular race
but did cause a separate, already-known, already-recorded bug (candidate 001, Logger null-deref
on an unwritable user data dir) to fire during this session's own earlier test-suite runs.
