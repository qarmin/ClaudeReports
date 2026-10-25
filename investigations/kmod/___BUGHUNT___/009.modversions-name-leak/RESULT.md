# modversions-name-leak: `kmod_elf_get_modversions()` leaks its result array on an over-long symbol name

`kmod_elf_get_modversions()` (libkmod/libkmod-elf.c) mallocs the result array, stores it in `*array`, and then returns `-EINVAL` from inside the loop when a `__versions` entry's name is not NUL-terminated within its field. The only caller, `kmod_module_get_versions()`, returns on `count < 0` without freeing `versions`, so the array is leaked.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `elf_sweep` testbed (structured mutations of real modules) with LeakSanitizer | HEAD ASAN | `asan:leak:kmod_elf_get_modversions@libkmod-elf.c:559` on `__versions.offset=7f` |
| Real `modprobe --show-modversions <bad .ko>` (`repro/`) | HEAD ASAN build, `detect_leaks=1`, 10 runs | 10/10 LeakSanitizer "Direct leak of 216 byte(s)" from `kmod_elf_get_modversions` <- `kmod_module_get_versions` <- `print_modversions` |

## Status

**Confirmed** - deterministic, 10/10 on HEAD (the run exits 1 either way because modprobe reports the invalid module; the repro script checks for the leak report).

**Fully reproducible**: **Yes** - the real `modprobe` binary on a crafted `.ko`.

## Class

memory leak (resource leak on an error path; matters to long-lived libkmod users that repeatedly query bad modules)

## Root cause

The `return -EINVAL` added with the symbol-name validation skips the cleanup; the header comment documents "just free *array" but `*array` is also set on the failing path and the caller treats a negative count as "no array". The sibling error path for `-ENOMEM` returns before `*array` is set, so only this return leaks.

## Introduced by

0930e7b "libkmod: Validate symbol names in ELF files"

## The fix

`patch.diff` (+2/-0, libkmod/libkmod-elf.c): `free(a); *array = NULL;` before the `-EINVAL` return. Verified: repro 10/10 leak -> 10/10 no leak. Project test suite (ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- Other `kmod_elf_get_*` getters (`get_symbols`, `get_dependency_symbols`) were not audited for the same missing-free pattern in this candidate.
