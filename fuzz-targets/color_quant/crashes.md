# color_quant - crashes

Source: https://github.com/image-rs/color_quant @ 0e2e48b82e61cc4795882e916aec64b26f1a761a

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = NeuQuant::new(i32::MIN, 0usize, &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at color_quant/src/lib.rs:336:30:
attempt to subtract with overflow
backtrace:
  &lt;color_quant::NeuQuant&gt;::learn (color_quant/src/lib.rs:336:30)
  &lt;color_quant::NeuQuant&gt;::init (color_quant/src/lib.rs:180:14)
  &lt;color_quant::NeuQuant&gt;::new (color_quant/src/lib.rs:148:14)
  repro_color_quant_src_lib_rs_336_30_7a27c0::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let neu_quant = NeuQuant::new(1i32, 1usize, &amp;vec![]);
neu_quant.map_pixel(&amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at color_quant/src/lib.rs:188:9:
assertion failed: pixel.len() == 4
backtrace:
  &lt;color_quant::NeuQuant&gt;::map_pixel (color_quant/src/lib.rs:188:9)
  repro_color_quant_src_lib_rs_188_9_3dc5ee::main (src/main.rs:5:15)</pre></td></tr>
<tr><td valign="top"><pre>let neu_quant = NeuQuant::new(1i32, 1usize, &amp;vec![]);
let _ = neu_quant.index_of(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at color_quant/src/lib.rs:202:9:
assertion failed: pixel.len() == 4
backtrace:
  &lt;color_quant::NeuQuant&gt;::index_of (color_quant/src/lib.rs:202:9)
  repro_color_quant_src_lib_rs_202_9_af45dc::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NeuQuant::new(1i32, 0usize, &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at color_quant/src/lib.rs:439:26:
attempt to subtract with overflow
backtrace:
  &lt;color_quant::NeuQuant&gt;::build_netindex (color_quant/src/lib.rs:439:26)
  &lt;color_quant::NeuQuant&gt;::init (color_quant/src/lib.rs:182:14)
  &lt;color_quant::NeuQuant&gt;::new (color_quant/src/lib.rs:148:14)
  repro_color_quant_src_lib_rs_439_26_823fea::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NeuQuant::new(0i32, 1000000000000000000usize, &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from color_quant/src/lib.rs:140:22)
capacity overflow
backtrace:
  &lt;color_quant::NeuQuant&gt;::new (color_quant/src/lib.rs:140:22)
  repro_color_quant_src_lib_rs_140_22_38a485::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NeuQuant::new(0i32, 0usize, &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at color_quant/src/lib.rs:338:28:
attempt to divide by zero
backtrace:
  &lt;color_quant::NeuQuant&gt;::learn (color_quant/src/lib.rs:338:28)
  &lt;color_quant::NeuQuant&gt;::init (color_quant/src/lib.rs:180:14)
  &lt;color_quant::NeuQuant&gt;::new (color_quant/src/lib.rs:148:14)
  repro_color_quant_src_lib_rs_338_28_6e6292::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NeuQuant::new(0i32, 1000000000usize, &amp;vec![]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 32000000000 bytes failed
backtrace:
  color_quant::NeuQuant::new (src/lib.rs:140)
  repro_out_of_memory_in_color_quant_neuquant_new_951657::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut neu_quant = NeuQuant::new(1i32, 65536usize, &amp;vec![]);
neu_quant.init(&amp;vec![]);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  color_quant::NeuQuant::build_netindex (src/lib.rs:414)
  color_quant::NeuQuant::init (src/lib.rs:182)
  color_quant::NeuQuant::new (src/lib.rs:148)
  repro_timeout_in_color_quant_neuquant_init_fdb65c::main (src/main.rs:4)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NeuQuant::new(1i32, 0usize, &amp;vec![73u8, 73u8, 42u8, 0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at color_quant/src/lib.rs:322:18:
index out of bounds: the len is 0 but the index is 18446744073709551615
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[f64]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[f64] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;color_quant::NeuQuant&gt;::contest (color_quant/src/lib.rs:322:18)
  &lt;color_quant::NeuQuant&gt;::learn (color_quant/src/lib.rs:368:26)
  &lt;color_quant::NeuQuant&gt;::init (color_quant/src/lib.rs:180:14)
  &lt;color_quant::NeuQuant&gt;::new (color_quant/src/lib.rs:148:14)
  repro_color_quant_src_lib_rs_322_18_18c3a4::main (src/main.rs:4:13)</pre></td></tr>
</table>
