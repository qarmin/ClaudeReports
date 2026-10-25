fn main() {
    let _ = <thread_local::ThreadLocal<String>>::with_capacity(10000000000000000000usize);
}
