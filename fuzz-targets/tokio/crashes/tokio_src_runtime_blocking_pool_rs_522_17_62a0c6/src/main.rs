use tokio::runtime::Builder;

fn main() {
    let mut builder = Builder::new_multi_thread();
    let _ = builder.thread_name("\0".to_string());
    let _ = builder.build();
}
