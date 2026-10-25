# tools-stdout-write-error-ignored: modinfo, modprobe, depmod and the other tools exit 0 when their output could not be written

All tools print their results with `printf`/`puts`/`fputs` and never look at the stream state, so a failing stdout (disk full, closed pipe reader that is not killed by SIGPIPE, `/dev/full`, quota) goes unnoticed: the output is silently truncated or lost and the exit status is 0. Scripts that capture `modprobe -c`, `modinfo -F ...`, `modprobe -D` or `depmod -n` into a file cannot tell that the data is incomplete. This is the stdout counterpart of candidate 031 (`static-nodes -o`), which only covered its own output file.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `modinfo`, `modprobe -c`, `modprobe -D`, `depmod -n` on a one-module synthetic tree with stdout redirected to `/dev/full` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: all four exit 0, no message |
| `kmod list`, `modprobe -V`, `modinfo -h` the same way | HEAD, once | also exit 0 |
| Patched | patch.diff, 5 runs | 5/5: all four exit 1 with "error writing to stdout"; normal output still exit 0 (test suite) |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real binaries with a real failing stdout.

## Class

logic (error swallowed: wrong exit status)

## Root cause

No tool checks `ferror(stdout)`/`fflush(stdout)`/`fclose(stdout)`; `main()` in tools/kmod.c returns the command's status as is.

## Introduced by

not determined: the tools never checked their output stream since they were added (c3d0a5f "tools: add modprobe" is the oldest of them)

## The fix

`patch.diff` (+6/-0, tools/kmod.c): after the command returns, `fflush(stdout)` and `ferror(stdout)` are checked once in `main()` for every command (compat symlinks included) and a success status is turned into `EXIT_FAILURE` with a one-line message. Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- Stderr write failures and the tools' direct writes to files other than stdout (depmod's index files are checked, static-nodes is 031) are not covered here.
- Upstream may prefer `atexit(close_stdout)`-style behavior; the effect is the same.
