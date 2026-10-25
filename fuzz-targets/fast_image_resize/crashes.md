# fast_image_resize - crashes

Source: fast_image_resize 6.1.0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let vec = vec![];
let _ = ImageRef::new(1380315911u32, u32::MAX, &amp;vec, PixelType::U8x4);</pre></td><td valign="top"><pre>thread 'main' panicked at fast_image_resize/src/images/image.rs:26:20:
attempt to multiply with overflow
backtrace:
  &lt;fast_image_resize::images::image::ImageRef&gt;::new (fast_image_resize/src/images/image.rs:26:20)
  repro_fast_image_resize_src_images_image_rs_26_20_3b5d38::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Image::new(u32::MAX, u32::MAX, PixelType::U16x2);</pre></td><td valign="top"><pre>thread 'main' panicked at fast_image_resize/src/images/image.rs:120:53:
attempt to multiply with overflow
backtrace:
  &lt;fast_image_resize::images::image::Image&gt;::new (fast_image_resize/src/images/image.rs:120:53)
  repro_fast_image_resize_src_images_image_rs_120_53_13e7f9::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let image = Image::new(1u32, 1000000000u32, PixelType::U16x2);
let _ = image.copy();</pre></td><td valign="top"><pre>memory allocation of 4000000000 bytes failed</pre></td></tr>
<tr><td valign="top"><pre>let image = Image::new(1u32, 1000000000u32, PixelType::F32x3);
let _ = image.pixel_type();</pre></td><td valign="top"><pre>memory allocation of 12000000000 bytes failed</pre></td></tr>
<tr><td valign="top"><pre>let pixel_type = PixelType::U16x4;
let image = Image::new(1u32, 1000000000u32, pixel_type);
let _ = image.height();</pre></td><td valign="top"><pre>memory allocation of 8000000000 bytes failed</pre></td></tr>
<tr><td valign="top"><pre>let _ = Image::new(1u32, 1000000000u32, PixelType::I32);</pre></td><td valign="top"><pre>memory allocation of 4000000000 bytes failed</pre></td></tr>
<tr><td valign="top"><pre>let _ = Image::from_slice_u8(u32::MAX, 1000000000u32, &amp;mut vec![],
    PixelType::F32x2);</pre></td><td valign="top"><pre>thread 'main' panicked at fast_image_resize/src/images/image.rs:160:20:
attempt to multiply with overflow
backtrace:
  &lt;fast_image_resize::images::image::Image&gt;::from_slice_u8 (fast_image_resize/src/images/image.rs:160:20)
  repro_fast_image_resize_src_images_image_rs_160_20_f73a56::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let pixel_type = PixelType::F32x4;
let _ = Image::from_vec_u8(1000000000u32, 1572920769u32, vec![], pixel_type);</pre></td><td valign="top"><pre>thread 'main' panicked at fast_image_resize/src/images/image.rs:136:20:
attempt to multiply with overflow
backtrace:
  &lt;fast_image_resize::images::image::Image&gt;::from_vec_u8 (fast_image_resize/src/images/image.rs:136:20)
  repro_fast_image_resize_src_images_image_rs_136_20_bcf205::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let image = Image::new(1000000000u32, 1u32, PixelType::U8x4);
let _ = Image::from_vec_u8(0u32, 0u32, vec![], image.pixel_type());</pre></td><td valign="top"><pre>memory allocation of 4000000000 bytes failed</pre></td></tr>
<tr><td valign="top"><pre>let image = Image::new(1000000000u32, 1u32, PixelType::I32);
let _ = image.width();</pre></td><td valign="top"><pre>memory allocation of 4000000000 bytes failed</pre></td></tr>
<tr><td valign="top"><pre>let _ = Image::new(1000000000u32, u32::MAX, PixelType::U8x4);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:51:31)
capacity overflow
backtrace:
  &lt;u8 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:51:31)
  &lt;fast_image_resize::images::image::Image&gt;::new (fast_image_resize/src/images/image.rs:120:45)
  repro_fast_image_resize_src_images_image_rs_120_45_8e5e69::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let image = Image::new(1u32, 1000000000u32, PixelType::U8x3);
let _ = ImageRef::new(0u32, 0u32, &amp;vec, image.pixel_type());</pre></td><td valign="top"><pre>memory allocation of 3000000000 bytes failed</pre></td></tr>
<tr><td valign="top"><pre>let image = Image::new(1u32, 1997974991u32, PixelType::U8x2);
let _ = Image::from_slice_u8(1u32, 0u32, &amp;mut vec![], image.pixel_type());</pre></td><td valign="top"><pre>memory allocation of 3995949982 bytes failed</pre></td></tr>
<tr><td valign="top"><pre>let pixel_type = PixelType::F32x4;
let image = Image::new(1u32, 1000000000u32, pixel_type);
let _ = image.buffer();</pre></td><td valign="top"><pre>memory allocation of 16000000000 bytes failed</pre></td></tr>
<tr><td valign="top"><pre>let mul_div = MulDiv::new();
let image = Image::new(7u32, 100000000u32, PixelType::U16x4);
let _ = mul_div.is_supported(image.pixel_type());</pre></td><td valign="top"><pre>memory allocation of 5600000000 bytes failed</pre></td></tr>
</table>
