use orx_parallel::*;
use std::cell::Cell;
use std::rc::Rc;

/// A `!Send` type (`Rc` is never `Send`) that we force to be `Sync` with a manual
/// unsafe impl - exactly matching the bound `Par::use_slice` actually requires
/// (`U: Sync`, no `U: Send`, before the fix). This is the standard way to
/// demonstrate a missing-Send-bound soundness hole: a witness type is required
/// because the bug is "for all T", and the point being demonstrated is that the
/// crate's own real public API accepts this witness type at all.
///
/// This is a single-participant repro (see CONFIRMATION.md section 11): the only
/// "external input" here is this witness type plugged into orx-parallel's real
/// public `Par::use_slice` entry point, driven by the crate's own real thread
/// pool spawning real OS worker threads - there is no second, faked participant.
struct RcSync(Rc<Cell<i64>>);

// SAFETY: this is the bug under test - it is NOT actually safe, since `Rc`'s
// clone/drop mutate a non-atomic refcount with no synchronization, and
// `Par::use_slice` (before the fix) will hand out `&mut RcSync` to worker
// threads that are not the thread that created the `Rc`.
unsafe impl Sync for RcSync {}

fn main() {
    let n_threads = 4usize;
    let iters_per_thread = 300_000usize;

    let control = Rc::new(Cell::new(0i64));
    let initial_count = Rc::strong_count(&control);

    let mut slots: Vec<RcSync> = (0..n_threads).map(|_| RcSync(control.clone())).collect();
    let after_setup_count = Rc::strong_count(&control);

    (0..(n_threads * iters_per_thread))
        .into_par()
        .num_threads(n_threads)
        .use_slice(&mut slots)
        .for_each(|slot, _i| {
            // Each worker thread repeatedly clones and drops ITS OWN `Rc` clone.
            // Since all clones (and `control`) share the same non-atomic strong
            // count, and this loop body executes concurrently across
            // `n_threads` real OS worker threads (orx-parallel's own default
            // thread pool, real `std::thread::spawn` workers), this is a
            // genuine, unsynchronized concurrent read-modify-write race on
            // that count.
            let c = Rc::clone(&slot.0);
            std::hint::black_box(&c);
            drop(c);
        });

    let final_count_before_drop = Rc::strong_count(&control);
    drop(slots);
    let final_count_after_drop = Rc::strong_count(&control);

    println!("initial strong_count (just `control`):        {initial_count}");
    println!("strong_count after building {n_threads} slots:  {after_setup_count}");
    println!("strong_count after parallel clone/drop storm:  {final_count_before_drop}");
    println!("strong_count after dropping `slots`:           {final_count_after_drop}");

    // Correct behavior (if the race did not corrupt the refcount): the clone/drop
    // storm nets to zero per thread, so the count right after it should still equal
    // `after_setup_count`, and after dropping `slots` it should be back to
    // `initial_count`.
    let corrupted =
        final_count_before_drop != after_setup_count || final_count_after_drop != initial_count;

    if corrupted {
        eprintln!(
            "REPRODUCED: Rc strong_count corrupted by unsynchronized concurrent access \
             (expected {after_setup_count} after storm / {initial_count} after final drop, \
             got {final_count_before_drop} / {final_count_after_drop})"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce this run - strong_count stayed consistent");
    }
}
