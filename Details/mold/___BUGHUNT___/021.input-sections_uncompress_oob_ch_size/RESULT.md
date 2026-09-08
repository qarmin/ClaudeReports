# input-sections_uncompress_oob_ch_size

`src/input-sections.cc`'s `InputSection<E>` constructor reads a `SHF_COMPRESSED` section's
`Elf64_Chdr` compression header straight from the untrusted file and stores its `ch_size` field
verbatim as the section's decompressed size, with no upper bound. This was a lead from a prior
static-audit pass (`___BUGHUNT___/IDEAS.md`, seeded 2026-08-13), retried after two earlier attempts
(candidates 015/019) on this same suspect were killed by an unrelated infra safety-filter
interruption before any real investigation started.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `src/input-sections.cc` (constructor lines 46-53, `uncompress()` lines 68-77, `copy_contents_to()` lines 80-130) to find exactly what's validated before the `ElfChdr` cast and before any allocation | static read only | Confirmed: constructor casts `ElfChdr<E>*` over `contents` with **no** check that the section is >= `sizeof(ElfChdr<E>)` bytes; `sh_size = chdr.ch_size` has **no** upper-bound check at all; `uncompress()`'s very first statement is `new u8[sh_size]`, called **before** `copy_contents_to()`'s own "corrupted compressed section" size check ever runs |
| Crafted a minimal x86_64 ET_REL object, one `.rodata.evil` section with `SHF_MERGE\|SHF_STRINGS\|SHF_COMPRESSED`, content = well-formed `Elf64_Chdr` (`ch_type=ELFCOMPRESS_ZLIB`) + `ch_size=0xFFFFFFFFFFFF0000`, linked with `mold -shared -o out.so evil.o` | plain debug build (`build-plain/`, `-DCMAKE_BUILD_TYPE=Debug`) | **First attempt used a shared `/tmp/evil.o` path and got a false result** - a sibling parallel-batch candidate in another worktree overwrote the file mid-session, so the crash observed initially was a real bug in a completely different function (`parse_fde_encoding`, the known candidate-008 bug) rather than this one. Caught it by getting a `gdb` backtrace and cross-checking the file's actual on-disk bytes against what the generator wrote. Moved all repro work to a private directory inside this worktree and re-ran - see below. |
| Same repro, private worktree-local directory (no shared-`/tmp` collision) | plain debug build | **Crashed for real**: SIGABRT (exit 134), no stdout/stderr output at all. `gdb` (with `set follow-fork-mode child`, since mold forks a child to do the actual link) confirmed the exact mechanism: `mi_new(size=18446744073709486080)` (mimalloc's `operator new` implementation) called from `InputSection<X86_64>::uncompress()` at `src/input-sections.cc:72` (`u8 *buf = new u8[sh_size];`), called from `MergeableSection<X86_64>::MergeableSection()` at `src/input-sections.cc:367`, called from `ObjectFile::convert_mergeable_sections()` (`src/input-files.cc`), called unconditionally by `create_merged_sections()` (`src/passes.cc:521`) on every link - no special CLI flag needed. |
| Repeat-run the same repro 8 times total (5 initial + 3 more) against the same pristine build | plain debug build | **8/8 identical**: SIGABRT, exit 134, zero stdout/stderr output every time. |
| Tried 2 more `ch_size` magnitudes to see how the failure mode varies | plain debug build | `0xFFFFFFFFFFFF0000` (~1.8e19): immediate, clean SIGABRT via mimalloc's failed-allocation handler (`mi_try_new_handler` -> `abort()`) - no exception thrown, nothing catchable. `0x10000000000` (1 TiB): mimalloc's `new` **succeeds** as a virtual-memory-only reservation (host has ~17GiB available RAM but Linux permits a large lazy/sparse `mmap`-backed reservation regardless) - the process then spins at ~100% CPU with tiny RSS, doing real work inside the (bogus) zlib-decompression loop rather than crashing; killed manually after confirming it does not terminate quickly. This matches the "harmless-looking but still a real DoS" branch anticipated in the assignment - a moderate-huge value doesn't crash cleanly, it hangs. `0x100000000` (4 GiB) similarly did not fail instantly (not conclusively characterized - killed alongside the 1TiB case for host safety before it was individually isolated). |
| Applied a bounded fix (see below), rebuilt, re-ran the same repro 3 times | plain debug build, patched | 3/3: clean `Fatal()` error message, exit 1, no crash. |
| Constructed one more, different malicious input in the same bug class: a different huge `ch_size` (`0x123456789ABC`) | plain debug build, patched | Also caught cleanly by the same check (`ch_size is unreasonably large`), confirming the fix isn't narrowly tuned to the one exact triggering value (CONFIRMATION.md section 7). |
| Constructed a second, related input: a `SHF_COMPRESSED` section whose on-disk `sh_size` (8 bytes) is smaller than `sizeof(Elf64_Chdr)` (24 bytes) - the "truncated header" secondary defect named in the assignment | plain debug build, patched | Caught cleanly by the new `shdr().sh_size < sizeof(ElfChdr<E>)` check (`corrupted compressed section`, exit 1). Not independently confirmed to crash the **pristine** binary (would need the section to end near a real mapping/page boundary to reliably fault, per the project's own mmap-page notes) - fixed defensively anyway since it's the same code path and explicitly named in the assignment, but not claimed as an independently-reproduced crash. |
| Full project test suite, before and after the fix | `ctest --test-dir build-plain -j4` | 476/476 passed both before (pristine) and after (patched) - see `tests_before`/`tests_after` below. Specifically checked `x86_64-compress-debug-sections`, `x86_64-compress-debug-sections-zlib-level`, `x86_64-compress-debug-sections-zstd-level` (real, legitimately-compressed debug sections via `--compress-debug-sections`) - all **Passed** with the fix applied, confirming the new checks don't reject real compressed input. (`x86_64-compressed-debug-info`/`-zstd` variants were Skipped in both runs - missing `dwarfdump`/`zstd` CLI tools on this host, not caused by the patch.) |

## Status

**Confirmed.**

Reproduces **8/8 runs**, plain debug build (`-DCMAKE_BUILD_TYPE=Debug`), on the assigned base
commit `c340272e6376f9a8e5b5cd6f6634f925f6d54e5c`. Deterministic - same exit code (134/SIGABRT),
same (empty) output, same `gdb` backtrace every time.

**Fully reproducible: Yes.** The repro builds a real, minimal, malformed `.o` file (the genuine
external artifact an attacker/build-system input would be) and feeds it to the real `mold` binary
via its real CLI (`mold -shared -o out.so evil.o`) - no internal state is hand-constructed and
called directly; this is a single-participant case (a malformed file handed to a parser's real
entry point) per `CONFIRMATION.md` section 11, so no second-participant concern applies.

## Class

crash (also security/DoS-shaped: no CLI flag needed, any relocatable object with a mergeable
compressed section triggers it during an ordinary link).

## Root cause

`InputSection<E>::InputSection()` (`src/input-sections.cc:46-49`, current HEAD) reads
`sh_size = chdr.ch_size` straight from a `SHF_COMPRESSED` section's `Elf64_Chdr` header - a fully
untrusted, attacker-controlled 64-bit value from the file - with **no upper-bound check of any
kind**. If the same section also carries `SHF_MERGE`, `ObjectFile<E>::convert_mergeable_sections()`
(`src/input-files.cc`), called unconditionally during every link by `create_merged_sections()`
(`src/passes.cc:521`), constructs a `MergeableSection` for it, whose constructor
(`src/input-sections.cc:367`) immediately calls `isec->uncompress(ctx)`. That function's very
first statement is `u8 *buf = new u8[sh_size];` (`src/input-sections.cc:72`) - i.e.
`new u8[<attacker-controlled 64-bit value>]` - executed **before** any of `copy_contents_to()`'s
own size/decompression-consistency checks ever run. mold links mimalloc as its allocator; when
this allocation request is implausibly large, mimalloc's `mi_try_new_handler` calls `abort()`
directly rather than mold ever getting a chance to catch a `std::bad_alloc` (mold builds with
`-fno-exceptions` in several places / never wraps this path in a `try`), so the process aborts
outright with no diagnostic on stdout/stderr.

A second, related defect in the same constructor (assigned as the harder secondary check): the
`ElfChdr<E> &chdr = *(ElfChdr<E> *)contents;` cast at line 47 happens with no check that the
section's raw on-disk `sh_size` is even large enough to contain the 24-byte `Elf64_Chdr` header in
the first place - an out-of-bounds read at construction time, before `copy_contents_to()`'s own
`view.size() < sizeof(ElfChdr<E>)` check (which exists, but much later and on a different code
path) ever has a chance to catch it.

## Introduced by

`30683645c247aa47146d97ecf19b6de2436deb32` "[ELF] Do not corrupt compressed debug sections on
i386" (2022-02-24) - this is the commit that introduced the exact `new u8[sh_size]` allocation
(originally directly in the `InputSection` constructor, for REL-type archs) using the unbounded
`sh_size` value with no cap. The underlying unchecked read itself (`sh_size = chdr.ch_size` with
no validation) is even older, from `7fcff98f290691efadb2432c12d40a15640eda40` "[ELF] Directly
uncompress debug sections into the output mmap'ed buffer" (2022-02-01) - at that point `uncompress()`
wrote into a caller-supplied `buf` rather than allocating internally, so the unbounded value didn't
yet drive its own allocation. Both commits are from the pre-rename `elf/input-sections.cc`; a later
"Reorganize directory structure" commit (`5b859497`) moved the file to its current `src/` path
without touching this logic. Neither is a recent regression - this bug shape has been present for
roughly four years.

## The fix

Added two checks to `InputSection<E>::InputSection()` right where the `SHF_COMPRESSED` branch
reads the compression header (`src/input-sections.cc`):

1. `if (shdr().sh_size < sizeof(ElfChdr<E>)) Fatal(ctx) << ... << ": corrupted compressed section";`
   before the `ElfChdr<E>` cast - rejects a section too short to even hold the header, matching the
   error message `copy_contents_to()` already uses for the same condition on its own (later,
   narrower) code path.
2. `if (chdr.ch_size > MAX_UNCOMPRESSED_SIZE) Fatal(ctx) << ... << ": ... ch_size is unreasonably large";`
   after reading `ch_size`, before it's stored into `sh_size` (and therefore before it can ever
   reach `uncompress()`'s `new u8[sh_size]`). `MAX_UNCOMPRESSED_SIZE` is a new file-local constant,
   `1ULL << 32` (4 GiB) - far larger than any real-world debug/mergeable section, while still
   generous enough not to reject any legitimate input.

Verified against more than just the one triggering input (CONFIRMATION.md section 7): the fix
also catches a different huge `ch_size` value (`0x123456789ABC`) and the separate
too-short-header case, not only the exact `0xFFFFFFFFFFFF0000` the original repro used.

- Project test suite: **476/476 passed before (pristine) -> 476/476 passed after (patched)**, no
  new failures, no previously-failing test fixed either. Specifically confirmed the real
  `--compress-debug-sections` tests (`x86_64-compress-debug-sections`,
  `-zlib-level`, `-zstd-level`) still pass with the fix applied, i.e. legitimate compressed
  sections (real zlib/zstd data, real `ch_size` matching the real decompressed size) are
  unaffected.

## What's not done / open questions

- The "truncated header" secondary defect (raw on-disk `sh_size < sizeof(ElfChdr<E>)`) was fixed
  defensively in the same patch and confirmed to be caught by the new check on a hand-crafted
  input, but was **not** independently confirmed to crash the *pristine* binary - per the
  assignment's own framing, reliably faulting on this specific defect alone needs the section to
  end very close to a real mapping/page boundary (see the project's own mmap-page notes in
  `___BUGHUNT___/README.md`), which wasn't attempted this session given the primary `ch_size`
  defect was already fully confirmed and demonstrates the same root unchecked-header-read pattern.
- `MAX_UNCOMPRESSED_SIZE` (4 GiB) is a judgment call, not derived from any spec - a legitimate
  compressed section this large is extremely unlikely in practice (real debug info rarely exceeds
  low hundreds of MB uncompressed even for huge binaries) but the exact right cutoff is ultimately
  a project-maintainer decision; a tighter cap tied to a multiple of the section's own on-disk
  (compressed) size was considered but not used, to keep the fix simple and avoid rejecting
  legitimately high compression ratios.
- A moderate-huge `ch_size` (observed at 1 TiB) does **not** hit the new cap's failure path the
  same way as the ~1.8e19 value - it's already rejected by the same `MAX_UNCOMPRESSED_SIZE` check
  (4 GiB < 1 TiB), so the fix also closes the "succeeds as a virtual reservation and spins" DoS
  variant observed against the pristine binary, not just the "aborts immediately" one - this was
  verified by construction (1 TiB > 4 GiB cap) but not by an additional live re-run against the
  patched binary specifically at that magnitude.
- Did not check whether the same unbounded-`ch_size` pattern is reachable through the other
  `uncompress()` call sites (`is_dwarf32()` in `src/passes.cc`, `--gdb-index`-related code in
  `src/gdb-index.cc`) with a *different* minimal trigger (e.g. not requiring `SHF_MERGE`) - not
  needed since the fix is at the single shared point (the constructor) that all call sites read
  `sh_size` from, but worth noting for a future session mapping out the full reachability surface.
