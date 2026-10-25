# modprobe-env-empty-overflow: empty `MODPROBE_OPTIONS` writes one byte past the argv buffer

`prepend_options_from_env()` (tools/modprobe.c) allocates `sizeof(char *) * (argc + space_count + 3 + envlen)` bytes and keeps a copy of the environment string, NUL included, at the end of the pointer array. For an empty (but set) `MODPROBE_OPTIONS` the string area is `envlen = 0` pointers wide, i.e. zero bytes, and `memcpy(str, env, envlen + 1)` writes the terminating NUL one byte past the allocation.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| libFuzzer `fuzz_modprobe_env` (testbed) | HEAD ASAN | heap-buffer-overflow at `modprobe.c:745` on the empty input |
| Real `MODPROBE_OPTIONS= modprobe -V` (`repro/`) | HEAD ASAN build, 10 runs | 10/10 ASAN `heap-buffer-overflow`, WRITE of size 1 in `__asan_memcpy` from `prepend_options_from_env` |
| Non-empty values (`a b`, `'a b' c`, `x  y`, `"q"`) | patched build | fine; the over-allocation `8 * envlen` bytes covers `envlen + 1` for any `envlen >= 1` |

## Status

**Confirmed** - deterministic, 10/10 on HEAD.

**Fully reproducible**: **Yes** - the real `modprobe` binary with an ordinary environment variable.

## Class

memory-safety (1-byte heap buffer overflow, NUL write)

## Root cause

The buffer size is computed in units of pointers for the string area (`envlen` pointers = `8 * envlen` bytes), which happens to be at least `envlen + 1` except when `envlen == 0`. glibc's malloc size rounding usually hides the extra byte, so it is silent in a normal build; ASAN (or an allocator with exact bounds) reports it.

## Introduced by

c3d0a5f "tools: add modprobe" - same allocation expression since the function was added.

## The fix

`patch.diff` (+1/-1, tools/modprobe.c): `malloc(sizeof(char *) * (argc + space_count + 3) + envlen + 1)` - pointer area plus exactly the string bytes needed. Verified: repro 10/10 crash -> 10/10 clean; non-empty values still produce the same `-V` output. Project test suite (ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- The tokenizer also differs from shell-like splitting in a few cases found by the testbed (`a  b` yields an empty argument, `'a b'c` splits into two arguments); these are behavior questions, not memory errors, and are not covered here (see `IDEAS.md`).
