# Avoid the `Range<DataRef>` -> `Range<Data>` whole-grid re-collect in `worksheet_range()` (XLSX)

**This candidate is a documented regression, kept unapplied.** Chasing the ~48x peak-RSS gap
between `worksheet_range()` (dense) and `worksheet_cells_reader()` (streaming) found in this
workspace's `README.md` smoke test, the original `worksheet_range()` looked like it was paying for
two full dense grids at once: it calls `worksheet_range_ref()` (builds a complete `Range<DataRef>`
dense grid), then does `rge.inner.into_iter().map(|v| v.into()).collect()` to build a *second*
complete `Range<Data>` dense grid. The hypothesis: converting `DataRef -> Data` per cell *during*
parsing (skipping the intermediate `Range<DataRef>` entirely) should avoid ever holding two dense
grids live simultaneously.

## What was tried

Rewrote `Xlsx::worksheet_range()` to duplicate `worksheet_range_ref()`'s cell-collection loop
(both `HeaderRow` branches) but push `Cell<Data>` directly - calling `cell.val.into()` (`DataRef ->
Data`) at push time instead of once, later, over the whole finished grid. `Range::from_sparse()`
is then called once, directly, with `Data` cells - no `Range<DataRef>` is ever built.

| Variant | `sparse_large_range.xlsx` (5M declared cells, 22 real, all `Int`) peak RSS | `big_300k_rows.xlsx` (2.4M real cells, ~75% `SharedString`, ~25% `Float`) peak RSS | notes |
|---|---|---|---|
| baseline (`HEAD` 5c02e86) | 159,128-159,528 KB (3 runs) | 171,564-172,000 KB (3 runs) | current code |
| patched | 159,208-159,536 KB (3 runs) | 227,816-228,080 KB (3 runs) | this candidate |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (`range` mode, peak RSS = `/proc/self/status`
`VmHWM`), release build, clean rebuild confirmed before each set of runs (ruled out stale-binary
pitfall explicitly after the first surprising result). `sig`/`cells` identical between baseline and
patched on both files in every run (bit-exact, see below).

`sparse_large_range.xlsx` is a hand-built file (`testdata/sparse_large_range.xlsx`, 2182 bytes):
`<dimension ref="A1:E1000000"/>` (5,000,000 declared cells) but only 22 real non-empty cells,
scattered, all small integers (`<v>N</v>`, no `t="s"` attribute - not shared strings).

## Result

- **On the sparse/mostly-empty/no-strings file:** no measurable difference (within noise, <0.3%
  spread either way) - matches the original hypothesis that avoiding the second dense grid should
  help when the sparse cell count is tiny relative to the declared grid, but the effect turned out
  to be masked by something else (see below) rather than simply "no effect to find".
- **On the dense, mostly-`SharedString` real file:** peak RSS goes from ~171.8 MB to ~227.9 MB -
  **a ~56 MB / ~33% regression**, consistent and reproducible across a clean rebuild and 3 runs
  each way, not noise.

This session's metric is peak RSS; wall-clock was not meaningfully different either way for either
file (not the point of this candidate).

## Does it change behavior/output?

- **Bit-exact?** yes - `sig`/`cells` identical to baseline in every run, both files.
- **Is this a breaking change?** No public API change (internal implementation only).
- Project test suite: 165/166 integration tests passing both before and after (1 pre-existing
  `#[ignore]`, unrelated), 45/45 unit tests both before and after, 67/67 doc tests both before and
  after (`cargo test --all-features`, run once on unpatched `HEAD` via `git stash`, once patched).
  `cargo clippy --all-features --all-targets` clean on the patched tree.

## Why the result looks the way it does

The original code's two-stage shape turns out to already be a (likely accidental, but real)
optimization for realistic spreadsheets, not the double-allocation waste it looked like:

- **`DataRef::SharedString(&'a str)` is a borrowed reference into the already-resident shared
  strings table - it costs nothing extra to store in a dense `Range<DataRef>` grid.** Building the
  full `Range<DataRef>` grid (`worksheet_range_ref()`) therefore only ever pays for the *structural*
  `Vec` backing buffers (the sparse `Vec<Cell<DataRef>>` collected during parsing, and the dense
  `Vec<DataRef>` `from_sparse()` builds from it) - no per-cell heap allocation for string content
  happens at this stage. That's where the baseline's ~172 MB peak comes from (matches candidate
  001's earlier analysis: sparse-buffer + dense-buffer overlap inside `from_sparse()`).
- **Only the later `.into_iter().map(\|v\| v.into()).collect()` step actually materializes owned
  `String`s** (`Data::String(String)`, one heap allocation each) - by that point, the *sparse*
  `Vec<Cell<DataRef>>` from stage 1 has already been dropped (it goes out of scope inside
  `from_sparse()`, before `worksheet_range_ref()` even returns). So the owned-string
  materialization only ever has to overlap with ONE dense grid's worth of `Vec` overhead, not two,
  and never overlaps with the (larger) sparse buffer at all.
- **This candidate's change deliberately moved that owned-string materialization earlier** - into
  the initial per-cell parsing loop, so it now happens *while the sparse `Vec<Cell<Data>>` is still
  being built*, i.e. simultaneously with both the sparse buffer AND (once `from_sparse()` runs) the
  dense buffer. For `big_300k_rows.xlsx`, ~1.8M of the 2.4M cells are `t="s"` shared-string
  references (6 of 8 columns) into a shared strings table containing only ~430 bytes of actual
  unique text (short, repeated category-like values, e.g. `"8"`, `"9"`, `"10"` visible in the raw
  worksheet XML) - so each of those 1.8M cells pays a small heap allocation whose cost is dominated
  by allocator bucket/metadata overhead rather than content length. `56 MB / 1.8M cells ≈ 31
  bytes/cell`, consistent with a typical small-allocation overhead for 1-2 byte string content -
  this matches the regression size closely enough to confirm the mechanism, not just correlate
  with it.
- The sparse-file test (`sparse_large_range.xlsx`) shows no regression because none of its 22 real
  cells are strings (`Int` values, `Copy`, no heap allocation either way) - the mechanism this
  candidate disturbs simply isn't exercised there, which is itself informative: it explains why the
  *original* hypothesis (avoid a second dense grid, expect a memory win on a sparse/mostly-empty
  sheet) produced neither a win nor a loss for that shape of file, rather than confirming the
  hypothesis either way.

## What's not done / open questions

- **This candidate is not applied** (per the workspace's standing rule - every candidate's patch is
  captured and reverted, never left live) - it would actively make things worse for any real
  spreadsheet with a meaningful fraction of shared-string cells, which is the common case, not the
  exception.
- **The real, unaddressed cost this surfaced**: ~1.8M individually-heap-allocated owned `String`s
  for what is, in the underlying shared-strings table, only a handful of *distinct* string values
  repeated many times. Interning/sharing repeated shared-string values (e.g. via `Rc<str>` or
  similar, if `Data::String` could tolerate that without a breaking API change) instead of giving
  every referencing cell its own independent heap-allocated copy looks like a substantially bigger,
  more realistic peak-RSS win than anything tried in this candidate - not attempted here (would
  likely require a public API discussion, since `Data::String` is `String`, not `Cow`/`Rc<str>`,
  in the current public API). Logged to `IDEAS.md` for a future session.
