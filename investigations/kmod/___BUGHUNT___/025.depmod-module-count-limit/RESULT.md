# depmod-module-count-limit: `assert(depmod->modules.count < UINT16_MAX)` with huge module trees

`depmod_calculate_dependencies()` (tools/depmod.c) keeps module indices and user counts in `uint16_t` and asserts `modules.count < UINT16_MAX`. If a tree could reach that count the assert would abort depmod.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real `depmod -b <root> 1.0` on 65534, 65535 and 70000 minimal synthetic modules (`repro/gen_tree.py`, no dependencies) | HEAD ASAN build, `memcap 8000`, 280 s kill | 65534: success in 3 s, all 11 index files written. 65535 and 70000: `depmod: FATAL: could not build module array: Numerical result out of range` (exit 1) after 1 s - the module array builder rejects the count before the assert can be reached |

## Status

**Not reproducible** - the limit is enforced with a clean error, the assert is unreachable through the CLI.

## Not done

- `users[]` overflow for one module depended on by more than 65535 others cannot occur given the count limit.
