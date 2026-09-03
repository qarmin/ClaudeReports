# XLSX/XLSB `worksheet_formula()`: reuse `Range::from_sparse_dense_hint()` for the formula-string read path

`rustc 1.98.0`, `cargo 1.98.0`.

`worksheet_formula()` in both `src/xlsx/mod.rs` and `src/xlsb/mod.rs` built a plain, only
size-hint-reserved `Vec<Cell<String>>` and fed it into the unmodified `Range::from_sparse()` -
the same sparse+dense buffer-overlap pattern candidate 007 in this workspace already eliminated
for the value-range path (`worksheet_range_ref()`) via a new `Range::from_sparse_dense_hint()`
helper, but 007's own patch was never applied to real source for real (per this workspace's rule
of always reverting), and 007's own RESULT.md explicitly flagged `worksheet_formula()` as an
out-of-scope second call site the same helper could in principle serve.

Since 007's patch isn't present in the real tree to build on top of, this candidate's own
`patch.diff` includes the `MAX_DENSE_RANGE_CELLS` constant and the
`Range::from_sparse_dense_hint()` / `Range::collect_sparse()` helper functions verbatim from
007's patch (needed as infrastructure - `worksheet_formula()` cannot call a helper that doesn't
exist), then changes only `worksheet_formula()` in `src/xlsx/mod.rs` and `src/xlsb/mod.rs` to use
it, passing `HeaderRow::FirstNonEmptyRow` (which reproduces the function's previous behavior
exactly: skip empty-formula cells, no other header-row filtering - `worksheet_formula()` never
consulted `self.options.header_row` even before this candidate). `worksheet_range_ref()` in both
files is left untouched, exactly as it is in real `HEAD` today.

No public/free-standing large formula-heavy XLSX fixture was found in the time budgeted for this
candidate (the workspace's own `testdata_external/formula_stress_test.xlsx`, 74KB/1,924 cells, is
documented in `../README.md` as too small to show a peak-RSS signal). A synthetic fixture was
generated instead (`scripts/gen_formula_xlsx.py`, stdlib `zipfile` only, no `openpyxl`/LibreOffice
available on this box): 300,000 rows x 8 cols, a formula in every cell (`=A{r-1}+1`-style, first
row `=1`), matching candidate 007's own `big_300k_rows.xlsx` shape (2.4M cells, ~18MB) but with
formulas instead of plain values.

## What was tried

Harness: `___OPTIMIZATIONS___/harness/rss_probe`, extended this session with a `formula` mode
(`worksheet_formula()` for every sheet instead of `worksheet_range()`) - release build, clean
rebuild per side, `sig`/`cells` compared every run.

| Variant | `big_formula.xlsx` (synthetic, 2.4M formula cells, `<dimension>` matches exactly) peak RSS | elapsed | notes |
|---|---|---|---|
| baseline (`HEAD` 5c02e86), `taskset -c 3`, run 1 | 209,788 KB | 792 ms | current code: sparse `Vec<Cell<String>>` then `from_sparse()` |
| baseline, `taskset -c 3`, run 2 | 209,556 KB | 791 ms | |
| baseline, unpinned, run 1 | 209,020 KB | 765 ms | |
| baseline, unpinned, run 2 | 208,892 KB | 772 ms | |
| patched, `taskset -c 3`, run 1 | 134,896 KB | 741 ms | this candidate: direct dense write via `from_sparse_dense_hint()` |
| patched, `taskset -c 3`, run 2 | 135,012 KB | 749 ms | |
| patched, unpinned, run 1 | 135,216 KB | 741 ms | |
| patched, unpinned, run 2 | 134,988 KB | 741 ms | |

Baseline mean (4 runs, 2 pinned + 2 unpinned): **209,314 KB**. Patched mean (4 runs): **135,028
KB**. That is a **35.5% peak-RSS reduction** on this synthetic 2.4M-formula-cell fixture. `sig`
(41765480) and `cells` (2,400,000) were bit-identical between baseline and patched on every run.

On the small real `formula_stress_test.xlsx` (74KB, 1,924 formula cells, no RSS signal expected
from a file this size): baseline `peak_rss_kb` 3712/4012 across 2 runs, patched 3992/3908 -
differences are within this box's own small-file process-baseline noise (both around ~3.7-4.0 MB),
`sig`/`cells` identical (13418538750662676205 / 1924) between baseline and patched. Consistent
with this candidate's own hypothesis (formula strings are typically a much smaller working set
than full-sheet value ranges) - the small fixture confirms correctness but was never expected to
show a peak-RSS signal, and doesn't.

Elapsed time is within run-to-run noise on both sides (baseline ~765-792ms, patched ~741-749ms) -
this candidate's metric of record is peak RSS, per this workspace's own convention; no confident
wall-clock claim is made either direction (patched trending a little faster is plausible - one
fewer full buffer's worth of allocation/copy work - but the spread here isn't wide enough apart
from the noise floor to call it beyond "consistent with, not conclusively demonstrating").

Reproduced via this candidate's own `just test` (clone at pinned `base_commit`, build baseline,
apply `patch.diff`, build patched, run both) in a scratch checkout - printed `peak_rss_kb=209324`
(baseline) vs `peak_rss_kb=134376` (patched) on that separate run, `sig`/`cells` identical - same
signal, independent of the manual runs above.

## Result

Real, reproducible **35.5% peak-RSS reduction** (209,314 KB -> 135,028 KB mean) on a 2.4M-cell
synthetic formula-heavy fixture, with bit-exact output (`sig`/`cells` match on every run,
including a separate `just test` run in a scratch clone). No RSS signal on the existing small
`formula_stress_test.xlsx` fixture, as expected for a 74KB file. Metric measured: **peak RSS**
(`VmHWM`), plus elapsed time as a secondary check per this workspace's usual practice - no
confident wall-clock claim either direction (see above).

## Does it change behavior/output?

- **Bit-exact?** Yes, on every file tried:
  - The synthetic 2.4M-cell fixture (`sig`/`cells` identical, every run, both `taskset`-pinned
    and unpinned, and independently via `just test`'s own scratch-clone run).
  - The existing `tests/formula.issue.xlsx` (3 scattered formula cells inside a 14x10 declared
    box) and 4 synthetic wrong-`<dimension>` variants built from it specifically for this
    candidate's own correctness check (the project's test suite has no coverage for this edge
    case - same gap 007 already flagged for the value-range path):
    - `A1:J14` (exact - matches the real used range)
    - `A1:A1` (declared box far smaller than the actual data - forces a mid-stream bail-to-sparse
      on the first out-of-bounds formula cell)
    - `A1:Z50` (declared box larger than the actual data - exercises the crop-to-tight-bounding-box
      path)
    - no `<dimension>` tag at all (defaults to `(0,0)-(0,0)`, same "far smaller" shape)
    - `J14:A1` (reversed/degenerate, `end < start`)

    Ran all 5 through a small standalone probe (`bench/formula_probe`) calling
    `worksheet_formula("Sheet1")` and printing `start()`/`end()`/every non-empty cell, once
    against unpatched `HEAD` and once against this candidate's patch - **all 5 byte-for-byte
    identical** (reproduced via this candidate's own `just correctness` recipe, printing
    `IDENTICAL` for every variant).
  - The existing `testdata_external/formula_stress_test.xlsx` (`sig`/`cells` identical).
- **Is this a breaking change?** No public API change - `worksheet_formula()`'s documented
  behavior is unchanged (still ignores `HeaderRow`, still returns only non-empty formula strings
  in a dense `Range<String>`). `Range::from_sparse_dense_hint()`/`Range::collect_sparse()` and
  `MAX_DENSE_RANGE_CELLS` are all `pub(crate)`/private, invisible outside the crate.
- Project test suite, run via `cargo test --release --all-features`, baseline (`HEAD` 5c02e86)
  and patched, identical both times: **45/45 unit tests, 165/166 integration tests (1
  pre-existing `#[ignore]`, unrelated to this change), 67/67 doc tests, 0 failures either side.**

## Why the result looks the way it does

The 35.5% reduction is larger than candidate 007's own 21.8% reduction for the value-range path on
an equivalently-shaped file, which is consistent with 007's own explanation of why *its* number
undershot the naive "just remove the sparse buffer" estimate: 007's `worksheet_range()` still pays
for a *second* full dense buffer (the `DataRef -> Data` conversion `worksheet_range()` performs on
top of `worksheet_range_ref()`) plus a `String` allocation per shared-string cell during that same
conversion - both still present after 007. `worksheet_formula()` has neither of those extra
overlapping buffers: it returns `Range<String>` directly, with no second dense-buffer conversion
step and no separate value-type allocation beyond the formula `String` itself, which is already
owned before this candidate's change (`next_formula()` already returns an owned `String`, not a
zero-copy `DataRef`). Removing the sparse `Vec<Cell<String>>` therefore removes a larger fraction
of this path's total peak allocation than it did for the value-range path.

The wrong-`<dimension>`-header fallback path never fires on any of this session's real fixtures
(`formula_stress_test.xlsx`, `formula.issue.xlsx`'s own real `<dimension>`, and every file in the
project's own `tests/` directory used by `cargo test`) - it only fires on the synthetic
mismatched-dimension variants built specifically to exercise it, and its own *performance* under a
large-file mismatch was not measured, only its correctness (same scope decision 007 made for the
equivalent value-range fallback path).

## What's not done / open questions

- The fallback path's own performance on a *large* mismatched-`<dimension>` file (does bailing out
  mid-stream and reconstructing a sparse list from an abandoned large dense buffer cost more than
  the original code) was not measured - judged out of scope given accurate `<dimension>` headers
  are the norm on real files, same judgment 007 made for the value-range path.
- No large, real-world (non-synthetic) formula-heavy XLSX/XLSB fixture was found or built by
  scraping a public source in the time budgeted for this candidate - the measurement above rests
  on a generated file (`scripts/gen_formula_xlsx.py`), not a real spreadsheet. The shape (2.4M
  cells, 8 cols, `<dimension>` accurate) mirrors 007's own real-world fixture, but a genuine
  real-world formula-heavy workbook could have a different formula-string-length distribution
  (this session's synthetic formulas are short, ~6-12 chars) that changes the *absolute* KB
  numbers, though not the qualitative sparse-vs-dense mechanism being measured.
- XLSB's own `worksheet_formula()` was patched identically to XLSX's (verified via the project's
  own `cargo test`, which does exercise XLSB) but was not separately peak-RSS-measured with a
  large XLSB-specific fixture - no large XLSB fixture (formula-heavy or otherwise) was available
  this session (`../README.md` already notes this format gap for the value-range work too).
- Did not attempt combining this candidate with 007's own (still-unapplied) value-range patch to
  see whether `worksheet_range()` and `worksheet_formula()` on the same workbook compound - each
  call is independent and this workspace's own rule keeps one fix per candidate.
