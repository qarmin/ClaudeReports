# bytemuck - crashes

Source: https://github.com/Lokathor/bytemuck @ ea3616110b0134e9991fdb66be8892dbc427c5ac

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_rc_slice::&lt;char&gt;(
    10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at alloc/src/rc.rs:2326:41:
called `Result::unwrap()` on an `Err` value: LayoutError
backtrace:
  bytemuck::allocation::zeroed_rc_slice::&lt;char&gt; (bytemuck/src/allocation.rs:179:16)
  repro_bytemuck_src_allocation_rs_179_16_a69375::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_vec::&lt;bool&gt;(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at bytemuck/src/allocation.rs:110:26:
called `Result::unwrap()` on an `Err` value: ()
backtrace:
  bytemuck::allocation::zeroed_vec::&lt;bool&gt; (bytemuck/src/allocation.rs:110:26)
  repro_bytemuck_src_allocation_rs_110_26_d627a8::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_slice_box::&lt;bool&gt;(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at bytemuck/src/allocation.rs:149:32:
called `Result::unwrap()` on an `Err` value: ()
backtrace:
  bytemuck::allocation::zeroed_slice_box::&lt;bool&gt; (bytemuck/src/allocation.rs:149:32)
  repro_bytemuck_src_allocation_rs_149_32_f50638::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_arc_slice::&lt;u8&gt;(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at alloc/src/sync.rs:2277:41:
called `Result::unwrap()` on an `Err` value: LayoutError
backtrace:
  bytemuck::allocation::zeroed_arc_slice::&lt;u8&gt; (bytemuck/src/allocation.rs:163:17)
  repro_bytemuck_src_allocation_rs_163_17_c9852e::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_arc_slice::&lt;u8&gt;(2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  bytemuck::allocation::zeroed_arc_slice&lt;u8&gt; (bytemuck/src/allocation.rs:163)
  repro_out_of_memory_in_bytemuck_allocation_zeroed_arc_slice_u8_d98164::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_arc_slice::&lt;char&gt;(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000016 bytes failed
backtrace:
  bytemuck::allocation::zeroed_arc_slice&lt;char&gt; (bytemuck/src/allocation.rs:163)
  repro_out_of_memory_in_bytemuck_allocation_zeroed_arc_slice_char_9aae50::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_arc_slice::&lt;bool&gt;(
    1000000000000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000016 bytes failed
backtrace:
  bytemuck::allocation::zeroed_arc_slice&lt;bool&gt; (bytemuck/src/allocation.rs:163)
  repro_out_of_memory_in_bytemuck_allocation_zeroed_arc_slice_bool_608293::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_rc_slice::&lt;bool&gt;(1000000000000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000016 bytes failed
backtrace:
  bytemuck::allocation::zeroed_rc_slice&lt;bool&gt; (bytemuck/src/allocation.rs:179)
  repro_out_of_memory_in_bytemuck_allocation_zeroed_rc_slice_bool_09f0ae::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_rc_slice::&lt;char&gt;(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000016 bytes failed
backtrace:
  bytemuck::allocation::zeroed_rc_slice&lt;char&gt; (bytemuck/src/allocation.rs:179)
  repro_out_of_memory_in_bytemuck_allocation_zeroed_rc_slice_char_b2acb6::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytemuck::allocation::zeroed_rc_slice::&lt;u8&gt;(4294967296usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967312 bytes failed
backtrace:
  bytemuck::allocation::zeroed_rc_slice&lt;u8&gt; (bytemuck/src/allocation.rs:179)
  repro_out_of_memory_in_bytemuck_allocation_zeroed_rc_slice_u8_58a77f::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
</table>
