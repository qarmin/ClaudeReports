# lsmod-negative-errno-used-by: lsmod prints a raw negative errno in the "Used by" column when the module has no refcnt file

`do_lsmod()` (tools/lsmod.c) prints `kmod_module_get_refcnt(mod)` with `%d`. That function returns `-ENOENT` (-2) when `/sys/module/<name>/refcnt` does not exist, which is the normal situation on kernels built without module unloading support (CONFIG_MODULE_UNLOAD off, common on embedded systems), so every module is listed with "Used by" = `-2`. lsmod(8) describes the tool as formatting `/proc/modules`, where such modules show `-` in the usage fields. (`size` is printed the same way from `kmod_module_get_size()`, which can also return a negative errno.)

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Kernel-interface simulator (`repro/sysfake.c`): module `a` without `refcnt`, module `b` with `refcnt` 0; `kmod list` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: `a  4096  -2` |
| Same, patched (`-` for a negative count) | patch.diff, 5 runs | 5/5: `a  4096  -`, `b  4096  0` |
| Real host (kernel with unloading support): `kmod list` vs `/proc/modules` | HEAD | identical (131 modules), so the bug only shows on kernels without refcnt files |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`); needs a kernel without `refcnt` files or the simulator.

**Fully reproducible**: **Yes** with the simulated sysfs.

## Class

logic (error code leaked into normal output)

## Root cause

The return value of `kmod_module_get_refcnt()` is used without checking for an error.

## Introduced by

72c51a9 "add lsmod, insmod and rmmod tools."

## The fix

`patch.diff` (+5/-1, tools/lsmod.c): print `-` instead of a negative count. Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- The size column has the same pattern but needs a module with a directory in /sys and neither `coresize` nor a /proc/modules entry, which was not simulated.
