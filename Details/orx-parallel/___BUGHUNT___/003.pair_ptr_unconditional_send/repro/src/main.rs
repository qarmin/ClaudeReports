// Reproduction for candidate 003: `unsafe impl<U, V> Send for PairPtr<U, V> {}`
// (src/use_var/pair_ptr.rs) grants `Send` to `PairPtr<U, V>` regardless of whether
// `U`/`V` are actually `Send`.
//
// This drives the crate's real, public API end-to-end: `Par::use_slice(...)` (the one
// public entry point whose bound is `U: Sync`, not `U: Send` - see the separate,
// already-confirmed candidate 002 about `UseSlice`'s own unconditional `Sync` impl,
// which is what lets a `Sync + !Send` element type into the pipeline in the first
// place) piped into `.fold()`, which is implemented via `UseFold`
// (src/use_var/use_fold.rs) wrapping the per-thread state pointer in a `PairPtr` and
// storing it into an `orx_concurrent_ordered_bag::ConcurrentOrderedBag`, whose own
// `set_value` has an explicit `where T: Send` safety bound - a real safety contract
// from a dependency this crate relies on. `PairPtr`'s unconditional blanket `Send` impl
// is exactly what lets a `!Send` `Use::Item` satisfy that bound anyway.
//
// `GuardLike` below models a realistic "Sync-but-!Send" type the same way
// `std::sync::MutexGuard` is: `Sync` (if the pointee is `Sync`) but explicitly `!Send`,
// because transferring exclusive access/ownership of it to a different OS thread than
// the one that is relying on it is exactly the operation `Send` exists to gate (for a
// real `MutexGuard`, unlocking on the wrong thread is documented undefined behavior on
// several platforms; for a real-world analog like a thread-affine FFI handle or a
// thread-local-arena-backed allocation, the same rationale applies).
//
// Exit code convention: 0 = the exploit ran to completion (bug present under the
// unpatched crate); a compile error (see `just reproduce-after`) is the *fixed*
// outcome for this specific bug class, since the correct fix is for the type system to
// reject this program outright, not for it to run differently at runtime.

use orx_parallel::*;
use std::cell::Cell;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::thread::{self, ThreadId};

struct GuardLike {
    last_thread: Cell<Option<ThreadId>>,
    val: Cell<i32>,
    // Blocks the auto-derived `Send`/`Sync` (raw pointers are neither); we then grant
    // `Sync` back explicitly below, exactly as `MutexGuard` does, while deliberately
    // never granting `Send`.
    _not_send: PhantomData<*const ()>,
}

unsafe impl Sync for GuardLike {}

impl GuardLike {
    fn new() -> Self {
        Self {
            last_thread: Cell::new(None),
            val: Cell::new(0),
            _not_send: PhantomData,
        }
    }

    fn bump(&self) {
        self.last_thread.set(Some(thread::current().id()));
        self.val.set(self.val.get() + 1);
    }
}

fn main() {
    let num_threads = 8usize;
    let n = 4096usize;

    let mut states: Vec<GuardLike> = (0..num_threads).map(|_| GuardLike::new()).collect();

    // `GuardLike` is Sync but NOT Send. `Par::use_slice`'s bound (`U: Sync`) lets it
    // through; `.fold()`'s `UseFold`/`PairPtr` machinery then hands each worker's
    // `&GuardLike` across the real thread-pool boundary with no `Send` bound ever
    // checked by the compiler, because `PairPtr<U::Item, T>: Send` holds unconditionally.
    let partials: Vec<usize> = (0..n)
        .into_par()
        .num_threads(num_threads)
        .use_slice(&mut states)
        .fold(
            || 0usize,
            |guard, acc, _x| {
                guard.bump();
                *acc += 1;
            },
        );

    assert_eq!(partials.iter().sum::<usize>(), n);

    let threads_seen: HashSet<_> = states.iter().map(|g| g.last_thread.get()).collect();

    println!("REPRODUCED: orx_parallel::ParUse::fold() compiled and ran to completion");
    println!("with a Sync-but-!Send Use::Item (GuardLike), via the real public");
    println!("Par::use_slice(...).fold(...) API, across {} real OS worker threads", num_threads);
    println!("(observed {} distinct OS ThreadIds exclusively mutating GuardLike slots).", threads_seen.len());
    println!(
        "This program should not even compile: `.fold()`'s `Self::Use` (here `GuardLike`) \
         is not Send, and per-thread state passed across a real OS thread boundary must be."
    );
}
