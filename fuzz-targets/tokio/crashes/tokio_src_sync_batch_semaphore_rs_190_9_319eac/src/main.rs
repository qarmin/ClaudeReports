use tokio::sync::Semaphore;

fn main() {
    let _ = Semaphore::const_new(usize::MAX);
}
