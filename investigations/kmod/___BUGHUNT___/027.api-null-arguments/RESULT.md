# api-null-arguments: every public libkmod function called with NULL/zero arguments

libkmod's header documents NULL-tolerant behavior for most entry points (`return -ENOENT`/NULL for a NULL context or module). A missing guard is a classic crash for library users.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `tools/api_null_gen.py` generates a driver calling each of the 74 prototypes from `libkmod.h` in a forked child (a) with all arguments NULL/0, (b) with a valid `kmod_ctx` as first argument and the rest NULL/0, (c) with a valid `kmod_module` (from a synthetic `.ko`) as first argument and the rest NULL/0; skips `kmod_set_log_fn` and `kmod_module_probe_insert_module` (callback signatures) | HEAD ASAN+UBSAN libkmod, `memcap 3000` | no signal, no ASAN/UBSAN report from any child |

## Status

**Not reproducible** - public API is robust against NULL/zero arguments in these three shapes.

## Not done

- List-typed arguments (`struct kmod_list *`) were only passed as NULL; random non-NULL garbage pointers are out of contract.
- The two skipped functions take callbacks and were not driven.
