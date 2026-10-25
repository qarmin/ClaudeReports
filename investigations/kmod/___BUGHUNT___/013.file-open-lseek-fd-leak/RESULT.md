# file-open-lseek-fd-leak: `kmod_file_open()` leaks the fd (and returns -EPERM) when the file is not seekable

`kmod_file_open()` (libkmod/libkmod-file.c) opens the file, probes seekability with `lseek(fd, 0, SEEK_CUR)` and on failure `return ret;` without closing `file->fd`. It also returns `ret`, which is the raw `-1` from `lseek()`, i.e. `-EPERM`, instead of `-errno` (`-ESPIPE`). The neighbouring `pread` failure path does `close(file->fd)`. Flagged by the clang static analyzer pass plus the earlier code-audit note on this function.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static review of `kmod_file_open()` (backlog item "lseek fd leak") | n/a | error path after `open()` lacks `close()` |
| Real `modinfo` on `/proc/self/fd/0` (a pipe), 100 args, `ulimit -n 40` | HEAD ASAN build | each arg reported as "Module ... not found": the tool treats a non-regular path as a module name, so the library path is not reachable from `modinfo` |
| Public API repro (`repro/repro.c`): `kmod_module_new_from_path()` + `kmod_module_get_info()` on `/proc/self/fd/<pipe read end>`, 100 iterations, counting `/proc/self/fd` entries | HEAD ASAN build of libkmod, 3 runs | 3/3: open fds 8 -> 108 (one leaked per call) |

## Status

**Confirmed** - deterministic, 3/3 on HEAD.

**Fully reproducible**: **Yes** - public libkmod API on a real pipe; there is no second participant. Not reachable from the stock `modinfo` binary, so the impact is limited to library users that open non-seekable paths (a daemon repeatedly querying such a path would run out of descriptors).

## Class

other (file descriptor leak and wrong errno on an error path)

## Root cause

Missing `close()` and `-errno` conversion on the `lseek()` failure branch added by the "unseekable files are unsupported" check.

## Introduced by

326b32c "libkmod: Fix open file which some char device"

## The fix

`patch.diff` (+4/-2, libkmod/libkmod-file.c): `if (lseek(...) < 0) { ret = -errno; close(file->fd); return ret; }`. Verified: repro 3/3 leak -> 3/3 no leak (fds 8 -> 8). Project test suite (ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- The `ENOMEM`-after-`calloc` path in the same function is fine (`_cleanup_free_`); the analyzer's "leak of file" report on the `open()` failure path is a false positive for that reason.
