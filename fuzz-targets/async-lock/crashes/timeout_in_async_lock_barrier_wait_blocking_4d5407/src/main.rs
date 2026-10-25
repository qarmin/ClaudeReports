use async_lock::Barrier;
use async_lock::Semaphore;

fn main() {
    let semaphore_guard_arc = std::sync::Arc::new(Semaphore::new(4999147007719037987usize)).acquire_arc_blocking();
    semaphore_guard_arc.forget();
    let barrier = Barrier::new(usize::MAX);
    let _ = barrier.wait_blocking();
}
