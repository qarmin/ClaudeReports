# ab_glyph - crashes

Source: https://github.com/alexheretic/ab-glyph @ 3eb21a592819d9ed78e9a13ad7392d46c60ce39e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Rasterizer::new(1usize, 1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000016 bytes failed
backtrace:
  ab_glyph_rasterizer::raster::Rasterizer::new (src/raster.rs:47)
  repro_out_of_memory_in_ab_glyph_rasterizer_rasterizer_new_25db9f::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut rasterizer = Rasterizer::new(0usize, 0usize);
rasterizer.reset(1usize, 1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000016 bytes failed
backtrace:
  ab_glyph_rasterizer::raster::Rasterizer::reset (src/raster.rs:66)
  repro_out_of_memory_in_ab_glyph_rasterizer_rasterizer_reset_a2528d::main (src/main.rs:5)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut rasterizer = Rasterizer::new(1usize, 10000usize);
rasterizer.draw_line(ab_glyph_rasterizer::point(f32::MAX, 356.0f32),
    Point::default());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  ab_glyph_rasterizer::raster::Rasterizer::draw_line_scalar (src/raster.rs:159)
  ab_glyph_rasterizer::raster::draw_line_avx2 (src/raster.rs:309)
  ab_glyph_rasterizer::raster::Rasterizer::draw_line (src/raster.rs:100)
  repro_timeout_in_ab_glyph_rasterizer_rasterizer_draw_line_24edcc::main (src/main.rs:6)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rasterizer::new(10000000000000000000usize, 3usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rasterizer/src/raster.rs:47:26:
attempt to multiply with overflow
backtrace:
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::new (rasterizer/src/raster.rs:47:26)
  repro_rasterizer_src_raster_rs_47_26_613453::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rasterizer::new(1000000000usize, 4294967295usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:24:31)
capacity overflow
backtrace:
  &lt;f32 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:24:31)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::new (rasterizer/src/raster.rs:47:16)
  repro_rasterizer_src_raster_rs_47_16_2b7017::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let rect = Rect { min: ab_glyph_rasterizer::point(1.0f32, 0.0f32),
    max: Point::default() };
let f32_v = rect.width();
let mut rasterizer = Rasterizer::new(0usize, 1usize);
rasterizer.draw_quad(ab_glyph_rasterizer::point(0.0f32, f32_v),
    Point::default(), Point { x: 1e30f32, y: 0.0f32 });</pre></td><td valign="top"><pre>thread 'main' panicked at rasterizer/src/raster.rs:187:17:
attempt to add with overflow
backtrace:
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_quad (rasterizer/src/raster.rs:187:17)
  repro_rasterizer_src_raster_rs_187_17_e35e9f::main (src/main.rs:9:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut rasterizer = Rasterizer::new(0usize, 0usize);
rasterizer.reset(10000000000000000000usize, 255usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rasterizer/src/raster.rs:66:23:
attempt to multiply with overflow
backtrace:
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::reset (rasterizer/src/raster.rs:66:23)
  repro_rasterizer_src_raster_rs_66_23_c0a698::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let point = &lt;ab_glyph_rasterizer::Point as ::core::ops::
    Sub&lt;ab_glyph_rasterizer::Point&gt;&gt;::sub(Point { x: f32::MAX, y: 0.0f32 },
    Point::default());
let mut rasterizer = Rasterizer::new(0usize, 1usize);
rasterizer.draw_line(Point { x: 0.0f32, y: 6189.0f32 }, point);</pre></td><td valign="top"><pre>thread 'main' panicked at rasterizer/src/raster.rs:143:23:
attempt to add with overflow
backtrace:
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_line_scalar (rasterizer/src/raster.rs:143:23)
  ab_glyph_rasterizer::raster::draw_line_avx2 (rasterizer/src/raster.rs:309:10)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_line (rasterizer/src/raster.rs:100:18)
  repro_rasterizer_src_raster_rs_143_23_987d12::main (src/main.rs:7:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut rasterizer = Rasterizer::new(0usize, 1usize);
rasterizer.reset(2147483648usize, 2147483648usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from rasterizer/src/raster.rs:66:16)
capacity overflow
backtrace:
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::reset (rasterizer/src/raster.rs:66:16)
  repro_rasterizer_src_raster_rs_66_16_c50612::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut rasterizer = Rasterizer::new(1usize, 10000usize);
rasterizer.draw_cubic(Point { x: 1.0f32, y: 65536.0f32 }, Point { x: 0.0f32,
    y: -7110000000.0f32 }, Point { x: 0.0f32, y: 0.0f32 },
    Point { x: -3.736144e16f32, y: 0.0f32 });</pre></td><td valign="top"><pre>thread 'main' panicked at rasterizer/src/raster.rs:165:35:
attempt to add with overflow
backtrace:
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_line_scalar (rasterizer/src/raster.rs:165:35)
  ab_glyph_rasterizer::raster::draw_line_avx2 (rasterizer/src/raster.rs:309:10)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_line (rasterizer/src/raster.rs:100:18)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::tessellate_cubic (rasterizer/src/raster.rs:241:18)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::tessellate_cubic (rasterizer/src/raster.rs:238:18)
... (previous line repeated 7 more times)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::default();
let mut rasterizer = Rasterizer::new(1000usize, 2usize);
rasterizer.draw_cubic(ab_glyph_rasterizer::point(0.0f32, 0.0f32), point,
    &lt;ab_glyph_rasterizer::Point as ::core::ops::Sub&lt;ab_glyph_rasterizer::
    Point&gt;&gt;::sub(ab_glyph_rasterizer::point(0.0f32, 0.0f32),
    Point { x: 256.0f32, y: 0.0f32 }), ab_glyph_rasterizer::point(0.0f32,
    13.0f32));</pre></td><td valign="top"><pre>thread 'main' panicked at rasterizer/src/raster.rs:160:43:
attempt to add with overflow
backtrace:
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_line_scalar (rasterizer/src/raster.rs:160:43)
  ab_glyph_rasterizer::raster::draw_line_avx2 (rasterizer/src/raster.rs:309:10)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_line (rasterizer/src/raster.rs:100:18)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::tessellate_cubic (rasterizer/src/raster.rs:241:18)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::tessellate_cubic (rasterizer/src/raster.rs:239:18)
... (previous line repeated 1 more times)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::tessellate_cubic (rasterizer/src/raster.rs:238:18)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_cubic (rasterizer/src/raster.rs:214:14)
  repro_rasterizer_src_raster_rs_160_43_b5c6e5::main (src/main.rs:7:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut rasterizer = Rasterizer::new(0usize, 10000usize);
rasterizer.draw_quad(ab_glyph_rasterizer::point(f32::INFINITY, 65536.0f32),
    ab_glyph_rasterizer::point(0.0f32, f32::NAN), Point::default());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  ab_glyph_rasterizer::raster::Rasterizer::draw_line_scalar (src/raster.rs:108)
  ab_glyph_rasterizer::raster::draw_line_avx2 (src/raster.rs:309)
  ab_glyph_rasterizer::raster::Rasterizer::draw_line (src/raster.rs:100)
  ab_glyph_rasterizer::raster::Rasterizer::draw_quad (src/raster.rs:197)
  repro_timeout_in_ab_glyph_rasterizer_rasterizer_draw_quad_6eb8f5::main (src/main.rs:6)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut rasterizer = Rasterizer::new(0usize, usize::MAX);
rasterizer.draw_cubic(
    &lt;ab_glyph_rasterizer::Point as ::core::ops::Sub&lt;ab_glyph_rasterizer::
    Point&gt;&gt;::sub(Point { x: 2.8e16f32, y: 1e-40f32 }, Point { x: f32::NAN,
    y: -2187873800.0f32 }), Point::default(), Point { x: -1e30f32, y: 1e30f32 },
    &lt;ab_glyph_rasterizer::Point as ::core::ops::Sub&lt;ab_glyph_rasterizer::
    Point&gt;&gt;::sub(ab_glyph_rasterizer::point(1.0f32, -6.4f32),
    ab_glyph_rasterizer::point(65536.0f32, 5.2e-6f32)));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  ab_glyph_rasterizer::raster::Rasterizer::draw_line_scalar (src/raster.rs:131)
  ab_glyph_rasterizer::raster::draw_line_avx2 (src/raster.rs:309)
  ab_glyph_rasterizer::raster::Rasterizer::draw_line (src/raster.rs:100)
  ab_glyph_rasterizer::raster::Rasterizer::tessellate_cubic (src/raster.rs:241)
  ab_glyph_rasterizer::raster::Rasterizer::draw_cubic (src/raster.rs:214)
  repro_timeout_in_ab_glyph_rasterizer_rasterizer_draw_cubic_42c57a::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let point = &lt;ab_glyph_rasterizer::Point as ::core::ops::
    Sub&lt;ab_glyph_rasterizer::Point&gt;&gt;::sub(Point::default(), Point { x: 255.0f32,
    y: -53.3f32 });
let mut rasterizer = Rasterizer::new(100usize, 100usize);
rasterizer.draw_cubic(point, Point { x: 0.0f32, y: 0.0f32 },
    ab_glyph_rasterizer::point(f32::NAN, 0.0f32), Point { x: 0.0f32,
    y: 256.0f32 });</pre></td><td valign="top"><pre>thread 'main' panicked at rasterizer/src/raster.rs:163:39:
attempt to add with overflow
backtrace:
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_line_scalar (rasterizer/src/raster.rs:163:39)
  ab_glyph_rasterizer::raster::draw_line_avx2 (rasterizer/src/raster.rs:309:10)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_line (rasterizer/src/raster.rs:100:18)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::tessellate_cubic (rasterizer/src/raster.rs:241:18)
  &lt;ab_glyph_rasterizer::raster::Rasterizer&gt;::draw_cubic (rasterizer/src/raster.rs:214:14)
  repro_rasterizer_src_raster_rs_163_39_deaab6::main (src/main.rs:7:16)</pre></td></tr>
</table>
