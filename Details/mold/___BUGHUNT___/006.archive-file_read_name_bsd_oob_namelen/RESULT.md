# archive-file_read_name_bsd_oob_namelen

`ArHdr::read_name()` in `src/archive-file.cc` handles the BSD extended-filename convention
(`ar_name` starting with `#1/N`, meaning the filename itself is the first `N` bytes of the
member's own data area) by computing `int namelen = atoi(ar_name + 3)` and immediately
constructing `std::string name{(char *)ptr, (size_t)namelen}`, with no check that `namelen`
actually fits within the archive's mapped extent. This is the sibling of candidate 003's now-fixed
SysV-style branch in the same function - noted as a follow-up backlog item while fixing that
candidate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | 1-member `!<arch>\n` archive whose sole member header uses BSD name `#1/999999999` (namelen ~1 billion) with only 8 real bytes of member data following | Real, deterministic crash |
| ASAN+UBSAN build (g++ 15.2.0) | `build-asan/mold --thread-count 5 -o out evil.a` | `AddressSanitizer: BUS` inside `std::string`'s copy constructor, called from `read_name`, `src/archive-file.cc:56` - a genuine OOB read walking far enough past the mapping to cross into an unmapped page |
| Plain release build (no sanitizer) | `-DCMAKE_BUILD_TYPE=Release`, `build-plain/mold` | Real `Bus error` (exit 135) - confirms this isn't an ASAN-only artifact |

## Status

**Confirmed** - 5/5 runs against the ASAN build, identical crash signature every time (a fixed,
large `namelen` walking a deterministic distance past a small mapping - not layout-dependent).
Also reproduces as a real `Bus error` in a plain, non-instrumented release build.

## Fully reproducible

Yes. `repro/run.sh` builds the malformed `.a` with `repro/gen_evil_ar.py` and runs it against the
real `mold` binary via its real CLI entry point (`mold -o <out> evil.a`) end to end.

## Class

crash (memory-safety: out-of-bounds read via an unchecked, attacker-controlled length used to
size a `std::string` construction)

## Root cause

Identical shape to candidate 003, one function up: `read_name()`'s BSD-style branch trusts the
decimal digits after `#1/` in `ar_name` as the exact byte length of a filename stored inline in
the member's own data (immediately following the 60-byte header). It never validates that length
against how many bytes are actually left in the archive's mapping before passing it to
`std::string`'s `(const char *, size_t)` constructor, which unconditionally copies that many bytes
starting at `ptr`. A `namelen` large enough walks past the mapped region entirely (SIGBUS/SIGSEGV,
depending on whether the walked-past memory happens to be entirely unmapped or crosses a
protection boundary first); a negative `namelen` would instead become an enormous `size_t` and hit
`std::string`'s own internal length limit (`std::length_error`, uncaught -> `std::terminate`/abort)
- not separately exercised this session since the large-positive case already gives a clean,
reliable memory-safety crash.

## Introduced by

8e709d2b6 "Split mold.h into multiple headers" (2026-01-14 per `git blame`; same commit and same
caveat as candidate 003 - this is the oldest point blame can attribute for this file's current
structure, not necessarily a later regression of an originally-safe version).

## The fix

`patch.diff`: extended `ArHdr::read_name` into a member template taking `Context<E> &ctx`,
the owning file's name, and the mapping's end pointer (`begin + mf->size`, already available at
both call sites) - the same shape of change candidate 003 made for the SysV branch, applied here
to the BSD branch instead:

- Validates `namelen >= 0 && ptr + namelen <= mapping_end` before constructing the `std::string`
  at all; `Fatal`s with a clear message ("archive member filename length out of range") otherwise.

Verified: `repro/run.sh` no longer crashes against the patched build - `mold` now exits 1 with
`mold: fatal: <path>: archive member filename length out of range`. Also re-tested linking the
real system `/usr/lib/x86_64-linux-gnu/libc.a` (a large, many-member fat archive with no BSD-style
members) - links successfully, confirming the fix doesn't disturb the common case.

- Project test suite (`ctest -j5` in `build-asan/`): 475/476 passed before and after the patch,
  identical single pre-existing, patch-unrelated failure both times (`x86_64-run-clang` - see
  candidates 001/003/005's own notes and `projects/mold.md`). No new failures introduced.

## What's not done / open questions

- This patch's `read_name` signature change is independent of, and was written against pristine
  `HEAD` rather than on top of, candidate 003's own patch to the same function's SysV branch -
  applying both `patch.diff`s to the same tree simultaneously would conflict (both touch the
  function's signature and the two call sites) even though neither's *fix* logically depends on
  the other. A maintainer merging both would need to combine them into one signature change; kept
  separate here per this workspace's "one bug per candidate" policy.
- The negative-`namelen` sub-case (`std::length_error`/abort via `std::terminate`, rather than a
  memory-safety SIGSEGV/SIGBUS) was reasoned about but not separately constructed/verified this
  session - the fix's `namelen < 0` check covers it regardless, but no dedicated repro exists for
  that specific crash *mechanism* (uncaught C++ exception vs. real OOB memory access).
