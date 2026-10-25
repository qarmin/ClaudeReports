# builtin-modinfo-fuzz: `modules.builtin.modinfo` reader under libFuzzer

`kmod_builtin_get_modinfo()` (libkmod/libkmod-builtin.c) reads the kernel-provided `modules.builtin.modinfo` with `getdelim()` and manual prefix/length handling. Backlog item from the static audit ("manual offsets/lengths").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| libFuzzer `testbed/fuzz/fuzz_builtin.c` (input = module name + file bytes; calls `kmod_builtin_get_modinfo` and frees the result), seed corpus from the real 7.0.0-38 `modules.builtin.modinfo` | clang ASAN+UBSAN+LSAN, `-max_len=65536`, seed 7, 10 minutes, `memcap 4096` | 10,901,343 execs, peak RSS 429 MB, no crash, leak, or timeout (`testbed/__results/long-fuzz/builtin.log`) |
| Clang static analyzer pass | HEAD | one report (uninitialized `info->fp` in `get_strings` when `fopen` failed and `errno == 0`) that needs `-errno == 0` after a failed `fopen`: not reachable |

## Status

**Not reproducible** - no finding.
