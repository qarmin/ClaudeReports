# archive-file_mf_slice_unchecked_member_size

While investigating candidate 003 (`ArHdr::read_name` SysV-offset OOB read), noticed that
`read_fat_archive_members()` / `read_thin_archive_members()` (`src/archive-file.cc`) pass an
entirely untrusted, attacker-controlled archive-member `size` (parsed via `atol(hdr.ar_size)`,
never validated against the archive's real remaining bytes) straight into
`MappedFile::slice(ctx, name, start, size)` (`src/mold.h:190`), which itself performs **zero**
range validation: `mf->data = data + start; mf->size = size;`. This produces a `MappedFile` whose
declared `.size` can be arbitrarily larger than the memory actually backing it.

This matters beyond an isolated OOB read: **every downstream bounds check in the codebase that
validates an offset against `mf->size`** (including the fixes already applied in candidates
001/002 of this workspace, which check `e_shoff` against `mf->get_contents().size()`) is
validating against a value the archive itself controls and can lie about - so those checks can be
satisfied by an offset that is "in range" per the lie while still pointing at genuinely unmapped
memory.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction, first against unpatched HEAD | 1-member fat archive, header declares `ar_size = 5000000`, only 64 real bytes (a fake ELF header with `e_shoff = 3000000`) actually follow | Real SIGSEGV via `is_gcc_lto_obj()` (the same crash site as candidate 001, but through a different root cause) |
| Same input, re-tested with candidates 001 AND 002's patches BOTH applied first | `git apply` both prior patches, rebuild, run the same crafted archive | **Still crashes** - proves this is a genuinely distinct root cause, not just an easier way to hit 001/002 (their fixes validate against `mf->size`, which this bug corrupts in the first place) |
| ASAN+UBSAN build (g++ 15.2.0) | `build-asan/mold --thread-count 5 -o out evil.a` | `AddressSanitizer: SEGV` in `Integer<...>::operator unsigned int()` called from `is_gcc_lto_obj`, `src/filetype.cc` - confirms a genuine OOB read |

## Status

**Confirmed** - 5/5 runs, identical crash address every time (deterministic: the wild offset is a
fixed integer distance from the real mapping's start, independent of ASLR/memory layout).

**Fully reproducible**: Yes. `repro/run.sh` builds the malformed archive with
`repro/gen_evil_ar.py` and runs the real `mold` binary via its real CLI entry point end to end
(`mold -o <out> evil.a` -> file-type detection -> `read_archive_members` ->
`read_fat_archive_members` -> `MappedFile::slice` -> `get_file_type` -> `is_gcc_lto_obj`).

## Class

crash (memory-safety: an unchecked, attacker-controlled length is used to construct a "file" view
that claims to be larger than the memory that actually backs it, undermining every subsequent
size-based bounds check performed against that view)

## Root cause

`read_fat_archive_members()`'s per-member loop computes `size = atol(hdr.ar_size)` (fully
attacker-controlled ASCII decimal from the archive header) and immediately uses it to build
`data = body + size` and `vec.push_back(mf->slice(ctx, name, body - begin, data - body))` with no
check that `body + size` actually stays within `begin + mf->size` (the archive's own real mapped
extent). `MappedFile::slice()` (`src/mold.h:190`) compounds this by performing no validation of
its own - it unconditionally sets `mf->data = data + start; mf->size = size;`, so the resulting
`MappedFile` for that archive member simply *asserts* it is `size` bytes long, regardless of how
much memory is genuinely mapped there. Every consumer of that sliced `MappedFile` (starting with
`get_file_type()`/`is_gcc_lto_obj()`, but not limited to them) treats `.size` as trustworthy when
performing its own bounds checks - including the very checks candidates 001 and 002 of this
workspace added, which compare an ELF offset against exactly this lied value. `read_thin_archive_members()`
has the identical unchecked-`size` pattern for its embedded string/symbol-table members (though
not for regular file-reference members, which never slice this file's own mapping - see "What's
not done" below and the scoping note in the fix).

## Introduced by

Not determined precisely - `size = atol(hdr.ar_size)` with no validation, and `MappedFile::slice()`
with no validation, both appear to be original/foundational code (archive support and the
`MappedFile`/slice mechanism are core, long-standing parts of mold's input-file handling) rather
than a later regression; not worth a full `git blame`/bisect this session given both call sites
and the `slice()` implementation itself show the identical unchecked pattern from their earliest
form in this file's history (see candidate 003's `Introduced by` for the same file/era).

## The fix

`patch.diff` (`src/archive-file.cc` only - `MappedFile::slice()` itself was deliberately left
generic/unchanged, see below):

- `read_fat_archive_members()`: validates the member header itself fits in the remaining mapped
  bytes before dereferencing it, then validates `size` doesn't exceed what's actually left in the
  mapping, before either is used - `Fatal()`s with a clear message otherwise. Every member in a fat
  archive embeds its own data, so this check applies unconditionally to every member.
- `read_thin_archive_members()`: same header-fits check, but the member-`size` check is **scoped
  to only the string-table/symbol-table branches** (`hdr.is_strtab() || hdr.is_symtab()`) - a
  thin archive's regular (non-strtab/symtab) members reference an *external* file by name and
  never slice this archive's own mapping using `size` (the existing code already does `data = body`,
  not `data = body + size`, for those). An unscoped check here is wrong: `ar_size` for a thin
  archive's regular member legitimately describes the *external* referenced file's size, which can
  be (and normally is) far larger than the archive file itself - the project's own
  `test/thin-archive.sh` caught this exact false positive during verification (see below).

Verified: `repro/run.sh` no longer crashes against the patched build - `mold` now exits 1 with
`mold: fatal: <path>: archive member size out of bounds`. Also re-tested:
- A well-formed archive with a genuine long filename (`ar rcs good.a
  this_is_a_very_long_object_filename_to_force_long_name.o`) still links via `--whole-archive`.
- The real system `/usr/lib/x86_64-linux-gnu/libc.a` (a large, many-member fat archive) still
  links a trivial test program successfully with the patch applied.

- Project test suite (`ctest -j5` in `build-asan/`): **first fix attempt regressed
  `x86_64-thin-archive`** (a real, legitimate thin-archive test with a long-filename external
  member) - caught precisely because the size check was initially applied unconditionally to
  every thin-archive member instead of only the embedded strtab/symtab ones; fixed by scoping the
  check as described above, then re-ran the full suite: 475/476 passed before and after (identical
  single pre-existing, patch-unrelated failure both times: `x86_64-run-clang`, the same
  ASAN/`mold-wrapper.so` `LD_PRELOAD` interaction noted in candidates 001-003's own README/project
  notes). No new failures remained after the scoping fix.

## What's not done / open questions

- `MappedFile::slice()` itself (`src/mold.h:190`) was intentionally left unchanged - it's a
  generic primitive with no notion of what "valid" bounds are for its caller, so the fix was
  applied at the one call site that currently passes untrusted input (`archive-file.cc`) rather
  than baking archive-specific assumptions into a shared low-level helper. If `slice()` grows
  other untrusted callers in the future, each would need the same care at its own call site (or
  `slice()` would need an explicit "trusted extent" contract documented/enforced some other way) -
  not pursued further this session since no other untrusted caller was found.
- The BSD-style `#1/N` long-filename branch of `ArHdr::read_name()` (noted as a new backlog item
  during candidate 003) has a related but distinct unchecked-length pattern (`namelen` used to
  slice the member *body*, not the whole-member `size` this candidate covers) - not investigated
  here, still open in `IDEAS.md`.
- Did not exhaustively re-audit every other place in the codebase that reads `mf->size` for a
  bounds check to see whether any of them are reachable via a *different* untrusted-size-producing
  path (this candidate only closes the archive-slicing path specifically) - a reasonable next
  suspect for a future session would be `mf->slice()`'s only other potential future callers, and
  double-checking whether `ObjectFile`/`InputFile` construction from an LTO/plugin-produced
  temporary file (a different `MappedFile` creation path, not `slice()`) has an analogous gap.
