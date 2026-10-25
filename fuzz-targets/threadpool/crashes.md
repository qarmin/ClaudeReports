# threadpool - crashes

Source: https://github.com/rust-threadpool/rust-threadpool @ a9dd15c278a8ab07d694efb169348138823bb89d

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut thread_pool = ThreadPool::default();
thread_pool.set_threads(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-threadpool/src/lib.rs:571:9:
assertion failed: num_threads &gt;= 1
backtrace:
  &lt;threadpool::ThreadPool&gt;::set_num_threads (rust-threadpool/src/lib.rs:571:9)
  &lt;threadpool::ThreadPool&gt;::set_threads (rust-threadpool/src/lib.rs:527:14)
  repro_rust_threadpool_src_lib_rs_571_9_69cf38::main (src/main.rs:5:17)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ThreadPool::new_with_name("\0".to_string(), 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/std/src/thread/thread.rs:24:40 (called from rust-threadpool/src/lib.rs:739:10)
thread name may not contain interior null bytes: NulError(0, [0])
backtrace:
  threadpool::spawn_in_pool (rust-threadpool/src/lib.rs:739:10)
  &lt;threadpool::Builder&gt;::build (rust-threadpool/src/lib.rs:301:13)
  &lt;threadpool::ThreadPool&gt;::with_name (rust-threadpool/src/lib.rs:401:14)
  &lt;threadpool::ThreadPool&gt;::new_with_name (rust-threadpool/src/lib.rs:408:9)
  repro_rust_threadpool_src_lib_rs_739_10_6040ca::main (src/main.rs:4:13)</pre></td></tr>
</table>
