# libkmod-probe-blacklist-all-doc: KMOD_PROBE_APPLY_BLACKLIST_ALL silently drops blacklisted dependencies instead of returning early

libkmod.h documents the flag as: "prior to probe, apply KMOD_FILTER_BLACKLIST filter to this module and its dependencies. If any of them are blacklisted and the blacklisted module is not live in the kernel, the function returns early with this enum". `kmod_module_probe_insert_module()` applies `kmod_module_apply_filter(KMOD_FILTER_BLACKLIST)` to the probe list and returns early only when the filtered list is *empty*. When some entries are blacklisted the filter just removes them, and the probe continues with the rest: a module whose hard dependency is blacklisted is probed without that dependency (the kernel would then reject it with an unresolved symbol), and a blacklisted dependency that is already live is also dropped from the list although the doc says that case is fine. The flag is not used by the modprobe binary (it uses `KMOD_PROBE_APPLY_BLACKLIST` for the named module only), so this affects API users.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| C program using the public API: `m` depends on `d`, config `blacklist d`, `kmod_module_probe_insert_module(m, DRY_RUN\|IGNORE_LOADED\|APPLY_BLACKLIST_ALL, ...)` with a print callback (`repro/repro.c`, `repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: returns 0 and reports one action (`m`), no early return |
| Same, patched (return the enum when a probed module is blacklisted and not live) | patch.diff, 5 runs | 5/5: returns 65536 (`KMOD_PROBE_APPLY_BLACKLIST_ALL`), no action |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - public API of the built libkmod.

## Class

logic (behavior differs from the documented contract)

## Root cause

The implementation reuses the list filter and only checks for an empty result.

## Introduced by

b1a5125 "libkmod-module: probe: change insertion to cover more use cases"

## The fix

`patch.diff` (+7/-8, libkmod/libkmod-module.c): walk the probe list; if a module is blacklisted and not live, free the list and return `KMOD_PROBE_APPLY_BLACKLIST_ALL`; otherwise leave the list unchanged. Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after. Alternative: change the documentation to say that blacklisted modules are skipped.

## What's not done / open questions

- Existing API users relying on the "skip blacklisted entries" behavior would see a behavior change; the doc is the only specification.
