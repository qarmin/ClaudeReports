# input-files_parse_ehframe_oob_range_substr

While investigating the `lib/lib.h` `read_uleb`/`read_sleb` backlog item (unbounded LEB128
decoders with no length check, used throughout `.eh_frame`/DWARF parsing), looked at
`ObjectFile<E>::parse_ehframe()` (`src/input-files.cc`) as one of the most directly reachable
callers - `.eh_frame` is present in essentially every real-world compiled ELF object, no special
flags needed. Found a more immediately exploitable defect one line earlier than the `read_uleb`
calls: the CIE/FDE record-length field read straight from the file is used unchecked to slice the
section's `std::string_view` via the single-argument `substr(pos)` overload, which throws
`std::out_of_range` when `pos` exceeds the view's size - and mold does not catch it anywhere.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | Hand-built a minimal x86_64 `ET_REL` object with one `.eh_frame` section containing a single fabricated "CIE" record whose 4-byte length field is `0x7FFFFFF0` (huge) | Real, deterministic crash: uncaught `std::out_of_range`, `std::terminate()` |
| ASAN+UBSAN build (g++ 15.2.0) | `build-asan/mold -m elf_x86_64 --thread-count 5 -shared -o out evil.o` | `terminate called after throwing an instance of 'std::out_of_range'` / `basic_string_view::substr: __pos (which is 2147483636) > __size (which is 8)`, `SIGABRT` (exit 134) |
| Plain release build (no sanitizer) | `-DCMAKE_BUILD_TYPE=Release`, `build-plain/mold` | Identical uncaught exception and `SIGABRT` (exit 134) - confirms this isn't an ASAN-only artifact, and isn't a memory-safety bug in the traditional sense (no OOB memory access occurs - `std::string_view::substr` validates its own argument and throws instead of reading OOB) |
| Control input | A real object compiled with `gcc -fexceptions` (genuine `.eh_frame`/`.rela.eh_frame` sections) | Links successfully both before and after the fix |

## Status

**Confirmed** - 5/5 runs, byte-for-byte identical exception message and exit code every time
(fully deterministic - the crash depends only on the fabricated length value, not on memory
layout/ASLR).

## Fully reproducible

Yes. `repro/run.sh` builds the malformed object with `repro/gen_evil_elf.py` and runs the real
`mold` binary via its real CLI entry point end to end (`mold -shared -o <out> evil.o` -> input
parsing -> `parse_ehframe()`).

## Class

crash (uncaught C++ exception / `std::terminate()` on malformed, untrusted section data - not a
memory-safety issue in the traditional OOB-read/-write sense, since `std::string_view::substr`
itself is what detects the problem and throws; the bug is that mold never validates the untrusted
value before handing it to an API that can throw, and never catches the exception)

## Root cause

`parse_ehframe()` walks a `.eh_frame` section's raw bytes in a loop, reading a 4-byte record
length directly from untrusted file data (`i64 size = *(U32<E> *)data.data();`) with no validation
against how much of the section actually remains, then immediately does
`data = data.substr(size + 4);`. `std::string_view::substr(pos)` (the single-argument overload
used here, as opposed to the two-argument `substr(pos, count)` used by `CieRecord::get_contents()`
elsewhere in the same file, which *does* silently clamp an oversized `count`) throws
`std::out_of_range` whenever `pos > size()` - and a large-enough fabricated length field makes
`size + 4` exceed the section's remaining bytes by construction. mold has no `try`/`catch` around
any of its input-parsing pipeline, so the exception unwinds all the way to the runtime's default
terminate handler, printing the C++ runtime's own diagnostic and aborting the whole process
(`SIGABRT`) - not a controlled `Fatal()` error message the way most other malformed-input cases in
this codebase are handled.

## Introduced by

f4c5a8a42 "elf/input-files.cc" (2023-11-29) - this is where `parse_ehframe()`'s current loop
structure (`data.substr(size + 4)` on an unvalidated `size`) originates, per `git blame`.

## The fix

`patch.diff`: added two guards inside the CIE/FDE loop before the raw length is used for anything:

- `data.size() < 4` -> `Fatal()` (can't even read the length field itself) - handles the case
  where a previous record's declared length left fewer than 4 bytes remaining, which the original
  code would otherwise silently mis-read via `*(U32<E> *)data.data()` reading past `data`'s own
  view (though, per candidate 004's mmap-paging finding, this specific sub-case likely wouldn't
  fault on its own - it's fixed here anyway since it's now trivially covered by the same guard).
- `data.size() < 8 || (u64)size + 4 > (u64)data.size()` -> `Fatal()` (can't read the `id` field, or
  the declared record doesn't fit in what's left) - this is the guard that directly prevents the
  `substr(size + 4)` call from ever receiving an out-of-range `pos`.

Verified: `repro/run.sh` no longer crashes against the patched build - `mold` now exits 1 with a
controlled `mold: fatal: <path>:(.eh_frame): corrupted CIE/FDE record: bad length` message instead
of an uncaught exception. Also re-tested linking a real object compiled with `gcc -fexceptions`
(genuine `.eh_frame` + relocations) - links successfully with the patch applied.

- Project test suite (`ctest -j5` in `build-asan/`): 475/476 passed before and after the patch,
  identical single pre-existing, patch-unrelated failure both times (`x86_64-run-clang` - see
  candidates 001/003/005/006/007's own notes and `projects/mold.md`). No new failures introduced.

## What's not done / open questions

- This candidate only closes the specific `size + 4 > data.size()` path that throws via
  `substr(pos)`. The broader backlog item that led here - `read_uleb`/`read_sleb`
  (`lib/lib.h:378-388`) having no bounds check *at all* against any buffer end - remains open and
  unfixed. `parse_fde_encoding()` (called immediately after this candidate's fix, for every CIE
  that passes the new length check) still calls `read_uleb(&data)` multiple times on the CIE's
  augmentation data with no length awareness; a CIE whose declared length is valid (passes this
  candidate's new check) but whose augmentation-data-length field is itself a runaway/never-
  terminating ULEB could still walk `read_uleb` past the CIE's own (now correctly bounded) slice
  and, if that slice sits near the end of the object file's mapped pages, into unmapped memory -
  not verified this session; a natural next candidate.
- `CieRecord::size()`/`get_contents()` (`src/mold.h:436-442`) uses the *same* untrusted `size()`
  computation independently of `parse_ehframe()`'s own copy of the same field - it happens to be
  safe today only because `substr(pos, count)`'s two-argument form clamps rather than throws, but
  it still returns a `.eh_frame`-CIE view whose length can exceed the record's real, individually-
  declared boundary if two different records' length fields disagree in an unusual way. Not
  investigated further this session - noted as a related, lower-confidence suspect.
