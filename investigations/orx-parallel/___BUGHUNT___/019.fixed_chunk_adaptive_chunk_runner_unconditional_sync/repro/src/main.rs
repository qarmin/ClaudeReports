use core::cell::Cell;
use core::num::NonZeroUsize;
use orx_parallel::{ParThreadPool, Runner};
use std::rc::Rc;

/// A witness thread pool that is deliberately `!Send + !Sync` (it owns an `Rc`,
/// whose non-atomic refcount is unsound to touch from more than one real OS thread
/// at a time without external synchronization).
struct RcPool {
    rc: Rc<Cell<i64>>,
}

impl ParThreadPool for RcPool {
    type ScopeRef<'s, 'env, 'scope>
        = ()
    where
        'scope: 's,
        'env: 'scope + 's;

    fn run_in_scope<'s, 'env, 'scope, W>(_s: &Self::ScopeRef<'s, 'env, 'scope>, work: W)
    where
        'scope: 's,
        'env: 'scope + 's,
        W: Fn() + Send + 'scope + 'env,
    {
        work();
    }

    fn scoped_computation<'env, 'scope, F>(&'env mut self, f: F)
    where
        'env: 'scope,
        for<'s> F: FnOnce(()) + Send,
    {
        f(());
    }

    fn max_num_threads(&self) -> NonZeroUsize {
        NonZeroUsize::new(1).unwrap()
    }
}

fn main() {
    let pool = RcPool {
        rc: Rc::new(Cell::new(0)),
    };

    // (1) Runner::fixed_with_pool is the crate's only public constructor that plugs
    // an arbitrary caller-supplied ParThreadPool into FixedChunkRunner<P>.
    let runner = Runner::fixed_with_pool(pool);

    // Attempting `let _ = runner.pool;` here fails to compile with
    // `error[E0616]: field \`pool\` of struct \`FixedChunkRunner\` is private` -
    // confirming the `pool: P` field has no public accessor either (see
    // output.txt in the candidate folder for the full compiler output).

    // (2) Static assertion: does the crate grant FixedChunkRunner<RcPool>: Sync
    // even though RcPool contains a non-atomic Rc (RcPool: !Sync, !Send)?
    fn assert_sync<T: Sync>(_: &T) {}
    assert_sync(&runner);
    println!(
        "FixedChunkRunner<RcPool> is Sync (confirms the suspected unconditional impl) \
         even though RcPool is !Sync/!Send."
    );

    // (3) Try to actually reach the inner `RcPool`/its `Rc` from a second real OS
    // thread using only the crate's PUBLIC API (no transmute/unsafe on our side).
    // `ParRunner` (the trait with `pool()`/`pool_mut()`) is private to the crate,
    // and `FixedChunkRunner` exposes no other public accessor to its `pool: P`
    // field - so there is no public API call reachable from a second thread.
    std::thread::scope(|s| {
        s.spawn(|| {
            let r = &runner; // only possible because FixedChunkRunner<RcPool>: Sync
            // Nothing publicly reachable on `r` exposes the inner RcPool/Rc.
            // (No `.pool()` method, no field access - both are compile errors,
            // see output.txt for the actual compiler errors from probing each.)
            let _ = r as *const _; // only pointer-level operations are available
            println!(
                "second real OS thread holds &FixedChunkRunner<RcPool> (shared \
                 only because of the Sync impl), but no public API call can reach \
                 the RcPool/Rc inside it"
            );
        });
    });

    println!("done - no public-API-reachable unsound operation was found");
}

// Attempting `let _: &dyn orx_parallel::ParRunner;` here fails to compile with
// `error[E0405]: cannot find trait \`ParRunner\` in crate \`orx_parallel\`` -
// confirming the trait that owns `pool()`/`pool_mut()` is not part of the public
// API at all (see output.txt in the candidate folder for the full compiler output).

#[test]
fn adaptive_chunk_runner_also_grants_unconditional_sync() {
    let pool = RcPool {
        rc: Rc::new(Cell::new(0)),
    };
    let runner = Runner::adaptive_with_pool(pool);
    fn assert_sync<T: Sync>(_: &T) {}
    assert_sync(&runner);
    std::thread::scope(|s| {
        s.spawn(|| {
            let _r = &runner;
        });
    });
}
