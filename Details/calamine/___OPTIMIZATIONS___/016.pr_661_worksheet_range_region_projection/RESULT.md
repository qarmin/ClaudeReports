# pr_661_worksheet_range_region_projection

Reviewed/tested open GitHub PR #661 (`tafia/calamine`, not merged, author `alexander-beedie`,
labeled "needs work for merge"), which adds `worksheet_range_region`/`worksheet_range_region_ref`
so a caller can project a column/row region at read time instead of allocating the full dense
`Range`. Directly motivated by a linked polars issue (pola-rs/polars#27677) showing a real ~40GB
allocation that only needed a few MB once the required region is known upfront. This lives in the
exact code area (worksheet range collection, `Range::from_sparse`, XLSX/XLSB dense-grid
allocation) as this session's candidates 001/002/004/007/008 (the from_sparse dense-grid
restructuring/allocation-cap work) - see "Overlap with the from_sparse restructuring work" below.

`rustc 1.98.0`, `cargo 1.98.0` (matches this workspace's recorded environment in README.md).

## What was tried

The PR's own branch point (`d206c9b`) is several commits behind this session's `base_commit`
(`5c02e86`) - `src/xlsx/mod.rs`/`src/xlsb/mod.rs` had drifted (0.36.0/0.36.1 releases, several
fixes landed in between). A plain `git apply` of the PR's diff against `base_commit` fails
outright. Rebased by cherry-picking the PR's single commit (`22e43ae`, from
`git fetch https://github.com/tafia/calamine pull/661/head`) onto `base_commit`: 2 real conflicts,
both trivial "insert alongside" conflicts with no semantic resolution needed -
`src/utils.rs` (an unrelated `push_column` optimization landed on master after the PR branched;
kept both, `collect_cells_into_range` ahead of the newer `push_column`) and `tests/test.rs`
(purely two independent test blocks appended back to back). `src/xlsx/mod.rs`/`src/xlsb/mod.rs`
auto-merged with no conflicts despite the stat-level drift. Result is `patch.diff` in this folder,
verified with `git apply --check --binary` against a **fresh clone** at `base_commit` (not just
the worktree it was built in).

Built a standalone bench (`bench/`, depends on `{ path = ".." }` locally / `{ path = "../clone" }`
via the justfile) since `worksheet_range_region` doesn't exist pre-patch - there is no baseline
binary to build from unpatched source, so "baseline" here is the same patched binary's own `full`
mode (`worksheet_range_region(sheet, .., ..)`, which is the PR's own default/pass-through path,
identical code shape to the pre-existing `worksheet_range_ref`/`worksheet_range`). Fixture:
`testdata/big_300k_rows.xlsx` (copied in, same file as this workspace's `testdata_external/`; 20MB,
1 sheet "Data", 300,000 rows x 8 cols, 2.4M cells) - the same real-world fixture used by candidates
001/002/004/006/007/008.

Four modes, all against the same file/sheet:

- `full` - `.., ..` (no projection - the "as if this PR didn't exist" baseline)
- `col1` - project column 0 only, all 300,000 rows (no row bound, so no early-break; isolates the
  per-column dense-grid-size reduction alone)
- `row1k` - all 8 columns, rows `0..1000` only (isolates the early-break-on-row-cutoff mechanism
  alone - `collect_cells_into_range` stops pulling cells once `cell.pos.0 > last_row`)
- `tiny` - column 0 only, rows `0..1000` (both mechanisms together - closest in shape to the
  linked polars issue: a small slice of a huge sheet)

3 pinned (`taskset -c 0`) runs and 3 unpinned runs per mode (12 total per mode across both), all
via `flock` on the shared build lock. `nproc` = 24 logical cores on this box; load was below core
count throughout (single-threaded workload, no parallelism in this candidate's own code to
exercise multi-core with - see "multi-core" note below).

| Mode | peak RSS (pinned, 3 runs) | peak RSS (unpinned, 3 runs) | elapsed (unpinned) | cells |
|---|---|---|---|---|
| `full` (baseline-equivalent) | 171,516 / 171,700 / 171,776 KB | 171,264 / 171,384 / 171,548 KB | ~602-612 ms | 2,400,000 |
| `col1` (column projection only) | 23,876 / 23,900 / 23,932 KB | 23,560 / 23,600 / 23,792 KB | ~482-496 ms | 300,000 |
| `row1k` (row early-break only) | 3,284 / 3,516 / 3,584 KB | 3,348 / 3,424 / 3,540 KB | ~2 ms | 8,000 |
| `tiny` (both, polars-issue shape) | 2,956 / 3,040 / 3,136 KB | 2,956 / 3,000 / 3,160 KB | ~1 ms | 1,000 |

"Multi-core" here: this candidate's own code path (XML parsing + cell collection) has no internal
parallelism, so an unpinned run on this 24-core box is not meaningfully different from a pinned
one for either metric - both are reported above for completeness per SKILL.md, but neither shows a
change in kind, only the pinned run's slightly lower noise floor (expected, matches this
project's own prior candidates' notes on this box).

## Result

**Peak RSS**, on the real 2.4M-cell fixture, projecting down from the full sheet:

- Column-only projection (1 of 8 cols, unbounded rows): **171.5 MB -> ~23.7 MB, ~86% reduction**
  (~7.2x), with no row-bound early break in play at all (this reduction is purely per-column dense
  grid + sparse-buffer sizing).
- Row-bound projection (first 1000 of 300,000 rows, all cols): **171.5 MB -> ~3.4 MB, ~98%
  reduction** (~50x) - the early-break-on-row-cutoff path (`collect_cells_into_range` stops
  streaming once past the last kept row) is what makes this a genuine "read the first N rows"
  operation instead of "read everything then discard", matching the PR description's own framing.
- Combined (1 col, first 1000 rows - closest to the linked polars issue's own shape): **171.5 MB ->
  ~3.0 MB, ~98.3% reduction** (~57x).
- Unprojected (`full`, `.., ..`): **unchanged**, 171.5-171.8 MB either way - confirms the PR adds
  no overhead to the existing default (no-projection) call shape, and this bench's own `full` mode
  is a fair stand-in for a pre-patch baseline (see correctness section below for the direct
  cross-check).

Wall-clock moved in the same direction as expected (proportional to work actually done - a
1000-row read takes ~2ms instead of ~600ms), which is a straightforward consequence of reading
less of the file, not a separate finding.

These numbers are not adversarial-edge-case numbers (unlike this session's 001/003/005/009): they
are the PR's own advertised, intended use case working as described, measured on a real file.

## Does it change behavior/output?

- **Bit-exact?** Yes, for the unprojected (`full`, `.., ..`) case against the pre-existing API:
  cross-checked the patched build's `full` mode `sig` (a cheap positional/value signature, see
  `harness/rss_probe`) against the **unpatched** `base_commit` build's own `rss_probe range` mode
  on the identical file - both produced `sig=860639953605989024`, `cells=2400000`, and peak RSS
  within this workspace's established run-to-run noise (171.26-171.78 MB patched vs. 171.58 MB
  unpatched single run, 172.3 MB documented in README.md's original smoke test). This confirms the
  PR's default (no-projection) code path reproduces the exact pre-existing output, not just "looks
  similar".
- For an actual projection (`col1`/`row1k`/`tiny`): this is new functionality, not a behavior
  change to any existing call - `worksheet_range`/`worksheet_range_ref`/`worksheet_range_region(_ref)(sheet, .., ..)`
  are all untouched in output. The projected APIs' own correctness (dropped-column cells read
  `Empty`, header-row retention, disjoint/overlapping index merging, XLS/ODS returning a clean
  `Err` instead of `unimplemented!()`) is covered by the PR's own 12 new test functions (`rstest`
  parameterized, several cases each) - all passed, see below.
- **Is this a breaking change?** No new breaking change to any existing public item. It is a pure
  API addition (`IndexSet`, two new trait methods with default-ish dispatch through `Sheets`). One
  pre-existing behavior tightens: `Sheets::worksheet_range_ref` on `Xls`/`Ods` changed from
  `unimplemented!()` (a panic) to returning `Err(Error::Msg(...))` - strictly a robustness
  improvement (no caller could have been relying on the panic), and it's the PR's own change, not
  something this candidate added.
- Project test suite (`cargo test --release`), run through the shared build lock, before
  (`base_commit`, no changes) and after (`base_commit` + `patch.diff`):
  - Before: **269 passed** (43 unit + 161 integration + 65 doctest), 0 failed, 1 ignored.
  - After: **315 passed** (70 unit + 179 integration + 66 doctest), 0 failed, 1 ignored.
  - The +46 passing tests are the PR's own new tests (`test_region_columns`/`test_region_rows`/
    `test_region_full_equals_plain`/`test_region_both_axes`/`test_region_overlaps_merge`/
    `test_region_header_columns`/`test_region_header_row_always_retained`/
    `test_region_xlsb_with_header_row`/`test_region_through_sheets_auto`/
    `test_region_unsupported_for_xls_ods`, several `rstest`-parameterized, plus `IndexSet`/
    `collect_cells_into_range` unit tests) - no pre-existing test's outcome changed.

## Why the result looks the way it does

The mechanism is exactly what the PR description claims: `collect_cells_into_range` (factored out,
shared by XLSX and XLSB) filters cells against the requested column/row `IndexSet` as they stream
in, so cells outside the projection never enter the intermediate sparse `Vec<Cell>` at all - not
"collect everything, then discard". When rows are bounded above, it also stops calling `next_cell`
entirely once past the last kept row (`if cell.pos.0 > last_row { break; }`), so an unbounded-row
projection (`col1` here) still scans the whole sheet (486-500ms, same order as `full`) while a
row-bounded one (`row1k`/`tiny`) finishes in ~1-2ms - the row bound is what turns this into genuine
early termination, not just a smaller final buffer. `Range::from_sparse` (the site of candidates
001/002/004/007's earlier work) is still called at the end, but on the much smaller *projected*
sparse buffer, so the final dense-grid allocation is sized to the request, not the sheet.

## Overlap with the from_sparse restructuring work (001/002/004/007/008)

Real and worth a maintainer's attention before landing either independently, but not a functional
conflict as measured here:

- This PR's `collect_cells_into_range` ends with a plain `Ok(Range::from_sparse(cells))` call - it
  does **not** use candidate 007's `from_sparse_dense_hint()` (the direct-dense-write path that
  skips the sparse buffer entirely when the `<dimension>` header is accurate). Landing both as-is
  would leave the *projected* read path on the slower/more-allocating `from_sparse` while the
  *unprojected* path (`worksheet_range_ref` still calls `worksheet_range_region_ref(name, .., ..)`
  per this PR's own `src/lib.rs` change) gets 007's optimization - an inconsistency a maintainer
  would likely want to resolve by wiring `from_sparse_dense_hint()` into
  `collect_cells_into_range` too, projected-region-sized rather than sheet-sized.
  Not measured separately in this candidate (out of scope - see IDEAS below).
- No overlap with candidates 001/003/005/009 (the allocation-cap crash fixes) - this PR's own
  `projected_len` calculation (`dimensions.len().saturating_mul(...)` then integer-divided) uses
  `saturating_mul`, so it doesn't reintroduce the unbounded-reserve shape those candidates fixed;
  it was not tested against an adversarial `<dimension>` header in this candidate (see IDEAS).
- No overlap with candidate 006/010's `Arc<str>` interning (`Data::String`) - unaffected by this
  PR, orthogonal mechanism (string sharing vs. cell-count reduction).
- No overlap with candidate 002's finding (moving `DataRef`->`Data` conversion earlier regressed
  RSS because it made owned-`String` allocation overlap the sparse buffer) - this PR only changes
  which cells end up *in* the sparse buffer, not when the DataRef->Data conversion happens.

## What's not done / open questions

- Not tested: an adversarial/malformed `<dimension>` header combined with a projection request
  (does `projected_len`'s arithmetic stay safe if `dimensions` is wrong/huge but the real column
  count is small - plausible given the `saturating_mul` but not independently PoC'd the way
  candidates 001/003/005/009 were for the unprojected path).
  Added to `IDEAS.md` as a follow-up (candidate 011's own numbering picks it up if not already
  covered by a sibling candidate this same batch, per SKILL.md's dedup guidance).
- Not measured: the projected-region path's peak RSS if `collect_cells_into_range` were rewired to
  candidate 007's `from_sparse_dense_hint()` instead of plain `from_sparse` - flagged above as the
  most direct actionable overlap, not attempted here (would be its own candidate, testing a
  hypothetical combination of two not-yet-merged/landed changes).
- Not tested on XLS/ODS data content (only confirmed they return a clean `Err` instead of
  panicking, per the PR's own `test_region_unsupported_for_xls_ods`) - region projection is
  XLSX/XLSB-only by design (PR's own doc comment: "Xls and Ods formats do not support lazy
  iteration").
- This is an **open, unmerged, "needs work for merge"-labeled** PR, not a candidate this session
  authored - `patch.diff` is the PR's own content (rebased for this session's `base_commit`), not
  an independent fix. No incidental crash/correctness bug was found while reviewing/testing it.
