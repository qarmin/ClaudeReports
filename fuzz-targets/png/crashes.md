# png - crashes

Source: https://github.com/image-rs/image-png @ b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut vec![80u8], 1usize, &amp;vec![], &amp;Adam7Info::new(1u8,
    0u32, 1u32), 10u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:556:13:
assertion failed: bits_per_pixel % 8 == 0
backtrace:
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:556:13)
  repro_image_png_src_adam7_rs_556_13_65056b::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::expand_interlaced_row(&amp;mut vec![], 10000000000000000000usize,
    &amp;include_bytes!("image_0.png").to_vec(), &amp;Adam7Info::new(4u8, 163954023u32,
    3780522784u32), 1u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:275:22:
attempt to multiply with overflow
backtrace:
  png::adam7::expand_adam7_bits (image-png/src/adam7.rs:275:22)
  png::adam7::expand_pass (image-png/src/adam7.rs:361:31)
  repro_image_png_src_adam7_rs_275_22_a194ec::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::adam7(&amp;mut vec![], &amp;vec![], 1u32, 1u32, 1u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/benchable_apis.rs:30:49:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  png::benchable_apis::adam7 (image-png/src/benchable_apis.rs:30:49)
  repro_image_png_src_benchable_apis_rs_30_49_f2eb82::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>png::expand_interlaced_row(&amp;mut vec![], 0usize, &amp;vec![], &amp;Adam7Info::new(1u8,
    0u32, 1u32), 100u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:403:13:
assertion failed: bits_per_pixel % 8 == 0
backtrace:
  png::adam7::expand_pass (image-png/src/adam7.rs:403:13)
  repro_image_png_src_adam7_rs_403_13_5d84ce::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::adam7(&amp;mut vec![], &amp;vec![0u8], 1u32, 1u32, 1u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:364:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  png::adam7::expand_pass (image-png/src/adam7.rs:364:17)
  png::benchable_apis::adam7 (image-png/src/benchable_apis.rs:30:9)
  repro_image_png_src_adam7_rs_364_17_5300de::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let output_info = OutputInfo { width: 0u32, height: 100000000u32,
    color_type: ColorType::Rgba, bit_depth: BitDepth::from_u8(1u8).unwrap(),
    line_size: 10000000000000000000usize };
let _ = output_info.buffer_size();</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/decoder/mod.rs:59:9:
attempt to multiply with overflow
backtrace:
  &lt;png::decoder::OutputInfo&gt;::buffer_size (image-png/src/decoder/mod.rs:59:9)
  repro_image_png_src_decoder_mod_rs_59_9_b6e4c1::main (src/main.rs:7:25)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut vec![], 0usize, &amp;vec![], &amp;Adam7Info::new(1u8,
    0u32, 1u32), 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:480:18:
attempt to divide by zero
backtrace:
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:480:18)
  repro_image_png_src_adam7_rs_480_18_b17aa7::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::expand_interlaced_row(&amp;mut vec![], usize::MAX, &amp;vec![],
    &amp;Adam7Info::new(1u8, 4294967294u32, 2862318691u32), 232u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:306:22:
attempt to multiply with overflow
backtrace:
  png::adam7::expand_adam7_bytes (image-png/src/adam7.rs:306:22)
  png::adam7::expand_pass (image-png/src/adam7.rs:405:32)
  repro_image_png_src_adam7_rs_306_22_bf99dc::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::adam7(&amp;mut vec![0u8, 80u8, 78u8, 71u8, 0u8 /* x8 */, 73u8,
    72u8, 68u8, 82u8], &amp;vec![0u8, 0u8, 0u8, 88u8, 0u8, 0u8, 0u8, 33u8, 62u8,
    112u8, 40u8, 0u8, 90u8, 113u8, 0u8, 93u8], 1u32, 1000000000u32, 128u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:408:20:
range start index 128 out of range for slice of length 16
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  png::adam7::expand_pass (image-png/src/adam7.rs:408:20)
  png::benchable_apis::adam7 (image-png/src/benchable_apis.rs:30:9)
  repro_image_png_src_adam7_rs_408_20_7fec7d::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::unfilter(Filter::MinEntropy, 0u8, &amp;vec![], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/benchable_apis.rs:12:49:
called `Option::unwrap()` on a `None` value
backtrace:
  png::benchable_apis::unfilter (image-png/src/benchable_apis.rs:12:49)
  repro_image_png_src_benchable_apis_rs_12_49_85ed66::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::adam7(&amp;mut vec![], &amp;vec![], 1u32, 1u32, 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:407:66:
chunk size must be non-zero
backtrace:
  &lt;[u8]&gt;::chunks (core/src/slice/mod.rs:1155:9)
  png::adam7::expand_pass (image-png/src/adam7.rs:407:66)
  png::benchable_apis::adam7 (image-png/src/benchable_apis.rs:30:9)
  repro_image_png_src_adam7_rs_407_66_be05d8::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>png::expand_interlaced_row(&amp;mut vec![], 0usize, &amp;vec![0u8], &amp;Adam7Info::new(4u8,
    0u32, 10000u32), 16u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:399:20:
range start index 4 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  png::adam7::expand_pass (image-png/src/adam7.rs:399:20)
  repro_image_png_src_adam7_rs_399_20_d5bf03::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::adam7(&amp;mut vec![], &amp;vec![0u8], 1u32, 1u32, 4u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:382:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  png::adam7::expand_pass (image-png/src/adam7.rs:382:17)
  png::benchable_apis::adam7 (image-png/src/benchable_apis.rs:30:9)
  repro_image_png_src_adam7_rs_382_17_a0bdd5::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut frame_control = FrameControl { sequence_number: u32::MAX, width: 0u32,
    height: 0u32, x_offset: 0u32, y_offset: 0u32, delay_num: 0u16,
    delay_den: 0u16, dispose_op: DisposeOp::Previous, blend_op: BlendOp::Over };
frame_control.inc_seq_num(1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/common.rs:310:9:
attempt to add with overflow
backtrace:
  &lt;png::common::FrameControl&gt;::inc_seq_num (image-png/src/common.rs:310:9)
  repro_image_png_src_common_rs_310_9_e1e429::main (src/main.rs:7:19)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut vec![0u8], 1usize, &amp;vec![], &amp;Adam7Info::new(1u8,
    0u32, 1u32), 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:565:41:
chunk size must be non-zero
backtrace:
  &lt;[u8]&gt;::chunks_exact (core/src/slice/mod.rs:1242:9)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:565:41)
  repro_image_png_src_adam7_rs_565_41_84253d::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::adam7(&amp;mut vec![], &amp;vec![0u8], 1u32, 1u32, 8u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:392:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  png::adam7::expand_pass (image-png/src/adam7.rs:392:17)
  png::benchable_apis::adam7 (image-png/src/benchable_apis.rs:30:9)
  repro_image_png_src_adam7_rs_392_17_eaf3d9::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let filter = Filter::Avg;
png::benchable_apis::unfilter(filter, 0u8, &amp;vec![], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/common.rs:895:18:
internal error: entered unreachable code: Not a possible byte rounded pixel width
backtrace:
  &lt;png::common::BytesPerPixel&gt;::from_usize (image-png/src/common.rs:895:18)
  png::benchable_apis::unfilter (image-png/src/benchable_apis.rs:13:16)
  repro_image_png_src_common_rs_895_18_ae7e97::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut vec![], 1usize, &amp;vec![], &amp;Adam7Info::new(1u8,
    1000000000u32, 1u32), 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:79:22:
attempt to multiply with overflow
backtrace:
  &lt;png::adam7::Adam7Info&gt;::splat_line_repeat (image-png/src/adam7.rs:79:22)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:481:35)
  repro_image_png_src_adam7_rs_79_22_584773::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut vec![], 1usize, &amp;vec![], &amp;Adam7Info::new(1u8,
    1u32, 1u32), 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:80:9:
attempt to subtract with overflow
backtrace:
  &lt;png::adam7::Adam7Info&gt;::splat_line_repeat (image-png/src/adam7.rs:80:9)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:481:35)
  repro_image_png_src_adam7_rs_80_9_1841cb::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::adam7(&amp;mut vec![], &amp;vec![35u8], 1u32, 1u32, 2u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:373:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  png::adam7::expand_pass (image-png/src/adam7.rs:373:17)
  png::benchable_apis::adam7 (image-png/src/benchable_apis.rs:30:9)
  repro_image_png_src_adam7_rs_373_17_5b125a::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::adam7(&amp;mut vec![], &amp;vec![73u8, 73u8, 42u8, 0u8, 39u8, 0u8,
    0u8, 0u8], 1u32, 1u32, 64u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:408:31:
range end index 8 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  png::adam7::expand_pass (image-png/src/adam7.rs:408:31)
  png::benchable_apis::adam7 (image-png/src/benchable_apis.rs:30:9)
  repro_image_png_src_adam7_rs_408_31_c62d1c::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut vec![], 1usize, &amp;vec![], &amp;Adam7Info::new(1u8,
    0u32, 1u32), 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:482:5:
assertion failed: y_repeat &gt; 0
backtrace:
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:482:5)
  repro_image_png_src_adam7_rs_482_5_e62e19::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut vec![0u8], 1usize, &amp;vec![0u8, 80u8, 78u8, 71u8,
    0u8 /* x8 */, 73u8, 72u8, 68u8, 82u8, 0u8 /* x11 */, 0u8],
    &amp;Adam7Info::new(1u8, 0u32, 1u32), 224u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:569:67:
range end index 28 out of range for slice of length 1
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:569:67)
  repro_image_png_src_adam7_rs_569_67_8e2a1b::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::benchable_apis::adam7(&amp;mut vec![], &amp;vec![0u8, 80u8], 1u32, 1u32, 16u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:399:31:
range end index 2 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  png::adam7::expand_pass (image-png/src/adam7.rs:399:31)
  png::benchable_apis::adam7 (image-png/src/benchable_apis.rs:30:9)
  repro_image_png_src_adam7_rs_399_31_b780a9::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut include_bytes!("data_0.bin").to_vec(), 1usize,
    &amp;vec![0u8], &amp;Adam7Info::new(1u8, 16u32, 1000000000u32), 1u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:460:70:
called `Option::unwrap()` on a `None` value
backtrace:
  png::adam7::expand_pass_splat::expand_bits_to_img (image-png/src/adam7.rs:460:70)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:502:21)
  repro_image_png_src_adam7_rs_460_70_180c70::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut vec![0u8], 1usize, &amp;vec![0u8, 80u8, 78u8, 71u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8], &amp;Adam7Info::new(2u8, 0u32, 10u32),
    88u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:569:42:
range start index 44 out of range for slice of length 1
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:569:42)
  repro_image_png_src_adam7_rs_569_42_24c167::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut vec![0u8, 80u8, 78u8, 71u8, 0u8, 0u8, 0u8, 0u8,
    0u8], 1usize, &amp;vec![0u8], &amp;Adam7Info::new(1u8, 1u32, 1000000000u32), 8u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:551:49:
range end index 8 out of range for slice of length 1
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:551:49)
  repro_image_png_src_adam7_rs_551_49_62112e::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::expand_interlaced_row(&amp;mut vec![0u8, 80u8], 0usize, &amp;vec![68u8],
    &amp;Adam7Info::new(1u8, 0u32, 1u32), 16u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:399:37:
copy_from_slice: source slice length (1) does not match destination slice length (2)
backtrace:
  &lt;[u8]&gt;::copy_from_slice (core/src/slice/mod.rs:4344:18)
  png::adam7::expand_pass (image-png/src/adam7.rs:399:37)
  repro_image_png_src_adam7_rs_399_37_822dc7::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let adam7_info = Adam7Info::new(1u8, 0u32, 1000000000u32);
png::splat_interlaced_row(&amp;mut vec![0u8], 1usize, &amp;vec![0u8], &amp;adam7_info, 2u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:468:55:
called `Option::unwrap()` on a `None` value
backtrace:
  png::adam7::expand_pass_splat::expand_bits_to_img (image-png/src/adam7.rs:468:55)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:519:21)
  repro_image_png_src_adam7_rs_468_55_d68e2e::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut include_bytes!("data_0.bin").to_vec(), 1usize,
    &amp;vec![0u8, 80u8, 78u8, 71u8, 0u8 /* x8 */, 73u8, 72u8, 68u8, 82u8,
    0u8 /* x10 */, 0u8], &amp;Adam7Info::new(2u8, 1u32, 10000u32), 2u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:460:39:
range start index 219 out of range for slice of length 218
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  png::adam7::expand_pass_splat::expand_bits_to_img (image-png/src/adam7.rs:460:39)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:519:21)
  repro_image_png_src_adam7_rs_460_39_a6d0ba::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>png::splat_interlaced_row(&amp;mut include_bytes!("data_0.bin").to_vec(), 1usize,
    &amp;vec![0u8, 80u8, 78u8, 71u8, 0u8, 0u8, 0u8], &amp;Adam7Info::new(2u8, 19u32,
    10000u32), 8u8);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/adam7.rs:551:24:
range start index 204 out of range for slice of length 200
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  png::adam7::expand_pass_splat (image-png/src/adam7.rs:551:24)
  repro_image_png_src_adam7_rs_551_24_62b5c0::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let encoder = &lt;png::Encoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![], 1000000000u32, 1u32);
let writer = encoder.write_header().unwrap();
let stream_writer = writer.into_stream_writer().unwrap();</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000 bytes failed
backtrace:
  png::encoder::StreamWriter&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::new&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (image-png/src/encoder.rs:1575)
  png::encoder::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::into_stream_writer_with_size&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (image-png/src/encoder.rs:1261)
  png::encoder::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::into_stream_writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (image-png/src/encoder.rs:1252)
  repro_out_of_memory_in_png_writer_vec_u8_into_stream_writer_b08e80::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = UnfilterRegion::default();
let encoder = &lt;png::Encoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![], 1u32, 1u32);
let writer = encoder.write_header().unwrap();
let stream_writer = writer.into_stream_writer_with_size(usize::MAX).unwrap();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483647 bytes failed
backtrace:
  png::encoder::ChunkWriter&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::new&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (image-png/src/encoder.rs:1352)
  png::encoder::StreamWriter&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::new&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (image-png/src/encoder.rs:1577)
  png::encoder::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::into_stream_writer_with_size&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (image-png/src/encoder.rs:1261)
  repro_out_of_memory_in_png_writer_vec_u8_into_stream_writer_with_s_8c828a::main (src/main.rs:7)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let decoder = &lt;png::Decoder&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::new_with_options(
    std::io::Cursor::new(include_bytes!("data_0.bin").to_vec()),
    DecodeOptions::default());
let mut reader = decoder.read_info().unwrap();
let _ = reader.read_row(&amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/decoder/mod.rs:573:47:
range end index 144 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;png::decoder::Reader&lt;core::io::cursor::Cursor&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;&gt;::read_row (image-png/src/decoder/mod.rs:573:47)
  repro_image_png_src_decoder_mod_rs_573_47_c6d562::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let decoder = &lt;png::Decoder&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::new(
    std::io::Cursor::new(include_bytes!("data_0.bin").to_vec()));
let mut reader = decoder.read_info().unwrap();
let _ = reader.finish();
let _ = reader.next_frame_info();</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/decoder/mod.rs:346:13:
attempt to subtract with overflow
backtrace:
  &lt;png::decoder::Reader&lt;core::io::cursor::Cursor&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;&gt;::next_frame_info (image-png/src/decoder/mod.rs:346:13)
  repro_image_png_src_decoder_mod_rs_346_13_43480e::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut encoder = &lt;png::Encoder&lt;Vec&lt;u8&gt;&gt;&gt;::with_info(vec![],
    Info::with_size(u32::MAX, 1874738817u32)).unwrap();
encoder.set_color(ColorType::Rgb);
let mut writer = encoder.write_header().unwrap();
let _ = writer.write_image_data(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at image-png/src/encoder.rs:829:25:
attempt to multiply with overflow
backtrace:
  &lt;png::encoder::Writer&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::write_image_data (image-png/src/encoder.rs:829:25)
  repro_image_png_src_encoder_rs_829_25_8ba825::main (src/main.rs:8:20)</pre></td></tr>
</table>
