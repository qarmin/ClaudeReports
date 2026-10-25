# rmp - crashes

Source: https://github.com/3Hren/msgpack-rust @ cf880019f72e35a884659a9b29d9eb89b862670b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = MessageLen::len_of(&amp;vec![201u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at rmp/src/decode/est.rs:167:30:
not yet implemented
backtrace:
  &lt;rmp::decode::est::MessageLen&gt;::read_one_item (rmp/src/decode/est.rs:167:30)
  &lt;rmp::decode::est::MessageLen&gt;::incremental_len (rmp/src/decode/est.rs:117:37)
  &lt;rmp::decode::est::MessageLen&gt;::len_of (rmp/src/decode/est.rs:89:42)
  repro_rmp_src_decode_est_rs_167_30_f96bb1::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteBuf::from(ByteBuf::with_capacity(4294967295usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967295 bytes failed
backtrace:
  rmp::encode::buffer::ByteBuf::with_capacity (rmp/src/encode/buffer.rs:96)
  repro_out_of_memory_in_rmp_encode_bytebuf_from_rmp_encode_bytebuf_31e479::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteBuf::with_capacity(4294967296usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  rmp::encode::buffer::ByteBuf::with_capacity (rmp/src/encode/buffer.rs:96)
  repro_out_of_memory_in_rmp_encode_bytebuf_with_capacity_548983::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let byte_buf = ByteBuf::with_capacity(2147483648usize);
let _ = byte_buf.as_vec();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  rmp::encode::buffer::ByteBuf::with_capacity (rmp/src/encode/buffer.rs:96)
  repro_out_of_memory_in_rmp_encode_bytebuf_as_vec_269321::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteBuf::with_capacity(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from rmp/src/encode/buffer.rs:96:23)
capacity overflow
backtrace:
  &lt;rmp::encode::buffer::ByteBuf&gt;::with_capacity (rmp/src/encode/buffer.rs:96:23)
  repro_rmp_src_encode_buffer_rs_96_23_70736e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rmp::decode::read_str_ref(&amp;vec![185u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at rmp/src/decode/str.rs:151:30:
range end index 25 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  rmp::decode::str::read_str_ref (rmp/src/decode/str.rs:151:30)
  repro_rmp_src_decode_str_rs_151_30_9d7927::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = MessageLen::len_of(&amp;include_bytes!("data_0.bin").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at rmp/src/decode/est.rs:182:33:
not yet implemented
backtrace:
  &lt;rmp::decode::est::MessageLen&gt;::read_one_item (rmp/src/decode/est.rs:182:33)
  &lt;rmp::decode::est::MessageLen&gt;::read_sequence (rmp/src/decode/est.rs:245:18)
  &lt;rmp::decode::est::MessageLen&gt;::read_marker_with_len (rmp/src/decode/est.rs:222:37)
  &lt;rmp::decode::est::MessageLen&gt;::read_one_item (rmp/src/decode/est.rs:164:35)
  &lt;rmp::decode::est::MessageLen&gt;::read_sequence (rmp/src/decode/est.rs:245:18)
  &lt;rmp::decode::est::MessageLen&gt;::read_one_item (rmp/src/decode/est.rs:148:41)
  &lt;rmp::decode::est::MessageLen&gt;::incremental_len (rmp/src/decode/est.rs:117:37)
  &lt;rmp::decode::est::MessageLen&gt;::len_of (rmp/src/decode/est.rs:89:42)
  repro_rmp_src_decode_est_rs_182_33_18821e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let len_error = LenError::Truncated(std::num::NonZero::new(
    1000000000000000000usize).unwrap());
let usize_v = len_error.len();
let _ = Marker::from(rmp::encode::write_ext_meta::&lt;rmp::encode::ByteBuf&gt;(
    &amp;mut ByteBuf::with_capacity(usize_v), 1u32, 0i8).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000000 bytes failed
backtrace:
  rmp::encode::buffer::ByteBuf::with_capacity (rmp/src/encode/buffer.rs:96)
  repro_out_of_memory_in_rmp_marker_from_rmp_marker_a6947c::main (src/main.rs:8)
  (18 std/libc frames omitted)</pre></td></tr>
</table>
