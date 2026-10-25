# cmdline-opts-self-substitution: options containing the literal `$CMDLINE_OPTS` expand without bound

`kmod_module_probe_insert_module()`'s install-command expansion (`libkmod/libkmod-module.c`, the `while ((p = strstr(cmd, "$CMDLINE_OPTS")))` loop) restarts the search at the beginning of the already-substituted command after every replacement. If the substituted options themselves contain `$CMDLINE_OPTS`, the loop finds the new occurrence, substitutes again, and never terminates while the buffer doubles in size.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real `modprobe` with `install foo /bin/true $CMDLINE_OPTS` and the command line option `a=$CMDLINE_OPTS` (`repro/`) | HEAD ASAN build, `hard_rss_limit_mb=500`, 30 s timeout, 5 runs | 5/5 runaway growth: ASAN hard RSS limit hit within ~1 s (1.5 GB with a 1500 MB limit), nothing printed |
| `-n -v` (dry run) | same | prints the unexpanded install line: the expansion only runs when the command is really executed |
| Legit use: `install foo /bin/echo [$CMDLINE_OPTS] and [$CMDLINE_OPTS]` with options `x=1 y=2` | HEAD and patched builds | both print `[x=1 y=2] and [x=1 y=2]` |

## Status

**Confirmed** - deterministic, 5/5 on HEAD.

**Fully reproducible**: **Yes** - real `modprobe` binary, real config file and command line.

## Class

logic (unbounded memory growth / denial of service: infinite substitution loop)

## Root cause

After replacing one occurrence the code sets `cmd = s` and loops with `strstr(cmd, ...)` from offset 0, so text that came from `options` is rescanned. The options are user-controlled in several places: the `modprobe` command line, `options` lines in `modprobe.d`, and module parameters passed on the kernel command line (`foo.param=...` via `/proc/cmdline`). The expansion happens in a root process, so a hostile value only matters where the caller is less privileged than modprobe's config (e.g. a setuid-like wrapper or boot parameters); it is mainly a robustness issue.

## Introduced by

ddbda02 "Add implementation of modprobe's insertion" - the loop has this shape since the first implementation (re-checked with `git log -S`; later commits only changed allocation style).

## The fix

`patch.diff` (+3/-2, libkmod/libkmod-module.c): remember where the substituted text ends (`scan = prefixlen + options_len`) and continue the `strstr()` search from there, so inserted options are never rescanned. Verified: repro 5/5 runaway -> 5/5 clean exit 0, legit multi-occurrence expansion unchanged. Project test suite (ASAN/UBSAN): 7/7 OK before -> 7/7 OK after.

## What's not done / open questions

- Whether upstream wants the literal `$CMDLINE_OPTS` inside options to be preserved verbatim (this patch does so) or rejected is a maintainer choice.
