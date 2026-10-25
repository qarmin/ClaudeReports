use tokio::runtime::Builder;

fn main() {
    let mut builder = Builder::new_multi_thread();
    let _ = builder.thread_stack_size(2147483648usize);
    let _ = builder.build();
}
