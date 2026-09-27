# 017.input-files_read_riscv_attributes_multi_bug

`read_riscv_attributes()` in `src/input-files.cc` (~line 187-233) parses a `.riscv.attributes`
section and was flagged by a prior static-audit pass as containing several independent defects in
one function. Dispatch is gated purely by C++ template instantiation (`if constexpr (is_riscv<E>)`
at the call site, `src/input-files.cc:352-357`), which is itself selected by the object file's
`e_machine` field (`EM_RISCV = 243`, `src/filetype.cc`) - so a real repro needs an ELF object whose
header claims `e_machine = EM_RISCV`, but no actual RISC-V toolchain is needed: the header bytes
were hand-crafted with a small Python `struct` generator, following the pattern already established
by candidates 008/014 in this same file.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read current source at `src/input-files.cc:186-233`, confirmed all 5 defects described in the audit are still present unchanged, and confirmed the `is_riscv<E>`/`EM_RISCV` dispatch gating | static read | Confirmed suspect unchanged; dispatch requires `e_machine == EM_RISCV` (hand-craftable) |
| Sub-case 2 (`sz<4` integer underflow): crafted RV64LE `ET_REL` object with `.riscv.attributes` section, sub-section `sz=2` (real content spells "riscv\0" + `ELF_TAG_FILE` + a ULEB tag/value right after the 4-byte `sz` field, so the parser doesn't bail out early on a vendor-name mismatch) | plain Debug build (`build-plain`, `-DCMAKE_BUILD_TYPE=Debug`), `mold -m elf64lriscv --thread-count 5 -shared -o out.so evil.o` | **Crashed**: `SIGBUS` (exit 135), 5/5 identical runs. `gdb` backtrace pinpoints the fault inside `read_uleb` (`lib/lib.h:383`) called from `read_riscv_attributes<RV64LE>` at `input-files.cc:213`, exactly the underflowed-`p` inner ULEB loop walking off the mmap'd region. |
| Sub-case 4 (throwing `substr` on a too-short arch sub-section): crafted a sub-section whose real content is exactly "riscv\0"+`ELF_TAG_FILE` (7 bytes, consistent `sz=11`, no underflow) - after `p.substr(6)`, only 1 byte remains, so `p.substr(5)` throws | same plain Debug build, same invocation style | **Crashed**: `terminate called after throwing an instance of 'std::out_of_range' / what(): basic_string_view::substr: __pos (which is 5) > __size (which is 1)`, exit 134 (SIGABRT), 5/5 identical runs |
| Boundary variation of sub-case 2 (CONFIRMATION.md section 7): re-ran with `sz=3` and `sz=0` (both still `<4`) instead of the original `sz=2` | same plain Debug build | Both also crash with SIGBUS, 3/3 runs each - confirms the fix target isn't narrowly tied to the exact `sz=2` byte value |
| Sub-case 3 (infinite loop, bonus): crafted `sz=0` with real content that does **not** spell "riscv\0" (so the vendor-name check fails, `continue` fires, and `data` never advances since `sz=0`) | same plain Debug build, `--thread-count 1`, `timeout 6` | **Hung**: `ps` snapshot mid-run showed the mold process in state `R` at 99.6% CPU; `timeout` killed it after 6s (exit 124) - genuine CPU-exhaustion hang, same bug *class* as candidate 007 |
| Sub-case 5 (arch-string missing NUL terminator, infinite loop) | not independently constructed a dedicated input this session | Not independently repro'd (see "What's not done" below) - covered by the same patch (an explicit `p.npos` check was added) but not driven through the real binary with its own crafted input |
| Sub-case 1 (truncated 1-3 byte section before the initial `U32` read) | not attempted | Skipped per the task's own priority guidance - flagged as hardest to construct reliably (needs the section to end exactly at a page boundary, per candidate 004's mmap-page lesson) and lowest priority of the five |
| Sub-case 6 (`read_uleb` itself unbounded past a buffer's end) | not attempted | Explicitly out of scope per the task instructions - tracked as a separate, harder-to-construct backlog item elsewhere |

All repros run the real, unmodified `mold` binary via its actual CLI entry point on a hand-crafted
ELF relocatable object file - a malformed object file handed to the linker's real parsing entry
point *is* the genuine external input (CONFIRMATION.md section 11's two-participant requirement
doesn't apply; this is a single-participant finding, same as candidates 008/014).

## Status

**Confirmed.**

Three independent sub-cases reproduced, each with a stated repeat count:
- Sub-case 2 (`sz<4` underflow -> OOB read): **5/5 runs**, plain Debug build, SIGBUS (exit 135).
  Boundary variants (`sz=3`, `sz=0`) also crash, 3/3 runs each.
- Sub-case 4 (throwing `substr` on a short arch sub-section): **5/5 runs**, plain Debug build,
  SIGABRT (exit 134), uncaught `std::out_of_range`.
- Sub-case 3 (`sz==0` infinite loop): confirmed via a live `ps` snapshot showing state `R` at
  99.6% CPU while hung, killed by `timeout` after 6s (exit 124) - same confirmation method as
  candidate 007. Re-run 5/5 with identical `timeout`-killed-at-124 behavior via `repro/run.sh`.

**Fully reproducible: Yes.** Every repro runs the real, unmodified `mold` binary via its actual CLI
entry point (`-m elf64lriscv --thread-count N -shared -o out.so evil.o`) on a hand-crafted-but-
otherwise-standard ELF relocatable object - the same kind of input a real user/attacker/build system
would hand to the linker (a `.riscv.attributes` section is emitted automatically by any RISC-V GCC/
Clang toolchain; only the malformed internal length fields are fabricated here). No internal
function is called directly; nothing fakes a second participant's behavior.

## Class

crash / hang (three distinct mechanisms in one function: a raw-pointer OOB read reached via an
integer-underflow-inflated `std::string_view` size (SIGBUS), an uncaught C++ exception from the
single-argument, throwing `substr(pos)` overload (SIGABRT/`std::terminate`), and a CPU-exhaustion
infinite loop from a cursor that fails to advance on a specific input shape)

## Root cause

`read_riscv_attributes()` walks a `.riscv.attributes` section's self-declared sub-section-length
(`sz`) and internal tag/name fields with no validation beyond a single upper-bound check:

```cpp
while (!data.empty()) {
  i64 sz = *(U32<E> *)data.data();
  if (data.size() < sz)                              // (A) only rejects sz too LARGE
    Fatal(ctx) << file << ": corrupted .riscv.attributes section";

  std::string_view p(data.data() + 4, sz - 4);        // (B) sz<4 underflows sz-4 to ~2^64
  data = data.substr(sz);                             // (C) sz==0 -> data never advances

  if (!p.starts_with("riscv\0"sv))
    continue;
  p = p.substr(6);                                    // (D) throws if p.size() < 6

  if (!p.starts_with(ELF_TAG_FILE))
    Fatal(ctx) << file << ": corrupted .riscv.attributes section";
  p = p.substr(5);                                    // (E) throws if p.size() < 5

  while (!p.empty()) {
    i64 tag = read_uleb(&p);
    ...
    case ELF_TAG_RISCV_ARCH: {
      i64 pos = p.find_first_of('\0');
      file.extra.arch = p.substr(0, pos);
      p = p.substr(pos + 1);                          // (F) pos==npos -> substr(0), no advance
      break;
    }
```

- **(B)** is sub-case 2: the check at (A) only rejects `sz` too large, never `sz < 4`. Since
  `sz - 4` is computed with `sz` implicitly converted to the unsigned `size_type` the
  `std::string_view` constructor expects, `sz` in `{0,1,2,3}` underflows to a value near `SIZE_MAX`,
  producing a view that claims ~2^64 bytes starting 4 bytes into the section. Every subsequent read
  through that view (the vendor-name check, the ULEB128 tag/value loop via `read_uleb`) walks
  straight past the section's real content, through the zero-padded tail of the mmap'd file page,
  and off the actual mapped region - `SIGBUS` (confirmed via `gdb`: the crash is inside
  `read_uleb`, `lib/lib.h:383`, called from `input-files.cc:213`).
- **(C)** is sub-case 3: when `sz == 0`, `data = data.substr(sz)` is a no-op. If the vendor-name
  check at (D)'s precondition also fails (`p` doesn't start with `"riscv\0"`), `continue` sends
  control back to the top of the outer loop with `data` completely unchanged - the loop spins
  forever, genuinely pegging one CPU core (confirmed via `ps` showing state `R` at ~100% CPU).
- **(D)/(E)** is sub-case 4: both are the single-argument, *throwing* `std::string_view::substr(pos)`
  overload (as opposed to the two-argument, *clamping* `substr(pos, count)` overload used
  elsewhere) - the same crash mechanism already confirmed twice in this project (candidate 008's
  `parse_ehframe()`, candidate 014's `parse_note_gnu_property()`). mold has no top-level exception
  handler anywhere in its input-parsing pipeline, so an out-of-range `pos` reaches
  `std::terminate()` -> `SIGABRT`.
- **(F)** is sub-case 5: if the arch string has no terminating NUL, `find_first_of('\0')` returns
  `std::string_view::npos`, assigned to the signed `i64 pos` this becomes `-1`; `p.substr(pos + 1)`
  becomes `p.substr(0)` - unchanged - so the inner `while (!p.empty())` loop also hangs forever, the
  same bug *class* as (C) reached through a different field. (Not independently repro'd with a
  dedicated crafted input this session - see "What's not done" below - but covered by the patch.)
- Sub-case 1 (the initial 4-byte `U32` struct read with no size check at all, if `data` holds only
  1-3 bytes) and sub-case 6 (`read_uleb` itself being unbounded past a buffer's end) are the same
  general shape but were explicitly deprioritized per the task's own guidance (hardest to construct
  reliably / tracked as a separate backlog item) - not independently investigated this session.

## Introduced by

aa644914 "Merge .riscv.attributes input sections" (2023-07-28) - the commit that originally
introduced RISC-V attribute-section parsing (then in `elf/input-files.cc`) with this exact
unguarded structure. `git log -S'read_riscv_attributes'` shows this as the only commit that ever
touched the function's introduction; no bounds checking was added between 2023 and this session's
base commit.

## The fix

Added explicit bounds checks before every slicing/dereference point identified in "Root cause"
above, matching the style of the existing `Fatal(ctx) << file << ": corrupted ..."` checks already
used elsewhere in this file (candidates 008/014's fixes for the sibling `parse_ehframe()`/
`parse_note_gnu_property()` functions):

- `data.size() < 4` before the initial `U32` struct read (covers sub-case 1).
- `sz < 4` alongside the existing `data.size() < sz` upper-bound check (covers sub-cases 2 and 3 -
  a rejected `sz < 4` can never underflow, and can never leave `data` unchanged either, since 0 is
  now rejected outright).
- `p.size() < 5` before the `p.substr(5)` advance (covers sub-case 4's second throw site; the first,
  `p.substr(6)`, is unreachable once `p.starts_with("riscv\0"sv)` has already verified `p.size() >= 6`).
- `pos == (i64)p.npos` (missing NUL terminator) treated as corruption via an explicit `Fatal` instead
  of silently not advancing `p` (covers sub-case 5).

Verified:
- Re-ran all three exercised repro inputs (sub-cases 2, 3, 4, plus the `sz=3`/`sz=0` boundary
  variants of sub-case 2) against the patched build: all now exit 1 with
  `mold: fatal: <file>: corrupted .riscv.attributes section` instead of crashing/hanging - confirmed
  each crash/hang is fully gone, not just relocated (CONFIRMATION.md section 6).
- Checked against more than just the one input that first triggered each sub-case
  (CONFIRMATION.md section 7): sub-case 2 was checked at `sz=2`, `sz=3`, and `sz=0` (the last of
  which also independently exercises sub-case 3's precondition), all three now cleanly rejected.
- Project test suite (`ctest --test-dir build-plain -j4`, plain Debug build): **476/476 pass before
  the patch, 476/476 pass after** (20 tests skipped by ctest itself for feature/environment gating,
  unaffected by this change either way). `test/arch-riscv64-attributes.sh` and
  `test/arch-riscv64-attributes2.sh` (the existing legitimate-`.riscv.attributes`-parsing
  regression tests) are both included in that count and continue to pass, confirming the added
  checks don't reject well-formed input.

## What's not done / open questions

- Sub-case 1 (truncated 1-3 byte section, the initial unchecked `U32` struct read) and sub-case 6
  (`read_uleb` itself unbounded past a buffer's end) were not independently repro'd with their own
  dedicated crafted input this session - both are covered by the same patch (sub-case 1 by the new
  `data.size() < 4` guard; sub-case 6 is unchanged and remains a known, separately-tracked concern
  shared with every other `read_uleb`/`read_sleb` call site in this codebase) but neither was driven
  end-to-end through the real binary. Deprioritized per the task's own guidance: sub-case 1 needs
  page-boundary luck to fault reliably (candidate 004's mmap-page lesson), and sub-case 6 is a
  known, harder-to-construct backlog item tracked separately, not specific to this function.
- Sub-case 5 (arch-string missing NUL terminator -> inner-loop hang) was not independently repro'd
  with its own dedicated crafted input this session either - the fix's `pos == p.npos` guard was
  verified only by code reading, not by a live `ps`-state-`R` observation the way sub-case 3 was.
  A future session could construct a `.riscv.attributes` section whose `ELF_TAG_RISCV_ARCH` value
  has no terminating NUL byte before the sub-section ends, to independently confirm this specific
  hang and its fix, for full completeness.
- This is the third confirmed instance in this project of the "single-argument, throwing
  `substr(pos)` fed an untrusted length" bug class (candidates 008 and 014 were the first two) -
  no further grep sweep was repeated this session since candidate 008 already did one and recorded
  the remaining candidates in `IDEAS.md`/`projects/mold.md`.
- This is also the second confirmed instance of a genuine infinite-loop/CPU-exhaustion hang bug
  class in this project (candidate 007 was the first, in `archive-file.cc`'s thin-archive
  `__.SYMDEF`-skip loop) - both share the same underlying shape ("a `continue`/skip branch that
  fires before the loop's only cursor-advancement statement, or a cursor-advancement amount that can
  legitimately be zero").
