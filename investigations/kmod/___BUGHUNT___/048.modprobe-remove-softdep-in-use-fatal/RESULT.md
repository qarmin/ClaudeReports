# modprobe-remove-softdep-in-use-fatal: `modprobe -r` dies with `FATAL: Module <softdep> is in use.` when a softdep module is still used elsewhere

`rmmod_do_module()` (tools/modprobe.c) removes a module's post-softdeps first and its pre-softdeps last, via `rmmod_do_modlist(list, false)` - "errors are non-fatal" according to the comment above step 5 and the `stop_on_errors = false` argument. But the "in use" message is printed with `LOG()`, which is `log_printf(log_priority, ...)` with `log_priority == LOG_CRIT` by default, and `log_printf()` calls `exit(EXIT_FAILURE)` for priorities at or above CRIT. So a softdep module that is still held by anything else terminates modprobe: for a pre-softdep it happens after the requested module was removed (the remaining arguments are never processed, exit status 1), for a post-softdep before the requested module is removed (it stays loaded). The `quiet_inuse` variable that would silence the message is never set. A shared softdep (a driver helper used by several modules) is the normal case, so `modprobe -r` of any module with a `softdep` on a shared module fails.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Kernel-interface simulator (`repro/sysfake.c`): `softdep c pre: a`, `a` held by loaded module `d`; `modprobe -r c e` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: `FATAL: Module a is in use.`, exit 1, only `c` removed, `e` untouched |
| Same with `softdep c post: a` | HEAD | `FATAL` before anything is removed, `c` stays loaded |
| Patched: softdep in-use message is a warning | patch.diff, 5 runs | 5/5: `WARNING: Module a is in use.`, exit 0, `c` and `e` removed, `a` and `d` kept |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes**, with the kernel side simulated (real modprobe binary and real syscalls into the fake).

## Class

logic

## Root cause

A fatal-priority log call in a code path designed to be non-fatal; `quiet_inuse` is dead.

## Introduced by

d8a6c0c "kmod-modprobe: mimic modprobe when removing deps with usecount=0"

## The fix

`patch.diff` (+11/-3, tools/modprobe.c): `rmmod_do_modlist()` lowers `log_priority` to `LOG_WARNING` for the duration of the call when `stop_on_errors` is false (the softdep callers), which is what `-a` already does globally, so every `LOG()` on that path (in use, not in kernel with `--first-time`, a failing `remove` command) becomes a warning instead of terminating modprobe. Holder removal (`stop_on_errors = true`) keeps the fatal messages. Verified with the repro (5/5 exit 1 -> 5/5 exit 0), the variations above (post-softdep in use; `--first-time -r c` with an unloaded pre-softdep; a pre-softdep whose remove command fails: all FATAL on HEAD, warnings now) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- Step 4 (unused hard dependencies) removes dependencies with `rmmod_do_remove_module()` directly, so a configured `remove` command for such a dependency is not run; not examined further.
- Exit status stays 0 when a softdep could not be removed, matching the "non-fatal" comment; a stricter status is a maintainer decision.
