# config-parser-fuzz: modprobe.d / softdep / weakdep / options parser under libFuzzer

The hand-written two-pass tokenizers in libkmod/libkmod-config.c (`kmod_config_add_softdep`, `kmod_config_add_weakdep`, option/alias/install/blacklist/remove handling, `kmod_config_new` over a directory of files and `/proc/cmdline`-style input) were flagged by the static audit as risky.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| libFuzzer `testbed/fuzz/fuzz_config.c` (writes the input as a `modprobe.d/*.conf` plus a kernel command line, then calls `kmod_config_new` and walks every `kmod_config_get_*` iterator, including `softdep_to_char`) | clang ASAN+UBSAN with leak detection, `-max_len=65536`, seed 7, 10 minutes, `memcap 4096` | 1,190,417 execs (1980/s), coverage 1701 features / 10406 ft, 645-unit corpus, no crash, no leak, no timeout (`testbed/__results/long-fuzz/config.log`) |
| Same harness, 75 s runs in `just testbed` | previous refresh | 0 failures |
| `config_model` (random configs vs expected `modprobe -c` lines) | HEAD ASAN | only the softdep separator mismatch, confirmed as candidate 004 |

## Status

**Not reproducible** - the parser holds up against 1.2M mutated inputs under ASAN/UBSAN/LSAN. The only defect found in this area by any approach is the output-formatting bug in 004.

## Not done

- The fuzzer feeds files, not the `/proc/cmdline` path via a fake file descriptor beyond what the harness wraps (`--wrap=open`); a dedicated kcmdline mutator could still be tried.
