# softdep-missing-separator: `modprobe -c` prints softdep `pre:` and `post:` lists glued together

`softdep_to_char()` (libkmod/libkmod-config.c) builds the textual form of a softdep for `kmod_config_iter` / `modprobe -c` / `--show-config`. When an entry has both a `pre:` and a `post:` list, nothing separates the last pre module from the `post:` keyword, so a config line `softdep x pre: p1 p2 post: q1 q2` is printed as `softdep x pre: p1 p2post: q1 q2`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Model test `config_model` in the testbed (random configs vs expected `modprobe -c` lines) | HEAD ASAN build | signature `model:modprobe-c-softdep`: every entry with both lists is printed glued |
| Real `modprobe -C <dir> -c` on `softdep x pre: p1 p2 post: q1 q2` (`repro/`) | HEAD ASAN build, 10 runs | 10/10 prints `pre: p1 p2post: q1 q2`; feeding that output back in as a config reproduces the same glued line (the parser reads `p2post:` as a module name and `post:` is lost) |
| Variants `pre: a`, `post: b`, `pre: a b c post: d`, `pre: a post: b c d e` | patched build | only entries with both lists were affected; all print correctly after the fix |

## Status

**Confirmed** - deterministic, 10/10 on HEAD.

**Fully reproducible**: **Yes** - real `modprobe` binary on a config file.

## Class

logic (wrong output of `modprobe -c`; the printed config is not equivalent to the input)

## Root cause

The pre list is copied with `memcpy(..., sz_pre + 1)` (including the terminating NUL) and the write cursor is then left on that NUL (`itr = p`, the loop stops at `itr + sz_pre`). The `post: ` memcpy overwrites it, so no space is emitted between the lists, and the size computation reserves no byte for it either. The weakdep variant has no second list and is unaffected.

## Introduced by

6b04ef3 "config: let softdeps dump their data" - the function was introduced with this behaviour.

## The fix

`patch.diff` (+4/-0, libkmod/libkmod-config.c): reserve one extra byte when both lists exist and write a space after the pre list. The `malloc` size and the final NUL stay consistent (checked by reading the arithmetic and by running the ASAN build on the four variants above).

Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- Only `modprobe -c` was driven end to end; `kmod_config_get_softdeps()` consumers in libkmod proper (`kmod_module_get_softdeps()`) read the parsed lists and do not go through `softdep_to_char()`.
