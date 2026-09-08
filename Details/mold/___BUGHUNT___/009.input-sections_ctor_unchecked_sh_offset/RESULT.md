# input-sections_ctor_unchecked_sh_offset

`InputSection<E>::InputSection()` (`src/input-sections.cc`) sets
`contents = file.mf->data + shdr().sh_offset` directly from the untrusted, file-controlled
`sh_offset` field with no validation whatsoever, and (for non-compressed sections) sets
`sh_size = shdr().sh_size` similarly unvalidated. This was an already-flagged backlog item
(`src/input-sections.cc:34-53`) from the prior static audit; investigated this session and found
to be the single highest-value remaining finding - the resulting wild `contents` pointer is
dereferenced during the final output-writing phase for **any** SHF_ALLOC section with a bogus
`sh_offset`, no relocations, symbols, or special section types required at all.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Complex construction attempt (relocation into a mergeable section, per `input-files.cc:1082`'s `get_addend(*isec, r)` call) | Hand-built an i386 object with a `.text` section (lying `sh_offset`), a `.rodata.str` mergeable section, and a relocation redirecting through `get_addend()` | Crashed, but investigation via `gdb` showed the actual fault was NOT in `get_addend` - it was in `copy_contents_to()` during output writing, reached independently of the relocation/mergeable-section machinery |
| Minimal construction (no relocations, no symbols, one bogus SHF_ALLOC section) | 192-byte i386 `ET_REL` object: NULL + `.text` (SHF_ALLOC, `sh_offset=0x7FFFFFF0`) + `.shstrtab` | Real, deterministic `SIGSEGV` - confirms the complex construction above was unnecessary; this is the true minimal trigger |
| `gdb` backtrace (`set follow-fork-mode child`) | `run -m elf_i386 --thread-count 5 -shared -o out evil.o` | `Thread ... received signal SIGSEGV` in `mold::InputSection<mold::I386>::copy_contents_to` at `src/input-sections.cc:82` (`memcpy(buf, contents, sz)`), called from `write_to` -> `OutputSection::write_to`'s TBB `parallel_for` -> `copy_buf` -> `copy_chunks()` |
| ASAN+UBSAN build (g++ 15.2.0) | Same minimal repro | Real `SIGSEGV` (exit 139) - notably, **no ASAN diagnostic text was printed** despite this being the ASAN-instrumented binary (the fault occurs inside a TBB worker thread during parallel output-copying; the plain kernel-delivered `SIGSEGV`/exit 139 is still solid evidence, confirmed identically via a real `gdb` backtrace and via the plain build below) |
| Plain release build (no sanitizer) | `-DCMAKE_BUILD_TYPE=Release`, `build-plain/mold` | Real `Segmentation fault` (exit 139) - confirms this isn't an ASAN-only artifact |
| Control inputs | A real object compiled with `gcc -fexceptions` (genuine `.eh_frame`), and the real system `/usr/lib/x86_64-linux-gnu/libc.a` | Both link successfully before and after the fix |

## Status

**Confirmed** - 5/5 runs, byte-for-byte identical crash (exit 139) every time; also confirmed via a
real `gdb` backtrace pinpointing the exact faulting instruction and line
(`src/input-sections.cc:82`), and independently via a plain, non-instrumented release build.
Fully deterministic (the fault address is a fixed distance from the real mapping, independent of
ASLR/memory layout, since it only depends on the fabricated `sh_offset` constant).

## Fully reproducible

Yes. `repro/run.sh` builds the malformed object with `repro/gen_evil_elf.py` and runs the real
`mold` binary via its real CLI entry point end to end (`mold -shared -o <out> evil.o` -> full
input processing -> output writing -> `copy_contents_to`).

## Class

crash (memory-safety: out-of-bounds read via a completely unvalidated, attacker-controlled pointer
constructed from a raw file offset)

## Root cause

Every `InputSection` object's `contents` pointer and `sh_size` are taken directly from the
corresponding ELF section header's `sh_offset`/`sh_size` fields - both fully attacker-controlled
for any object file mold parses - with **no bounds check against the mapped file's real size at
all**. This is more fundamental than any single call site that reads through `contents`: it
affects every consumer of `InputSection::contents`/`sh_size` uniformly, including (as found and
confirmed this session) the universal output-writing path
(`InputSection::copy_contents_to`'s `memcpy(buf, contents, sz)`, reached for every SHF_ALLOC
section in every input object during the final output-copy phase, entirely independent of
relocations or symbol resolution) and (not separately re-verified, but structurally identical)
every relocation-addend read via `get_addend(InputSection&, ...)`
(`isec.contents + rel.r_offset`, `src/mold.h:3232`) for REL-type architectures. This is the
section-level analogue of candidate 005's archive-member-size-lie bug: a length/offset pair
trusted at construction time, with every later consumer inheriting the lie.

## Introduced by

046101eda "Store input section contents as a pointer" (2026-07-28) - this commit changed
`InputSection::contents` from a bounds-safe accessor into a raw pointer computed once at
construction time with no validation, per `git blame`. Notably recent relative to most of this
session's other findings (which trace to 2022-2023) - a genuine regression, not merely an
old, never-revisited code path.

## The fix

`patch.diff` (`src/input-sections.cc` only): added two checks in the constructor, at the exact
points `sh_offset` and `sh_size` are first used:

- `shdr().sh_offset > (u64)file.mf->size` -> `Fatal()` before `contents` is ever computed, closing
  off the wild-pointer construction itself.
- For non-compressed sections: `shdr().sh_type != SHT_NOBITS && shdr().sh_size > (u64)file.mf->size - shdr().sh_offset`
  -> `Fatal()`. **`SHT_NOBITS` (`.bss`-like) sections are explicitly exempted** - they occupy no
  space in the file at all, so a legitimately huge `sh_size` (e.g. a multi-gigabyte `.bss` from
  `-mcmodel=large`) has nothing to validate against the file's real size. **This exemption was
  discovered the hard way**: the first fix attempt (no `SHT_NOBITS` exemption) passed the crafted
  repro but broke 3 real tests (`x86_64-arch-x86_64-gotpcrelx`, `x86_64-arch-x86_64-large-bss`,
  `x86_64-arch-x86_64-tls-large-tbss`) with `section is out of range: sh_offset=232
  sh_size=3221225472` - a real `.bss` section correctly declaring a 3GB size with a tiny
  `sh_offset` and no real file backing. Caught immediately by running the full test suite before
  finalizing, exactly the scenario `SKILL.md`'s candidate loop step 7 exists to catch.

Verified: `repro/run.sh` no longer crashes against the patched build - `mold` now exits 1 with a
controlled `mold: fatal: <path>: <section>: section offset is out of range` (or `section is out of
range: sh_offset=... sh_size=...` for the size half) message. Also re-tested linking a real object
with genuine `.eh_frame` content and the real system `libc.a` - both link successfully with the
patch applied.

- Project test suite (`ctest -j5` in `build-asan/`): 475/476 passed before and after the final
  (NOBITS-exempted) patch, identical single pre-existing, patch-unrelated failure both times
  (`x86_64-run-clang` - see every prior candidate's own notes and `projects/mold.md`). The
  intermediate, non-exempted fix attempt regressed 3 tests as described above; the final patch
  fixes all of them.

## What's not done / open questions

- Only the `copy_contents_to`/output-writing consumer of the unvalidated `contents`/`sh_size` was
  built and run as a repro this session. `get_addend(InputSection&, const ElfRel<E>&)`
  (`src/mold.h:3232`, `isec.contents + rel.r_offset`, used by REL-type architectures like i386/
  arm32/sh4 to read relocation addends from section data) is structurally identical and should
  already be covered by this same fix (since it also reads through the now-validated `contents`
  pointer) - not separately re-verified with its own dedicated repro, since the fix is at the
  single shared root cause (the constructor), not per-consumer.
- Did not check whether `InputSection::uncompress()` (`src/input-sections.cc:67-77`, allocates
  `new u8[sh_size]` then calls `copy_contents_to`) has its own additional exposure for a
  compressed section with a similarly-lying `ElfChdr::ch_size` (read from `contents` itself, which
  is now bounds-checked via `sh_offset`, but `ch_size` is a separate field inside the section data,
  not validated against anything) - a compressed-section variant of this same bug shape is a
  plausible next candidate, not investigated this session.
