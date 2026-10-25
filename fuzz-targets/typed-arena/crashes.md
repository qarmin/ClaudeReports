# typed-arena - crashes

Source: https://github.com/SimonSapin/rust-typed-arena @ f1f26c3994dc881c35591cb629d8466df29e778a

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;typed_arena::Arena&lt;u32&gt;&gt;::with_capacity(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  typed_arena::Arena&lt;u32&gt;::with_capacity&lt;u32&gt; (rust-typed-arena/src/lib.rs:142)
  repro_out_of_memory_in_typed_arena_arena_u32_with_capacity_af28c7::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;typed_arena::Arena&lt;String&gt;&gt;::with_capacity(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 24000000000 bytes failed
backtrace:
  typed_arena::Arena&lt;alloc::string::String&gt;::with_capacity&lt;alloc::string::String&gt; (rust-typed-arena/src/lib.rs:142)
  repro_out_of_memory_in_typed_arena_arena_string_with_capacity_26492a::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;typed_arena::Arena&lt;u8&gt;&gt;::with_capacity(4294967295usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967295 bytes failed
backtrace:
  typed_arena::Arena&lt;u8&gt;::with_capacity&lt;u8&gt; (rust-typed-arena/src/lib.rs:142)
  repro_out_of_memory_in_typed_arena_arena_u8_with_capacity_23ba28::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let arena = &lt;typed_arena::Arena&lt;u32&gt;&gt;::new();
arena.reserve_extend(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  typed_arena::ChunkList&lt;u32&gt;::reserve&lt;u32&gt; (rust-typed-arena/src/lib.rs:560)
  typed_arena::Arena&lt;u32&gt;::reserve_extend&lt;u32&gt; (rust-typed-arena/src/lib.rs:387)
  repro_out_of_memory_in_typed_arena_arena_u32_reserve_extend_48d29e::main (src/main.rs:3)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let arena = &lt;typed_arena::Arena&lt;u8&gt;&gt;::new();
arena.reserve_extend(2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  typed_arena::ChunkList&lt;u8&gt;::reserve&lt;u8&gt; (rust-typed-arena/src/lib.rs:560)
  typed_arena::Arena&lt;u8&gt;::reserve_extend&lt;u8&gt; (rust-typed-arena/src/lib.rs:387)
  repro_out_of_memory_in_typed_arena_arena_u8_reserve_extend_f99d05::main (src/main.rs:3)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let arena = &lt;typed_arena::Arena&lt;String&gt;&gt;::new();
arena.reserve_extend(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 25769803776 bytes failed
backtrace:
  typed_arena::ChunkList&lt;alloc::string::String&gt;::reserve&lt;alloc::string::String&gt; (rust-typed-arena/src/lib.rs:560)
  typed_arena::Arena&lt;alloc::string::String&gt;::reserve_extend&lt;alloc::string::String&gt; (rust-typed-arena/src/lib.rs:387)
  repro_out_of_memory_in_typed_arena_arena_string_reserve_extend_7ad741::main (src/main.rs:3)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;typed_arena::Arena&lt;u32&gt;&gt;::new();
let _ = &lt;typed_arena::Arena&lt;u8&gt;&gt;::with_capacity(0usize);
let _ = &lt;typed_arena::Arena&lt;u8&gt;&gt;::with_capacity(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from rust-typed-arena/src/lib.rs:142:26)
capacity overflow
backtrace:
  &lt;typed_arena::Arena&lt;u8&gt;&gt;::with_capacity (rust-typed-arena/src/lib.rs:142:26)
  repro_rust_typed_arena_src_lib_rs_142_26_028f82::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let arena = &lt;typed_arena::Arena&lt;String&gt;&gt;::new();
let _ = &lt;typed_arena::Arena&lt;String&gt; as ::core::default::Default&gt;::default();
arena.reserve_extend(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-typed-arena/src/lib.rs:558:14:
capacity overflow
backtrace:
  &lt;typed_arena::ChunkList&lt;alloc::string::String&gt;&gt;::reserve (rust-typed-arena/src/lib.rs:558:14)
  &lt;typed_arena::Arena&lt;alloc::string::String&gt;&gt;::reserve_extend (rust-typed-arena/src/lib.rs:387:20)
  repro_rust_typed_arena_src_lib_rs_558_14_a619c8::main (src/main.rs:4:11)</pre></td></tr>
<tr><td valign="top"><pre>let arena = &lt;typed_arena::Arena&lt;u32&gt; as ::core::default::Default&gt;::default();
arena.reserve_extend(7252174369975988866usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from rust-typed-arena/src/lib.rs:560:53)
capacity overflow
backtrace:
  &lt;typed_arena::ChunkList&lt;u32&gt;&gt;::reserve (rust-typed-arena/src/lib.rs:560:53)
  &lt;typed_arena::Arena&lt;u32&gt;&gt;::reserve_extend (rust-typed-arena/src/lib.rs:387:20)
  repro_rust_typed_arena_src_lib_rs_560_53_8a44ab::main (src/main.rs:3:11)</pre></td></tr>
</table>
