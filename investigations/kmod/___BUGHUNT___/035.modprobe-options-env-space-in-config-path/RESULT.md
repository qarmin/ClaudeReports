# modprobe-options-env-space-in-config-path: `modprobe -C <path with a space>` hands child modprobe processes a broken MODPROBE_OPTIONS

`do_modprobe()` records every `-C <path>` in `$MODPROBE_OPTIONS` so that modprobe processes started by `install`/`remove` commands (the usual `install foo /sbin/modprobe --ignore-install foo`) use the same configuration. The value is appended unquoted, and the tokenizer in `prepend_options_from_env()` splits on spaces, so `-C /x/my conf` becomes `-C /x/my` plus a stray `conf`. The child reads the wrong (nonexistent) config directory and does not see the configuration of its parent.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real `modprobe -d <root> -S 1.0 -C "<root>/my conf" m0` with an `install m0` command that prints `$MODPROBE_OPTIONS` and runs `modprobe -c` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: env is `-C ROOT/my conf`, the child's `-c` output lacks `options m0 marker=1` |
| Control, same with a path without a space | HEAD | child sees the option |
| Patched (quote the value, plus the tokenizer fix of 036) | patch.diff, 5 runs | 5/5: env is `-C "ROOT/my conf"`, child sees the config |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - a real parent modprobe starts a real child modprobe through a real `install` command.

## Class

logic

## Root cause

`env_modprobe_options_append(optarg)` never quotes; the reader splits on every space. Quoting alone is not enough because the tokenizer then emits an empty argument after a closing quote (candidate 036), so `patch.diff` contains both changes.

## Introduced by

c3d0a5f "tools: add modprobe"

## The fix

`patch.diff` (+17/-3, tools/modprobe.c): `env_modprobe_options_append_quoted()` wraps a value containing a space in `"` (or `'` when it contains `"`), plus 036's tokenizer change. Verified with the repro (5/5 exit 1 -> 5/5 exit 0). Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- A path containing both quote characters is still appended unquoted (no quoting syntax for it in the tokenizer).
- `-d`/`-S` are not propagated at all (by design), only `-C`, `-s`, `-q`, `-v`.
