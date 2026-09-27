// Standalone repro for candidate 009: State::should_stop_exploration's non-saturating
// `total * EXPLORATION_CAP_PCT` (src/runner/runner_variants/adaptive_chunk/state.rs:108)
// panics with "attempt to multiply with overflow" under a debug-assertions build, for any
// input whose exact size (`total`, taken from the iterator's ExactSizeIterator upper bound)
// exceeds `usize::MAX / 10`.
//
// This goes through the crate's real public entry point: `Range<usize>::par()` (the
// default runner is `AdaptiveChunkRunner`, exercised via ordinary `.par().sum()`), driven by
// a `std::ops::Range`, which is a genuine ExactSizeIterator that reports a huge size_hint
// without needing to materialize any actual elements - so this is not a fabricated internal
// state, it is exactly what a real caller handing a large-but-lazy range to `.par()` produces.
use orx_parallel::*;

fn main() {
    // usize::MAX / 10 + 2: just above the overflow boundary for `total * 10` on 64-bit usize.
    // A plain Range costs O(1) memory regardless of this value - no allocation needed.
    let total: usize = usize::MAX / 10 + 2;
    println!("starting .par().sum() over a range of {total} items (should panic near-instantly if the bug is present, or hang/run for a long time if fixed)...");
    let sum: usize = (0usize..total).par().sum();
    println!("sum = {sum}");
}
