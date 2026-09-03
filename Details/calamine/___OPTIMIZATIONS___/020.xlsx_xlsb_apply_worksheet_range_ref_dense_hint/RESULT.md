# Apply candidate 007's `from_sparse_dense_hint()` fix for real, to a fresh worktree at current HEAD

`rustc 1.98.0`, `cargo 1.98.0` (same toolchain as prior candidates this session).

Candidate 007 in this workspace already authored, measured, and saved `patch.diff` for exactly
this fix - `Range::from_sparse_dense_hint()` in `src/lib.rs`, wired into `Xlsx`/`Xlsb`'s
`worksheet_range_ref()` (`src/xlsx/mod.rs:2656-2722`, `src/xlsb/mod.rs:565-624`) - replacing the
sparse `Vec<Cell<DataRef>>` collect + `Range::from_sparse(cells)` with a direct dense write
pre-sized from the sheet's own declared `<dimension>` header. Candidate 017 additionally verified
this same patch composes cleanly with two sibling candidates (011, 013) when combined. Neither 007
nor 017 left the patch applied to `src/` (per this workflow's "always revert" rule) - this
candidate exists purely to confirm 007's exact saved `patch.diff` still applies cleanly against
current `HEAD` (`5c02e86`, unchanged since 007/017 were written) and to re-run the full
apply -> build -> measure -> test cycle once more, end to end, in a fresh isolated worktree.

## What was tried

Applied 007's `patch.diff` unmodified (`git apply --check` succeeded with no fuzz/offset) to a
fresh `git worktree` at `5c02e86`, built, and re-measured with the same `rss_probe` harness and
fixture (`big_300k_rows.xlsx`, 2.4M cells) 007/017 used.

| Variant | `big_300k_rows.xlsx` `worksheet_range()` peak RSS | elapsed |
|---|---|---|
| baseline (`HEAD` 5c02e86), pinned (`taskset -c 3`) x3 | 172,328 / 172,288 / 172,392 KB | 662 / 650 / 641 ms |
| baseline, unpinned x2 | 172,144 / 171,568 KB | 641 / 641 ms |
| patched, pinned (`taskset -c 3`) x3 | 134,688 / 134,704 / 134,532 KB | 590 / 598 / 631 ms |
| patched, unpinned x2 | 133,940 / 134,764 KB | 604 / 595 ms |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (`range` mode = `worksheet_range()`, peak
RSS = `/proc/self/status` `VmHWM`), release build, clean rebuild each time, `sig`/`cells` compared
every run. Also ran the small `formula_stress_test.xlsx` fixture (4,542 cells) as a sanity check:
`sig=16125784316220351567`, `cells=4542`, matching 007's own recorded baseline value for that file
exactly.

Baseline mean (5 runs: 3 pinned + 2 unpinned): **172,144.0 KB**. Patched mean (5 runs, same mix):
**134,525.6 KB**. That is a **21.9% peak-RSS reduction**, reproducing 007's own originally
measured -21.8% and 017's re-measured -21.9% for the identical patch, within run-to-run noise.
`sig`/`cells` (`860639953605989024` / `2,400,000`) were bit-identical between baseline and patched
on every run of the large fixture; elapsed time was within run-to-run noise both ways (no
wall-clock metric of record claimed, consistent with 007/017).

## Result

Confirms 007's original -21.8% / 017's re-measured -21.9% peak-RSS reduction reproduces exactly
when the same `patch.diff` is applied fresh to a clean worktree at current `HEAD` (`5c02e86`,
unchanged since those candidates were authored) and run through a full independent
apply -> build -> measure -> test cycle: **-21.9%** peak RSS (172,144.0 KB -> 134,525.6 KB mean)
on the 2.4M-cell real-world fixture, bit-identical `sig`/`cells` output. Metric measured: **peak
RSS** (`VmHWM`); elapsed time is a secondary check only, unaffected within noise.

## Does it change behavior/output?

- **Bit-exact?** Yes - `sig`/`cells` identical to baseline on both fixtures tested
  (`big_300k_rows.xlsx`: 2,400,000 cells, `sig=860639953605989024`; `formula_stress_test.xlsx`:
  4,542 cells, `sig=16125784316220351567`), matching 007's own recorded values for the same files.
  007's own `RESULT.md` documents the from-scratch correctness testing done against 
  wrong/mismatched/missing/reversed `<dimension>` header edge cases (12 file/header-row
  combinations, all byte-for-byte identical) - not independently re-run here, since this
  candidate's patch is the exact same diff 007 already tested against those edge cases, not a new
  implementation.
- **Is this a breaking change?** No public API change - `Range::from_sparse_dense_hint()` and
  `MAX_DENSE_RANGE_CELLS` are both `pub(crate)`, invisible outside the crate.
  `worksheet_range()`/`worksheet_range_ref()`'s documented behavior (including `HeaderRow`
  handling) is unchanged.
- Project test suite (`cargo test --release --all-features`): baseline (`HEAD` 5c02e86) and
  patched, identical both times: **45/45 unit tests, 165/166 integration tests** (1 pre-existing
  `#[ignore]`, unrelated to this change), **67/67 doc tests**. `cargo clippy --release
  --all-features --all-targets` clean on the patched tree.

## Why the result looks the way it does

Same mechanism as 007: replacing the sparse `Vec<Cell<DataRef>>` intermediate with a single
pre-sized dense write removes the window where both the sparse and dense buffers are alive at
once for `worksheet_range_ref()`'s common case (an accurate `<dimension>` header). Re-testing from
a fresh worktree at unchanged `HEAD` was expected to reproduce 007/017's numbers essentially
exactly, since nothing about the surrounding code changed between when 007/017 were authored and
this candidate's run - the measurement confirms that expectation rather than revealing anything
new.

## What's not done / open questions

- This candidate is a re-application and re-verification of an already-designed fix, not new
  discovery - see candidate 007's own `RESULT.md` for the fix's design rationale and its from-
  scratch edge-case correctness testing (wrong/mismatched/missing/reversed `<dimension>` headers),
  and candidate 017's `RESULT.md` for how this same patch composes with sibling candidates 011
  (XLS) and 013 (XLSX/XLSB `worksheet_formula()`) when combined.
- Per this workflow's "the one rule that matters most", the patch was applied, built, measured,
  and then reverted from this worktree like every other candidate - it was not left applied to any
  tree. `patch.diff` in this folder (identical to 007's own) is the mechanism for a reader to
  actually apply it, as a separate, conscious action.
- 007's own "what's not done" note about a second, still-unaddressed dense-buffer overlap in
  `worksheet_range()`'s `DataRef -> Data` conversion step (outside `worksheet_range_ref()` itself)
  remains open; this candidate doesn't touch it either.
