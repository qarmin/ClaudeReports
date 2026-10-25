# depmod-truncated-output-published: a truncated modules.* file replaces the previous good one before the write error is checked

`depmod_output()` (tools/depmod.c) writes each index file to a temporary file, then calls `ferror(fp) | fclose(fp)`, then `tmpfile_publish()` (a `renameat()` over the real name), and only afterwards looks at the saved write error (`if (ferr)`). When the writes fail (disk full, quota, file size limit), the half-written temporary file is therefore renamed over the existing, complete `modules.dep` first, `depmod` reports "Output is truncated" and exits 1, but the previous good index is already gone. The `r < 0` branch above it correctly calls `tmpfile_release()` instead of publishing; the `ferr` check is simply on the wrong side of the publish. The rest of the `modules.*` set (e.g. `modules.dep.bin`) is left from the old run, so the files also disagree with each other.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real `depmod -b <root> 1.0` over a tree of 60 synthetic modules (62280-byte `modules.dep`), then a second run with `ulimit -f 2` and SIGXFSZ ignored so that writes fail with EFBIG (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: second run exits 1 with "Output is truncated" and `modules.dep` is a 2048-byte truncated file |
| Same, patched | patch.diff, 5 runs | 5/5: same error and exit status 1, `modules.dep` keeps its 62280 bytes, no temporary file left |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real depmod binary and real write failures (`RLIMIT_FSIZE`); ENOSPC on a full filesystem takes the same path.

## Class

logic (data loss on error path)

## Root cause

Order of operations in the per-file loop: publish first, check `ferr` second.

## Introduced by

aae48bc "depmod: add tmpfile-util to generate temporary file"

(the `ferror`/`fclose` check itself is older, 3f376cd; the rename-before-check order came with the tmpfile helper)

## The fix

`patch.diff` (+6/-5, tools/depmod.c): test `ferr` before `tmpfile_publish()` and `tmpfile_release()` the temporary file, like the `r < 0` branch. Verified with the repro (5/5 exit 1 -> 5/5 exit 0). Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- Earlier files of the same run (published before the failing one) stay replaced, so a failure in the middle still leaves a mix of old and new indexes; a transactional publish of the whole set would be a larger change.
- The message always says ENOSPC even when the cause is EFBIG/EDQUOT.
