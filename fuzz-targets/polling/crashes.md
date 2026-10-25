# polling - crashes

Source: https://github.com/smol-rs/polling @ 35ba7a4fa8bf1e9f33711cb140a015d6bfe84c1b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Events::with_capacity(std::num::NonZero::new(1000000000usize).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 12000000000 bytes failed
backtrace:
  polling::epoll::Events::with_capacity (src/epoll.rs:336)
  polling::Events::with_capacity (polling/src/lib.rs:874)
  repro_out_of_memory_in_polling_events_with_capacity_fa98af::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Events::with_capacity(std::num::NonZero::new(
    10000000000000000000usize).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from polling/src/epoll.rs:336:19)
capacity overflow
backtrace:
  &lt;polling::epoll::Events&gt;::with_capacity (polling/src/epoll.rs:336:19)
  &lt;polling::Events&gt;::with_capacity (polling/src/lib.rs:874:21)
  repro_polling_src_epoll_rs_336_19_b2ef7e::main (src/main.rs:4:13)</pre></td></tr>
</table>
