# libkmod-install-commands-doc: kmod_module_get_install_commands()/get_remove_commands() are documented to return all commands, they return only the first

libkmod.h documents both functions as "Returns: a string with all install (remove) commands separated by semicolons ... concatenated by ';'". The implementation (`kmod_module_get_install_commands()`/`get_remove_commands()` in libkmod/libkmod-module.c) walks the configuration and stops at the first matching `install`/`remove` stanza ("find only the first command, as modprobe from module-init-tools does"), returning that single command. With two `install m` lines the string is just the first one; nothing is concatenated.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Public API call from a C program: config with `install m echo one`, `install m echo two`, `remove m echo rone`, `remove m echo rtwo`; print both getters (`repro/repro.c`, `repro/run.sh`) | HEAD ASAN build, 3 runs | 3/3: `echo one` and `echo rone` only |
| Same configuration through `modprobe -c` | HEAD | lists all four lines, modprobe uses the first |

## Status

**Confirmed** - deterministic (`output.txt`). The conflict is between the API documentation and the code; the code matches the long-standing modprobe behavior, so the patch corrects the documentation.

**Fully reproducible**: **Yes** - public API of the built libkmod.

## Class

docs (documentation does not match behavior)

## Root cause

The comments were written for an earlier design in which commands were concatenated.

## Introduced by

not determined

## The fix

`patch.diff` (+8/-8, libkmod/libkmod.h): the two gtk-doc comments now say that only the first matching command is used and returned, or NULL if there is none. Not mechanically verifiable (the repro measures the behavior); no code change, test suite unaffected (not run).

## What's not done / open questions

- If concatenation was the intent, the code would have to change and `modprobe`'s "first stanza wins" install semantics with it.
