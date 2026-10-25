# softdep-pre-hard-dependency-duplicate: a module that is both a hard dependency and a `softdep pre:` of the requested module is probed twice

`__kmod_module_get_probe_list()` (libkmod/libkmod-module.c) walks the hard dependencies of the requested module and calls `__kmod_module_fill_softdep()` on each of them, but unlike the recursive path it never sets `m->visited` on them. When the config also lists one of those dependencies as a `softdep ... pre:` of the requested module (or of one of its dependencies), the softdep walk goes through `__kmod_module_get_probe_list()`, finds `visited == false` and appends the module (and its dependencies) a second time. `modprobe` then processes the module twice: `modprobe -n -v` prints it twice, and an `install` command configured for that module is executed twice (for a module without an `install` command the second `insmod` is turned into a harmless `-EEXIST`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Hand-built tree of three synthetic modules, m2 -> m1 -> m0 (hard deps via symbols, indexes by real `depmod`), config `softdep m2 pre: m0`, `modprobe -d <root> -S 1.0 -C <conf> -n -v m2` | HEAD ASAN build | output `m0, m1, m0, m2` instead of `m0, m1, m2` |
| Same with `install m0 echo m0 >> log` and `install m1 echo m1 >> log` (no root needed), real `modprobe m2` (`repro/run.sh`) | HEAD, 5 runs | 5/5: the install command of m0 ran twice (`log` has `m0` twice) |
| Variant `softdep m2 pre: m1` + `softdep m1 pre: m3` | HEAD | `m0, m3, m1, m0, m1, m2`: both m0 and m1 duplicated |
| Patched | patch.diff, 5 runs | 5/5: each module once, install command ran once |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real `modprobe` binary on a real module tree (indexes produced by the real `depmod`), observing the real side effect of the install command. The final `insmod` is not reached (needs root and a kernel); the duplicate happens before it.

## Class

logic (module probed twice, install command run twice)

## Root cause

`m->visited` guards against processing a module twice, but it is only set on entry to `__kmod_module_get_probe_list()`. The hard dependencies of the requested module are handled in the loop over `dep` by calling `__kmod_module_fill_softdep()` directly, so they are never marked.

## Introduced by

b1a5125 "libkmod-module: probe: change insertion to cover more use cases"

## The fix

`patch.diff` (+5/-0, libkmod/libkmod-module.c): in the `dep` loop skip modules that are already visited and mark the others before filling their softdeps. Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and both variants above (each module once, dependency order kept). Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- Whether upstream considers a softdep on a hard dependency a supported configuration was not checked; the dedupe logic (`visited`) shows the intent to never process a module twice.
- Removal (`modprobe -r`) uses a different walk and was not examined.
