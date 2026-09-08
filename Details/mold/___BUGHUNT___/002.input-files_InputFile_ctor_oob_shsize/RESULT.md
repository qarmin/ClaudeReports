# input-files.cc InputFile ctor OOB read via unchecked e_shoff (e_shnum==0 path)

`src/input-files.cc`'s `InputFile<E>::InputFile()` - the base-class constructor every object/DSO
input file goes through - dereferences `sh_begin->sh_size` (where `sh_begin = mf->data +
ehdr.e_shoff`) *before* its own existing bounds check (`if (mf->data + mf->size < (u8 *)(sh_begin
+ num_sections)) Fatal(...)`) ever runs, whenever `ehdr.e_shnum == 0`. Seeded from
`code-audit-structured`'s prior static review (`___FINDINGS___/findings-05-input.toml`,
"critical/easy/panic").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Hand-crafted malformed ELF, real CLI invocation | 64-byte ET_REL/EM_X86_64 object, `e_shoff=0x7fff0000`, `e_shnum=0`, `e_shstrndx=0`, fed to `mold -o out bad.o` via a real command-line invocation | SIGSEGV (exit 139) |
| Same input under gdb, `set follow-fork-mode child` | mold forks a worker child (`src/subprocess-unix.cc::fork_child`) to do the actual link, so the real fault happens in the child, not the process a plain `gdb --args mold ...` attaches to - needed `follow-fork-mode child` to see the real backtrace | Deterministic SIGSEGV in `InputFile<X86_64>::InputFile` at `src/input-files.cc:95` |
| ASAN+UBSAN build, 5/5 repeat runs (plain, non-gdb) | `build-asan/`, checks exit status/stderr, no gdb needed for detection (the parent re-raises the same signal after the crashed child dies, so the outer process is itself SIGSEGV-terminated) | 5/5 crash |
| Boundary-value variant: `e_shoff == mf->size` exactly (0 bytes of slack, not "wildly out of range") | Same file layout, `e_shoff=64` (== file size), `e_shnum=0` | Did **not** crash this run - `sh_begin` points exactly one-past-the-end (still UB to dereference, but the read landed on zero-filled bytes within the same mmap'ed page rather than an unmapped one) and produced a differently-worded, non-crashing `Fatal()` further downstream instead. Recorded per `SKILL.md` critical pitfall #3 ("vary the input, not just the exact trigger") - confirms the *far*-out-of-range case is the reliable trigger, the near-boundary case is the same missing-validation bug but doesn't reliably manifest as an observable crash. The fix (below) closes both variants regardless. |

Two escalating tools (hand-crafted input against the real binary, then gdb with
`follow-fork-mode child` to pin the exact source line) plus one boundary-value variation were
enough - no fuzzing/miri needed given the deterministic, immediately-reproducing crash.

## Status

**Confirmed** - 5/5 repeat runs, deterministic (same faulting line every time). Also independently
reproduces with the plain system-installed `mold` 2.40.4 binary (`/usr/bin/mold`), confirming it
isn't specific to this repo's own build.

**Fully reproducible**: **Yes** - `repro/run.sh` feeds a real, crafted `.o` file to the real
`mold` binary via a real command-line invocation, the same path a user/build-system invocation of
`mold <bad-object>` would take.

## Class

`crash` (memory-safety: out-of-bounds read via unchecked file offset from an untrusted input
file).

## Root cause

```c++
ElfShdr<E> *sh_begin = (ElfShdr<E> *)(mf->data + ehdr.e_shoff);
i64 num_sections = (ehdr.e_shnum == 0) ? sh_begin->sh_size : ehdr.e_shnum;   // <-- deref before check
if (mf->data + mf->size < (u8 *)(sh_begin + num_sections))
  Fatal(ctx) << mf->name << ": e_shoff or e_shnum corrupted: " << mf->size << " " << num_sections;
```

Per the ELF spec, `e_shnum == 0` is the escape hatch for "more than 65535 sections" - the real
count is supposed to be stored in the first section header's `sh_size` field instead. mold reads
that fallback value (`sh_begin->sh_size`) unconditionally, before the very next line's bounds
check (which only validates the *already-computed* `num_sections`) has any chance to run. If
`e_shoff` is corrupt/adversarial, `sh_begin` itself points outside the mapped file, and reading
`->sh_size` through it is an out-of-bounds read - confirmed SIGSEGV when `e_shoff` is far enough
past the mapping to land on an unmapped page (`src/input-files.cc:95`, `gdb` backtrace: `mold::
InputFile<mold::X86_64>::InputFile`, called from `ObjectFile<E>::ObjectFile` at `src/mold.h:1985`,
via `new_object_file` -> `read_file` -> `read_input_files` -> `mold_main`).

This is the exact issue the audit flagged: the file's own later range check (`sh_begin +
num_sections` against `mf->size`) is correct in spirit but runs one statement too late for the
`e_shnum == 0` case, and (per the audit's secondary note, addressed by this fix too) compares
*pointers* rather than *counts*, which for a sufficiently adversarial `sh_size`/`e_shoff`
combination could itself overflow the pointer arithmetic and defeat the check.

## Introduced by

ad3bd347 "Handle object files having more than 0xff00 sections" (2021-03-04) - the commit that
introduced the `e_shnum == 0` -> read `sh_begin->sh_size` fallback (then in `object_file.cc`,
later renamed to `src/input-files.cc`) with this exact unchecked-dereference-before-bounds-check
ordering; unchanged in that respect ever since.

## The fix

Validate `e_shoff` (and, specifically for the `e_shnum == 0` case, that at least one full
`ElfShdr<E>` fits at that offset) *before* `sh_begin` is dereferenced at all, and switched the
final bounds check from pointer arithmetic (`sh_begin + num_sections` vs `mf->data + mf->size`,
which can itself overflow for an adversarial `num_sections`) to an integer-count comparison:

```c++
u64 shoff = ehdr.e_shoff;
if (shoff > (u64)mf->size ||
    (ehdr.e_shnum == 0 && (u64)mf->size - shoff < sizeof(ElfShdr<E>)))
  Fatal(ctx) << *this << ": e_shoff is out of range";

ElfShdr<E> *sh_begin = (ElfShdr<E> *)(mf->data + shoff);
i64 num_sections = (ehdr.e_shnum == 0) ? (i64)(u64)sh_begin->sh_size : (i64)(u16)ehdr.e_shnum;

if (num_sections < 0 ||
    (u64)num_sections > ((u64)mf->size - shoff) / sizeof(ElfShdr<E>))
  Fatal(ctx) << mf->name << ": e_shoff or e_shnum corrupted: " << mf->size << " " << num_sections;
elf_sections = {sh_begin, sh_begin + num_sections};
```

Verified: the same crafted `bad.o` (`e_shoff=0x7fff0000, e_shnum=0`) no longer crashes after the
patch, 5/5 runs - `mold` now exits with `mold: fatal: bad.o: e_shoff is out of range` (exit 1)
instead of a SIGSEGV. The boundary-value variant (`e_shoff == mf->size`, `e_shnum=0`) is also now
caught explicitly (previously silently read zero-filled past-EOF bytes without crashing - still
fixed by this patch's second bounds condition).

- Project test suite (`ctest -j24` in `build-asan/`, ASAN+UBSAN build, this patch applied alone,
  `src/filetype.cc`/candidate 001 reverted for this run to isolate the two changes): 475/476
  passed before and after (unchanged). The one failure, `x86_64-run-clang`, is the same
  pre-existing ASAN+`LD_PRELOAD` environment artifact noted in candidate 001's `RESULT.md`,
  unrelated to this file. 20 further tests skipped by ctest itself (environment-gated), same set
  before and after.

## What's not done / open questions

- **New suspect noticed, not chased this session**: every `Fatal(ctx) << *this << ...` call made
  from *inside* `InputFile<E>::InputFile()` itself (both the pre-existing "e_shoff or e_shnum
  corrupted" one and this patch's new "e_shoff is out of range" one) triggers a UBSAN "invalid
  downcast" diagnostic at `src/input-files.cc:1508` ("downcast of address ... which does not
  point to an object of type 'ObjectFile' ... object is of type
  'mold::InputFile<mold::X86_64>'") - `*this` is only a base `InputFile` at this point in
  construction (the derived `ObjectFile` hasn't finished/started its own construction yet, since
  `ObjectFile`'s constructor calls `InputFile`'s constructor first via its member-init list), so
  whatever `operator<<`/error-formatting code path this hits and that assumes a fully-constructed
  `ObjectFile` is invoking UB. This doesn't crash under the runs tried here (UBSAN reports and
  continues), but is a real, distinct bug worth its own candidate - added to `IDEAS.md`.
- `src/mold.h`'s `SharedFile<E>` (DSO) constructor may share the same base-class-`InputFile`
  construction path and thus the same `e_shoff`/`e_shnum` validation gap for `.so` inputs - not
  separately re-verified this session (this candidate's repro used an `ET_REL` object; `ET_DYN`
  wasn't tried). Worth a follow-up check since `InputFile::InputFile` is shared by both.
