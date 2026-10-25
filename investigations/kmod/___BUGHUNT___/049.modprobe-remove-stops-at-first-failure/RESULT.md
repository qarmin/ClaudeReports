# modprobe-remove-stops-at-first-failure: `modprobe -r a b` stops at the first module that cannot be removed

modprobe(8) allows several modules with `-r`, and `rmmod_all()` is written as a loop that records the error of each argument and continues (`if (r < 0) err = r;`, final status is the last error). But the failures it handles are reported with `LOG()` = `log_printf(log_priority, ...)`, which exits the process at the default `LOG_CRIT` priority (only `-a` lowers it to `LOG_WARNING`). `modprobe -r a e` with `a` in use (or unknown, or - with `--first-time` - not loaded) therefore prints `FATAL: Module a is in use.` and exits; `e` is never touched. rmmod(8), by contrast, documents "log an error AND continue with the next module".

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Kernel-interface simulator (`repro/sysfake.c`): `a` held by loaded `d`, `e` free; `modprobe -r a e` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: `FATAL: Module a is in use.`, exit 1, no `delete_module` call, `e` still loaded |
| `modprobe -r zz e` (zz unknown), `--first-time -r d e` (d not loaded) | HEAD | both stop at the first argument with `FATAL`, `e` untouched |
| Patched | patch.diff, 5 runs | 5/5: warning for `a`, `e` removed, exit status 1 (a failure happened) |
| Single argument `-r zz` | patch.diff | unchanged (`FATAL: Module zz not found.`, exit 1) |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes**, with the kernel side simulated.

## Class

logic

## Root cause

Fatal-by-priority `LOG()` inside a loop designed to continue after errors.

## Introduced by

c3d0a5f "tools: add modprobe"

## The fix

`patch.diff` (+8/-0, tools/modprobe.c): `rmmod_all()` lowers `log_priority` to `LOG_WARNING` while it iterates when more than one module was given (the same switch `-a` performs for insertion), restoring it afterwards. Exit status is still non-zero if any removal failed. Verified with the repro (5/5 exit 1 -> 5/5 exit 0), the unknown-module and `--first-time` variations, and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- Whether upstream prefers "stop at first failure" for `-r` is not documented; the loop structure and the rmmod(8) wording suggest continuing.
- Messages change from `FATAL:` to `WARNING:` for multi-module invocations.
