# static-nodes-write-error-ignored: `kmod static-nodes` exits 0 when it cannot write its output

`do_static_nodes()` (tools/static-nodes.c) ignores the return value of `format->write()`, never checks `ferror()` and ignores the result of the final `fclose()`. Output is buffered, so a write failure (disk full, quota, `-o` on a full or read-only device) only shows up when the stream is flushed at `fclose()`, and that is not looked at. The tool prints nothing and exits 0 although the file is empty or truncated. `kmod static-nodes -f tmpfiles -o <file>` is what packages and units run to produce the tmpfiles.d snippet for statically created device nodes, so a truncated snippet is silently accepted.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `do_static_nodes()` | n/a | `format->write()` result dropped, `fclose(out)` unchecked |
| Real `kmod static-nodes -f tmpfiles -o /dev/full` (`repro/run.sh`), build with `-Dmoduledir=<scratch>` and a two-line `modules.devname` for the running kernel | HEAD ASAN build, 5 runs | 5/5: no message, exit code 0 |
| Same with output on stdout redirected to `/dev/full` | HEAD | exit code 0 as well |
| Same, patched | patch.diff applied, 5 runs | 5/5: `Error: could not write output - No space left on device`, exit code 1; normal `-o file` still exits 0 |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real `kmod` binary writing to a real device that always fails.

## Class

logic (error swallowed: wrong exit status)

## Root cause

The writers return `EXIT_FAILURE` on a failed `fprintf()`, but the caller discards it; the final `fclose(out)` (where buffered data is actually written) is not checked either. Only the invalid-entry path sets `ret`.

## Introduced by

db6f2fc "tools: add static-nodes tool"

## The fix

`patch.diff` (+8/-3, tools/static-nodes.c): propagate the writer's return value, and on close check `ferror(out) || fclose(out) != 0`, printing `Error: could not write output - <errno>` and exiting 1. Verified with the repro (5/5 exit 0 -> 5/5 exit 1) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after (the suite does not exercise `static-nodes`).

## What's not done / open questions

- The same pattern (unchecked `fclose`/`fflush`) was not audited in the other tools; `depmod -n` stdout writers are already a backlog item.
- `fclose(stdout)` is used when no `-o` is given; harmless here, kept as is.
