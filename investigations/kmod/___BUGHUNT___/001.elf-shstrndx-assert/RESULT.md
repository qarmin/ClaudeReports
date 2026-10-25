# elf-shstrndx-assert: crafted e_shstrndx / e_shnum aborts modinfo, depmod and any libkmod ELF caller

`kmod_elf_new()` (libkmod/libkmod-elf.c) passes the ELF header's `e_shstrndx` straight to `elf_get_section_info()` -> `elf_get_section_header_offset()`, which starts with `assert(idx != SHN_UNDEF)` and `assert(idx < elf->header.section.count)`. A 60s smoke fuzz of `kmod_elf_new()` hit the second assert within seconds (seed `_reference/seed-crash-elf-shstrndx-assert.bin`); the static audit had flagged the same function as a parser of untrusted module files.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real `modinfo` on the smoke-fuzz seed | `__build-asan` (clang 21, ASAN+UBSAN, debugoptimized, asserts on), `modinfo seed.ko` (182 KB, e_shnum=42, e_shstrndx=0x5f29) | `Assertion 'idx < elf->header.section.count' failed`, SIGABRT, exit 134 |
| Manual boundary values via the public API + real binaries (`repro/`) | Minimal 208-byte ELF64 ET_REL (null section + `.shstrtab`, e_shnum=2) with e_shstrndx = 0x5f29, 2 (== e_shnum), 0 (SHN_UNDEF), 1 (valid control); `kmod_module_new_from_path()` + `kmod_module_get_info()`, real `modinfo`, real `depmod -b` on a tree holding one bad `.ko` | 0x5f29 and 2 hit the `idx < count` assert, 0 hits the `idx != SHN_UNDEF` assert, all SIGABRT; control returns -ENODATA normally; `depmod` aborts on the whole tree |
| libFuzzer, `fuzz/fuzz_elf_new.c` (no pre-filter), unpatched | harness + `libkmod-elf.c` compiled with `-fsanitize=fuzzer,address,undefined` (`fuzz/build.sh` - linking only `libkmod-internal.a` gives cov: 2 because the archive has no coverage instrumentation), seed corpus `_reference/__corpus_elf` (22 files), `-max_len=65536`, seeds 1/2/3 | All 3 runs abort right after INITED (cov 25); artifacts in `fuzz/crashes/` (68-70 bytes, e_shnum=0 variants - both asserts hit) |
| libFuzzer, same harness, patched | same corpus + the seed, 180 s, `-rss_limit_mb=2048` | 68,893,011 execs, 322 new units, no crash; the 3 crash artifacts replay cleanly |

## Status

**Confirmed** - deterministic: `tools/confirm_repro.sh` on `repro/run.sh` against the unpatched main tree, 10/10 runs reproduced; also reproduced from a fresh clone via `just test`.

**Fully reproducible**: **Yes** - the input is a single file artifact, and `repro/` drives the real `modinfo` and `depmod` binaries plus libkmod's public `kmod_module_new_from_path()` + `kmod_module_get_info()` on it.

## Class

crash (assertion abort / denial of service on a malformed module file)

## Root cause

`kmod_elf_new()` loads `e_shnum`, `e_shentsize`, `e_shoff` and `e_shstrndx` from the header and checks that the section header table fits in the file, but never checks that `e_shstrndx` is a valid index into that table. It immediately calls `elf_get_section_info(elf, elf->header.strings.section, ...)`, whose helper `elf_get_section_header_offset()` treats an invalid index as a programming error (`assert`) before its own graceful `return 0` path. The asserts are live in any build without `NDEBUG`: meson's default is `b_ndebug=false`, and kmod's own `meson.build` / `build-dev.ini` do not change it, so a plain `meson setup` (or a distro build that doesn't pass `-Db_ndebug=true`) keeps them. Values that trip it: `e_shstrndx >= e_shnum` (including any value when `e_shnum == 0`), and `e_shstrndx == SHN_UNDEF` (0), which the ELF spec allows for "no section name string table".

Reachability: every libkmod path that parses the ELF goes through `kmod_module_get_elf()` / `do_init_module()` -> `kmod_elf_new()` - `kmod_module_get_info()` (modinfo), `kmod_module_get_versions()`, `kmod_module_get_symbols()`, `kmod_module_get_dependency_symbols()` (depmod), and insmod/modprobe with `--force-vermagic`/`--force-modversion`. One malformed `.ko` under `/lib/modules/<ver>` makes `depmod` abort for the entire tree instead of skipping that module.

With `NDEBUG` the existing runtime check in `elf_get_section_header_offset()` returns 0 and `kmod_elf_new()` fails with -EINVAL, so there is no out-of-bounds access; the defect is the abort.

## Introduced by

708624a "ELF: initial support for modinfo and strip of modversions and vermagic." (2011-12-18) - both asserts and the unchecked `e_shstrndx` use in `kmod_elf_new()` are present in that initial commit (`git log -S` on the assert line and on the `READV(e_shstrndx)` line; `git log -L` on the function shows only later reformat/refactor commits).

## The fix

`patch.diff` (+7/-0, libkmod/libkmod-elf.c): in `kmod_elf_new()`, after the section-table range check, reject `e_shstrndx == SHN_UNDEF || e_shstrndx >= e_shnum` with an `ELFDBG` message and `goto invalid` (-EINVAL). The asserts in `elf_get_section_header_offset()` are kept: all other callers pass indices from `1..count-1` (`kmod_elf_save_sections()`, `kmod_elf_get_section()` loops, and `elf_strip_versions_section()` with an index returned by `kmod_elf_get_section()`), so after this check the asserts genuinely describe an internal invariant.

Checked against more than the one input: all three boundary variants (oob, == count, SHN_UNDEF), the original 182 KB smoke-fuzz seed (`modinfo` now prints "could not get modinfo ... Invalid argument", exit 1), the three e_shnum=0 fuzz artifacts, and a 3-minute patched fuzz run (68.9M execs, no crash). After the patch `depmod` logs "failed to load symbols from .../evil.ko: Invalid argument" and finishes with exit 0.

- Project test suite (`tools/run_tests.sh __build-asan`, kmod's meson suite under ASAN/UBSAN): 7/7 OK before -> 7/7 OK after, no new failures.

## What's not done / open questions

- `SHN_XINDEX` extended numbering (e_shnum == 0 / e_shstrndx == 0xffff with the real values in section 0's `sh_size`/`sh_link`) is not supported by libkmod at all; with the patch such files are rejected with -EINVAL, as they effectively were in NDEBUG builds before. Kernel modules never need it in practice.
- Only `kmod_elf_new()` was fuzzed; the deeper getters (`kmod_elf_get_strings/_symbols/_dependency_symbols/_modversions`, signature parsing) remain a separate backlog item - with this fix applied, a harness no longer needs the shstrndx pre-filter.
- A maintainer may prefer to drop the asserts in `elf_get_section_header_offset()` entirely (the function already has a graceful error path); the patch keeps them deliberately.
