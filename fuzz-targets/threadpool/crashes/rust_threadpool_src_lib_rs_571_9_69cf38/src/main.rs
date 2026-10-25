use threadpool::ThreadPool;

fn main() {
    let mut thread_pool = ThreadPool::default();
    thread_pool.set_threads(0usize);
}
