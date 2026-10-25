# softdep-module-vs-admin-precedence-by-filename: whether a module's own softdep or the admin's `softdep` line wins depends on the admin file's name

libkmod reads `modules.softdep` and `modules.weakdep` (written by depmod from `softdep=`/`weakdep=` module metadata) as additional configuration files. `kmod_config_new()` inserts them with `conf_files_insert_sorted()` into the same list that holds the `*.conf` files, so they are parsed in lexicographic order of their *file names* among the admin's files. For `softdep` and `weakdep` the first matching stanza wins (`kmod_module_get_softdeps()`/`get_weakdeps()` stop at the first match), therefore a stanza in `aa.conf` overrides the module-provided one (`modules.softdep` sorts after it) while the same stanza in `zz.conf` is silently overridden by it. The precedence between administrator configuration and vendor/module-provided data should not depend on how the administrator happens to name the file; modprobe.d(5) only promises sorting among the configuration files.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Module `m` with `softdep=pre: a`, admin `softdep m pre: b` in `aa.conf` / in `zz.conf`; `modprobe -n -v m` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: `aa.conf` -> b, m; `zz.conf` -> a, m |
| Same, patched (module-provided files parsed after all admin files) | patch.diff, 5 runs | 5/5: b, m in both cases |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - real depmod and modprobe on a synthetic tree.

## Class

logic

## Root cause

The two module-provided files are put into the filename-sorted list of configuration files instead of being treated as a separate, lowest-priority source (commit 8240333 added them with the same helper).

## Introduced by

8240333 "config: also parse softdeps from modules"

## The fix

`patch.diff` (+17/-3, libkmod/libkmod-config.c): parse `modules.softdep` and `modules.weakdep` after all configuration files (and before the kernel command line, as before), so admin stanzas always win. Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after. Side effect: `modprobe -c` lists the module-provided softdep/weakdep lines after the configuration files' lines.

## What's not done / open questions

- Upstream may have wanted the opposite priority (module data beats admin); this would then be a documentation gap rather than a bug, but a filename-dependent answer cannot be intended either way.
- Several stanzas for the same module in one source still resolve to the first one (separate design question, upstream issue #33).
