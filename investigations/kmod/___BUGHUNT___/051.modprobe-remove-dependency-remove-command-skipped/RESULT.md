# modprobe-remove-dependency-remove-command-skipped: a `remove` command configured for a dependency is never run when the dependency is removed as "unused"

Step 4 of `rmmod_do_module()` (tools/modprobe.c) removes dependencies that became unused with `rmmod_do_remove_module(dep)`, which calls `delete_module(2)` directly. A `remove <dep> <command>` line for that dependency is ignored: the module is unloaded behind the back of the command that is supposed to replace the plain unload (modprobe.d(5): "remove ... is invoked when modprobe -r is run"). The insertion side does the opposite - `install` commands of dependencies do run - and modprobe(8) for `-i` says "any dependent modules are still subject to commands set for them in the configuration file". The effect is also visible with `-r -i c`: the dependency's remove command still does not run.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Kernel-interface simulator (`repro/sysfake.c`): c uses b, both loaded, config `remove b echo ... >> file`; `modprobe -r c` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: `delete_module` for c and b, the command of b never ran |
| Same, patched | patch.diff, 5 runs | 5/5: only c is deleted, b's remove command runs |
| `-r -i c` with remove commands on both | patch.diff | c's command ignored, b's command runs, as documented for `-i` |
| `remove b /bin/false` | patch.diff | warning, exit 0 (dependency errors are non-fatal) |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes**, with the kernel side simulated.

## Class

logic

## Root cause

Step 4 bypasses the command lookup that the top-level path (`cmd = kmod_module_get_remove_commands(mod)`) performs.

## Introduced by

ea3c8ad "modprobe: Make rmmod_do_module() contain all the removal sequence" (the direct removal of unused dependencies dates from 0e9bd2d)

## The fix

`patch.diff` (+11/-2, tools/modprobe.c): for an unused dependency, run its `remove` command if it has one (non-fatally), otherwise unload it as before. Verified with the repro (5/5 exit 1 -> 5/5 exit 0), the variations above and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- When the requested module itself has a `remove` command, step 4 is skipped entirely (`if (!cmd)`), so its unused dependencies are neither removed nor given their commands; whether that is intended was not decided.
- The command replaces the unload, so nothing is deleted if the command does not do it itself (same as for the requested module).
