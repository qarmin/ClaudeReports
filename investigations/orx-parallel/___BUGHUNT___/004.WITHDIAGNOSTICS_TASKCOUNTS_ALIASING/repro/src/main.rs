// Repro for the raw-pointer aliasing UB in WithDiagnostics's TaskCounts::push /
// ThreadLifetimes::begin/end (src/runner/runner_variants/with_diagnostics.rs).
//
// `std::thread::available_parallelism()` reports 1 under Miri regardless of the real host, and
// orx-parallel's built-in pools always clamp the requested thread count against it - so under
// Miri the default/`Pool::once`/`Pool::basic` pools can never actually run more than a single
// worker thread, which would hide this bug (no concurrent, disjoint-index pushes to exercise the
// aliasing violation across threads). `ParThreadPool` is the crate's own public extension point
// (documented in `docs/thread_usage.md`: "the trait is small and straightforward to implement, so
// you can plug in any executor") - this fixed-thread-count pool is a legitimate real use of that
// public API to force N genuine `std::thread::scope` worker threads under Miri, not a stand-in
// for anything the library itself wouldn't do.
use orx_parallel::*;
use std::num::NonZeroUsize;

struct FixedPool(NonZeroUsize);

impl ParThreadPool for FixedPool {
    type ScopeRef<'s, 'env, 'scope>
        = &'s std::thread::Scope<'s, 'env>
    where
        'scope: 's,
        'env: 'scope + 's;

    fn max_num_threads(&self) -> NonZeroUsize {
        self.0
    }

    fn scoped_computation<'env, 'scope, F>(&'env mut self, f: F)
    where
        'env: 'scope,
        for<'s> F: FnOnce(&'s std::thread::Scope<'s, 'env>) + Send,
    {
        std::thread::scope(f)
    }

    fn run_in_scope<'s, 'env, 'scope, W>(s: &Self::ScopeRef<'s, 'env, 'scope>, work: W)
    where
        'scope: 's,
        'env: 'scope + 's,
        W: Fn() + Send + 'scope + 'env,
    {
        s.spawn(work);
    }
}

fn main() {
    let num_threads = 4;
    let pool = FixedPool(NonZeroUsize::new(num_threads).unwrap());

    let total: usize = (0..400usize)
        .par()
        .runner(Runner::fixed_with_pool(pool))
        .num_threads(num_threads)
        .chunk_size(4)
        .runner_with_diagnostics()
        .map(|x| x)
        .reduce(|a, b| a + b)
        .unwrap_or(0);

    println!("total = {total}");
}
