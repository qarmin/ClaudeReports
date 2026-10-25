# modprobe-remove-holders-not-transitive: `modprobe -r --remove-holders a` fails when a holder of `a` is itself held by another module

modprobe(8): "--remove-holders: remove the mentioned module(s), alongside other modules that depend on it/them - aka the reverse dependency list." `rmmod_do_module()` implements this by calling `rmmod_do_modlist(holders, true)`, but `rmmod_do_modlist()` calls `rmmod_do_module(m, RMMOD_FLAG_IGNORE_BUILTIN)`: the `RMMOD_FLAG_REMOVE_HOLDERS` flag is dropped, so only the direct holders are considered. With `c` using `b` using `a`, `b` is still held by `c` when it is processed and modprobe aborts with `FATAL: Module b is in use.` leaving all three loaded, instead of removing `c`, `b`, `a`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Kernel-interface simulator (`repro/sysfake.c`, LD_PRELOAD for `/sys/module`, `/proc/modules`, `delete_module`), chain a <- b <- c all loaded, `modprobe -r --remove-holders a` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: `Module b is in use.`, nothing removed, exit 1 |
| Same, patched | patch.diff, 5 runs | 5/5: c, b, a removed in that order |
| Existing behavior for a direct holder only (a <- b) | HEAD | works (b removed, then a) |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes**, with the kernel side simulated (real modprobe binary, real `delete_module`/sysfs reads against the fake).

## Class

logic

## Root cause

`rmmod_do_modlist()` hardcodes the flags passed to the recursive `rmmod_do_module()` call.

## Introduced by

42b32d3 "modprobe: Fix holders removal"

## The fix

`patch.diff` (+5/-5, tools/modprobe.c): `rmmod_do_modlist()` gets a `flags` parameter; the holders call passes `RMMOD_FLAG_REMOVE_HOLDERS` so the recursion continues up the holder chain, the softdep calls pass 0. Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- A module with a cycle in its holders (not possible in a real kernel) would recurse; not guarded.
- Interaction with 045's reverse walk was checked only on chains.
