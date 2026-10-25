use tokio::runtime::Builder;

fn main() {
    let mut builder = Builder::new_multi_thread();
    let _ = builder.worker_threads(usize::MAX);
    let _ = builder.build();
}
