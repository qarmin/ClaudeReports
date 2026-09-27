# gdb_index_compunit_header_unchecked_length

`read_compunits()` in `src/gdb-index.cc` (loop at lines 704-714 of the base commit) parses the
compile-unit headers of every input `.debug_info` section when mold is run with `--gdb-index`. It
reads each CU's `unit_length`, computes the stride `size` from it, appends a `Compunit` record, and
advances `p += size` - with no validation of `unit_length` and a loop condition of only `p < end`.
A prior static audit flagged this (and the sibling `read_pubnames`/`read_pubnames_cu` functions)
for a signed/unsigned overflow of the length field (A) and for reading a multi-byte header with no
minimum-size guard (B).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read current source of `read_compunits`/`read_pubnames`/`read_pubnames_cu`/`get_dwarf_kind`, and confirm the integer types via `lib/integers.h` | code reading | Confirmed: `CuHdrDwarf2_32::size` is `U32<E>` -> `->size + 4` is 32-bit unsigned arithmetic (wraps mod 2^32); `CuHdrDwarf2_64::size` is `U64<E>` -> `->size + 12` in u64, then assigned to `i64 size` (negative when the top bit is set). Both feed `p += size` with no `p + size <= end` bound; loop is only `for (u8 *p = begin; p < end;)`. |
| Build plain Debug mold; craft a real DWARF32 object and rewrite the first CU `unit_length` to `0xFFFFFFFC` (so `size = len+4` wraps to 0), link with `mold --gdb-index` under `timeout` + `ulimit -v` | `build-plain` (Debug, gcc 15.2.0), one crafted `.o`, `ulimit -v 10GiB`, `timeout 25s` | **Reproduced**: mold aborts (exit 134). `gdb` backtrace (frame #16 = `gdb-index.cc:712`, the `cus.push_back(Compunit{...})`) shows `std::vector<mold::Compunit>::_M_realloc_append` growing to 16.7M elements (~1.28 GB) before mimalloc's new-handler fails under the cap. The size-0 stride means `p += 0` -> the `p < end` loop never terminates. |
| Same with a real DWARF64 object (`gcc -gdwarf-5 -gdwarf64`), 64-bit `unit_length` rewritten to `0xFFFFFFFFFFFFFFF4` (`size = len+12` wraps to 0 in u64) | `build-plain`, one crafted `.o`, same safeguards | **Reproduced**: identical runaway-allocation abort (exit 134). |
| DWARF64 `unit_length` set to `0x8000000000000000` (top bit set -> `i64 size` is a large negative number) | `build-plain`, same safeguards | **Different behavior**: `p += (negative)` jumps `p` far out of `[begin,end)`, so `p < end` fails and the loop exits after **one** iteration - having pushed a single `Compunit{offset=0, size=-9.2e18}` (a corrupt record whose declared size runs past the section). mold exits 0 with no diagnostic. This is the "oversized/past-end record written verbatim into the output CU list" secondary effect the task's (A) also describes. |
| Oversized-but-positive DWARF32 `unit_length` `0x00FFFFFF` (16 MB, far larger than the ~292-byte section) | `build-plain` | Same as the negative case pre-patch: loop exits after recording one corrupt CU (size 16 MB > section), exit 0. Caught cleanly post-patch. |
| Repeat-run bar: 5x each for the two aborting inputs, 3x baseline control | `build-plain` | `evil-dwarf32` 5/5 abort (134); `evil-dwarf64-zero` 5/5 abort (134); unmodified baseline 3/3 exit 0. Deterministic, no layout dependence. |
| Re-confirm bug live on a freshly-rebuilt pristine binary (CONFIRMATION section 9) | `build-plain` rebuilt from reverted source | `evil32` -> 134, baseline -> 0. Bug present on genuinely pristine code. |

## Status

**Confirmed**

Reproduction rate: **5/5 runs** for each of the two size-wraps-to-zero inputs (DWARF32
`unit_length=0xFFFFFFFC` and DWARF64 `unit_length=0xFFFFFFFFFFFFFFF4`), plain Debug build
(`-DCMAKE_BUILD_TYPE=Debug`, gcc 15.2.0). Deterministic (same abort location every run, no ASLR/heap
dependence). Symptom is a runaway `std::vector<Compunit>::push_back` loop (`gdb-index.cc:712`):
under `timeout` alone it hangs; under a `ulimit -v` cap it aborts (SIGABRT, exit 134) within ~1-2s
when mimalloc's allocation fails. Both are the same underlying denial-of-service. This is sub-case
**(A)** from the task.

**Fully reproducible: Yes.** `repro/run.sh` compiles a 100%-real gcc DWARF object, overwrites a
single length field in its actual `.debug_info` section, and links it with the real `mold` binary
via the exact `--gdb-index` CLI a user would run - no internal function is called directly and no
DWARF structure is hand-built beyond the one corrupted length. This is a single-participant finding
(a malformed input file fed to a parser's real entry point); CONFIRMATION.md section 11's
two-participant bar does not apply.

## Class

crash (denial-of-service via unbounded allocation / infinite loop from an unchecked, overflowing
length field; the DWARF64 path additionally exhibits signed-integer-overflow UB in `u64 -> i64`).

## Root cause

`read_compunits()` reads a compile unit's `unit_length` straight from the (untrusted) input
`.debug_info` section and computes the stride to the next CU as `size = unit_length + 4` (DWARF32,
in 32-bit unsigned arithmetic) or `size = unit_length + 12` (DWARF64, in u64 then narrowed to a
signed `i64`). Nothing validates `unit_length`:

- A DWARF32 `unit_length` of `0xFFFFFFFC` makes `unit_length + 4` wrap mod 2^32 to `0`.
- A DWARF64 `unit_length` of `0xFFFFFFFFFFFFFFF4` makes `unit_length + 12` wrap mod 2^64 to `0`.

With `size == 0`, `p += size` never advances, and the loop condition `for (u8 *p = begin; p < end;)`
stays true forever, so the function pushes `Compunit` records without bound until memory is
exhausted. Separately, a DWARF64 `unit_length` with the top bit set (e.g. `0x8000000000000000`)
becomes a large negative `i64`, so `p += size` walks the cursor out of range and the loop instead
exits after recording one `Compunit` whose declared `offset`/`size` extends past the section end -
those bogus values are then written verbatim into the output `.gdb_index` CU list. The missing
`p + size <= end` bound (and the missing minimum-header-size guard before `get_dwarf_kind`/the CU
header reads, sub-case B) is the same shape in the sibling `read_pubnames`/`read_pubnames_cu`
pubnames parser (`i64 size = hdr.size + offsetof(...) + sizeof(hdr.size)`, unbounded `strlen` on the
name tuples).

## Introduced by

a396fa40 "Re-implement --gdb-index" (2023-10-11) introduced `read_compunits()` in essentially its
current form - `i64 len = *(U32<E> *)p + 4; cus.push_back(...); p += len;` inside
`for (u8 *p = begin; p < end;)` with no bound on `len` (DWARF64 was `Fatal` at that time). The
DWARF64 signed-narrowing path (`((CuHdrDwarf2_64<E> *)p)->size + 12`) was added by af460006
"Support DWARF64 for --gdb_index" (2023-10-17). Confirmed via `git log -S`/`git show` on the
affected lines.

## The fix

`src/gdb-index.cc` only (independent of sibling candidate 024, which patches `read_rnglist_range()`
- no overlapping lines):

- `read_compunits()`: compute the stride in `u64` (so the wrap produces a small value, not a
  negative `i64`), then require `hdr_size <= size <= end - p` before use - `Fatal(ctx)` on a
  corrupt length. This rejects the wrap-to-zero (infinite loop) and top-bit-set (negative
  stride / past-end record) inputs alike.
- `get_dwarf_kind()`: take the section `end` and require the CU header bytes it reads (4-byte
  escape word, then the full 32- or 64-bit header) to be in-bounds (sub-case B).
- `read_pubnames_cu()`/`read_pubnames()`: same u64-computed length bound against the section end,
  a bounded tuple loop (`p + sizeof(Offset) <= end`), and `memchr` instead of `strlen` for the
  name so a missing NUL cannot run off the section.

Verified against the exact triggering inputs plus additional same-class inputs per CONFIRMATION.md
section 7 (DWARF32 wrap-0, DWARF64 wrap-0, DWARF64 top-bit-set-negative, and an oversized-positive
DWARF32 length): pre-patch each either aborts (exit 134) or silently records a corrupt CU (exit 0);
post-patch all four produce a clean
`mold: fatal: <obj>: --gdb-index: corrupted .debug_info: invalid compile unit length`, while the
unmodified baseline object still links successfully (exit 0). See `output.txt` for the captured
before/after runs.

- Project test suite: **476/476 passing before, 476/476 passing after** (`ctest --test-dir
  build-plain -j4`, plain Debug build; 0 failed both times). All 10 `gdb-index*` tests pass after
  the patch, including `gdb-index-dwarf5`, `gdb-index-dwarf64`, `gdb-index-rnglistx` and
  `gdb-index-split-dwarf`, so the length bound does not disturb legitimate DWARF32/64/5 parsing.

## What's not done / open questions

- Sub-case (B) (minimum-header-size before parsing) is fixed defensively but was not independently
  crash-reproduced: an uncompressed `.debug_info` truncated a few bytes after the last complete CU
  reads silent zero bytes from the mmap page tail on this host (see project README's mmap-page note),
  so it does not fault. The task notes a *compressed* `.debug_info` truncated mid-header (heap
  buffer, ASAN-instrumented) as the variant most likely to fault; not attempted this session because
  sub-case (A) already gave a deterministic 5/5 confirmation.
- The DWARF64 top-bit-set (`0x8000...`) and oversized-positive inputs demonstrate the "corrupt CU
  record written to the output" effect (exit 0, no crash) rather than a hang; the same fix closes
  both. Whether a downstream read of that corrupt `offset`/`size` in `write_gdb_index` could itself
  be turned into an OOB write was not separately pursued once the length bound made the record
  unreachable.
- The pubnames hardening (`read_pubnames`/`read_pubnames_cu`) is included in the patch and covered
  by the existing gdb-index tests, but no crafted malformed-pubnames input was built to
  independently confirm a pre-patch crash there - it shares the identical unchecked-length shape as
  the confirmed `read_compunits` bug and is fixed by symmetry.
