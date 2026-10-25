# depmod-symlink-loop-stack-overflow: depmod recurses forever into a directory symlink that points to an ancestor

`depmod_modules_search_dir()` (tools/depmod.c) follows directory symlinks (it classifies entries with `fstatat(..., 0)`, which resolves them) and recurses with `openat(dfd, name)`, which is relative to the parent descriptor and therefore has no `PATH_MAX` limit. A symlink that points to one of its ancestors (`kernel/loop -> .`) makes the recursion unbounded: depth is limited only by the file descriptor limit or the stack. With `ulimit -n 65536` the stack (8 MB) overflows first and `depmod` dies with SIGSEGV before writing any index file. Upstream clearly knows about symlink loops (`build` and `source` are excluded by name, `should_exclude_dir()`), and the sibling walker `depfile_up_to_date_dir()` (used by `depmod -A`) is bounded by a `PATH_MAX` check; this walker is not.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `depmod -b <root> 1.0` on a tree with one module and `kernel/loop -> .` (`repro/run.sh`) | HEAD plain (non-ASAN) build, `ulimit -n 65536`, 5 runs | 5/5: SIGSEGV (exit 139), no `modules.dep` |
| Same | HEAD ASAN build | ASAN `stack-overflow` in `depmod_modules_search_dir` recursion (line 1403) |
| Patched, same tree | patch.diff, 5 runs | 5/5: `directory loop at ...` reported, exit 0, `modules.dep` written |
| Patched: two-level loop (`a/b/up -> ../..`) plus a legitimate symlink to a sibling directory (`c/sib -> ../a`) | patch.diff | both loops reported, the legitimate symlink still walked, indexes written |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`; the before runs use the non-ASAN build, the after runs the ASAN build).

**Fully reproducible**: **Yes** - the real depmod binary on a real directory tree. With a default `ulimit -n` of 1024 the descriptor limit is reached first (about 1000 levels, each reports `Too many open files`), so the crash needs a higher limit (containers and systemd services commonly have 65536 or more); a lower limit just burns time and memory.

## Class

crash (unbounded recursion)

## Root cause

No cycle detection in the directory walk; the walk is bounded neither by depth nor by path length.

## Introduced by

64b8b58 "kmod-depmod: initial code (no files generated, untested)"

(the recursive walk exists since the initial depmod; best-effort, not bisected)

## The fix

`patch.diff` (+23/-3, tools/depmod.c): carry a chain of `(st_dev, st_ino)` of the directories being walked and skip (with an error message) a directory that is already on the chain. Verified with the repro (5/5 exit 1 -> 5/5 exit 0), the two variations above, and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- Whether upstream wants symlinked directories followed at all was not checked; the fix keeps that behavior.
- The same walk exists for `depmod -A` (`depfile_up_to_date_dir()`), which is bounded by the path length check and was not changed.
- Module files reachable through two different symlinked paths are still discovered twice; depmod's own de-duplication behavior was not examined.
