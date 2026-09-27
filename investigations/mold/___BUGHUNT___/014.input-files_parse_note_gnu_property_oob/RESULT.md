# 014.input-files_parse_note_gnu_property_oob

`ObjectFile<E>::parse_note_gnu_property()` in `src/input-files.cc` (lines ~146-182 at this
session's HEAD, `c340272e6376f9a8e5b5cd6f6634f925f6d54e5c` base / `d3cf4a23` actual worktree HEAD -
the function is byte-identical between the two) walks a `.note.gnu.property` ELF note section with
no bounds checking on the attacker-controlled `n_namesz`/`n_descsz` fields inside each note record.
This was flagged by a prior static-audit pass, investigated fresh this session (never previously
built/verified). It is the same bug *class* already confirmed once in this project (candidate 008,
`parse_ehframe()`): an untrusted length used as the argument to the single-argument
`std::string_view::substr(pos)` overload, which throws `std::out_of_range` (not the
clamping two-argument `substr(pos, count)` overload) when `pos > size()`. mold has no top-level
exception handler anywhere in its input-parsing pipeline, so this reaches `std::terminate()` ->
`SIGABRT`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read current source at `src/input-files.cc:146-182`, confirmed it matches the audit's description exactly (four separate untrusted `substr`/underflow issues as described) | static read | Confirmed suspect still present, unchanged |
| Built minimal x86_64 `ET_REL` object with a `.note.gnu.property` section (`SHT_NOTE`, dispatch is by section *name* only - `if (name == ".note.gnu.property")`, not by `sh_type`) containing one `ElfNhdr` record with `n_namesz = 0xFFFFFFF0` (huge, fabricated), `n_descsz = 0`, `n_type = 5` (`NT_GNU_PROPERTY_TYPE_0`) | plain Debug build (`build-plain`, `-DCMAKE_BUILD_TYPE=Debug`, default g++), `mold -m elf_x86_64 --thread-count 5 -shared -o out.so evil.o` | **Crashed**: `terminate called after throwing an instance of 'std::out_of_range' / what(): basic_string_view::substr: __pos (which is 4294967280) > __size (which is 0)`, exit 134 (SIGABRT). Reproduced 5/5 identical runs. |
| Second, independent input in the same bug class: valid `n_namesz=4` ("GNU\0"), huge fabricated `n_descsz = 0x7FFFFFF0`, `n_type = 5` | same plain Debug build, same invocation | **Crashed** the same way against pristine code: `__pos (which is 2147483632) > __size (which is 0)`, exit 134/SIGABRT (confirmed via `core dumped` message and exit code) |

Both were run against the real `mold` binary via its actual CLI entry point (`-m elf_x86_64 -shared -o ... evil.o`) - this is a single-participant finding (a malformed object file handed to the linker's real parsing entry point *is* the genuine external input; no second participant to fake), so CONFIRMATION.md section 11's two-participant requirement doesn't apply here.

## Status

**Confirmed.**

Reproduces deterministically, 5/5 runs, plain Debug build (`-DCMAKE_BUILD_TYPE=Debug`, default
g++, no sanitizer needed or used - the crash mechanism is an uncaught C++ exception, not a raw
memory-safety fault that would need ASAN to surface). A second, independent malformed input in the
same bug class (huge `n_descsz` instead of huge `n_namesz`) also crashes identically against
pristine code, satisfying CONFIRMATION.md section 7 (checked more than just the one input that
first triggered the bug).

**Fully reproducible: Yes.** The repro runs the real, unmodified `mold` binary via its actual CLI
entry point on a hand-crafted-but-otherwise-completely-standard ELF relocatable object file - the
same kind of input a real user/attacker would hand to the linker. No internal function is called
directly; no state is faked on behalf of a second participant (this is not a client/server
interaction - the file is the entire external input).

## Class

crash (uncaught C++ exception -> `std::terminate()` -> `SIGABRT`; not a raw pointer OOB)

## Root cause

`parse_note_gnu_property()` reads each `ElfNhdr<E>` record from a `.note.gnu.property` section and
advances a `std::string_view` cursor using the attacker-controlled `n_namesz`/`n_descsz` fields with
zero validation:

```cpp
ElfNhdr<E> &hdr = *(ElfNhdr<E> *)data.data();   // (1) no size check before this struct read
data = data.substr(sizeof(hdr));

std::string_view name = data.substr(0, hdr.n_namesz - 1);   // (2) n_namesz==0 underflows to 0xffffffff (harmless here: clamping substr)
data = data.substr(align_to(hdr.n_namesz, 4));              // (3) THROWS if pos > data.size() - single-arg substr(pos)

std::string_view desc = data.substr(0, hdr.n_descsz);
data = data.substr(align_to(hdr.n_descsz, sizeof(Word<E>))); // (4) same throw risk for n_descsz
```

and the inner property-record loop does the same again:

```cpp
u32 type = *(U32<E> *)desc.data();
u32 size = *(U32<E> *)(desc.data() + 4);   // (5) reads 8 bytes whenever desc is merely non-empty, no size>=8 check
desc = desc.substr(8);                      // (6) throws if desc.size() < 8 (single-arg substr(pos))
...
desc = desc.substr(align_to(size, sizeof(Word<E>)));  // (7) throws if size is a fabricated huge value
```

Any of (1), (3), (4), (6), (7) is independently sufficient to crash. This session's repro triggered
(3) directly (huge `n_namesz`) and separately confirmed (4) (huge `n_descsz`) - both hit the exact
same underlying flaw (an untrusted length fed straight into the throwing `substr(pos)` overload,
with mold having no exception handler anywhere in its input-parsing call stack).

Note: `get_string(ctx, shdr)` (the caller that produces the initial `data` view) *does* already
validate that the section's own `sh_offset`/`sh_size` stay within the mapped file - the vulnerable
fields are entirely internal to the note record's own self-declared lengths, not the section
header's.

## Introduced by

7977909c "Merge .note.gnu.property" (2021-05-01) - the commit that originally introduced
`.note.gnu.property` parsing (then in `object_file.cc`) with this exact unguarded structure; later
moved verbatim to `src/input-files.cc` by 5b859497 "Reorganize directory structure" (2024-08-14),
which only renamed/relocated the file (`git diff` between the two commits shows zero changes to
this function's body). No bounds checking was ever added to this function between 2021 and this
session's base commit.

## The fix

Added explicit bounds checks before every slicing point identified in "Root cause" above, matching
the style of the existing `Fatal(ctx) << *this << ": corrupted ..."` checks elsewhere in this file
(e.g. candidate 008's `parse_ehframe()` fix): before the initial `ElfNhdr<E>` struct read, before
each `substr(align_to(...))` advance (both name and desc), a guard against `n_namesz == 0`
underflowing the `name` slice length, and a `desc.size() >= 8` check before the inner loop's
type/size read - each replaced by an early `Fatal(ctx) << *this << ": corrupted .note.gnu.property"`
that stops processing this input file with a clean, catchable error instead of an uncaught
exception.

Verified:
- Re-ran both repro inputs (huge `n_namesz`, huge `n_descsz`) against the patched build: both now
  exit 1 with `mold: fatal: <file>: corrupted .note.gnu.property` instead of crashing - confirmed
  the crash is fully gone, not just relocated (CONFIRMATION.md section 6).
- Checked against a second input in the same bug class (CONFIRMATION.md section 7), not just the
  one that first triggered the bug, as noted above.
- Project test suite (`ctest --test-dir build-plain -j4`, plain Debug build): **493/493 pass
  before the patch, 493/493 pass after** (20 tests skipped by ctest itself for
  feature/environment gating, unaffected by this change either way).

## What's not done / open questions

- Sub-cases (1) and (6)/(7) above (truncated-header-at-start, and the inner desc-loop's 8-byte
  read/size-based advance) were not independently repro'd with their own dedicated crafted input
  this session - they're covered by the same patch (verified by code reading and by the fact the
  patch adds an explicit guard at each of those exact points) but only (3) and (4) were driven end-
  to-end through the real binary. A future session could construct two more minimal inputs (a
  `.note.gnu.property` section truncated to fewer than 12 bytes; a name="GNU" record whose `desc`
  is exactly 1-7 bytes) to independently confirm those two sub-cases crash on pristine code and are
  fixed by the same patch, for full completeness - considered low value given the fix mechanically
  covers them and the underlying flaw (untrusted length -> throwing `substr(pos)`) is identical.
- This is the second confirmed instance in this project of the exact "single-argument
  `substr(pos)` fed an untrusted length" bug class (candidate 008 was the first, in
  `parse_ehframe()`). Per candidate 008's own notes, a grep sweep for other single-argument
  `.substr(pos)` call sites fed untrusted data was already done and recorded in
  `___BUGHUNT___/IDEAS.md`/`projects/mold.md` - this candidate doesn't repeat that sweep, but the
  broader pattern ("any single-arg `.substr(pos)` call fed an attacker-controlled `pos`") remains
  worth grepping for wherever a future session touches a new parsing function in this file.
