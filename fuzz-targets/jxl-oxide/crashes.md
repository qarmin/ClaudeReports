# jxl-oxide - crashes

Source: https://github.com/tirr-c/jxl-oxide @ c0cc4c7ea57c1207f38ff2970d94757470613be4

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = ChannelShift::from_jpeg_upsampling([0u32, 0u32, 0u32], 3usize);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-modular/src/param.rs:106:26:
index out of bounds: the len is 3 but the index is 3
backtrace:
  &lt;jxl_modular::param::ChannelShift&gt;::from_jpeg_upsampling (jxl-modular/src/param.rs:106:26)
  repro_jxl_modular_src_param_rs_106_26_ebafd0::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = jxl_color::icc::colour_encoding_to_icc(&amp;EnumColourEncoding::xyb(
    RenderingIntent::Perceptual));</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-color/src/icc/synthesize.rs:98:9:
not yet implemented: ICC profile for XYB color space is not supported yet
backtrace:
  jxl_color::icc::synthesize::colour_encoding_to_icc (jxl-color/src/icc/synthesize.rs:98:9)
  repro_jxl_color_src_icc_synthesize_rs_98_9_0e9ae2::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ChannelShift::from_jpeg_upsampling([1000000000u32, 0u32, 0u32], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-modular/src/param.rs:114:18:
Invalid jpeg_upsampling value of 1000000000
backtrace:
  &lt;jxl_modular::param::ChannelShift&gt;::from_jpeg_upsampling (jxl-modular/src/param.rs:114:18)
  repro_jxl_modular_src_param_rs_114_18_463ff2::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = FrameBuffer::new(1usize, 1000000000usize, 1usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  jxl_oxide::fb::FrameBuffer::new (src/fb.rs:24)
  repro_out_of_memory_in_jxl_oxide_framebuffer_new_2ed721::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let region = Region { left: 0i32, top: 0i32, width: 1u32, height: 0u32 };
let _ = region.downsample(1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-render/src/region.rs:146:19:
attempt to shift left with overflow
backtrace:
  &lt;jxl_render::region::Region&gt;::downsample (jxl-render/src/region.rs:146:19)
  repro_jxl_render_src_region_rs_146_19_4daa0d::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = FrameBuffer::new(1usize, 2usize, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-oxide/src/fb.rs:33:14:
framebuffer dimensions overflow
backtrace:
  &lt;jxl_oxide::fb::FrameBuffer&gt;::buffer_len (jxl-oxide/src/fb.rs:33:14)
  &lt;jxl_oxide::fb::FrameBuffer&gt;::new (jxl-oxide/src/fb.rs:19:19)
  repro_jxl_oxide_src_fb_rs_33_14_317ae0::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let region = Region::with_size(0u32, 1u32);
let region2 = region.translate(0i32, 0i32);
let _ = region2.upsample(1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-render/src/region.rs:204:19:
attempt to shift left with overflow
backtrace:
  &lt;jxl_render::region::Region&gt;::upsample_separate (jxl-render/src/region.rs:204:19)
  &lt;jxl_render::region::Region&gt;::upsample (jxl-render/src/region.rs:198:14)
  repro_jxl_render_src_region_rs_204_19_e4526e::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let channel_shift = ChannelShift::Shifts(46u32);
let _ = channel_shift.shift_size((0u32, 0u32));</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-modular/src/param.rs:165:27:
attempt to shift left with overflow
backtrace:
  &lt;jxl_modular::param::ChannelShift&gt;::shift_size (jxl-modular/src/param.rs:165:27)
  repro_jxl_modular_src_param_rs_165_27_3513fa::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let mut bitstream = Bitstream::new(&amp;vec);
let _ = bitstream.peek_bits(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-bitstream/src/bitstream.rs:94:9:
assertion failed: n &lt;= 32
backtrace:
  &lt;jxl_bitstream::bitstream::Bitstream&gt;::peek_bits (jxl-bitstream/src/bitstream.rs:94:9)
  repro_jxl_bitstream_src_bitstream_rs_94_9_4ffeef::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let region = Region { left: 0i32, top: 0i32, width: 0u32, height: 0u32 };
let _ = region.downsample_separate(1000000000u32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-render/src/region.rs:165:21:
attempt to shift left with overflow
backtrace:
  &lt;jxl_render::region::Region&gt;::downsample_separate (jxl-render/src/region.rs:165:21)
  repro_jxl_render_src_region_rs_165_21_628589::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let channel_shift = ChannelShift::from_shift(1u32);
let _ = channel_shift.shift_size((u32::MAX, 0u32));</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-modular/src/param.rs:166:18:
attempt to add with overflow
backtrace:
  &lt;jxl_modular::param::ChannelShift&gt;::shift_size (jxl-modular/src/param.rs:166:18)
  repro_jxl_modular_src_param_rs_166_18_fcdf4a::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let mut bitstream = Bitstream::new(&amp;vec);
let _ = bitstream.peek_bits_prefilled(1000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-bitstream/src/bitstream.rs:114:9:
assertion failed: n &lt;= 32
backtrace:
  &lt;jxl_bitstream::bitstream::Bitstream&gt;::peek_bits_prefilled (jxl-bitstream/src/bitstream.rs:114:9)
  repro_jxl_bitstream_src_bitstream_rs_114_9_be9193::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![73u8, 73u8, 42u8, 0u8, 39u8, 0u8];
let mut bitstream = Bitstream::new(&amp;vec);
let _ = jxl_coding::read_clusters(&amp;mut bitstream, u32::MAX);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  jxl_coding::read_clusters (src/lib.rs:698)
  repro_timeout_in_jxl_coding_read_clusters_fe6752::main (src/main.rs:6)
  (14 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let bit_depth = BitDepth::IntegerSample { bits_per_sample: 1000000000u32 };
let _ = bit_depth.parse_integer_sample(0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-image/src/lib.rs:461:27:
attempt to shift left with overflow
backtrace:
  &lt;jxl_image::BitDepth&gt;::parse_integer_sample (jxl-image/src/lib.rs:461:27)
  repro_jxl_image_src_lib_rs_461_27_58c91d::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let bit_depth = BitDepth::FloatSample { bits_per_sample: 0u32, exp_bits: 0u32 };
let _ = bit_depth.parse_integer_sample(0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at jxl-image/src/lib.rs:469:37:
attempt to subtract with overflow
backtrace:
  &lt;jxl_image::BitDepth&gt;::parse_integer_sample (jxl-image/src/lib.rs:469:37)
  repro_jxl_image_src_lib_rs_469_37_11c5d9::main (src/main.rs:5:23)</pre></td></tr>
</table>
