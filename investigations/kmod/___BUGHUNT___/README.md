# kmod bug-hunt workspace

Dynamic verification of kmod (libkmod + tools) at the commit pinned in each candidate's justfile. Companion to the static audit in `../___FINDINGS___/findings.toml`.

## Toolchain / environment (as of 2026-10-25)

- clang 21.1.8 with libFuzzer (`-fsanitize=fuzzer,address,undefined`), gcc, AFL++, meson/ninja.
- Container has no kernel headers and no root. The test suite's dummy modules need them, so `linux-headers-7.0.0-38-generic` (+ its `_all` companion) were fetched with `apt-get download` into a private apt state dir and extracted with `dpkg -x` into `__kheaders/root/`. `tools/run_tests.sh` exports `KDIR` to it.
- `scripts/sanitizer-env.sh` (used by `meson test`) needs `CC=clang` in the environment or every test fails with exit 127 (`libclang_rt.asan-x86_64.so` not found). `tools/run_tests.sh` sets it.
- Disk: ~52 GB free on a 98% full volume - keep builds inside `__build-*`, remove fuzz corpora that grow big.

## Shared build

- `__build-asan/` - clang, `build-dev.ini` (ASAN+UBSAN, debugoptimized, asserts ON - no NDEBUG). Baseline `tools/run_tests.sh __build-asan`: **7/7 OK**.
- Rebuild after applying a patch to the main tree: `tools/build_asan.sh .. __build-asan` (incremental).

## Helper scripts (`tools/`)

- `build_asan.sh <src> <bld>` - configure once + build with sanitizers.
- `run_tests.sh <bld>` - kmod's own meson suite under ASAN/UBSAN, prints per-test status and Ok/Fail counts.
- `build_fuzz.sh <src> <bld> <harness.c> <out>` - links a libFuzzer harness against `libkmod-internal.a` + `libshared.a` (internal API such as `kmod_elf_new`, `index_mm_open`, `kmod_config_new` is reachable). Add `-DNDEBUG`-free workarounds in the harness, not in the library.
- `confirm_repro.sh <bin> [-- args]` - N-run repeatability check (0 exit = not reproduced).
- `tidy.py` (via `just tidy`) - removes core dumps / crash litter, sorts SUMMARY tables.

## Repro convention for C candidates

`NNN.NAME/repro/` contains `repro.c` (or input files + a driver script) and `run.sh <kmod_src_dir> <build_dir>` which builds the repro against that tree's sanitizer build and runs it. Exit 0 = bug not present, non-zero (incl. sanitizer abort) = bug reproduced. The candidate `justfile` clones the repo at `base_commit`, builds with `tools/build_asan.sh` logic inlined, runs `repro/run.sh` before and after applying `patch.diff`.

## Fuzzing notes

- Seed corpus for ELF: the `.ko` files built under `__build-asan/testsuite/` (module-playground) and `../testsuite/rootfs-pristine/**.ko*`.
- The very first 60s smoke run of an ELF harness (`_reference/smoke_elf.c`) aborted on `assert(idx < elf->header.section.count)` in `elf_get_section_header_offset()` (crafted `e_shstrndx`). Input saved as `_reference/seed-crash-elf-shstrndx-assert.bin`. A harness that wants to fuzz deeper than `kmod_elf_new()` must pre-filter inputs with an out-of-range `e_shstrndx`/`e_shnum == 0` (or the run stops on this one bug every time).

## Session 2 additions (2026-10-25)

- `tools/memcap.sh <MB> <cmd...>` - hard RSS cap for everything that can run away (depmod cycles, `$CMDLINE_OPTS`, fuzzers, test suite). Forwards stdin. Put `timeout -s KILL N` inside it (`memcap.sh MB timeout -s KILL N cmd`), otherwise killing memcap leaves the setsid'd child running. `ulimit -v` does not work with ASAN.
- `tools/failmalloc.c` + `tools/failmalloc_sweep.sh <label> <cmd...>` - LD_PRELOAD allocation-failure injection (fail the n-th malloc/calloc/realloc/strdup for every n). Needs a plain non-ASAN build (`meson setup ... -Dbuildtype=debug` without `build-dev.ini`), and the preload must be applied to the target only (`env LD_PRELOAD=... cmd`), not to `timeout`/gdb. OpenSSL deadlocks in its own init when an allocation fails there (candidate 022) - those hangs are not libkmod bugs.
- `tools/api_null_gen.py` - generates a driver that calls every public `kmod_*` function with NULL/zero arguments.
- `tools/mkjustfile.sh`, `tools/mkoutput.sh`, `tools/addrow.py` - candidate scaffolding (justfile from the template, before/after `output.txt` from `repro/run.sh`, SUMMARY/SUMMARY-SHORT rows + tidy).
- Fixes are developed in a scratch worktree `_reference/__wt` (+ `wt-build` ASAN build) so that uncommitted work in the main tree is never touched; remove with `git worktree remove --force _reference/__wt`.
- Real-binary index repros need all five `modules.*.bin` (builtin.alias optional) next to each other; lookups use the bare name (no `alias ` prefix, that is dump-only).
