# XLS: apply candidate 011's push-based `DenseRangeBuilder` to `src/xls.rs`'s BIFF record loop (fresh re-measurement)

`rustc 1.98.0`, `cargo 1.98.0` (this session's toolchain).

This is the same patch candidate 011 already produced and measured (and candidate 017
re-confirmed composed with other fixes), re-run from scratch in its own isolated worktree against
the current `HEAD` (which happens to be the exact same commit, `5c02e86`, both candidates used as
`base_commit`) - a fresh, independent measurement rather than a copy of the old numbers. The patch
replaces `src/xls.rs`'s per-sheet BIFF record loop (multi-arm match pushing into a sparse
`Vec<Cell<Data>>` across the `0x0203` Number, `0x0204`/`0x00D6` Label, `0x0205` BoolErr, `0x0207`
formula-string, `0x027E` Rk, `0x00FD` LabelSst, `0x00BD` MulRk, and `0x0006` Formula record arms)
plus the two trailing `Range::from_sparse()` calls with a push-based `DenseRangeBuilder<T>`
(`src/lib.rs`) that writes straight into a dense buffer pre-sized from the BIFF8 `Dimensions`
record (`0x0200`), falling back to the old sparse-then-`from_sparse` path if a cell ever lands
outside a wrong/absent hint. `git apply --check src/../022.../patch.diff` against this worktree's
`base_commit` succeeded with zero conflicts (identical to candidate 011's own patch.diff,
byte-for-byte - `diff -q` confirmed no drift).

## What was tried

Measured with the shared `rss_probe` harness (`___OPTIMIZATIONS___/harness/rss_probe`, path-dep
on this candidate's own worktree, `range` mode = `worksheet_range()` for every sheet, peak RSS =
`/proc/self/status` `VmHWM`), release build, in a dedicated git worktree
(`___OPTIMIZATIONS___/wt-022`) at `base_commit` (`5c02e86e99ec5c1fb1dcc15e64c2bbaf19bff60d`).
`sig`/`cells` (a cheap correctness signature) compared every run.

| Variant | `adhocallbabynames1996to2016.xls` (18MB, 1,471,681 real cells) peak RSS | end RSS | elapsed | notes |
|---|---|---|---|---|
| baseline, `taskset -c 3` (run 1) | 151,476 KB | 78,296 KB | 157 ms | current code: sparse `Vec<Cell<Data>>` then dense `Vec<Data>` |
| baseline, `taskset -c 3` (run 2) | 151,496 KB | 78,208 KB | 156 ms | repeat |
| baseline, unpinned (run 1) | 151,196 KB | 78,176 KB | 160 ms | |
| baseline, unpinned (run 2) | 151,292 KB | 78,216 KB | 157 ms | |
| patched, `taskset -c 3` (run 1) | 142,648 KB | 52,272 KB | 177 ms | this candidate: direct dense write via `DenseRangeBuilder` |
| patched, `taskset -c 3` (run 2) | 142,836 KB | 52,328 KB | 173 ms | repeat |
| patched, unpinned (run 1) | 142,076 KB | 52,108 KB | 158 ms | |
| patched, unpinned (run 2) | 141,820 KB | 52,252 KB | 151 ms | |

Baseline mean (4 runs: 2 pinned + 2 unpinned): peak **151,365 KB**, end **78,224 KB**, elapsed
**157.5 ms**. Patched mean (4 runs): peak **142,345 KB**, end **52,240 KB**, elapsed **164.75 ms**.
`sig`/`cells` (`1471681`, `9293658855915755249`) were bit-identical between baseline and patched
on every run.

A separate `just test` run (fresh `git clone` at `base_commit`, same harness, same fixture, single
unpinned run each) independently reproduced the same shape: baseline `peak_rss_kb=151456`,
patched `peak_rss_kb=142560`, `sig`/`cells` identical both times.

On two small fixtures (`tests/date.xls`, 6 cells; `tests/merge_cells.xls`, 6 cells - no RSS signal
expected from files this size), `sig`/`cells` were bit-identical baseline vs. patched
(`sig=14008655284499714809`/`cells=6` and `sig=11400714819323198512`/`cells=6` respectively);
peak RSS on these tiny files (~3.6-3.8 MB) moved by less than normal process-startup noise in
either direction (baseline 3,760/3,632 KB vs. patched 3,600/3,640 KB).

## Result

**Peak RSS: -5.96%** (151,365 KB -> 142,345 KB mean, 4 runs each). **End RSS: -33.2%** (78,224 KB
-> 52,240 KB mean). Both numbers match candidate 011's original measurement (-5.9% peak, -33.3%
end) within session-to-session noise, as expected for a byte-identical patch against the same
commit. **Elapsed time did not show an improvement in this session's runs**: mean went from 157.5
ms baseline to 164.75 ms patched (+4.6%), the opposite direction from candidate 011's own session
(which saw -15.2%). Both sessions' individual per-run spreads (this session: 151-177 ms; candidate
011's session: 142-182 ms) are wide enough relative to the ~5-10 ms delta that elapsed time on
this fixture should be read as **noise-dominated, not a reliably measurable direction either way**
- only 2 pinned + 2 unpinned runs were taken per variant this session (fewer than 011's 3+3), which
narrows confidence further. The peak/end RSS deltas are the reliable signal here (consistent
across both sessions and both measurement methods - live measurement and independent `just test`
clone); the wall-clock number is not. Metrics measured: peak RSS, end RSS, and elapsed time.

## Does it change behavior/output?

- **Bit-exact?** Yes, on every file tried this session (the real 1.47M-cell fixture, two small
  fixtures) - `sig`/`cells` identical in every comparison, and the project's own test suite
  (below) passed identically before and after.
- **Is this a breaking change?** No public API change - `DenseRangeBuilder` and
  `MAX_DENSE_RANGE_CELLS` are both `pub(crate)`. `parse_mul_rk`'s signature changed (out-param ->
  `Result<Vec<Cell<Data>>, XlsError>` return) but it is a private, non-`pub` free function.
- Project test suite, run via `cargo test --release --all-features`, baseline (`HEAD` `5c02e86`)
  and patched, **identical both times**: **45/45 unit tests, 165/166 integration tests** (1
  pre-existing `#[ignore]`, unrelated to this change), **67/67 doc tests**. `cargo clippy --release
  --all-features --all-targets` clean on the patched tree.
- This candidate did not add or re-run the temporary `DenseRangeBuilder` fallback-path unit tests
  candidate 011 wrote and then removed before capturing its `patch.diff` (exact-hint, cropped-hint,
  bail-to-sparse, no-hint, oversized-hint, reversed-hint) - that verification work already exists
  in candidate 011's own `RESULT.md` and wasn't repeated here since this candidate applies the
  byte-identical patch, not a new implementation.

## Why the result looks the way it does

Same mechanism, same patch, same base commit as candidate 011 - see that candidate's own
`RESULT.md` "Why the result looks the way it does" section for the full explanation (XLS's
per-sheet loop was already building `Data` directly with no intermediate `DataRef` stage, so the
peak-RSS win is proportionally smaller than the XLSX/XLSB equivalent candidate 007, while the
sparse `Vec<Cell<Data>>` being the only extra buffer alive at peak in the old code makes the
end-RSS win proportionally larger). The one new observation from this independent re-run: the
wall-clock direction is not stable across sessions on this fixture/machine at this run count
(011's session: faster; this session: slightly slower) - worth flagging so a reader doesn't treat
either session's elapsed-time number as more precise than the underlying noise floor supports.

## What's not done / open questions

- Same open items as candidate 011 (formulas' `Vec<Cell<String>>` left unconverted,
  `MAX_DENSE_RANGE_CELLS` bound not independently re-derived, no synthetic higher-shared-string
  XLS fixture tried, candidate 006's `Arc<str>` interning not tried combined with this) - see that
  candidate's own "What's not done" section, not repeated in full here since nothing new was
  learned on those specific points this session.
- Did not re-run with a higher run count (011 used 3+3; this session used 2+2) to see whether more
  samples resolve the elapsed-time direction one way or the other - flagged above as the one
  genuinely new open question this re-run surfaced.
