# depmod-output-failure-exit-status: depmod reports success when it cannot create any output file

`depmod_output()` (tools/depmod.c) loops over the index files; when `tmpfile_openat()` fails it logs "Could not create temporary file" and `continue`s without recording an error. `err` stays 0, so `depmod` finishes with exit status 0 although none of the `modules.*` files was written (read-only filesystem, no permission, quota). Kernel package scripts that run `depmod -a` and rely on its exit status would then believe the module indexes are fresh.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `depmod_output()` after an exploratory run of `depmod -b <root>` on an unwritable directory | n/a | the `continue` on `tmpfile_openat() == NULL` swallows the error |
| Real `depmod -b <root> 1.0` with `lib/modules/1.0` made read-only (`chmod a-w`), one synthetic module, as a non-root user (`repro/`) | HEAD ASAN build, 3 runs | 3/3: 11 "Could not create temporary file" errors, exit status 0, 0 index files written |
| Same, `-o /nonexistent/x` | HEAD | fails correctly (exit 1, `could not create directory`) - only the per-file open path is affected |

## Status

**Confirmed** - deterministic, 3/3 on HEAD (requires a non-root user so that the permission check applies; a read-only filesystem or full disk would behave the same for root).

**Fully reproducible**: **Yes** - real `depmod` binary and a real unwritable directory.

## Class

logic (error swallowed: wrong exit status)

## Root cause

The failure branch uses `continue` and never sets `err`; the other failure branches in the loop (`r < 0`, publish, truncated output) set `err` and `break`.

## Introduced by

aae48bc "depmod: add tmpfile-util to generate temporary file"

## The fix

`patch.diff` (+4/-2, tools/depmod.c): set `err` (from `errno`, falling back to `-EIO`), include the reason in the message and `break`, like the neighbouring branches. Verified: repro 3/3 exit 0 -> 3/3 exit 1 with one "Permission denied" error; project test suite (ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- With `depmod -n` (output to stdout) `r < 0` from the writers is also ignored (`if (fp == out) continue;`); not exercised.
- Upstream may prefer to keep trying the remaining files before failing; the patch fails fast.
