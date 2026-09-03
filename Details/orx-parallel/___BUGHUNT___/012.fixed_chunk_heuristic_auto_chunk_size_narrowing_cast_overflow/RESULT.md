# fixed_chunk_heuristic_auto_chunk_size_narrowing_cast_overflow

`auto_chunk_size` (`src/runner/runner_variants/fixed_chunk/heuristic.rs:15-45`) computes
`thread_load = initial_len / max_num_threads` from `ConcurrentIter::size_hint().1` (a caller-
supplied iterator's exact upper bound, e.g. a `Range<usize>`), then does
`let mut diff = (c as i64 - DESIRED_CHUNK_SIZE as i64).abs();` (line 26, with `c = thread_load`)
and the same cast/subtraction again inside the halving loop (line 30). This is on the default
`ChunkSize::Auto` path, called unconditionally from `FixedChunkRunner::new_state` (and, since
commit b9214fbe, also from `AdaptiveChunkRunner::configure_for_serialized_input`), reachable from
ordinary public API calls - flagged from `IDEAS.md` as analogous to confirmed candidate 009's
overflow (a lazy `Range` reaching astronomically large `size_hint` values with O(1) memory).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Manual boundary-value construction through the crate's real public API: `(0usize..total).par().runner(Runner::fixed_with_pool(Pool::basic(1))).sum()` (a real `std::ops::Range`, the crate's own public `FixedChunkRunner` + a real single-worker `Pool::basic(1)`), `total = 1usize << 63` (the exact left edge of the panic window for `c as i64 - 1024`), `cargo run` (dev profile, debug-assertions on). First attempt used `.num_threads(1)` instead of a 1-thread pool, which turned out to route through `params.is_sequential()`'s fast path (`Params::is_sequential` returns true for `NumThreads::Max(1)`) - that path bypasses `nt_state`/`new_state`/the heuristic entirely, so no overflow (or even any call into `compute_chunk_size`, confirmed via a temporary debug `eprintln!` in the function). Switching to `NumThreads::Auto` (the default) with a real 1-thread `Pool::basic(1)` makes `max_num_threads_for_computation` resolve to 1 via the pool-capacity constraint instead, without tripping the sequential-request fast path, and reaches the heuristic. | Panics immediately (well under 1 second) with `attempt to subtract with overflow` at `heuristic.rs:26:36`, process exit code 101. |
| 2. Repeat runs, identical build/input | 4/4 runs panic identically (`heuristic.rs:26:36`, same message), deterministic - not layout/seed-dependent (see `output.txt`). |
| 3. Release build (`cargo build --release`), same input | No panic within an 8s timeout (killed by `timeout`, still running) - the multiplication^H^H^Hsubtraction silently wraps under `overflow-checks = false` and the (would-be astronomically long) actual computation proceeds instead of crashing. Confirms this is a debug-assertions-only manifestation, per `CONFIRMATION.md` section 2. |

No escalation past a debug-assertions build + boundary-value construction was needed - approach 1
gave a clean, deterministic, on-demand reproduction (per `HEURISTICS.md`'s "arithmetic on
untrusted sizes" guidance), so the soft cap-of-3 in `SKILL.md`'s candidate loop was not fully
exercised (the first attempt's dead end from `NumThreads::Max(1)`'s sequential shortcut was
resolved within the same approach, not counted as a separate escalation).

## Status

**Confirmed.** Reproduces 4/4 runs, debug-assertions build (`cargo run`'s default dev profile,
equally reachable via `cargo test`), deterministic (pure integer-arithmetic overflow, not
layout/seed-dependent).

**Fully reproducible**: **Yes**. The repro drives the crate's real public entry points
(`Range<usize>::par()`, `Runner::fixed_with_pool(Pool::basic(1))`, `.sum()`) with a real
`std::ops::Range` and a real single-worker thread pool - a genuine external input/configuration a
real caller could construct (e.g. summing a range-based computation while explicitly requesting
the fixed-chunk runner, or any caller whose configured thread count or pool capacity happens to
resolve to 1 thread for a huge input), not a hand-built internal `State`. This is a
single-participant finding (one caller handing one iterator + runner/pool choice to one public API
call, no peer/wire interaction to fake), so `CONFIRMATION.md` section 11's two-participant bar
does not apply.

## Class

crash

## Root cause

`auto_chunk_size` computes `thread_load = initial_len / max_num_threads` as a plain `usize`, then
immediately casts it to `i64` (`c as i64`) to compute `diff = (c as i64 - DESIRED_CHUNK_SIZE as
i64).abs()` (`DESIRED_CHUNK_SIZE = 1024`), and repeats the same cast+subtract inside the
chunk-size-halving loop for `c2 = c / 2`. A `usize` value `v` in `[1usize << 63, usize::MAX]` does
not fit in `i64` (whose max is `2^63 - 1`), so `v as i64` wraps to a negative number computed as
`v - 2^64` (two's-complement truncation) - the most negative possible result, `i64::MIN`, occurs
exactly at `v = 1usize << 63`. For `v` in the narrow window `[1usize << 63, (1usize << 63) +
1023]`, `v as i64` is close enough to `i64::MIN` that subtracting `1024` underflows past
`i64::MIN`, which panics with "attempt to subtract with overflow" under a debug-assertions build
(the default `cargo build`/`cargo test` profile) - or silently wraps to a nonsensical value in
release, corrupting the chunk-size selection instead of crashing (see `should_stop_exploration`'s
near-identical failure mode in candidate 009's `RESULT.md`, an unrelated but structurally similar
overflow in the same runner-heuristics area).

`thread_load` is `initial_len / max_num_threads`, where `initial_len` comes straight from the
input iterator's `size_hint().1` (its exact/upper-bound length, e.g. any `ExactSizeIterator`-shaped
input such as a plain `Range<usize>`) - a lazy range reporting a huge size_hint costs O(1) memory,
so no actual allocation of ~9.2e18 items is needed to reach the panic window. The window is only
hit when `max_num_threads` divides `initial_len` down to land in
`[1usize << 63, (1usize << 63) + 1023]` (verified here with `max_num_threads = 1` via a real
1-thread `Pool::basic(1)` and `initial_len = 1usize << 63` exactly, giving `thread_load =
initial_len` unchanged) - a narrow but entirely reachable combination of "huge lazy range" +
"pool/request that resolves to few threads", not requiring any internal/private API to construct.

Note one dead end while constructing the repro: requesting `NumThreads::Max(1)` directly (e.g.
via `.num_threads(1)`) does *not* reach this code at all - `Params::is_sequential()` treats an
explicit request for exactly 1 thread as "run fully sequentially" and takes a different code path
(`iter.into_seq_iter()...`) that never calls `nt_state`/`new_state`/the heuristic. The panic
requires `max_num_threads` to resolve to 1 through the *pool's own capacity* constraint (or some
other combination that lands in the same numeric window with more than 1 thread requested), not
through the user directly asking for 1 thread.

## Introduced by

293f08d9 "define chunk size resolution" (2026-03-26) introduced this exact `(c as i64 -
DESIRED_CHUNK_SIZE as i64).abs()` pattern (both occurrences) in the original
`src/runner/runner_impl/fixed_chunk/resolved_chunk_size.rs` (later renamed/moved to today's
`src/runner/runner_variants/fixed_chunk/heuristic.rs` across commits 56803081 "module renaming",
a6cee8ab/2565e0d3 "rename chunk size heuristic", 769dd770 "define state of fixed chunk runner",
792d0610 "clean up", cb2b36a6 "provide more information through size_hint rather than initial
length" - none of these renames/refactors touched the arithmetic itself). Identified via `git log
-S "as i64 - DESIRED_CHUNK_SIZE as i64"` plus `git log --follow` on the file's rename history.
Commit b9214fbe "iter of iter uses fixed chunk heuristic" (2026-08-30, the branch tip this
worktree is checked out at) did not introduce the bug - it only added a second call site
(`AdaptiveChunkRunner::configure_for_serialized_input`) that reaches the same pre-existing buggy
function.

## The fix

Replaced both `(c as i64 - DESIRED_CHUNK_SIZE as i64).abs()` expressions with `c.abs_diff
(DESIRED_CHUNK_SIZE)` (`usize::abs_diff`, stable since Rust 1.60, available in `core` so it stays
`no_std`-compatible) - this computes the exact same "absolute difference from 1024" value the code
already wanted, entirely within `usize` arithmetic, with no narrowing cast and no possibility of
underflow: `usize::abs_diff` returns `|a - b|` computed without ever materializing a negative
intermediate value, unlike `(a as i64 - b as i64).abs()`. The resulting `diff`/`diff2` values are
used only in a `<` comparison between two `usize`s (`diff2 < diff`), which is exactly as
well-defined as before, just without the through-`i64` detour.

Verified against the primary repro (`total = 1usize << 63`, the exact left edge of the panic
window) both directly in the working tree and via `just test` from a fresh clone (`repro/` +
`patch.diff`, see `output.txt`): before the fix, `just reproduce-before` panics 101 within under a
second (4/4 direct runs in the working tree, plus the `just test` run itself); after `just apply`,
`just reproduce-after` shows no panic within a 5-second timeout - it instead keeps running
normally (as expected: correctly proceeding rather than crashing, since actually summing ~9.2e18
items is intentionally not waited out to completion).

Checked a second input in the same bug class (`CONFIRMATION.md` section 7), not just the exact
value that first triggered it: `total = (1usize << 63) + 500` (a different point inside the same
`[1usize << 63, (1usize << 63) + 1023]` panic window) - built and run (1/1 run each) against a
separate `git worktree` checkout of the same base commit: panics identically before the fix
(`attempt to subtract with overflow`, same line), and no longer panics within a 5s timeout after
applying the same patch (see `output.txt`).

- Project test suite (`cargo test --lib`): 99 passed / 0 failed before the fix -> 99 passed / 0
  failed after (identical - this change is purely additive safety on an arithmetic edge case none
  of the existing tests exercise).

## What's not done / open questions

- The `AdaptiveChunkRunner::configure_for_serialized_input` call site (wired up by b9214fbe) calls
  the exact same `heuristic::compute_chunk_size`/`auto_chunk_size` function, so it shares this bug
  and is fixed by the same patch - it was not separately exercised with its own dedicated repro
  this session (that path only runs when `is_source_serialized()` is true for the input's
  `ConcurrentIter` implementation, which needs a specific iterator-of-iterator/arbitrary-source
  shape rather than a plain `Range`; not investigated further since the shared root cause and
  shared fix make a second repro redundant for confirming the bug exists, though a future session
  could build one to double-check the call site is reached the way this analysis assumes).
- `ChunkSize::Min(min_chunk)` (`min_chunk_size`, line 47-53) calls the same buggy
  `auto_chunk_size` internally and is fixed by the same patch, but was not independently
  triggered with its own repro (`ChunkSize::Auto` and `ChunkSize::Min` share the exact same
  underlying arithmetic, so this is considered covered by the same root cause, not a separate
  finding).
- Unlike candidate 009's `total * PCT` overflow (a same-width `usize * usize` multiplication whose
  threshold shrinks proportionally on a 32-bit `usize`), this bug is specific to a 64-bit `usize`:
  the cast here is `usize as i64` (a *widening* cast on a 32-bit target, since every `u32` value
  fits in `i64` without truncation), so on `wasm32` (32-bit `usize`) `thread_load as i64` can never
  wrap and this exact panic window is unreachable - no further wasm32-specific check was needed or
  performed.
