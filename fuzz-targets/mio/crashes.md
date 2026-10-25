# mio - crashes

Source: https://github.com/tokio-rs/mio @ a88e5e119172e4fb1638926faf65a4fe889e05b1

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Events::with_capacity(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  mio::event::events::Events::with_capacity (src/event/events.rs:94)
  repro_out_of_memory_in_mio_events_with_capacity_67886b::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Events::with_capacity(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from mio/src/event/events.rs:94:20)
capacity overflow
backtrace:
  &lt;mio::event::events::Events&gt;::with_capacity (mio/src/event/events.rs:94:20)
  repro_mio_src_event_events_rs_94_20_c7aee4::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let events = Events::with_capacity(1000000000usize);
let _ = events.iter();</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  mio::event::events::Events::with_capacity (src/event/events.rs:94)
  repro_out_of_memory_in_mio_events_iter_2ed5c0::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
</table>
