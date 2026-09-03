# Review of GitHub PR #697: `Range::from_sparse` returns `Result` instead of aborting

`rustc 1.98.0`, `cargo 1.98.0`.

PR #697 (tafia/calamine, open, not merged - fixes issue #693) changes `Range::from_sparse`'s
public signature from `fn from_sparse(cells: Vec<Cell<T>>) -> Range<T>` to
`fn from_sparse(cells: Vec<Cell<T>>) -> Result<Range<T>, RangeError>`. Instead of
`vec![T::default(); len]` (which aborts the process via Rust's global allocator on allocation
failure, uncatchable), it does `Vec::new()` + `try_reserve_exact(len)` + `resize(len,
T::default())`, returning `Err(RangeError { width, height })` if the reservation fails. It also
widens the width/height extent arithmetic to `u64` before the `+ 1` (a second, 32-bit-only
overflow route the current `usize` arithmetic has). The new error is threaded through `XlsError`,
`XlsbError`, `XlsxError` via a new `Range(RangeError)` variant on each, so
`worksheet_range()`/`worksheet_range_ref()` return `Err` instead of crashing.

**This is the exact same underlying crash this workspace already found and fixed independently**
in candidate 001 (`001.xlsx_xlsb_dense_range_cell_cap`, `___BUGHUNT___/001.xlsx_xlsb_dense_range_oom`)
- a sheet with two cells at opposite extreme corners (e.g. `A1` and `XFD1048576`) makes
`from_sparse` attempt a ~549 GB `vec![T::default(); len]` allocation and `SIGABRT`s the whole
process. Verified this PR's fix actually closes that exact crash on this checkout (see
"Correctness" below) - independent confirmation, not just reading the PR description.

The two fixes differ in mechanism and in what they change for a legitimate large file:
candidate 001 imposes a **hard 100M-cell cap** before allocating (silently degrading/rejecting
anything past that, regardless of whether the allocation would have actually succeeded), whereas
this PR **always attempts the allocation** via `try_reserve_exact` and only errors if the system
genuinely can't provide the memory - so a real (if unusual) worksheet needing, say, 200M dense
cells that the machine can actually satisfy still succeeds under this PR, but would be rejected by
candidate 001's fixed cap. Neither is a strict subset of the other; a maintainer combining both
approaches would need to decide whether the cap should stay as a defensive ceiling on top of (or
instead of) the fallible-allocation approach.

## What was applied to this worktree

The PR's own branch had already diverged from this project's `master` in `src/xls.rs` and
`src/xlsx/mod.rs` (unrelated later commits added new error variants/behavior in both files since
the PR's own base commit) - `git apply` of the PR's raw diff failed outright on those two files
(clean on `src/lib.rs` and `src/xlsb/mod.rs`). Hand-rebased the same logical change onto this
candidate's `base_commit` (`5c02e86`): same `RangeError` type and `from_sparse` body as the PR,
same `XlsError::Range`/`XlsbError::Range`/`XlsxError::Range` variant + `Display`/`source()`/`?`
wiring at each of the four call sites (`xls.rs`, `xlsb/mod.rs` x2, `xlsx/mod.rs` x2). Did **not**
port the PR's own new tests (`tests/issue_693.xlsx`, the two new unit tests in `tests/test.rs`) or
its doc-comment `# Errors` section - out of scope for a performance-only rebase; the correctness
behavior itself was independently verified below instead of relying on the PR's own test additions.

## What was tried

| Variant | single-core p50 (`taskset -c 3`, elapsed_ms) | multi-core p50 (unpinned, elapsed_ms) | peak RSS (`VmHWM`) | notes |
|---|---|---|---|---|
| baseline (`HEAD` 5c02e86) | 638 / 746 (2 runs) | 633 / 637 (2 runs) | 172,236 / 172,260 KB pinned, 171,684 / 171,916 KB unpinned | current: `vec![T::default(); len]` |
| patched (PR #697 rebased) | 643 / 692 (2 runs) | 621 / 622 (2 runs) | 172,340 / 172,424 KB pinned, 171,892 / 172,336 KB unpinned | `Vec::new()` + `try_reserve_exact` + `resize` |

Paired alternating comparison (`tools/paired.sh`, adapted to parse `elapsed_ms=`, `RUNS=9`,
`taskset -c 3`, same `big_300k_rows.xlsx` fixture):

```
A (baseline): 652 626 622 631 624 630 634 629 639
B (patched):  627 626 625 631 628 623 622 621 624
median 630.000 -> 625.000 (-0.8%)  B faster in 5 of 9 pairs
verdict: NOT consistent - the medians are not evidence
```

## Result

**No measurable performance difference**, time or peak RSS. 5/9 paired-alternating rounds is
indistinguishable from noise (a real, consistent effect would show up as 8-9/9 or 0-1/9 per this
workspace's own paired.sh convention). Peak RSS is likewise within run-to-run noise on both sides
(~171.7-172.4 MB either way). This matches the code-level expectation: `Data` (this crate's
element type for the hot `worksheet_range()` path) is not a primitive/all-zero type the standard
library's `vec![x; n]` "IsZero" specialization applies to, so both `vec![T::default(); len]` and
`Vec::new(); try_reserve_exact(len); resize(len, T::default())` go through the same general
clone-and-extend fill loop - `try_reserve_exact` only changes *how allocation failure is reported*
(a catchable `Err` vs. an uncatchable process abort), not the cost of a successful allocation.
Metrics measured: wall-clock (primary, since neither mechanism should move memory pressure) and
peak RSS (`VmHWM`) as a secondary check, both on `big_300k_rows.xlsx` (2.4M real cells, 20MB
XLSX). No conflict found with the pre-sized dense-buffer restructuring (candidate 007's
`from_sparse_dense_hint`) - this PR only touches the fallback `from_sparse` path's allocation
mechanics, not its calling convention or the surrounding `HeaderRow` logic candidate 007 also
touches; the two changes are compatible (both could be applied together - `from_sparse_dense_hint`
would just need its own equivalent `try_reserve_exact` treatment for the same crash class, since it
also does an upfront dense allocation from a declared/derived size).

## Does it change behavior/output?

- **Bit-exact?** Yes, on the real fixture (`sig`/`cells` identical baseline vs. patched, every
  run) and on `tests/issue_174.xlsx` (sparse-but-large-declared-extent real fixture, densifies to
  2x11 both before and after - manually re-checked with a standalone probe, not just read from the
  PR description).
- **Is this a breaking change?** Yes - `Range::from_sparse`'s public return type changes from
  `Range<T>` to `Result<Range<T>, RangeError>`; every direct caller (in this crate and any
  downstream user) must add `?`/`.unwrap()`. Confirmed the crash this fixes is real and current:
  built a standalone repro (two cells at `(0,0)` and `(1_048_575, 16_383)`) against this
  patched worktree - it now returns `Err("Cannot allocate a 1048576 x 16384 range")` cleanly
  instead of aborting (the same shape as this workspace's own `___BUGHUNT___/001.xlsx_xlsb_dense_range_oom`).
- Project test suite: **269 passed, 1 ignored, 0 failed before AND after** (43 unit + 162
  integration + 65 doctest, unchanged pass/fail counts - the PR's own new tests were not ported
  into this rebase, see "What was applied" above, so the count didn't grow, but nothing regressed
  either).

## Why the result looks the way it does

`try_reserve_exact` and the `vec![]` macro both ultimately call the same allocator entry point on
the success path for a non-trivially-zero element type like `Data` - the PR's own change is a
correctness/robustness fix (turn an uncatchable abort into a catchable `Result`), not a
performance optimization, and it doesn't claim to be one. It was worth checking anyway because it
touches the exact allocation site three other candidates in this workspace (001/003/009) also
touch for the same crash class, with a different fix shape (hard cap vs. fallible reserve) -
confirming there's no hidden performance cost to adopting the PR's approach instead of (or
alongside) this workspace's own cap-based fix removes that as a blocker to any future decision
about which approach to prefer.

## What's not done / open questions

- This rebase does not include the PR's own new tests (`tests/issue_693.xlsx`,
  `from_sparse_oversized_extent_returns_an_error`, `from_sparse_keeps_working_for_a_sparse_but_sane_extent`)
  - a maintainer adopting the PR for real would use the PR's own branch/tests, not this
    performance-review rebase.
- Not measured: the 32-bit overflow route the PR also closes (`u64`-widened extent arithmetic) -
  this environment is x86_64 only; the fix there is straightforward arithmetic widening with no
  plausible performance cost, not re-verified here.
- Not decided (out of scope for this candidate): whether this crate should prefer PR #697's
  "always try, report failure" approach or this workspace's own candidate 001's "hard cap"
  approach, or combine both (cap as a fast-reject guard, `try_reserve_exact` as the second line of
  defense for anything under the cap that still can't be satisfied on a memory-constrained
  machine). Recorded as a question for whoever reviews both, not resolved here.
- `from_sparse_dense_hint` (candidate 007, not applied to `master`) was read but not modified or
  re-measured together with this PR's change - noted above that they look compatible, not
  verified by actually combining both patches in one build.
