# How `big_300k_rows.xlsb` was built

No public, sufficiently large, real-world-shaped `.xlsb` fixture was found (same search gap noted
in `___OPTIMIZATIONS___/README.md` and candidate 006's `RESULT.md`). Built one synthetically
instead, matching the shape of the existing `testdata_external/big_300k_rows.xlsx` fixture used by
candidates 001-006: 300,000 rows x 8 columns (2.4M cells), even columns numeric, odd columns a
shared string drawn from a 10-entry pool (`North/South/East/West/Active/Inactive/Pending/Closed/
Alpha/Beta`) - i.e. ~1.2M string cells sharing only 10 unique string values, heavier reuse than the
XLSX fixture's ~430 unique bytes but the same "many refs, tiny unique-text table" shape that
candidate 002/006 characterized as the interesting case for `Data::String` interning.

Built with SheetJS's `xlsx` npm package (community edition), `bookType: 'xlsb', bookSST: true`
(`gen_xlsb_fixture.js`, included here).

## A real bug found while building this fixture (not a calamine perf finding - see note below)

The first attempt (before `bookSST: true` and before the monkey-patch described below) produced a
file calamine's own `worksheet_range()`/`worksheet_cells_reader()` silently read as only 300,000
cells instead of 2.4M - **one cell per row instead of eight, no error, no panic.** Root cause:
`calamine`'s `src/xlsb/cells_reader.rs` `next_cell()`/`next_formula()` only recognizes the "long
form" MS-XLSB cell records (`BrtCellRk` 0x0002, `BrtCellIsst` 0x0007, `BrtCellSt` 0x0006, etc. -
0x0000-0x000B). MS-XLSB also defines "short form" variants for cells immediately following another
cell in the same row with no gap (`BrtShortRk` 0x000D, `BrtShortIsst` 0x0012, `BrtShortSt` 0x0011,
`BrtShortReal` 0x0010, `BrtShortBool` 0x000F, `BrtShortBlank` 0x000C, etc. - these omit the column
field entirely, relying on "previous column + 1"). SheetJS's own xlsb writer (`xlsx.js`,
`write_ws_bin_cell`) emits these short forms for every cell after the first non-gapped one in a
row - a legitimate, spec-compliant space optimization, not a malformed file. calamine's `_ =>
continue` catch-all silently skips every such record without adjusting `self.row`/column tracking,
so 7 of every 8 cells in this fixture's rows were dropped with no diagnostic of any kind. Confirmed
with a standalone debug reader (`debug_xlsb.rs`, not kept - built and discarded in the worktree
used for this candidate) that prints each record's raw type/length before/after the fix.

Worked around **for this fixture's own generation** by monkey-patching the local
`node_modules/xlsx/xlsx.js` copy used to build it (`write_ws_bin_cell`: force `last_seen = false`
unconditionally, i.e. always emit long-form records) - this does not touch calamine, it only
changes what the *fixture-generation tool* writes, so the shipped `.xlsb` here is 100%
spec-compliant, just written without short-form cells. **This finding is a real, non-performance
correctness bug in calamine itself** (silent data loss reading a spec-compliant file from at least
one real-world writer, not a crash) - reported separately, see this session's final report /
`___BUGHUNT___/` (not written directly into that shared folder by this candidate, per the
perf-optimize skill's batch-transcription convention - left for the batch-level step to fold in).

## Correctness of the shipped fixture

Verified against this session's own worktree (base commit `5c02e86`, before candidate 006's patch
was applied) that `range` and `cells` modes both report `cells=2400000` and identical `sig` values
across repeated runs - the fixture reads back exactly as generated, all 2.4M cells present, no
short-form-record cells lost.
