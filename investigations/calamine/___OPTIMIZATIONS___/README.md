# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

Session focus (2026-08-28): **peak RSS / memory usage**, not wall-clock, primarily on
real-world-sized spreadsheet files (this project's own `tests/` fixtures are almost all tiny -
the only pre-existing large one is `tests/adhocallbabynames1996to2016.xls`, 18 MB).

## Workload

| file | source | size | shape | what it stresses |
|---|---|---|---|---|
| `testdata_external/big_300k_rows.xlsx` | SheetJS's own public test-file bucket (`test-files.sheetjs.com/8-by-300000-cells.xlsx`), used by SheetJS's own "large datasets" streaming docs | 20 MB | 1 sheet, 300,000 rows x 8 cols, mixed numeric/string cells | dense-grid materialization (`worksheet_range`) vs streaming (`worksheet_cells_reader`) on a genuinely large real xlsx |
| `testdata_external/formula_stress_test.xlsx` | SheetJS's public `oss.sheetjs.com/test_files/formula_stress_test.xlsx`, used in their own fetch/Ajax demo | 74 KB | many formula cells | formula-metadata/shared-formula parsing paths (small file, cheap to iterate many times, not a memory-stress fixture by itself) |
| `tests/adhocallbabynames1996to2016.xls` | already in this repo's own `tests/` (tracked in git) | 18 MB | legacy BIFF8 XLS, many rows | XLS-format parsing path, a second large real file so a finding isn't XLSX-only |

`big_300k_rows.xlsx` is the primary fixture for RSS work this session - it is the only file large
enough (20 MB on disk, 2.4M cells) to make a peak-RSS difference show up clearly above measurement
noise. Candidates needing an ODS or XLSB equivalent did not find one worth downloading in the time
budgeted for setup (searched sample-files.com, eforexcel/excelbianalytics.com - dead/blocked - and
a few others; nothing free-standing and reliably large was found for those two formats). If a
candidate specifically needs a large ODS/XLSB, either generate one synthetically (calamine's own
writer support, if any, or LibreOffice headless) and note that it's synthetic, or flag the gap
rather than silently skipping the format.

These files are not committed to the main project - they live only under
`___OPTIMIZATIONS___/testdata_external/`, which is itself untracked (this whole workspace is, per
SKILL.md). Re-download commands, so a fresh checkout of just this workspace can reproduce them:

```
curl -sL -o testdata_external/big_300k_rows.xlsx https://test-files.sheetjs.com/8-by-300000-cells.xlsx
curl -sL -o testdata_external/formula_stress_test.xlsx https://oss.sheetjs.com/test_files/formula_stress_test.xlsx
```

## Measurement harness

`harness/rss_probe/` - a standalone Rust binary (own `Cargo.toml`, path-dependency on calamine at
`../../..`) built specifically for this session because calamine's own `benches/basic.rs`
(criterion) measures wall-clock only, with no RSS instrumentation. Since this project is
untracked by the main repo's git history, every candidate's own `justfile` must copy it in from
here (see SKILL.md "Why per-candidate folders clone" / "When the project's own benchmark harness
is untracked").

Usage: `./target/release/rss_probe <mode> <file>`, modes:

- `open` - `open_workbook_auto` + `sheet_names()` only, no per-sheet parse. Baseline: cost of just
  opening the zip/CFB container and any eagerly-parsed metadata.
- `range` - opens + calls `worksheet_range()` for every sheet (the common/default API - builds the
  dense `Range<Data>` grid) and iterates every cell.
- `cells` - opens + calls `worksheet_cells_reader()` (streaming, XLSX/XLSB only - not on the
  generic `Reader` trait, so this mode resolves the concrete `Xlsx<_>`/`Xlsb<_>` type from the file
  extension itself rather than going through `Sheets`/`open_workbook_auto`) and streams cell by
  cell.

Each run prints one line: `mode=<m> file=<f> elapsed_ms=<n> peak_rss_kb=<n> end_rss_kb=<n> cells=<n> sig=<n>`.

- `peak_rss_kb` = `/proc/self/status` `VmHWM` (high-water mark), read once at process exit. This
  box has no `/usr/bin/time -v` installed (checked at session start), so VmHWM read directly from
  the process's own `/proc/self/status` is the measurement technique used, per SKILL.md's
  guidance for when `/usr/bin/time -v` isn't available.
- `end_rss_kb` = `VmRSS` at the same moment, for sanity-checking peak vs. resident-at-exit (a large
  gap between the two means something big was allocated and freed mid-run, not held throughout).
- `sig` = a cheap wrapping-add/XOR signature over cell positions and values, for a fast
  before/after correctness check without hashing the whole 2.4M-cell dataset by hand each time -
  NOT a substitute for the project's real test suite (`cargo test`), which is still mandatory per
  candidate per SKILL.md's Correctness section.
- `cells` = total cell count visited, a second cheap correctness signal (must match exactly
  between baseline and patched runs for a bit-exact candidate).

Smoke-tested at setup time (release build, single run each, `big_300k_rows.xlsx`, base commit
5c02e86):

| mode | elapsed_ms | peak_rss_kb | end_rss_kb | cells |
|---|---|---|---|---|
| open | 0 | 3636 | 3636 | 0 |
| range | 727 | **172316** | 3912 | 2,400,000 |
| cells | 468 | **3596** | 3596 | 2,400,000 |

This is the first real signal of the session, not a finding yet: `worksheet_range()` (the default,
most commonly used API) peaks at ~172 MB RSS on a 20 MB input with 2.4M cells, while the streaming
`worksheet_cells_reader()` path peaks at ~3.6 MB for the identical file/cell count. That ~48x gap
is exactly the shape of thing this session should chase into an actual candidate (is it the dense
`Range<Data>` grid's own size, `String` allocation overhead per shared-string cell, some transient
buffer not being freed, or something else) - not something to fix blind from this one number alone.

For paired/noise-floor comparisons across many runs, `tools/paired.sh` (copied from the skill's
templates, not yet adapted to this harness's exact CLI/output format - adapt its `ADAPT:` sections
before relying on it for a candidate).

## Instrumentation added for this workspace

| what | file | why |
|---|---|---|
| `rss_probe` binary (open/range/cells modes) | `harness/rss_probe/src/main.rs` | calamine has no existing memory-focused bench; `benches/basic.rs` is wall-clock/criterion only. Built once, reused by every RSS candidate this session. |
| Downloaded real-world large xlsx fixtures | `testdata_external/` | project's own `tests/` fixtures are almost all KB-sized; a genuinely large (multi-MB, multi-hundred-thousand-cell) real file was needed to make peak-RSS differences visible above noise. |
| `rss_probe` `formula` mode (added by candidate 013) | `harness/rss_probe/src/main.rs` | measures `worksheet_formula()` instead of `worksheet_range()` - no existing large formula-heavy fixture was found publicly, so candidate 013 also ships a synthetic-fixture generator (`013.../scripts/gen_formula_xlsx.py`, stdlib `zipfile` only) rather than adding another `testdata_external/` download. |

## Environment

- CPU: AMD Ryzen 9 7900 12-Core Processor, 24 logical cores (`nproc`).
- `rustc 1.98.0`, `cargo 1.98.0`, kernel `7.0.0-30-generic`.
- No `/usr/bin/time -v` installed on this box - use `/proc/self/status` `VmHWM` instead (built into
  `rss_probe` itself, see above). `valgrind` is available if a candidate needs massif-level
  allocation-site detail beyond a single peak-RSS number.
- `perf` availability not yet checked this session (checked lazily by whichever candidate first
  wants profiling, per SKILL.md's discovery step 1).
- Disk: 118 GB free on `/home/rafal/Projekty/Obce/calamine`'s filesystem at session start - ample
  for this project (no huge `target/` history yet), but re-check with `df -h` if candidates start
  building large clones per SKILL.md's disk-space watch.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus the project's own tests (`cargo test`) run before and after.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a short `CORRECTION.md` in its
   folder explaining what changed and why, and update its `SUMMARY.md` row to point at it.
5. This session's metric of record is **peak RSS** (`VmHWM` via `rss_probe`), not wall-clock -
   state explicitly in each `RESULT.md` which dimension(s) were measured; a wall-clock regression
   with a real RSS win (or vice versa) is a `Mixed` result, not silently dropped.
