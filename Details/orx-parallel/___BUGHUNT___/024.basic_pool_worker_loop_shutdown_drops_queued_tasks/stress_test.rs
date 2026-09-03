// Bug-hunt candidate 024: attempt to reproduce a hang caused by BasicPool's Inner::drop
// discarding queued-but-not-yet-run tasks without ever calling complete_task() on their
// ScopeRuntime, which would make some concurrent scoped_computation's wait_for_completion()
// block forever.
//
// This drives the real public API end to end (Runner::fixed_with_pool + Par::for_each), the
// same entry point a real caller would use, rather than poking BasicPool's internals directly.
// It stresses concurrent creation/use/drop of BasicPool clones, hammering exactly the
// interleaving the candidate worried about: one clone being dropped by one thread while another
// clone (sharing the same Arc<Inner>) is in the middle of an active parallel computation with
// tasks still in flight.
//
// The whole stress loop runs on a watchdog thread with a bounded wait; if the real bug were
// reachable, this test would hang instead of completing.

use orx_parallel::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

fn run_with_timeout<F: FnOnce() + Send + 'static>(f: F, timeout: Duration) -> bool {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        f();
        let _ = tx.send(());
    });
    rx.recv_timeout(timeout).is_ok()
}

#[test]
fn stress_concurrent_clone_drop_during_active_scope() {
    let completed = run_with_timeout(
        || {
            for iter in 0..500 {
                // A single worker thread with many chunks widens the window during which the
                // queue is non-empty, maximizing the chance a concurrent drop interleaves with
                // it if that were ever actually possible.
                let pool = Pool::basic(1);
                let counter = AtomicUsize::new(0);

                let droppers: Vec<_> = (0..4)
                    .map(|_| {
                        let dropper_pool = pool.clone();
                        std::thread::spawn(move || {
                            // Drop extra clones concurrently while the main clone below is
                            // actively running a computation with in-flight tasks.
                            drop(dropper_pool);
                        })
                    })
                    .collect();

                (0..256)
                    .into_par()
                    .runner(Runner::fixed_with_pool(pool))
                    .num_threads(8)
                    .for_each(|_| {
                        std::thread::sleep(Duration::from_micros(50));
                        counter.fetch_add(1, Ordering::SeqCst);
                    });

                for d in droppers {
                    d.join().unwrap();
                }
                assert_eq!(counter.load(Ordering::SeqCst), 256, "iteration {iter} lost tasks");
            }
        },
        Duration::from_secs(60),
    );
    assert!(completed, "stress test hung - possible task-drop-without-run bug");
}
