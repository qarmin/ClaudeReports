# Cap the dense grid `worksheet_range()`/`worksheet_range_ref()` build on XLSX/XLSB

`Range::from_sparse()` (the function XLSX's and XLSB's `worksheet_range()`/`worksheet_range_ref()`
use to turn parsed cells into a `Range`) allocates a dense grid sized from the min/max row/column
among the file's *actual* non-empty cells, with no upper bound. Two genuinely-positioned cells (one
near the origin, one at Excel's real max corner `XFD1048576`) are enough to request a
`rows * cols` grid of ~17.2 billion elements - a ~549 GB single allocation that aborts the process
(`memory allocation of 549755813888 bytes failed`, `SIGABRT`, not a catchable `Result`). Full
writeup and a standalone repro against the published `calamine = "0.36"` crate:
`___BUGHUNT___/001.xlsx_xlsb_dense_range_oom/`. ODS already guards the identical 17-billion-cell
shape with its own `MAX_CELLS` constant (per this project's own Changelog); XLSX/XLSB apparently
never got the equivalent guard.

## What was tried

Added `Range::from_sparse_bounded(cells, max_cells) -> Result<Range<T>, usize>` (`src/lib.rs`) -
same logic as the existing public `from_sparse`, but returns `Err(requested_len)` instead of
building the dense `Vec` if the computed grid size exceeds `max_cells`. The existing public
`from_sparse` now delegates to it with `max_cells = usize::MAX`, so its behavior/signature for
existing callers (including `src/xls.rs`, left untouched - not in scope, see below) is unchanged.
Both `worksheet_range_ref()` call sites in `src/xlsx/mod.rs` and both in `src/xlsb/mod.rs`
(the `Data` range builder and the `String` formula-range builder) now call
`from_sparse_bounded(cells, MAX_DENSE_RANGE_CELLS)` with `MAX_DENSE_RANGE_CELLS = 100_000_000`
(matches `ods.rs`'s existing cap) and map a rejection to a new `CellLimitExceeded { requested, max
}` variant on `XlsxError`/`XlsbError`.

| Variant | PoC file (`XFD1048576` + `A1`, 2 real cells) peak RSS | big real file (300,000x8, 2.4M cells) peak RSS | notes |
|---|---|---|---|
| baseline (`HEAD` 5c02e86) | process aborts attempting a 549 GB allocation (`ulimit -v 4000000`, exit 134) | 172,148-172,316 KB (3 runs) | `worksheet_range` |
| patched | 3,468-3,500 KB (3 runs), returns `Err(CellLimitExceeded)` cleanly, no `ulimit` needed | 171,684-172,148 KB (3 runs), `sig`/`cells` identical to baseline | `worksheet_range` |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (`range` mode; peak RSS = `/proc/self/status`
`VmHWM`), release build, single run each except where noted "(N runs)". PoC file built by hand,
see `___BUGHUNT___/001.xlsx_xlsb_dense_range_oom/repro/poc.xlsx`.

## Result

This session's metric is peak RSS, not wall-clock (wall-clock wasn't meaningfully different either
way for the big real file - both ~620-730ms - and is not the point of this candidate).

- **Adversarial input (the PoC file):** peak RSS drops from "process aborts trying to allocate 549
  GB" to a bounded ~3.5 MB. This is the actual fix - closing an unbounded/crash-causing allocation.
- **Legitimate large input (`big_300k_rows.xlsx`, 2.4M real cells):** peak RSS is unchanged within
  normal run-to-run noise (~172.0-172.3 MB baseline vs. ~171.7-172.1 MB patched, well under 1%
  spread across 3 runs each) and the `sig`/`cells` correctness signature is bit-identical
  (`sig=860639953605989024`, `cells=2400000` in every run, both baseline and patched) - the new cap
  (100,000,000 cells) sits far above any realistic worksheet, so normal usage is unaffected.

## Does it change behavior/output?

- **Bit-exact for every input this cap doesn't reject?** yes - `sig`/`cells` identical to baseline
  on `big_300k_rows.xlsx`, `formula_stress_test.xlsx`, and every other file exercised by the
  project's own test suite (see below); the new code path (`from_sparse_bounded` returning `Err`)
  is only reachable when the requested dense grid would exceed 100,000,000 cells.
- **What changes, for whom:** a file whose real cells' bounding box would require building a dense
  grid of more than 100,000,000 elements now returns `Err(XlsxError::CellLimitExceeded { requested,
  max })` / `Err(XlsbError::CellLimitExceeded { .. })` from `worksheet_range()`,
  `worksheet_range_ref()`, and `worksheet_formula()`, instead of the process aborting. No known
  legitimate spreadsheet needs a used range anywhere near 100M cells (Excel's absolute max sheet
  size is ~17.2 billion cells, but a *real* 100M-cell used range would itself already be an
  unusually large workbook, tens of GB in `.xlsx` form).
- **Is this a breaking change?** New enum variant added to `XlsxError` and `XlsbError` (both are
  `#[derive(Debug)]`, not `#[non_exhaustive]`) - a caller that exhaustively `match`es either enum
  without a wildcard arm will fail to compile until they add an arm for `CellLimitExceeded`. This
  is the same shape of change `ods.rs` already made for `OdsError::CellLimitExceeded` in this
  project's own history (per its Changelog entry), so there's in-project precedent for treating
  this as an acceptable minor-version addition. Behaviorally: no input that previously returned
  `Ok` can now return `Err` (the cap is far above anything reachable without hitting the abort
  first) - the only observable change is that the previously-crashing inputs now return an `Err`
  instead of aborting the process, which for anyone actually hitting this today is a strict
  improvement (a catchable error instead of a hard process abort).
- Project test suite: **165/166 integration tests passing both before and after (1 pre-existing
  `#[ignore]`d test, unrelated), 45/45 unit tests both before and after, 67/67 doc tests both
  before and after.** (`cargo test --all-features`, run once on unpatched `HEAD` via `git stash`,
  once on the patched tree - see command output captured during this session; no new failures
  either direction.) `cargo clippy --all-features --all-targets` is clean on the patched tree.

## Why the result looks the way it does

The vulnerability is structural: `Range::from_sparse()`'s dense-grid size comes from the *actual*
min/max cell position found while parsing, which an attacker fully controls by placing two genuine
non-empty cells at chosen positions - no malformed XML, oversized `<dimension>` header, or anything
else "suspicious-looking" is needed. The existing `if len < 100_000 { cells.reserve(len) }` guard
already present at both call sites (visible in the diff context) only ever protected the *sparse*
cell buffer from an over-eager `Vec::reserve` based on the `<dimension>` header's claimed size; it
never bounded the *dense* grid `from_sparse()` builds afterward from the real cell positions, which
is the actual unbounded allocation. `ods.rs` had already solved the identical problem (its own
Changelog cites "attempting to allocate memory for 17+ billion cells") with a local `MAX_CELLS`
cap before its own dense-grid build - this candidate ports the same pattern to XLSX/XLSB via a
shared, opt-in `from_sparse_bounded()` helper instead of duplicating the bounds-computation loop a
third and fourth time.

## What's not done / open questions

- **XLS (`src/xls.rs`) was left out of this specific candidate's real-cell-position fix** (it also
  called the now-still-unbounded public `Range::from_sparse()`) - see candidate 003
  (`___OPTIMIZATIONS___/003.xls_dimension_reserve_cap/`), which found and fixed a *worse*,
  more-directly-triggerable variant of this same bug class for XLS (an unvalidated `Dimensions`
  record driving a `Vec::reserve()` directly, no real cell data needed) and, while there, also
  applied `from_sparse_bounded()` to XLS's own real-cell-position path for consistency.
- **XLSB was independently confirmed exploitable**, not just inferred from identical code shape:
  `___BUGHUNT___/001.xlsx_xlsb_dense_range_oom/repro/poc.xlsb` (calamine's own `tests/date.xlsb`
  with one `BrtCellRk` record's column field binary-patched to `0xFFFFFFFE`) reproduces a ~412 GB
  allocation attempt and process abort against the *unpatched* tree, and returns cleanly with this
  candidate's patch applied - both confirmed by actually running it, not just read from the code.
- **`MAX_DENSE_RANGE_CELLS = 100_000_000` is a judgment call**, chosen to match `ods.rs`'s existing
  precedent rather than independently derived - a maintainer may want a different value (or to make
  it configurable) depending on real-world worksheet sizes they expect to support.
- The public `Range::from_sparse()` itself remains unbounded by design (see "What was tried" - this
  was deliberate, to avoid a breaking change to a documented public API used in the crate's own doc
  examples) - a caller building their own `Vec<Cell<T>>` from an untrusted source and calling
  `from_sparse()` directly is still exposed to the same class of issue; only calamine's own
  XLSX/XLSB file-parsing paths were hardened.
