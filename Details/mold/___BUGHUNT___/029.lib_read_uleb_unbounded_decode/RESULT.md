# 029.lib_read_uleb_unbounded_decode

`read_uleb`/`read_sleb` in `lib/lib.h` (~378-410) decode LEB128 with no length
parameter and no bounds check: `do { byte = *(*buf)++; ... } while (byte & 0x80);`.
They consume bytes for as long as the 0x80 continuation bit is set, so a crafted
run of continuation bytes walks the pointer forward past the end of its buffer.
This was flagged in the project risk map and deferred twice because the originally
planned `.eh_frame` page-boundary repro needed fiddly page-alignment control.

The reachable, deterministic path is mold's `--gdb-index` DWARF parser
(`src/gdb-index.cc`), building on the DWARF-repro tooling from sibling candidates
024/027.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Enumerate call sites + what bounds each | code read of all ~30 `read_uleb`/`read_sleb` callers | `.debug_abbrev` walk in `find_cu_abbrev` (gdb-index.cc:323) reads from `ctx.debug_abbrev` with no end pointer; the buffer is `new u8[sh_size]` (heap) when both `--gdb-index` and `--compress-debug-sections` are set (output-chunks.cc:3358 / get_buffer:759) -> ASAN-instrumentable |
| fill80: overwrite entire `.debug_abbrev` of a `gcc -g` DWARF5 object with 0x80, link `--gdb-index --compress-debug-sections=zlib`, ASAN build | build-asan (g++, `-DMOLD_USE_ASAN=ON`), 165-byte abbrev section | **ASAN heap-buffer-overflow READ of size 1** in `mold::read_uleb` (lib.h:383) via `find_cu_abbrev` (gdb-index.cc:323); exit 1. Reproduced **5/5** |
| Same on a DWARF4 object (`-gdwarf-4`) | build-asan | Same ASAN heap-buffer-overflow via `find_cu_abbrev<CuHdrDwarf2_32>` -- not DWARF5-specific |
| tail80: set only the last 8 bytes of `.debug_abbrev` to 0x80 | build-asan | **No crash (exit 0)** -- `find_cu_abbrev` matches abbrev #1 at the start of the table and never walks to the tail, so a short tail run is simply never reached. Kept in `run.sh` as an informational contrast |

## Status

**Confirmed** -- ASAN heap-buffer-overflow READ, reproduces **5/5** runs, deterministic,
on the ASAN+UBSAN build (`-DMOLD_USE_ASAN=ON`, g++). A UBSan `shift exponent 35 is
too large for 32-bit type 'int'` (lib.h:384) fires first as a recoverable warning
from the same overlong decode, then the walk reaches the heap-buffer boundary and
ASAN aborts. Under a plain (non-ASAN) build the all-0x80 run self-terminates on the
first adjacent heap byte with a cleared top bit and does not fault, which is why the
ASAN + compressed-section route (heap buffer with a redzone at exactly `sh_size`) is
what makes it a precise, caught fault rather than a page-alignment gamble.

**Fully reproducible**: **Yes**. `repro/run.sh` drives the real `mold` binary
end-to-end (`mold --gdb-index --compress-debug-sections=zlib ... evil.o`) on a real
gcc-compiled object whose real `.debug_abbrev` bytes are corrupted -- the same single
external artifact a malicious/broken `.o` would present. No second participant is
faked.

## Class

memory-safety (heap-buffer-overflow READ; co-occurring shift-exponent UB).

## Root cause

`read_uleb(u8 **buf)` has no upper bound. In `find_cu_abbrev`, the abbrev-table
walk starts at `&ctx.debug_abbrev[0] + hdr.abbrev_offset` and the first
`read_uleb(&abbrev)` reads the abbrev `code`. When the abbrev bytes are an
uninterrupted run of 0x80-continuation bytes, the decoder never sees a cleared
continuation bit within the section and keeps advancing `*buf` past the end of the
buffer. With `--gdb-index --compress-debug-sections`, that buffer is a heap
allocation of exactly `ch_size` bytes (`CompressedSection`: `new u8[sh_size]`,
kept in `uncompressed_data` only when `--gdb-index` is set), so the overrun is a
genuine heap out-of-bounds read caught precisely at the boundary. The same
unbounded loop also shifts by `>= 32` on an overlong encoding, which is UB
(the UBSan diagnostic).

## Introduced by

not determined to a single fix commit -- the unbounded decoder has existed since the
function was first introduced. `git log -S` on the decode loop bottoms out at
19b1b025 "[Mach-O] Dump export table entries" (2021-10-02, v1.0.0 era); later commits
only move/rename it (d2f8bffd renamed lib/common.h -> lib/lib.h; 5b859497 reorganized
gdb-index). The specific `find_cu_abbrev` caller was added with the `--gdb-index`
feature. So this is long-standing rather than a recent regression.

## The fix

Added bounds-checked overloads `read_uleb(u8 **buf, u8 *end)` and
`read_sleb(u8 **buf, u8 *end)` in `lib/lib.h` that stop at `end` and cap `shift`
below 64 (removing both the OOB read and the shift UB), then threaded the section
bounds through the gdb-index DWARF readers that consume untrusted `.debug_abbrev`
/`.debug_info`: `find_cu_abbrev` (the abbrev-table + abbrev-code reads),
`read_scalar` (the `DW_FORM_udata`/`strx`/`addrx`... path), and the abbrev
name/form loop in `read_address_ranges`.

With the fix, the all-0x80 abbrev walk reaches `end`, `read_uleb` returns 0, and the
existing `if (code == 0) Fatal(...)` fires: `mold: fatal: --gdb-index: .debug_abbrev
does not contain a record for the first .debug_info record` (exit 1). No ASAN error,
no UBSan warning.

- Repro re-verified against the patched build: Case A (DWARF5 fill80) and Case B
  (DWARF4 fill80) both now exit 1 with the clean Fatal instead of the ASAN abort
  (checked against two distinct same-class inputs per CONFIRMATION section 7). The
  unmodified-object baseline still links cleanly (exit 0).
- Project test suite (`ctest --test-dir build-asan -j4`): **475/476 before -> 475/476
  after**. The one failure is `x86_64-run-clang`, a pre-existing ASAN-only failure
  documented in `___BUGHUNT___/README.md` (mold's `mold-wrapper.so` `LD_PRELOAD`
  collides with ASAN's own preload), unrelated to this change and failing identically
  on pristine HEAD. All 10 `x86_64-gdb-index-*` tests pass after the fix, including
  `x86_64-gdb-index-compress-output` (the exact compressed-output path the repro
  uses) and dwarf2/3/4/5/64/rnglistx/split-dwarf.

## What's not done / open questions

- The fix bounds the abbrev/debug_info `read_uleb` sites in gdb-index.cc that the
  repro exercises. Other `read_uleb`/`read_sleb` callers with the same unbounded
  decoder were NOT converted this session and remain same-class candidates:
  - `read_rnglist_range` (gdb-index.cc:~450-490) reads `.debug_rnglists` with the
    unbounded overload; it also has fixed-size overreads that are candidate 024's
    territory, so bounding it properly needs a joined fix and was left out of scope.
  - `parse_fde_encoding`'s CIE augmentation-data reads in `src/input-files.cc`
    (573-581) -- the original page-boundary lead.
  - `read_riscv_attributes` (input-files.cc:213-226), `src/passes.cc:2518`
    (`file->symbols[read_uleb(&p)]`), and `src/mold.h:3277-3317`
    (GNU_PROPERTY-adjacent relocation reads).
  A maintainer may prefer to make the unbounded overloads unavailable (or route all
  DWARF/attribute parsing through the bounded ones) rather than fixing callers one at
  a time; that is a wider decision than one candidate.
- A longer fuzz campaign (via `fuzz-bug-repro`) pointed at the `--gdb-index` DWARF
  readers would likely surface the sibling sites above and other malformed-DWARF
  faults; not run here.
