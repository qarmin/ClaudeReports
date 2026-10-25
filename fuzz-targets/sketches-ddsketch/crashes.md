# sketches-ddsketch - crashes

Source: https://github.com/mheffner/rust-sketches-ddsketch @ 9c517a0996152f9f12a609d7b720d7cead01fede

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::new(Config::new(0.063f64, 1000000000u32,
    1.0f64));
d_d_sketch.add(f64::MAX);
d_d_sketch.add_with_count(f64::INFINITY, 1u64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  sketches_ddsketch::store::Store::extend_range (src/store.rs:89)
  sketches_ddsketch::store::Store::get_index (src/store.rs:66)
  sketches_ddsketch::store::Store::add_count (src/store.rs:50)
  sketches_ddsketch::ddsketch::DDSketch::add_with_count (src/ddsketch.rs:91)
  repro_out_of_memory_in_sketches_ddsketch_ddsketch_add_with_count_28ea8a::main (src/main.rs:7)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::new(Config::new(0.103f64, 1000000000u32,
    1.0f64));
d_d_sketch.add(f64::MIN);
d_d_sketch.add(f64::NEG_INFINITY);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  sketches_ddsketch::store::Store::extend_range (src/store.rs:89)
  sketches_ddsketch::store::Store::get_index (src/store.rs:66)
  sketches_ddsketch::store::Store::add_count (src/store.rs:50)
  sketches_ddsketch::ddsketch::DDSketch::add_with_count (src/ddsketch.rs:94)
  sketches_ddsketch::ddsketch::DDSketch::add (src/ddsketch.rs:72)
  repro_out_of_memory_in_sketches_ddsketch_ddsketch_add_ebb1af::main (src/main.rs:7)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::new(Config::new(0.0f64, 100u32, -15.0f64));
d_d_sketch.add_with_count(-0.0f64, 1u64);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:156:9:
attempt to subtract with overflow
backtrace:
  &lt;sketches_ddsketch::store::Store&gt;::shift_bins (rust-sketches-ddsketch/src/store.rs:156:9)
  &lt;sketches_ddsketch::store::Store&gt;::center_bins (rust-sketches-ddsketch/src/store.rs:162:14)
  &lt;sketches_ddsketch::store::Store&gt;::adjust (rust-sketches-ddsketch/src/store.rs:135:18)
  &lt;sketches_ddsketch::store::Store&gt;::extend_range (rust-sketches-ddsketch/src/store.rs:81:18)
  &lt;sketches_ddsketch::store::Store&gt;::get_index (rust-sketches-ddsketch/src/store.rs:61:18)
  &lt;sketches_ddsketch::store::Store&gt;::add_count (rust-sketches-ddsketch/src/store.rs:50:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add_with_count (rust-sketches-ddsketch/src/ddsketch.rs:91:24)
  repro_rust_sketches_ddsketch_src_store_rs_156_9_8f13ab::main (src/main.rs:6:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::default();
d_d_sketch.add_with_count(0.0f64, u64::MAX);
d_d_sketch.add_with_count(0.0f64, 1u64);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/ddsketch.rs:96:13:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add_with_count (rust-sketches-ddsketch/src/ddsketch.rs:96:13)
  repro_rust_sketches_ddsketch_src_ddsketch_rs_96_13_fda8a9::main (src/main.rs:6:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::default();
d_d_sketch.add(-0.0f64);
d_d_sketch.add_with_count(1.0f64, u64::MAX);
let _ = d_d_sketch.sum();</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/ddsketch.rs:182:9:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::total_count (rust-sketches-ddsketch/src/ddsketch.rs:182:9)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::count (rust-sketches-ddsketch/src/ddsketch.rs:178:30)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::empty (rust-sketches-ddsketch/src/ddsketch.rs:224:14)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::sum (rust-sketches-ddsketch/src/ddsketch.rs:167:17)
  repro_rust_sketches_ddsketch_src_ddsketch_rs_182_9_5edabe::main (src/main.rs:7:24)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Config::new(0.0f64, 0u32, -0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/config.rs:42:21:
attempt to subtract with overflow
backtrace:
  &lt;sketches_ddsketch::config::Config&gt;::new (rust-sketches-ddsketch/src/config.rs:42:21)
  repro_rust_sketches_ddsketch_src_config_rs_42_21_9d66ad::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&amp;DDSketch::new(
    Config::new(0.0f64, 0u32, 1.0f64)));
let _ = sketches_ddsketch::encoding::decode_from_java_bytes(&amp;vec);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/config.rs:89:21:
attempt to subtract with overflow
backtrace:
  &lt;sketches_ddsketch::config::Config&gt;::from_gamma (rust-sketches-ddsketch/src/config.rs:89:21)
  sketches_ddsketch::encoding::decode_from_java_bytes (rust-sketches-ddsketch/src/encoding.rs:410:18)
  repro_rust_sketches_ddsketch_src_config_rs_89_21_36ad00::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::default();
d_d_sketch.add(f64::INFINITY);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:161:21:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::store::Store&gt;::center_bins (rust-sketches-ddsketch/src/store.rs:161:21)
  &lt;sketches_ddsketch::store::Store&gt;::adjust (rust-sketches-ddsketch/src/store.rs:135:18)
  &lt;sketches_ddsketch::store::Store&gt;::extend_range (rust-sketches-ddsketch/src/store.rs:81:18)
  &lt;sketches_ddsketch::store::Store&gt;::get_index (rust-sketches-ddsketch/src/store.rs:66:18)
  &lt;sketches_ddsketch::store::Store&gt;::add_count (rust-sketches-ddsketch/src/store.rs:50:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add_with_count (rust-sketches-ddsketch/src/ddsketch.rs:91:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add (rust-sketches-ddsketch/src/ddsketch.rs:72:14)
  repro_rust_sketches_ddsketch_src_store_rs_161_21_a3a9b5::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::new(Config::new(0.0f64, 0u32, -86.0f64));
d_d_sketch.add(0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:112:26:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u64]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[u64] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;sketches_ddsketch::store::Store&gt;::adjust (rust-sketches-ddsketch/src/store.rs:112:26)
  &lt;sketches_ddsketch::store::Store&gt;::extend_range (rust-sketches-ddsketch/src/store.rs:81:18)
  &lt;sketches_ddsketch::store::Store&gt;::get_index (rust-sketches-ddsketch/src/store.rs:61:18)
  &lt;sketches_ddsketch::store::Store&gt;::add_count (rust-sketches-ddsketch/src/store.rs:50:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add_with_count (rust-sketches-ddsketch/src/ddsketch.rs:91:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add (rust-sketches-ddsketch/src/ddsketch.rs:72:14)
  repro_rust_sketches_ddsketch_src_store_rs_112_26_d5b861::main (src/main.rs:6:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::default();
d_d_sketch.add_with_count(41.0f64, u64::MAX);
d_d_sketch.add_with_count(f64::INFINITY, 18446744073709551614u64);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:52:9:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::store::Store&gt;::add_count (rust-sketches-ddsketch/src/store.rs:52:9)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add_with_count (rust-sketches-ddsketch/src/ddsketch.rs:91:24)
  repro_rust_sketches_ddsketch_src_store_rs_52_9_db2b8c::main (src/main.rs:6:16)</pre></td></tr>
<tr><td valign="top"><pre>let config = Config::defaults();
let mut d_d_sketch = DDSketch::new(config);
d_d_sketch.add(1.0f64);
d_d_sketch.add(f64::INFINITY);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:96:30:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::store::Store&gt;::get_new_length (rust-sketches-ddsketch/src/store.rs:96:30)
  &lt;sketches_ddsketch::store::Store&gt;::extend_range (rust-sketches-ddsketch/src/store.rs:87:35)
  &lt;sketches_ddsketch::store::Store&gt;::get_index (rust-sketches-ddsketch/src/store.rs:66:18)
  &lt;sketches_ddsketch::store::Store&gt;::add_count (rust-sketches-ddsketch/src/store.rs:50:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add_with_count (rust-sketches-ddsketch/src/ddsketch.rs:91:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add (rust-sketches-ddsketch/src/ddsketch.rs:72:14)
  repro_rust_sketches_ddsketch_src_store_rs_96_30_5cc0b4::main (src/main.rs:8:16)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DDSketch::from_java_bytes(&amp;include_bytes!("data_0.bin").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/encoding.rs:272:17:
attempt to add with overflow
backtrace:
  sketches_ddsketch::encoding::decode_store (rust-sketches-ddsketch/src/encoding.rs:272:17)
  sketches_ddsketch::encoding::decode_from_java_bytes (rust-sketches-ddsketch/src/encoding.rs:393:39)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::from_java_bytes (rust-sketches-ddsketch/src/ddsketch.rs:239:9)
  repro_rust_sketches_ddsketch_src_encoding_rs_272_17_3f3f7c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::new(Config::default());
d_d_sketch.add_with_count(1.0f64, u64::MAX);
let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&amp;d_d_sketch);
let mut d_d_sketch2 = sketches_ddsketch::encoding::decode_from_java_bytes(
    &amp;vec).unwrap();
let _ = d_d_sketch2.merge(&amp;d_d_sketch);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:208:13:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::store::Store&gt;::merge (rust-sketches-ddsketch/src/store.rs:208:13)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::merge (rust-sketches-ddsketch/src/ddsketch.rs:201:20)
  repro_rust_sketches_ddsketch_src_store_rs_208_13_ddf6a7::main (src/main.rs:9:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::new(Config::new(-0.0f64, 0u32, 1.0f64));
d_d_sketch.add(1e300f64);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:105:31:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::store::Store&gt;::adjust (rust-sketches-ddsketch/src/store.rs:105:31)
  &lt;sketches_ddsketch::store::Store&gt;::extend_range (rust-sketches-ddsketch/src/store.rs:81:18)
  &lt;sketches_ddsketch::store::Store&gt;::get_index (rust-sketches-ddsketch/src/store.rs:66:18)
  &lt;sketches_ddsketch::store::Store&gt;::add_count (rust-sketches-ddsketch/src/store.rs:50:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add_with_count (rust-sketches-ddsketch/src/ddsketch.rs:91:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add (rust-sketches-ddsketch/src/ddsketch.rs:72:14)
  repro_rust_sketches_ddsketch_src_store_rs_105_31_73d244::main (src/main.rs:6:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::default();
d_d_sketch.add(12.0f64);
d_d_sketch.add(f64::INFINITY);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:10:6:
attempt to add with overflow
backtrace:
  sketches_ddsketch::store::div_ceil (rust-sketches-ddsketch/src/store.rs:10:6)
  &lt;sketches_ddsketch::store::Store&gt;::get_new_length (rust-sketches-ddsketch/src/store.rs:98:27)
  &lt;sketches_ddsketch::store::Store&gt;::extend_range (rust-sketches-ddsketch/src/store.rs:87:35)
  &lt;sketches_ddsketch::store::Store&gt;::get_index (rust-sketches-ddsketch/src/store.rs:66:18)
  &lt;sketches_ddsketch::store::Store&gt;::add_count (rust-sketches-ddsketch/src/store.rs:50:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add_with_count (rust-sketches-ddsketch/src/ddsketch.rs:91:24)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::add (rust-sketches-ddsketch/src/ddsketch.rs:72:14)
  repro_rust_sketches_ddsketch_src_store_rs_10_6_580896::main (src/main.rs:6:16)</pre></td></tr>
<tr><td valign="top"><pre>let _ = sketches_ddsketch::encoding::decode_from_java_bytes(&amp;vec![15u8, 56u8,
    215u8, 0u8, 137u8, 116u8, 194u8, 60u8, 65u8, 126u8, 83u8, 41u8, 233u8,
    100u8, 13u8, 150u8, 174u8, 61u8, 158u8, 109u8, 22u8, 152u8, 242u8, 6u8,
    193u8, 140u8, 201u8, 61u8, 117u8, 46u8, 126u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:51:9:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::store::Store&gt;::add_count (rust-sketches-ddsketch/src/store.rs:51:9)
  sketches_ddsketch::encoding::decode_store (rust-sketches-ddsketch/src/encoding.rs:290:23)
  sketches_ddsketch::encoding::decode_from_java_bytes (rust-sketches-ddsketch/src/encoding.rs:400:39)
  repro_rust_sketches_ddsketch_src_store_rs_51_9_f4b355::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DDSketch::from_java_bytes(&amp;vec![15u8, 163u8, 105u8, 188u8, 160u8, 125u8,
    245u8, 147u8, 144u8, 223u8, 173u8, 236u8, 202u8, 161u8, 70u8, 101u8, 3u8,
    174u8, 224u8, 79u8, 15u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/encoding.rs:291:17:
attempt to add with overflow
backtrace:
  sketches_ddsketch::encoding::decode_store (rust-sketches-ddsketch/src/encoding.rs:291:17)
  sketches_ddsketch::encoding::decode_from_java_bytes (rust-sketches-ddsketch/src/encoding.rs:400:39)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::from_java_bytes (rust-sketches-ddsketch/src/ddsketch.rs:239:9)
  repro_rust_sketches_ddsketch_src_encoding_rs_291_17_85ddbf::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&amp;DDSketch::new(
    Config::default()));
let mut d_d_sketch = DDSketch::default();
let mut d_d_sketch2 = sketches_ddsketch::encoding::decode_from_java_bytes(
    &amp;vec).unwrap();
d_d_sketch2.add_with_count(0.0f64, u64::MAX);
d_d_sketch.add(0.0f64);
let _ = d_d_sketch.merge(&amp;d_d_sketch2);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/ddsketch.rs:203:9:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::merge (rust-sketches-ddsketch/src/ddsketch.rs:203:9)
  repro_rust_sketches_ddsketch_src_ddsketch_rs_203_9_a70f5c::main (src/main.rs:10:24)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::new(Config::default());
d_d_sketch.add(1.0f64);
let mut d_d_sketch2 = DDSketch::default();
d_d_sketch2.add_with_count(3.922934895103397e30f64, u64::MAX);
let _ = d_d_sketch.merge(&amp;d_d_sketch2);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:211:9:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::store::Store&gt;::merge (rust-sketches-ddsketch/src/store.rs:211:9)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::merge (rust-sketches-ddsketch/src/ddsketch.rs:201:20)
  repro_rust_sketches_ddsketch_src_store_rs_211_9_c86fe7::main (src/main.rs:9:24)</pre></td></tr>
<tr><td valign="top"><pre>let _ = sketches_ddsketch::encoding::decode_from_java_bytes(&amp;include_bytes!(
    "data_0.bin").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/encoding.rs:280:17:
attempt to add with overflow
backtrace:
  sketches_ddsketch::encoding::decode_store (rust-sketches-ddsketch/src/encoding.rs:280:17)
  sketches_ddsketch::encoding::decode_from_java_bytes (rust-sketches-ddsketch/src/encoding.rs:400:39)
  repro_rust_sketches_ddsketch_src_encoding_rs_280_17_8ba3b1::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut d_d_sketch = DDSketch::default();
d_d_sketch.add(131.0f64);
d_d_sketch.add(f64::INFINITY);
let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&amp;d_d_sketch);
let _ = d_d_sketch.merge(&amp;DDSketch::from_java_bytes(&amp;vec).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at rust-sketches-ddsketch/src/store.rs:196:61:
attempt to add with overflow
backtrace:
  &lt;sketches_ddsketch::store::Store&gt;::merge (rust-sketches-ddsketch/src/store.rs:196:61)
  &lt;sketches_ddsketch::ddsketch::DDSketch&gt;::merge (rust-sketches-ddsketch/src/ddsketch.rs:201:20)
  repro_rust_sketches_ddsketch_src_store_rs_196_61_b95161::main (src/main.rs:8:24)</pre></td></tr>
</table>
