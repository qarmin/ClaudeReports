# zune-image - crashes

Source: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let frame = Frame::new(vec![]);
let _ = frame.channels_ref(ColorSpace::RGB, false);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/frame.rs:277:27:
range end index 3 out of range for slice of length 0
backtrace:
  &lt;[zune_image::channel::Channel] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::frame::Frame&gt;::channels_ref (zune-image/src/frame.rs:277:27)
  repro_zune_image_src_frame_rs_277_27_066cf8::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Channel::new_with_bit_type(usize::MAX, BitType::U8);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/channel.rs:225:67:
called `Result::unwrap()` on an `Err` value: LayoutError
backtrace:
  &lt;zune_image::channel::Channel&gt;::alloc (zune-image/src/channel.rs:225:67)
  &lt;zune_image::channel::Channel&gt;::new_with_capacity_and_type (zune-image/src/channel.rs:378:38)
  &lt;zune_image::channel::Channel&gt;::new_with_length_and_type (zune-image/src/channel.rs:297:27)
  &lt;zune_image::channel::Channel&gt;::new_with_bit_type (zune-image/src/channel.rs:328:9)
  repro_zune_image_src_channel_rs_225_67_a7ca75::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let frame = Frame::new_with_duration(vec![], 0usize, 0usize);
let _ = frame.u16_to_u8_endian(Endianness::Native);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/frame.rs:520:35:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[zune_image::channel::Channel]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[zune_image::channel::Channel] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::frame::Frame&gt;::u16_to_u8_endian (zune-image/src/frame.rs:520:35)
  repro_zune_image_src_frame_rs_520_35_a4ee6b::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let frame = Frame::new(vec![]);
let _ = frame.u16_to_u8_endian_into(Endianness::Little, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/frame.rs:413:35:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[zune_image::channel::Channel]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[zune_image::channel::Channel] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::frame::Frame&gt;::u16_to_an_endian::&lt;false&gt; (zune-image/src/frame.rs:413:35)
  &lt;zune_image::frame::Frame&gt;::u16_to_u8_endian_into (zune-image/src/frame.rs:507:40)
  repro_zune_image_src_frame_rs_413_35_567a68::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new_frames(vec![], BitDepth::Sixteen, 0usize, 0usize,
    ColorSpace::HSL);
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::execute_impl(&amp;ColorspaceConv::new(
    ColorSpace::RGBA), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/colorspace/conversion_functions.rs:16:43:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[&amp;zune_image::channel::Channel]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[&amp;zune_image::channel::Channel] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  zune_image::core_filters::colorspace::conversion_functions::convert_adding_opaque_alpha (zune-image/src/core_filters/colorspace/conversion_functions.rs:16:43)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:75:37)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute (zune-image/src/traits.rs:199:14)
  &lt;zune_image::image::Image&gt;::convert_color (zune-image/src/image.rs:691:33)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:165:23)
  repro_zune_image_src_core_filters_colorspace_conversion_functions__b96bf8::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![66u8];
let mut deflate_encoder = DeflateEncoder::new(&amp;vec);
deflate_encoder.encode_deflate();
let _ = deflate_encoder.encode_zlib();</pre></td><td valign="top"><pre>thread 'main' panicked at zune-inflate/src/encoder.rs:203:44:
range end index 2 out of range for slice of length 1
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_inflate::encoder::DeflateEncoder&gt;::encode_no_compression (zune-inflate/src/encoder.rs:203:44)
  &lt;zune_inflate::encoder::DeflateEncoder&gt;::encode_deflate (zune-inflate/src/encoder.rs:217:22)
  &lt;zune_inflate::encoder::DeflateEncoder&gt;::encode_zlib (zune-inflate/src/encoder.rs:229:14)
  repro_zune_inflate_src_encoder_rs_203_44_942e3a::main (src/main.rs:7:29)</pre></td></tr>
<tr><td valign="top"><pre>let image = Image::new(vec![Channel::new_with_bit_type(10000usize, BitType::U8),
    Channel::new_with_bit_type(0usize, BitType::F32)], BitDepth::Eight, 0usize,
    0usize, ColorSpace::ARGB);
assert!(image.clone() == image, "clone is not equal to the original");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:9:5:
clone is not equal to the original
backtrace:
  repro_contract_cloneeq_for_zune_image_image_image_ca0b2b::main (src/main.rs:9:5)</pre></td></tr>
<tr><td valign="top"><pre>let frame = Frame::from_u16(&amp;[0u16, 0u16], ColorSpace::LumaA, 0usize, 0usize);
let _ = frame.channels_ref(ColorSpace::BGRA, true);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/frame.rs:275:59:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;zune_image::frame::Frame&gt;::channels_ref (zune-image/src/frame.rs:275:59)
  repro_zune_image_src_frame_rs_275_59_7d5fa1::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let image = Image::new_frames(vec![], BitDepth::Unknown, 0usize, 0usize,
    ColorSpace::LumaA);
let _ = image.pixels::&lt;f32&gt;();</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/iters.rs:134:33:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[zune_image::frame::Frame]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[zune_image::frame::Frame] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::image::Image&gt;::pixels::&lt;f32&gt; (zune-image/src/iters.rs:134:33)
  repro_zune_image_src_iters_rs_134_33_8849a8::main (src/main.rs:7:19)</pre></td></tr>
<tr><td valign="top"><pre>let vec = include_bytes!("data_0.bin").to_vec();
let mut deflate_encoder = DeflateEncoder::new(&amp;vec);
deflate_encoder.encode_deflate();
deflate_encoder.encode_deflate();</pre></td><td valign="top"><pre>thread 'main' panicked at zune-inflate/src/encoder.rs:202:24:
range end index 2040 out of range for slice of length 2039
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;zune_inflate::encoder::DeflateEncoder&gt;::encode_no_compression (zune-inflate/src/encoder.rs:202:24)
  &lt;zune_inflate::encoder::DeflateEncoder&gt;::encode_deflate (zune-inflate/src/encoder.rs:217:22)
  repro_zune_inflate_src_encoder_rs_202_24_e81c63::main (src/main.rs:7:21)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![83u8];
let mut deflate_encoder = DeflateEncoder::new(&amp;vec);
let _ = deflate_encoder.encode_zlib();
deflate_encoder.encode_deflate();</pre></td><td valign="top"><pre>thread 'main' panicked at zune-inflate/src/encoder.rs:186:24:
index out of bounds: the len is 0 but the index is 12
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u8]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[u8] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;zune_inflate::encoder::DeflateEncoder&gt;::encode_no_compression (zune-inflate/src/encoder.rs:186:24)
  &lt;zune_inflate::encoder::DeflateEncoder&gt;::encode_deflate (zune-inflate/src/encoder.rs:217:22)
  repro_zune_inflate_src_encoder_rs_186_24_0acf64::main (src/main.rs:7:21)</pre></td></tr>
<tr><td valign="top"><pre>let frame = Frame::new_with_duration(vec![Channel::new_with_bit_type(10000usize,
    BitType::F32), Channel::new_with_bit_type(0usize, BitType::U16)], 0usize,
    0usize);
assert!(frame.clone() == frame, "clone is not equal to the original");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:7:5:
clone is not equal to the original
backtrace:
  repro_contract_cloneeq_for_zune_image_frame_frame_65e1b6::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>let bit_type = BitType::F32;
let mut frame = Frame::new_with_duration(vec![], 0usize, 0usize);
frame.insert(1usize, Channel::new_with_bit_type(0usize, bit_type));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/frame.rs:322:23:
insertion index (is 1) should be &lt;= len (is 0)
backtrace:
  &lt;zune_image::frame::Frame&gt;::insert (zune-image/src/frame.rs:322:23)
  repro_zune_image_src_frame_rs_322_23_fd3fdd::main (src/main.rs:8:11)</pre></td></tr>
<tr><td valign="top"><pre>let mut frame = Frame::new(vec![]);
let _ = frame.channels_mut(ColorSpace::RGB, false);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/frame.rs:300:31:
range end index 3 out of range for slice of length 0
backtrace:
  &lt;[zune_image::channel::Channel] as core::ops::index::IndexMut&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;zune_image::frame::Frame&gt;::channels_mut (zune-image/src/frame.rs:300:31)
  repro_zune_image_src_frame_rs_300_31_f71aec::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut frame = Frame::from_u8(&amp;include_bytes!("data_0.bin").to_vec(),
    ColorSpace::RGB, 0usize, 0usize);
frame.set_channels(vec![Channel::new_with_bit_type(4294967296usize,
    BitType::F32)]);</pre></td><td valign="top"><pre>thread 'main' panicked at alloc/src/alloc.rs:168:30:
unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null
This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
backtrace:
  &lt;zune_image::channel::Channel&gt;::dealloc (zune-image/src/channel.rs:259:18)
  &lt;zune_image::channel::Channel as core::ops::drop::Drop&gt;::drop (zune-image/src/channel.rs:657:18)
  &lt;*mut [zune_image::channel::Channel]&gt;::drop_in_place (core/src/ptr/mut_ptr.rs:1384:18)
  repro_zune_image_src_channel_rs_259_18_ea97d9::main (src/main.rs:9:1)</pre></td></tr>
<tr><td valign="top"><pre>let mut zune_webp_image_encoder = ZuneWebpImageEncoder::default();
let _ = &lt;zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut zune_webp_image_encoder,
    &amp;Image::new_frames(vec![],
    &lt;zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::
    EncoderTrait&gt;::default_depth(&amp;JpegEncoder::new(), BitDepth::Unknown),
    0usize, 0usize,
    &lt;zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait&gt;::
    default_colorspace(&amp;HdrEncoder::default(), ColorSpace::YCbCr)));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/codecs/webp.rs:102:34:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[alloc::vec::Vec&lt;u8&gt;]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[alloc::vec::Vec&lt;u8&gt;] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::EncoderTrait&gt;::encode_inner::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/codecs/webp.rs:102:34)
  &lt;zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::EncoderTrait&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/traits.rs:397:18)
  &lt;zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::EncoderTrait&gt;::encode_to_result (zune-image/src/traits.rs:420:25)
  repro_zune_image_src_codecs_webp_rs_102_34_5ce10a::main (src/main.rs:10:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new(vec![], ImageMetadata::default().depth(), 0usize,
    0usize, ColorSpace::BGR);
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::execute_impl(&amp;ColorspaceConv::new(
    ColorSpace::Unknown), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/colorspace/conversion_functions.rs:174:24:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;[zune_image::channel::Channel]&gt;::swap (core/src/slice/mod.rs:910:18)
  zune_image::core_filters::colorspace::conversion_functions::convert_rgb_bgr (zune-image/src/core_filters/colorspace/conversion_functions.rs:174:24)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:139:17)
  repro_zune_image_src_core_filters_colorspace_conversion_functions__68e829::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let bit_depth = &lt;zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::
    traits::EncoderTrait&gt;::default_depth(&amp;ZuneWebpImageEncoder::new(),
    ImageMetadata::default().depth());
let mut qoi_encoder = QoiEncoder::new();
let _ = &lt;zune_image::codecs::qoi::QoiEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut qoi_encoder, &amp;Image::new_frames(vec![],
    bit_depth, 1usize, 0usize, ColorSpace::HSV));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/codecs/qoi.rs:110:34:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[alloc::vec::Vec&lt;u8&gt;]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[alloc::vec::Vec&lt;u8&gt;] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait&gt;::encode_inner::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/codecs/qoi.rs:110:34)
  &lt;zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/traits.rs:395:18)
  &lt;zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait&gt;::encode_to_result (zune-image/src/traits.rs:420:25)
  repro_zune_image_src_codecs_qoi_rs_110_34_8a4e8a::main (src/main.rs:10:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut hdr_encoder = HdrEncoder::new();
let _ = &lt;zune_image::codecs::hdr::HdrEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut hdr_encoder, &amp;Image::fill::&lt;u8&gt;(0u8,
    ColorSpace::ARGB, 0usize, 1usize));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-hdr/src/encoder.rs:177:35:
chunk size must be non-zero
backtrace:
  &lt;[f32]&gt;::chunks_exact (core/src/slice/mod.rs:1242:9)
  &lt;zune_hdr::encoder::HdrEncoder&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-hdr/src/encoder.rs:177:35)
  &lt;zune_image::codecs (zune-image/src/codecs/hdr.rs:103:14)
  &lt;zune_image::codecs (zune-image/src/traits.rs:395:18)
  &lt;zune_image::codecs (zune-image/src/traits.rs:420:25)
  repro_zune_hdr_src_encoder_rs_177_35_858435::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut jpeg_encoder = JpegEncoder::new();
let _ = &lt;zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut jpeg_encoder,
    &amp;Image::new_frames(vec![], BitDepth::Sixteen, 0usize, 0usize,
    ColorSpace::ARGB));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/codecs/jpeg.rs:172:51:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[alloc::vec::Vec&lt;u8&gt;]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[alloc::vec::Vec&lt;u8&gt;] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::EncoderTrait&gt;::encode_inner::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/codecs/jpeg.rs:172:51)
  &lt;zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::EncoderTrait&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/traits.rs:395:18)
  &lt;zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::EncoderTrait&gt;::encode_to_result (zune-image/src/traits.rs:420:25)
  repro_zune_image_src_codecs_jpeg_rs_172_51_8d1503::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let usize_v = MAX_CHANNELS;
let _ = Image::fill::&lt;u16&gt;(0u16,
    ColorSpace::MultiBand(std::num::NonZero::new(10000u32).unwrap()), usize_v,
    65536usize);</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/ptr/mod.rs:652:9:
unsafe precondition(s) violated: ptr::copy requires that both pointer arguments are aligned and non-null
This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.</pre></td></tr>
<tr><td valign="top"><pre>let color_space = ColorSpace::BGRA;
let channel = Channel::new_with_bit_type(0usize, BitType::U8);
let usize_v = channel.len();
let bit_depth = BitDepth::Eight;
let mut farb_feld_encoder = FarbFeldEncoder::new();
let _ = &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut farb_feld_encoder,
    &amp;Image::new_frames(vec![], bit_depth, 1usize, usize_v, color_space));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/codecs/farbfeld.rs:118:34:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[alloc::vec::Vec&lt;u8&gt;]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[alloc::vec::Vec&lt;u8&gt;] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait&gt;::encode_inner::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/codecs/farbfeld.rs:118:34)
  &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/traits.rs:395:18)
  &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait&gt;::encode_to_result (zune-image/src/traits.rs:420:25)
  repro_zune_image_src_codecs_farbfeld_rs_118_34_f9c9a6::main (src/main.rs:14:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut frame = Frame::from_u16(&amp;[1u16, 0u16, 0u16], ColorSpace::HSL, 0usize,
    0usize);
let _ = frame.channels_mut(ColorSpace::LumaA, true);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/frame.rs:298:59:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;zune_image::frame::Frame&gt;::channels_mut (zune-image/src/frame.rs:298:59)
  repro_zune_image_src_frame_rs_298_59_d44e0d::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let mut deflate_encoder = DeflateEncoder::new(&amp;vec);
let _ = deflate_encoder.encode_zlib();
deflate_encoder.encode_deflate();</pre></td><td valign="top"><pre>thread 'main' panicked at zune-inflate/src/encoder.rs:165:24:
index out of bounds: the len is 0 but the index is 11
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u8]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[u8] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;zune_inflate::encoder::DeflateEncoder&gt;::encode_no_compression (zune-inflate/src/encoder.rs:165:24)
  &lt;zune_inflate::encoder::DeflateEncoder&gt;::encode_deflate (zune-inflate/src/encoder.rs:217:22)
  repro_zune_inflate_src_encoder_rs_165_24_b4c163::main (src/main.rs:7:21)</pre></td></tr>
<tr><td valign="top"><pre>let image = Image::fill::&lt;u16&gt;(0u16,
    ColorSpace::MultiBand(std::num::NonZero::new(1000u32).unwrap()), 0usize,
    0usize);
let _ = image.pixels::&lt;u16&gt;();</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/iters.rs:138:13:
index out of bounds: the len is 4 but the index is 4
backtrace:
  &lt;zune_image::image::Image&gt;::pixels::&lt;u16&gt; (zune-image/src/iters.rs:138:13)
  repro_zune_image_src_iters_rs_138_13_43fb61::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let color_space = ColorSpace::RGBA;
let _ = Image::fill::&lt;u16&gt;(0u16, color_space, 1usize, 4294967295usize);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/channel.rs:524:34:
unsafe precondition(s) violated: slice::from_raw_parts_mut requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`
This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
backtrace:
  &lt;zune_image::channel::Channel&gt;::reinterpret_as_mut::&lt;u16&gt; (zune-image/src/channel.rs:524:34)
  &lt;zune_image::channel::Channel&gt;::fill::&lt;u16&gt; (zune-image/src/channel.rs:600:26)
  &lt;zune_image::channel::Channel&gt;::from_elm::&lt;u16&gt; (zune-image/src/channel.rs:413:18)
  &lt;zune_image::image::Image&gt;::fill::&lt;u16&gt; (zune-image/src/image.rs:342:29)
  repro_zune_image_src_channel_rs_524_34_d271f2::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = zune_image::utils::swizzle_channels::&lt;u8&gt;(&amp;[Channel::new_with_bit_type(
    0usize, BitType::U8)], &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/utils.rs:149:20:
copy_from_slice: source slice length (0) does not match destination slice length (1)
backtrace:
  &lt;[u8]&gt;::copy_from_slice (core/src/slice/mod.rs:4344:18)
  zune_image::utils::swizzle_channels::&lt;u8&gt; (zune-image/src/utils.rs:149:20)
  repro_zune_image_src_utils_rs_149_20_1292d3::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut hdr_encoder = HdrEncoder::default();
let _ = &lt;zune_image::codecs::hdr::HdrEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut hdr_encoder, &amp;Image::fill::&lt;u16&gt;(1u16,
    ColorSpace::ARGB, 0usize, 0usize));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-core/src/options/encoder.rs:121:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;zune_core::options::encoder::EncoderOptions&gt; (zune-core/src/options/encoder.rs:121:9)
  &lt;zune_hdr::encoder::HdrEncoder&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-hdr/src/encoder.rs:133:39)
  &lt;zune_image::codecs (zune-image/src/codecs/hdr.rs:103:14)
  &lt;zune_image::codecs (zune-image/src/traits.rs:395:18)
  &lt;zune_image::codecs (zune-image/src/traits.rs:420:25)
  repro_zune_core_src_options_encoder_rs_121_9_75bd4a::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut hdr_encoder = HdrEncoder::default();
let _ = &lt;zune_image::codecs::hdr::HdrEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut hdr_encoder,
    &amp;Image::fill::&lt;f32&gt;(0.0f32, ColorSpace::Luma, 1usize, 1usize));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/frame.rs:354:17:
assertion `left == right` failed
  left: 3
 right: 12
backtrace:
  &lt;zune_image::frame::Frame&gt;::flatten::&lt;f32&gt; (zune-image/src/frame.rs:354:17)
  &lt;zune_image::image::Image&gt;::flatten_frames::&lt;f32&gt;::{closure#0} (zune-image/src/image.rs:189:44)
  &lt;zune_image::image::Image&gt;::flatten_frames::&lt;f32&gt; (zune-image/src/image.rs:189:55)
  &lt;zune_image::codecs (zune-image/src/codecs/hdr.rs:98:27)
  &lt;zune_image::codecs (zune-image/src/traits.rs:395:18)
  &lt;zune_image::codecs (zune-image/src/traits.rs:420:25)
  repro_zune_image_src_frame_rs_354_17_982464::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let channel = Channel::new_with_bit_type(0usize, BitType::U8);
let _ = zune_image::utils::swizzle_channels::&lt;u8&gt;(&amp;[channel.clone(),
    channel.clone(), Channel::new_with_bit_type(0usize, BitType::U8),
    channel.clone(), channel.clone()], &amp;mut vec![0u8, 80u8, 78u8, 71u8, 0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/utils.rs:206:30:
index out of bounds: the len is 0 but the index is 0
backtrace:
  zune_image::utils::swizzle_channels::&lt;u8&gt; (zune-image/src/utils.rs:206:30)
  repro_zune_image_src_utils_rs_206_30_c950c9::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut farb_feld_encoder = FarbFeldEncoder::new();
let _ = &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut farb_feld_encoder,
    &amp;Image::new_frames(vec![], BitDepth::Unknown, 0usize, 0usize,
    ColorSpace::YCbCr));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-core/src/bit_depth.rs:171:30:
Unknown bit type
backtrace:
  &lt;zune_core::bit_depth::BitDepth&gt;::size_of (zune-core/src/bit_depth.rs:171:30)
  zune_image::traits::confirm_invariants (zune-image/src/traits.rs:270:41)
  &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/traits.rs:344:9)
  &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait&gt;::encode_to_result (zune-image/src/traits.rs:420:25)
  repro_zune_core_src_bit_depth_rs_171_30_bc4870::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let color_space = ColorSpace::MultiBand(std::num::NonZero::new(1u32).unwrap());
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::clone_and_execute(&amp;ColorspaceConv::new(
    color_space), &amp;Image::new(vec![Channel::new_with_bit_type(2147483648usize,
    BitType::U8), Channel::new_with_bit_type(0usize, BitType::F32)],
    BitDepth::Float32, 0usize, 0usize, color_space));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/channel.rs:640:18:
unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`
This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
backtrace:
  &lt;zune_image::channel::Channel&gt;::alias (zune-image/src/channel.rs:640:18)
  &lt;zune_image::frame::Frame as core::clone::Clone&gt;::clone (zune-image/src/frame.rs:66:59)
  &lt;zune_image::frame::Frame as &lt;[_]&gt;::to_vec_in::ConvertVec&gt;::to_vec::&lt;alloc::alloc::Global&gt; (alloc/src/slice.rs:432:38)
  &lt;[zune_image::frame::Frame]&gt;::to_vec_in::&lt;alloc::alloc::Global&gt; (alloc/src/slice.rs:400:16)
  &lt;zune_image::image::Image as core::clone::Clone&gt;::clone (zune-image/src/image.rs:39:5)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::clone_and_execute (zune-image/src/traits.rs:224:31)
  repro_zune_image_src_channel_rs_640_18_bf9b62::main (src/main.rs:10:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut jxl_encoder = JxlEncoder::new();
let _ = &lt;zune_image::codecs::jpeg_xl::JxlEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut jxl_encoder, &amp;Image::new_frames(vec![],
    BitDepth::Float32, 0usize, 0usize, ColorSpace::RGB));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/codecs/jpeg_xl.rs:74:34:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[alloc::vec::Vec&lt;u8&gt;]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[alloc::vec::Vec&lt;u8&gt;] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::codecs::jpeg_xl::JxlEncoder as zune_image::traits::EncoderTrait&gt;::encode_inner::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/codecs/jpeg_xl.rs:74:34)
  &lt;zune_image::codecs::jpeg_xl::JxlEncoder as zune_image::traits::EncoderTrait&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/traits.rs:395:18)
  &lt;zune_image::codecs::jpeg_xl::JxlEncoder as zune_image::traits::EncoderTrait&gt;::encode_to_result (zune-image/src/traits.rs:420:25)
  repro_zune_image_src_codecs_jpeg_xl_rs_74_34_d24ff8::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new(vec![], BitDepth::Sixteen, 0usize, 0usize,
    &lt;zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::
    EncoderTrait&gt;::default_colorspace(&amp;ZuneWebpImageEncoder::default(),
    ColorSpace::RGBA));
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::execute_impl(&amp;ColorspaceConv::new(
    ColorSpace::CMYK), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/colorspace/conversion_functions.rs:200:9:
assertion `left == right` failed
  left: 0
 right: 3
backtrace:
  zune_image::core_filters::colorspace::conversion_functions::convert_rgb_to_cmyk (zune-image/src/core_filters/colorspace/conversion_functions.rs:200:9)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:78:37)
  repro_zune_image_src_core_filters_colorspace_conversion_functions__7cd46b::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new(vec![], BitDepth::Eight, 0usize, 0usize,
    ColorSpace::CMYK);
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::execute_impl(&amp;ColorspaceConv::new(
    ColorSpace::Unknown), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/colorspace/conversion_functions.rs:260:9:
assertion `left == right` failed
  left: 0
 right: 4
backtrace:
  zune_image::core_filters::colorspace::conversion_functions::convert_cmyk_to_rgb (zune-image/src/core_filters/colorspace/conversion_functions.rs:260:9)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:132:17)
  repro_zune_image_src_core_filters_colorspace_conversion_functions__6a9957::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new_frames(vec![],
    &lt;zune_image::codecs::ppm::PPMEncoder as zune_image::traits::EncoderTrait&gt;::
    default_depth(&amp;PPMEncoder::new(), BitDepth::Unknown), 0usize, 0usize,
    ColorSpace::HSL);
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::execute(&amp;ColorspaceConv::new(ColorSpace::LumaA),
    &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/colorspace/conversion_functions.rs:164:5:
assertion `left != right` failed
  left: Unknown
 right: Unknown
backtrace:
  zune_image::core_filters::colorspace::conversion_functions::convert_rgb_to_grayscale (zune-image/src/core_filters/colorspace/conversion_functions.rs:164:5)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:77:38)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute (zune-image/src/traits.rs:199:14)
  &lt;zune_image::image::Image&gt;::convert_color (zune-image/src/image.rs:691:33)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:165:23)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute (zune-image/src/traits.rs:199:14)
  repro_zune_image_src_core_filters_colorspace_conversion_functions__7f73b0::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut hdr_encoder = HdrEncoder::new();
let _ = &lt;zune_image::codecs::hdr::HdrEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut hdr_encoder, &amp;Image::new_frames(vec![],
    &lt;zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::
    EncoderTrait&gt;::default_depth(&amp;ZuneWebpImageEncoder::default(),
    ImageMetadata::default().depth()), 0usize, 0usize, ColorSpace::ARGB));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/codecs/hdr.rs:98:43:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[alloc::vec::Vec&lt;f32&gt;]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[alloc::vec::Vec&lt;f32&gt;] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::codecs (zune-image/src/codecs/hdr.rs:98:43)
  &lt;zune_image::codecs (zune-image/src/traits.rs:395:18)
  &lt;zune_image::codecs (zune-image/src/traits.rs:420:25)
  repro_zune_image_src_codecs_hdr_rs_98_43_000c57::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut p_p_m_encoder = PPMEncoder::new();
let _ = &lt;zune_image::codecs::ppm::PPMEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut p_p_m_encoder,
    &amp;Image::new_frames(vec![], BitDepth::Float32, 0usize, 0usize,
    ColorSpace::RGBA));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/codecs/ppm.rs:50:34:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[alloc::vec::Vec&lt;u8&gt;]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[alloc::vec::Vec&lt;u8&gt;] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;zune_image::codecs::ppm::PPMEncoder as zune_image::traits::EncoderTrait&gt;::encode_inner::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/codecs/ppm.rs:50:34)
  &lt;zune_image::codecs::ppm::PPMEncoder as zune_image::traits::EncoderTrait&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/traits.rs:395:18)
  &lt;zune_image::codecs::ppm::PPMEncoder as zune_image::traits::EncoderTrait&gt;::encode_to_result (zune-image/src/traits.rs:420:25)
  repro_zune_image_src_codecs_ppm_rs_50_34_bce188::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new(vec![], BitDepth::Float32, 0usize, 1usize,
    &lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt; as zune_image::traits::DecoderTrait&gt;::out_colorspace(
    &amp;&lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::
    new(std::io::Cursor::new(vec![82u8, 73u8, 70u8, 70u8, 26u8, 0u8, 0u8, 0u8,
    87u8, 69u8, 66u8, 80u8, 86u8, 80u8, 56u8, 76u8, 13u8, 0u8, 0u8, 0u8, 47u8,
    0u8, 0u8, 0u8, 16u8])).unwrap()));
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::execute_impl(&amp;ColorspaceConv::new(
    ColorSpace::RGB), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/colorspace/conversion_functions.rs:399:36:
called `Option::unwrap()` on a `None` value
backtrace:
  zune_image::core_filters::colorspace::conversion_functions::pop_channel (zune-image/src/core_filters/colorspace/conversion_functions.rs:399:36)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:90:36)
  repro_zune_image_src_core_filters_colorspace_conversion_functions__a1d03b::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::fill::&lt;f32&gt;(0.0f32, ColorSpace::RGBA, 0usize, 0usize);
image.set_depth(BitDepth::Sixteen);
let mut farb_feld_encoder = FarbFeldEncoder::new();
let _ = &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::
    EncoderTrait&gt;::encode_to_result(&amp;mut farb_feld_encoder, &amp;image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/image.rs:205:65:
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x1f814bf16f812452b283244dea52bee6) from expected TypeId(0x20eff0baf1c4a6b94cfebd6565ec4c14). This indicates you are converting a channel
             to a type it wasn't instantiated with
backtrace:
  &lt;zune_image::image::Image&gt;::to_u8::{closure#0} (zune-image/src/image.rs:205:65)
  &lt;zune_image::image::Image&gt;::to_u8 (zune-image/src/image.rs:206:18)
  &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait&gt;::encode_inner::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/codecs/farbfeld.rs:118:27)
  &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait&gt;::encode::&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt; (zune-image/src/traits.rs:397:18)
  &lt;zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait&gt;::encode_to_result (zune-image/src/traits.rs:420:25)
  repro_zune_image_src_image_rs_205_65_d5c4f1::main (src/main.rs:10:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;zune_image::image::Image as ::core::ops::Add&lt;zune_image::image::
    Image&gt;&gt;::add(
    &lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt; as zune_image::traits::DecoderTrait&gt;::decode(
    &amp;mut &lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::new(std::io::Cursor::new(include_bytes!(
    "data_0.bin").to_vec())).unwrap()).unwrap(),
    &lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt; as zune_image::traits::DecoderTrait&gt;::decode(
    &amp;mut &lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::new(std::io::Cursor::new(include_bytes!(
    "data_1.bin").to_vec())).unwrap()).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/ops.rs:53:9:
assertion `left == right` failed: Dimensions mismatch, (1, 1) != (1, 3)
  left: (1, 1)
 right: (1, 3)
backtrace:
  zune_image::ops::check_full_compatibility (zune-image/src/ops.rs:32:5)
  &lt;zune_image::image::Image as core::ops::arith::Add&gt;::add (zune-image/src/ops.rs:53:9)
  repro_zune_image_src_ops_rs_53_9_1cb364::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;zune_image::image::Image as ::core::ops::Sub&lt;zune_image::image::
    Image&gt;&gt;::sub(Image::from_u8(&amp;vec![], 1usize, usize::MAX, ColorSpace::YCCK),
    &lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt; as zune_image::traits::DecoderTrait&gt;::decode(
    &amp;mut &lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::new(std::io::Cursor::new(include_bytes!(
    "png.json").to_vec())).unwrap()).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/image.rs:711:10:
called `Option::unwrap()` on a `None` value
backtrace:
  zune_image::image::checked_mul (zune-image/src/image.rs:711:10)
  &lt;zune_image::image::Image&gt;::from_u8 (zune-image/src/image.rs:481:28)
  repro_zune_image_src_image_rs_711_10_cf99f3::main (src/main.rs:5:91)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new(vec![], BitDepth::Unknown, 0usize, 0usize,
    ColorSpace::ARGB);
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::execute_impl(&amp;ColorspaceConv::new(
    ColorSpace::YCCK), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/colorspace/conversion_functions.rs:421:52:
called `Option::unwrap()` on a `None` value
backtrace:
  zune_image::core_filters::colorspace::conversion_functions::convert_rgba_to_argb_or_vice_versa (zune-image/src/core_filters/colorspace/conversion_functions.rs:421:52)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:156:17)
  repro_zune_image_src_core_filters_colorspace_conversion_functions__5880df::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new(vec![Channel::new_with_bit_type(0usize,
    BitType::U16), Channel::new_with_bit_type(0usize, BitType::U8),
    Channel::new_with_bit_type(0usize, BitType::F32)], BitDepth::Eight, 0usize,
    0usize,
    &lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt; as zune_image::traits::DecoderTrait&gt;::out_colorspace(
    &amp;&lt;zune_image::codecs::webp::ZuneWebpDecoder&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::
    new(std::io::Cursor::new(vec![82u8, 73u8, 70u8, 70u8, 0u8, 0u8, 0u8, 0u8,
    87u8, 69u8, 66u8, 80u8, 86u8, 80u8, 56u8, 76u8, 0u8, 0u8, 0u8, 0u8, 47u8,
    0u8, 0u8, 0u8, 0u8])).unwrap()));
let _ = &lt;zune_image::core_filters::depth::Depth as zune_image::traits::
    OperationsTrait&gt;::execute_impl(&amp;Depth::new(BitDepth::Sixteen), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/depth.rs:120:61:
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x20eff0baf1c4a6b94cfebd6565ec4c14) from expected TypeId(0xcfb1622e7734f8af272d8d182905dc25). This indicates you are converting a channel
             to a type it wasn't instantiated with
backtrace:
  &lt;zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/depth.rs:120:61)
  repro_zune_image_src_core_filters_depth_rs_120_61_37b19d::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new(vec![], ImageMetadata::default().depth(), 0usize,
    1usize, ColorSpace::ARGB);
let _ = &lt;zune_image::core_filters::depth::Depth as zune_image::traits::
    OperationsTrait&gt;::execute(&amp;Depth::new(BitDepth::Float32), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-core/src/bit_depth.rs:148:30:
Unknown bit type
backtrace:
  &lt;zune_core::bit_depth::BitDepth&gt;::bit_type (zune-core/src/bit_depth.rs:148:30)
  &lt;zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait&gt;::execute (zune-image/src/traits.rs:187:47)
  repro_zune_core_src_bit_depth_rs_148_30_5a3dce::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Image::fill::&lt;f32&gt;(0.0f32, ColorSpace::BGR, 11usize, 16777217usize);</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/ptr/mod.rs:652:9:
unsafe precondition(s) violated: ptr::copy requires that both pointer arguments are aligned and non-null
This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
backtrace:
  &lt;*mut u8&gt;::copy_from (core/src/ptr/mut_ptr.rs:1348:18)
  &lt;zune_image::channel::Channel&gt;::extend_unchecked::&lt;u8&gt; (zune-image/src/channel.rs:461:48)
  &lt;zune_image::channel::Channel as core::clone::Clone&gt;::clone (zune-image/src/channel.rs:126:25)
  &lt;zune_image::channel::Channel as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:15:11)
  &lt;zune_image::image::Image&gt;::fill::&lt;f32&gt; (zune-image/src/image.rs:342:24)
  repro_zune_image_src_channel_rs_461_48_96f220::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;zune_image::image::Image as ::core::ops::Sub&lt;zune_image::image::
    Image&gt;&gt;::sub(Image::new_frames(vec![], BitDepth::Sixteen, 0usize, 0usize,
    ColorSpace::HSL), Image::new_frames(vec![], BitDepth::Sixteen, 0usize,
    1usize, ColorSpace::HSL));</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/ops.rs:113:9:
assertion `left == right` failed: Dimensions mismatch, (0, 0) != (0, 1)
  left: (0, 0)
 right: (0, 1)
backtrace:
  zune_image::ops::check_full_compatibility (zune-image/src/ops.rs:32:5)
  &lt;zune_image::image::Image as core::ops::arith::Sub&gt;::sub (zune-image/src/ops.rs:113:9)
  repro_zune_image_src_ops_rs_113_9_2b03c4::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let usize_v = MIN_ALIGNMENT;
let mut image = Image::new(vec![Channel::new_with_bit_type(usize_v,
    BitType::U16)], BitDepth::Float32, 0usize, 0usize, ColorSpace::Luma);
let _ = &lt;zune_image::core_filters::depth::Depth as zune_image::traits::
    OperationsTrait&gt;::execute_impl(&amp;Depth::new(
    &lt;zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait&gt;::
    default_depth(&amp;QoiEncoder::default(), BitDepth::Float32)), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/depth.rs:141:68:
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x20eff0baf1c4a6b94cfebd6565ec4c14) from expected TypeId(0x1f814bf16f812452b283244dea52bee6). This indicates you are converting a channel
             to a type it wasn't instantiated with
backtrace:
  &lt;zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/depth.rs:141:68)
  repro_zune_image_src_core_filters_depth_rs_141_68_34438b::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let color_space = &lt;zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::
    EncoderTrait&gt;::default_colorspace(&amp;JpegEncoder::default(),
    ColorSpace::BGRA);
let _ = Depth::new(Image::from_f32(&amp;[], usize::MAX, 2147483648usize,
    color_space).depth());</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/image.rs:707:10:
called `Option::unwrap()` on a `None` value
backtrace:
  zune_image::image::checked_mul (zune-image/src/image.rs:707:10)
  &lt;zune_image::image::Image&gt;::from_f32 (zune-image/src/image.rs:543:28)
  repro_zune_image_src_image_rs_707_10_5c2fe1::main (src/main.rs:8:24)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::new(vec![Channel::new_with_bit_type(0usize,
    BitType::F32), Channel::new_with_bit_type(0usize, BitType::U8),
    Channel::new_with_bit_type(0usize, BitType::U8),
    Channel::new_with_bit_type(0usize, BitType::F32)],
    ImageMetadata::default().depth(), 0usize, 0usize, ColorSpace::CMYK);
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::execute_impl(&amp;ColorspaceConv::new(
    ColorSpace::Unknown), &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/colorspace/conversion_functions.rs:287:18:
internal error: entered unreachable code
backtrace:
  zune_image::core_filters::colorspace::conversion_functions::convert_cmyk_to_rgb (zune-image/src/core_filters/colorspace/conversion_functions.rs:287:18)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:132:17)
  repro_zune_image_src_core_filters_colorspace_conversion_functions__2665d4::main (src/main.rs:10:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image = Image::fill::&lt;f32&gt;(0.0f32, ColorSpace::HSL, 0usize, 0usize);
image.set_depth(
    &lt;zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait&gt;::
    default_depth(&amp;QoiEncoder::default(), BitDepth::Float32));
let _ = &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::
    traits::OperationsTrait&gt;::execute(&amp;ColorspaceConv::new(ColorSpace::CMYK),
    &amp;mut image);</pre></td><td valign="top"><pre>thread 'main' panicked at zune-image/src/core_filters/depth.rs:167:67:
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x1f814bf16f812452b283244dea52bee6) from expected TypeId(0xcfb1622e7734f8af272d8d182905dc25). This indicates you are converting a channel
             to a type it wasn't instantiated with
backtrace:
  &lt;zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/depth.rs:167:67)
  &lt;zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait&gt;::execute (zune-image/src/traits.rs:199:14)
  &lt;zune_image::image::Image&gt;::convert_depth (zune-image/src/image.rs:698:24)
  zune_image::core_filters::colorspace::conversion_functions::convert_hsl_to_rgb (zune-image/src/core_filters/colorspace/conversion_functions.rs:329:11)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute_impl (zune-image/src/core_filters/colorspace.rs:162:17)
  &lt;zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait&gt;::execute (zune-image/src/traits.rs:199:14)
  repro_zune_image_src_core_filters_depth_rs_167_67_44bfdc::main (src/main.rs:10:13)</pre></td></tr>
</table>
