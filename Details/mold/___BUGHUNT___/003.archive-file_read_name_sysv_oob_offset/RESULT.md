# archive-file_read_name_sysv_oob_offset

`ArHdr::read_name()` in `src/archive-file.cc` handles the SysV extended-filename convention
(`ar_name` starting with `/N`) by computing `strtab.data() + atoi(ar_name + 1)` and immediately
passing that pointer to `strstr()`, with no check that the decoded offset actually lies inside the
archive's string table. Flagged by a prior static audit
(`___FINDINGS___/findings-05-input.toml`, seeded into `___BUGHUNT___/IDEAS.md` as
`src/archive-file.cc:52-74`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | Hand-built a 1-member `!<arch>\n` archive whose sole member header uses SysV name `/999999999` (a huge, out-of-range decimal offset) with no string-table member present at all | Real, deterministic SIGSEGV |
| ASAN+UBSAN build (`-DMOLD_USE_ASAN=ON`, g++ 15.2.0) | `build-asan/mold --thread-count 5 -o out evil.a` | `AddressSanitizer: SEGV` in `strstr` called from `read_name`, `src/archive-file.cc:67` - confirms genuine OOB read, not a false crash from unrelated memory corruption |
| Plain release build (no sanitizer) | `-DCMAKE_BUILD_TYPE=Release`, `build-plain/mold` | Real `Segmentation fault` (exit 139) - confirms this isn't an ASAN-only artifact |

## Status

**Confirmed** - 5/5 runs against the ASAN build (identical crash address every time - the wild
pointer is `nullptr + 999999999`, a fixed integer offset independent of memory layout/ASLR, so
this is fully deterministic, not merely likely). Also reproduces in a plain, non-instrumented
release build (real SIGSEGV, exit 139).

**Fully reproducible**: Yes. `repro/run.sh` builds the malformed `.a` with
`repro/gen_evil_ar.py` and runs it against the real `mold` binary via its real CLI entry point
(`mold -o <out> evil.a`) - the actual end-to-end trigger path (`main.cc` -> file-type detection ->
`read_archive_members` -> `read_fat_archive_members` -> `ArHdr::read_name`), not a direct call
into the parsing function.

## Class

crash (memory-safety: out-of-bounds read via unchecked, attacker-controlled pointer arithmetic)

## Root cause

`read_name()`'s SysV-style branch trusts the decimal digits after `/` in the 16-byte `ar_name`
field as a byte offset into `strtab` (a `std::string_view` over whatever the archive's own `// `
string-table member happened to contain, or an empty/default view if the archive carries none at
all). It never validates that offset against `strtab.size()` before forming
`strtab.data() + offset` and dereferencing through `strstr()`. `strstr()` itself compounds this:
it expects a NUL-terminated C string, but the mmap'd archive contents are not NUL-terminated, so
even a merely-slightly-out-of-range offset can scan arbitrarily far past both the string table and
the whole mmap'd file until it happens to hit an unmapped page (SIGSEGV) or, worse, an accidental
`/\n` byte pair somewhere in adjacent process memory, silently returning attacker-uncontrolled but
process-memory-derived bytes as the "filename". With no string-table member present at all (as in
this repro), `strtab.data()` is `nullptr`, so any nonzero offset produces a wild low-address
pointer that reliably faults.

## Introduced by

8e709d2b6 "Split mold.h into multiple headers" (2022-01-14) - this is the commit that moved the
existing `read_name`/`ArHdr` code into `archive-file.h` in essentially its current form; the
unchecked-offset pattern itself appears to predate this specific commit (not a regression it
introduced, just the oldest point `git blame` can attribute for this file's current structure) -
not further bisected past this point since the underlying pattern is present in every commit that
touches this function up to the file being renamed `.cc`.

## The fix

`patch.diff`: turned `ArHdr::read_name` into a member template taking `Context<E> &ctx` and the
owning file's name (both already available at both call sites in `read_thin_archive_members`/
`read_fat_archive_members`), so it can report through mold's existing `Fatal(ctx) << ...`
diagnostic path instead of dereferencing blindly:

- Validates `0 <= offset < strtab.size()` before touching `strtab` at all; `Fatal`s with a clear
  message ("archive string table offset out of range") otherwise.
- Replaces the unbounded `strstr(start, "/\n")` (which can read past the mapped region looking for
  a terminator that was never guaranteed to exist) with a bounded `std::string_view::find("/\n")`
  over the remaining `strtab.substr(offset)` - `Fatal`s ("corrupted long filename in archive
  string table") if no `/\n` is found within the string table's own bounds, instead of scanning
  into adjacent memory.

Verified: `repro/run.sh` no longer crashes against the patched build - `mold` now exits 1 with
`mold: fatal: <path>: archive string table offset out of range`. Also re-tested with a real,
well-formed archive containing a genuine long filename (`ar rcs good.a
this_is_a_very_long_object_filename_to_force_long_name.o`, name > 15 chars so `ar` itself is
forced to use the SysV/GNU long-name extension) linked with `--whole-archive` - links
successfully, confirming the fix doesn't reject legitimate long filenames.

- Project test suite (`ctest -j5` in `build-asan/`): 475/476 passed before and after the patch,
  identical single failure both times (`x86_64-run-clang`, a pre-existing, patch-unrelated
  ASAN/`LD_PRELOAD` interaction with mold's own `mold-wrapper.so` - see
  `___BUGHUNT___/README.md`/`projects/mold.md`, confirmed pre-existing on unpatched `HEAD` last
  session). No new failures introduced by this patch.

## What's not done / open questions

- `ArHdr::read_name`'s **other** branch (BSD-style `#1/N` long filenames) has the same shape of
  problem: `int namelen = atoi(ar_name + 3)` is used unchecked to slice `ptr` (the member body) -
  a negative or oversized `namelen` isn't validated against the member's actual size before
  `std::string name{(char *)ptr, (size_t)namelen}` and `ptr += namelen`. Not investigated as part
  of this candidate (different code path, would need its own repro); added back to
  `___BUGHUNT___/IDEAS.md` as a new backlog item.
- The `read_thin_archive_members` variant of this same function was not separately re-verified
  with its own repro (thin archives reference external files by name rather than embedding member
  data) - the vulnerable `read_name` call is identical code, so the fix covers both callers, but
  no dedicated thin-archive crash input was constructed this session.
