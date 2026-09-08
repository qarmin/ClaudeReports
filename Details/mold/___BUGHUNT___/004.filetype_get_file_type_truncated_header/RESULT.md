# filetype_get_file_type_truncated_header

`get_file_type()` in `src/filetype.cc` only checks `data.empty()` and the 4-byte `"\177ELF"`
magic before immediately dereferencing `e_ident[EI_DATA]` (byte offset 5), `e_ident[EI_CLASS]`
(byte offset 4), and later the full `ElfEhdr<E>` struct (up to 64 bytes for a 64-bit header) -
with no check that the mapped file is actually that large. Flagged by the prior static audit as
`src/filetype.cc:80-117` (seeded into `IDEAS.md`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | 4-byte file (`\x7fELF`, nothing else) and 8-byte file, fed to `build-asan/mold` (ASAN+UBSAN, `--thread-count 5`), both with and without an explicit `-m elf_x86_64` to bypass arch auto-detection | No crash, no ASAN diagnostic either time - `mold: fatal: -m option is missing` / `mold: fatal: tiny4.o: unknown file type`, both clean, expected error paths |
| Reasoning from `MappedFile`'s mmap semantics | Read `src/mapped-file-unix.cc` (mmap-based file mapping) | `mmap()` always maps in whole pages (minimum 4096 bytes on this host) regardless of the underlying file's actual size; bytes beyond the file's real length within that final page are kernel-zero-filled but **still mapped and readable**, not a fault. `get_file_type()` never reads past the first ~64 bytes of the file (the largest field it touches is the fixed-size `ElfEhdr<E>` struct), which is always well within the first mapped page no matter how small the file is (even a 1-byte file still gets a full readable page). Reading past `mf->size` here is real (an information/type-confusion issue, not memory-safety), but it can never reach an unmapped page and therefore can never SIGSEGV - a categorically different situation from candidates 001-003, where the OOB offsets involved (`e_shoff`, archive string-table offsets) are attacker-controlled and can be set to values that jump many pages past the mapped region. |

## Status

**Not reproducible.** Every truncated/malformed input constructed (files as small as 4 bytes)
produced a clean, correctly-handled error path (`unknown file type` / `-m option is missing`)
under both a plain build and the ASAN+UBSAN build - no crash, no sanitizer diagnostic. This isn't
a flaky negative; it's a structural property of how the file is mapped (see above) that rules out
a memory-safety crash for this specific finding, as long as `get_file_type()` itself never reads
more than one page's worth of bytes past `data()`, which is true for its current implementation
(reads are all fixed small offsets/struct-sized, no untrusted offset is dereferenced in this
function itself - unlike `is_gcc_lto_obj()`, which it calls next and which candidate 001 already
fixed for exactly that reason).

## Root cause (why the suspect turned out to be safe)

The static audit finding is technically correct that `get_file_type()` reads fixed struct
fields/offsets without checking `data.size()` against them first - so on paper this is an
out-of-bounds read. In practice, mold maps input files via `mmap()` (`src/mapped-file-unix.cc`),
which always rounds the mapping up to a whole number of pages; any file smaller than one page
(4096 bytes on this host, the overwhelmingly common case for a truncated/malformed file) still has
a full page mapped, with the tail zero-filled by the kernel rather than left unmapped. Since
`get_file_type()` only ever reads within the first 64 bytes of the file, that read can never leave
the first mapped page, so there is no unmapped memory to actually fault against. The bug is real
in the "reads bytes not logically part of the file" sense (those trailing zero bytes could
misclassify a truncated file, e.g. as `ELF_OBJ` when it should be rejected), but it is not the
crash the static audit's `type = "panic"` classification implied.

## Class

n/a (not reproducible as a crash)

## Introduced by

n/a - not applicable to a Not-reproducible result.

## The fix

No fix attempted - there is no confirmed defect to fix. If a maintainer wants to harden this
regardless (rejecting a file that's shorter than `sizeof(ElfEhdr<E>)` before returning anything
other than `FileType::UNKNOWN`/an error), that would be a defensive correctness improvement rather
than a crash fix - not pursued this session since there's no reproduction motivating it.

## What's not done / open questions

- Only tested on this host's default page size (4096 bytes, x86_64 Linux). A platform with a
  different mmap-granularity guarantee (unlikely on any mainstream Linux/macOS target mold
  supports) could theoretically behave differently - not considered a realistic concern.
- Did not check whether any *other* early-file-type-detection code path (before
  `is_gcc_lto_obj()`, which is already covered by candidate 001) reads further into the file than
  `get_file_type()` itself does for a still-tiny-but-larger-than-64-byte truncated input - not
  suspected to be an issue since the same "always at least one full page mapped" reasoning applies
  up to the full page size (4096 bytes), and no code path in `filetype.cc` reads anywhere near
  that far without an untrusted-offset dereference (which would be a different, already-covered
  bug shape).
