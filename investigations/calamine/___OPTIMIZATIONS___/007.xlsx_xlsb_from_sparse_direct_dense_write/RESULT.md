# XLSX/XLSB: stream cells directly into a pre-sized dense buffer, skip the sparse `Vec<Cell>`

`rustc 1.98.0`, `cargo 1.98.0` (this session's toolchain, unchanged from prior candidates).

`Range::from_sparse()` (`src/lib.rs`) has always built its dense `Vec<T>` grid from an
already-fully-collected sparse `Vec<Cell<T>>`, so both buffers are alive at once right before the
sparse one is dropped - candidates 001/002/004/006 in this workspace all point at this as the
dominant, still-unaddressed factor behind XLSX/XLSB's large peak-RSS gap vs. the streaming
`worksheet_cells_reader()` API on the same file. This candidate adds
`Range::from_sparse_dense_hint()`, used by `Xlsx`/`Xlsb`'s `worksheet_range_ref()`: it pre-sizes a
single dense buffer from the sheet's own declared `<dimension>` header and writes cells straight
into it as they stream from the cell reader, never materializing the sparse `Vec<Cell>` at all in
the common case where that header is accurate. If a cell ever turns out to lie outside the
declared box (a wrong/mismatched/missing header), it reconstructs a sparse `Vec<Cell>` from
whatever the abandoned dense buffer already holds, and finishes the *same* cell stream exactly the
way the original code did (feeding into the unmodified `Range::from_sparse()`) - this is the
"little coverage for wrong/mismatched `<dimension>` headers" edge case flagged when this candidate
was picked, and it needed its own from-scratch correctness testing since the project's own test
suite doesn't exercise it (see below).

## What was tried

| Variant | `big_300k_rows.xlsx` (2.4M real cells, `<dimension>` matches used range exactly) peak RSS | elapsed (`taskset -c 3`, 3 runs) | notes |
|---|---|---|---|
| baseline (`HEAD` 5c02e86), single-core | 172,140 / 172,332 / (172,024, 171,760 unpinned) KB | 628 / 625 ms | current code: sparse `Vec<Cell<DataRef>>` then dense `Vec<DataRef>` |
| patched, single-core | 134,724 / 134,728 / 134,536 KB | 655 / 605 / 616 ms | this candidate: direct dense write, no sparse buffer |
| patched, unpinned multi-core | 134,716 / 134,176 KB | 594 / 599 ms | same file, unpinned |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (`range` mode = `worksheet_range()`, peak
RSS = `/proc/self/status` `VmHWM`), release build, clean rebuild, `sig`/`cells` compared every run.

Baseline mean (5 runs, 3 pinned + 2 unpinned): **172,064 KB**. Patched mean (5 runs, 3 pinned + 2
unpinned): **134,576 KB**. That is a **21.8% peak-RSS reduction** on this file. `sig` and `cells`
(2,400,000) were bit-identical between baseline and patched on every run.

Elapsed time is within run-to-run noise both ways (baseline ~613-668ms, patched ~594-655ms across
these runs) - this candidate's metric of record is peak RSS, per this workspace's own convention;
no meaningful wall-clock change either direction was established.

On the small `formula_stress_test.xlsx` (4,542 cells, no RSS signal expected from a file this
size): baseline `peak_rss_kb=3936`, patched `peak_rss_kb=3888`, both `sig=16125784316220351567`,
`cells=4542` - consistent (small absolute difference, same order as process-baseline noise on a
tiny file), correctness match confirmed.

## Result

Real, reproducible **21.8% peak-RSS reduction** (172,064 KB -> 134,576 KB mean) on the 2.4M-cell
real-world fixture, with bit-identical output (`sig`/`cells` match on every run). This does not
reach the naive "sparse+dense -> dense-only" estimate from the backlog note
(~172MB -> ~77MB, i.e. sparse's ~40B/cell fully subtracted) - see "Why the result looks the way it
does" below for the two remaining overlapping buffers this candidate didn't touch. Wall-clock is
unaffected within measurement noise. Metric measured: **peak RSS** (`VmHWM`), plus elapsed time as
a secondary check per this workspace's usual practice.

## Does it change behavior/output?

- **Bit-exact?** Yes, on every file tried, including four synthetic wrong-`<dimension>` files
  built specifically for this candidate (see below) - not just the project's existing fixtures,
  which don't cover this edge case at all.
- **Is this a breaking change?** No public API change - `Range::from_sparse_dense_hint()` and the
  new `MAX_DENSE_RANGE_CELLS` constant are both `pub(crate)`, invisible outside the crate.
  `worksheet_range()`/`worksheet_range_ref()`'s documented behavior (including `HeaderRow`
  handling) is unchanged.
- **Correctness testing done specifically for the flagged edge case** (wrong/mismatched
  `<dimension>` headers - the project's own test suite has no coverage for this): built four
  variants of `tests/date.xlsx` (3 rows x 2 cols of real data, dimension originally `A1:B3`) with
  a Python script editing `xl/worksheets/sheet1.xml` in place before rezipping:
  - `<dimension ref="A1:A1"/>` (declared box far smaller than the actual data - forces a
    mid-stream bail-to-sparse on the very first out-of-bounds cell)
  - `<dimension ref="A1:D10"/>` (declared box larger than the actual data - exercises the
    "crop to the tight bounding box" path)
  - `<dimension>` tag removed entirely (defaults to `(0,0)-(0,0)`, the same "far smaller" shape)
  - `<dimension ref="B3:A1"/>` (reversed/degenerate - `end < start`)
  - a fifth variant, `<dimension ref="B2:C3"/>` (declared box doesn't start at the origin), used
    to test `HeaderRow::Row(0)` specifically (`0 < dims.start.0 == 1`, exercising the "skip direct
    path outright" guard)

  Ran each of the 4 (+1) files through a small standalone probe binary calling
  `worksheet_range("Sheet1")` and printing `start()`/`end()`/every row, once against unpatched
  `HEAD` and once against this candidate's patch, for both the default header row and
  `HeaderRow::Row(1)`/`HeaderRow::Row(5)`/`HeaderRow::Row(0)` - **12 file/header-row combinations
  total, all byte-for-byte identical** between baseline and patched output.
- Project test suite, run via `cargo test --release --all-features`, baseline (`HEAD` 5c02e86)
  and patched, identical both times: **45/45 unit tests, 165/166 integration tests (1
  pre-existing `#[ignore]`, unrelated to this change), 67/67 doc tests**. `cargo clippy --release
  --all-features --all-targets` clean on the patched tree.

## Why the result looks the way it does

The measured 21.8% reduction is smaller than the backlog note's naive "subtract the sparse
buffer's ~40B/cell entirely" estimate (which would predict something closer to a 55% reduction,
172MB -> ~77MB) because **two other buffer-overlap issues this candidate didn't touch are still
present** and were already identified by candidate 002 in this workspace:

1. `worksheet_range()` calls the now-optimized `worksheet_range_ref()` (builds one dense
   `Range<DataRef>`), then does `.into_iter().map(Into::into).collect()` to build a *second* dense
   `Range<Data>` - the old `Range<DataRef>` allocation is only dropped once its `IntoIter` is
   fully consumed, i.e. once the *second* buffer has also finished growing to full size, so both
   dense buffers (32B/cell each, per candidate 002's own measurement) are briefly alive together.
   This candidate's fix removes the *sparse* buffer from that picture entirely, but the two dense
   buffers still overlap exactly as they did before - eliminating that would need candidate 002's
   flagged (but explicitly not-yet-attempted, needs a public API discussion per that candidate's
   own notes) `Data::String` interning change, or a redesign of `worksheet_range()` to avoid the
   second dense buffer altogether.
2. On this specific fixture (~75% shared-string cells per this workspace's README), converting
   `DataRef::SharedString(&str)` (free, an existing borrow) into `Data::String(String)` during
   that same `.map(Into::into)` step allocates an owned `String` per shared-string cell - this
   candidate doesn't touch that either (candidate 006 already explored fixing it via
   `Arc<str>` interning; it measured no peak change on this exact file because the peak was
   already set earlier, by the sparse+dense overlap this candidate now removes - a follow-up
   re-measurement of candidate 006's `Arc<str>` change *on top of* this candidate's fix is a very
   plausible next lever now that the sparse+dense floor has moved down, see IDEAS.md).

The wrong-`<dimension>`-header fallback path itself never fires on any of this session's *real*
fixtures (`big_300k_rows.xlsx`, `formula_stress_test.xlsx`, and every file in the project's own
`tests/` directory used by `cargo test`) - their `<dimension>` headers are all accurate, so the
peak-RSS number above reflects the fast (no-sparse-buffer-ever-allocated) path exclusively, not an
average blended with the fallback.

## What's not done / open questions

- The two remaining dense-buffer-overlap issues in `worksheet_range()` (see above) were
  identified but not fixed here - candidate 002 already tried one angle at this (moving the
  `DataRef -> Data` conversion earlier) and found it regresses; a different angle (avoiding two
  full dense buffers altogether, e.g. converting in place) is now flagged in IDEAS.md as a
  follow-up worth re-attempting given this candidate's fix changes the baseline it would be
  compared against.
- `worksheet_formula()` (both XLSX and XLSB) still uses the old sparse-then-`from_sparse` pattern
  unconditionally - it wasn't in scope for this candidate (it doesn't use `HeaderRow` filtering
  the same way, and formula strings are a much smaller/rarer memory concern than full-sheet
  value ranges), but the same `from_sparse_dense_hint()` helper could in principle be reused there
  too.
- XLS (`src/xls.rs`) was out of scope - it builds cells in one pass already, with different
  binary-format constraints (see candidate 003's notes on XLS's own dimension-record handling),
  and wasn't part of this candidate's target.
- Did not re-run candidate 006's `Arc<str>` interning patch on top of this one to see whether the
  two combine into a bigger combined win now that the sparse-buffer floor moved - flagged as a
  follow-up idea, not attempted (would be its own candidate per this workspace's "one fix per
  candidate" rule).
- The synthetic wrong-`<dimension>` correctness check above used a small (3-row) real fixture,
  not a large one - the fallback path's own *performance* (does bailing out mid-stream on a large
  file cost meaingfully more than the original code, since it re-scans the abandoned dense buffer
  to reconstruct the sparse list) was not measured, only its correctness. Given this path is
  believed rare on real-world files (accurate `<dimension>` headers are the norm), this was judged
  out of scope for this candidate's time budget.
