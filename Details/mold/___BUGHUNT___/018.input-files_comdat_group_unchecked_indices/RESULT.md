# input-files_comdat_group_unchecked_indices

`src/input-files.cc` (`ObjectFile<E>::parse_sections()`) and `src/mold.h`
(`ComdatGroupRef<E>::members()`) both consume the raw `u32` member-section-index entries of an
`SHT_GROUP` section without validating them against `sections.size()`/`elf_sections.size()`. This
was flagged by a prior static-audit pass (`code-audit-structured`, 2026-08-13) and picked up here
for real repro/fix work (retrying candidate 015, which was killed by an unrelated infra
interruption before any repro was built).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `read_section_metadata()`, `parse_sections()` (input-files.cc) and `ComdatGroupRef<E>::members()` (mold.h) at current commit c340272e | static read | Confirmed both locations match the audit description; found the same unchecked-index pattern read a *third* time in `src/passes.cc`'s `parse_input_sections()` apply-selection loop (`file->sections[i]`) and a *fourth* time in `src/relocatable.cc`'s `create_comdat_group_sections()` |
| Traced why `ComdatGroupRef<E>::members()` (finding B, raw span over mmap with no extent check) can't independently fault in the current call graph | static read | `members()` is only ever called with a `sect_idx` whose `shdr` already passed the checked `get_data<U32<E>>(ctx, shdr)` call earlier in `read_section_metadata()` (same shdr, same object, no mutation in between) - so finding B is real UB in isolation (the code pattern is genuinely unchecked) but currently unreachable as an independent crash; only finding A (the unvalidated *values* inside an already-well-formed group) is live |
| Crafted 2 minimal x86_64 ET_REL `.o` files (`good.o`/`evil.o`) sharing one GLOBAL symbol "foo" as a COMDAT signature, `evil.o`'s `.group` section body containing one malicious out-of-range member index (`0xFFFFFFF0`); ran real `mold` (`-DCMAKE_BUILD_TYPE=Debug` plain build) on `good.o evil.o` | plain Debug build, real mold binary, real CLI | **Crashed**: `std::vector<bool>::operator[]` hardened-assertion abort (SIGABRT, exit 134), 5/5 runs |
| Repeated with a second, different malicious index in the same bug class: exactly `sections.size()` (off-by-one boundary, not the original huge value) | same build | **Crashed** the same way, 3/3 runs |
| Applied a bounded fix (see below), rebuilt, re-ran both repro variants | plain Debug build | Both variants now exit cleanly with `mold: fatal: ...: invalid section index in SHT_GROUP` (exit 1), 5/5 and 3/3 respectively |
| Full project test suite (`ctest --test-dir build-plain -j4`), before and after the patch | plain Debug build | 476/476 passed both times, including the project's own real-COMDAT-group tests (`arch-x86_64-comdat-signatures`, `arch-x86_64-comdat-early-elimination`, `lto-comdat`, `lto-comdat-archive`, `arch-x86_64-lto-comdat-mixed`) |
| Standalone `justfile` (`just test`): fresh clone of c340272e, build, repro-before (crash), apply patch, rebuild, repro-after (clean) | plain Debug build in a fresh clone, independent of the dev worktree | Full cycle passed end to end |

## Status

**Confirmed**, reproduces 5/5 runs (original 0xFFFFFFF0 index) and 3/3 runs (boundary-value
variant, index == `sections.size()`), plain `-DCMAKE_BUILD_TYPE=Debug` build. The crash is a
libstdc++-hardened-assertion abort (`_GLIBCXX_ASSERTIONS`, which this project's own
`CMakeLists.txt` enables for Debug builds) catching `std::vector<bool>::operator[]` being called
with an out-of-range index - i.e. the assertion is standing in for what would otherwise be a
silent out-of-bounds *write* into the packed bit storage backing `comdat_discarded`
(`std::vector<bool>`) on a build without libstdc++ hardening (a typical Release/production build).

**Fully reproducible**: **Yes.** The repro (`repro/gen_evil_elf.py` + `repro/run.sh`) builds two
genuine ET_REL ELF object files and runs the real `mold` binary's real CLI entry point
(`mold -o out.exe good.o evil.o`) end to end - no internal function is called directly, no
participant is faked. `evil.o`'s malicious `SHT_GROUP` section is a completely ordinary,
correctly-sized, well-formed section (it passes every other section-header/extent check in the
file, including the `entries.empty()` check on the same object) - only the *value* of its one
member-section-index entry is malicious. This models exactly the untrusted input mold's whole job
is to parse: a malformed but structurally-valid relocatable object file handed to the linker on
the command line.

## Class

memory-safety

## Root cause

`ObjectFile<E>::read_section_metadata()` (src/input-files.cc) reads an `SHT_GROUP` section's body
via the checked `get_data<U32<E>>(ctx, shdr)` accessor (so the section's own *extent* - offset/size
against the mmap - is validated), checks `entries[0]` (the `GRP_COMDAT` flags word), and then
stores every subsequent entry unchanged as a "member section index" in the file's
`comdat_groups` list - without ever checking that those `u32` values are themselves valid indices
into `elf_sections`/`sections`. Those values are then consumed in two (really three, counting
`relocatable.cc`) places without a second bounds check:

1. `ObjectFile<E>::parse_sections()` (src/input-files.cc):
   `comdat_discarded[i] = true` where `comdat_discarded` is a `std::vector<bool>` sized to this
   object's own (small) section count. `std::vector<bool>::operator[]` computes a raw bit offset
   with no bounds check; a huge `i` (or even just `i == sections.size()`) walks the write far
   outside the vector's backing allocation.
2. `parse_input_sections()` (src/passes.cc): `file->sections[i]` (a small `std::vector`/table)
   indexed the same way, feeding a possibly-garbage pointer into `isec->is_alive = ...`.
3. `create_comdat_group_sections()` (src/relocatable.cc, only used under `-r`/`--relocatable`):
   the same `file.sections[j]`/`file.elf_sections[j]` pattern.

`ComdatGroupRef<E>::members()` (src/mold.h) - the accessor all three call sites use to iterate
these entries - additionally reconstructs its `std::span` directly from
`file.mf->data + shdr.sh_offset` / `shdr.sh_size` with **no** extent check of its own (an `assert`
that's compiled out under `NDEBUG`/release stood in for one). In the current code this specific
span-construction path can't independently fault, because every `sect_idx` passed to `members()`
was already validated via the checked `get_data<U32<E>>()` call in `read_section_metadata()` for
the exact same `shdr` - but it's still a latent landmine (raw pointer arithmetic duplicating a
check that already happened, ready to break the moment a caller passes an `sect_idx` that hasn't
gone through that path).

## Introduced by

7813baa40f656496e035e754988a9afb65da78ee "Avoid building discarded COMDAT sections" (2026-07-25)
introduced the vulnerable `comdat_discarded.resize(...)` / `comdat_discarded[i] = true` write in
`parse_sections()` and the equivalent `file->sections[i]` read in the passes.cc apply-selection
loop, as part of a performance refactor deferring COMDAT-member section construction. At that
point `ComdatGroupRef` still stored an already-validated `std::span<U32<E>>` (populated via the
checked `get_data()` accessor) directly as a member field, so span *extent* was safe even though
the *values* inside it were (and still are) never checked against `elf_sections.size()`.

5f7764124c7c7eb6f18909cf3f708fe1957aaccc "Use 32-bit references for linker metadata" (same day,
~34 minutes later) then replaced that stored, already-checked span with the current
`ComdatGroupRef<E>::members(ObjectFile<E>&)` accessor that reconstructs the span lazily via raw
`file.mf->data + shdr.sh_offset` pointer arithmetic (a memory-footprint optimization, shrinking
`ComdatGroupRef` to 8 bytes) - this is what introduced finding (B), the unchecked-extent span
reconstruction, on top of (A)'s still-unvalidated member values.

## The fix

Two changes, matching the two findings:

- **(A)** Added a single bounds check in `read_section_metadata()`, right where an `SHT_GROUP`'s
  entries are first read (after the existing `entries[0]` flag checks, before the group is ever
  added to `comdat_groups`): every subsequent entry (`entries.subspan(1)`) must be
  `< this->elf_sections.size()`, else `Fatal(ctx) << *this << ": invalid section index in
  SHT_GROUP"`. This is a single authoritative check point that covers every consumer
  (`parse_sections()` in input-files.cc, the apply-selection loop in passes.cc, and
  `create_comdat_group_sections()` in relocatable.cc) without needing a bounds check duplicated
  at each call site.
- **(B)** Changed `ComdatGroupRef<E>::members()` to take a `Context<E>&` and use the checked
  `file.get_data<U32<E>>(ctx, shdr)` accessor instead of raw `file.mf->data + shdr.sh_offset`
  pointer arithmetic, with an explicit `Fatal(ctx) << file << ": empty SHT_GROUP"` replacing the
  compiled-out-under-NDEBUG `assert(!entries.empty())`. Threaded `Context<E>&` through the three
  call sites (`input-files.cc:1143`, `passes.cc:393` and `passes.cc:3696` (stats), and
  `relocatable.cc:86`), all of which already had `ctx` in scope.

Verified against **two** inputs in the same bug class (per CONFIRMATION.md section 7): the
original `0xFFFFFFF0` value and a second, independently-constructed boundary case
(`member index == elf_sections.size()`, i.e. exactly one past the last valid index) - both crash
identically on the unpatched tree and both are cleanly rejected (`mold: fatal: ...: invalid
section index in SHT_GROUP`, exit 1) on the patched tree.

- Project test suite: **476/476 passed before, 476/476 passed after** (`ctest --test-dir
  build-plain -j4`, plain `-DCMAKE_BUILD_TYPE=Debug` build) - no new failures, no regressions. The
  suite includes real COMDAT-group tests exercising legitimate C++-template-instantiation-style
  groups (`arch-x86_64-comdat-signatures`, `arch-x86_64-comdat-early-elimination`, `lto-comdat`,
  `lto-comdat-archive`, `arch-x86_64-lto-comdat-mixed`); all continued to pass, confirming the new
  checks don't reject well-formed groups.

## What's not done / open questions

- Finding (B) (the raw-span-over-mmap construction in `members()`) was fixed as requested even
  though it's currently unreachable as an *independent* crash in this codebase's actual call
  graph (every current caller's `sect_idx` already passed the checked `get_data()` call earlier
  for the same `shdr` in `read_section_metadata()`) - it's still worth fixing defensively since it
  duplicates a check via unchecked raw pointer arithmetic instead of the project's own established
  checked-accessor pattern, and a future caller of `members()` that doesn't go through
  `read_section_metadata()` first would otherwise silently reintroduce the bug.
- The ARM32-only `.arm.exidx` attachment path and the `SHT_CREL`-decoding branch in
  `initialize_sections()` were not touched by this candidate (already fixed by an earlier
  candidate, 010) - no interaction expected, but not independently re-verified here.
- Did not attempt to construct a repro that reaches finding (B) via a route that bypasses
  `read_section_metadata()`'s own extent check (e.g. calling `members()` from a hypothetically
  different, not-yet-existing caller) - the fix was applied on the strength of code reading plus
  matching the project's own established accessor pattern, not a demonstrated independent crash
  for (B) specifically.
- Did not fuzz this area beyond the two hand-constructed boundary values described above; a short
  fuzzing campaign over `SHT_GROUP` member-index values (via `fuzz-bug-repro`, run separately)
  would be reasonable follow-up but wasn't judged necessary given the bug's index-arithmetic
  simplicity (any out-of-range value up to and including exactly `elf_sections.size()` triggers
  it, confirmed for two representative values on opposite ends of that range).
