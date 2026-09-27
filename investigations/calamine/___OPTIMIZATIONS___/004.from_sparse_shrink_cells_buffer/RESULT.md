# `shrink_to_fit()` the sparse `cells` buffer before `Range::from_sparse()` builds the dense grid

Candidates 001/002's analysis established that `Range::from_sparse()`'s peak RSS comes from the
sparse `Vec<Cell<T>>` (collected during parsing) staying alive while the dense `Vec<T>` grid is
built from it. For inputs large enough that the caller skips an upfront `Vec::reserve()` (XLSX/XLSB
do this above a size threshold, specifically to avoid trusting an attacker-controlled `<dimension>`
hint for the reservation amount), `cells` grows via `Vec`'s normal amortized-doubling push
strategy, which can leave its `capacity()` up to ~2x its `len()` once collection finishes -
wasted, unused memory carried through the exact window where it overlaps the dense grid. Calling
`cells.shrink_to_fit()` right before allocating the dense grid (`cells` is still alive/needed for
the copy loop after) should reclaim that slack for free, no correctness risk.

## What was tried

Added `cells.shrink_to_fit()` in `Range::from_sparse()` (`src/lib.rs`), immediately after the
bounds-computation loop and before `let mut v = vec![T::default(); len]`. Changed the function
signature from `pub fn from_sparse(cells: Vec<Cell<T>>)` to `pub fn from_sparse(mut cells:
Vec<Cell<T>>)` to allow it.

| Variant | `big_300k_rows.xlsx` (2.4M real cells, no upfront reserve) peak RSS | `sparse_large_range.xlsx` (5M declared, 22 real cells) peak RSS | notes |
|---|---|---|---|
| baseline (`HEAD` 5c02e86) | 171,564-172,244 KB (6 runs across candidates 001/002/this one) | 159,128-159,668 KB (6 runs across candidate 002/this one) | current code |
| patched | 172,036-172,168 KB (3 runs) | 159,408-159,808 KB (3 runs) | this candidate |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (`range` mode, peak RSS = `/proc/self/status`
`VmHWM`), release build, clean rebuild.

## Result

**No measurable change** - patched numbers fall entirely within the baseline's own run-to-run
spread on both files (a wider spread than the ~56 KB difference between the two variants' means).
This session's metric is peak RSS; wall-clock also showed no meaningful difference.

## Does it change behavior/output?

- **Bit-exact?** yes - `sig`/`cells` identical to baseline on both files, every run.
- **Is this a breaking change?** No public API change (the new `mut` is on an owned parameter,
  invisible to callers).
- Project test suite: 165/166 integration tests passing both before and after (1 pre-existing
  `#[ignore]`, unrelated), 45/45 unit tests both before and after, 67/67 doc tests both before and
  after (`cargo test --all-features`, run once on unpatched `HEAD` via `git stash`, once patched).
  `cargo clippy --all-features --all-targets` clean.

## Why the result looks the way it does

Two plausible explanations, not distinguished from each other in this session (would need
allocator-level instrumentation, e.g. a custom global allocator counting realloc calls/sizes, to
tell apart - out of scope for the time spent on this candidate):

1. **`cells`' actual growth pattern may already land close to its final length** for these
   particular cell counts (2.4M and 22 real cells respectively) - Rust's amortized-doubling growth
   doesn't guarantee exactly 2x overshoot, and depending on where the final `push` lands relative
   to a capacity doubling boundary, there may simply be little slack to reclaim for these specific
   sizes.
2. **`shrink_to_fit()`'s `realloc()` may not return memory to the OS in a way `VmHWM` reflects
   immediately** - depending on the allocator (glibc malloc here) and whether the block was
   serviced by `mmap` (large allocations) vs. the main arena, shrinking a large allocation can
   leave the process's mapped address space (and therefore RSS, if pages were ever touched) larger
   than the logical `Vec` capacity suggests, especially since `VmHWM` is a high-water mark that
   never decreases within a process's lifetime regardless of later frees - if the ORIGINAL
   over-sized allocation's pages were already touched (written to) before `shrink_to_fit()` runs,
   the high-water mark it set is unaffected by freeing memory afterward. This second explanation
   is the more likely one on reflection: `VmHWM` records the peak that already occurred, so
   `shrink_to_fit()` can only help if it runs *before* the peak is reached, not after - and here it
   runs after `cells` has already been fully populated (already at its peak capacity), immediately
   before the dense grid's own allocation adds to the total. Freeing `cells`' slack at that point
   could still lower what the dense-grid-allocation moment's *total* resident set would have been,
   unless the freed pages aren't actually returned to the OS by the allocator (glibc's `malloc`
   frequently keeps freed memory in its arena for reuse rather than returning it via `munmap`,
   which would explain a realloc-to-smaller not reducing observed RSS at all).

## What's not done / open questions

- Did not instrument the allocator to determine which of the two explanations above (or some
  combination) is actually responsible - would need a custom `GlobalAlloc` wrapper counting/logging
  `realloc` calls and their before/after sizes to settle this with evidence rather than plausible
  reasoning.
- Did not try `Vec::with_capacity` + manual growth tuning, or switching the fallback (no-upfront-
  reserve) path to a different growth strategy - this candidate only tried the cheapest possible
  fix (`shrink_to_fit()` after the fact), consistent with it being a bounded, low-risk experiment
  rather than a deep dive.
