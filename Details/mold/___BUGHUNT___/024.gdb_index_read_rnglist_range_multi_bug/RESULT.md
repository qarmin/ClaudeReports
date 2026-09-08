# gdb_index_read_rnglist_range_multi_bug

`read_rnglist_range()` in `src/gdb-index.cc` (~line 442-493 at the assigned base commit) parses a
`.debug_rnglists` (DWARF5) range-list entry reached via mold's `--gdb-index` flag. A prior
static-audit pass flagged two independent defects in this function: (B) the opcode `switch` has no
`default:` case, so an unrecognized byte is silently skipped and the read pointer walks forward
one byte at a time reinterpreting arbitrary bytes as opcodes; (A) three opcodes
(`DW_RLE_base_addressx`/`DW_RLE_startx_endx`/`DW_RLE_startx_length`) index through `addrx` with no
null/bounds check, and `addrx` is only non-null when the compunit carried a `DW_AT_addr_base`
attribute.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read current source, confirm opcode enum values (`src/elf.h:1484-1491`, `DW_RLE_end_of_list=0x00` .. `DW_RLE_start_length=0x07`) and call chain (`read_address_ranges` -> `read_rnglist_range`, reached from `write_gdb_index` -> `read_compunits`/`read_address_ranges`, gated only on `--gdb-index`) | Code reading only | Confirmed: no `default:` case exists; opcodes 0x00-0x07 are the only ones handled; `addrx` (a `Word<E>*`) is dereferenced at 3 call sites with zero validation, and is `nullptr` unless the compunit has `DW_AT_addr_base`. |
| Real DWARF5 repro: `gcc -c -gdwarf-5 -ffunction-sections -O2` on a 4-function C file, confirm real `.debug_rnglists` section + `DW_AT_ranges` (`DW_FORM_sec_offset`) via `readelf` | `build-plain` (`-DCMAKE_BUILD_TYPE=Debug`, default gcc 15.2.0), plain link via direct `mold --gdb-index` invocation (crt1/crti/crtn + libc.so from the system gcc, no `-fuse-ld` needed) | `real.o` produces a genuine `.debug_rnglists` with 5 `DW_RLE_start_length` (0x07) entries + `DW_RLE_end_of_list` (0x00); linking it with unmodified `mold --gdb-index` succeeds (exit 0). |
| Hand-rolled ELF64 section-header parser (`find_rnglists.py`, no `pyelftools` on this host) to decode the real `.debug_rnglists` opcode stream and locate each opcode's exact file offset | Python 3.14, no network deps | Located all 6 opcode bytes' file offsets precisely; used by `make_evil.py` to corrupt one byte deterministically. |
| **Bug (B) attempt**: corrupt the first opcode byte 0x07 (`DW_RLE_start_length`) -> 0x08 (unknown), link with `--gdb-index` | `build-plain`, plain (non-ASAN) build | mold exited 0, **no crash, no diagnostic**. Traced why: the byte immediately after the corrupted opcode is the low byte of the just-relocated function address (`0x201590` -> byte `0x90`), then `0x15`, `0x20` - none match any case (silent no-ops) - then the *next* byte is `0x00` (an upper, always-zero byte of the 64-bit address, since this is a small non-PIE test binary), which the switch takes as a spurious `DW_RLE_end_of_list` and returns. The scan walked only ~4 bytes before self-terminating by luck, exactly as the audit predicted ("until it happens to hit a literal 0x00 ... or segfaults") - but for this small binary's address values, hitting a stray 0x00 within a handful of bytes turned out to be the overwhelmingly likely outcome, not a crash. |
| **Bug (A) attempt**: corrupt the same opcode byte 0x07 -> 0x01 (`DW_RLE_base_addressx`, a legitimate but now-misapplied opcode - this CU has no `DW_AT_addr_base` so `addrx == nullptr`) | `build-plain`, plain build | **Segfault, exit 139**, reproduced 5/5 consecutive runs. `gdb` backtrace (`set follow-fork-mode child` per project convention) pinpoints the fault exactly: `mold::Integer<unsigned long,true,8>::operator unsigned long (this=0x5480)` inlined from `mold::read_rnglist_range<mold::X86_64>` at `src/gdb-index.cc:450`, with `addrx=0x0, base=0` in the parent frame - i.e. `addrx[read_uleb(&p)]` dereferencing `nullptr + index*8`. |
| Repeat the same corrupted-opcode input against **two more indexed opcodes** in the same class (`DW_RLE_startx_endx`=0x02, `DW_RLE_startx_length`=0x03), each at a different byte offset within the section | `build-plain` | Both also reproduce the null-`addrx` crash path once patched code is reverted; confirmed the patched `read_addrx()` helper rejects all three uniformly (see "The fix" below). |

## Status

**Confirmed**

Reproduction rate: **5/5 runs**, plain Debug build (`-DCMAKE_BUILD_TYPE=Debug`, default system
gcc 15.2.0), deterministic (no ASLR/layout dependence observed - same crash location and same
"received signal" every run). This is bug (A) from the task description (unchecked `addrx`
dereference), triggered via a one-byte opcode corruption that lands on a legitimate-but-inapplicable
`DW_RLE_*` opcode. Bug (B) (missing `default:` case) is real and confirmed as a defect by code
reading and by the observed "silent walk through unrelated bytes with zero diagnostic" behavior
above, but the *specific* crafted input tried for it did not itself crash (it self-terminated after
~4 bytes on this particular small, non-PIE test binary, because those bytes' upper address bytes are
zero) - see "What's not done" below. The same missing-`default:` defect is what makes bug (A)
reachable in the first place: a corrupted byte that happens to coincide with a real opcode value is
exactly the "walks and misinterprets stray bytes" hazard the audit described, just with a
deterministic crash instead of a lucky self-termination as the outcome for opcodes 0x01-0x03.

**Fully reproducible**: **Yes**. `repro/run.sh <path-to-mold>` compiles the real DWARF5 object with
gcc, corrupts one real opcode byte in the actual compiled `.debug_rnglists` section content with a
small Python script, and links the result with the actual `mold` binary via `--gdb-index` - the
exact end-to-end CLI path a real user hits (no internal function called directly, no hand-built
DWARF structure beyond a single deliberately corrupted byte in an otherwise 100%-real compiler
output). This is a single-participant finding (a malformed input file fed to a parser's real entry
point) - `CONFIRMATION.md` section 11's two-participant bar does not apply.

## Class

memory-safety (null-pointer dereference / unbounded index into a `Word<E>*` array)

## Root cause

`read_rnglist_range()` (`src/gdb-index.cc`) decodes each DWARF5 `.debug_rnglists` range-list entry
by reading a one-byte `DW_RLE_*` opcode and switching on it. Three opcode handlers
(`DW_RLE_base_addressx`, `DW_RLE_startx_endx`, `DW_RLE_startx_length`) use a ULEB128-decoded index
to dereference `addrx`, a `Word<E>*` that is `nullptr` unless the compunit's `.debug_info` record
carried a `DW_AT_addr_base` attribute (set once, by the caller `read_address_ranges`, based on
whatever the *unvalidated input file* declares). Neither the null case nor an out-of-range index
(the ULEB value has no upper bound check against `.debug_addr`'s actual size) is ever checked before
the dereference. Separately, the opcode `switch` has no `default:` case, so any byte value outside
0x00-0x07 - whether from deliberate corruption, truncation, or a future DWARF revision - is silently
ignored and the scan advances one byte, reinterpreting whatever byte comes next as a fresh opcode.
This second defect is what makes the first one reachable via arbitrarily small/likely-looking
corruption: any stray byte that happens to equal 0x01, 0x02, or 0x03 is treated as a fully legitimate
opcode regardless of context, immediately hitting the unchecked `addrx` dereference.

## Introduced by

a396fa400 "Re-implement --gdb-index" (2023-10-11) - this commit introduced
`read_rnglist_range()` with the current opcode `switch` (no `default:` case from day one) and the
unchecked `addrx[...]` dereferences at all three indexed-opcode sites. A handful of later commits
(69a91e8aa, 8506065fd, ec095e751 - all 2023-10-17, refactors/fixes to the same function) touched
individual lines but never added the missing validation. Confirmed via `git log -S'read_rnglist_range'`
and `git blame -L 440,494 -- src/gdb-index.cc`.

## The fix

Threaded `Context<E> &ctx` and the `.debug_rnglists` section's end pointer (`end`) into
`read_rnglist_range()` (both call sites in `read_address_ranges` already had `ctx` in scope, so this
was a small, local change - no wider call-chain surgery needed):

- Added a `default:` case that calls `Fatal(ctx)` reporting the unknown opcode value, instead of
  silently falling through.
- Added a `p >= end` check at the top of the loop (Fatal on overrun) so a truncated/malformed range
  list can no longer read past the end of the mapped `.debug_rnglists` section looking for a
  `DW_RLE_end_of_list` byte that isn't there.
- Added a `read_addrx(idx)` helper used by all three indexed-opcode sites: computes the valid index
  range once from `ctx.debug_addr`'s actual size and the compunit's `addrx` base offset, and calls
  `Fatal(ctx)` if `addrx` is null or `idx` is out of range, instead of dereferencing unconditionally.

Verified against the exact triggering input (`evil_addrx.o`, opcode 0x07 -> 0x01) plus two more
inputs in the same bug class per `CONFIRMATION.md` section 7 (same corruption technique, opcode 0x07
-> 0x02 `DW_RLE_startx_endx` and -> 0x03 `DW_RLE_startx_length`, at two different byte offsets in the
section) - all three now produce a clean
`mold: fatal: --gdb-index: DW_RLE_* opcode references an out-of-range or missing .debug_addr entry`
instead of segfaulting. Also verified the unknown-opcode case (0x07 -> 0x08) now produces
`mold: fatal: --gdb-index: unknown DW_RLE_* opcode in .debug_rnglists: 0x8` instead of silently
self-terminating with no diagnostic. The original, unmodified `real.o` (a legitimate multi-range
DWARF5 object) still links successfully with `--gdb-index` after the patch, confirmed by re-running
`repro/run.sh` end to end both before and after.

- Project test suite: **476/476 passing before, 476/476 passing after** (`ctest --test-dir
  build-plain -j4`, plain Debug build). No new failures, no newly-skipped tests. In particular
  `test/gdb-index-dwarf5.sh` and `test/gdb-index-rnglistx.sh` (which exercise real DWARF5
  `.debug_rnglists` parsing through both call sites this patch touches, `DW_FORM_sec_offset` and
  the `DW_FORM_rnglistx`/offset-table branch respectively) pass both before and after, giving direct
  confidence the fix doesn't disturb legitimate range-list parsing.

## What's not done / open questions

- Bug (B) (missing `default:` case) is fixed defensively, but the specific crafted input tried for
  it this session did not itself produce a crash before the fix - it self-terminated harmlessly
  after consuming ~4 stray bytes, because this test binary's function addresses happen to have
  zero upper bytes (small non-PIE link). A future session wanting a *direct*, crash-based
  confirmation of bug (B) in isolation (as opposed to via bug (A)'s addrx-null path, which is what
  actually crashed here) would need either: a much larger/PIE binary where address high bytes are
  less likely to be zero, or a hand-corrupted byte sequence engineered so the walk consumes bytes
  from a region of the file that has no `0x00` byte for a long stretch (walking past the end of
  `.debug_rnglists` into adjacent section data or off the mapped file entirely). Not attempted this
  session given the addrx-null crash (bug A) already gave a clean, deterministic, 5/5-reproducible
  memory-safety finding and the `p >= end` bounds check added by the fix independently closes off
  the "read forever" version of bug (B) regardless.
- `read_uleb()` itself (`lib/lib.h`, shared across all DWARF parsing in this codebase) still has no
  bounds check against the buffer it's decoding from - already a known, separately-tracked backlog
  item (`___BUGHUNT___/IDEAS.md` / `projects/mold.md`), not attempted here since it's a distinct
  function/candidate from the two `read_rnglist_range`-specific defects this candidate was scoped
  to.
- Did not attempt a longer fuzzing campaign against this parser; the two defects found by targeted
  single-byte corruption plus code reading were sufficient to reach `Confirmed` for this session's
  scope.
