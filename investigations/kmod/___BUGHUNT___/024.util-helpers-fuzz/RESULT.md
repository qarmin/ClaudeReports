# util-helpers-fuzz: shared/util.c string and path helpers under libFuzzer

`freadline_wrapped()`, `path_to_modname()`, `path_ends_with_kmod_ext()`, `alias_normalize()` and `underscores()` (shared/util.c) are used by the config parsers, depmod and module lookup on attacker-influenced text (config lines, module paths, aliases). Hand-written scanning with `PATH_MAX` buffers and a backslash/bracket state machine.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| libFuzzer `testbed/fuzz/fuzz_util.c` with invariant checks (reported length equals `strlen` of the output buffer; no dash survives `underscores()` outside `[...]`; total bytes returned by `freadline_wrapped` never exceed the input) | clang ASAN+UBSAN+LSAN, `-max_len=4096`, 4 minutes, `memcap 3000` | 49,436,600 execs, no crash, leak or invariant violation |
| Manual look at `alias_normalize()` | n/a | an alias longer than `PATH_MAX - 1` is silently truncated and reported as success (no overflow); not reachable with a realistic alias |

## Status

**Not reproducible** - no finding.

## Not done

- `read_str_long`/`read_str_ulong`/`pread_str_safe` take file descriptors and were not fuzzed.
