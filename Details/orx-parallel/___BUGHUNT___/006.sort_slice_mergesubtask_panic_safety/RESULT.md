# sort_slice_mergesubtask_panic_safety

`MergeSubTask::execute` in `src/sort/sort_slice.rs` (the experimental parallel sort behind the
`experimental` Cargo feature, `par_experimental_sort`) moves elements between the caller's own
slice and an auxiliary `Vec<MaybeUninit<T>>` scratch buffer via raw `copy_nonoverlapping`, driven
by `T::lt`/`T::le` comparisons on the caller-supplied `T: Ord`, with no panic-safety guard around
those comparisons. Flagged by an existing `code-audit-structured` pass
(`___FINDINGS___/findings.toml`, use_after_free/hard).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning from the merge algorithm's own invariants (boundary-value construction, per `HEURISTICS.md`'s "logic bug"/"state-machine invariant" guidance) | Traced how `orig` (the caller's slice) and `aux` alternate as source/destination across passes, and what a *partial* write into one merge task's own destination range leaves behind if interrupted mid-loop | Predicted a concrete corruption mechanism (see "Root cause") entirely from reading the code, before running anything |
| A one-off comparator-panic probe (`examples/panic_probe.rs`, deleted after use, not part of the deliverable): a custom `Elem{key,id}` whose `Ord` counts total comparisons and panics at a chosen threshold, sweeping the threshold over 1-99% of a measured total-comparison count for `n=200_000` | debug-assertions off, `--release --features experimental`, fixed xorshift64 seed, single machine (24 logical CPUs) | Roughly 15 of 99 sampled thresholds produced a slice whose ids were no longer a valid permutation (duplicated ids exactly matched missing ids in count) - confirming the predicted mechanism land specifically in odd merge passes (`dst == orig`) |
| Standalone `repro/` (real `Box<u8>`-backed payload, so a duplicated id's two physical copies own the same heap pointer) run directly against the pristine tree, 5 repeat runs at a fixed threshold (`2_731_348`) and `n=200_000`, plus 3 more repeat runs at a second, different threshold (`3_090_736`) to vary the input within the same bug class | `--release`, same fixed seed each time | 5/5 runs at the first threshold, and 3/3 runs at the second threshold, aborted with a real double-free reported by glibc's allocator (`double free or corruption (!prev)` / `corrupted size vs. prev_size while consolidating`, `SIGABRT`, exit code 134) |
| Same `repro/`, patched tree (`patch.diff` applied), same two thresholds, 5 repeat runs each | `--release` | 5/5 (first threshold) and 3/3 (second threshold) completed cleanly: the comparator still panics (expected - a user comparator is allowed to panic), but the caller's slice was a valid permutation every time, so it dropped without crashing |
| Full standalone `just test` cycle (fresh `git clone` of `base_commit`, `prereq_001.patch`, `reproduce-before`, `patch.diff`, `reproduce-after`) in a scratch directory outside this checkout | same as above | Reproduced the double-free from a genuinely fresh clone (not the working tree used to develop the fix), then showed it gone after `patch.diff` |
| Project test suite, `cargo test --release --features experimental --lib`, before and after `patch.diff` (with `prereq_001.patch` applied in both cases, since the crate does not compile under `--features experimental` at all without it - see "What's not done" below) | `--release` | 113 passed; 0 failed, identically before and after (the existing suite has no test that supplies a panicking `Ord`, so it does not exercise this path either way) |
| Project test suite, `cargo test --release --lib` (default features, no `experimental`) | `--release` | 99 passed; 0 failed, identically before and after (this code is entirely feature-gated) |

No `miri`/ASAN was used: the mechanism is a plain, deterministic logic/memory-ownership error
(the invariant "every address in the slice holds exactly one live element" is violated by a
partial raw copy), fully reproducible and explainable via reasoning plus a real allocator crash -
a sanitizer would add confirmation but nothing new to the diagnosis. A fuzz harness was not used
either: the panicking-comparator technique is the standard, targeted way to test this exact bug
class (the same one the Rust standard library's own sort test suite uses), and is far more direct
than fuzzing for an equivalent trigger.

## Status

**Confirmed**. Reproduces 5/5 runs (plus 3/3 at a second, independently chosen threshold) under
`cargo run --release` against the pristine tree, both from the working checkout and from a fresh
`git clone` via `just test` - a real double-free reported by the system allocator (SIGABRT, exit
134), not merely an internal bookkeeping mismatch. After `patch.diff`, the same repro at the same
two thresholds is clean 5/5 and 3/3.

**Fully reproducible**: **Yes**. There is no second participant to fake here (unlike
`CONFIRMATION.md` section 11's client/server case): the "input" this finding is about is a
user-supplied `Ord`/`PartialOrd` implementation, which `par_experimental_sort`'s own public
signature (`T: Ord + Send`) accepts from any real caller with no promise that it can't panic.
`repro/` calls the real, unmodified public `par_experimental_sort` end-to-end with a
comparator that panics after a fixed number of calls - the standard, direct technique for
panic-safety testing (not a hand-constructed internal state fed straight to a private helper) -
and then really drops the resulting (corrupted) slice, letting the system allocator's own
double-free detector be the witness, rather than only inspecting bookkeeping.

## Class

memory-safety (a real double-free / use-after-free on unwind, plus a paired memory leak for the
element whose only surviving copy sits in the abandoned scratch buffer - see "Root cause").

## Root cause

`par_experimental_sort` sorts each initial chunk in place (`orig`), then runs a hierarchical
sequence of merge passes that alternate the roles of `orig` and an auxiliary
`Vec<MaybeUninit<T>>` buffer (`aux`) as source/destination: pass 0 reads `orig`/writes `aux`,
pass 1 reads `aux`/writes `orig`, and so on. Each `MergeSubTask` covers one absolute address
range that is *the same range* in both buffers (its destination in one buffer is exactly its own
source range in the other), and by construction of this address partition, the two buffers'
current contents for that same range are always two different, individually valid permutations
of the exact same underlying value set.

The "standard merge" branch of `MergeSubTask::execute` (used whenever neither buffer's whole
sub-range already precedes the other, i.e. not the two fast paths above it) walks `src_a`/`src_b`
and writes one element at a time into `dst`, driven by `*ptr_b < *ptr_a`. If that comparison
(the caller's own `T::lt`) panics partway through, the function unwinds with only a *prefix* of
`dst`'s range written - the untouched suffix still holds whatever was in that address range
*before this task ran* (a different, unrelated permutation of the same value set, not the
complementary remainder of the sorted merge). Because a raw `copy_nonoverlapping` never "moves
out" its source, the values just written into the prefix are also still physically present,
unmodified, in the *other* buffer where they were copied from. In general these two facts
combine as: some values now sit at two different addresses within `orig` (whichever ones the
partial merge finished copying, still also present in their old spot), while other values (the
ones the stale suffix displaced) exist nowhere else in `orig` at all - their only remaining copy
is in `aux`, a buffer whose owning `Vec<MaybeUninit<T>>` never runs `T`'s destructor when it is
eventually dropped.

Concretely, once the caller's real `Vec<T>` backing `orig` is later dropped (as unwinding
continues past that point, or the caller inspects/reuses it after a caught panic), it calls
`T::drop` once per address - so the duplicated value gets dropped twice (double-free/use-after-
free for any `T` that owns a resource, e.g. `Box`/`String`/`Rc`), while the displaced value's own
resource is never dropped at all (a memory leak). This only happens for merge tasks whose
destination buffer is `orig` (odd passes, i.e. whenever there are at least 2 merge passes,
`num_chunks >= 4`); a panic during pass 0 (`dst == aux`) is harmless, since `orig` is read-only
that pass and `aux`'s `MaybeUninit<T>` never drops its contents regardless of how much of it got
written. The two fast-path branches (`last_a <= first_b`, `last_b < first_a`) are also safe: their
one comparison happens before any write in that task, so a panic there leaves the task's whole
range untouched (the "0% done" case, which - like "100% done" - is safe; only a *partial* write is
unsafe). `find_split` (used only to size sub-tasks, before any task's own writes) is safe for the
same reason.

## Introduced by

8b0e5655 "experimental sort" (2026-08-28) - this commit added `src/experimental_sort.rs`
(later renamed, unchanged, to `src/sort/sort_slice.rs` by 35ff6900 "experimental sort",
2026-08-28, a pure file move with no content diff) already containing today's exact "Standard
merge" loop shape with no panic-safety guard. `git log --follow -- src/sort/sort_slice.rs` shows
only these two commits ever touched this file's content; the bug has existed since the feature's
introduction.

## The fix

Replaced the manually-tracked `ptr_a`/`ptr_b`/`out` locals with a small `MergeGuard<T>` holding
the same state, whose `Drop` impl unconditionally (no further comparisons) blits whatever remains
of `src_a`'s and `src_b`'s un-merged tails into `dst`. This runs identically whether the scope
exits normally (after the loop finishes, exactly reproducing the previous explicit
"copy the remaining tail" logic) or via an unwind triggered by a panicking comparison partway
through - so `dst`'s whole range always ends up fully populated with exactly one copy of every
element assigned to that task, closing the "partial write" state that caused the corruption. This
mirrors the same "hole guard" panic-safety pattern the Rust standard library's own stable merge
sort uses for an equivalent hazard.

Verified against two different failure points in the same bug class (`CONFIRMATION.md` section
7): threshold `2_731_348` (the one first found) and threshold `3_090_736` (a different point,
found by the same sweep, also inside an odd merge pass) - both corrupted the slice 5/5 (resp.
3/3) times before the fix and were clean 5/5 (resp. 3/3) times after, so the fix is not narrow to
the one specific comparison count that was first observed to trigger it.

- Project test suite: `cargo test --release --features experimental --lib` - 113 passed; 0 failed
  before, 113 passed; 0 failed after (unchanged; no existing test supplies a panicking comparator).
  `cargo test --release --lib` (default features) - 99 passed; 0 failed before and after
  (unaffected either way, since this code is entirely behind `--features experimental`).

## What's not done / open questions

- On this exact `base_commit`, the crate does not compile at all under `--features experimental`
  - a separate, already-`Confirmed` bug in this same `___BUGHUNT___` session, candidate 001
    (`collect_arb_over_bag_bool_type_mismatch`, a one-line type mismatch in
    `src/infallible/par_runner.rs`, unrelated to this candidate). This candidate's own `repro/`
    and its `justfile` apply candidate 001's already-verified one-line fix
    (`prereq_001.patch`, a copy of that candidate's own `patch.diff`) immediately after cloning,
    purely so the tree can build under the feature this candidate needs - it is not part of this
    candidate's own fix, and both `reproduce-before` and `reproduce-after` apply it identically.
    If a future session finds 001 has been fixed upstream independently, `prereq_001.patch` can
    simply be dropped from the `clone`/`prereq` step.
- The same "partial write into a buffer the caller still owns, on a comparator panic" shape was
  only checked for `MergeSubTask::execute`'s standard-merge branch. `SortChunkTask::execute`
  (`slice::sort_unstable`, standard library, already panic-safe by design) and `CopyChunkTask`
  (an unconditional bulk copy with no comparisons at all, so nothing can panic partway through it)
  were read and are not affected by this same class.
- A longer fuzzing campaign (varying `n`, chunk-count boundaries, and comparator panic points
  systematically, via `fuzz-bug-repro` run separately) would give broader coverage than the
  two manually-swept thresholds checked here, but was not run this session.
- The reproduction rate is exactly 5/5 and 3/3 (not merely "usually"), but the *specific*
  duplicated/missing id counts vary slightly run-to-run (e.g. 1061 to 3150 out of 200,000) since
  real OS thread scheduling affects exactly how many elements a given merge task had written by
  the time the Nth comparison (across all threads) lands - this is the "reproduces on demand, but
  the exact corruption shape is layout-dependent" case `CONFIRMATION.md` section 3 describes, not
  a reason to discount the finding.
