# tiny-skia - crashes

Source: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let pixmap = Pixmap::new(1u32, 1000000000u32).unwrap();
let pixmap_ref = pixmap.as_ref();
let _ = pixmap_ref.height();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  tiny_skia::pixmap::Pixmap::new (src/pixmap.rs:51)
  repro_out_of_memory_in_tiny_skia_pixmap_new_631fb6::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::from((1.0f32, 0.0f32));
let _ = CubicCoeff { a: tiny_skia_path::f32x2::splat(0.0f32),
    b: point.to_f32x2(), c: tiny_skia_path::f32x2::new(0.0f32, 0.0f32),
    d: PathBuilder::with_capacity(100000000000000usize,
    0usize).last_point().unwrap().to_f32x2() };</pre></td><td valign="top"><pre>out of memory: memory allocation of 100000000000000 bytes failed
backtrace:
  tiny_skia_path::path_builder::PathBuilder::with_capacity (src/path_builder.rs:57)
  repro_out_of_memory_in_tiny_skia_path_pathbuilder_with_capacity_ec4468::main (src/main.rs:7)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Mask::new(u32::MAX, u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:51:31)
capacity overflow
backtrace:
  &lt;u8 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:51:31)
  &lt;tiny_skia::mask::Mask&gt;::new (tiny-skia/src/mask.rs:51:19)
  repro_tiny_skia_src_mask_rs_51_19_029431::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let f32x2 = tiny_skia_path::f32x2::splat(0.0f32);
let mut path_builder = PathBuilder::new();
path_builder.line_to(0.0f32, 0.0f32);
tiny_skia_path::path_geometry::chop_quad_at(&amp;[Point::from_f32x2(f32x2),
    Point::from_xy(0.0f32, 0.0f32)], NormalizedF32Exclusive::ANY,
    &amp;mut [Point { x: 0.0f32, y: 0.0f32 }, path_builder.last_point().unwrap(),
    Point::default(), path_builder.last_point().unwrap(),
    path_builder.last_point().unwrap()]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:86:14:
index out of bounds: the len is 2 but the index is 2
backtrace:
  tiny_skia_path::path_geometry::chop_quad_at (path/src/path_geometry.rs:86:14)
  repro_path_src_path_geometry_rs_86_14_31a915::main (src/main.rs:9:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PathBuilder::with_capacity(usize::MAX, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from path/src/path_builder.rs:57:20)
capacity overflow
backtrace:
  &lt;tiny_skia_path::path_builder::PathBuilder&gt;::with_capacity (path/src/path_builder.rs:57:20)
  repro_path_src_path_builder_rs_57_20_0337f4::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let int_rect = IntRect::from_xywh(0i32, 0i32, 1u32, 1u32).unwrap();
let _ = int_rect.inset(i32::MIN, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:125:13:
attempt to subtract with overflow
backtrace:
  &lt;tiny_skia_path::rect::IntRect&gt;::inset (path/src/rect.rs:125:13)
  repro_path_src_rect_rs_125_13_a91e08::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::from_xy(0.0f32, 0.0f32);
tiny_skia_path::path_geometry::chop_cubic_at2(&amp;[point, point, Point { x: 0.0f32,
    y: 0.0f32 }, point], NormalizedF32Exclusive::ANY, &amp;mut vec![point, point]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:175:5:
index out of bounds: the len is 2 but the index is 2
backtrace:
  tiny_skia_path::path_geometry::chop_cubic_at2 (path/src/path_geometry.rs:175:5)
  repro_path_src_path_geometry_rs_175_5_b22d0b::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point { x: 0.0f32, y: 0.0f32 };
let f32_v = point.length();
tiny_skia_path::path_geometry::chop_cubic_at2(&amp;[Point { x: f32_v, y: 0.0f32 },
    point, Point::from_xy(0.0f32, 1.0f32), point], NormalizedF32Exclusive::ANY,
    &amp;mut vec![Point::from((0.0f32, 0.0f32)), point, point]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:176:5:
index out of bounds: the len is 3 but the index is 3
backtrace:
  tiny_skia_path::path_geometry::chop_cubic_at2 (path/src/path_geometry.rs:176:5)
  repro_path_src_path_geometry_rs_176_5_ae2e51::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>let size = Size::from_wh(1.0f32, 1.1754944e-38f32).unwrap();
let _ = size.to_non_zero_rect(0.0f32, 1.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/size.rs:202:75:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;tiny_skia_path::size::Size&gt;::to_non_zero_rect (path/src/size.rs:202:75)
  repro_path_src_size_rs_202_75_88347b::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let int_size = IntSize::from_wh(1u32, u32::MAX).unwrap();
let _ = int_size.to_int_rect(0i32, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/size.rs:88:63:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;tiny_skia_path::size::IntSize&gt;::to_int_rect (path/src/size.rs:88:63)
  repro_path_src_size_rs_88_63_5912ce::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let path = PathBuilder::from_circle(1.0f32, 1.0f32, 0.0f32).unwrap();
let path_segments_iter = path.segments();
let _ = path_segments_iter.curr_verb();</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path.rs:325:25:
attempt to subtract with overflow
backtrace:
  &lt;tiny_skia_path::path::PathSegmentsIter&gt;::curr_verb (path/src/path.rs:325:25)
  repro_path_src_path_rs_325_25_8522d7::main (src/main.rs:6:32)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PathBuilder::with_capacity(0usize, 4669571948208348195usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from path/src/path_builder.rs:58:21)
capacity overflow
backtrace:
  &lt;tiny_skia_path::path_builder::PathBuilder&gt;::with_capacity (path/src/path_builder.rs:58:21)
  repro_path_src_path_builder_rs_58_21_8e5d69::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>tiny_skia_path::path_geometry::chop_cubic_at2(&amp;[Point::from((0.0f32, 0.0f32)),
    Point::default(), Point::default(), Point::from((0.0f32, 0.0f32))],
    NormalizedF32Exclusive::default(), &amp;mut vec![Point { x: 0.0f32,
    y: 0.0f32 }]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:174:5:
index out of bounds: the len is 1 but the index is 1
backtrace:
  tiny_skia_path::path_geometry::chop_cubic_at2 (path/src/path_geometry.rs:174:5)
  repro_path_src_path_geometry_rs_174_5_1ab30d::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NonZeroRect::from_xywh(0.0f32, 0.0f32, 1.0f32,
    10000000000.0f32).unwrap().to_int_rect();</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:707:10:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;tiny_skia_path::rect::NonZeroRect&gt;::to_int_rect (path/src/rect.rs:707:10)
  repro_path_src_rect_rs_707_10_e8bf18::main (src/main.rs:4:87)</pre></td></tr>
<tr><td valign="top"><pre>tiny_skia_path::path_geometry::chop_cubic_at2(&amp;[Point::from_xy(0.0f32, 0.0f32),
    Point::default(), Point::default(), Point { x: 0.0f32, y: 0.0f32 }],
    NormalizedF32Exclusive::ANY, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:173:5:
index out of bounds: the len is 0 but the index is 0
backtrace:
  tiny_skia_path::path_geometry::chop_cubic_at2 (path/src/path_geometry.rs:173:5)
  repro_path_src_path_geometry_rs_173_5_111e37::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>tiny_skia_path::path_geometry::chop_cubic_at2(&amp;[Point::from((1.0f32, 0.0f32)),
    Point::from((0.0f32, 0.0f32)), Point::zero(), Point::from((0.0f32,
    0.0f32))], NormalizedF32Exclusive::HALF, &amp;mut vec![Point { x: 0.0f32,
    y: 0.0f32 }, Point::default(), Point::zero(), Point::default()]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:177:5:
index out of bounds: the len is 4 but the index is 4
backtrace:
  tiny_skia_path::path_geometry::chop_cubic_at2 (path/src/path_geometry.rs:177:5)
  repro_path_src_path_geometry_rs_177_5_c32e3e::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let non_zero_rect = NonZeroRect::from_xywh(0.0f32, 0.0f32, 1.0f32,
    1.1754944e-38f32).unwrap();
let _ = non_zero_rect.bbox_transform(NonZeroRect::from_xywh(0.0f32, 1.0f32,
    1.0f32, 1.0f32).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:691:37:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;tiny_skia_path::rect::NonZeroRect&gt;::bbox_transform (path/src/rect.rs:691:37)
  repro_path_src_rect_rs_691_37_21837d::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let mut path_stroker = PathStroker::new();
let _ = path_stroker.stroke(&amp;PathBuilder::from_rect(Rect::from_xywh(0.0f32,
    0.0f32, 0.0f32, 0.0f32).unwrap()), &amp;Stroke::default(), -74.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/scalar.rs:69:9:
assertion failed: tolerance &gt;= 0.0
backtrace:
  &lt;f32 as tiny_skia_path::scalar::Scalar&gt;::is_nearly_zero_within_tolerance (path/src/scalar.rs:69:9)
  &lt;tiny_skia_path::Point&gt;::equals_within_tolerance (path/src/lib.rs:123:28)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::line_to (path/src/stroker.rs:454:14)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::stroke_inner (path/src/stroker.rs:390:26)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::stroke (path/src/stroker.rs:301:14)
  repro_path_src_scalar_rs_69_9_624d1a::main (src/main.rs:8:26)</pre></td></tr>
<tr><td valign="top"><pre>tiny_skia_path::path_geometry::chop_quad_at(&amp;[Point::from_xy(0.0f32, 0.0f32)],
    NormalizedF32Exclusive::ANY, &amp;mut [Point::default(),
    Point::from_f32x2(tiny_skia_path::f32x2::new(1.0f32, 0.0f32)),
    Point { x: 0.0f32, y: 0.0f32 }, Point { x: 0.0f32, y: 0.0f32 },
    Point { x: 0.0f32, y: 0.0f32 }]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:85:14:
index out of bounds: the len is 1 but the index is 1
backtrace:
  tiny_skia_path::path_geometry::chop_quad_at (path/src/path_geometry.rs:85:14)
  repro_path_src_path_geometry_rs_85_14_447c28::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let int_rect = IntRect::from_xywh(0i32, 0i32, 1u32, 1u32).unwrap();
let int_rect2 = int_rect.translate_to(0i32, 0i32).unwrap();
let _ = int_rect2.inset(0i32, i32::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:126:13:
attempt to subtract with overflow
backtrace:
  &lt;tiny_skia_path::rect::IntRect&gt;::inset (path/src/rect.rs:126:13)
  repro_path_src_rect_rs_126_13_0f6793::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let f32_v = SCALAR_MAX;
let point = Point { x: f32_v, y: 0.0f32 };
tiny_skia_path::path_geometry::chop_cubic_at2(&amp;[point, point, point, point],
    NormalizedF32Exclusive::ANY, &amp;mut vec![point, Point::from((0.0f32, 1.0f32)),
    point, point, Point::zero(), point]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:179:5:
index out of bounds: the len is 6 but the index is 6
backtrace:
  tiny_skia_path::path_geometry::chop_cubic_at2 (path/src/path_geometry.rs:179:5)
  repro_path_src_path_geometry_rs_179_5_1381ed::main (src/main.rs:8:5)</pre></td></tr>
<tr><td valign="top"><pre>let int_rect = IntRect::from_ltrb(1i32, 0i32, 10000i32, 1i32).unwrap();
let _ = int_rect.translate(i32::MAX, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:142:28:
attempt to add with overflow
backtrace:
  &lt;tiny_skia_path::rect::IntRect&gt;::translate (path/src/rect.rs:142:28)
  repro_path_src_rect_rs_142_28_7a31e4::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let non_zero_rect = NonZeroRect::from_ltrb(0.0f32, 0.0f32, 1.1920929e-7f32,
    1.0f32).unwrap();
let size = Size::from_wh(1.0f32, 1e-40f32).unwrap();
let _ = size.scale_to(non_zero_rect.size());</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/size.rs:217:42:
called `Option::unwrap()` on a `None` value
backtrace:
  tiny_skia_path::size::size_scale_f64 (path/src/size.rs:217:42)
  &lt;tiny_skia_path::size::Size&gt;::scale_to (path/src/size.rs:158:9)
  repro_path_src_size_rs_217_42_0b5f0a::main (src/main.rs:7:18)</pre></td></tr>
<tr><td valign="top"><pre>let int_rect = IntRect::from_xywh(1i32, 0i32, 1u32, 1u32).unwrap();
let _ = int_rect.inset(i32::MAX, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:123:13:
attempt to add with overflow
backtrace:
  &lt;tiny_skia_path::rect::IntRect&gt;::inset (path/src/rect.rs:123:13)
  repro_path_src_rect_rs_123_13_786c4e::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let color = Color::from_rgba8(0u8, 0u8, 0u8, 0u8);
let premultiplied_color = color.premultiply();
let f32_v = premultiplied_color.blue();
tiny_skia_path::path_geometry::chop_quad_at(&amp;[], NormalizedF32Exclusive::ANY,
    &amp;mut [Point { x: f32_v, y: 0.0f32 }, Point::default(), Point::zero(),
    Point::from_xy(0.0f32, f32_v), Point { x: 0.0f32, y: 0.0f32 }]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:84:14:
index out of bounds: the len is 0 but the index is 0
backtrace:
  tiny_skia_path::path_geometry::chop_quad_at (path/src/path_geometry.rs:84:14)
  repro_path_src_path_geometry_rs_84_14_acd878::main (src/main.rs:9:5)</pre></td></tr>
<tr><td valign="top"><pre>let normalized_f32_exclusive = NormalizedF32Exclusive::ANY;
let point = &lt;tiny_skia_path::Point as ::core::ops::Add&lt;tiny_skia_path::Point&gt;&gt;::
    add(Point::from_xy(0.0f32, 0.0f32), Point::from_xy(0.0f32, 0.0f32));
tiny_skia_path::path_geometry::chop_cubic_at2(&amp;[point, point, point, point],
    normalized_f32_exclusive, &amp;mut vec![point, point, point, point, point]);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:178:5:
index out of bounds: the len is 5 but the index is 5
backtrace:
  tiny_skia_path::path_geometry::chop_cubic_at2 (path/src/path_geometry.rs:178:5)
  repro_path_src_path_geometry_rs_178_5_ed7d84::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rect::from_xywh(f32::MAX, 0.0f32, 0.0f32,
    0.0f32).unwrap().bbox_transform(NonZeroRect::from_xywh(0.0f32, 0.0f32,
    1000.0f32, 1.0f32).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:489:37:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;tiny_skia_path::rect::Rect&gt;::bbox_transform (path/src/rect.rs:489:37)
  repro_path_src_rect_rs_489_37_4b5742::main (src/main.rs:5:72)</pre></td></tr>
<tr><td valign="top"><pre>let int_rect = IntRect::from_xywh(0i32, 1i32, 1u32, 1u32).unwrap();
let _ = int_rect.inset(0i32, i32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:124:13:
attempt to add with overflow
backtrace:
  &lt;tiny_skia_path::rect::IntRect&gt;::inset (path/src/rect.rs:124:13)
  repro_path_src_rect_rs_124_13_faf526::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let size = Size::from_wh(f32::MAX, 1.0f32).unwrap();
let _ = size.expand_to(IntSize::from_wh(1u32, 100000000u32).unwrap().to_size());</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/size.rs:214:44:
called `Option::unwrap()` on a `None` value
backtrace:
  tiny_skia_path::size::size_scale_f64 (path/src/size.rs:214:44)
  &lt;tiny_skia_path::size::Size&gt;::expand_to (path/src/size.rs:163:9)
  repro_path_src_size_rs_214_44_3f049e::main (src/main.rs:6:18)</pre></td></tr>
<tr><td valign="top"><pre>let int_rect = IntRect::from_ltrb(0i32, -1000000000i32, 1i32, 0i32).unwrap();
let _ = int_rect.translate(0i32, i32::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:142:43:
attempt to add with overflow
backtrace:
  &lt;tiny_skia_path::rect::IntRect&gt;::translate (path/src/rect.rs:142:43)
  repro_path_src_rect_rs_142_43_fa9dd9::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let mut pixmap = Pixmap::new(1u32, 1u32).unwrap();
let mut pixmap_mut = pixmap.as_mut();
pixmap_mut.apply_mask(&amp;Mask::new(1000000000u32, 8u32).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  tiny_skia::mask::Mask::new (src/mask.rs:51)
  repro_out_of_memory_in_tiny_skia_mask_new_e554f5::main (src/main.rs:7)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let pixmap = Pixmap::new(1u32, 100000000u32).unwrap();
let _ = pixmap.encode_png();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  simd_adler32::imp::get_imp (src/imp/mod.rs:24)
  simd_adler32::Adler32::from_checksum (src/lib.rs:144)
  miniz_oxide::shared::update_adler32 (src/shared.rs:22)
  miniz_oxide::deflate::core::compress_inner (src/deflate/core.rs:2493)
  miniz_oxide::deflate::core::compress (src/deflate/core.rs:2411)
  miniz_oxide::deflate::stream::deflate (src/deflate/stream.rs:54)
  flate2::ffi::miniz_oxide::{impl#7}::compress (src/ffi/miniz_oxide.rs:172)
  flate2::ffi::DeflateBackend::compress_uninit&lt;flate2::ffi::miniz_oxide::Deflate&gt; (src/ffi/mod.rs:57)
  flate2::mem::Compress::compress_uninit (src/mem.rs:370)
  flate2::mem::{impl#1}::compress_vec::{closure#0} (src/mem.rs:391)
  flate2::mem::write_to_spare_capacity_of_vec&lt;core::result::Result&lt;flate2::mem::Status, flate2::mem::CompressError&gt;, flate2::mem::{impl#1}::compress_vec::{closure_env#0}&gt; (src/mem.rs:654)
  flate2::mem::Compress::compress_vec (src/mem.rs:389)
  flate2::zio::{impl#0}::run_vec (src/zio.rs:59)
  flate2::zio::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, flate2::mem::Compress&gt;::write_with_status&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, flate2::mem::Compress&gt; (flate2/src/zio.rs:233)
  flate2::zio::{impl#5}::write&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, flate2::mem::Compress&gt; (flate2/src/zio.rs:268)
  flate2::zlib::write::{impl#1}::write&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (flate2/src/zlib/write.rs:174)
  png::encoder::Writer&lt;&amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::write_image_data&lt;&amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (png/src/encoder.rs:826)
  tiny_skia::pixmap::PixmapRef::encode_png (src/pixmap.rs:424)
  tiny_skia::pixmap::Pixmap::encode_png (src/pixmap.rs:176)
  repro_timeout_in_tiny_skia_pixmap_encode_png_dd804c::main (src/main.rs:5)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let non_zero_rect = NonZeroRect::from_ltrb(0.0f32, f32::MIN, 1.0f32,
    -1e30f32).unwrap();
let _ = non_zero_rect.transform(Transform::from_skew(0.0f32, 1.0f32));</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/rect.rs:696:74:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;tiny_skia_path::rect::NonZeroRect&gt;::to_rect (path/src/rect.rs:696:74)
  &lt;tiny_skia_path::rect::NonZeroRect&gt;::transform (path/src/rect.rs:681:18)
  repro_path_src_rect_rs_696_74_8357fd::main (src/main.rs:6:27)</pre></td></tr>
<tr><td valign="top"><pre>let transform = Transform::from_skew(1.0f32, 1.0f32);
let transform2 = transform.pre_rotate(0.0f32);
let bool_v = transform2.has_skew();
let fill_rule = FillRule::default();
let path = PathBuilder::from_rect(Rect::from_ltrb(-1e30f32, 0.0f32, 0.0f32,
    1.0f32).unwrap());
let mut mask = Mask::decode_png(&amp;include_bytes!(
    "data_0.bin").to_vec()).unwrap();
mask.fill_path(&amp;path, fill_rule, bool_v, transform);</pre></td><td valign="top"><pre>thread 'main' panicked at tiny-skia/src/scan/path.rs:221:13:
assertion failed: edges[curr_idx].last_y &gt;= curr_y as i32
backtrace:
  tiny_skia::scan::path::walk_edges (tiny-skia/src/scan/path.rs:221:13)
  tiny_skia::scan::path::fill_path_impl (tiny-skia/src/scan/path.rs:188:5)
  tiny_skia::scan::path_aa::fill_path_impl (tiny-skia/src/scan/path_aa.rs:120:5)
  tiny_skia::scan::path_aa::fill_path (tiny-skia/src/scan/path_aa.rs:73:5)
  &lt;tiny_skia::mask::Mask&gt;::fill_path (tiny-skia/src/mask.rs:331:21)
  &lt;tiny_skia::mask::Mask&gt;::fill_path (tiny-skia/src/mask.rs:345:18)
  repro_tiny_skia_src_scan_path_rs_221_13_11b844::main (src/main.rs:14:10)</pre></td></tr>
<tr><td valign="top"><pre>let mask = Mask::new(1u32, 100000000u32).unwrap();
let _ = mask.encode_png();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  flate2::mem::{impl#1}::compress_vec::{closure#0} (src/mem.rs:392)
  flate2::mem::write_to_spare_capacity_of_vec&lt;core::result::Result&lt;flate2::mem::Status, flate2::mem::CompressError&gt;, flate2::mem::{impl#1}::compress_vec::{closure_env#0}&gt; (src/mem.rs:654)
  flate2::mem::Compress::compress_vec (src/mem.rs:389)
  flate2::zio::{impl#0}::run_vec (src/zio.rs:59)
  flate2::zio::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, flate2::mem::Compress&gt;::write_with_status&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, flate2::mem::Compress&gt; (flate2/src/zio.rs:233)
  flate2::zio::{impl#5}::write&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, flate2::mem::Compress&gt; (flate2/src/zio.rs:268)
  flate2::zlib::write::{impl#1}::write&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (flate2/src/zlib/write.rs:174)
  png::encoder::Writer&lt;&amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::write_image_data&lt;&amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (png/src/encoder.rs:827)
  tiny_skia::mask::Mask::encode_png (src/mask.rs:234)
  repro_timeout_in_tiny_skia_mask_encode_png_0b883b::main (src/main.rs:5)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let color_u8 = ColorU8::from_rgba(0u8, 0u8, 0u8, 0u8);
let u8_v = color_u8.alpha();
let color = Color::BLACK;
let _ = LinearGradient::new(Point::zero(), Point::default(),
    vec![GradientStop::new(1.0f32, color), GradientStop::new(0.0f32,
    Color::from_rgba8(u8_v, 0u8, 0u8, 0u8)), GradientStop::new(0.0f32,
    Color::WHITE)], SpreadMode::Reflect, Transform::default());</pre></td><td valign="top"><pre>thread 'main' panicked at tiny-skia/src/shaders/linear_gradient.rs:121:50:
called `Option::unwrap()` on a `None` value
backtrace:
  tiny_skia::shaders::linear_gradient::average_gradient_color::store_color (tiny-skia/src/shaders/linear_gradient.rs:121:50)
  tiny_skia::shaders::linear_gradient::average_gradient_color (tiny-skia/src/shaders/linear_gradient.rs:161:5)
  &lt;tiny_skia::shaders::linear_gradient::LinearGradient&gt;::new (tiny-skia/src/shaders/linear_gradient.rs:77:52)
  repro_tiny_skia_src_shaders_linear_gradient_rs_121_50_846c1c::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let line_cap = LineCap::default();
let mut path_stroker = PathStroker::new();
let _ = path_stroker.stroke(&amp;PathBuilder::from_circle(0.0f32, 0.0f32,
    1.0f32).unwrap().transform(Transform::from_scale(-1e30f32,
    4.5584144e30f32)).unwrap(), &amp;Stroke { width: 1.0f32, miter_limit: 0.0f32,
    line_cap: line_cap, line_join: LineJoin::Round, dash: None }, 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/path_geometry.rs:248:27:
called `Option::unwrap()` on a `None` value
backtrace:
  tiny_skia_path::path_geometry::find_quad_max_curvature (path/src/path_geometry.rs:248:27)
  tiny_skia_path::stroker::check_quad_linear (path/src/stroker.rs:1801:13)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::quad_to (path/src/stroker.rs:478:43)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::stroke_inner (path/src/stroker.rs:394:26)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::stroke (path/src/stroker.rs:301:14)
  repro_path_src_path_geometry_rs_248_27_d7e021::main (src/main.rs:11:26)</pre></td></tr>
<tr><td valign="top"><pre>let pixmap = Pixmap::new(1u32, 100000000u32).unwrap();
let pixmap_ref = pixmap.as_ref();
let _ = pixmap_ref.encode_png();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  flate2::ffi::initialize_buffer (src/ffi/mod.rs:10)
  flate2::ffi::DeflateBackend::compress_uninit&lt;flate2::ffi::miniz_oxide::Deflate&gt; (src/ffi/mod.rs:57)
  flate2::mem::Compress::compress_uninit (src/mem.rs:370)
  flate2::mem::{impl#1}::compress_vec::{closure#0} (src/mem.rs:391)
  flate2::mem::write_to_spare_capacity_of_vec&lt;core::result::Result&lt;flate2::mem::Status, flate2::mem::CompressError&gt;, flate2::mem::{impl#1}::compress_vec::{closure_env#0}&gt; (src/mem.rs:654)
  flate2::mem::Compress::compress_vec (src/mem.rs:389)
  flate2::zio::{impl#0}::run_vec (src/zio.rs:59)
  flate2::zio::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, flate2::mem::Compress&gt;::write_with_status&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, flate2::mem::Compress&gt; (flate2/src/zio.rs:233)
  flate2::zio::{impl#5}::write&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, flate2::mem::Compress&gt; (flate2/src/zio.rs:268)
  flate2::zlib::write::{impl#1}::write&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (flate2/src/zlib/write.rs:174)
  png::encoder::Writer&lt;&amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::write_image_data&lt;&amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (png/src/encoder.rs:827)
  tiny_skia::pixmap::PixmapRef::encode_png (src/pixmap.rs:424)
  repro_timeout_in_tiny_skia_pixmapref_encode_png_9300a3::main (src/main.rs:6)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![50u8, 50u8, 50u8, 50u8, 50u8];
let pixmap_ref = PixmapRef::from_bytes(&amp;vec, 1u32, 1u32).unwrap();
let pixmap = pixmap_ref.to_owned();
let pixmap_ref2 = pixmap.as_ref();
let _ = pixmap_ref2.pixel(0u32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at bytemuck/src/internal.rs:33:3:
cast_slice&gt;OutputSliceWouldHaveSlop
backtrace:
  bytemuck::internal::something_went_wrong::&lt;bytemuck::PodCastError&gt; (bytemuck/src/internal.rs:33:3)
  bytemuck::internal::cast_slice::&lt;u8, tiny_skia::color::PremultipliedColorU8&gt; (bytemuck/src/internal.rs:257:15)
  bytemuck::cast_slice::&lt;u8, tiny_skia::color::PremultipliedColorU8&gt; (bytemuck/src/lib.rs:413:12)
  &lt;tiny_skia::pixmap::PixmapRef&gt;::pixels (tiny-skia/src/pixmap.rs:377:9)
  &lt;tiny_skia::pixmap::PixmapRef&gt;::pixel (tiny-skia/src/pixmap.rs:372:14)
  repro_bytemuck_src_internal_rs_33_3_5e6286::main (src/main.rs:8:25)</pre></td></tr>
<tr><td valign="top"><pre>let mask = Mask::new(803485030u32, 1u32).unwrap();
let _ = mask.encode_png();</pre></td><td valign="top"><pre>out of memory: memory allocation of 803485030 bytes failed
backtrace:
  png::encoder::Writer&lt;&amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::write_image_data&lt;&amp;mut alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (png/src/encoder.rs:819)
  tiny_skia::mask::Mask::encode_png (src/mask.rs:234)
  repro_out_of_memory_in_tiny_skia_mask_encode_png_4ca3f8::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut path_builder = PathBuilder::default();
path_builder.quad_to(0.0f32, 0.0f32, 0.0f32, 0.0f32);
path_builder.cubic_to(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32);
let _ = PathStroker::default().stroke(&amp;path_builder.clone().finish().unwrap(),
    &amp;Stroke::default(), 1.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at path/src/stroker.rs:918:9:
assertion failed: self.segment_count &gt;= 0
backtrace:
  &lt;tiny_skia_path::stroker::PathStroker&gt;::pre_join_to (path/src/stroker.rs:918:9)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::line_to (path/src/stroker.rs:466:18)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::cubic_to (path/src/stroker.rs:558:18)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::stroke_inner (path/src/stroker.rs:398:26)
  &lt;tiny_skia_path::stroker::PathStroker&gt;::stroke (path/src/stroker.rs:301:14)
  repro_path_src_stroker_rs_918_9_575860::main (src/main.rs:9:36)</pre></td></tr>
</table>
