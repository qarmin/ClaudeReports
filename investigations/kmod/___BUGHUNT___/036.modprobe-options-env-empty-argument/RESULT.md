# modprobe-options-env-empty-argument: `$MODPROBE_OPTIONS` tokenizer produces empty arguments

`prepend_options_from_env()` emits a token at every space, also when the token is empty. Two consecutive spaces, a leading space, or a space right after a closing quote therefore insert an empty string into argv. `getopt_long()` treats it as the first non-option argument, so `MODPROBE_OPTIONS='-n  -v' modprobe m0` fails with `Module  not found` (the module name is the empty string; `m0` becomes an option string of the module).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real `modprobe -d <root> -S 1.0 m0` on a synthetic tree with `MODPROBE_OPTIONS` = `-n -v`, `-n  -v`, `-C "<dir>" -n` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: first value works, the other two exit 1 with `Module  not found` |
| Patched | patch.diff, 5 runs | 5/5: all three exit 0 |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real binary reading the real environment variable.

## Class

logic

## Root cause

The `' '` case in the tokenizer loop stores `str` even when `s == str`.

## Introduced by

c3d0a5f "tools: add modprobe"

## The fix

`patch.diff` (+4/-2, tools/modprobe.c): skip empty tokens. Verified with the repro (5/5 exit 1 -> 5/5 exit 0). Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- This is the item "MODPROBE_OPTIONS tokenizer semantics" from the backlog; other divergences from shell splitting (`'a b'c` splitting, quote leftovers) were not judged and remain upstream design questions.
- Candidate 035 depends on this fix being applied together with the quoting change.
