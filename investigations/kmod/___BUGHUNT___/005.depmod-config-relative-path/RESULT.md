# depmod-config-relative-path: `depmod -C <file>` with a slash-less file name underflows the directory length

`cfg_files_insert_sorted()` (tools/depmod.c) derives the directory length of a `-C` config file as `strlen(dir) - strlen(basename(dir)) - 1`. When the path has no `/` (`depmod -C t.conf`), `basename()` returns the whole string, so `dirlen` underflows to `SIZE_MAX` and `memcpy(f->path, dir, dirlen)` is called with size -1.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cli_matrix` testbed component, `depmod -C t.conf` with cwd holding the file | HEAD ASAN+UBSAN build | `depmod.c:846: addition of unsigned offset ... overflowed` (UBSAN halts) |
| Real `depmod -b <root> -C t.conf 1.0` (`repro/`), cwd = dir with `t.conf` | HEAD ASAN+UBSAN build, 10 runs | 10/10 abort. With UBSAN recover off: ASAN `negative-size-param: (size=-1)` in `__asan_memcpy` from `cfg_files_insert_sorted:847` |
| Same command, non-sanitized gcc build (`testbed/__build-gcc`) | `-D_FORTIFY_SOURCE` default of the distro toolchain | `*** buffer overflow detected ***: terminated`, SIGABRT (exit 134) |
| `./t.conf` and absolute paths | HEAD build | work: only a path without any `/` is affected |

## Status

**Confirmed** - deterministic, 10/10 on HEAD (sanitized) and also with the plain gcc build.

**Fully reproducible**: **Yes** - the real `depmod` binary driven by its documented `-C` option.

## Class

memory-safety (size_t underflow -> `memcpy` with size -1; crash / would-be heap overflow in an unfortified build)

## Root cause

`malloc(sizeof(*f) + dirlen + namelen + 2)` wraps consistently with the underflowed `dirlen`, so the allocation is small and the following `memcpy(f->path, dir, dirlen)` copies `SIZE_MAX` bytes. Without `_FORTIFY_SOURCE`/ASAN this is a heap overflow with an enormous length (segfault in practice).

## Introduced by

64b8b58 "kmod-depmod: initial code (no files generated, untested)" - the `basename()`-based dirlen computation is there from the start (8ea02fe later corrected the off-by-one for paths that do contain a slash, leaving the slash-less case).

## The fix

`patch.diff` (+5/-1, tools/depmod.c): when `basename()` returns the input itself (no directory part), use `"."` as the directory. Verified: repro 10/10 crash -> 10/10 clean (depmod only warns about the missing `modules.order` etc. in the empty tree). Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- Only the `-C <file>` path was exercised; `-C <dir>` goes through `cfg_files_list()` with an explicit `name` and is not affected.
