use tokio::runtime::Builder;

fn main() {
    let mut builder = Builder::new_multi_thread();
    let _ = builder.start_paused(true);
    let _ = builder.build();
}
