# harfrust - crashes

Source: https://github.com/harfbuzz/harfrust @ facb5efa7f301e2a27cadb3a43dfde93bba3f1f2

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let scale = Scale::new(Some((0i32, 1000000000i32)), 1i32);
let _ = scale.scale_y(-1000000000i32);</pre></td><td valign="top"><pre>thread 'main' panicked at harfrust/src/scale.rs:110:11:
attempt to multiply with overflow
backtrace:
  &lt;harfrust::scale::Scale&gt;::scale_by_mult (harfrust/src/scale.rs:110:11)
  &lt;harfrust::scale::Scale&gt;::scale_y (harfrust/src/scale.rs:64:9)
  repro_harfrust_src_scale_rs_110_11_bf53cc::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut buffer = Buffer::new();
let _ = buffer.reserve(100000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2000000000 bytes failed
backtrace:
  harfrust::buffer::Buffer::enlarge (src/buffer.rs:2093)
  harfrust::buffer::Buffer::ensure (harfrust/src/buffer.rs:2081)
  harfrust::buffer::Buffer::reserve (harfrust/src/buffer.rs:677)
  repro_out_of_memory_in_harfrust_buffer_reserve_63a821::main (src/main.rs:5)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut buffer = Buffer::new();
let _ = buffer.set_length(100000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2000000000 bytes failed
backtrace:
  harfrust::buffer::Buffer::enlarge (src/buffer.rs:2093)
  harfrust::buffer::Buffer::ensure (src/buffer.rs:2081)
  harfrust::buffer::Buffer::set_length (src/buffer.rs:657)
  repro_out_of_memory_in_harfrust_buffer_set_length_766f48::main (src/main.rs:5)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut buffer = Buffer::default();
buffer.reverse_range(10000000000000000000usize, 255usize);</pre></td><td valign="top"><pre>thread 'main' panicked at harfrust/src/buffer.rs:1003:12:
attempt to subtract with overflow
backtrace:
  &lt;harfrust::buffer::Buffer&gt;::reverse_range (harfrust/src/buffer.rs:1003:12)
  repro_harfrust_src_buffer_rs_1003_12_eecec4::main (src/main.rs:5:12)</pre></td></tr>
</table>
