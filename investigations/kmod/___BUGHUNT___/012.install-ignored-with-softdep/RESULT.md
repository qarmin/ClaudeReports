# install-ignored-with-softdep: an `install` command is silently skipped for any module that has a softdep

`__kmod_module_fill_softdep()` (libkmod/libkmod-module.c) sets `mod->ignorecmd = (pre != NULL || post != NULL)`, and `kmod_module_probe_insert_module()` then prints/executes a plain `insmod` instead of the configured `install` command for that module. A `modprobe.d` file with both `softdep foo pre: bar` and `install foo <cmd>` therefore never runs `<cmd>`. Upstream issue #446 ("Can't blacklist ksmbd, even with install /bin/false") is this case; open PR #461 proposes the same fix as `patch.diff`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| GitHub issue #446 (`__issues_raw/issues/446.json`) as lead | n/a | claim: `install /bin/false` ignored for modules with softdeps |
| Real `depmod` + `modprobe -n -v -C etc -d root -S 1.0 foo` on two synthetic modules (`repro/gen_tree.py`), config `softdep foo pre: bar` + `install foo /bin/echo INSTALL-CMD-RAN` | HEAD ASAN build, 5 runs | 5/5 prints `insmod .../foo.ko`; the install command never appears. Control with only the `install` line: prints `install /bin/echo INSTALL-CMD-RAN` |

## Status

**Confirmed** - deterministic, 5/5 on HEAD. The behavior contradicts modprobe.d(5) ("install ... run command instead of inserting the module") and reporter expectations; whether the 2012 design (commit 89e9248) intentionally lets softdep win is a maintainer question, recorded under open questions.

**Fully reproducible**: **Yes** - real `depmod` and `modprobe` on a real (minimal) module tree.

## Class

logic

## Root cause

`ignorecmd` is meant to stop install commands from re-entering softdep resolution, but it is set purely on the presence of a softdep, so it also suppresses an explicit `install` directive (including the documented blacklist idiom `install foo /bin/false`).

## Introduced by

89e9248 "libkmod-module: probe: add flag to ignore commands"

## The fix

`patch.diff` (+1/-1, libkmod/libkmod-module.c): set `ignorecmd` only when no install command is configured. Verified: repro 5/5 wrong -> 5/5 prints the install command (dependency `bar` is still inserted first); project test suite (ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- If an install command itself calls `modprobe foo`, honoring it for softdep modules could recurse; the `--ignore-install` idiom in the man page is the documented way around it, but upstream may prefer another behavior.
- The upstream testsuite has no softdep+install fixture; PR #461 adds one.
