# elf-strip-vermagic over-read: non-NUL-terminated .modinfo in a heap module buffer

`elf_strip_vermagic()` (libkmod/libkmod-elf.c) walks the `.modinfo` section as a run of NUL-terminated strings using `strlen()`, but nothing in libkmod guarantees `.modinfo` ends in a NUL byte. When the section's bytes are not NUL-terminated and reach the end of the module buffer, `strlen()` reads past the buffer. For a module that libkmod holds in a heap buffer (any compressed `.ko`, decompressed into an exact-size `malloc()` buffer by `kmod_file`), this is a genuine heap-buffer-overflow read; `insmod --force` / `modprobe --force-vermagic` hit it end to end.

Found by a libFuzzer harness over the ELF getters (`fuzz/fuzz_elf_getters.c`) that drives `kmod_module_signature_info`, `kmod_elf_new`, `kmod_elf_get_section/_modinfo_strings/_build_id/_modversions/_symbols/_dependency_symbols` and `kmod_elf_strip`, reading every returned string/buffer so ASAN sees over-reads. A `-DNDEBUG` build of the harness (asserts off, to see past the known candidate 001 `e_shstrndx` assert) left exactly one memory-safety class standing: this over-read, from 8 independent crash inputs.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| libFuzzer, asserts ON, all getters | harness + libkmod-elf.c + signature .c with `-fsanitize=fuzzer,address,undefined`, 26 seeds (corpus_elf + debug-stripped mod-simple*.ko with signature trailers), fork=8, 300 s, 83.3M execs, cov 340 | 5812 crash artifacts; by call stack: 4493 `kmod_elf_get_modversions` LeakSanitizer (known static finding), 1315 `elf_get_section_header_offset` assert via `kmod_elf_resolve_crc` (001 variant reached through `.symtab`), 1 heap-buffer-overflow in `elf_strip_vermagic` |
| libFuzzer, asserts OFF (`-DNDEBUG`), leaks off | same harness built with `-DNDEBUG -detect_leaks=0`, fork=8, 366 s, 226M execs, cov 343 | 8 crash artifacts, all the same class: heap-buffer-overflow READ in `elf_strip_vermagic` (`libkmod-elf.c:643`, `strlen`). No other memory-safety survivor |
| Minimal hand-built ELF | 287-byte ELF64 REL, 3 sections (null, .shstrtab, .modinfo), `.modinfo = "license=GPL"` with no trailing NUL at EOF (`repro/gen_min.py`); `kmod_elf_new` + `kmod_elf_strip(FORCE_VERMAGIC)` on an exact-size `malloc` buffer (`repro/repro.c`) | ASAN heap-buffer-overflow, READ of size 12, 0 bytes after the 287-byte region, in `strlen` <- `elf_strip_vermagic` <- `kmod_elf_strip` |
| Real `insmod --force` binary | `__build-asan/insmod --force evil.ko.xz` (xz-compressed copy of the minimal module; kernel compression on this host is zstd, so the compression mismatch forces the `do_init_module` userspace-decompress-then-strip path) | ASAN heap-buffer-overflow in `elf_strip_vermagic` <- `kmod_elf_strip` <- `do_init_module` <- `do_insmod` |
| Repeatability | `tools/confirm_repro.sh` on `repro/run.sh` against the unpatched tree, 10 runs | 10/10 reproduced |
| libFuzzer, patched (`-DNDEBUG`) | harness rebuilt against patched tree, minimal input + the 8 artifacts replayed | no crash; `kmod_elf_strip` returns -ENODATA cleanly |

## Status

**Confirmed** - deterministic (10/10 via `tools/confirm_repro.sh`; also reproduced from a fresh clone with `just test`).

**Fully reproducible**: **Yes** - a single 287-byte module file (generated deterministically by `repro/gen_min.py`); `repro/run.sh` drives both libkmod's internal strip path and the real `insmod --force` binary on an xz-compressed copy.

## Class

crash (heap-buffer-overflow, out-of-bounds read of untrusted module data)

## Root cause

`.modinfo` is treated as a sequence of C strings. `elf_strip_vermagic()` scans it with `strlen(s)` in two places (line 643 `i += strlen(s)` for the non-`vermagic=` branch, line 648 `len = strlen(s)` for the match), relying on a terminating NUL that libkmod never verifies exists. Other string sections are checked for a trailing NUL: `.strtab` (`kmod_elf_get_symbols`), `__ksymtab_strings` (`kmod_elf_get_symbols_symtab`) and `.shstrtab` (`kmod_elf_new`) all reject a section that does not end in `\0`. `.modinfo` has no such check anywhere, so `elf_get_section_info()` validates only that the section lies inside the file, and a crafted section ending in a non-NUL byte at the end of the module buffer makes `strlen()` read past the buffer.

Whether the over-read is observable depends on how libkmod holds the module:
- Uncompressed `.ko`: `kmod_file` `mmap`s it; the bytes between EOF and the page boundary are zero-filled, so `strlen()` usually stops there and the over-read is masked (still UB, just not a fault).
- Compressed `.ko` (xz/zstd/gzip): `kmod_file` decompresses into an exact-size `malloc()` buffer (e.g. `libkmod-file-zstd.c` `malloc(frame_size)`), with no trailing page padding, so the over-read runs into heap metadata/redzone - a true heap-buffer-overflow.

Reachability: `elf_strip_vermagic()` runs only via `kmod_elf_strip(KMOD_INSERT_FORCE_VERMAGIC)`, called from `do_init_module()` when a force flag is set, i.e. `insmod --force` / `insmod --force-vermagic` / `modprobe --force` / `modprobe --force-vermagic`. `do_init_module()` (the userspace decompress-and-strip fallback) is taken either on a kernel without `finit_module`, or - as on this host - whenever the module's compression differs from the kernel's, which `do_finit_module()` turns into a forced `-ENOSYS`. So an attacker-supplied compressed module plus a user forcing the load reaches the heap over-read.

The modversions LeakSanitizer hits and the `resolve_crc` assert that the asserts-on run also surfaced are already tracked (the leak is static-audit finding "kmod_elf_get_modversions leaks on too-long symbol"; the assert is candidate 001's unchecked-`e_shstrndx` reached through `.symtab`/`st_shndx`), so they are not re-reported here.

## Introduced by

708624a "ELF: initial support for modinfo and strip of modversions and vermagic." (2011-12-18) - `elf_strip_vermagic()` with the two unbounded `strlen()` calls on `.modinfo` is present in that initial commit (`git log -S 'i += strlen(s)'` and `git log -L` on the function bottom out there; later commits only reformat it).

## The fix

`patch.diff` (+2/-2, libkmod/libkmod-elf.c): bound both `strlen(s)` calls in `elf_strip_vermagic()` with `strnlen(s, size - i)` so the scan can never read past the section (and therefore past the module buffer). `size` is the section length and `i` the current in-section offset, so `size - i` is exactly the bytes still available. A `vermagic=` string that runs to the section end without a NUL is then cleared up to the end instead of over-reading; a non-matching unterminated trailing string advances `i` to `size` and ends the loop. Real modules always NUL-terminate `.modinfo`, so there is no behavior change for valid input.

- `repro/run.sh` against the patched tree: internal API returns -ENODATA, `insmod --force` prints "Failed to strip version information: No data available" and exits 1 - no ASAN abort.
- Project test suite (`tools/run_tests.sh __build-asan`): 7/7 OK before -> 7/7 OK after.

A broader alternative is to reject a non-NUL-terminated `.modinfo` once in `kmod_elf_new()` (mirroring the existing `.strtab`/`.shstrtab`/`__ksymtab_strings` checks), which would also cover the separate 1-byte over-read in `kmod_elf_get_modinfo_strings()` (static-audit finding, `libkmod-elf.c:460-470`, reads `strings[size]` when the last byte is NUL). The localized `strnlen` fix here is the minimal change that closes the confirmed crash; the `kmod_elf_get_modinfo_strings` over-read remains a separate open suspect.

## What's not done / open questions

- `kmod_elf_get_modinfo_strings()` 1-byte over-read (static finding) was exercised by the same harness but did not fault on the seeds used (its `.modinfo` ended in NUL and was not at the buffer tail); it is a distinct suspect and not fixed by this patch.
- `fuzz/crashes_sample/` keeps 3 of the 8 `-DNDEBUG` artifacts; `fuzz/fuzz_asserts.log.gz` and `fuzz/fuzz_ndebug.log` hold the campaign stats. The harness binaries and full corpora were deleted to save disk.
