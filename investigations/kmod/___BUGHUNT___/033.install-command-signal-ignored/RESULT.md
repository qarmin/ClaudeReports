# install-command-signal-ignored: an `install`/`remove` command killed by a signal is treated as success

Both `command_do()` copies (libkmod/libkmod-module.c for the library's probe/insert path, tools/modprobe.c for `modprobe` install/remove commands) run the configured command with `system()` and only look at `WEXITSTATUS(ret)`. For a command that was terminated by a signal `WEXITSTATUS` is 0, so the failure is silently ignored: `modprobe` carries on (for `install` it never inserts the module, for `remove` it never removes it), prints nothing and exits 0.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real `modprobe -d <root> -S 1.0 -C <conf> m0` with `install m0 kill -9 $$` and `remove m0 kill -9 $$` (`-r`), control `exit 3` | HEAD ASAN build, 5 runs (`repro/run.sh`) | 5/5: `exit 3` gives exit status 1, the killed command gives exit status 0 and no message, for both install and remove |
| Same, patched | patch.diff, 5 runs | 5/5: exit status 1 for all four |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real `modprobe` binary and a real `sh` child; no fake participant. The module insertion itself (needs root) is not reached, the wrong status is determined before it.

## Class

logic (command failure reported as success)

## Root cause

`system()` returns a wait status; the exit code is only meaningful if `WIFEXITED`. For a signalled child `WEXITSTATUS()` is 0. The same assumption is in both copies of `command_do()`.

## Introduced by

ddbda02 "Add implementation of modprobe's insertion"

## The fix

`patch.diff` (+10/-1): check `WIFSIGNALED()` first and report failure (-EINVAL) with the signal number in both places. Verified with the repro (5/5 exit 1 -> 5/5 exit 0). Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- `system()` ignores SIGINT/SIGQUIT in the parent while the child runs, so Ctrl-C kills only the install command; with this fix modprobe then reports an error instead of continuing, which was not further examined.
- Other `system()`/`WEXITSTATUS` users were grepped: only these two.
