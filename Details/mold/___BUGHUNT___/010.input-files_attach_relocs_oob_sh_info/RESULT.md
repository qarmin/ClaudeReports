# input-files_attach_relocs_oob_sh_info

`___BUGHUNT___/IDEAS.md` backlog item ("src/input-files.cc:362-367... uses sh_info and sh_link of
relocation/EXIDX sections as section indices without any range check", from a prior
`code-audit-structured` audit pass) plus a second, higher-value sibling site found while reading
the same function: `ObjectFile<E>::initialize_sections()`'s "Attach relocation sections to their
target sections" loop (`src/input-files.cc:509-516`) indexes `sections[shdr.sh_info]` with no
bounds check at all, where `sections` is an `InputSectionTable<E>` backed by a small
`std::vector<u32> indices` sized to `elf_sections.size()`. `sh_info` on a `SHT_REL`/`SHT_RELA`/
`SHT_CREL` section is the index of the section the relocations apply to - fully attacker-controlled,
copied straight from the file. This site is reachable by *any* relocatable object file with a
`.rela.*`/`.rel.*` section, no special CLI flag or unusual ELF feature (COMDAT groups, `--gdb-index`,
`--relocatable`) needed - the most directly reachable of the three sibling sites in this function.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `src/input-files.cc:509-527`, cross-referenced against `InputSectionTable<E>::operator[]` (`src/mold.h:3554-3560`, backed by `std::vector<u32> indices`, resized to `elf_sections.size()` at `src/input-files.cc:1137`) | Confirmed `shdr.sh_info` is used as a raw vector index with zero validation anywhere upstream on this path |
| Crafted-input repro against real `mold` binary | `build-plain/mold` (plain debug build, no sanitizers), minimal x86_64 ET_REL object with one `.rela.text` section, `sh_info = 0xFFFFFFF0` | SIGSEGV, 5/5 runs |
| `gdb` backtrace (`set follow-fork-mode child`, per README's fork_child gotcha) | same input against `build-plain/mold` | Crash pinpointed exactly to `InputSectionTable<X86_64>::operator[]`, called from `ObjectFile<X86_64>::initialize_sections`, called from `parse_sections` - matches the suspected mechanism precisely |
| Same repro against `build-asan/mold` (ASAN+UBSAN, g++, `_GLIBCXX_ASSERTIONS` active) | `-DMOLD_USE_ASAN=ON` build | `std::vector::operator[]`'s libstdc++ hardening assertion fires (`Assertion '__n < this->size()' failed`), `SIGABRT` (exit 134) - a different, complementary diagnostic confirming the same OOB index, not a plain segfault this time since the vector's own bounds-checked debug instrumentation caught it first |
| First attempt with a smaller malicious `sh_info` (`0xFFFFFF`, ~64MB past the allocation) | `build-plain/mold` | Did **not** crash - exited 0 with only mold's unrelated "entry symbol is not defined" warning; the garbage value read from that offset happened to compare `== 0`/return `nullptr`. Increased to `0xFFFFFFF0` (several GB past the allocation) to reliably land in unmapped memory - worth remembering as its own lesson, see `projects/mold.md` |

## Status

**Confirmed** - reproduces 5/5 runs against the plain build (SIGSEGV every time), and reproduces
identically (as a libstdc++ hardening `SIGABRT`) against the ASAN/UBSAN build. Both failure modes
were captured with real command output in `output.txt`.

**Fully reproducible**: Yes. `repro/run.sh` runs the real `mold` binary end-to-end against a
crafted-but-otherwise-ordinary ET_REL object file - the exact kind of input `mold` is designed to
link, no faked second participant or hand-simulated internal state involved.

## Class

crash (memory-safety: out-of-bounds `std::vector` read via an unvalidated file-controlled index)

## Root cause

`ObjectFile<E>::initialize_sections()` has a loop, run unconditionally for every parsed object
file, that attaches each relocation section to the section it relocates:

```cpp
for (i64 i = 0; i < this->elf_sections.size(); i++) {
  const ElfShdr<E> &shdr = this->elf_sections[i];
  if (shdr.sh_type == (E::is_rela ? SHT_RELA : SHT_REL) || shdr.sh_type == SHT_CREL) {
    if (InputSection<E> *target = sections[shdr.sh_info]) {   // <-- shdr.sh_info unchecked
      ...
```

`shdr.sh_info` is a raw 32-bit field copied directly from the object file with no validation
anywhere on this path. `sections` is an `InputSectionTable<E>`, whose `operator[]` (`src/mold.h:
3554-3560`) indexes straight into `std::vector<u32> indices` (sized to `elf_sections.size()` - a
handful of entries for a small object) with `indices[idx]`, no bounds check. A relocation section
whose `sh_info` names a huge, out-of-range section index reads far outside that vector's backing
allocation. Two adjacent sibling sites in the same function share the identical shape:
- the `SHT_CREL` branch a few lines earlier (`src/input-files.cc:362-366`) reads
  `this->elf_sections[shdr.sh_info].sh_flags` with the same unchecked `sh_info`;
- the `.arm.exidx`-attachment loop right after (`src/input-files.cc:521-526`, ARM32-only) reads
  `sections[isec->shdr().sh_link]` with the same unchecked pattern on `sh_link` instead of
  `sh_info`.

Both were fixed alongside the primary finding as the same bug class in the same function (see "The
fix" below), though only the primary `SHT_RELA` site was independently built into its own crafted
repro this session - see "What's not done" below.

## Introduced by

not determined - `git log -S`/pickaxe on the "Attach relocation sections to their target sections"
comment and on the `sections[shdr.sh_info]` expression both bottom out at commit 2b974da2
("temporary", 2020-10-20), one of mold's earliest bulk-squash development commits with no real
subject line - this predates any commit history worth bisecting further. The bug shape has
apparently been present since very close to the project's original ELF-linking implementation.

## The fix

Bounds-check the untrusted index against `this->elf_sections.size()` at all three sibling sites
before using it, mirroring the existing bounds-checked pattern already used elsewhere in the same
file (e.g. `src/input-files.cc:293`, `if (shdr.sh_info >= this->elf_syms.size())` for COMDAT group
signatures):

- `SHT_CREL` branch: raise `Fatal(ctx)` on an out-of-range `sh_info` (this branch already does
  real, non-trivial work - `decode_crel()` - if allowed to proceed with an invalid index, so
  refusing outright is more defensible than silently skipping).
- The relocation-attachment loop and the ARM EXIDX-attachment loop: fold the range check into the
  existing `if` condition, so an out-of-range index is treated the same way the code already
  treats a *valid* index whose slot happens to be empty (`sections[idx]` returning `nullptr`) -
  i.e., the relocation/EXIDX section is simply left unattached, no `Fatal()` needed since this
  matches an existing, already-tolerated outcome shape.

Verified: `repro/run.sh` no longer crashes against either the patched plain build or the patched
ASAN build (mold now links successfully, modulo its own unrelated "entry symbol is not defined"
warning for this deliberately entry-less test object) - see `output.txt`'s "after patch.diff
applied" section, 3/3 runs each build.

- Project test suite (`ctest -j5` in `build-asan/`): **before patch: 475/476 passed** (1 known
  pre-existing failure, `x86_64-run-clang` - documented in `___BUGHUNT___/README.md` as failing on
  unpatched HEAD regardless of source changes, an ASAN/`LD_PRELOAD` interaction, not a regression
  signal). **After patch: 475/476 passed**, same single pre-existing failure, no new failures.

## What's not done / open questions

- The `SHT_CREL` branch (`src/input-files.cc:362-366`) and the ARM32 `.arm.exidx`-attachment loop
  (`src/input-files.cc:521-526`) were fixed as the same bug class in the same patch, but neither
  was independently confirmed with its own crafted-input repro this session - `SHT_CREL` is an
  experimental relocation-table encoding (lower real-world prevalence than plain `SHT_RELA`), and
  the EXIDX path is ARM32-only (this host/repro toolchain targets x86_64). A future session could
  build a targeted repro for either if an independent confirmation is wanted; both share the exact
  same `indices[idx]`-without-bounds-check mechanism already confirmed for the `SHT_RELA` case, so
  the risk of the fix being wrong for those two branches specifically is low, but it is unverified.
- Not checked whether any *other* function in this codebase reads `sh_info`/`sh_link` off a
  relocation-type section without validation outside `initialize_sections()` itself (a grep for
  other `sh_info`/`sh_link` uses on `SHT_REL`/`SHT_RELA`/`SHT_CREL` sections specifically, as
  opposed to the broader sh_info/sh_link-as-section-index sweep already partially covered by other
  `IDEAS.md` backlog items, e.g. the `src/input-files.cc:296-302` `get_shndx()` item).
