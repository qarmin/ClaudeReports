# depmod-conf-generative: `depmod -C <file>` configuration parser

`cfg_file_parse()` / `cfg_kernel_matches()` (tools/depmod.c) tokenize `depmod.d` lines with `strtok_r` and compile user-supplied regular expressions; the `override`/`external` lines allocate with `strlen`-derived sizes. 005 was found in the neighbouring path-building code.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Generated token-soup config files (directives `search override external exclude include make_map_files`, regex metacharacters, unbalanced `[`/`(`, 300-byte words, backslash-newline continuations, tabs, `..`) given to real `depmod -b <root> -C <file> 1.0` on a two-module tree (`kernel/foo.ko`, `updates/foo.ko`), 300 files, seed 5 | HEAD ASAN+UBSAN+LSAN, `memcap 3000`, 30 s kill | 0 crashes, 0 leaks, 0 hangs, 0 UBSAN reports |

## Status

**Not reproducible** - no finding in the parser besides 005.

## Not done

- Stateful combinations across several files in a directory (`-C <dir>` with many `.conf` files) were not generated.
