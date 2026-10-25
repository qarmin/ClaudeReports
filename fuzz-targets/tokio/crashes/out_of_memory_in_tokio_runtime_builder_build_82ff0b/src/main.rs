use tokio::runtime::Builder;

fn main() {
    let mut builder = Builder::new_multi_thread();
    let _ = builder.worker_threads(1000000000usize);
    let _ = builder.build();
}
