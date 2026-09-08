# filetype.cc is_gcc_lto_obj OOB read via unchecked e_shoff/e_shnum

`src/filetype.cc`'s `is_gcc_lto_obj<E>()` - the very first classification mold does on any
relocatable (`ET_REL`) input file, run before any other validation - dereferences the section
header table at `data + ehdr.e_shoff` with no check that the offset (or the `e_shnum` section
count) actually lies within the mapped file. Seeded from `code-audit-structured`'s prior static
review (`___FINDINGS___/findings-05-input.toml`, "high/medium/panic").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Hand-crafted malformed ELF, real CLI invocation | 64-byte ET_REL/EM_X86_64 object, `e_shoff=0x7fff0000`, `e_shnum=1`, `e_shstrndx=0`, fed to `mold -o out bad.o` via a real command-line invocation (`/usr/bin/mold` system install, mold 2.40.4) | Immediate SIGSEGV (exit 139) |
| Same input against this repo's own HEAD (c340272e), plain build | `-DCMAKE_BUILD_TYPE=Debug`, g++ 15.2.0 | Same SIGSEGV |
| Same input under an ASAN+UBSAN build (`-DMOLD_USE_ASAN=ON`, g++) | `build-asan/`, 5/5 repeat runs | Deterministic `AddressSanitizer: SEGV` every run, pinpointing the exact faulting line |

Two escalating approaches were enough - the crash was immediate and 100% deterministic from the
first attempt, so no third tool/fuzzing was needed (per `SKILL.md`'s soft cap, escalation stops
once a clear signal is confirmed).

## Status

**Confirmed** - 5/5 repeat runs under the ASAN build, deterministic (same faulting address/line
every time, `output.txt` captures one representative run). Also reproduces with the plain
system-installed `mold` 2.40.4 binary, confirming it isn't an ASAN-build artifact.

**Fully reproducible**: **Yes** - `repro/run.sh` builds no synthetic stand-in; it feeds a real,
crafted `.o` file to the real `mold` binary via a real command-line invocation, exactly the path
an actual user/build-system invocation of `mold <bad-object>` would take.

## Class

`crash` (memory-safety: out-of-bounds read via unchecked file offset from an untrusted input file).

## Root cause

`get_file_type()` (`src/filetype.cc:80-117`) calls `is_gcc_lto_obj<E>()` for every `ET_REL`
object, before `InputFile<E>::InputFile()` (which does have its own, later, bounds check) ever
runs. Inside `is_gcc_lto_obj` (pre-fix):

```c++
ElfShdr<E> *sh_begin = (ElfShdr<E> *)(data + ehdr.e_shoff);
std::span<ElfShdr<E>> shdrs{(ElfShdr<E> *)(data + ehdr.e_shoff), ehdr.e_shnum};
i64 shstrtab_idx = (ehdr.e_shstrndx == SHN_XINDEX) ? sh_begin->sh_link : ehdr.e_shstrndx;
for (ElfShdr<E> &sec : shdrs) { ... if (sec.sh_type != SHT_SYMTAB) ... }
```

`ehdr.e_shoff` and `ehdr.e_shnum` are raw fields from the untrusted input file's ELF header, used
to build a pointer/span with zero validation against `mf->size`. With `e_shoff` far past the end
of a tiny (64-byte) mapped file, the `for` loop's first iteration reads `sec.sh_type` (crash site:
`lib/integers.h:68`, `Integer<u32>::operator u32()`, called from `filetype.cc:39`) at an address
far outside the mmap - SIGSEGV.

This is the exact same bug *shape* as the already-checked-in bounds check in
`InputFile<E>::InputFile()` (`src/input-files.cc:97`, `if (mf->data + mf->size < (u8 *)(sh_begin +
num_sections)) Fatal(...)`) - that check exists precisely because this pattern is dangerous, but
`is_gcc_lto_obj` runs *earlier* and had no equivalent check at all.

## Introduced by

dd8002f0 "[ELF] Detect GCC LTO object files early" (2022-02-10) - the original commit that added
`is_gcc_lto_obj()` with this unchecked pointer arithmetic (function later moved as-is from
`src/filetype.h` into its own `src/filetype.cc` by 7b4d2e77 "Refactor", 2024-10-05, which didn't
change this logic).

## The fix

Added a bounds check at the top of `is_gcc_lto_obj()`, before `sh_begin`/`shdrs` are formed or
dereferenced anywhere in the function: computes the number of section-header-table entries that
must actually fit in the mapped file (`e_shnum`, or 1 if `e_shstrndx == SHN_XINDEX` since that case
unconditionally reads `sh_begin->sh_link` even when `e_shnum` is 0) and returns `false` (i.e. "not
a GCC LTO object", the safe default) instead of dereferencing out of bounds when it doesn't fit.
This mirrors the validate-before-dereference style `input-files.cc:97` already uses for the same
kind of untrusted offset.

```c++
u64 shoff = ehdr.e_shoff;
i64 need = std::max<i64>(ehdr.e_shnum, ehdr.e_shstrndx == SHN_XINDEX ? 1 : 0);
if (shoff > (u64)size || need > ((i64)size - (i64)shoff) / (i64)sizeof(ElfShdr<E>))
  return false;
```

Verified: the same crafted `bad.o` (`e_shoff=0x7fff0000, e_shnum=1, e_shstrndx=0`) no longer
crashes after the patch, 5/5 runs; `is_gcc_lto_obj` now returns `false` safely, `get_file_type`
classifies the file as plain `FileType::ELF_OBJ`, and control passes to
`InputFile<E>::InputFile()`, whose own existing check on line 97 catches the same bad `e_shoff`
and exits cleanly:

```
mold: fatal: bad.o: e_shoff or e_shnum corrupted: 64 1
```

(exit 1, no crash) instead of the pre-patch `AddressSanitizer: SEGV ... filetype.cc:39` abort.

- Project test suite (`ctest -j24` in `build-asan/`, ASAN+UBSAN build): 475/476 passed before and
  after the patch (unchanged). The one failure, `x86_64-run-clang`, is a pre-existing environment
  artifact of testing an ASAN-instrumented `mold`/`mold-wrapper.so` build under `LD_PRELOAD`
  ("ASan runtime does not come first in initial library list") - confirmed to fail identically on
  unpatched HEAD (c340272e) before this patch was ever applied, unrelated to `filetype.cc`. 20
  further tests were skipped by ctest itself (environment-gated: zstd/hwasan/apx-feature/etc.
  tests not applicable to this host/toolchain), same set skipped before and after.

## What's not done / open questions

- `is_gcc_lto_obj` has further unchecked reads later in the same function that this patch does
  *not* address (out of scope for this candidate, which targets the one confirmed crash): line 34
  (`shdrs[shstrtab_idx].sh_offset` - `shstrtab_idx` itself is never bounds-checked against
  `shdrs.size()`) and the `sec.sh_offset`-based `elf_syms` span further down. These look like the
  same bug class and are good candidates for a follow-up session (already only partially covered
  by `IDEAS.md`'s seeded backlog).
- `is_gcc_lto_obj` is also called for `I386`/`M68K`/`SPARC64` in the big-endian and 32-bit branches
  of `get_file_type` (`src/filetype.cc:88, 104, 107`) - not separately re-verified this session,
  but the fix is inside the shared template function so it covers all four instantiations
  identically; not a separate code path.
- `src/input-files.cc:86-107`'s own `InputFile::InputFile()` has a related but distinct bug (the
  `e_shnum == 0` case dereferences `sh_begin->sh_size` *before* its own bounds check runs) -
  tracked separately, see candidate 002.
