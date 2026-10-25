# kmod testbed status

Last full run: 2026-10-25 17:13-17:31 Europe/Warsaw (session 5), base HEAD 692c5a0, `python3 run.py` = 1045 s wall (`results/2026-10-25_171346/`, 222 failures, 29 signatures; the new components `regression`, `arch_matrix`, `fuzz_api` are clean). Earlier: 11:55-12:08, 760 s.

All builds are ASAN+UBSAN (`build-dev.ini`, asserts on). Every command runs under `tools/memcap.sh` (hard RSS cap, default 4096 MB, fuzzers `-rss_limit_mb=1536`) because runaway candidates froze the host once. `_src/` is a pristine `git archive HEAD` (the user's uncommitted work in the main tree is deliberately not tested); fixes are developed in `_reference/__wt`.

## Components (what each covers)

| Component | Kind | Covers | Time |
|---|---|---|---|
| `suite_clang`, `suite_gcc` | existing suite | kmod's own meson suite (7 tests) under ASAN/UBSAN with clang and gcc | 3 s each |
| `modinfo_diff` | differential | libkmod modinfo/versions/symbols/deps (`drivers/kinfo`) vs a Python ELF parser (`lib/kelf.py`) on the real 7.0.0-38-generic module tree | 1 s |
| `signature_diff` | differential | libkmod PKCS#7 signer/key/hash vs `openssl cms -print` | <1 s |
| `compress_diff` | differential + mutation | raw/xz/gz/zst variants and truncations of real modules must give the same info or an error | 3 s |
| `depmod_model` | model | `depmod` output on the real tree vs a model (dep closure, aliases, softdep, weakdep, devname) | 7 s |
| `index_diff` | differential | `modules.*.bin` mm reader vs stdio reader vs depmod source data (`drivers/idxcheck`) | 10 s |
| `cli_matrix` | CLI matrix | flag/argument combinations of depmod/modinfo/modprobe/lsmod/insmod/rmmod on corpus inputs, sanitizer + exit-code checks | 15 s |
| `config_model` | model | random `modprobe.d` configs vs expected `modprobe -c` output | 1 s |
| `modprobe_env_diff` | differential | `MODPROBE_OPTIONS` tokenizer (`drivers/envtok`) vs a shell-like reference splitter | <1 s |
| `depmod_graph` | model + sweep | random synthetic module graphs (`lib/synth.py` builds minimal ELF modules with exports/imports): acyclic trees must give `modules.dep` = transitive closure, cyclic/messy ones must not crash or hang | 5-30 s |
| `alias_glob` | differential | `modprobe --resolve-alias` over a depmod-built alias index vs libc `fnmatch` for random patterns/keys (generator limited to the supported glob subset) | 2 s |
| `oom_sweep` | fault injection | LD_PRELOAD shim (`fuzz/failmalloc.c`, plain build `__build-plain`) fails each allocation of modprobe/modinfo/depmod runs in turn; signals and hangs are failures | 10-60 s |
| `elf_sweep` | boundary sweep | field-by-field ELF header/section/`__versions` mutations of real modules through `drivers/kinfo` | 2 s |
| `regression` | regression | every candidate's `repro/run.sh` (exit non-zero = bug present) against the tested HEAD build, skipping 030/031; `regression:NNN:no-longer-reproduces` means fixed upstream (write a CORRECTION.md) | 40 s |
| `arch_matrix` | differential | the same synthetic modules as ELF32/ELF64, LE/BE (`lib/synth_arch.py`, real CRCs): `--show-exports`, `--show-modversions`, depmod `-E` CRC mismatch warnings (clean 2026-10-25) | 3 s |
| `sim/` (not a component yet) | simulator | `sim/sysfake.c`: LD_PRELOAD fake of /sys/module, /proc/modules, delete_module, finit_module with refcount/holders state; `sim/run_sim.sh <build> "mod:refcnt:holders;..." [TOOL=rmmod|lsmod] [CONF=..] [DEPS=..] args` runs modprobe/rmmod/lsmod against it. Found 045/046; scenarios checked clean: --first-time, -a, softdep pre/post removal order, remove commands, -w retry, rmmod chains, lsmod | manual |
| `corpus_run` | corpus | every real module through every ELF entry point | <1 s |
| `fuzz_api` | libFuzzer state machine | public libkmod API as random op sequences (lookup, deps, softdeps, info, dry-run probe, filters, resource reload, out-of-order unrefs) over a synthetic tree + fuzzed modprobe.d (178k execs clean, 150 s, 2026-10-25) | 75-165 s |
| `fuzz_elf`, `fuzz_modfile`, `fuzz_index`, `fuzz_config`, `fuzz_builtin`, `fuzz_modprobe_env`, `fuzz_util` | libFuzzer | `kmod_elf_new` + getters + strip (with an invariant that strip changes only SHF_ALLOC bits and vermagic), compressed file loaders, both index readers (+ reader/dump equivalence), config parser, `modules.builtin.modinfo`, `prepend_options_from_env` | 75-165 s each |

Corpus: Ubuntu `linux-modules-7.0.0-38-generic` (6901 signed `.ko.zst`, System.map, builtin.modinfo) fetched by `fetch_corpus.sh` into `__corpus-data/realroot` (250 MB, kept out of `pack`).

## Coverage map

| Entry point | Components |
|---|---|
| ELF parsing (`kmod_elf_*`) | modinfo_diff, elf_sweep, corpus_run, fuzz_elf |
| Compressed loaders (xz/zstd/zlib) | compress_diff, fuzz_modfile |
| Index files (`index_mm_*`, `index_*`) | index_diff, fuzz_index, depmod_model |
| depmod | depmod_model, cli_matrix, suite_* |
| modprobe config / install / softdep | config_model, cli_matrix, fuzz_config |
| modprobe options env | modprobe_env_diff, fuzz_modprobe_env |
| Module signatures | signature_diff |
| builtin modinfo | fuzz_builtin |

## Planned / gaps

- Session 4 (2026-10-25): `regression` added; process-level plumbing (install/remove status, MODPROBE_OPTIONS to child modprobe, `ulimit -f` write failures, directory symlink loops) was found by hand in 033-040 and is covered only by those repros, a generator-driven component for them is still open.

- depmod graph generator is built (`depmod_graph`); next step is a model of the cycle report text and larger dense SCCs under memcap.
- `oom_sweep` covers 5 command lines on a tiny tree; compressed-module and large-tree sweeps were done by hand (candidate 022) and could be added with a bounded sample.
- `install`/`remove` command expansion fuzz (`$CMDLINE_OPTS`, 010 found by hand).
- `kmod_module_new_from_lookup` / alias matching against `fnmatch` reference.
- `insmod`/`rmmod` flag handling needs root and a kernel: only argument parsing is reachable.

## Not worth building

- Roundtrip tests: kmod has no encoder besides `depmod` itself, covered by `depmod_model`/`index_diff`.
- Concurrency stress: libkmod is not thread-safe by design (single `kmod_ctx` user).
- Network/peer simulators: no network surface.

Session 4 note (2026-10-25): all session-4 `patch.diff` files (033-043, 035 already contains 036's hunk so 036 is skipped) apply together to HEAD 692c5a0 (checked in a scratch copy): the meson suite is 7/7 and every session-4 repro exits 0 on the combined build. Extended `failmalloc` sweeps (modprobe -n/-r/--show-depends with softdep/options/install config, depmod -n, modinfo, modprobe -c; 8-module tree) found only 019.
