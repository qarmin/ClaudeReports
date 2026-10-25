# async-lock - crashes

Source: https://github.com/smol-rs/async-lock @ 23e181d5cfb91a59382e5719c682caa58ff43ace

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let semaphore_guard_arc = std::sync::Arc::new(Semaphore::new(
    4999147007719037987usize)).acquire_arc_blocking();
semaphore_guard_arc.forget();
let barrier = Barrier::new(usize::MAX);
let _ = barrier.wait_blocking();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  parking::Inner::park (src/lib.rs:377)
  parking::Parker::park (src/lib.rs:132)
  event_listener::InnerListener&lt;(), alloc::sync::Arc&lt;event_listener::Inner&lt;()&gt;, alloc::alloc::Global&gt;&gt;::wait_with_parker&lt;(), alloc::sync::Arc&lt;event_listener::Inner&lt;()&gt;, alloc::alloc::Global&gt;&gt; (event-listener/src/lib.rs:1143)
  event_listener::{impl#19}::wait_internal::{closure#0}&lt;(), alloc::sync::Arc&lt;event_listener::Inner&lt;()&gt;, alloc::alloc::Global&gt;&gt; (event-listener/src/lib.rs:1112)
  event_listener::InnerListener&lt;(), alloc::sync::Arc&lt;event_listener::Inner&lt;()&gt;, alloc::alloc::Global&gt;&gt;::wait_internal&lt;(), alloc::sync::Arc&lt;event_listener::Inner&lt;()&gt;, alloc::alloc::Global&gt;&gt; (event-listener/src/lib.rs:1110)
  event_listener::{impl#26}::wait&lt;()&gt; (event-listener/src/lib.rs:862)
  event_listener_strategy::{impl#5}::poll&lt;(), event_listener::EventListener&lt;()&gt;&gt; (event-listener-strategy/src/lib.rs:514)
  async_lock::barrier::{impl#2}::poll_with_strategy&lt;event_listener_strategy::Blocking&gt; (src/barrier.rs:219)
  event_listener_strategy::EventListenerFuture::wait&lt;async_lock::barrier::BarrierWaitInner&gt; (event-listener-strategy/src/lib.rs:315)
  async_lock::barrier::BarrierWait::wait (event-listener-strategy/src/lib.rs:224)
  async_lock::barrier::Barrier::wait_blocking (src/barrier.rs:134)
  repro_timeout_in_async_lock_barrier_wait_blocking_4d5407::main (src/main.rs:8)
  (6 std/libc frames omitted)</pre></td></tr>
</table>
