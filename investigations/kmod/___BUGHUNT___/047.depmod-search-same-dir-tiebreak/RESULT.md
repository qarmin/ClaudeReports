# depmod-search-same-dir-tiebreak: for two same-named modules in the same search directory, the winner depends on where `built-in` sits in the `search` list

`depmod_module_is_higher_priority()` (tools/depmod.c) computes a priority for the new and the existing module from the `search` list, but the two checks are an `else if` chain: when the new path matches a search entry, the old path is not tested against that entry. If both modules live below the same entry (`updates/a/foo.ko` and `updates/b/foo.ko`) the existing module therefore gets no priority of its own and falls back to the priority of `built-in`, so the comparison is "new module in `updates`" against "`built-in`" instead of "new" against "old". Result: with `search updates built-in` (the default) the module scanned last replaces the earlier one, with `search built-in updates` the first one is kept. Two candidates of equal priority are resolved differently depending on the relative order of an unrelated keyword.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `updates/a/foo.ko` and `updates/b/foo.ko`, depmod with `search updates built-in` and `search built-in updates` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: `updates/b/foo.ko` vs `updates/a/foo.ko` |
| Same, patched (independent checks for new and old) | patch.diff, 5 runs | 5/5: `updates/a/foo.ko` in both configurations |
| Different-tier cases (`search extra built-in`, `override`, `search weird`, ...) | HEAD | behave as documented; only ties are affected |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real depmod binary and a real directory tree.

## Class

logic

## Root cause

`else if` between the new-path and old-path tests inside the `search` loop.

## Introduced by

7da6884 "depmod: implement external directories support"

## The fix

`patch.diff` (+8/-6, tools/depmod.c): test the new and the old path independently. Equal priorities then always keep the module found first, in every configuration. Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- Which module is "found first" is `readdir()` order of the module tree (not sorted), so the choice among equal-priority duplicates is still file system dependent; sorting directory entries would make `modules.dep` reproducible but is a bigger behavior change.
