// Real end-to-end check of thread-count computation (issue #165's LOGIC_1: a `.min(1)` used
// where `.max(1)` was meant, silently forcing everything down to 1 thread).
//
// Uses the crate's real public API (`.par().num_threads(n).for_each(..)`) over a large enough
// input that thread count should never be artificially reduced to 1, and records every distinct
// `std::thread::current().id()` actually observed inside the closure body - not a hand-built
// stand-in for the thread pool's internal decision, the real decision as exercised by a real
// caller.

use orx_parallel::*;
use std::collections::HashSet;
use std::sync::Mutex;

fn distinct_threads_used<F: Fn(usize) + Sync>(n: usize, num_threads: usize, run: F) -> usize {
    let seen: Mutex<HashSet<std::thread::ThreadId>> = Mutex::new(HashSet::new());
    (0..n)
        .into_par()
        .num_threads(num_threads)
        .for_each(|i| {
            seen.lock().unwrap().insert(std::thread::current().id());
            run(i);
        });
    seen.into_inner().unwrap().len()
}

fn busy(i: usize) {
    // enough work per item that the runner doesn't just decide "not worth spawning"
    let mut acc: u64 = i as u64;
    for _ in 0..2000 {
        acc = acc.wrapping_mul(2654435761).wrapping_add(1);
    }
    std::hint::black_box(acc);
}

fn main() {
    let available = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    println!("available_parallelism = {available}");
    assert!(available > 1, "test host must have >1 cores to exercise this");

    let n = 2_000_000usize;

    // requested = min(available, 8) should be able to use up to 8 threads, never forced to 1.
    let requested = 8usize.min(available);
    let used = distinct_threads_used(n, requested, busy);
    println!("num_threads({requested}) on n={n} -> distinct threads observed = {used}");
    assert!(
        used > 1,
        "LOGIC_1-shaped bug reproduced: requested {requested} threads on a large input, but only \
         {used} distinct thread(s) were ever observed - looks like thread count was silently \
         clamped down to 1"
    );

    // NumThreads::Auto with a large input should also use more than 1 thread.
    let used_auto = distinct_threads_used(n, 0, busy); // 0.into() -> NumThreads::Auto
    println!("num_threads(Auto) on n={n} -> distinct threads observed = {used_auto}");
    assert!(
        used_auto > 1,
        "LOGIC_1-shaped bug reproduced under NumThreads::Auto: large input, but only \
         {used_auto} distinct thread(s) were ever observed"
    );

    println!("OK: thread-count computation not clamped to 1 (min/max not swapped)");
}
