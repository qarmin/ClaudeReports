# lz4_flex - crashes

Source: https://github.com/pseitz/lz4_flex @ f6251928e30af958243a3ef8f22e3071d241543d

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = lz4_flex::decompress_size_prepended(&amp;vec![82u8, 184u8, 46u8, 226u8]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3794712658 bytes failed
backtrace:
  lz4_flex::block::decompress_safe::decompress (lz4_flex/src/block/decompress_safe.rs:361)
  lz4_flex::block::decompress_safe::decompress_size_prepended (lz4_flex/src/block/decompress_safe.rs:348)
  repro_out_of_memory_in_lz4_flex_decompress_size_prepended_6dc8f5::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = lz4_flex::decompress(&amp;vec![], 4294967295usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967295 bytes failed
backtrace:
  lz4_flex::block::decompress_safe::decompress (lz4_flex/src/block/decompress_safe.rs:361)
  repro_out_of_memory_in_lz4_flex_decompress_546653::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
</table>
