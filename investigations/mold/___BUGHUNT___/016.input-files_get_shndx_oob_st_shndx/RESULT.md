# input-files_get_shndx_oob_st_shndx

`ObjectFile<E>::get_shndx()` (src/mold.h) resolves an ELF symbol's section index
(`st_shndx`, or the `SHT_SYMTAB_SHNDX`-resolved value when `st_shndx == SHN_XINDEX`)
with no validation against the object file's actual number of sections. Every caller in
`src/input-files.cc`/`src/mold.h` then uses the returned value directly as a raw index
into `elf_sections` (a `std::span`) or `sections`/`comdat_discarded` (small
`std::vector`-backed containers) - flagged by a prior static-audit pass
(`code-audit-structured`, commit c340272e63) as an unchecked-index pattern already found
several times elsewhere in this file (candidates 010, 011, 012 this session).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `get_shndx()` (src/mold.h:3541-3551) and every call site (`read_section_metadata`, `initialize_local_symbols`, `reattach_section_pieces`, `InputSection::get_fragment`, `ObjectFile::get_section`, `ObjectFile::is_discarded_comdat`, `Symbol::name`) | code reading only | Confirmed `elf_sections.size()`/`sections.size()` are established (both == `elf_sections.size()`, set in `parse_sections()` before `initialize_local_symbols()` runs) but never checked against the returned index anywhere. |
| Variant 1: hand-crafted x86_64 ET_REL object, 5 sections, `.symtab` with one local `STT_SECTION` symbol using `st_shndx = SHN_XINDEX` (0xffff), paired with a 2-entry `SHT_SYMTAB_SHNDX` section whose second entry is `0xFFFFFFF0` (maximally huge, per this project's own accumulated lesson from candidate 010 that a merely-moderate bad index into a small heap container can silently miss) | plain `-DCMAKE_BUILD_TYPE=Debug` build (g++ 15, `_GLIBCXX_ASSERTIONS` active by default in this build config), real `mold` binary via CLI | **Crashed**: `std::vector<unsigned int>::operator[]` assertion failure (`__n < this->size()`), SIGABRT, exit 134. gdb backtrace confirms the fault is `InputSectionTable<X86_64>::operator[](idx=4294967280)` called from `ObjectFile<X86_64>::initialize_local_symbols` at src/input-files.cc:916. |
| Variant 2 (same bug class, different trigger path per CONFIRMATION.md section 7): same object shape but no `SHN_XINDEX`/`SHT_SYMTAB_SHNDX` at all - a direct `st_shndx = 0xFEFF` (the largest value below the reserved `SHN_LORESERVE` range), 4 sections total | same plain build | **Crashed identically** (same assertion, exit 134) - confirms the bug is in `get_shndx()`'s direct-return path too, not just the `SHN_XINDEX` path. |
| Repeat variant-1 repro 5 times in a row (no fixed seed needed - the crash is fully deterministic, not memory-layout-dependent) | same plain build | 5/5 runs: identical SIGABRT/exit 134 every time. |

## Status

**Confirmed**, reproduces 5/5 runs (variant 1) under the plain Debug build (g++ 15,
`_GLIBCXX_ASSERTIONS` on) - fully deterministic, no ASAN/UBSAN instrumentation needed to
observe it (libstdc++'s own hardened `std::vector::operator[]` assertion is what aborts
the process). Variant 2 (a different, simpler trigger within the same bug class) also
reproduces identically.

**Fully reproducible**: **Yes**. The repro runs the real `mold` binary via its actual CLI
entry point (`mold -o <out> <evil.o>`) on a hand-crafted-but-spec-compliant ELF object -
this is a malformed-file-to-parser case (CONFIRMATION.md section 11's stated exception
does not even apply: there is no second participant to fake here, a crafted `.o` file
handed to `mold`'s real `parse_symbols`/`parse_sections` path *is* the genuine external
input a real attacker/build system would supply).

## Class

memory-safety (unchecked index into small `std::vector`/`std::span`-backed container;
degrades to a raw OOB heap read/segfault in a build without libstdc++'s
`_GLIBCXX_ASSERTIONS` hardening, and to a clear SIGABRT with it).

## Root cause

`ObjectFile<E>::get_shndx()` returns a symbol's resolved section index straight from
`st_shndx` (or, for the `SHN_XINDEX` escape value, straight from the corresponding
`SHT_SYMTAB_SHNDX` table entry) with no check that the value is less than
`elf_sections.size()`. Every caller then indexes `elf_sections`/`sections`/
`comdat_discarded` directly with the result - in `initialize_local_symbols()`
(src/input-files.cc:915-932), this reaches `InputSectionTable<E>::operator[]`
(src/mold.h:3554), which does `indices[idx]` on a small heap `std::vector<u32>` with no
bounds check of its own. A `_GLIBCXX_ASSERTIONS`-hardened libstdc++ build turns this into
an immediate, deterministic `std::vector::operator[]` assertion failure/SIGABRT; a build
without that hardening would instead perform a raw OOB heap read (silently returning
garbage for a moderate offset, or segfaulting for a sufficiently large one, per this
project's session notes on candidate 010).

A second, independent instance of the identical unchecked-index shape exists one line
earlier in the same function: `symtab_shndx_sec[&esym - &this->elf_syms[0]]` indexes the
`SHT_SYMTAB_SHNDX` table by symbol number with no check that the table is at least as
long as `elf_syms` - a short (or entirely construct-around-able) `SHT_SYMTAB_SHNDX`
section combined with any symbol using `SHN_XINDEX` triggers the same OOB-index bug
through a different immediate cause. The fix (below) closes both at once.

## Introduced by

ad3bd347 "Handle object files having more than 0xff00 sections" (2021-03-04) - the commit
that introduced `get_shndx()`/`symtab_shndx_sec`/SHN_XINDEX support in the first place;
no bounds check against `elf_sections.size()` (or against `symtab_shndx_sec.size()` for
the symbol-number lookup) was ever present, from this function's very first version
through the current codebase (`git log -S'get_shndx'` shows 14 later commits touching
the string, none adding such a check - confirmed by reading the current function body,
which is functionally unchanged in this respect from the 2021 original).

## The fix

Centralized the validation inside `get_shndx()` itself (src/mold.h), so every caller is
protected at once with no signature/call-site changes needed:

- The `SHN_XINDEX` branch now checks `idx < symtab_shndx_sec.size()` before indexing the
  shndx table (closes the second, independent bug described above).
- The resolved `shndx` (from either the `SHN_XINDEX` table or the direct `st_shndx`
  value) is now checked against `this->elf_sections.size()` before being returned.
- Both failure cases return `0` (the null section index), matching the function's
  existing, established convention for "this symbol has no real section" (the
  `st_shndx >= SHN_LORESERVE` case already returned `0` before this patch) - every caller
  already treats index `0`/a null `InputSection*` as "no section" safely (confirmed by
  reading `get_section`, `is_discarded_comdat`, `initialize_local_symbols`,
  `reattach_section_pieces`, and `Symbol::name`).

No `Context<E>&` threading was needed: several callers of `get_shndx()`
(`ObjectFile::get_section`, `ObjectFile::is_discarded_comdat`, `Symbol::name`) have no
`ctx` parameter available at all, so validating inside `get_shndx()` itself (returning
the existing "no section" sentinel rather than calling `Fatal(ctx)`) was both the
simpler and the more centrally-effective fix, matching what the task brief itself
suggested as the preferred option.

Verified: variant-1 repro re-run against the patched build - clean link (exit 0, only
the expected "entry symbol is not defined" warning), 3/3 runs. Variant-2 repro (the
different, direct-`st_shndx` trigger in the same bug class, per CONFIRMATION.md section
7) also re-run against the patched build - clean link, exit 0.

- Project test suite (`ctest --test-dir build-plain -j4`): **476/476 passing before**,
  **476/476 passing after** (no new failures, no fewer skips - the same 16 tests were
  skipped both times, all pre-existing environment/feature gating unrelated to this
  change: apx/ifunc/incompatible-libs/sframe/zstd/hwasan-etc.).

## What's not done / open questions

- The second, independent `symtab_shndx_sec`-too-short trigger mechanism (an `SHN_XINDEX`
  symbol whose index exceeds a genuinely short `SHT_SYMTAB_SHNDX` section, as opposed to
  variant 1's approach of just putting a huge value *in* a correctly-sized shndx table)
  was not separately repro'd as its own standalone crash - the fix's `idx <
  symtab_shndx_sec.size()` check covers it, and the underlying bug class (an unchecked
  index into a small container) is the same one variants 1/2 already demonstrate, so a
  third repro variant was judged low marginal value for the time budget.
- The ARM32-only `.arm.exidx` attachment path and the `SHT_CREL` path in
  `initialize_sections()` were not touched by this patch - they don't call `get_shndx()`
  and were already fixed independently by candidate 010's patch (unrelated `sh_info`
  fields, not `st_shndx`).
- No `SHN_ABS`/`SHN_COMMON` edge case was separately tested, since every real call site
  already guards `get_shndx()` calls with `esym.is_abs()`/`esym.is_common()` checks
  first (confirmed by reading each call site) - `get_shndx()` is only ever invoked with
  esym values where a section index is actually meaningful.
