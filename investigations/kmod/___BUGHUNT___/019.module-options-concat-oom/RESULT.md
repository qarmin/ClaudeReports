# module-options-concat-oom: `module_options_concat()` dereferences an unchecked `malloc()` result

`module_options_concat()` (libkmod/libkmod-module.c) `malloc()`s the combined option string and immediately `memcpy()`s into it. When the allocation fails, `modprobe` (via `kmod_module_probe_insert_module()`) segfaults. Upstream PR #459 reports and fixes the same line; this candidate reproduces it with the real binary.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| New allocation-failure sweep (`tools/failmalloc.c` LD_PRELOAD shim + `tools/failmalloc_sweep.sh`): fail the n-th `malloc/calloc/realloc/strdup` of one run, for every n | plain (non-ASAN) debug builds of HEAD; targets `modprobe -c` (config parsing), `depmod`, `modprobe -n -v foo` with softdep+options, `modprobe --show-depends`, `modprobe -c -d`, `modinfo` | only `modprobe -n -v` and `modprobe --show-depends` crashed, both at the same line (allocation #99/#94 of 190/180): SIGSEGV in `memcpy` called from `module_options_concat` |
| `repro/run.sh`: builds HEAD plain, creates a tiny tree (`foo` with softdep on `bar`, `options foo x=1`), sweeps all 166 allocations of `modprobe -n -v -C etc -d root -S 1.0 foo` | 1 run | allocation #87 -> killed by signal 11 (deterministic: the count is fixed for the input) |

## Status

**Confirmed** - deterministic (same allocation index every run of the same tree), reproduced on HEAD with the real `modprobe`; the other targets are clean under allocation failure.

**Fully reproducible**: **Yes** - real `modprobe`; the injected condition is an out-of-memory `malloc`, produced by an `LD_PRELOAD` shim rather than by exhausting memory.

## Class

crash (NULL dereference on allocation failure)

## Root cause

The result of `malloc(optlen + xoptlen + 2)` is used without a NULL check. The function's callers (`kmod_module_probe_insert_module()` -> `kmod_module_insert_module()` / install commands) already accept a NULL options pointer ("no options").

## Introduced by

ddbda02 "Add implementation of modprobe's insertion"

## The fix

`patch.diff` (+2/-0, libkmod/libkmod-module.c): return NULL when `malloc` fails, matching upstream PR #459's choice (the module is then inserted without options rather than crashing; reporting the error to the caller would be a larger change). Verified: sweep 1 crash -> 0 crashes over all 166 allocations; project test suite (ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- Only five command lines were swept (about 1000 injected failures in total); `insmod`/`rmmod`/`lsmod` need a kernel and were not covered. Sweeping `kmod_module_get_info`/`depmod` on a larger tree is a natural testbed addition.
- Silently dropping options on OOM may be undesirable; upstream's contract note in PR #459 argues it is acceptable.
