# image - crashes

Source: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgb32f(10u32, 0u32);
let dynamic_image2 = dynamic_image.crop(Rect { x: 1u32, y: 0u32, width: 1u32,
    height: 1u32 });
let _ = dynamic_image2.blur_advanced(GaussianBlurParameters::SMOOTHING_3);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/filter_1d.rs:80:59:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[[f32; 3]] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  image::imageops::filter_1d::make_arena_row::&lt;f32, 3&gt; (image/src/imageops/filter_1d.rs:80:59)
  image::imageops::filter_1d::filter_2d_separable_ring_queue::&lt;f32, f32, 3&gt; (image/src/imageops/filter_1d.rs:529:5)
  image::imageops::filter_1d::filter_2d_separable::&lt;f32, f32, f32, 3&gt; (image/src/imageops/filter_1d.rs:674:16)
  image::imageops::filter_1d::filter_2d_sep_rgb_f32 (image/src/imageops/filter_1d.rs:866:5)
  image::imageops::sample::gaussian_blur_dyn_image (image/src/imageops/sample.rs:1474:13)
  &lt;image::images::dynimage::DynamicImage&gt;::blur_advanced (image/src/images/dynimage.rs:1244:9)
  repro_image_src_imageops_filter_1d_rs_80_59_f88e79::main (src/main.rs:8:28)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::default().blur_advanced(
    GaussianBlurParameters::SMOOTHING_5);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/filter_1d.rs:83:50:
called `Option::unwrap()` on a `None` value
backtrace:
  image::imageops::filter_1d::make_arena_row::&lt;u8, 4&gt; (image/src/imageops/filter_1d.rs:83:50)
  image::imageops::filter_1d::filter_2d_separable_ring_queue::&lt;u8, u32, 4&gt; (image/src/imageops/filter_1d.rs:529:5)
  image::imageops::filter_1d::filter_2d_separable::&lt;u8, f32, u32, 4&gt; (image/src/imageops/filter_1d.rs:674:16)
  image::imageops::filter_1d::filter_2d_sep_rgba (image/src/imageops/filter_1d.rs:818:5)
  image::imageops::sample::gaussian_blur_dyn_image (image/src/imageops/sample.rs:1376:13)
  &lt;image::images::dynimage::DynamicImage&gt;::blur_advanced (image/src/images/dynimage.rs:1244:9)
  repro_image_src_imageops_filter_1d_rs_83_50_884a88::main (src/main.rs:5:37)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::new_luma16(1u32, 1u32).filter3x3(&amp;[0.0f32, 0.0f32, 0.0f32,
    0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, f32::NEG_INFINITY]);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/sample.rs:962:62:
called `Option::unwrap()` on a `None` value
backtrace:
  image::imageops::sample::filter3x3::&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u16&gt;, alloc::vec::Vec&lt;u16&gt;&gt;, image::color::Luma&lt;u16&gt;, u16&gt;::{closure#1} (image/src/imageops/sample.rs:962:62)
  &lt;&amp;mut core::array::drain::Drain&lt;f32, &lt;core::ops::try_trait::NeverShortCircuit&lt;u16&gt;&gt;::wrap_mut_1&lt;f32, image::imageops::sample::filter3x3&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u16&gt;, alloc::vec::Vec&lt;u16&gt;&gt;, image::color::Luma&lt;u16&gt;, u16&gt;::{closure#1}&gt;::{closure#0}&gt; as core::ops::functio... (+61 chars)
  &lt;[f32; 4]&gt;::try_map::&lt;core::ops::try_trait::NeverShortCircuit&lt;u16&gt;, &lt;core::ops::try_trait::NeverShortCircuit&lt;u16&gt;&gt;::wrap_mut_1&lt;f32, image::imageops::sample::filter3x3&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u16&gt;, alloc::vec::Vec&lt;u16&gt;&gt;, image::color::Luma&lt;u16&gt;, u16&gt;::{closure#1}&gt;::{cl... (+39 chars)
  &lt;[f32; 4]&gt;::map::&lt;image::imageops::sample::filter3x3&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u16&gt;, alloc::vec::Vec&lt;u16&gt;&gt;, image::color::Luma&lt;u16&gt;, u16&gt;::{closure#1}, u16&gt; (core/src/array/mod.rs:598:14)
  image::imageops::sample::filter3x3::&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u16&gt;, alloc::vec::Vec&lt;u16&gt;&gt;, image::color::Luma&lt;u16&gt;, u16&gt; (image/src/imageops/sample.rs:962:18)
  &lt;image::images::dynimage::DynamicImage&gt;::filter3x3 (image/src/images/dynimage.rs:1296:38)
  repro_image_src_imageops_sample_rs_962_62_f1c465::main (src/main.rs:4:50)</pre></td></tr>
<tr><td valign="top"><pre>let _ = GaussianBlurParameters::new_anisotropic_kernel_size(0.357f32, 1.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/sample.rs:1272:25:
attempt to subtract with overflow
backtrace:
  &lt;image::imageops::sample::GaussianBlurParameters&gt;::round_to_nearest_odd (image/src/imageops/sample.rs:1272:25)
  &lt;image::imageops::sample::GaussianBlurParameters&gt;::new_anisotropic_kernel_size (image/src/imageops/sample.rs:1207:29)
  repro_image_src_imageops_sample_rs_1272_25_a998ee::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut dynamic_image = DynamicImage::from(DynamicImage::new_luma16(0u32,
    0u32));
dynamic_image.resize_to_fill(0u32, 1u32, FilterType::Triangle);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/images/dynimage.rs:1144:70:
called `Result::unwrap()` on an `Err` value: "Image size must not be zero"
backtrace:
  &lt;image::images::dynimage::DynamicImage&gt;::resize_exact (image/src/images/dynimage.rs:1144:70)
  &lt;image::images::dynimage::DynamicImage&gt;::resize_to_fill (image/src/images/dynimage.rs:1191:14)
  repro_image_src_images_dynimage_rs_1144_70_d85153::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let sample_layout = SampleLayout { channels: 0u8, channel_stride: usize::MAX,
    width: 0u32, width_stride: 0usize, height: 0u32, height_stride: 1usize };
let _ = sample_layout.in_bounds_index(1u8, 0u32, 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/images/flat.rs:441:9:
attempt to add with overflow
backtrace:
  &lt;image::images::flat::SampleLayout&gt;::in_bounds_index (image/src/images/flat.rs:441:9)
  repro_image_src_images_flat_rs_441_9_0b1f1e::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::from(&lt;image::ImageBuffer&lt;image::Rgba&lt;f32&gt;,
    Vec&lt;f32&gt;&gt; as ::core::default::Default&gt;::default());
let dynamic_image2 = dynamic_image.flipv();
let _ = dynamic_image2.unsharpen(65536.0f32, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/filter_1d.rs:691:10:
chunk size must be non-zero
backtrace:
  &lt;[f32]&gt;::chunks_exact_mut (core/src/slice/mod.rs:1290:9)
  image::imageops::filter_1d::filter_2d_separable::&lt;f32, f32, f32, 4&gt; (image/src/imageops/filter_1d.rs:691:10)
  image::imageops::filter_1d::filter_2d_sep_rgba_f32 (image/src/imageops/filter_1d.rs:882:5)
  image::imageops::sample::gaussian_blur_indirect_impl::&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, 4&gt; (image/src/imageops/sample.rs:1599:13)
  image::imageops::sample::gaussian_blur_indirect::&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;&gt; (image/src/imageops/sample.rs:1515:14)
  image::imageops::sample::blur_advanced::&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;&gt; (image/src/imageops/sample.rs:1069:5)
  image::imageops::sample::unsharpen::&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, image::color::Rgba&lt;f32&gt;, f32&gt; (image/src/imageops/sample.rs:1638:19)
  &lt;image::images::dynimage::DynamicImage&gt;::unsharpen (image/src/images/dynimage.rs:1278:38)
  repro_image_src_imageops_filter_1d_rs_691_10_6dcebd::main (src/main.rs:6:28)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_luma16(1u32, 0u32).blur(16777216.0f32);
let _ = dynamic_image.as_rgb16();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::prepare_symmetric_kernel&lt;u32&gt; (src/imageops/filter_1d.rs:432)
  image::imageops::filter_1d::filter_2d_separable&lt;u16, f32, u32, 1&gt; (src/imageops/filter_1d.rs:665)
  image::imageops::filter_1d::filter_2d_sep_plane_u16 (src/imageops/filter_1d.rs:946)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1390)
  image::images::dynimage::DynamicImage::blur (src/images/dynimage.rs:1227)
  repro_timeout_in_image_dynamicimage_as_rgb16_637a1c::main (src/main.rs:4)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;image::codecs::gif::GifEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new_with_speed(vec![], 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/gif.rs:479:9:
speed needs to be in the range [1, 30]
backtrace:
  &lt;image::codecs::gif::GifEncoder&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::new_with_speed (image/src/codecs/gif.rs:479:9)
  repro_image_src_codecs_gif_rs_479_9_2da710::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let _ = &lt;image::Rgba&lt;u8&gt; as image::Pixel&gt;::from_slice(&amp;vec);</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:3:13:
assertion `left == right` failed
  left: 0
 right: 4
backtrace:
  &lt;image::color::Rgba&lt;u8&gt; as image::traits::Pixel&gt;::from_slice (image/src/color.rs:390:9)
  repro_image_src_color_rs_390_9_7c1dc5::main (src/main.rs:3:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;image::Rgb&lt;f32&gt; as image::Pixel&gt;::from_slice_mut(&amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:2:13:
assertion `left == right` failed
  left: 0
 right: 3
backtrace:
  &lt;image::color::Rgb&lt;f32&gt; as image::traits::Pixel&gt;::from_slice_mut (image/src/color.rs:396:9)
  repro_image_src_color_rs_396_9_a63a32::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut tiff_decoder = &lt;image::codecs::tiff::TiffDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::new(std::io::Cursor::new(include_bytes!(
    "data_0.bin").to_vec())).unwrap();
let _ = &lt;image::codecs::tiff::TiffDecoder&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt; as image::
    ImageDecoder&gt;::read_image(&amp;mut tiff_decoder, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/tiff.rs:464:9:
assertion `left == right` failed
  left: Ok(0)
 right: Ok(2)
backtrace:
  &lt;image::codecs::tiff::TiffDecoder&lt;core::io::cursor::Cursor&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt; as image::io::decoder::ImageDecoder&gt;::read_image (image/src/codecs/tiff.rs:464:9)
  repro_image_src_codecs_tiff_rs_464_9_704faf::main (src/main.rs:3:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut png_decoder = &lt;image::codecs::png::PngDecoder&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::with_limits(std::io::Cursor::new(include_bytes!(
    "data_0.bin").to_vec()), Limits::no_limits());
let _ = &lt;image::codecs::png::PngDecoder&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt; as image::
    ImageDecoder&gt;::read_image(&amp;mut png_decoder, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/png.rs:374:9:
assertion `left == right` failed
  left: Ok(0)
 right: Ok(1)
backtrace:
  &lt;image::codecs::png::PngDecoder&lt;core::io::cursor::Cursor&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt; as image::io::decoder::ImageDecoder&gt;::read_image (image/src/codecs/png.rs:374:9)
  repro_image_src_codecs_png_rs_374_9_5cddb6::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Delay::from_saturating_duration(std::time::Duration::new(1000000000u64,
    1073741824u32 % 1_000_000_000));</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/animation.rs:212:35:
attempt to add with overflow
backtrace:
  &lt;image::animation::Delay&gt;::from_saturating_duration (image/src/animation.rs:212:35)
  repro_image_src_animation_rs_212_35_c92aab::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let hdr_encoder = &lt;image::codecs::hdr::HdrEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![]);
let _ = hdr_encoder.encode(&amp;[], 100usize, 1000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/hdr/encoder.rs:64:44:
attempt to multiply with overflow
backtrace:
  &lt;image::codecs (image/src/codecs/hdr/encoder.rs:64:44)
  &lt;image::codecs (image/src/codecs/hdr/encoder.rs:51:14)
  repro_image_src_codecs_hdr_encoder_rs_64_44_af8b35::main (src/main.rs:3:25)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;image::codecs::hdr::HdrEncoder&lt;Vec&lt;u8&gt;&gt; as image::ImageEncoder&gt;::
    write_image(&lt;image::codecs::hdr::HdrEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![]), &amp;vec![],
    1u32, 1u32, ExtendedColorType::Rgb32F);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/hdr/encoder.rs:63:9:
not enough pixels provided
backtrace:
  &lt;image::codecs (image/src/codecs/hdr/encoder.rs:63:9)
  &lt;image::codecs (image/src/codecs/hdr/encoder.rs:30:22)
  repro_image_src_codecs_hdr_encoder_rs_63_9_ac93cd::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut gif_encoder = &lt;image::codecs::gif::GifEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![]);
let _ = gif_encoder.encode(&amp;vec![], 1u32, 1u32, ExtendedColorType::Rgba8);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/gif.rs:515:57:
assertion `left == right` failed: Too much or too little pixel data for the given width and height to create a GIF Frame
  left: 4
 right: 0
backtrace:
  &lt;gif::common::Frame&gt;::from_rgba_speed (gif/src/common.rs:216:9)
  &lt;image::codecs::gif::GifEncoder&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::encode (image/src/codecs/gif.rs:515:57)
  repro_image_src_codecs_gif_rs_515_57_7a48f3::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_luma_a32f(1u32, 1u32);
let dynamic_image2 = dynamic_image.grayscale();
let dynamic_image3 = dynamic_image2.adjust_contrast(0.0f32);
let _ = dynamic_image3.fast_blur(f32::NEG_INFINITY);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;f32, f32, 2&gt; (src/imageops/fast_blur.rs:343)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::LumaA&lt;f32&gt;, f32&gt; (src/imageops/fast_blur.rs:263)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::LumaA&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:239)
  image::imageops::fast_blur::fast_blur&lt;image::color::LumaA&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_fast_blur_834cee::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let mut gif_encoder = &lt;image::codecs::gif::GifEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![]);
let _ = gif_encoder.encode(&amp;vec![], 1u32, 1u32, ExtendedColorType::L8);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/gif.rs:524:33:
assertion `left == right` failed: Too many or too little pixels for the given width and height to create a GIF Frame
  left: 1
 right: 0
backtrace:
  &lt;gif::common::Frame&gt;::from_palette_pixels::&lt;&amp;[u8], alloc::vec::Vec&lt;u8&gt;&gt; (gif/src/common.rs:394:9)
  &lt;image::codecs::gif::GifEncoder&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::encode (image/src/codecs/gif.rs:524:33)
  repro_image_src_codecs_gif_rs_524_33_44b37f::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::from(&lt;image::ImageBuffer&lt;image::Rgb&lt;f32&gt;,
    Vec&lt;f32&gt;&gt;&gt;::from_raw(1u32, 0u32, vec![]).unwrap());
let _ = dynamic_image.blur(409.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/filter_1d.rs:119:35:
range end index 3 out of range for slice of length 0
backtrace:
  &lt;[f32] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  image::imageops::filter_1d::make_columns_arenas::&lt;f32, 3&gt; (image/src/imageops/filter_1d.rs:119:35)
  image::imageops::filter_1d::filter_2d_separable::&lt;f32, f32, f32, 3&gt; (image/src/imageops/filter_1d.rs:728:9)
  image::imageops::filter_1d::filter_2d_sep_rgb_f32 (image/src/imageops/filter_1d.rs:866:5)
  image::imageops::sample::gaussian_blur_dyn_image (image/src/imageops/sample.rs:1474:13)
  &lt;image::images::dynimage::DynamicImage&gt;::blur (image/src/images/dynimage.rs:1227:9)
  repro_image_src_imageops_filter_1d_rs_119_35_edb970::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::new_luma_a8(10u32, 10000u32).thumbnail(0u32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/sample.rs:739:51:
attempt to divide by zero
backtrace:
  &lt;u32 as core::ops::arith::Div&gt;::div (core/src/ops/arith.rs:493:45)
  image::imageops::sample::thumbnail_sample_block::&lt;image::images::buffer::ImageBuffer&lt;image::color::LumaA&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::LumaA&lt;u8&gt;, u8&gt;::{closure#0} (image/src/imageops/sample.rs:739:51)
  &lt;&amp;mut core::array::drain::Drain&lt;u32, &lt;core::ops::try_trait::NeverShortCircuit&lt;u8&gt;&gt;::wrap_mut_1&lt;u32, image::imageops::sample::thumbnail_sample_block&lt;image::images::buffer::ImageBuffer&lt;image::color::LumaA&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::LumaA&lt;u8&gt;, u8&gt;::{closure#0}&gt;::{closure#0}&gt; as core::op... (+15 chars)
  &lt;[u32; 4]&gt;::try_map::&lt;core::ops::try_trait::NeverShortCircuit&lt;u8&gt;, &lt;core::ops::try_trait::NeverShortCircuit&lt;u8&gt;&gt;::wrap_mut_1&lt;u32, image::imageops::sample::thumbnail_sample_block&lt;image::images::buffer::ImageBuffer&lt;image::color::LumaA&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::LumaA&lt;u8&gt;, u8&gt;::{closure... (+15 chars)
  &lt;[u32; 4]&gt;::map::&lt;image::imageops::sample::thumbnail_sample_block&lt;image::images::buffer::ImageBuffer&lt;image::color::LumaA&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::LumaA&lt;u8&gt;, u8&gt;::{closure#0}, u8&gt; (core/src/array/mod.rs:598:14)
  image::imageops::sample::thumbnail_sample_block::&lt;image::images::buffer::ImageBuffer&lt;image::color::LumaA&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::LumaA&lt;u8&gt;, u8&gt; (image/src/imageops/sample.rs:739:29)
  image::imageops::sample::thumbnail::&lt;image::images::buffer::ImageBuffer&lt;image::color::LumaA&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::LumaA&lt;u8&gt;, u8&gt; (image/src/imageops/sample.rs:629:17)
  &lt;image::images::dynimage::DynamicImage&gt;::thumbnail_exact (image/src/images/dynimage.rs:1176:38)
  &lt;image::images::dynimage::DynamicImage&gt;::thumbnail (image/src/images/dynimage.rs:1162:14)
  repro_image_src_imageops_sample_rs_739_51_4bb987::main (src/main.rs:4:56)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::new_luma8(1u32, 1u32).adjust_contrast(f32::NAN);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/colorops.rs:110:30:
called `Option::unwrap()` on a `None` value
backtrace:
  image::imageops::colorops::contrast::&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::Luma&lt;u8&gt;, u8&gt;::{closure#0} (image/src/imageops/colorops.rs:110:30)
  &lt;image::color::Luma&lt;u8&gt; as image::traits::Pixel&gt;::apply::&lt;image::imageops::colorops::contrast&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::Luma&lt;u8&gt;, u8&gt;::{closure#0}&gt; (image/src/color.rs:450:18)
  &lt;image::color::Luma&lt;u8&gt; as image::traits::Pixel&gt;::map::&lt;image::imageops::colorops::contrast&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::Luma&lt;u8&gt;, u8&gt;::{closure#0}&gt; (image/src/traits.rs:514:14)
  image::imageops::colorops::contrast::&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u8&gt;, alloc::vec::Vec&lt;u8&gt;&gt;, image::color::Luma&lt;u8&gt;, u8&gt; (image/src/imageops/colorops.rs:104:23)
  &lt;image::images::dynimage::DynamicImage&gt;::adjust_contrast (image/src/images/dynimage.rs:1307:38)
  repro_image_src_imageops_colorops_rs_110_30_bdeb90::main (src/main.rs:4:49)</pre></td></tr>
<tr><td valign="top"><pre>let pnm_header = PnmHeader::from(BitmapHeader::from(BitmapHeader::from(
    BitmapHeader { encoding: SampleEncoding::Ascii, height: 1u32,
    width: u32::MAX })));
let _ = pnm_header.as_graymap();
let image_buffer = &lt;image::ImageBuffer&lt;image::Rgb&lt;u16&gt;,
    Vec&lt;u16&gt;&gt; as ::core::convert::From&lt;image::DynamicImage&gt;&gt;::from(
    DynamicImage::new_rgb16(55u32,
    65536u32).blur_advanced(GaussianBlurParameters::new_from_sigma(256.0f32)));
let static__vec = image_buffer.as_raw();
let _ = &lt;image::Luma&lt;u16&gt; as image::Pixel&gt;::from_slice(static__vec);
let _ = RowsDecoded::BottomUp { rows: 3181653221u32 };
let mut farbfeld_encoder = &lt;image::codecs::farbfeld::FarbfeldEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::
    new(include_bytes!("data_0.bin").to_vec());
let _ = &lt;image::codecs::farbfeld::FarbfeldEncoder&lt;Vec&lt;u8&gt;&gt; as image::
    ImageEncoder&gt;::set_icc_profile(&amp;mut farbfeld_encoder, vec![227u8, 44u8,
    78u8, 252u8, 235u8, 165u8, 163u8, 176u8, 82u8, 49u8, 41u8, 68u8, 61u8,
    106u8, 156u8, 187u8, 172u8, 18u8, 189u8, 53u8, 103u8, 80u8, 64u8]);
let _ = PixelDensityUnit::Inches;</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::math::utils::multiply_accumulate&lt;u32&gt; (src/math/utils.rs:46)
  image::imageops::filter_1d::filter_symmetric_row&lt;u16, u32, 3&gt; (src/imageops/filter_1d.rs:364)
  image::imageops::filter_1d::filter_2d_separable&lt;u16, f32, u32, 3&gt; (src/imageops/filter_1d.rs:712)
  image::imageops::filter_1d::filter_2d_sep_rgb_u16 (src/imageops/filter_1d.rs:898)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1418)
  image::images::dynimage::DynamicImage::blur_advanced (src/images/dynimage.rs:1244)
  repro_timeout_in_image_imagebuffer_image_rgb_u16_vec_u16_from_imag_e72ae8::main (src/main.rs:12)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;image::codecs::gif::GifEncoder&lt;Vec&lt;u8&gt;&gt; as image::ImageEncoder&gt;::
    write_image(&lt;image::codecs::gif::GifEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![]), &amp;vec![],
    1u32, 1u32, ExtendedColorType::Rgb8);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/gif.rs:513:33:
assertion `left == right` failed: Too much or too little pixel data for the given width and height to create a GIF Frame
  left: 3
 right: 0
backtrace:
  &lt;gif::common::Frame&gt;::from_rgb_speed (gif/src/common.rs:477:9)
  &lt;image::codecs::gif::GifEncoder&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::encode (image/src/codecs/gif.rs:513:33)
  &lt;image::codecs::gif::GifEncoder&lt;alloc::vec::Vec&lt;u8&gt;&gt; as image::io::encoder::ImageEncoder&gt;::write_image (image/src/codecs/gif.rs:639:14)
  repro_image_src_codecs_gif_rs_513_33_af9d14::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_luma8(100u32,
    65535u32).unsharpen(1000.0f32, 8i32);
let _ = dynamic_image.as_luma32f();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::filter_symmetric_row&lt;f32, f32, 1&gt; (src/imageops/filter_1d.rs:359)
  image::imageops::filter_1d::filter_2d_separable&lt;f32, f32, f32, 1&gt; (src/imageops/filter_1d.rs:712)
  image::imageops::filter_1d::filter_2d_sep_plane_f32 (src/imageops/filter_1d.rs:850)
  image::imageops::sample::gaussian_blur_indirect_impl&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, 1&gt; (src/imageops/sample.rs:1569)
  image::imageops::sample::gaussian_blur_indirect&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1512)
  image::imageops::sample::blur_advanced&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1069)
  image::imageops::sample::unsharpen&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, image::color::Luma&lt;u8&gt;, u8&gt; (src/imageops/sample.rs:1638)
  image::images::dynimage::DynamicImage::unsharpen (src/images/dynimage.rs:1278)
  repro_timeout_in_image_dynamicimage_as_luma32f_d93189::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgba32f(1u32,
    1u32).fast_blur(10000000000.0f32);
let _ = dynamic_image.as_rgba16();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::{impl#1}::from_primitive&lt;f32&gt; (src/imageops/fast_blur.rs:80)
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;f32, f32, 4&gt; (src/imageops/fast_blur.rs:344)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgba&lt;f32&gt;, f32&gt; (src/imageops/fast_blur.rs:271)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgba&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:239)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgba&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_as_rgba16_9acd83::main (src/main.rs:4)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let hdr_encoder = &lt;image::codecs::hdr::HdrEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![]);
let _ = hdr_encoder.encode(
    &amp;[&lt;image::Rgb&lt;f32&gt; as ::core::convert::From&lt;[f32; 3]&gt;&gt;::from([0.0f32,
    0.0f32, f32::INFINITY])], 0usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/hdr/encoder.rs:287:19:
attempt to add with overflow
backtrace:
  image::codecs (image/src/codecs/hdr/encoder.rs:287:19)
  &lt;image::codecs (image/src/codecs/hdr/encoder.rs:51:50)
  &lt;&amp;mut &lt;image::codecs (core/src/ops/function.rs:310:21)
  &lt;image::codecs (image/src/codecs/hdr/encoder.rs:76:26)
  &lt;image::codecs (image/src/codecs/hdr/encoder.rs:51:14)
  repro_image_src_codecs_hdr_encoder_rs_287_19_c9941f::main (src/main.rs:3:25)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;image::DynamicImage as image::GenericImageView&gt;::dimensions(
    &amp;DynamicImage::from(DynamicImage::new_rgba8(1u32,
    1u32).fast_blur(1.7197081e24f32).to_rgba16()));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u8, f32, 4&gt; (src/imageops/fast_blur.rs:344)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgba&lt;u8&gt;, f32&gt; (src/imageops/fast_blur.rs:271)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgba&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:244)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgba&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_dimensions_c07a71::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let sample_layout = SampleLayout { channels: 0u8, channel_stride: 0usize,
    width: 0u32, width_stride: usize::MAX, height: 1u32,
    height_stride: 2147483648usize };
let _ = sample_layout.in_bounds_index(2u8, 4294967294u32, 1212462995u32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/images/flat.rs:441:35:
attempt to multiply with overflow
backtrace:
  &lt;image::images::flat::SampleLayout&gt;::in_bounds_index (image/src/images/flat.rs:441:35)
  repro_image_src_images_flat_rs_441_35_125af4::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgba8(2u32, 8u32);
let _ = dynamic_image.blur(16777216.0f32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::prepare_symmetric_kernel&lt;u32&gt; (src/imageops/filter_1d.rs:432)
  image::imageops::filter_1d::filter_2d_separable&lt;u8, f32, u32, 4&gt; (src/imageops/filter_1d.rs:665)
  image::imageops::filter_1d::filter_2d_sep_rgba (src/imageops/filter_1d.rs:818)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1376)
  image::images::dynimage::DynamicImage::blur (src/images/dynimage.rs:1227)
  repro_timeout_in_image_dynamicimage_blur_a5abe7::main (src/main.rs:5)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgba8(65536u32, 8u32);
let _ = dynamic_image.unsharpen(256.0f32, 0i32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_traits::cast::{impl#214}::as_ (num-traits/src/cast.rs:742)
  image::imageops::filter_1d::filter_symmetric_row&lt;f32, f32, 4&gt; (src/imageops/filter_1d.rs:363)
  image::imageops::filter_1d::filter_2d_separable&lt;f32, f32, f32, 4&gt; (src/imageops/filter_1d.rs:712)
  image::imageops::filter_1d::filter_2d_sep_rgba_f32 (src/imageops/filter_1d.rs:882)
  image::imageops::sample::gaussian_blur_indirect_impl&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, 4&gt; (src/imageops/sample.rs:1599)
  image::imageops::sample::gaussian_blur_indirect&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1515)
  image::imageops::sample::blur_advanced&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1069)
  image::imageops::sample::unsharpen&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, image::color::Rgba&lt;u8&gt;, u8&gt; (src/imageops/sample.rs:1638)
  image::images::dynimage::DynamicImage::unsharpen (src/images/dynimage.rs:1278)
  repro_timeout_in_image_dynamicimage_unsharpen_2802e7::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let sample_layout = SampleLayout { channels: 0u8, channel_stride: usize::MAX,
    width: 3632196899u32, width_stride: 1usize, height: 13u32,
    height_stride: 2usize };
let _ = sample_layout.in_bounds_index(232u8, 16u32, 8u32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/images/flat.rs:441:61:
attempt to multiply with overflow
backtrace:
  &lt;image::images::flat::SampleLayout&gt;::in_bounds_index (image/src/images/flat.rs:441:61)
  repro_image_src_images_flat_rs_441_61_48deb3::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::ImageRgb16(DynamicImage::new_rgb8(97u32,
    65535u32).unsharpen(291.0f32, 0i32).to_rgb16());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::filter_symmetric_row&lt;f32, f32, 3&gt; (src/imageops/filter_1d.rs:364)
  image::imageops::filter_1d::filter_2d_separable&lt;f32, f32, f32, 3&gt; (src/imageops/filter_1d.rs:712)
  image::imageops::filter_1d::filter_2d_sep_rgb_f32 (src/imageops/filter_1d.rs:866)
  image::imageops::sample::gaussian_blur_indirect_impl&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, 3&gt; (src/imageops/sample.rs:1589)
  image::imageops::sample::gaussian_blur_indirect&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1514)
  image::imageops::sample::blur_advanced&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1069)
  image::imageops::sample::unsharpen&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, image::color::Rgb&lt;u8&gt;, u8&gt; (src/imageops/sample.rs:1638)
  image::images::dynimage::DynamicImage::unsharpen (src/images/dynimage.rs:1278)
  repro_timeout_in_image_dynamicimage_imagergb16_dba14d::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let _ = image::imageops::resize::&lt;image::DynamicImage&gt;(&amp;DynamicImage::new_rgb8(
    1u32, 1u32), 1000000000u32, 0u32, FilterType::Nearest);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::sample::horizontal_sample&lt;image::color::Rgba&lt;u8&gt;, u8&gt; (image/src/imageops/sample.rs:303)
  image::imageops::sample::resize&lt;image::images::dynimage::DynamicImage&gt; (image/src/imageops/sample.rs:1033)
  repro_timeout_in_image_imageops_resize_db5e3b::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DecoderPreparedImage::new(0u32, 1u32, DynamicImage::new_rgb8(1u32,
    1u32).fast_blur(f32::MAX).color());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u8, f32, 3&gt; (src/imageops/fast_blur.rs:344)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgb&lt;u8&gt;, f32&gt; (src/imageops/fast_blur.rs:267)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgb&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:244)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgb&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_io_decoderpreparedimage_new_4fd6d3::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Frame::from_parts(DynamicImage::new_rgba8(0u32,
    2u32).blur(16777216.0f32).to_rgba8(), 0u32, u32::MAX,
    Frame::from_parts(&lt;image::ImageBuffer&lt;image::Rgba&lt;u8&gt;,
    Vec&lt;u8&gt;&gt; as ::core::default::Default&gt;::default(), 65535u32, 256u32,
    Delay::from_millis(84u32)).delay());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::prepare_symmetric_kernel&lt;u32&gt; (src/imageops/filter_1d.rs:432)
  image::imageops::filter_1d::filter_2d_separable&lt;u8, f32, u32, 4&gt; (src/imageops/filter_1d.rs:665)
  image::imageops::filter_1d::filter_2d_sep_rgba (src/imageops/filter_1d.rs:818)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1376)
  image::images::dynimage::DynamicImage::blur (src/images/dynimage.rs:1227)
  repro_timeout_in_image_frame_from_parts_d4b955::main (src/main.rs:6)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let rows_decoded = RowsDecoded::TopDown { rows: 0u32 };
let u32_v = rows_decoded.rows();
let mut gif_encoder = &lt;image::codecs::gif::GifEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![]);
let _ = gif_encoder.encode_frame(Frame::new(&lt;image::ImageBuffer&lt;image::Rgba&lt;u8&gt;,
    Vec&lt;u8&gt;&gt;&gt;::from_raw(u32_v, 1u32, vec![71u8]).unwrap()));</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/codecs/gif.rs:582:25:
assertion `left == right` failed: Too much or too little pixel data for the given width and height to create a GIF Frame
  left: 0
 right: 1
backtrace:
  &lt;gif::common::Frame&gt;::from_rgba_speed (gif/src/common.rs:216:9)
  &lt;image::codecs::gif::GifEncoder&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::convert_frame (image/src/codecs/gif.rs:582:25)
  &lt;image::codecs::gif::GifEncoder&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::encode_frame (image/src/codecs/gif.rs:542:26)
  repro_image_src_codecs_gif_rs_582_25_7dec08::main (src/main.rs:8:25)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Frame::new(DynamicImage::new_luma32f(1u32,
    1u32).fast_blur(f32::INFINITY).into_rgba8());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;f32, f32, 1&gt; (src/imageops/fast_blur.rs:341)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Luma&lt;f32&gt;, f32&gt; (src/imageops/fast_blur.rs:259)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Luma&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:239)
  image::imageops::fast_blur::fast_blur&lt;image::color::Luma&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_frame_new_a086fd::main (src/main.rs:5)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_luma16(1u32, 53293975u32);
let _ = dynamic_image.blur_advanced(
    GaussianBlurParameters::new_from_kernel_size(34.0f32));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::filter_2d_separable_ring_queue&lt;u16, u32, 1&gt; (src/imageops/filter_1d.rs:583)
  image::imageops::filter_1d::filter_2d_separable&lt;u16, f32, u32, 1&gt; (src/imageops/filter_1d.rs:674)
  image::imageops::filter_1d::filter_2d_sep_plane_u16 (src/imageops/filter_1d.rs:946)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1390)
  image::images::dynimage::DynamicImage::blur_advanced (src/images/dynimage.rs:1244)
  repro_timeout_in_image_dynamicimage_blur_advanced_171947::main (src/main.rs:6)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgb8(65535u32, 38u32).blur(16777216.0f32);
let _ = dynamic_image.as_rgba8();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::prepare_symmetric_kernel&lt;u32&gt; (src/imageops/filter_1d.rs:432)
  image::imageops::filter_1d::filter_2d_separable&lt;u8, f32, u32, 3&gt; (src/imageops/filter_1d.rs:665)
  image::imageops::filter_1d::filter_2d_sep_rgb (src/imageops/filter_1d.rs:802)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1362)
  image::images::dynimage::DynamicImage::blur (src/images/dynimage.rs:1227)
  repro_timeout_in_image_dynamicimage_as_rgba8_3fb118::main (src/main.rs:4)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgba8(1u32,
    1u32).fast_blur(-3882911600000000.0f32);
let _ = dynamic_image.as_luma8();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u8, f32, 4&gt; (src/imageops/fast_blur.rs:343)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgba&lt;u8&gt;, f32&gt; (src/imageops/fast_blur.rs:271)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgba&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:244)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgba&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_as_luma8_d53a54::main (src/main.rs:4)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DecoderPreparedImage::from(ImageLayout::empty(DynamicImage::new_rgba16(
    1u32, 1u32).fast_blur(f32::MIN).color()));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u16, f32, 4&gt; (src/imageops/fast_blur.rs:343)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgba&lt;u16&gt;, f32&gt; (src/imageops/fast_blur.rs:271)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgba&lt;u16&gt;&gt; (src/imageops/fast_blur.rs:239)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgba&lt;u16&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_io_decoderpreparedimage_from_image_imagelay_faaae2::main (src/main.rs:6)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let decoder_prepared_image = DecoderPreparedImage::from(ImageLayout::from(
    ImageLayout::from(ImageLayout::new(16u32, 1495293434u32,
    ColorType::Rgba8))));
let _ = decoder_prepared_image.total_bytes();
let mut png_encoder = &lt;image::codecs::png::PngEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::
    new_with_quality(include_bytes!(
    "mutated_0_V4_A8_R8_G8_B8.bmp.png").to_vec(), CompressionType::default(),
    FilterType::Avg);
let _ = &lt;image::codecs::png::PngEncoder&lt;Vec&lt;u8&gt;&gt; as image::ImageEncoder&gt;::
    set_xmp_metadata(&amp;mut png_encoder, include_bytes!("iptc.jpg.png").to_vec());
let _ = image::load::&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt;(std::io::Cursor::new(
    include_bytes!("image_2.gif").to_vec()), ImageFormat::Farbfeld);
let _ = CicpMatrixCoefficients::Smpte2085;
let _ = DecodedMetadataHint::None;
let dynamic_image = DynamicImage::ImageRgb8(DynamicImage::new_rgba8(16u32,
    65535u32).brighten(45i32).unsharpen(1000.0f32, 65536i32).into_rgb8());
let image_buffer = dynamic_image.to_luma_alpha32f();
let _ = image_buffer.pixels();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::math::utils::multiply_accumulate&lt;f32&gt; (src/math/utils.rs:46)
  image::imageops::filter_1d::filter_symmetric_row&lt;f32, f32, 4&gt; (src/imageops/filter_1d.rs:362)
  image::imageops::filter_1d::filter_2d_separable&lt;f32, f32, f32, 4&gt; (src/imageops/filter_1d.rs:712)
  image::imageops::filter_1d::filter_2d_sep_rgba_f32 (src/imageops/filter_1d.rs:882)
  image::imageops::sample::gaussian_blur_indirect_impl&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, 4&gt; (src/imageops/sample.rs:1599)
  image::imageops::sample::gaussian_blur_indirect&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1515)
  image::imageops::sample::blur_advanced&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1069)
  image::imageops::sample::unsharpen&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, image::color::Rgba&lt;u8&gt;, u8&gt; (src/imageops/sample.rs:1638)
  image::images::dynimage::DynamicImage::unsharpen (src/images/dynimage.rs:1278)
  repro_timeout_in_image_dynamicimage_imagergb8_61c963::main (src/main.rs:19)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgb8(100u32, 65535u32).unsharpen(151.0f32,
    0i32);
let _ = &lt;image::DynamicImage as image::GenericImageView&gt;::pixels(
    &amp;dynamic_image);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::filter_symmetric_row&lt;f32, f32, 3&gt; (src/imageops/filter_1d.rs:364)
  image::imageops::filter_1d::filter_2d_separable&lt;f32, f32, f32, 3&gt; (src/imageops/filter_1d.rs:712)
  image::imageops::filter_1d::filter_2d_sep_rgb_f32 (src/imageops/filter_1d.rs:866)
  image::imageops::sample::gaussian_blur_indirect_impl&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, 3&gt; (src/imageops/sample.rs:1589)
  image::imageops::sample::gaussian_blur_indirect&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1514)
  image::imageops::sample::blur_advanced&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1069)
  image::imageops::sample::unsharpen&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, image::color::Rgb&lt;u8&gt;, u8&gt; (src/imageops/sample.rs:1638)
  image::images::dynimage::DynamicImage::unsharpen (src/images/dynimage.rs:1278)
  repro_timeout_in_image_dynamicimage_pixels_b1eab5::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let _ = CicpTransform::new(&lt;image::ImageBuffer&lt;image::Rgb&lt;f32&gt;,
    Vec&lt;f32&gt;&gt; as ::core::default::Default&gt;::default().color_space(),
    DynamicImage::new_rgb16(10000u32,
    1000u32).blur(1000.0f32).to_rgb32f().color_space());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::filter_symmetric_row&lt;u16, u32, 3&gt; (src/imageops/filter_1d.rs:357)
  image::imageops::filter_1d::filter_2d_separable&lt;u16, f32, u32, 3&gt; (src/imageops/filter_1d.rs:712)
  image::imageops::filter_1d::filter_2d_sep_rgb_u16 (src/imageops/filter_1d.rs:898)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1418)
  image::images::dynimage::DynamicImage::blur (src/images/dynimage.rs:1227)
  repro_timeout_in_image_metadata_cicptransform_new_aba0d1::main (src/main.rs:5)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_luma16(1u32,
    1u32).fast_blur(f32::NEG_INFINITY);
let _ = dynamic_image.as_rgba32f();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u16, f32, 1&gt; (src/imageops/fast_blur.rs:343)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Luma&lt;u16&gt;, f32&gt; (src/imageops/fast_blur.rs:259)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Luma&lt;u16&gt;&gt; (src/imageops/fast_blur.rs:239)
  image::imageops::fast_blur::fast_blur&lt;image::color::Luma&lt;u16&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_as_rgba32f_a9593a::main (src/main.rs:4)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = SpecCompliance::Strict;
let dynamic_image = DynamicImage::ImageRgba8(DynamicImage::new_rgba8(90u32,
    62u32).fast_blur(f32::MIN).into_luma8().expand_palette(&amp;[(202u8, 2u8,
    154u8), (232u8, 2u8, 27u8), (16u8, 13u8, 8u8), (149u8, 14u8, 69u8), (0u8,
    8u8, 87u8), (16u8, 100u8, 100u8)], Some(255u8)));
let image_buffer = dynamic_image.clone().into_luma16();
let static__vec = image_buffer.as_raw();
let _ = &lt;image::Rgba&lt;u16&gt; as image::Pixel&gt;::from_slice(static__vec);
let _ = LimitErrorKind::from(LimitErrorKind::InsufficientMemory);
let _ = NormalForm::Unaliased;</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u8, f32, 4&gt; (src/imageops/fast_blur.rs:343)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgba&lt;u8&gt;, f32&gt; (src/imageops/fast_blur.rs:271)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgba&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:244)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgba&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_imagergba8_7ae4b9::main (src/main.rs:8)
  (3 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgba8(1u32,
    1u32).fast_blur(10000000000.0f32);
let _ = dynamic_image.as_luma16();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_traits::cast::{impl#10}::to_f32 (num-traits/src/cast.rs:252)
  image::imageops::fast_blur::{impl#1}::from_primitive&lt;u8&gt; (src/imageops/fast_blur.rs:80)
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u8, f32, 4&gt; (src/imageops/fast_blur.rs:344)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgba&lt;u8&gt;, f32&gt; (src/imageops/fast_blur.rs:271)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgba&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:244)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgba&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_as_luma16_c0afb4::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let mut gif_encoder = &lt;image::codecs::gif::GifEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![]);
let _ = gif_encoder.encode(&amp;vec![], 1u32, 1u32,
    ExtendedColorType::from(ColorType::La8));</pre></td><td valign="top"><pre>thread 'main' panicked at gif/src/common.rs:299:9:
assertion `left == right` failed: Too much or too little pixel data for the given width and height to create a GIF Frame
  left: 2
 right: 0
backtrace:
  &lt;gif::common::Frame&gt;::from_grayscale_with_alpha (gif/src/common.rs:299:9)
  &lt;image::codecs::gif::GifEncoder&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::encode (image/src/codecs/gif.rs:529:33)
  repro_gif_src_common_rs_299_9_7a5082::main (src/main.rs:6:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut image_buffer = &lt;image::ImageBuffer&lt;image::Luma&lt;u16&gt;,
    Vec&lt;u16&gt;&gt; as ::core::default::Default&gt;::default();
let _ = &lt;image::ImageBuffer&lt;image::Luma&lt;u16&gt;,
    Vec&lt;u16&gt;&gt; as image::GenericImage&gt;::sub_image(&amp;mut image_buffer,
    Rect { x: 0u32, y: 0u32, width: 0u32, height: 1u32 });</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/math/rect.rs:119:5:
The rectangle Rect { x: 0, y: 0, width: 0, height: 1 } is out of bounds for the image with dimensions 0×0
backtrace:
  image::math::rect::panic_out_of_bounds (image/src/math/rect.rs:119:5)
  &lt;image::math::rect::Rect&gt;::assert_in_bounds_of::&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u16&gt;, alloc::vec::Vec&lt;u16&gt;&gt;&gt; (image/src/math/rect.rs:94:13)
  &lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;u16&gt;, alloc::vec::Vec&lt;u16&gt;&gt; as image::images::generic_image::GenericImage&gt;::sub_image (image/src/images/generic_image.rs:362:14)
  repro_image_src_math_rect_rs_119_5_89a7f1::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::ImageRgba32F(DynamicImage::new_luma16(1u32,
    1u32).to_rgba32f());
let dynamic_image2 = dynamic_image.filter3x3(&amp;[0.0f32, 0.0f32, f32::NAN, 0.0f32,
    0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32]);
let _ = dynamic_image2.brighten(0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/colorops.rs:170:47:
called `Option::unwrap()` on a `None` value
backtrace:
  image::imageops::colorops::brighten::&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, image::color::Rgba&lt;f32&gt;, f32&gt;::{closure#0} (image/src/imageops/colorops.rs:170:47)
  &lt;image::color::Rgba&lt;f32&gt; as image::traits::Pixel&gt;::apply_with_alpha::&lt;image::imageops::colorops::brighten&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, image::color::Rgba&lt;f32&gt;, f32&gt;::{closure#0}, image::imageops::colorops::brighten&lt;image::images::buffer::ImageB... (+16 chars)
  &lt;image::color::Rgba&lt;f32&gt; as image::traits::Pixel&gt;::map_with_alpha::&lt;image::imageops::colorops::brighten&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, image::color::Rgba&lt;f32&gt;, f32&gt;::{closure#0}, image::imageops::colorops::brighten&lt;image::images::buffer::ImageBuf... (+16 chars)
  image::imageops::colorops::brighten::&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, image::color::Rgba&lt;f32&gt;, f32&gt; (image/src/imageops/colorops.rs:168:23)
  &lt;image::images::dynimage::DynamicImage&gt;::brighten (image/src/images/dynimage.rs:1318:38)
  repro_image_src_imageops_colorops_rs_170_47_dfc6fd::main (src/main.rs:6:28)</pre></td></tr>
<tr><td valign="top"><pre>let mut dynamic_image = DynamicImage::new_luma8(1u32, 1u32);
image::imageops::contrast_in_place::&lt;image::DynamicImage&gt;(&amp;mut dynamic_image,
    f32::NAN);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/colorops.rs:143:34:
called `Option::unwrap()` on a `None` value
backtrace:
  image::imageops::colorops::contrast_in_place::&lt;image::images::dynimage::DynamicImage&gt;::{closure#0} (image/src/imageops/colorops.rs:143:34)
  &lt;image::color::Rgba&lt;u8&gt; as image::traits::Pixel&gt;::apply::&lt;image::imageops::colorops::contrast_in_place&lt;image::images::dynimage::DynamicImage&gt;::{closure#0}&gt; (image/src/color.rs:450:18)
  &lt;image::color::Rgba&lt;u8&gt; as image::traits::Pixel&gt;::map::&lt;image::imageops::colorops::contrast_in_place&lt;image::images::dynimage::DynamicImage&gt;::{closure#0}&gt; (image/src/traits.rs:514:14)
  image::imageops::colorops::contrast_in_place::&lt;image::images::dynimage::DynamicImage&gt; (image/src/imageops/colorops.rs:137:43)
  repro_image_src_imageops_colorops_rs_143_34_fef311::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgb16(255u32, 10u32).thumbnail(65535u32,
    1000000000u32);
let _ = dynamic_image.as_luma_alpha16();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::color::{impl#52}::from_slice&lt;u16&gt; (src/color.rs:390)
  image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u16&gt;, alloc::vec::Vec&lt;u16, alloc::alloc::Global&gt;&gt;::get_pixel&lt;image::color::Rgb&lt;u16&gt;, alloc::vec::Vec&lt;u16, alloc::alloc::Global&gt;&gt; (src/images/buffer.rs:617)
  image::images::buffer::{impl#38}::get_pixel&lt;image::color::Rgb&lt;u16&gt;, alloc::vec::Vec&lt;u16, alloc::alloc::Global&gt;&gt; (src/images/buffer.rs:1302)
  image::imageops::sample::thumbnail_sample_fraction_both&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u16&gt;, alloc::vec::Vec&lt;u16, alloc::alloc::Global&gt;&gt;, image::color::Rgb&lt;u16&gt;, u16&gt; (src/imageops/sample.rs:852)
  image::imageops::sample::thumbnail&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u16&gt;, alloc::vec::Vec&lt;u16, alloc::alloc::Global&gt;&gt;, image::color::Rgb&lt;u16&gt;, u16&gt; (src/imageops/sample.rs:663)
  image::images::dynimage::DynamicImage::thumbnail_exact (src/images/dynimage.rs:1176)
  image::images::dynimage::DynamicImage::thumbnail (src/images/dynimage.rs:1162)
  repro_timeout_in_image_dynamicimage_as_luma_alpha16_e27773::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let _ = image::imageops::resize::&lt;image::DynamicImage&gt;(
    &amp;DynamicImage::new_luma_a32f(1u32, 1u32), 0u32, 1u32, FilterType::Gaussian);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/sample.rs:242:18:
attempt to add with overflow
backtrace:
  image::imageops::sample (image/src/imageops/sample.rs:242:18)
  image::imageops::sample::resize::&lt;image::images::dynimage::DynamicImage&gt; (image/src/imageops/sample.rs:1033:5)
  repro_image_src_imageops_sample_rs_242_18_06038a::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::from(DynamicImage::new_luma_a8(1u32,
    65535u32).fast_blur(16777216.0f32));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u8, f32, 2&gt; (src/imageops/fast_blur.rs:343)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::LumaA&lt;u8&gt;, f32&gt; (src/imageops/fast_blur.rs:263)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::LumaA&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:244)
  image::imageops::fast_blur::fast_blur&lt;image::color::LumaA&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_from_image_dynamicimage_98dbe0::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let cicp_color_primaries = CicpColorPrimaries::RgbB;
let _ = cicp_color_primaries.is_supported_for_transform();
let _ = &lt;image::codecs::hdr::HdrEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(include_bytes!(
    "image_0.tiff").to_vec());
let _ = DecodedImageAttributes::default();
let _ = SampleEncoding::Binary;
let _ = PixelDensity::dpi(2048u16);
let _ = BiLevel;
let _ = DynamicImage::from(DynamicImage::new_rgba16(2u32,
    65536u32).fast_blur(f32::MAX).to_rgba8());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::{impl#1}::from_primitive&lt;u16&gt; (src/imageops/fast_blur.rs:80)
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u16, f32, 4&gt; (src/imageops/fast_blur.rs:344)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgba&lt;u16&gt;, f32&gt; (src/imageops/fast_blur.rs:271)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgba&lt;u16&gt;&gt; (src/imageops/fast_blur.rs:239)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgba&lt;u16&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_from_image_imagebuffer_image_r_0c3986::main (src/main.rs:16)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_rgba32f(1000u32, 65536u32);
let dynamic_image2 = dynamic_image.thumbnail(4010821916u32, 65535u32);
let _ = dynamic_image2.color();
let _ = FilterType::Avg;
let _ = &lt;image::codecs::ico::IcoEncoder&lt;Vec&lt;u8&gt;&gt; as image::ImageEncoder&gt;::
    write_image(&lt;image::codecs::ico::IcoEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(include_bytes!(
    "image_0.bmp").to_vec()), &amp;vec![], u32::MAX, 0u32,
    ExtendedColorType::Cmyk16);
let _ = dynamic_image2.as_rgba16();
let _ = ColorSpace::Srgb;
let mut image_reader_options = &lt;image::ImageReaderOptions&lt;::std::io::
    Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::with_format(std::io::Cursor::new(include_bytes!(
    "mutated_1_rgb24.bmp").to_vec()), ImageFormat::Farbfeld);
... (12 lines omitted)
let tga_encoder = &lt;image::codecs::tga::TgaEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(include_bytes!(
    "image_3.jpg").to_vec());
let mut tga_encoder2 = tga_encoder.disable_rle();
let _ = &lt;image::codecs::tga::TgaEncoder&lt;Vec&lt;u8&gt;&gt; as image::ImageEncoder&gt;::
    set_icc_profile(&amp;mut tga_encoder2,
    include_bytes!("mutated_4_pal8os2sp.bmp").to_vec());
let _ = Repeat::Finite(23566u16);
let _ = UnsupportedErrorKind::GenericFeature("\u{200b}\u{200d}".to_string());
let _ = &lt;image::ImageReaderOptions&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt;&gt;::new(
    std::io::Cursor::new(include_bytes!("image_5.bmp").to_vec()));
let vec2 = include_bytes!("image_6.gif").to_vec();
let _ = &lt;image::LumaA&lt;u8&gt; as image::Pixel&gt;::from_slice(&amp;vec2);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_traits::cast::{impl#43}::from&lt;f64&gt; (num-traits/src/cast.rs:674)
  image::traits::Enlargeable::clamp_from&lt;f32&gt; (src/traits.rs:85)
  image::imageops::sample::thumbnail_sample_block::{closure#0}&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt;, image::color::Rgba&lt;f32&gt;, f32&gt; (src/imageops/sample.rs:739)
  image::imageops::sample::thumbnail_sample_block&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt;, image::color::Rgba&lt;f32&gt;, f32&gt; (src/imageops/sample.rs:739)
  image::imageops::sample::thumbnail&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt;, image::color::Rgba&lt;f32&gt;, f32&gt; (src/imageops/sample.rs:629)
  image::images::dynimage::DynamicImage::thumbnail_exact (src/images/dynimage.rs:1176)
  image::images::dynimage::DynamicImage::thumbnail (src/images/dynimage.rs:1162)
  repro_timeout_in_image_dynamicimage_thumbnail_a87366::main (src/main.rs:18)
  (6 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::ImageLumaA32F(
    &lt;image::ImageBuffer&lt;image::LumaA&lt;f32&gt;, Vec&lt;f32&gt;&gt;&gt;::from_raw(1u32, 0u32,
    vec![0.0f32]).unwrap());
let _ = dynamic_image.blur_advanced(GaussianBlurParameters::SMOOTHING_3);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/sample.rs:1467:14:
called `Result::unwrap()` on an `Err` value: Limits(LimitError { kind: DimensionError })
backtrace:
  image::imageops::sample::gaussian_blur_dyn_image (image/src/imageops/sample.rs:1467:14)
  &lt;image::images::dynimage::DynamicImage&gt;::blur_advanced (image/src/images/dynimage.rs:1244:9)
  repro_image_src_imageops_sample_rs_1467_14_c2382c::main (src/main.rs:6:27)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::ImageLuma32F(
    &lt;image::ImageBuffer&lt;image::Luma&lt;f32&gt;, Vec&lt;f32&gt;&gt;&gt;::from_raw(0u32, 0u32,
    vec![0.0f32]).unwrap());
let _ = dynamic_image.blur_advanced(GaussianBlurParameters::SMOOTHING_7);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/sample.rs:1453:14:
called `Result::unwrap()` on an `Err` value: Limits(LimitError { kind: DimensionError })
backtrace:
  image::imageops::sample::gaussian_blur_dyn_image (image/src/imageops/sample.rs:1453:14)
  &lt;image::images::dynimage::DynamicImage&gt;::blur_advanced (image/src/images/dynimage.rs:1244:9)
  repro_image_src_imageops_sample_rs_1453_14_23d0af::main (src/main.rs:6:27)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::ImageLuma32F(DynamicImage::new_rgb8(1u32,
    65536u32).fast_blur(16777216.0f32).to_luma32f());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::{impl#1}::from_primitive&lt;u8&gt; (src/imageops/fast_blur.rs:80)
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u8, f32, 3&gt; (src/imageops/fast_blur.rs:344)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgb&lt;u8&gt;, f32&gt; (src/imageops/fast_blur.rs:267)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgb&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:244)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgb&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_imageluma32f_49f918::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::ImageLuma8(DynamicImage::new_rgba32f(1u32,
    1u32).fast_blur(1.84e16f32).into_luma8());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::{impl#1}::from_primitive&lt;f32&gt; (src/imageops/fast_blur.rs:80)
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;f32, f32, 4&gt; (src/imageops/fast_blur.rs:344)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgba&lt;f32&gt;, f32&gt; (src/imageops/fast_blur.rs:271)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgba&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:239)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgba&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_imageluma8_eed0ee::main (src/main.rs:4)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::from(DynamicImage::new_rgb32f(1u32,
    1u32).fast_blur(-20200000000000.0f32).to_rgb8());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;f32, f32, 3&gt; (src/imageops/fast_blur.rs:343)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::Rgb&lt;f32&gt;, f32&gt; (src/imageops/fast_blur.rs:267)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::Rgb&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:239)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgb&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_from_image_imagebuffer_image_r_06adc1::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;image::ImageBuffer&lt;image::Rgb&lt;u16&gt;,
    Vec&lt;u16&gt;&gt; as image::GenericImageView&gt;::buffer_like(
    &amp;DynamicImage::new_rgba16(255u32, 65535u32).blur(92.0f32).into_rgb16());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::filter_symmetric_row&lt;u16, u32, 4&gt; (src/imageops/filter_1d.rs:359)
  image::imageops::filter_1d::filter_2d_separable&lt;u16, f32, u32, 4&gt; (src/imageops/filter_1d.rs:712)
  image::imageops::filter_1d::filter_2d_sep_rgba_u16 (src/imageops/filter_1d.rs:914)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1432)
  image::images::dynimage::DynamicImage::blur (src/images/dynimage.rs:1227)
  repro_timeout_in_image_imagebuffer_image_rgb_u16_vec_u16_buffer_li_5af0a4::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let mut dynamic_image = DynamicImage::new_luma_a8(1u32, 433870799u32);
image::imageops::brighten_in_place::&lt;image::DynamicImage&gt;(&amp;mut dynamic_image,
    0i32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::color::{impl#20}::from_color&lt;u8, u8&gt; (src/color.rs:731)
  image::color::{impl#11}::into_color&lt;image::color::Rgba&lt;u8&gt;, image::color::Rgba&lt;u8&gt;&gt; (src/color.rs:629)
  image::images::dynimage::{impl#14}::get_pixel (src/images/dynimage.rs:1755)
  image::imageops::colorops::brighten_in_place&lt;image::images::dynimage::DynamicImage&gt; (image/src/imageops/colorops.rs:200)
  repro_timeout_in_image_imageops_brighten_in_place_fdf6ea::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let gaussian_blur_parameters = GaussianBlurParameters::SMOOTHING_3;
let mut limits = Limits::default();
limits.free_usize(256usize);
let mut luma = &lt;image::Luma&lt;u8&gt; as ::core::convert::From&lt;[u8; 1]&gt;&gt;::from(
    [254u8]);
&lt;image::Luma&lt;u8&gt; as image::Pixel&gt;::apply2::&lt;fn(u8, u8) -&gt; u8&gt;(&amp;mut luma,
    &amp;&lt;image::Luma&lt;u8&gt; as image::Pixel&gt;::to_luma(
    &amp;&lt;image::Luma&lt;u8&gt; as ::core::convert::From&lt;[u8; 1]&gt;&gt;::from([7u8])), |_,
    _| 1 as u8);
let _ = UnsupportedErrorKind::Format(ImageFormatHint::Unknown);
let dynamic_image = DynamicImage::from(DynamicImage::from(
    DynamicImage::new_luma_a8(1000u32, 65535u32).to_luma_alpha32f()));
let dynamic_image2 = dynamic_image.blur_advanced(gaussian_blur_parameters);
let dynamic_image3 = dynamic_image2.rotate270();
let _ = dynamic_image3.as_rgb32f();
let image_buffer = dynamic_image2.as_rgba16().unwrap();
let static__vec = image_buffer.as_raw();
let _ = &lt;image::Rgba&lt;u16&gt; as image::Pixel&gt;::from_slice(static__vec);
let _ = RowsDecoded::BottomUp { rows: 4294967294u32 };
let image_buffer2 = dynamic_image2.to_rgba32f();
let static__vec2 = image_buffer2.as_raw();
let _ = &lt;image::Rgba&lt;f32&gt; as image::Pixel&gt;::from_slice(static__vec2);
let _ = FilterType::Paeth;</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::filter_symmetric_column&lt;f32, f32&gt; (src/imageops/filter_1d.rs:252)
  image::imageops::filter_1d::filter_2d_separable_ring_queue&lt;f32, f32, 2&gt; (src/imageops/filter_1d.rs:592)
  image::imageops::filter_1d::filter_2d_separable&lt;f32, f32, f32, 2&gt; (src/imageops/filter_1d.rs:674)
  image::imageops::filter_1d::filter_2d_sep_la_f32 (src/imageops/filter_1d.rs:834)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1460)
  image::images::dynimage::DynamicImage::blur_advanced (src/images/dynimage.rs:1244)
  repro_timeout_in_image_codecs_png_filtertype_paeth_8aa32a::main (src/main.rs:17)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::from(DynamicImage::new_luma8(1u32,
    44917274u32).into_rgba16());
let _ = dynamic_image.thumbnail(0u32, 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/sample.rs:578:13:
attempt to add with overflow
backtrace:
  &lt;u32 as core::ops::arith::AddAssign&gt;::add_assign (core/src/ops/arith.rs:790:51)
  &lt;image::imageops::sample::ThumbnailSum&lt;u16&gt;&gt;::add_pixel::&lt;image::color::Rgba&lt;u16&gt;&gt; (image/src/imageops/sample.rs:578:13)
  image::imageops::sample::thumbnail_sample_block::&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u16&gt;, alloc::vec::Vec&lt;u16&gt;&gt;, image::color::Rgba&lt;u16&gt;, u16&gt; (image/src/imageops/sample.rs:712:21)
  image::imageops::sample::thumbnail::&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgba&lt;u16&gt;, alloc::vec::Vec&lt;u16&gt;&gt;, image::color::Rgba&lt;u16&gt;, u16&gt; (image/src/imageops/sample.rs:629:17)
  &lt;image::images::dynimage::DynamicImage&gt;::thumbnail_exact (image/src/images/dynimage.rs:1176:38)
  &lt;image::images::dynimage::DynamicImage&gt;::thumbnail (image/src/images/dynimage.rs:1162:14)
  repro_image_src_imageops_sample_rs_578_13_2bdd45::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let mut tga_encoder = &lt;image::codecs::tga::TgaEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(
    SIGNATURE.to_vec());
let _ = &lt;image::codecs::tga::TgaEncoder&lt;Vec&lt;u8&gt;&gt; as image::ImageEncoder&gt;::
    set_icc_profile(&amp;mut tga_encoder, vec![]);
let _ = PnmSubtype::Graymap(SampleEncoding::Ascii);
let dynamic_image = DynamicImage::new_rgb32f(65535u32, 24u32);
let dynamic_image2 = dynamic_image.thumbnail_exact(26u32, 65535u32);
let dynamic_image3 = dynamic_image2.thumbnail(65535u32, 16u32);
let _ = dynamic_image3.huerotate(-1329250857i32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::sample::ThumbnailSum&lt;f32&gt;::add_pixel&lt;f32, image::color::Rgb&lt;f32&gt;&gt; (src/imageops/sample.rs:577)
  image::imageops::sample::thumbnail_sample_fraction_vertical&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt;, image::color::Rgb&lt;f32&gt;, f32&gt; (src/imageops/sample.rs:813)
  image::imageops::sample::thumbnail&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt;, image::color::Rgb&lt;f32&gt;, f32&gt; (src/imageops/sample.rs:657)
  image::images::dynimage::DynamicImage::thumbnail_exact (src/images/dynimage.rs:1176)
  repro_timeout_in_image_dynamicimage_thumbnail_exact_415190::main (src/main.rs:11)
  (7 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let dynamic_image = DynamicImage::new_luma32f(1u32, 1u32);
let dynamic_image2 = dynamic_image.filter3x3(&amp;[0.0f32, 0.0f32, 0.0f32, 0.0f32,
    0.0f32, f32::INFINITY, 0.0f32, 0.0f32, 0.0f32]);
let _ = dynamic_image2.unsharpen(0.0f32, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/sample.rs:1650:48:
called `Option::unwrap()` on a `None` value
backtrace:
  image::imageops::sample::unsharpen::&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, image::color::Luma&lt;f32&gt;, f32&gt;::{closure#0} (image/src/imageops/sample.rs:1650:48)
  &lt;image::color::Luma&lt;f32&gt; as image::traits::Pixel&gt;::apply2::&lt;image::imageops::sample::unsharpen&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, image::color::Luma&lt;f32&gt;, f32&gt;::{closure#0}&gt; (image/src/color.rs:468:18)
  &lt;image::color::Luma&lt;f32&gt; as image::traits::Pixel&gt;::map2::&lt;image::imageops::sample::unsharpen&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, image::color::Luma&lt;f32&gt;, f32&gt;::{closure#0}&gt; (image/src/traits.rs:566:14)
  image::imageops::sample::unsharpen::&lt;image::images::buffer::ImageBuffer&lt;image::color::Luma&lt;f32&gt;, alloc::vec::Vec&lt;f32&gt;&gt;, image::color::Luma&lt;f32&gt;, f32&gt; (image/src/imageops/sample.rs:1649:23)
  &lt;image::images::dynimage::DynamicImage&gt;::unsharpen (image/src/images/dynimage.rs:1278:38)
  repro_image_src_imageops_sample_rs_1650_48_170566::main (src/main.rs:6:28)</pre></td></tr>
<tr><td valign="top"><pre>let decoder_prepared_image = DecoderPreparedImage::new(56u32, 4294705150u32,
    ColorType::Rgb32F);
let _ = decoder_prepared_image.total_bytes();
let _ = PixelDensityUnit::PixelAspectRatio;
let _ = &lt;image::ImageBuffer&lt;image::Rgba&lt;u8&gt;,
    Vec&lt;u8&gt;&gt; as image::GenericImageView&gt;::in_bounds(
    &amp;&lt;image::ImageBuffer&lt;image::Luma&lt;u8&gt;,
    Vec&lt;u8&gt;&gt; as ::core::default::Default&gt;::default().expand_palette(&amp;[(19u8,
    17u8, 94u8), (255u8, 150u8, 64u8), (220u8, 8u8, 232u8), (18u8, 7u8, 8u8)],
    Some(254u8)), 6u32, 52u32);
let _ = ImageFormatHint::Unknown;
let _ = PixelDensity { density: (96u16, 8u16), unit: PixelDensityUnit::Inches };
let _ = &lt;image::LumaA&lt;u8&gt; as ::core::convert::From&lt;[u8; 2]&gt;&gt;::from([14u8,
    136u8]);
let _ = SampleEncoding::Binary;
let _ = ConvertColorOptions::default();
let mut image_buffer = &lt;image::ImageBuffer&lt;image::Luma&lt;f32&gt;,
    Vec&lt;f32&gt;&gt; as ::core::convert::From&lt;image::DynamicImage&gt;&gt;::from(
    DynamicImage::new_rgb16(40u32, 67u32).blur(16777216.0f32).to_alpha_mask());
image_buffer.set_transfer_function(CicpTransferCharacteristics::Unspecified);
let mut image_buffer2 = &lt;image::ImageBuffer&lt;image::Luma&lt;u8&gt;,
    Vec&lt;u8&gt;&gt;&gt;::from_raw(1000u32, 27u32,
    include_bytes!("mutated_0_pal4rletrns.bmp").to_vec()).unwrap();
let enumerate_pixels_mut_par = image_buffer2.par_enumerate_pixels_mut();
let _ = ImageError::Parameter(ParameterError::from_kind(
    ParameterError::from_kind(ParameterErrorKind::DimensionMismatch).kind()));
let _ = &lt;image::SubImage&lt;String&gt;&gt;::new(include_str!("string_1.txt").to_string(),
    Rect { x: 1000u32, y: 256u32, width: 0u32, height: 2439582098u32 });</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::prepare_symmetric_kernel&lt;u32&gt; (src/imageops/filter_1d.rs:432)
  image::imageops::filter_1d::filter_2d_separable&lt;u16, f32, u32, 3&gt; (src/imageops/filter_1d.rs:665)
  image::imageops::filter_1d::filter_2d_sep_rgb_u16 (src/imageops/filter_1d.rs:898)
  image::imageops::sample::gaussian_blur_dyn_image (src/imageops/sample.rs:1418)
  image::images::dynimage::DynamicImage::blur (src/images/dynimage.rs:1227)
  repro_timeout_in_image_imagebuffer_image_luma_f32_vec_f32_from_ima_f3c3d3::main (src/main.rs:25)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorSpace::Srgb;
let _ = SampleEncoding::Ascii;
let dynamic_image = DynamicImage::new_luma_a32f(65535u32, 1000u32);
let dynamic_image2 = dynamic_image.adjust_contrast(0.0f32);
let _ = dynamic_image2.adjust_contrast(0.0f32);
let _ = &lt;image::codecs::png::PngEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new_with_quality(
    include_bytes!("image_0.png").to_vec(), CompressionType::Level(0u8),
    FilterType::Up);
let _ = &lt;image::ImageBuffer&lt;image::Rgba&lt;u16&gt;,
    Vec&lt;u16&gt;&gt; as image::GenericImageView&gt;::in_bounds(&amp;dynamic_image.to_rgba16(),
    2303440273u32, 7u32);
let _ = Limits::default();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::images::buffer::ImageBuffer&lt;image::color::LumaA&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt;::get_pixel_mut&lt;image::color::LumaA&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt; (src/images/buffer.rs:864)
  image::images::buffer::ImageBuffer&lt;image::color::LumaA&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt;::put_pixel&lt;image::color::LumaA&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt; (src/images/buffer.rs:883)
  image::imageops::colorops::contrast&lt;image::images::buffer::ImageBuffer&lt;image::color::LumaA&lt;f32&gt;, alloc::vec::Vec&lt;f32, alloc::alloc::Global&gt;&gt;, image::color::LumaA&lt;f32&gt;, f32&gt; (src/imageops/colorops.rs:112)
  image::images::dynimage::DynamicImage::adjust_contrast (src/images/dynimage.rs:1307)
  repro_timeout_in_image_imagebuffer_image_rgba_u16_vec_u16_in_bound_53686e::main (src/main.rs:13)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::from(DynamicImage::new_rgb8(256u32,
    42u32).unsharpen(65536.0f32, 33554434i32).into_luma_alpha8());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::filter_1d::filter_symmetric_row&lt;f32, f32, 3&gt; (src/imageops/filter_1d.rs:364)
  image::imageops::filter_1d::filter_2d_separable&lt;f32, f32, f32, 3&gt; (src/imageops/filter_1d.rs:712)
  image::imageops::filter_1d::filter_2d_sep_rgb_f32 (src/imageops/filter_1d.rs:866)
  image::imageops::sample::gaussian_blur_indirect_impl&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, 3&gt; (src/imageops/sample.rs:1589)
  image::imageops::sample::gaussian_blur_indirect&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1514)
  image::imageops::sample::blur_advanced&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (src/imageops/sample.rs:1069)
  image::imageops::sample::unsharpen&lt;image::images::buffer::ImageBuffer&lt;image::color::Rgb&lt;u8&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;, image::color::Rgb&lt;u8&gt;, u8&gt; (src/imageops/sample.rs:1638)
  image::images::dynimage::DynamicImage::unsharpen (src/images/dynimage.rs:1278)
  repro_timeout_in_image_dynamicimage_from_image_imagebuffer_image_l_39b13f::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let _ = SampleEncoding::Binary;
let _ = &lt;image::Rgb&lt;u16&gt; as image::Pixel&gt;::to_luma_alpha(
    &amp;&lt;image::Rgb&lt;u16&gt; as image::Pixel&gt;::broadcast(1000u16));
let dynamic_image = DynamicImage::ImageRgb32F(DynamicImage::new_luma_a16(
    16777216u32, 1u32).to_rgb32f());
let _ = ParameterErrorKind::CicpMismatch { expected: dynamic_image.fast_blur(
    -42.5f32).into_luma8().color_space(),
    found: &lt;image::ImageBuffer&lt;image::Rgba&lt;u16&gt;,
    Vec&lt;u16&gt;&gt; as ::core::default::Default&gt;::default().color_space() };</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_vertical_pass&lt;f32, f32&gt; (src/imageops/fast_blur.rs:448)
  image::imageops::fast_blur::box_blur_vertical_pass_strategy&lt;image::color::Rgb&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:292)
  image::imageops::fast_blur::fast_blur&lt;image::color::Rgb&lt;f32&gt;&gt; (src/imageops/fast_blur.rs:159)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_error_parametererrorkind_cicpmismatch_0ded85::main (src/main.rs:9)</pre></td></tr>
<tr><td valign="top"><pre>let mut dynamic_image = DynamicImage::ImageLumaA8(
    &lt;image::ImageBuffer&lt;image::LumaA&lt;u8&gt;, Vec&lt;u8&gt;&gt;&gt;::from_raw(1u32, 1u32,
    vec![77u8, 54u8]).unwrap());
image::imageops::brighten_in_place::&lt;image::DynamicImage&gt;(&amp;mut dynamic_image,
    i32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/imageops/colorops.rs:203:35:
attempt to add with overflow
backtrace:
  image::imageops::colorops::brighten_in_place::&lt;image::images::dynimage::DynamicImage&gt;::{closure#0} (image/src/imageops/colorops.rs:203:35)
  &lt;image::color::Rgba&lt;u8&gt; as image::traits::Pixel&gt;::apply_with_alpha::&lt;image::imageops::colorops::brighten_in_place&lt;image::images::dynimage::DynamicImage&gt;::{closure#0}, image::imageops::colorops::brighten_in_place&lt;image::images::dynimage::DynamicImage&gt;::{closure#1}&gt; (image/src/color.rs:457:18)
  &lt;image::color::Rgba&lt;u8&gt; as image::traits::Pixel&gt;::map_with_alpha::&lt;image::imageops::colorops::brighten_in_place&lt;image::images::dynimage::DynamicImage&gt;::{closure#0}, image::imageops::colorops::brighten_in_place&lt;image::images::dynimage::DynamicImage&gt;::{closure#1}&gt; (image/src/traits.rs:531:14)
  image::imageops::colorops::brighten_in_place::&lt;image::images::dynimage::DynamicImage&gt; (image/src/imageops/colorops.rs:200:43)
  repro_image_src_imageops_colorops_rs_203_35_3aad51::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DynamicImage::from(DynamicImage::new_luma_a8(1u32,
    1u32).fast_blur(-2147483600.0f32).into_rgb32f());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  image::imageops::fast_blur::box_blur_horizontal_pass&lt;u8, f32, 2&gt; (src/imageops/fast_blur.rs:343)
  image::imageops::fast_blur::box_blur_horizontal_pass_dispatch_cn&lt;image::color::LumaA&lt;u8&gt;, f32&gt; (src/imageops/fast_blur.rs:263)
  image::imageops::fast_blur::box_blur_horizontal_pass_strategy&lt;image::color::LumaA&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:244)
  image::imageops::fast_blur::fast_blur&lt;image::color::LumaA&lt;u8&gt;&gt; (src/imageops/fast_blur.rs:150)
  image::images::dynimage::DynamicImage::fast_blur (src/images/dynimage.rs:1260)
  repro_timeout_in_image_dynamicimage_from_image_imagebuffer_image_r_7d99cb::main (src/main.rs:4)
  (4 std/libc frames omitted)</pre></td></tr>
</table>
