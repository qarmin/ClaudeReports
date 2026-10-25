# modprobe-d-softdep-blacklist-doc: modprobe.d(5) says `--use-blacklist` applies to softdep modules, modprobe ignores it for them

modprobe.d(5), softdep section: "Running `modprobe c` is now equivalent to `modprobe a b c d e` without the softdep. Flags such as --use-blacklist are applied to all the specified modules". In reality `-b` is only passed as `KMOD_PROBE_APPLY_BLACKLIST` for the module named on the command line (`module_is_blacklisted(mod)` in `kmod_module_probe_insert_module()`); blacklist entries for softdep modules are not consulted, so with `blacklist a` and `softdep c pre: a`, `modprobe -b c` still inserts `a`. The project's own test `modprobe_blacklist_softdep` (commit f997c1c, 2025) says so explicitly ("check if modprobe -b dependent ignores blacklist entry of softdep") and expects the blacklisted softdep to be loaded. A `mask` entry, in contrast, does skip a softdep module (also pinned by tests).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `modprobe -b -n -v c` with `softdep c pre: a post: d`, `blacklist a`, `blacklist d` (`repro/run.sh`) | HEAD ASAN build, 3 runs | 3/3: a, c, d all printed as `insmod` |
| Same with `mask a` | HEAD | `a` skipped, c and d inserted |
| `testsuite/test-modprobe.c` | reading | the test encodes the ignore-blacklist behavior |

## Status

**Confirmed** - deterministic (`output.txt`). The conflict is between the man page and the code/tests; which one is intended is settled by the test, hence the patch changes the documentation.

**Fully reproducible**: **Yes** - the real binary against the documented claim.

## Class

docs (documentation does not match behavior)

## Root cause

The sentence dates from the original scdoc conversion (f3a423b) of the module-init-tools era text; the later softdep/blacklist rework pinned the opposite behavior.

## Introduced by

f3a423b "man: add scdoc based man pages"

## The fix

`patch.diff` (+4/-3, man/modprobe.d.5.scd): the sentence now says the flags apply only to the module named on the command line and that `mask` does affect softdeps. Not mechanically verifiable (the repro measures the behavior, which is unchanged); the meson test suite is unaffected by a man page edit (not run).

## What's not done / open questions

- If upstream prefers the documented behavior the code needs the change instead (apply the blacklist filter to the probe list); the test would then flip.
