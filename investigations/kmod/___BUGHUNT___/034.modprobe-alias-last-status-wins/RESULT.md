# modprobe-alias-last-status-wins: when an alias resolves to several modules, modprobe's exit status is the status of the last one only

`insmod()` in tools/modprobe.c loops over the modules an alias/name resolved to and assigns `err = insmod_insert(...)` on every iteration. An insertion error of an earlier module is overwritten by a later success, so `modprobe foo` prints `could not insert 'm0'` and still exits 0, while with the two modules in the other order the same failure gives exit status 1. Callers (scripts, udev rules, systemd units) that rely on the exit status miss the failure. `rmmod_all()`/`insmod_all()` (several arguments) already keep any error; only the per-alias loop does not.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real `modprobe -d <root> -S 1.0 -C <conf> foo`, `alias foo m0`, `alias foo m1`, install command of one module `exit 3` and of the other `true` (`repro/run.sh`, modules from `synth.py`, indexes from the real depmod) | HEAD ASAN build, 5 runs | 5/5: failing m0 -> exit 0 (error message printed), failing m1 -> exit 1 |
| Same, patched | patch.diff, 5 runs | 5/5: exit 1 in both orders |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - real `modprobe` and a real `sh` running the install command; the failure is produced by the configured install command, so no root is needed. A failing `init_module` takes the same code path (`insmod_insert()` returns < 0) but was not exercised.

## Class

logic (failure reported as success depending on module order)

## Root cause

`err = insmod_insert(mod, flags, extra_options);` inside `kmod_list_foreach` overwrites the previous result.

## Introduced by

297a318 "modprobe: properly handle errors from init_module"

(the loop was later moved to `insmod()` by f3db15e; the overwrite was present in the original loop as well)

## The fix

`patch.diff` (+5/-1, tools/modprobe.c): remember the last negative result in the loop. Verified with the repro (5/5 exit 1 -> 5/5 exit 0). Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- Whether upstream deliberately wants "success if any module of the alias loaded" semantics is unknown; the error message is printed either way, which suggests the exit status was simply overlooked.
- `rmmod()` (the `-r` loop) instead stops at the first error (`break`), a different convention; not changed.
- The same overwrite pattern exists in `show_modversions()` and `show_exports()` (`err = print_modversions(mod)` / `print_exports(mod)` per list entry, only the last status is returned); not changed in `patch.diff` and not reproduced separately.
