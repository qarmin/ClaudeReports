# XLS: stream cells directly into a pre-sized dense buffer, skip the sparse `Vec<Cell>`

`rustc 1.98.0`, `cargo 1.98.0` (this session's toolchain).

A perf profile taken this session on the real 18MB/1.47M-cell fixture
`tests/adhocallbabynames1996to2016.xls` showed `_int_malloc`/`__libc_malloc2` dominating (~30%+
combined samples), tracing to `Range::from_sparse()`'s dense-init loop (`src/lib.rs:942-970`).
`src/xls.rs`'s per-sheet BIFF record loop (~line 605-707) builds a full sparse `Vec<Cell<Data>>`
during its single-pass parse and then calls plain `Range::from_sparse(cells)` -
the same sparse-then-dense double-allocation shape candidate 007 already fixed for XLSX/XLSB via
`from_sparse_dense_hint()`. XLS's BIFF8 `Dimensions` record (type `0x0200`) gives the same kind of
file-declared bounding-box hint XLSX's `<dimension>` header gives, so the same restructuring
applies: this candidate adds `DenseRangeBuilder<T>` (`src/lib.rs`), a **push-based** counterpart to
007's pull-based `from_sparse_dense_hint()` - XLS's single interleaved BIFF loop discovers cells as
a side effect of parsing unrelated record types (formulas, merge-cell records, string tables, ...)
rather than through a dedicated cell-only stream, so it needs a builder it can `push()` into
inline rather than a closure it pulls from.

## What was tried

Measured with a standalone `rss_probe` binary (path-deps on this worktree's `calamine`, `range`
mode = `worksheet_range()` for every sheet, peak RSS = `/proc/self/status` `VmHWM`), release
build, in a dedicated git worktree at `base_commit`. `sig`/`cells` (a cheap correctness signature)
compared every run.

| Variant | `adhocallbabynames1996to2016.xls` (18MB, 1,471,681 real cells) peak RSS | end RSS | elapsed | notes |
|---|---|---|---|---|
| baseline (`HEAD` 5c02e86), `taskset -c 3` | 151,260 / 151,264 / 151,492 KB | 78,060 / 78,188 / 78,268 KB | 182 / 181 / 181 ms | current code: sparse `Vec<Cell<Data>>` then dense `Vec<Data>` |
| baseline, unpinned | 151,080 / 151,192 / 150,992 KB | 78,220 / 78,184 / 78,096 KB | 179 / 150 / 155 ms | same file, unpinned |
| patched, `taskset -c 3` | 142,468 / 142,552 / 142,468 KB | 52,092 / 52,200 / 52,092 KB | 148 / 142 / 143 ms | this candidate: direct dense write via `DenseRangeBuilder`, no sparse buffer |
| patched, unpinned | 142,288 / 142,244 / 141,956 KB | 52,240 / 52,252 / 52,052 KB | 144 / 148 / 146 ms | same file, unpinned |

Baseline mean (6 runs: 3 pinned + 3 unpinned): peak **151,213 KB**, end **78,169 KB**, elapsed
**171.3 ms**. Patched mean (6 runs): peak **142,329 KB**, end **52,155 KB**, elapsed **145.2 ms**.
`sig`/`cells` (`1471681`, `9293658855915755249`) were bit-identical between baseline and patched
on every run.

A separate `just test` run (fresh `git clone` at `base_commit`, same harness, same fixture)
reproduced the same shape independently: baseline `peak_rss_kb=151420`, patched
`peak_rss_kb=142424`, `sig`/`cells` identical both times.

On two small fixtures (`tests/date.xls`, 6 cells; `tests/merge_cells.xls`, 6 cells - no RSS signal
expected from files this size), `sig`/`cells` were bit-identical baseline vs. patched
(`sig=14008655284499714809`/`cells=6` and `sig=11400714819323198512`/`cells=6` respectively);
peak RSS on these tiny files (~3.5-3.7 MB) moved by less than normal process-startup noise in
either direction.

## Result

**Peak RSS: -5.9%** (151,213 KB -> 142,329 KB mean, 6 runs each). **End RSS: -33.3%** (78,169 KB ->
52,155 KB mean) - the bigger and arguably more relevant number for "load once, hold onto it"
usage, since it's not diluted by the transient sparse-buffer peak. **Elapsed: -15.2%** (171.3 ms ->
145.2 ms mean) - directionally consistent across all 12 runs (every patched run was faster than
every baseline run in this session), though the individual-run spread (150-182ms baseline,
142-148ms patched) is wide enough relative to the delta that this should be read as "consistently
faster in every run measured this session," not as a tightly bounded percentage. `sig`/`cells`
bit-identical in all comparisons. Metrics measured: peak RSS, end RSS, and elapsed time.

The measured peak-RSS reduction (5.9%) is much smaller than candidate 007's XLSX/XLSB result
(21.8%) on a similarly-shaped file - see "Why the result looks the way it does" below for why this
is expected, not a sign the restructuring did less work than it should have.

## Does it change behavior/output?

- **Bit-exact?** Yes, on every file tried this session (the real 1.47M-cell fixture, two small
  fixtures, and 6 targeted unit tests below covering the fallback path) - `sig`/`cells` identical
  in every comparison.
- **Is this a breaking change?** No public API change - `DenseRangeBuilder` and
  `MAX_DENSE_RANGE_CELLS` are both `pub(crate)`, invisible outside the crate. `parse_mul_rk`'s
  signature changed (`&mut Vec<Cell<Data>> -> Result<Vec<Cell<Data>>, XlsError>` return instead of
  an out-param) but it's a private (non-`pub`) free function, not part of the crate's public API.
- **Correctness testing done specifically for the fallback path** (wrong/mismatched/absent/
  oversized/reversed `Dimensions` hints - the project's own test suite has no dedicated coverage
  for this): added 6 temporary `#[cfg(test)]` unit tests directly exercising `DenseRangeBuilder`
  (exact hint, looser hint requiring a crop, a real cell landing outside a too-small hint
  triggering the sparse bail-out, no hint at all, a hint larger than `max_cells`, and a
  reversed/degenerate hint) - each asserts the builder's output is identical (`start()`, `end()`,
  every row) to `Range::from_sparse()` given the exact same cells. All 6 passed. These tests were
  **not** kept in the final `patch.diff` (removed before capturing it, per this workspace's own
  convention of not adding permanent test files to the project's source tree) - they were run
  in-session for verification only; the scenarios they covered are described above and in "What
  was tried" so they can be reconstructed if wanted.
- Project test suite, run via `cargo test --release --all-features`, baseline (`HEAD` 5c02e86)
  and patched, identical both times: **45/45 unit tests, 165/166 integration tests (1
  pre-existing `#[ignore]`, unrelated to this change), 67/67 doc tests**. `cargo clippy --release
  --all-features --all-targets` clean on the patched tree.

## Why the result looks the way it does

Same mechanism as candidate 007 (streams cells directly into a pre-sized dense buffer, skipping
the sparse `Vec<Cell>` entirely when the file-declared hint is accurate), but the measured win is
proportionally smaller here (-5.9% peak RSS vs. 007's -21.8% on XLSX/XLSB) because XLS's baseline
peak was already lower relative to its data volume: `sizeof(Cell<Data>)` sparse-buffer overhead is
the same fixed cost being removed in both cases, but XLS's per-sheet loop was already building
`Data` values directly (no intermediate `DataRef<'a>` borrow stage XLSX/XLSB have), and this
fixture's cells skew toward numeric/compact variants rather than the ~75% shared-string mix
candidate 007's XLSX fixture had - so there was proportionally less sparse-buffer *content* (as
opposed to per-cell overhead) to remove. The **end-RSS** number (-33.3%) is proportionally larger
than 007's own end-RSS story because XLS's sparse `Vec<Cell<Data>>` was the *only* extra buffer
alive at peak in the old code (unlike XLSX/XLSB's separate `Range<DataRef>` -> `Range<Data>`
double-dense-buffer issue flagged in 007/002, which XLS's single-pass design never had to begin
with) - removing it here removes essentially the whole gap between peak and steady-state.

## What's not done / open questions

- `worksheet_formula()`'s `Vec<Cell<String>>` (the `formulas` variable in the same loop) was left
  as plain `Range::from_sparse(formulas)`, unconverted - formula strings are a much smaller/rarer
  memory concern than the full-sheet value range on real files (candidate 007 made the same scope
  call for XLSX/XLSB's `worksheet_formula()`), and formulas don't have their own `Dimensions`-style
  hint to key a `DenseRangeBuilder` off in the first place.
- `MAX_DENSE_RANGE_CELLS = 100_000_000` is the same judgment call as candidates 001/003/007/009 -
  not independently re-derived for this candidate. This candidate's `DenseRangeBuilder` caps its
  *own* new dense-alloc attempt at this value (falling back to the old sparse-then-`from_sparse`
  path, which is itself still unbounded, above it) - it does not fix the separately-tracked
  `Dimensions`-record-`reserve()` OOM (candidate 003) or the real-cell-position OOM (candidate 009)
  bugs; those remain exactly as they were before this patch (this patch's own bounds check
  prevents its *new* code path from being a third variant of the same bug, but doesn't retrofit a
  fix into the pre-existing sparse fallback path it defers to).
- Did not measure a synthetic large XLS file with a much higher shared-string fraction (to see
  whether the gap to candidate 007's 21.8% narrows on a fixture shaped more like XLSX's) - only
  the one real fixture available in this session's testdata was used.
- Did not re-run candidate 006's `Arc<str>` interning patch on top of this one - flagged as a
  follow-up idea in candidate 007's own notes for XLSX/XLSB, the same combination question applies
  to XLS now that this candidate moves its own sparse-buffer floor down.
