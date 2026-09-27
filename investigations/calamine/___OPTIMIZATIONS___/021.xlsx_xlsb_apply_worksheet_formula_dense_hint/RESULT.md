# XLSX/XLSB `worksheet_formula()`: real for-real application of `Range::from_sparse_dense_hint()` reuse (candidate 013's finding, independently re-measured)

`rustc 1.98.0` / `cargo 1.98.0` (this session's toolchain, same version candidate 013 recorded).

`worksheet_formula()` in both `src/xlsx/mod.rs:2599-2622` and `src/xlsb/mod.rs:520-539` builds a
plain, only size-hint-reserved `Vec<Cell<String>>` and feeds it into the unmodified
`Range::from_sparse()`. Candidate 013 (this workspace) already measured switching this to
`Range::from_sparse_dense_hint()` - a 35.5% peak-RSS reduction on a synthetic 2.4M-formula-cell
fixture - and candidate 017 re-confirmed the same number composed with candidates 007/011's own
patches on top. Both of those sessions, per this workspace's own rule, reverted their change from
the main tree afterward. This candidate independently re-runs the full baseline -> apply -> build
-> measure -> test -> revert cycle from scratch (its own git worktree, its own builds, its own
`rss_probe` runs) against real `HEAD` (5c02e86), to re-confirm 013's number stands rather than just
re-citing it.

## What was tried

Harness: `___OPTIMIZATIONS___/harness/rss_probe`'s existing `formula` mode (added by candidate
013) - release build, clean rebuild per side, `sig`/`cells` compared every run. Fixture:
`scripts/gen_formula_xlsx.py 300000 8` (same generator as 013), a synthetic 2.4M-formula-cell XLSX,
18 MB on disk, accurate `<dimension>` header.

| Variant | peak RSS (KB) | elapsed | notes |
|---|---|---|---|
| baseline (`HEAD` 5c02e86), `taskset -c 3`, run 1 | 209,884 | 885 ms | current code: sparse `Vec<Cell<String>>` then `from_sparse()` |
| baseline, `taskset -c 3`, run 2 | 209,988 | 797 ms | |
| baseline, unpinned, run 1 | 209,032 | 789 ms | |
| baseline, unpinned, run 2 | 209,384 | 784 ms | |
| patched, `taskset -c 3`, run 1 | 135,036 | 724 ms | this candidate: direct dense write via `from_sparse_dense_hint()` |
| patched, `taskset -c 3`, run 2 | 134,800 | 743 ms | |
| patched, unpinned, run 1 | 134,812 | 731 ms | |
| patched, unpinned, run 2 | 134,212 | 721 ms | |

Baseline mean (4 runs, 2 pinned + 2 unpinned): **209,572 KB**. Patched mean (4 runs): **134,715
KB**. That is a **35.7% peak-RSS reduction** - matching candidate 013's own 35.5% (this session's
baseline/patched means both land within ~0.3% of 013's original numbers). `sig` (41765480) and
`cells` (2,400,000) were bit-identical between baseline and patched on every run, including a
separate, independent run of `just build-baseline`/`just build-patched`/`just bench` in this
candidate's own scratch clone (`peak_rss_kb=209104` baseline vs `peak_rss_kb=134380` patched,
`sig`/`cells` identical - printed directly by `just test`'s component recipes, verified for real
this session, not just read).

## Result

Real, reproducible **35.7% peak-RSS reduction** (209,572 KB -> 134,715 KB mean) on a 2.4M-cell
synthetic formula-heavy fixture, bit-exact output (`sig`/`cells` identical on every run). Metric
measured: **peak RSS** (`VmHWM`); elapsed time is a secondary, non-conclusive signal (patched
trending faster, ~730ms vs ~810ms, but the spread isn't wide enough above this box's own
run-to-run noise to make a confident wall-clock claim either direction - same caveat 013 recorded).
This is an independent re-measurement of candidate 013's own finding, not a new mechanism - the
patch content is identical (copied verbatim from `013.../patch.diff`), applied and measured for
real in this candidate's own isolated git worktree rather than cited from 013's prior run.

## Does it change behavior/output?

- **Bit-exact?** Yes, on every check run this session:
  - The synthetic 2.4M-cell fixture (`sig`/`cells` identical, pinned and unpinned, both the manual
    worktree runs above and the independent scratch-clone `just test` run).
  - The project's own `tests/formula.issue.xlsx` (3 scattered formula cells, real `<dimension>
    ref="A1:J14"`) plus 4 synthetic wrong-`<dimension>` variants built specifically to exercise
    `from_sparse_dense_hint()`'s fallback-to-sparse path (`scripts/gen_dimtest.py`, reused
    verbatim from 013): `A1:A1` (declared box far smaller - forces a mid-stream bail-to-sparse),
    `A1:Z50` (declared box larger - exercises the crop-to-tight-bounding-box path), no
    `<dimension>` tag at all, and `J14:A1` (reversed/degenerate). Ran all 5 through
    `bench/formula_probe` (calls `worksheet_formula("Sheet1")`, prints `start()`/`end()`/every
    non-empty cell) against this candidate's own worktree, once before applying the patch (`git
    checkout -- src/`) and once after - **all 5 byte-for-byte identical**, and independently
    reproduced via `just correctness` in this candidate's own scratch clone (printed `IDENTICAL`
    for all 5).
- **Is this a breaking change?** No public API change - `worksheet_formula()`'s documented
  behavior is unchanged (still ignores `HeaderRow`, still returns only non-empty formula strings
  in a dense `Range<String>`). `Range::from_sparse_dense_hint()`/`Range::collect_sparse()` and
  `MAX_DENSE_RANGE_CELLS` are all `pub(crate)`/private, invisible outside the crate.
- Project test suite, `cargo test --release --all-features`, run in this candidate's own worktree,
  baseline and patched:
  - Baseline: **45 passed / 0 failed** (unit), **165 passed / 0 failed / 1 ignored** (integration -
    the 1 ignored test is pre-existing, unrelated to this change), **67 passed / 0 failed** (doc
    tests).
  - Patched: identical counts on all three - **45/0/0**, **165/0/1**, **67/0/0**. No new failures,
    no fewer passes.

## Why the result looks the way it does

Same underlying mechanism candidate 013 already explained: `worksheet_formula()` returns
`Range<String>` directly with no second dense-buffer conversion step (unlike `worksheet_range()`,
which candidate 007 addressed separately and which still pays for a `DataRef -> Data` conversion
on top), so removing the intermediate sparse `Vec<Cell<String>>` removes a larger fraction of this
path's total peak allocation than it did for the value-range path - hence the larger percentage
reduction (35.7% here vs 007's own 21.8% for `worksheet_range_ref()` on an equivalently-shaped
file). This candidate's own numbers landing within ~0.3% of 013's original run is expected - same
toolchain, same fixture generator, same patch content - the measurement is a straightforward
re-run, not a re-derivation.

## What's not done / open questions

Same open items 013 already flagged, none newly investigated this session:

- No large, real-world (non-synthetic) formula-heavy XLSX/XLSB fixture was found - this
  measurement rests on a generated file (`scripts/gen_formula_xlsx.py`), not a real spreadsheet.
- XLSB's own `worksheet_formula()` was patched identically (verified via `cargo test`, which does
  exercise XLSB) but was not separately peak-RSS-measured with a large XLSB-specific fixture - no
  large XLSB fixture was available this session either.
- The fallback-to-sparse path's own *performance* on a large mismatched-`<dimension>` file (does
  bailing out mid-stream cost more than the original code) was checked for correctness only, not
  measured for performance - out of scope given accurate `<dimension>` headers are the norm on
  real files.
- Did not re-measure combined with candidates 007/011's own patches on top (that combination is
  candidate 017's own scope, already re-confirmed there) - this candidate targets only the
  `worksheet_formula()` call site in isolation, applied for real.
