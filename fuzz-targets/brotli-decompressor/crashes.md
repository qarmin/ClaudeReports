# brotli-decompressor - crashes

Source: https://github.com/dropbox/rust-brotli-decompressor @ 910099df8c358e58367593047f6b758920076ed6

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let i32_v = brotli_decompressor::transform::kNumTransforms;
let _ = TransformDictionaryWord(&amp;mut vec![], &amp;vec![], 0i32, i32_v);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli-decompressor/src/transform.rs:745:30:
index out of bounds: the len is 121 but the index is 121
backtrace:
  brotli_decompressor::transform::TransformDictionaryWord (rust-brotli-decompressor/src/memory.rs:21:8)
  repro_rust_brotli_decompressor_src_transform_rs_745_30_ff132a::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let i32_v = brotli_decompressor::transform::kNumTransforms;
let _ = TransformDictionaryWord(&amp;mut vec![], &amp;vec![], i32_v, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli-decompressor/src/transform.rs:768:40:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli_decompressor::transform::TransformDictionaryWord (rust-brotli-decompressor/src/memory.rs:28:8)
  repro_rust_brotli_decompressor_src_transform_rs_768_40_b28f4c::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TransformDictionaryWord(&amp;mut vec![], &amp;vec![], -100000000i32, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli-decompressor/src/transform.rs:762:12:
range start index 18446744073609551616 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  brotli_decompressor::transform::TransformDictionaryWord (rust-brotli-decompressor/src/memory.rs:34:17)
  repro_rust_brotli_decompressor_src_transform_rs_762_12_a61dad::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TransformDictionaryWord(&amp;mut vec![], &amp;vec![], 0i32, 2i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli-decompressor/src/transform.rs:747:7:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli_decompressor::transform::TransformDictionaryWord (rust-brotli-decompressor/src/memory.rs:49:8)
  repro_rust_brotli_decompressor_src_transform_rs_747_7_ec691f::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TransformDictionaryWord(&amp;mut vec![], &amp;vec![71u8], 1i32, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli-decompressor/src/transform.rs:768:7:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli_decompressor::transform::TransformDictionaryWord (rust-brotli-decompressor/src/memory.rs:49:8)
  repro_rust_brotli_decompressor_src_transform_rs_768_7_cb2e9c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TransformDictionaryWord(&amp;mut vec![], &amp;vec![], 0i32, 1i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli-decompressor/src/transform.rs:789:7:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli_decompressor::transform::TransformDictionaryWord (rust-brotli-decompressor/src/memory.rs:49:8)
  repro_rust_brotli_decompressor_src_transform_rs_789_7_15b4b9::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TransformDictionaryWord(&amp;mut vec![], &amp;vec![], 0i32, 4i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli-decompressor/src/transform.rs:721:7:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli_decompressor::transform::ToUpperCase (rust-brotli-decompressor/src/memory.rs:28:8)
  brotli_decompressor::transform::TransformDictionaryWord (rust-brotli-decompressor/src/transform.rs:774:7)
  repro_rust_brotli_decompressor_src_transform_rs_721_7_a78435::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = brotli_decompressor::brotli_decode_prealloc(&amp;vec![], &amp;mut vec![],
    &amp;mut vec![], &amp;mut vec![], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at alloc-no-stdlib/src/stack_allocator.rs:52:13:
OOM
backtrace:
  &lt;alloc_no_stdlib::stack_allocator::StackAllocator&lt;brotli_decompressor (alloc-no-stdlib/src/stack_allocator.rs:52:13)
  &lt;brotli_decompressor::state::BrotliState&lt;alloc_no_stdlib::stack_allocator::StackAllocator&lt;u8, brotli_decompressor::MemPool&lt;u8&gt;&gt;, alloc_no_stdlib::stack_allocator::StackAllocator&lt;u32, brotli_decompressor::MemPool&lt;u32&gt;&gt;, alloc_no_stdlib::stack_allocator::StackAllocator&lt;brotli_decompressor (rust-brot... (+36 chars)
  brotli_decompressor::brotli_decode_prealloc (rust-brotli-decompressor/src/lib.rs:395:5)
  repro_alloc_no_stdlib_src_stack_allocator_rs_52_13_76ad39::main (src/main.rs:2:13)</pre></td></tr>
</table>
