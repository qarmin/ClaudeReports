// Standalone repro for candidate 012: `auto_chunk_size`
// (src/runner/runner_variants/fixed_chunk/heuristic.rs) casts a `usize` `thread_load` to `i64`
// and subtracts `DESIRED_CHUNK_SIZE` (1024) from it without any overflow protection, unlike the
// rest of the crate's arithmetic on untrusted sizes. For `thread_load` in the narrow window
// `[1usize << 63, (1usize << 63) + 1023]`, `thread_load as i64` wraps to (or near) `i64::MIN`,
// and subtracting 1024 panics with "attempt to subtract with overflow" under a debug-assertions
// build.
//
// This goes through the crate's real public entry points: a real `std::ops::Range<usize>` (a
// genuine `ExactSizeIterator` reporting a huge size_hint without needing to materialize any
// actual elements - no allocation required to trigger this), `Runner::fixed_with_pool` (the
// crate's own public "fixed chunk" runner selector) with `Pool::basic(1)` (a real, public,
// single-worker thread pool) so `max_num_threads_for_computation` resolves to exactly 1 without
// going through the `NumThreads::Max(1)`-is-sequential fast path that would otherwise skip the
// buggy heuristic entirely (see RESULT.md's "What was tried" for why that matters).
use orx_parallel::*;

fn main() {
    // 1usize << 63: the exact left edge of the panic window for `c as i64 - 1024`.
    // A plain Range costs O(1) memory regardless of this value - no allocation needed.
    let total: usize = 1usize << 63;
    println!(
        "starting .par().sum() over a range of {total} items (fixed chunk runner, 1-thread pool)..."
    );
    let par = (0usize..total)
        .par()
        .runner(Runner::fixed_with_pool(Pool::basic(1)));
    let sum: usize = par.sum();
    println!("sum = {sum}");
}
