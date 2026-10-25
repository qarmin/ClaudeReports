use threadpool::ThreadPool;

fn main() {
    let _ = ThreadPool::new_with_name("\0".to_string(), 1usize);
}
