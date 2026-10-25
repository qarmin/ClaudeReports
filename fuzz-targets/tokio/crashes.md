# tokio - crashes

Source: https://github.com/tokio-rs/tokio @ cdd9dae3aa5056e23dc4d5bc4f852ab8fbf27aff

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Semaphore::const_new(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at tokio/src/sync/batch_semaphore.rs:190:9:
assertion failed: permits &lt;= Self::MAX_PERMITS
backtrace:
  &lt;tokio::sync::batch_semaphore::Semaphore&gt;::const_new (tokio/src/sync/batch_semaphore.rs:190:9)
  &lt;tokio::sync::semaphore::Semaphore&gt;::const_new (tokio/src/sync/semaphore.rs:535:21)
  repro_tokio_src_sync_batch_semaphore_rs_190_9_319eac::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new_multi_thread();
let _ = builder.worker_threads(1000000000usize);
let _ = builder.build();</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  tokio::runtime::scheduler::multi_thread::worker::create (src/runtime/scheduler/multi_thread/worker.rs:283)
  tokio::runtime::scheduler::multi_thread::MultiThread::new (src/runtime/scheduler/multi_thread/mod.rs:67)
  tokio::runtime::builder::Builder::build_threaded_runtime (src/runtime/builder.rs:2202)
  tokio::runtime::builder::Builder::build (src/runtime/builder.rs:1154)
  repro_out_of_memory_in_tokio_runtime_builder_build_82ff0b::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new_multi_thread();
let _ = builder.start_paused(true);
let _ = builder.build();</pre></td><td valign="top"><pre>thread 'main' panicked at tokio/src/time/clock.rs:315:21:
`time::pause()` requires the `current_thread` Tokio runtime. This is the default Runtime used by `#[tokio::test].
backtrace:
  &lt;tokio::time::clock::Clock&gt;::new (tokio/src/time/clock.rs:315:21)
  tokio::runtime::driver::create_clock (tokio/src/runtime/driver.rs:316:9)
  &lt;tokio::runtime::driver::Driver&gt;::new (tokio/src/runtime/driver.rs:52:21)
  &lt;tokio::runtime::builder::Builder&gt;::build_threaded_runtime (tokio/src/runtime/builder.rs:2191:43)
  &lt;tokio::runtime::builder::Builder&gt;::build (tokio/src/runtime/builder.rs:1154:39)
  repro_tokio_src_time_clock_rs_315_21_e6cc0c::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new_multi_thread();
let _ = builder.worker_threads(10000000000000000000usize);
let _ = builder.build();</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from tokio/src/runtime/scheduler/multi_thread/worker.rs:283:21)
capacity overflow
backtrace:
  tokio::runtime::scheduler::multi_thread::worker::create (tokio/src/runtime/scheduler/multi_thread/worker.rs:283:21)
  &lt;tokio::runtime::scheduler::multi_thread::MultiThread&gt;::new (tokio/src/runtime/scheduler/multi_thread/mod.rs:67:32)
  &lt;tokio::runtime::builder::Builder&gt;::build_threaded_runtime (tokio/src/runtime/builder.rs:2202:47)
  &lt;tokio::runtime::builder::Builder&gt;::build (tokio/src/runtime/builder.rs:1154:39)
  repro_tokio_src_runtime_scheduler_multi_thread_worker_rs_283_21_bf7196::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = UnixStream::pair();</pre></td><td valign="top"><pre>thread 'main' panicked at tokio/src/net/unix/stream.rs:936:18:
there is no reactor running, must be called from the context of a Tokio 1.x runtime
backtrace:
  &lt;tokio::runtime::scheduler::Handle&gt;::current (tokio/src/runtime/scheduler/mod.rs:92:27)
  &lt;tokio::io::poll_evented::PollEvented&lt;mio::net::uds::stream::UnixStream&gt;&gt;::new_with_interest (tokio/src/io/poll_evented.rs:111:58)
  &lt;tokio::io::poll_evented::PollEvented&lt;mio::net::uds::stream::UnixStream&gt;&gt;::new (tokio/src/io/poll_evented.rs:90:9)
  &lt;tokio::net::unix::stream::UnixStream&gt;::new (tokio/src/net/unix/stream.rs:936:18)
  &lt;tokio::net::unix::stream::UnixStream&gt;::pair (tokio/src/net/unix/stream.rs:915:17)
  repro_tokio_src_net_unix_stream_rs_936_18_33474f::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new_multi_thread();
let _ = builder.worker_threads(usize::MAX);
let _ = builder.build();</pre></td><td valign="top"><pre>thread 'main' panicked at tokio/src/runtime/builder.rs:2195:54:
attempt to add with overflow
backtrace:
  &lt;tokio::runtime::builder::Builder&gt;::build_threaded_runtime (tokio/src/runtime/builder.rs:2195:54)
  &lt;tokio::runtime::builder::Builder&gt;::build (tokio/src/runtime/builder.rs:1154:39)
  repro_tokio_src_runtime_builder_rs_2195_54_ce06f3::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new_multi_thread();
let _ = builder.thread_stack_size(2147483648usize);
let _ = builder.build();</pre></td><td valign="top"><pre>thread 'main' panicked at tokio/src/runtime/scheduler/multi_thread/worker.rs:515:13:
OS can't spawn worker thread: Resource temporarily unavailable (os error 11)
backtrace:
  &lt;tokio::runtime::blocking::pool::Spawner&gt;::spawn_blocking::&lt;&lt;tokio::runtime::scheduler::multi_thread::worker::Launch&gt;::launch::{closure#0}, ()&gt; (tokio/src/runtime/blocking/pool.rs:386:17)
  &lt;tokio::runtime (tokio/src/runtime/handle.rs:239:39)
  tokio::runtime::blocking::pool::spawn_blocking::&lt;&lt;tokio::runtime::scheduler::multi_thread::worker::Launch&gt;::launch::{closure#0}, ()&gt; (tokio/src/runtime/blocking/pool.rs:244:8)
  &lt;tokio::runtime::scheduler::multi_thread::worker::Launch&gt;::launch (tokio/src/runtime/scheduler/multi_thread/worker.rs:515:13)
  &lt;tokio::runtime::builder::Builder&gt;::build_threaded_runtime (tokio/src/runtime/builder.rs:2236:20)
  &lt;tokio::runtime::builder::Builder&gt;::build (tokio/src/runtime/builder.rs:1154:39)
  repro_tokio_src_runtime_scheduler_multi_thread_worker_rs_515_13_557bad::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new_multi_thread();
let _ = builder.thread_name("\0".to_string());
let _ = builder.build();</pre></td><td valign="top"><pre>thread 'main' panicked at library/std/src/thread/thread.rs:24:40 (called from tokio/src/runtime/blocking/pool.rs:522:17)
thread name may not contain interior null bytes: NulError(0, [0])
backtrace:
  &lt;tokio::runtime::blocking::pool::Spawner&gt;::spawn_thread (tokio/src/runtime/blocking/pool.rs:522:17)
  &lt;tokio::runtime::blocking::pool::Spawner&gt;::spawn_task::{closure#0} (tokio/src/runtime/blocking/pool.rs:482:32)
  &lt;tokio::runtime::blocking::pool::LockedImpl&gt;::spawn_task::&lt;&lt;tokio::runtime::blocking::pool::Spawner&gt;::spawn_task::{closure#0}&gt; (tokio/src/runtime/blocking/pool.rs:628:13)
  &lt;tokio::runtime::blocking::pool::InnerImpl&gt;::spawn_task::&lt;&lt;tokio::runtime::blocking::pool::Spawner&gt;::spawn_task::{closure#0}&gt; (tokio/src/runtime/blocking/pool.rs:559:39)
  &lt;tokio::runtime::blocking::pool::Spawner&gt;::spawn_task (tokio/src/runtime/blocking/pool.rs:468:14)
  &lt;tokio::runtime::blocking::pool::Spawner&gt;::spawn_blocking_inner::&lt;&lt;tokio::runtime::scheduler::multi_thread::worker::Launch&gt;::launch::{closure#0}, ()&gt; (tokio/src/runtime/blocking/pool.rs:451:28)
  &lt;tokio::runtime::blocking::pool::Spawner&gt;::spawn_blocking::&lt;&lt;tokio::runtime::scheduler::multi_thread::worker::Launch&gt;::launch::{closure#0}, ()&gt; (tokio/src/runtime/blocking/pool.rs:373:18)
  &lt;tokio::runtime (tokio/src/runtime/handle.rs:239:39)
  tokio::runtime::blocking::pool::spawn_blocking::&lt;&lt;tokio::runtime::scheduler::multi_thread::worker::Launch&gt;::launch::{closure#0}, ()&gt; (tokio/src/runtime/blocking/pool.rs:244:8)
  &lt;tokio::runtime::scheduler::multi_thread::worker::Launch&gt;::launch (tokio/src/runtime/scheduler/multi_thread/worker.rs:515:13)
  &lt;tokio::runtime::builder::Builder&gt;::build_threaded_runtime (tokio/src/runtime/builder.rs:2236:20)
  &lt;tokio::runtime::builder::Builder&gt;::build (tokio/src/runtime/builder.rs:1154:39)</pre></td></tr>
</table>
