# kurbo - crashes

Source: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let simplify_bez_path = SimplifyBezPath::new(vec![PathEl::QuadTo(
    Point::default(), Point::ORIGIN)]);
let _ = &lt;kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit&gt;::
    sample_pt_deriv(&amp;simplify_bez_path, 1.5763829091294793e257f64);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/simplify.rs:198:23:
index out of bounds: the len is 1 but the index is 18446744073709551615
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[kurbo::simplify::SimplifyCubic]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[kurbo::simplify::SimplifyCubic] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;kurbo::simplify::SimplifyBezPath as kurbo::fit::ParamCurveFit&gt;::sample_pt_deriv (kurbo/src/simplify.rs:198:23)
  repro_kurbo_src_simplify_rs_198_23_13279b::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point { x: 0.0f64, y: 0.0f64 };
let path_el = PathEl::QuadTo(point.expand(), point);
let _ = kurbo::fit_to_cubic(&amp;SimplifyBezPath::new(vec![path_el]),
    85.0f64..0.0f64, 0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/simplify.rs:208:23:
index out of bounds: the len is 1 but the index is 85
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[kurbo::simplify::SimplifyCubic]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[kurbo::simplify::SimplifyCubic] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;kurbo::simplify::SimplifyBezPath as kurbo::fit::ParamCurveFit&gt;::sample_pt_tangent (kurbo/src/simplify.rs:208:23)
  kurbo::fit::fit_to_cubic::&lt;kurbo::simplify::SimplifyBezPath&gt; (kurbo/src/fit.rs:392:24)
  repro_kurbo_src_simplify_rs_208_23_a10a9e::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let path_el = PathEl::ClosePath;
let _ = SimplifyBezPath::new(vec![path_el]);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/bezpath.rs:902:42:
Can't start a segment on a ClosePath
backtrace:
  &lt;kurbo::bezpath::Segments&lt;alloc::vec::into_iter::IntoIter&lt;kurbo::bezpath::PathEl&gt;&gt; as core::iter::traits::iterator::Iterator&gt;::next::{closure#0} (kurbo/src/bezpath.rs:902:42)
  &lt;kurbo::bezpath::Segments&lt;alloc::vec::into_iter::IntoIter&lt;kurbo::bezpath::PathEl&gt;&gt; as core::iter::traits::iterator::Iterator&gt;::next (kurbo/src/bezpath.rs:896:49)
  &lt;kurbo::simplify::SimplifyBezPath&gt;::new::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt; (kurbo/src/simplify.rs:168:14)
  repro_kurbo_src_bezpath_rs_902_42_6d59e5::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BezPath::with_capacity(1000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from kurbo/src/bezpath.rs:192:17)
capacity overflow
backtrace:
  &lt;kurbo::bezpath::BezPath&gt;::with_capacity (kurbo/src/bezpath.rs:192:17)
  repro_kurbo_src_bezpath_rs_192_17_e542fd::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit&gt;::
    sample_pt_deriv(&amp;SimplifyBezPath::new(vec![]), 0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/simplify.rs:195:13:
attempt to subtract with overflow
backtrace:
  &lt;kurbo::simplify::SimplifyBezPath as kurbo::fit::ParamCurveFit&gt;::sample_pt_deriv (kurbo/src/simplify.rs:195:13)
  repro_kurbo_src_simplify_rs_195_13_b419ef::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit&gt;::
    moment_integrals(&amp;SimplifyBezPath::new(vec![]), f64::NAN..0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/simplify.rs:185:36:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[kurbo::simplify::SimplifyCubic]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[kurbo::simplify::SimplifyCubic] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;kurbo::simplify::SimplifyBezPath&gt;::moment_integrals (kurbo/src/simplify.rs:185:36)
  &lt;kurbo::simplify::SimplifyBezPath as kurbo::fit::ParamCurveFit&gt;::moment_integrals (kurbo/src/simplify.rs:221:18)
  repro_kurbo_src_simplify_rs_185_36_9113e2::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit&gt;::
    moment_integrals(&amp;SimplifyBezPath::new(vec![PathEl::LineTo(Point::ORIGIN)]),
    0.0f64..1e300f64);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/simplify.rs:228:42:
index out of bounds: the len is 1 but the index is 18446744073709551614
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[kurbo::simplify::SimplifyCubic]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[kurbo::simplify::SimplifyCubic] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;kurbo::simplify::SimplifyBezPath as kurbo::fit::ParamCurveFit&gt;::moment_integrals (kurbo/src/simplify.rs:228:42)
  repro_kurbo_src_simplify_rs_228_42_78f49c::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::fit_to_cubic(&amp;SimplifyBezPath::new(vec![]), 0.0f64..0.0f64,
    0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/simplify.rs:205:13:
attempt to subtract with overflow
backtrace:
  &lt;kurbo::simplify::SimplifyBezPath as kurbo::fit::ParamCurveFit&gt;::sample_pt_tangent (kurbo/src/simplify.rs:205:13)
  kurbo::fit::fit_to_cubic::&lt;kurbo::simplify::SimplifyBezPath&gt; (kurbo/src/fit.rs:392:24)
  repro_kurbo_src_simplify_rs_205_13_c6dabc::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::path_elements(&amp;BezPath::with_capacity(
    1000000000usize), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_bezpath_path_elements_49df64::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(&amp;BezPath::with_capacity(
    1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_bezpath_as_circle_0f2fad::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::cubics_to_quadratic_splines(&amp;[], 0.805f64);
let _ = Moments { moment_x: 1e300f64, moment_y: 0.631f64, moment_xx: 1e300f64,
    moment_xy: 243.0f64, moment_yy: f64::MAX };
let _ = Point::ORIGIN;
let _ = MAX_EXTREMA;
let _ = &lt;kurbo::Line as kurbo::ParamCurveDeriv&gt;::deriv(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(
    &amp;&lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(
    &amp;CircleSegment { center: Point::ZERO, outer_radius: 876.0f64,
    inner_radius: f64::NEG_INFINITY, start_angle: 2.2250738585072014e-308f64,
    sweep_angle: 0.047f64 }, f64::MIN)).unwrap());
let _ = Join::Round;</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_line_deriv_79de84::main (src/main.rs:12)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let bez_path = &lt;kurbo::Ellipse as kurbo::Shape&gt;::into_path(
    Ellipse::default().with_radii(Affine::new([486.0f64,
    -9.36676457517332e-32f64, 3.914127681039034e297f64,
    2.2250738585072014e-308f64, 0.9999999999999999f64,
    0.0f64]).then_rotate_about(0.29f64, Point::ZERO).translation()), 1e-310f64);
let axis = Axis::Vertical;
let axis2 = axis.cross();
let _ = axis2.cross();
let _ = SimplifyOptLevel::Optimize;
let _ = &lt;kurbo::PathSeg as kurbo::Shape&gt;::as_circle(&amp;bez_path.get_seg(
    255usize).unwrap());
let _ = Arc::new(Vec2::from((68.9f64, 267.0f64)).to_point(),
    Vec2::from((-56.4f64, 704.0f64)), 29.7f64, f64::MAX, 10000000000.0f64);
let _ = Moments::default();
let _ = StrokeCtx::default();</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_ellipse_into_path_1d350e::main (src/main.rs:12)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(
    &amp;&lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(&amp;Triangle::from_coords((
    0.0f64, 0.0f64), (0.0f64, 0.0f64), (0.0f64,
    0.0f64)).inscribed_circle().segment(1.0f64, 1.0f64, 1.0f64), 0.0f64));</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_bezpath_as_line_c8cc2a::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Ellipse as kurbo::Shape&gt;::to_path(&amp;Ellipse::from_affine(
    Affine::FLIP_Y), -0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_ellipse_to_path_b9cd6a::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let bez_path = &lt;kurbo::Rect as kurbo::Shape&gt;::into_path(Rect::new(0.0f64,
    f64::MIN, 1.0f64, 0.0f64), 1.0f64);
let path_seg = bez_path.get_seg(1usize).unwrap();
let path_seg2 = path_seg.reverse();
let cubic_bez = path_seg2.to_cubic();
let _ = cubic_bez.approx_spline(0.0f64);</pre></td><td valign="top"><pre>stack overflow: thread 'main' (236842) has overflowed its stack
backtrace:
  kurbo::cubicbez::{impl#3}::eval (src/cubicbez.rs:550)
  kurbo::cubicbez::{impl#3}::subdivide (src/cubicbez.rs:573)
  kurbo::cubicbez::CubicBez::fit_inside (src/cubicbez.rs:336)
  kurbo::cubicbez::CubicBez::fit_inside (src/cubicbez.rs:337)
... (previous line repeated 56 more times)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::common::solve_itp(|_| 1 as f64, 0.0f64, 0.0f64, 0.0f64,
    1000000000usize, 0.0f64, 0.0f64, 0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/common.rs:715:40:
attempt to shift left with overflow
backtrace:
  kurbo::common::solve_itp_fallible::&lt;core::convert::Infallible, kurbo::common::solve_itp&lt;repro_kurbo_src_common_rs_715_40_608e30::main::{closure#0}&gt;::{closure#0}&gt; (kurbo/src/common.rs:715:40)
  kurbo::common::solve_itp::&lt;repro_kurbo_src_common_rs_715_40_608e30::main::{closure#0}&gt; (kurbo/src/common.rs:689:18)
  repro_kurbo_src_common_rs_715_40_608e30::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::PathSeg as kurbo::ParamCurve&gt;::subdivide(
    &amp;BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_pathseg_subdivide_0e2e64::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let triangle = Triangle::from_coords((0.0f64, 0.0f64), (0.0f64, 1.0f64),
    (0.0f64, 1.0f64));
let circle = triangle.circumscribed_circle();
let _ = &lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(&amp;circle.segment(1.0f64,
    0.0f64, 1.0f64), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_circlesegment_to_path_a9367c::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let circle_segment = CircleSegment { center: Point::from((0.0f64, 0.0f64)),
    outer_radius: 1.0f64, inner_radius: f64::NAN, start_angle: 0.384f64,
    sweep_angle: 42.2f64 };
let arc = circle_segment.outer_arc();
arc.to_cubic_beziers::&lt;fn(kurbo::Point, kurbo::Point, kurbo::Point)&gt;(1e-310f64,
    |_, _, _| ());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::sample_ellipse (src/arc.rs:165)
  kurbo::arc::{impl#1}::next (src/arc.rs:145)
  kurbo::arc::Arc::to_cubic_beziers&lt;fn(kurbo::point::Point, kurbo::point::Point, kurbo::point::Point)&gt; (kurbo/src/arc.rs:113)
  repro_timeout_in_kurbo_arc_to_cubic_beziers_a04320::main (src/main.rs:7)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::CircleSegment as kurbo::Shape&gt;::into_path(CircleSegment::new(
    Point::ZERO, 0.0f64, 0.0f64, 0.0f64, f64::NEG_INFINITY), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_circlesegment_into_path_e164cb::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BezPath::with_capacity(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_bezpath_with_capacity_ed9026::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let ellipse = Ellipse::default();
let _ = &lt;kurbo::TranslateScale as ::core::ops::Mul&lt;kurbo::PathSeg&gt;&gt;::mul(
    TranslateScale::default(),
    &lt;kurbo::Ellipse as kurbo::Shape&gt;::to_path(&amp;ellipse,
    0.0f64).get_seg(0usize).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_translatescale_mul_kurbo_pathseg_c7a809::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::into_path(
    Triangle::EQUILATERAL.circumscribed_circle().segment(0.0f64, 0.0f64,
    1.0f64).outer_arc(), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_arc_into_path_0407a1::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let vec2 = Vec2::ZERO;
let f64_v = vec2.get_coord(Axis::Horizontal);
let _ = Size::default();
let _ = LineIntersection { line_t: -1.5520280845898103e-89f64,
    segment_t: 0.9999999999999999f64 };
let _ = Triangle::from_coords((423.0f64, -0.0f64), (f64::INFINITY, 890.0f64),
    (0.625f64, -50.5f64));
let axis = Axis::Vertical;
let _ = axis.pack_size(-14.7f64, f64::NAN);
let rounded_rect = RoundedRect::new(4.289189471386518e-131f64, 0.285f64,
    -0.0f64, 2.2250738585072014e-308f64,
    RoundedRectRadii { top_left: -8.797486174408505e-240f64,
    top_right: 102.0f64, bottom_right: 925.0f64, bottom_left: f64_v });
let _ = &lt;kurbo::PathSeg as kurbo::ParamCurve&gt;::subdivide(&amp;PathSeg::from(
    &lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(
    &amp;&lt;kurbo::RoundedRect as kurbo::Shape&gt;::to_path(&amp;rounded_rect,
    1e-310f64)).unwrap()));</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;kurbo::rounded_rect::RoundedRectPathIter&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::rounded_rect::RoundedRect&gt; (kurbo/src/shape.rs:68)
  repro_timeout_in_kurbo_pathseg_subdivide_6a1fb2::main (src/main.rs:19)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PathEl::MoveTo(RoundedRect::default().origin());
let line_intersection = LineIntersection { line_t: 10000000000.0f64,
    segment_t: f64::MAX };
let _ = line_intersection.is_finite();
let _ = MAX_EXTREMA;
let rect = Rect::new(1.0f64, 0.0f64, 207249957868.58365f64, f64::MAX);
let ellipse = rect.to_ellipse();
let point = ellipse.center();
let _ = CuspType::Loop;
... (9 lines omitted)
let _ = Diagonal2::new(1e-310f64, 1.1946697462982718e140f64);
let _ = rounded_rect_radii.as_single_radius();
let affine3 = affine2.pre_scale_non_uniform(50.5f64, f64::MIN);
let _ = affine3.with_translation(ellipse.with_center(Point::from((1.0000001f32,
    f32::NEG_INFINITY))).radii().rotate_scale(Vec2::new(34.8f64, 340.0f64)));
let _ = Cap::Butt;
let _ = Join::Bevel;
let bez_path = &lt;kurbo::BezPath as kurbo::Shape&gt;::into_path(
    &lt;kurbo::Arc as kurbo::Shape&gt;::into_path(Arc { center: point,
    radii: Vec2 { x: f64::MAX, y: 2.2250738585072014e-308f64 },
    start_angle: 5.9068586614308054e-229f64,
    sweep_angle: 2.2250738585072014e-308f64, x_rotation: f64::NAN }, 0.0f64),
    -1e300f64);
let _ = bez_path.is_finite();</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:83)
  repro_timeout_in_kurbo_bezpath_as_rounded_rect_89663f::main (src/main.rs:36)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let svg_parse_error = SvgParseError::Wrong;
let bez_path = &lt;kurbo::CircleSegment as kurbo::Shape&gt;::into_path(
    CircleSegment::new(
    &lt;kurbo::Diagonal2 as ::core::ops::Mul&lt;kurbo::Point&gt;&gt;::mul(Diagonal2::new(
    f64::INFINITY, 1.0f64), Point { x: 0.241f64, y: 0.0f64 }),
    9007199254740992.0f64, 0.836f64, 0.5f64, -1.4174536431560663e60f64),
    10000000000.0f64);
let _ = &lt;kurbo::Moments as ::core::ops::Add&lt;kurbo::Moments&gt;&gt;::add(
    Moments { moment_x: -1e300f64, moment_y: 1.0f64, moment_xx: 0.5f64,
    moment_xy: 1.0f64, moment_yy: f64::MAX }, Moments::default());
let f64_v = DEFAULT_ACCURACY;
let _ = LineIntersection { line_t: -0.0f64, segment_t: 308.0f64 };
let diagonal2 = Diagonal2 { xx: 0.308f64, yy: 0.182f64 };
let diagonal22 = diagonal2.abs();
let vec2 = diagonal22.scale_normal(Vec2::from_angle(f64_v));
let point = Point::from((8.3f64, 1e-310f64));
let stroke_ctx = StrokeCtx::default();
let _ = stroke_ctx.output();
let _ = Cap::Round;
let _ = SimplifyOptLevel::Subdivide;
let _ = &lt;kurbo::PathSeg as kurbo::ParamCurveNearest&gt;::nearest(&amp;PathSeg::from(
    &lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;bez_path).unwrap()), point,
    -1.0f64);
let f64_v2 = vec2.length();
let _ = GAUSS_LEGENDRE_COEFFS_8_HALF.to_vec();
let _ = &lt;kurbo::Circle as kurbo::Shape&gt;::as_circle(
    &amp;&lt;kurbo::Rect as kurbo::Shape&gt;::as_circle(
    &amp;&lt;kurbo::Arc as kurbo::Shape&gt;::as_rect(&amp;Arc::from_svg_arc(
    &amp;SvgArc { from: point, to: point, radii: vec2, x_rotation: f64_v2,
    large_arc: false, sweep: true }).unwrap()).unwrap()).unwrap());
let _ = svg_parse_error.to_string();
let _ = SvgParseError::UninitializedPath;</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:83)
  repro_timeout_in_kurbo_circlesegment_into_path_86d55c::main (src/main.rs:19)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::bounding_box(&amp;BezPath::with_capacity(
    1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_bezpath_bounding_box_c57f4a::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::common::solve_itp(|_| 1 as f64, 0.0f64, 1.0f64, -0.0f64, 1usize,
    0.0f64, f64::MIN, 0.0f64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::common::solve_itp_fallible&lt;core::convert::Infallible, kurbo::common::solve_itp::{closure_env#0}&lt;repro_timeout_in_kurbo_common_solve_itp_3e7515::main::{closure_env#0}&gt;&gt; (kurbo/src/common.rs:746)
  kurbo::common::solve_itp&lt;repro_timeout_in_kurbo_common_solve_itp_3e7515::main::{closure_env#0}&gt; (kurbo/src/common.rs:689)
  repro_timeout_in_kurbo_common_solve_itp_3e7515::main (src/main.rs:2)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::default();
let _ = kurbo::expand_path(Circle { center: point, radius: 1.0f64 },
    Diagonal2::new(0.0f64, 0.0f64), Join::Bevel, 0.0f64, 0.0f64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::bezpath::Segments&lt;kurbo::bezpath::CloseSubpaths&lt;kurbo::circle::CirclePathIter&gt;&gt;::area&lt;kurbo::bezpath::CloseSubpaths&lt;kurbo::circle::CirclePathIter&gt;&gt; (kurbo/src/bezpath.rs:944)
  kurbo::expand::expand_path&lt;kurbo::circle::Circle&gt; (kurbo/src/expand.rs:134)
  repro_timeout_in_kurbo_expand_path_circle_b3e158::main (src/main.rs:8)
  (6 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BezPath::from_vec(vec![&lt;kurbo::Rect as kurbo::Shape&gt;::to_path(
    &amp;Rect::default(), 0.0f64).pop().unwrap()]);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/bezpath.rs:208:9:
BezPath must begin with MoveTo
backtrace:
  &lt;kurbo::bezpath::BezPath&gt;::from_vec (kurbo/src/bezpath.rs:208:9)
  repro_kurbo_src_bezpath_rs_208_9_6aae70::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::into_path(
    &lt;kurbo::CircleSegment as kurbo::Shape&gt;::into_path(
    CircleSegment { center: Point::new(0.0f64, 0.0f64), outer_radius: 0.0f64,
    inner_radius: 0.0f64, start_angle: 0.0f64, sweep_angle: f64::NEG_INFINITY },
    0.0f64), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_bezpath_into_path_0836d5::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::RoundedRect as kurbo::Shape&gt;::into_path(RoundedRect::new(1.0f64,
    1.0f64, 0.0f64, 0.0f64, RoundedRectRadii::new(0.0f64, 0.0f64, 1.0f64,
    1.0f64)), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;kurbo::rounded_rect::RoundedRectPathIter&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::rounded_rect::RoundedRect&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::rounded_rect::RoundedRect&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_roundedrect_into_path_388ef1::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::expand_path_signed(CircleSegment::new(Point::from((0.0f32,
    0.0f32)), 1.0f64, 1.0f64, 0.0f64, 1.0f64).outer_arc(),
    Diagonal2 { xx: 0.0f64, yy: 0.0f64 }, Join::Bevel, 0.0f64, 0.0f64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::expand::{impl#3}::mul (src/expand.rs:104)
  kurbo::expand::ExpandCtx::do_cubic (src/expand.rs:245)
  kurbo::expand::expand_path_signed&lt;kurbo::arc::Arc&gt; (kurbo/src/expand.rs:176)
  repro_timeout_in_kurbo_expand_path_signed_arc_ea8cb0::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::common::solve_itp(|_| 1 as f64, 0.0f64, f64::NEG_INFINITY,
    -1.0f64, 1usize, 0.0f64, 0.0f64, 0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/common.rs:714:16:
attempt to add with overflow
backtrace:
  kurbo::common::solve_itp_fallible::&lt;core::convert::Infallible, kurbo::common::solve_itp&lt;repro_kurbo_src_common_rs_714_16_465a3a::main::{closure#0}&gt;::{closure#0}&gt; (kurbo/src/common.rs:714:16)
  kurbo::common::solve_itp::&lt;repro_kurbo_src_common_rs_714_16_465a3a::main::{closure#0}&gt; (kurbo/src/common.rs:689:18)
  repro_kurbo_src_common_rs_714_16_465a3a::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::as_rounded_rect(
    &amp;BezPath::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_bezpath_as_rounded_rect_7c37a1::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::PathSeg as kurbo::Shape&gt;::path_elements(
    &amp;&lt;kurbo::Ellipse as kurbo::Shape&gt;::to_path(&amp;Ellipse::from_affine(
    Affine::rotate(0.0f64)), 0.0f64).get_seg(0usize).unwrap(), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_pathseg_path_elements_5211eb::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::perimeter(
    &amp;Arc { center: Axis::Vertical.pack_vec2(0.0f64, 0.0f64).to_point(),
    radii: Vec2::splat(f64::INFINITY), start_angle: 0.0f64, sweep_angle: 1.0f64,
    x_rotation: 0.0f64 }, 2.220446049250313e-16f64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::cubicbez::arclen_rec (src/cubicbez.rs:651)
  kurbo::cubicbez::arclen_rec (src/cubicbez.rs:665)
... (previous line repeated 19 more times)
  kurbo::cubicbez::{impl#5}::arclen (src/cubicbez.rs:674)
  kurbo::bezpath::{impl#17}::arclen (src/bezpath.rs:1006)
  kurbo::bezpath::{impl#15}::perimeter::{closure#0}&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:939)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::perimeter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:... (+4 chars)
  kurbo::arc::{impl#3}::perimeter (kurbo/src/arc.rs:215)
  repro_timeout_in_kurbo_arc_perimeter_ba15d1::main (src/main.rs:6)
  (11 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let affine = Affine::scale(1.0f64);
let affine2 = affine.pre_scale(0.0f64);
let f64_v = affine2.frobenius_norm_squared();
let circle_segment = CircleSegment { center: Size::default().to_rounded_rect(
    RoundedRectRadii::from_single_radius(0.0f64)).origin(),
    outer_radius: 0.0f64, inner_radius: 0.0f64, start_angle: 0.0f64,
    sweep_angle: f64_v };
let ellipse = Ellipse::from_affine(affine);
let _ = &lt;kurbo::CircleSegment as kurbo::Shape&gt;::winding(&amp;circle_segment,
    &lt;kurbo::PathSeg as kurbo::ParamCurve&gt;::end(
    &amp;&lt;kurbo::Ellipse as kurbo::Shape&gt;::into_path(ellipse,
    0.0f64).get_seg(0usize).unwrap()));</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_circlesegment_winding_3af68c::main (src/main.rs:13)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Ellipse as kurbo::Shape&gt;::perimeter(&amp;Ellipse::default(),
    f64::MIN);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::ellipse::agm_elliptic_perimeter (src/ellipse.rs:413)
  kurbo::ellipse::{impl#5}::perimeter (kurbo/src/ellipse.rs:286)
  repro_timeout_in_kurbo_ellipse_perimeter_e3928f::main (src/main.rs:4)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::QuadBez as kurbo::ParamCurve&gt;::subdivide(
    &amp;QuadBez { p0: BezPath::with_capacity(
    1000000000usize).current_position().unwrap(), p1: Point { x: 0.0f64,
    y: 0.0f64 }, p2: Point::new(0.0f64, 0.0f64) });</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_quadbez_subdivide_6d94d5::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::simplify::simplify_bezpath(vec![PathEl::LineTo(Point::from((
    0.0f32, 0.0f32)))], 0.0f64, &amp;SimplifyOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/simplify.rs:315:36:
called `Option::unwrap()` on a `None` value
backtrace:
  kurbo::simplify::simplify_bezpath::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt; (kurbo/src/simplify.rs:315:36)
  repro_kurbo_src_simplify_rs_315_36_017632::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut bez_path = BezPath::with_capacity(0usize);
bez_path.push(PathEl::QuadTo(Axis::Horizontal.pack_point(0.0f64, 0.0f64),
    Point::ZERO));</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/bezpath.rs:223:9:
BezPath must begin with MoveTo
backtrace:
  &lt;kurbo::bezpath::BezPath&gt;::push (kurbo/src/bezpath.rs:223:9)
  repro_kurbo_src_bezpath_rs_223_9_915335::main (src/main.rs:8:14)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::contains(&amp;Arc { center: Point::default(),
    radii: Vec2::new(0.0f64, 0.0f64), start_angle: 0.0f64,
    sweep_angle: f64::MIN, x_rotation: 0.0f64 }, Point::ORIGIN);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::common::solve_quadratic (src/common.rs:254)
  kurbo::cubicbez::{impl#9}::extrema::one_coord (src/cubicbez.rs:749)
  kurbo::cubicbez::{impl#9}::extrema (src/cubicbez.rs:761)
  kurbo::bezpath::{impl#20}::extrema (src/bezpath.rs:1044)
  kurbo::param_curve::ParamCurveExtrema::extrema_ranges&lt;kurbo::bezpath::PathSeg&gt; (src/param_curve.rs:205)
  kurbo::bezpath::PathSeg::winding (src/bezpath.rs:1154)
  kurbo::bezpath::{impl#15}::winding::{closure#0}&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:949)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::winding&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:94... (+2 chars)
  kurbo::arc::{impl#3}::winding (kurbo/src/arc.rs:221)
  kurbo::shape::Shape::contains&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:137)
  repro_timeout_in_kurbo_arc_contains_30c508::main (src/main.rs:6)
  (5 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let circle = Circle::new(Point::ZERO, 1.0f64);
let _ = &lt;kurbo::Line as kurbo::Shape&gt;::to_path(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(
    &amp;&lt;kurbo::Circle as kurbo::Shape&gt;::to_path(&amp;circle, 0.0f64)).unwrap(),
    0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;kurbo::circle::CirclePathIter&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::Circle&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_line_to_path_77b261::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::as_rect(
    &amp;&lt;kurbo::Ellipse as kurbo::Shape&gt;::into_path(Ellipse::default(), 0.0f64));</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_bezpath_as_rect_a10cf2::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis::Horizontal;
let axis2 = axis.cross();
let _ = axis2.cross();
let arc = Arc::new(Point::new(f64::MAX, -0.0f64), Vec2::new(-1.0f64, f64::NAN),
    f64::MAX, 2.5598864519886808e92f64, -1e300f64);
let arc2 = arc.reversed();
let arc3 = &lt;kurbo::Affine as ::core::ops::Mul&lt;kurbo::Arc&gt;&gt;::mul(Affine::scale(
    -1.5391913684270659e263f64), arc2);
let _ = PathSeg::Line(Line::from((&lt;kurbo::Arc as kurbo::Shape&gt;::bounding_box(
    &amp;arc3).origin(), Vec2::splat(66.8f64))));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::cubicbez::{impl#9}::extrema::one_coord (src/cubicbez.rs:746)
  kurbo::cubicbez::{impl#9}::extrema (src/cubicbez.rs:761)
  kurbo::bezpath::{impl#20}::extrema (src/bezpath.rs:1044)
  kurbo::param_curve::ParamCurveExtrema::bounding_box&lt;kurbo::bezpath::PathSeg&gt; (src/param_curve.rs:216)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::bounding_box&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.... (+7 chars)
  kurbo::arc::{impl#3}::bounding_box (kurbo/src/arc.rs:226)
  repro_timeout_in_kurbo_pathseg_line_97c142::main (src/main.rs:16)</pre></td></tr>
<tr><td valign="top"><pre>let circle_segment = CircleSegment { center: Point::ZERO, outer_radius: 0.0f64,
    inner_radius: 1.0f64, start_angle: 0.0f64, sweep_angle: f64::INFINITY };
let _ = kurbo::expand_path(circle_segment.outer_arc(), Diagonal2 { xx: 0.0f64,
    yy: 0.0f64 }, Join::Bevel, 0.0f64, 0.0f64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::{impl#1}::next (src/arc.rs:154)
  kurbo::bezpath::{impl#13}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:837)
  kurbo::bezpath::{impl#14}::next&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt;::area&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::Pat... (+61 chars)
  kurbo::expand::expand_path&lt;kurbo::arc::Arc&gt; (kurbo/src/expand.rs:134)
  repro_timeout_in_kurbo_expand_path_arc_367806::main (src/main.rs:8)
  (9 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis::Vertical;
let _ = axis.pack_size(255.0f64, f64::MAX);
let _ = TranslateScale::translate(Vec2::ZERO);
let stroke_ctx = StrokeCtx::default();
let _ = stroke_ctx.output();
let rounded_rect_radii = RoundedRectRadii { top_left: f64::INFINITY,
    top_right: f64::NEG_INFINITY, bottom_right: 0.9999999999999999f64,
    bottom_left: 0.283f64 };
let _ = rounded_rect_radii.clamp(10000000000.0f64);
let _ = &lt;kurbo::Moments as ::core::ops::Add&lt;kurbo::Moments&gt;&gt;::add(
    &lt;kurbo::Moments as ::core::ops::Add&lt;kurbo::Moments&gt;&gt;::add(
    Moments { moment_x: 0.478f64, moment_y: 0.573f64, moment_xx: -1e300f64,
    moment_xy: 557.0f64, moment_yy: f64::MAX }, Moments::default()),
    &lt;kurbo::Moments as ::core::ops::Add&lt;kurbo::Moments&gt;&gt;::add(
    Moments::default(),
    &lt;kurbo::Moments as ::core::ops::Add&lt;kurbo::Moments&gt;&gt;::add(
    Moments::default(), Moments::default())));
let _ = LineIntersection { line_t: 0.266f64, segment_t: -69.6f64 };
let stroke_ctx2 = StrokeCtx::default();
let _ = stroke_ctx2.output();
let affine = Affine::skew(f64::MIN, 71.8f64);
let affine2 = affine.pre_skew(-1e300f64, -80.9f64);
let _ = affine2.inverse();
let quad_spline = QuadSpline::new(vec![]);
let _ = quad_spline.points();
let _ = &lt;kurbo::RoundedRect as kurbo::Shape&gt;::to_path(&amp;Rect::new(
    -4.9786148735079455e270f64, 9007199254740992.0f64, 1e300f64,
    f64::MIN).inflate(22.5f64, 537.0f64).to_rounded_rect(rounded_rect_radii),
    65.7f64);
let _ = MAX_EXTREMA;</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;kurbo::rounded_rect::RoundedRectPathIter&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::rounded_rect::RoundedRect&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_roundedrect_to_path_428d96::main (src/main.rs:30)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::winding(
    &amp;&lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(
    &amp;CircleSegment { center: &lt;kurbo::Point as ::core::ops::Sub&lt;(f64,
    f64)&gt;&gt;::sub(Point::from((256.0f32, 384.0f32)), (2.2250738585072014e-308f64,
    1.0f64)), outer_radius: 255.0f64, inner_radius: 166.0f64,
    start_angle: -1e300f64, sweep_angle: f64::MAX }, 585.0f64),
    Point::new(f64::NAN, 0.5f64).ceil());</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  repro_timeout_in_kurbo_bezpath_winding_c08346::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let arc = Arc::new(Point::from((0.0f32, 0.0f32)),
    Axis::Vertical.pack_vec2(0.0f64, 0.0f64), 0.0f64, -6.067230110266265e29f64,
    0.0f64);
let _ = &lt;kurbo::Rect as kurbo::Shape&gt;::as_circle(
    &amp;&lt;kurbo::Arc as kurbo::Shape&gt;::bounding_box(&amp;arc));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::sample_ellipse (src/arc.rs:165)
  kurbo::arc::{impl#1}::next (src/arc.rs:148)
  kurbo::bezpath::{impl#14}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::bounding_box&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.... (+7 chars)
  kurbo::arc::{impl#3}::bounding_box (kurbo/src/arc.rs:226)
  repro_timeout_in_kurbo_rect_as_circle_f616fb::main (src/main.rs:7)
  (8 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let insets = &lt;kurbo::Insets as ::core::ops::Mul&lt;f64&gt;&gt;::mul(Insets::uniform_xy(
    0.0f64, 0.0f64), 0.0f64);
let f64_v = insets.x_value();
let _ = &lt;kurbo::Circle as kurbo::Shape&gt;::to_path(
    &amp;Circle { center: Point { x: 1.0f64, y: 0.0f64 }, radius: 1.0f64 }, f64_v);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;kurbo::circle::CirclePathIter&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::Circle&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_circle_to_path_9d678d::main (src/main.rs:8)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::ZERO;
let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::winding(&amp;Arc::new(point, Vec2::ZERO,
    0.0f64, f64::INFINITY, 0.0f64), point);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  arrayvec::arrayvec::{impl#2}::len&lt;f64, 2&gt; (arrayvec/src/arrayvec.rs:749)
  arrayvec::arrayvec_impl::ArrayVecImpl::as_slice&lt;arrayvec::arrayvec::ArrayVec&lt;f64, 2&gt;&gt; (arrayvec/src/arrayvec_impl.rs:18)
  arrayvec::arrayvec::ArrayVec&lt;f64, 2&gt;::as_slice&lt;f64, 2&gt; (arrayvec/src/arrayvec.rs:726)
  arrayvec::arrayvec::{impl#3}::deref&lt;f64, 2&gt; (arrayvec/src/arrayvec.rs:769)
  arrayvec::arrayvec::{impl#7}::into_iter&lt;f64, 2&gt; (arrayvec/src/arrayvec.rs:847)
  kurbo::cubicbez::{impl#9}::extrema::one_coord (src/cubicbez.rs:750)
  kurbo::cubicbez::{impl#9}::extrema (src/cubicbez.rs:760)
  kurbo::bezpath::{impl#20}::extrema (src/bezpath.rs:1044)
  kurbo::param_curve::ParamCurveExtrema::extrema_ranges&lt;kurbo::bezpath::PathSeg&gt; (src/param_curve.rs:205)
  kurbo::bezpath::PathSeg::winding (src/bezpath.rs:1154)
  kurbo::bezpath::{impl#15}::winding::{closure#0}&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:949)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::winding&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:94... (+2 chars)
  kurbo::arc::{impl#3}::winding (kurbo/src/arc.rs:221)
  repro_timeout_in_kurbo_arc_winding_7a58c3::main (src/main.rs:7)
  (5 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Circle as kurbo::Shape&gt;::into_path(Triangle::from_coords((
    1.0f64, 0.0f64), (1.0f64, 1.0f64), (0.0f64, 0.0f64)).circumscribed_circle(),
    0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;kurbo::circle::CirclePathIter&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::Circle&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::circle::Circle&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_circle_into_path_d028e0::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis::Vertical;
let vec2 = axis.pack_vec2(6.193250200388348e-167f64, 0.546f64);
let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::bounding_box(
    &amp;Arc { center: &lt;kurbo::Point as ::core::ops::Add&lt;kurbo::Vec2&gt;&gt;::add(
    Point::default(), vec2), radii: Vec2::ZERO,
    start_angle: -3.099657922243612e195f64, sweep_angle: 9007199254740992.0f64,
    x_rotation: -44.8f64 });
let svg_parse_error = SvgParseError::UnexpectedEof;
let _ = svg_parse_error.to_string();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::sample_ellipse (src/arc.rs:165)
  kurbo::arc::{impl#1}::next (src/arc.rs:146)
  kurbo::bezpath::{impl#14}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::bounding_box&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.... (+7 chars)
  kurbo::arc::{impl#3}::bounding_box (kurbo/src/arc.rs:226)
  repro_timeout_in_kurbo_arc_bounding_box_432137::main (src/main.rs:10)
  (7 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Circle as ::core::ops::Add&lt;kurbo::Vec2&gt;&gt;::add(
    &lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(
    &amp;&lt;kurbo::Ellipse as kurbo::Shape&gt;::to_path(&amp;Ellipse::from_affine(
    Affine::scale_non_uniform(1.0f64, 0.0f64)), 0.0f64)).unwrap(),
    Vec2 { x: 0.0f64, y: 0.0f64 });</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_circle_add_kurbo_vec2_288c52::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit&gt;::
    sample_pt_deriv(&amp;SimplifyBezPath::new(vec![BezPath::with_capacity(
    1000000000usize).pop().unwrap()]), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_simplify_simplifybezpath_sample_pt_de_d1510c::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::expand_path_signed(
    &lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(&amp;CircleSegment::new(
    Point::ORIGIN, 0.0f64, 0.0f64, f64::MAX, f64::MAX),
    0.0f64).get_seg(0usize).unwrap(), Diagonal2 { xx: 0.0f64, yy: 0.0f64 },
    Join::Round, f64::NAN, f64::NEG_INFINITY);</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  repro_timeout_in_kurbo_expand_path_signed_pathseg_e33acf::main (src/main.rs:7)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Line as kurbo::Shape&gt;::as_line(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_line_as_line_437102::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Line as kurbo::ParamCurve&gt;::subdivide(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_line_subdivide_fb6967::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::perimeter(
    &amp;&lt;kurbo::Circle as kurbo::Shape&gt;::to_path(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap(), 0.0f64), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_bezpath_perimeter_103835::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::to_path(&amp;Arc::new(Point::default(),
    Vec2 { x: 0.0f64, y: 65536.0f64 }, 0.0f64, -1e300f64, 1e-310f64), f64::NAN);</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:68)
  repro_timeout_in_kurbo_arc_to_path_140d95::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::from((0.0f32, 55433.484f32));
let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::area(&amp;kurbo::expand_path(
    Arc { center: point, radii: Vec2::splat(f64::NAN),
    start_angle: f64::NEG_INFINITY, sweep_angle: 9007199254740992.0f64,
    x_rotation: 9007199254740992.0f64 },
    &lt;kurbo::Diagonal2 as ::core::ops::Neg&gt;::neg(Diagonal2::new(255.0f64,
    65536.0f64)), Join::Round, 0.796f64, f64::NAN));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::rotate_pt (src/arc.rs:173)
  kurbo::arc::sample_ellipse (src/arc.rs:168)
  kurbo::arc::{impl#1}::next (src/arc.rs:146)
  kurbo::bezpath::{impl#13}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:837)
  kurbo::bezpath::{impl#14}::next&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt;::area&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::Pat... (+61 chars)
  kurbo::expand::expand_path&lt;kurbo::arc::Arc&gt; (kurbo/src/expand.rs:134)
  repro_timeout_in_kurbo_bezpath_area_2d9188::main (src/main.rs:9)
  (12 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis::Vertical;
let point = axis.pack_point(0.0f64, 0.0f64);
let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::into_path(CircleSegment { center: point,
    outer_radius: 0.0f64, inner_radius: 0.0f64, start_angle: 0.0f64,
    sweep_angle: f64::MAX }.inner_arc(), 0.0f64);</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:83)
  repro_timeout_in_kurbo_arc_into_path_8e1e73::main (src/main.rs:7)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::TranslateScale as ::core::ops::Mul&lt;kurbo::BezPath&gt;&gt;::mul(
    TranslateScale::from_scale_about(0.0f64, Point::from((0.0f32, 0.0f32))),
    &lt;kurbo::CircleSegment as kurbo::Shape&gt;::into_path(CircleSegment::new(
    Point::ORIGIN, 0.0f64, 1.0f64, 0.0f64, 1.0f64), 0.0f64));</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_translatescale_mul_kurbo_bezpath_e80e8e::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::RoundedRect as kurbo::Shape&gt;::as_line(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_rounded_rect(
    &amp;&lt;kurbo::Ellipse as kurbo::Shape&gt;::into_path(Ellipse::from_affine(
    Affine::IDENTITY), 0.0f64)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_roundedrect_as_line_8183a2::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Stroke::default();
let arc = &lt;kurbo::Arc as kurbo::ParamCurve&gt;::subsegment(
    &amp;Arc { center: Point::new(49.0f64, 0.545f64), radii: Vec2::new(0.387f64,
    f64::INFINITY), start_angle: f64::INFINITY, sweep_angle: 195.0f64,
    x_rotation: 9007199254740992.0f64 }.reversed(), -0.0f64..255.0f64);
let _ = Triangle::EQUILATERAL;
let _ = &lt;kurbo::Rect as kurbo::Shape&gt;::as_rounded_rect(
    &amp;&lt;kurbo::Rect as kurbo::Shape&gt;::bounding_box(
    &amp;&lt;kurbo::Arc as kurbo::Shape&gt;::bounding_box(&amp;arc).scale_from_origin(
    1e300f64)));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::rotate_pt (src/arc.rs:173)
  kurbo::arc::sample_ellipse (src/arc.rs:168)
  kurbo::arc::{impl#1}::next (src/arc.rs:146)
  kurbo::bezpath::{impl#14}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::bounding_box&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.... (+7 chars)
  kurbo::arc::{impl#3}::bounding_box (kurbo/src/arc.rs:226)
  repro_timeout_in_kurbo_rect_as_rounded_rect_9e6be9::main (src/main.rs:11)
  (8 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let circle = &lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(
    &amp;&lt;kurbo::CircleSegment as kurbo::Shape&gt;::into_path(Circle::new(Point::from((
    0.841f32, f32::MIN)), 0.123f64).segment(10000000000.0f64,
    1.7915043128019068e151f64, 0.236f64), 2.2250738585072014e-308f64)).unwrap();
let _ = MinDistance { distance: 0.68f64, t1: 1e300f64,
    t2: -1.6184141932547296e-256f64 };
let _ = Size::INFINITY;
let _ = &lt;kurbo::Circle as kurbo::Shape&gt;::as_line(&amp;circle);
let _ = &lt;kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit&gt;::
    sample_pt_deriv(&amp;SimplifyBezPath::new(vec![PathEl::ClosePath]),
    2.2250738585072014e-308f64);</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:83)
  repro_timeout_in_kurbo_bezpath_as_circle_e5a99d::main (src/main.rs:9)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::new(0.0f64, 0.0f64);
let vec2 = point.to_vec2();
let _ = &lt;kurbo::Rect as kurbo::Shape&gt;::as_path_slice(
    &amp;&lt;kurbo::Arc as kurbo::Shape&gt;::bounding_box(&amp;Arc::new(Point::new(1.0f64,
    1.0f64), Vec2::splat(f64::INFINITY).lerp(vec2, f64::MIN), 0.5f64, f64::MAX,
    0.216f64)));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::rotate_pt (src/arc.rs:173)
  kurbo::arc::sample_ellipse (src/arc.rs:168)
  kurbo::arc::{impl#1}::next (src/arc.rs:145)
  kurbo::bezpath::{impl#14}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::bounding_box&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.... (+7 chars)
  kurbo::arc::{impl#3}::bounding_box (kurbo/src/arc.rs:226)
  repro_timeout_in_kurbo_rect_as_path_slice_41d505::main (src/main.rs:8)
  (8 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let ellipse = Ellipse::default();
let f64_v = ellipse.rotation();
let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::winding(
    &amp;&lt;kurbo::Ellipse as kurbo::Shape&gt;::into_path(ellipse, 1.0f64),
    &lt;kurbo::Arc as kurbo::ParamCurve&gt;::eval(
    &amp;CircleSegment { center: Point::from((0.0f32, 0.0f32)),
    outer_radius: 0.0f64, inner_radius: f64::INFINITY, start_angle: f64_v,
    sweep_angle: 0.0f64 }.inner_arc(), 0.0f64));</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/cubicbez.rs:470:9:
assertion failed: start.y.min(end.y) &lt;= y &amp;&amp; y &lt;= start.y.max(end.y)
backtrace:
  &lt;kurbo::cubicbez::CubicBez&gt;::solve_monotonic_for_y (kurbo/src/cubicbez.rs:470:9)
  &lt;kurbo::bezpath::PathSeg&gt;::winding_inner (kurbo/src/bezpath.rs:1143:31)
  &lt;kurbo::bezpath::PathSeg&gt;::winding::{closure#0} (kurbo/src/bezpath.rs:1156:49)
  &lt;arrayvec::arrayvec::IntoIter&lt;core::ops::range::Range&lt;f64&gt;, 5&gt; as core::iter::traits::iterator::Iterator&gt;::fold::&lt;i32, core::iter::adapters::map::map_fold&lt;core::ops::range::Range&lt;f64&gt;, i32, i32, &lt;kurbo::bezpath::PathSeg&gt;::winding::{closure#0}, &lt;i32 as core::iter::traits::accum::Sum&gt;::sum&lt;core::ite... (+16 chars)
  &lt;i32 as core::iter::traits::accum::Sum&gt;::sum::&lt;core::iter::adapters::map::Map&lt;arrayvec::arrayvec::IntoIter&lt;core::ops::range::Range&lt;f64&gt;, 5&gt;, &lt;kurbo::bezpath::PathSeg&gt;::winding::{closure#0}&gt;&gt; (core/src/iter/traits/accum.rs:52:22)
  &lt;kurbo::bezpath::PathSeg&gt;::winding (kurbo/src/bezpath.rs:1157:14)
  &lt;kurbo::bezpath::Segments&lt;core::iter::adapters::copied::Copied&lt;core::slice::iter::Iter&lt;kurbo::bezpath::PathEl&gt;&gt;&gt;&gt;::winding::{closure#0} (kurbo/src/bezpath.rs:949:28)
  &lt;kurbo::bezpath::Segments&lt;core::iter::adapters::copied::Copied&lt;core::slice::iter::Iter&lt;kurbo::bezpath::PathEl&gt;&gt;&gt; as core::iter::traits::iterator::Iterator&gt;::fold::&lt;i32, core::iter::adapters::map::map_fold&lt;kurbo::bezpath::PathSeg, i32, i32, &lt;kurbo::bezpath::Segments&lt;core::iter::adapters::copied::Co... (+16 chars)
  &lt;i32 as core::iter::traits::accum::Sum&gt;::sum::&lt;core::iter::adapters::map::Map&lt;kurbo::bezpath::Segments&lt;core::iter::adapters::copied::Copied&lt;core::slice::iter::Iter&lt;kurbo::bezpath::PathEl&gt;&gt;&gt;, &lt;kurbo::bezpath::Segments&lt;core::iter::adapters::copied::Copied&lt;core::slice::iter::Iter&lt;kurbo::bezpath::Path... (+15 chars)
  &lt;kurbo::bezpath::Segments&lt;core::iter::adapters::copied::Copied&lt;core::slice::iter::Iter&lt;kurbo::bezpath::PathEl&gt;&gt;&gt;&gt;::winding (kurbo/src/bezpath.rs:949:40)
  &lt;&amp;[kurbo::bezpath::PathEl] as kurbo::shape::Shape&gt;::winding (kurbo/src/bezpath.rs:1525:40)
  &lt;kurbo::bezpath::BezPath as kurbo::shape::Shape&gt;::winding (kurbo/src/bezpath.rs:1445:25)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::to_path(
    &amp;Arc { center: Axis::Horizontal.pack_point(0.0f64, 0.0f64),
    radii: Vec2::default(), start_angle: 0.0f64, sweep_angle: 1e300f64,
    x_rotation: 0.0f64 }, 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_arc_to_path_66d370::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let path_el = PathEl::CurveTo(Point::from((0.0f32, 957.0f32)),
    &lt;kurbo::QuadBez as kurbo::ParamCurve&gt;::start(&amp;QuadBez { p0: Point::ORIGIN,
    p1: Point::ZERO, p2: Ellipse::default().center() }),
    &lt;kurbo::TranslateScale as ::core::ops::Mul&lt;kurbo::Point&gt;&gt;::mul(
    TranslateScale::new(Vec2 { x: 0.0f64, y: 0.0f64 }, 255.0f64),
    Point::from((256.0f32, 1.1920929e-7f32)).trunc()));
let simplify_bez_path = SimplifyBezPath::new(vec![path_el, path_el, path_el,
    path_el]);
let _ = CircleSegment::new(kurbo::fit_to_bezpath_opt(&amp;simplify_bez_path,
    2.220446049250313e-16f64).current_position().unwrap(), 1e-310f64, 0.5f64,
    9007199254740992.0f64, -0.0f64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::quadbez::QuadBez::new&lt;kurbo::point::Point&gt; (src/quadbez.rs:39)
  kurbo::cubicbez::{impl#4}::deriv (src/cubicbez.rs:608)
  kurbo::simplify::{impl#2}::sample_pt_tangent (src/simplify.rs:210)
  kurbo::fit::fit_to_cubic&lt;kurbo::simplify::SimplifyBezPath&gt; (src/fit.rs:393)
  kurbo::fit::measure_one_seg&lt;kurbo::simplify::SimplifyBezPath&gt; (src/fit.rs:661)
  kurbo::fit::fit_opt_segment::{closure#0}&lt;kurbo::simplify::SimplifyBezPath&gt; (src/fit.rs:687)
  kurbo::common::solve_itp_fallible&lt;f64, kurbo::fit::fit_opt_segment::{closure_env#0}&lt;kurbo::simplify::SimplifyBezPath&gt;&gt; (src/common.rs:733)
  kurbo::fit::fit_opt_segment&lt;kurbo::simplify::SimplifyBezPath&gt; (src/fit.rs:692)
  kurbo::fit::fit_to_bezpath_opt_inner&lt;kurbo::simplify::SimplifyBezPath&gt; (src/fit.rs:612)
  kurbo::fit::fit_to_bezpath_opt&lt;kurbo::simplify::SimplifyBezPath&gt; (src/fit.rs:570)
  repro_timeout_in_kurbo_circlesegment_new_5bc9e8::main (src/main.rs:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::expand_path_signed(Triangle::EQUILATERAL.circumscribed_circle(),
    &lt;kurbo::Diagonal2 as ::core::ops::Neg&gt;::neg(Diagonal2 { xx: 1.0f64,
    yy: -1.2817015970884436e126f64 }), Join::Round, 0.0f64, 1.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::BezPath::push (src/bezpath.rs:222)
  kurbo::bezpath::BezPath::curve_to&lt;kurbo::point::Point&gt; (src/bezpath.rs:267)
  kurbo::expand::{impl#4}::do_join::{closure#0} (src/expand.rs:338)
  kurbo::arc::Arc::to_cubic_beziers&lt;kurbo::expand::{impl#4}::do_join::{closure_env#0}&gt; (src/arc.rs:114)
  kurbo::expand::ExpandCtx::do_join (src/expand.rs:337)
  kurbo::expand::ExpandCtx::do_line (src/expand.rs:198)
  kurbo::expand::ExpandCtx::do_cubic (src/expand.rs:279)
  kurbo::expand::expand_path_signed&lt;kurbo::circle::Circle&gt; (kurbo/src/expand.rs:176)
  repro_out_of_memory_in_kurbo_expand_path_signed_circle_94a80d::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let triangle = Triangle::EQUILATERAL;
let circle = triangle.inscribed_circle();
let _ = &lt;kurbo::PathSeg as kurbo::Shape&gt;::as_rounded_rect(
    &amp;&lt;kurbo::Arc as kurbo::Shape&gt;::to_path(&amp;circle.segment(0.0f64, 571.0f64,
    10000000000.0f64).inner_arc(),
    -2.6531947294924042e60f64).get_seg(0usize).unwrap());</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:68)
  repro_timeout_in_kurbo_pathseg_as_rounded_rect_e25acd::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::PathSeg as kurbo::Shape&gt;::to_path(&amp;BezPath::with_capacity(
    1000000000usize).get_seg(0usize).unwrap(), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_pathseg_to_path_c5d50b::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::ConstPoint as kurbo::ParamCurve&gt;::subdivide(
    &amp;&lt;kurbo::Line as kurbo::ParamCurveDeriv&gt;::deriv(
    &amp;&lt;kurbo::Rect as kurbo::Shape&gt;::as_line(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_rect(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap()).unwrap()));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_constpoint_subdivide_342dd6::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = StrokeOptLevel::Subdivide;
let stroke = Stroke::default();
let stroke2 = stroke.clone().with_caps(Cap::Butt);
let stroke3 = stroke2.clone().with_join(Join::Miter);
let _ = stroke3.clone().with_caps(Cap::Square);
let _ = MinDistance { distance: 7.506104286755182e39f64,
    t1: -2.1348825792426762e278f64, t2: 1.199469914557974e245f64 };
let axis = Axis::Horizontal;
let axis2 = axis.cross();
let axis3 = axis2.cross();
let point = axis3.pack_point(f64::MIN, -51.8f64);
let _ = &lt;kurbo::QuadBez as kurbo::Shape&gt;::as_rounded_rect(
    &amp;&lt;kurbo::TranslateScale as ::core::ops::Mul&lt;kurbo::QuadBez&gt;&gt;::mul(
    TranslateScale::scale(f64::NAN), QuadBez::new::&lt;kurbo::Point&gt;(point, point,
    Point::default())));
let _ = SimplifyOptions::default();
let _ = Ellipse::from_rect(&lt;kurbo::Arc as kurbo::Shape&gt;::bounding_box(
    &amp;CircleSegment::new(point, 237.0f64, 57.4f64, -1e300f64,
    1e300f64).outer_arc()));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::sample_ellipse (src/arc.rs:165)
  kurbo::arc::{impl#1}::next (src/arc.rs:145)
  kurbo::bezpath::{impl#14}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::bounding_box&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.... (+7 chars)
  kurbo::arc::{impl#3}::bounding_box (kurbo/src/arc.rs:226)
  repro_timeout_in_kurbo_ellipse_from_rect_ddd458::main (src/main.rs:27)
  (8 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis::Horizontal;
let point = axis.pack_point(0.0f64, 0.0f64);
let circle_segment = CircleSegment::new(point, 0.0f64, 0.0f64, 0.0f64,
    1.3270698809889517e78f64);
let arc = circle_segment.inner_arc();
let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::to_path(
    &amp;&lt;kurbo::Arc as kurbo::Shape&gt;::into_path(arc, 0.0f64), 0.0f64);</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:83)
  repro_timeout_in_kurbo_bezpath_to_path_3f965c::main (src/main.rs:9)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let circle = Circle::default();
let _ = &lt;kurbo::PathSeg as kurbo::Shape&gt;::as_rounded_rect(
    &amp;&lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(&amp;circle.segment(0.0f64,
    0.0f64, f64::INFINITY), 0.0f64).get_seg(0usize).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_pathseg_as_rounded_rect_51e5b1::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::QuadBez as kurbo::ParamCurveExtrema&gt;::extrema(
    &amp;QuadBez { p0: Point::from((0.0f32, 0.0f32)), p1: Point::ORIGIN,
    p2: BezPath::with_capacity(1000000000usize).current_position().unwrap() });</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_quadbez_extrema_51c277::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::PathSeg as kurbo::ParamCurveArea&gt;::signed_area(
    &amp;BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_pathseg_signed_area_ae2fea::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let cubic_bez = CubicBez { p0: Point::from((0.0f32, 0.0f32)),
    p1: Point::default(),
    p2: PathEl::MoveTo(Point::default()).end_point().unwrap(),
    p3: Point::default() };
let _ = kurbo::simplify::simplify_bezpath(vec![PathSeg::from(
    cubic_bez).as_path_el()], 0.0f64, &amp;SimplifyOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/simplify.rs:329:36:
called `Option::unwrap()` on a `None` value
backtrace:
  kurbo::simplify::simplify_bezpath::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt; (kurbo/src/simplify.rs:329:36)
  repro_kurbo_src_simplify_rs_329_36_5a9b24::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let diagonal2 = Diagonal2::new(0.0f64, 0.0f64);
let vec2 = diagonal2.scale_normal(Vec2::new(0.0f64, 0.0f64));
let point = &lt;kurbo::Arc as kurbo::ParamCurve&gt;::end(&amp;Arc { center: Point::ORIGIN,
    radii: vec2, start_angle: 0.0f64, sweep_angle: 0.0f64,
    x_rotation: 0.0f64 });
let path_el = PathEl::QuadTo(point.expand(), point);
let _ = kurbo::simplify::simplify_bezpath(vec![path_el], 0.0f64,
    &amp;SimplifyOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/simplify.rs:322:36:
called `Option::unwrap()` on a `None` value
backtrace:
  kurbo::simplify::simplify_bezpath::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt; (kurbo/src/simplify.rs:322:36)
  repro_kurbo_src_simplify_rs_322_36_2ad12e::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Line::new(Point::from((0.0f32, 0.0f32)),
    BezPath::with_capacity(100000000usize).current_position().unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 5600000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_line_new_a484d1::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::from((0.0f64, 0.0f64));
let f64_v = &lt;kurbo::Line as kurbo::ParamCurveArclen&gt;::arclen(&amp;Line::from((
    Point { x: 0.0f64, y: 0.0f64 }, Vec2 { x: 0.0f64, y: 0.0f64 })), 0.0f64);
let _ = kurbo::fit_to_bezpath_opt(&amp;SimplifyBezPath::new(vec![PathEl::QuadTo(
    point, Point::new(f64_v, 1.0f64))]), 0.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/fit.rs:646:61:
called `Option::unwrap()` on a `None` value
backtrace:
  kurbo::fit::fit_to_bezpath_opt_inner::&lt;kurbo::simplify::SimplifyBezPath&gt; (kurbo/src/fit.rs:646:61)
  kurbo::fit::fit_to_bezpath_opt::&lt;kurbo::simplify::SimplifyBezPath&gt; (kurbo/src/fit.rs:570:15)
  repro_kurbo_src_fit_rs_646_61_75818e::main (src/main.rs:10:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::expand_path_signed(&lt;kurbo::Ellipse as kurbo::Shape&gt;::into_path(
    Ellipse::default(), 0.0f64), Diagonal2 { xx: 0.0f64, yy: 0.0f64 },
    Join::Round, 0.0f64, 1.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_expand_path_signed_bezpath_128e23::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Rect as kurbo::Shape&gt;::perimeter(
    &amp;&lt;kurbo::Arc as kurbo::Shape&gt;::bounding_box(&amp;Arc::new(Point { x: 0.0f64,
    y: 0.0f64 }, Vec2 { x: f64::INFINITY, y: 0.0f64 }, 0.0f64, 1.0f64, 0.0f64)),
    1.0f64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::bezpath::{impl#14}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:907)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::bounding_box&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.... (+7 chars)
  kurbo::arc::{impl#3}::bounding_box (kurbo/src/arc.rs:226)
  repro_timeout_in_kurbo_rect_perimeter_436d7f::main (src/main.rs:6)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let point = &lt;kurbo::Point as ::core::ops::Sub&lt;kurbo::Vec2&gt;&gt;::sub(Point::ZERO,
    Vec2::from_angle(9007199254740992.0f64).turn_90());
let translate_scale = TranslateScale::default();
let join = Join::Round;
let translate_scale2 = translate_scale.inverse();
let vec2 = &lt;kurbo::Point as ::core::ops::Sub&lt;kurbo::Point&gt;&gt;::sub(
    &lt;kurbo::Vec2 as ::core::ops::Mul&lt;f64&gt;&gt;::mul(Vec2::new(0.9999999999999999f64,
    -0.0f64).round(), 358.0f64).to_point(), point);
let _ = &lt;kurbo::TranslateScale as ::core::ops::Mul&lt;kurbo::BezPath&gt;&gt;::mul(
    translate_scale2, kurbo::expand_path(Arc { center: point, radii: vec2,
    start_angle: f64::NAN, sweep_angle: f64::INFINITY,
    x_rotation: 2.220446049250313e-16f64 }, Diagonal2 { xx: 1e-310f64,
    yy: 255.0f64 }, join, 62.0f64, f64::MAX));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::bezpath::{impl#14}::next&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt;::area&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::Pat... (+61 chars)
  kurbo::expand::expand_path&lt;kurbo::arc::Arc&gt; (kurbo/src/expand.rs:134)
  repro_timeout_in_kurbo_translatescale_mul_kurbo_bezpath_dae6c3::main (src/main.rs:14)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::CubicBez as kurbo::Shape&gt;::to_path(&amp;BezPath::with_capacity(
    100000000000usize).get_seg(0usize).unwrap().to_cubic(), 1.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 5600000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_cubicbez_to_path_56159d::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::expand_path(BezPath::with_capacity(1000000000usize),
    Diagonal2::new(0.0f64, 0.0f64), Join::Miter, 1.0f64, 1.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_expand_path_bezpath_3979f5::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = CircleSegment::new(BezPath::with_capacity(
    100000000000usize).current_position().unwrap(), 0.0f64, 0.0f64, 0.0f64,
    0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 5600000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_circlesegment_new_31d5bf::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let f64_v = &lt;kurbo::Circle as kurbo::Shape&gt;::area(&amp;Circle::new(Point::from((
    0.0f32, 0.0f32)), 0.0f64));
let axis = Axis::Horizontal;
let axis2 = axis.cross();
let axis3 = axis2.cross();
let point = axis3.pack_point(f64_v, 0.0f64);
let _ = &lt;kurbo::CubicBez as kurbo::ParamCurve&gt;::subdivide(&amp;CubicBez { p0: point,
    p1: &lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap().midpoint(), p2: point, p3: point });</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_cubicbez_subdivide_2c7858::main (src/main.rs:13)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = CircleSegment { center: &lt;kurbo::PathSeg as kurbo::ParamCurve&gt;::eval(
    &amp;BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap(), 0.0f64),
    outer_radius: 0.0f64, inner_radius: 0.0f64, start_angle: 0.0f64,
    sweep_angle: 0.0f64 };</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_circlesegment_literal_37de29::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::CubicBez as kurbo::ParamCurveArclen&gt;::inv_arclen(
    &amp;BezPath::with_capacity(1000000000usize).get_seg(
    0usize).unwrap().to_cubic(), 0.0f64, 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_cubicbez_inv_arclen_db9ea3::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis::Vertical;
let point = axis.pack_point(0.0f64, 0.0f64);
let _ = &lt;kurbo::PathSeg as kurbo::ParamCurveNearest&gt;::nearest(
    &amp;&lt;kurbo::CircleSegment as kurbo::Shape&gt;::into_path(CircleSegment::new(point,
    0.0f64, 0.0f64, 0.0f64, f64::INFINITY), 0.0f64).get_seg(0usize).unwrap(),
    point, 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_pathseg_nearest_9f30f5::main (src/main.rs:7)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::PathSeg as kurbo::ParamCurveExtrema&gt;::extrema_ranges(
    &amp;BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_pathseg_extrema_ranges_e0f6dc::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Affine as ::core::ops::Mul&lt;kurbo::Circle&gt;&gt;::mul(Affine::new(
    [0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64]),
    &lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_affine_mul_kurbo_circle_54c982::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let vec2 = &lt;kurbo::Vec2 as ::core::ops::Div&lt;f64&gt;&gt;::div(Point::new(1.0f64,
    0.0f64).to_vec2(), 1.0f64);
let vec22 = vec2.round();
let point = Point::new(0.0f64, 0.0f64);
let f64_v = point.distance(Point::from((0.0f64, 0.0f64)));
let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::to_path(
    &amp;&lt;kurbo::Ellipse as kurbo::Shape&gt;::to_path(&amp;Ellipse::new(point, vec22,
    f64_v), -0.0f64), f64_v);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_bezpath_to_path_9438c0::main (src/main.rs:9)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::PathSeg as kurbo::Shape&gt;::path_segments(&amp;BezPath::with_capacity(
    1000000000usize).get_seg(0usize).unwrap(), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_pathseg_path_segments_d3347b::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Affine as ::core::ops::Mul&lt;kurbo::BezPath&gt;&gt;::mul(
    Affine::translate::&lt;kurbo::Vec2&gt;(Vec2::ZERO),
    BezPath::with_capacity(1000000000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_affine_mul_kurbo_bezpath_d880ef::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::as_path_slice(
    &amp;kurbo::expand_path_signed(Arc { center: Point::from((0.0f64, 0.0f64)),
    radii: Vec2 { x: 0.0f64, y: 0.0f64 }, start_angle: 0.0f64,
    sweep_angle: -1.1805916207174112e21f64, x_rotation: 0.0f64 },
    Diagonal2::new(0.0f64, 0.0f64), Join::Miter, 0.0f64, 0.0f64));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::{impl#1}::next (src/arc.rs:154)
  kurbo::expand::expand_path_signed&lt;kurbo::arc::Arc&gt; (kurbo/src/expand.rs:167)
  repro_timeout_in_kurbo_bezpath_as_path_slice_8f1571::main (src/main.rs:8)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::TranslateScale as ::core::ops::Mul&lt;kurbo::PathSeg&gt;&gt;::mul(
    TranslateScale::default(),
    &lt;kurbo::RoundedRect as kurbo::Shape&gt;::into_path(RoundedRect::from_points(
    Point { x: 0.5f64, y: 9007199254740992.0f64 },
    Point::new(3.494079430643412e-158f64, f64::NEG_INFINITY),
    RoundedRectRadii::from_single_radius(2e-323f64)),
    0.0f64).get_seg(256usize).unwrap());</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;kurbo::rounded_rect::RoundedRectPathIter&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::rounded_rect::RoundedRect&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::rounded_rect::RoundedRect&gt; (kurbo/src/shape.rs:83)
  repro_timeout_in_kurbo_translatescale_mul_kurbo_pathseg_035347::main (src/main.rs:7)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::TranslateScale as ::core::ops::Mul&lt;kurbo::PathEl&gt;&gt;::mul(
    TranslateScale::scale(0.0f64),
    BezPath::with_capacity(10000000000000000usize).pop().unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 560000000000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_translatescale_mul_kurbo_pathel_7e79dd::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(&amp;CircleSegment::new(
    Point::ORIGIN, 0.0f64, 1.0f64, 9007199254740992.0f64, 1.0f64), 0.0f64);</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  repro_timeout_in_kurbo_circlesegment_to_path_bc6abd::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Affine as ::core::ops::Mul&lt;kurbo::CubicBez&gt;&gt;::mul(
    Affine::reflect(Vec2::from_angle(0.0f64).to_point(), Vec2::default()),
    BezPath::with_capacity(1000000000usize).get_seg(
    0usize).unwrap().to_cubic());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_affine_mul_kurbo_cubicbez_1904d6::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let diagonal2 = Diagonal2::new(0.0f64, 0.0f64);
let diagonal22 = diagonal2.abs();
let axis = Axis::Vertical;
let point = axis.pack_point(f64::NEG_INFINITY, 0.0f64);
let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::as_circle(
    &amp;Arc { center: kurbo::expand_path_signed(Circle::new(point, -1e300f64),
    &lt;kurbo::Diagonal2 as ::core::ops::Neg&gt;::neg(diagonal22), Join::Miter,
    571.0f64, 65536.0f64).current_position().unwrap(),
    radii: &lt;kurbo::Diagonal2 as ::core::ops::Mul&lt;kurbo::Vec2&gt;&gt;::mul(diagonal2,
    Vec2 { x: 0.5f64, y: 255.0f64 }), start_angle: f64::INFINITY,
    sweep_angle: 0.0f64, x_rotation: 1e300f64 });</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::circle::{impl#5}::next (src/circle.rs:177)
  kurbo::expand::expand_path_signed&lt;kurbo::circle::Circle&gt; (kurbo/src/expand.rs:167)
  repro_timeout_in_kurbo_arc_as_circle_5de2db::main (src/main.rs:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::perimeter(
    &amp;&lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(&amp;CircleSegment::new(
    Triangle::EQUILATERAL.centroid(), 0.0f64, 0.0f64, 25800000000.0f64,
    10000000000.0f64), 75.50000000000001f64), 65536.0f64);</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  repro_timeout_in_kurbo_bezpath_perimeter_a12416::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Line as kurbo::ParamCurve&gt;::start(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_line_start_92d096::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let vec2 = &lt;kurbo::Vec2 as ::core::ops::Neg&gt;::neg(Vec2::default());
let vec22 = vec2.round();
let _ = &lt;kurbo::TranslateScale as ::core::ops::Mul&lt;kurbo::Circle&gt;&gt;::mul(
    TranslateScale::translate(vec22),
    &lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_translatescale_mul_kurbo_circle_700452::main (src/main.rs:8)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let join = Join::Miter;
let affine = Affine::new([0.0f64, 1.0f64, 0.0f64, 1.0f64, 0.0f64, 0.0f64]);
let f64_v = affine.frobenius_norm_squared();
let _ = kurbo::expand_path_signed(Arc::new(Point::from((1.0f64, 0.0f64)),
    Vec2::new(f64::NEG_INFINITY, f64::INFINITY), 0.0f64, 1.0f64, -1e300f64),
    Diagonal2 { xx: -0.0f64, yy: -4.60134496568648e87f64 }.inv(), join, f64_v,
    0.9999999999999999f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::BezPath::push (src/bezpath.rs:222)
  kurbo::bezpath::BezPath::line_to&lt;kurbo::point::Point&gt; (src/bezpath.rs:243)
  kurbo::expand::ExpandCtx::do_join (src/expand.rs:346)
  kurbo::expand::ExpandCtx::do_line (src/expand.rs:198)
  kurbo::expand::ExpandCtx::do_cubic (src/expand.rs:279)
  kurbo::expand::expand_path_signed&lt;kurbo::arc::Arc&gt; (kurbo/src/expand.rs:176)
  repro_out_of_memory_in_kurbo_expand_path_signed_arc_ec1944::main (src/main.rs:12)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis::Vertical;
let point = axis.pack_point(0.0f64, 2.2250738585072014e-308f64);
let line_intersection = LineIntersection { line_t: 0.0f64, segment_t: 0.0f64 };
let bool_v = line_intersection.is_nan();
let _ = Arc::from_svg_arc(&amp;SvgArc { from: RoundedRect::default().origin(),
    to: point, radii: Vec2::from_angle(1.0f64), x_rotation: 0.0f64,
    large_arc: bool_v, sweep: false });</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/svg.rs:463:9:
assertion failed: sum_of_sq != 0.0
backtrace:
  &lt;kurbo::arc::Arc&gt;::from_svg_arc (kurbo/src/svg.rs:463:9)
  repro_kurbo_src_svg_rs_463_9_2a08ec::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Vec2 as ::core::ops::Neg&gt;::neg(
    &lt;kurbo::Point as ::core::ops::Sub&lt;kurbo::Point&gt;&gt;::sub(
    BezPath::with_capacity(100000000000usize).current_position().unwrap(),
    Point::ZERO));</pre></td><td valign="top"><pre>out of memory: memory allocation of 5600000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_vec2_neg_c5f5b4::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Circle as kurbo::Shape&gt;::as_path_slice(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_circle_as_path_slice_79a420::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::area(&amp;BezPath::with_capacity(
    1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_bezpath_area_a97d80::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::ConstPoint as kurbo::ParamCurveDeriv&gt;::deriv(
    &amp;&lt;kurbo::Line as kurbo::ParamCurveDeriv&gt;::deriv(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap()));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_constpoint_deriv_0537b9::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::into_path(kurbo::expand_path(Arc::new(
    Point::ZERO, Vec2::from_angle(1.0f64), 0.0f64, 1.0f64, 0.0f64),
    Diagonal2::new(0.0f64, 0.0f64), Join::Bevel, 0.0f64, 0.0f64), 0.0f64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::bezpath::{impl#14}::next&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt; (kurbo/src/bezpath.rs:918)
  kurbo::bezpath::Segments&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt;::area&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::Pat... (+61 chars)
  kurbo::expand::expand_path&lt;kurbo::arc::Arc&gt; (kurbo/src/expand.rs:134)
  repro_timeout_in_kurbo_bezpath_into_path_632bc4::main (src/main.rs:8)
  (5 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let size = Size::default();
let size2 = &lt;kurbo::Size as ::core::ops::Add&lt;kurbo::Size&gt;&gt;::add(size,
    &lt;kurbo::Size as ::core::ops::Mul&lt;f64&gt;&gt;::mul(Size::from((
    0.9999999999999999f64, -34.6f64)).expand(), -3.67e19f64).floor());
let rounded_rect = size2.to_rounded_rect(RoundedRectRadii { top_left: 1e-310f64,
    top_right: 1.0f64, bottom_right: 0.5f64,
    bottom_left: 0.0f64 }.clamp(853.0f64).abs());
let _ = MinDistance { distance: 65536.0f64, t1: 0.106f64,
    t2: 0.9999999999999999f64 };
let simplify_opt_level = SimplifyOptLevel::Optimize;
let _ = &lt;kurbo::PathSeg as kurbo::Shape&gt;::area(
    &amp;&lt;kurbo::RoundedRect as kurbo::Shape&gt;::to_path(&amp;rounded_rect,
    0.0f64).get_seg(11usize).unwrap());
let simplify_options = SimplifyOptions::default();
let simplify_options2 = simplify_options.opt_level(simplify_opt_level);
let _ = simplify_options2.angle_thresh(0.25000000000000006f64);</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;kurbo::rounded_rect::RoundedRectPathIter&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::rounded_rect::RoundedRect&gt; (kurbo/src/shape.rs:68)
  repro_timeout_in_kurbo_pathseg_area_3a8849::main (src/main.rs:13)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let triangle = Triangle::from_coords((-0.9000000000000004f64, 1e300f64),
    (1e-310f64, 43.1f64), (-6.68e-31f64, 9007199254740992.0f64));
let circle = triangle.inscribed_circle();
let circle_segment = circle.segment(f64::MAX, 7.450580596923553e-9f64,
    810.0f64);
let arc = circle_segment.inner_arc();
let _ = &lt;kurbo::Line as kurbo::Shape&gt;::contains(&amp;Line::new(Point::ORIGIN,
    (&lt;kurbo::Arc as kurbo::Shape&gt;::bounding_box(
    &amp;&lt;kurbo::Arc as kurbo::ParamCurve&gt;::subsegment(&amp;arc,
    1e300f64..1e-310f64))).center()),
    &lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(&amp;circle_segment,
    -4.87e-19f64).current_position().unwrap());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::sample_ellipse (src/arc.rs:165)
  kurbo::arc::{impl#1}::next (src/arc.rs:148)
  kurbo::bezpath::{impl#14}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;::bounding_box&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.... (+7 chars)
  kurbo::arc::{impl#3}::bounding_box (kurbo/src/arc.rs:226)
  repro_timeout_in_kurbo_line_contains_42825d::main (src/main.rs:10)
  (8 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Rect as kurbo::Shape&gt;::as_path_slice(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::bounding_box(&amp;BezPath::with_capacity(
    1000000000usize)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_rect_as_path_slice_30d2d7::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::from((0.0f64, 0.0f64));
let circle = Circle::new(point, 1.0f64);
let _ = SimplifyBezPath::new(vec![kurbo::expand_path(Circle::from(circle),
    Diagonal2 { xx: 4398046511103.9995f64, yy: 0.5f64 }, Join::Miter,
    -5.9460652595886345e-151f64, 1e-323f64).pop().unwrap(),
    PathEl::CurveTo(point, point, point),
    &lt;kurbo::Affine as ::core::ops::Mul&lt;kurbo::PathEl&gt;&gt;::mul(Affine::reflect(
    point, Vec2::from((1e300f64, 1e-310f64))), PathEl::ClosePath),
    PathEl::MoveTo(Point::from((-0.0f64, 1.5482317065146463e31f64)))]);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::bezpath::{impl#14}::next&lt;kurbo::bezpath::CloseSubpaths&lt;kurbo::circle::CirclePathIter&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;kurbo::bezpath::CloseSubpaths&lt;kurbo::circle::CirclePathIter&gt;&gt;::area&lt;kurbo::bezpath::CloseSubpaths&lt;kurbo::circle::CirclePathIter&gt;&gt; (kurbo/src/bezpath.rs:944)
  kurbo::expand::expand_path&lt;kurbo::circle::Circle&gt; (kurbo/src/expand.rs:134)
  repro_timeout_in_kurbo_simplify_simplifybezpath_new_874b01::main (src/main.rs:13)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let point = Point::from(Point::ORIGIN);
let _ = &lt;kurbo::Line as kurbo::ParamCurveDeriv&gt;::gauss_arclen(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(
    &amp;&lt;kurbo::CircleSegment as kurbo::Shape&gt;::to_path(&amp;CircleSegment::new(point,
    0.0f64, f64::NEG_INFINITY, f64::INFINITY, 1.0f64), 0.0f64)).unwrap(), &amp;[]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  repro_out_of_memory_in_kurbo_line_gauss_arclen_3f3d79::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::CubicBez as kurbo::Shape&gt;::as_rounded_rect(
    &amp;BezPath::with_capacity(1000000000usize).get_seg(
    0usize).unwrap().to_cubic());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_cubicbez_as_rounded_rect_7066eb::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = LineIntersection { line_t: 0.0f64, segment_t: 1.0f64 };
let axis = Axis::Horizontal;
let point = axis.pack_point(0.0f64, 0.0f64);
let _ = SvgParseError::UnknownCommand('a');
let _ = &lt;kurbo::Line as kurbo::Shape&gt;::winding(
    &amp;&lt;kurbo::Circle as kurbo::Shape&gt;::as_line(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(
    &amp;&lt;kurbo::CircleSegment as kurbo::Shape&gt;::into_path(CircleSegment::new(point,
    0.0f64, 1.0f64, 0.0f64, 0.9999999999999999f64), 0.0f64)).unwrap()).unwrap(),
    point);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;&gt;, kurbo::... (+133 chars)
  kurbo::shape::Shape::to_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::circle::CircleSegment&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_line_winding_831567::main (src/main.rs:11)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Circle as kurbo::Shape&gt;::area(
    &amp;&lt;kurbo::Line as kurbo::Shape&gt;::as_circle(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap()).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_circle_area_31a024::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let circle_segment = CircleSegment::new(Point::from((0.0f32, 0.0f32)), 1.0f64,
    0.0f64, -1e300f64, 0.714f64);
let arc = circle_segment.outer_arc();
let _ = &lt;kurbo::RoundedRect as kurbo::Shape&gt;::as_rounded_rect(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_rounded_rect(
    &amp;&lt;kurbo::Arc as kurbo::Shape&gt;::into_path(arc, 0.0f64)).unwrap());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::sample_ellipse (src/arc.rs:165)
  kurbo::arc::{impl#1}::next (src/arc.rs:145)
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:83)
  repro_timeout_in_kurbo_roundedrect_as_rounded_rect_3797b5::main (src/main.rs:7)
  (14 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let affine = Affine::skew(0.0f64, 2.5699999999999997e-14f64);
let axis = Axis::Horizontal;
let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::as_rect(&amp;kurbo::expand_path(
    &lt;kurbo::Affine as ::core::ops::Mul&lt;kurbo::Arc&gt;&gt;::mul(affine,
    Arc::new(Point::from((f64::MIN, 430.0f64)), axis.pack_vec2(f64::MIN,
    f64::NAN), 0.5f64, -1.1805916207174113e21f64, 0.0f64)),
    Diagonal2::new(-20.5f64, f64::NEG_INFINITY), Join::Bevel, 65536.0f64,
    1e-310f64));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::arc::rotate_pt (src/arc.rs:173)
  kurbo::arc::sample_ellipse (src/arc.rs:168)
  kurbo::arc::{impl#1}::next (src/arc.rs:145)
  kurbo::bezpath::{impl#13}::next&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:837)
  kurbo::bezpath::{impl#14}::next&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt; (kurbo/src/bezpath.rs:893)
  kurbo::bezpath::Segments&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt;&gt;::area&lt;kurbo::bezpath::CloseSubpaths&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::Pat... (+61 chars)
  kurbo::expand::expand_path&lt;kurbo::arc::Arc&gt; (kurbo/src/expand.rs:134)
  repro_timeout_in_kurbo_bezpath_as_rect_6910da::main (src/main.rs:11)
  (12 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::simplify::simplify_bezpath(vec![PathEl::ClosePath], 0.0f64,
    &amp;SimplifyOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at kurbo/src/bezpath.rs:275:9:
uninitialized subpath (missing MoveTo)
backtrace:
  &lt;kurbo::bezpath::BezPath&gt;::close_path (kurbo/src/bezpath.rs:275:9)
  kurbo::simplify::simplify_bezpath::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt; (kurbo/src/simplify.rs:337:30)
  repro_kurbo_src_bezpath_rs_275_9_5a4815::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = kurbo::expand_path(&lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(
    &amp;BezPath::with_capacity(1000000000usize)).unwrap(), Diagonal2::new(0.0f64,
    0.0f64), Join::Round, 0.0f64, 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_expand_path_circle_2777f1::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Rect as kurbo::Shape&gt;::as_circle(&amp;BezPath::with_capacity(
    1000000000usize).control_box());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_rect_as_circle_bd8c42::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::PathSeg as kurbo::ParamCurveExtrema&gt;::extrema(
    &amp;BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_pathseg_extrema_96c87e::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Line as kurbo::Shape&gt;::as_rounded_rect(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_line_as_rounded_rect_3a5f68::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Circle as kurbo::Shape&gt;::as_circle(&amp;Circle::new(
    BezPath::with_capacity(1000000000usize).current_position().unwrap(),
    0.0f64));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_circle_as_circle_7373d3::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::CubicBez as kurbo::Shape&gt;::path_elements(
    &amp;BezPath::with_capacity(1000000000usize).get_seg(
    0usize).unwrap().to_cubic(), 0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_cubicbez_path_elements_f6e794::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Stroke::default();
let _ = Cap::Square;
let f64_v = DEFAULT_ACCURACY;
let _ = &lt;kurbo::Ellipse as kurbo::Shape&gt;::to_path(
    &amp;&lt;kurbo::Ellipse as ::core::ops::Add&lt;kurbo::Vec2&gt;&gt;::add(
    Ellipse::from_affine(
    &lt;kurbo::Affine as ::core::ops::Mul&lt;kurbo::Affine&gt;&gt;::mul(Affine::FLIP_X,
    Affine::rotate(-4.34e22f64))), Vec2::default()), 0.0f64);
let _ = StrokeCtx::default();
let _ = Diagonal2 { xx: f64::NAN, yy: 922.0f64 };
let _ = Join::Bevel;
let svg_parse_error = SvgParseError::Wrong;
let _ = svg_parse_error.to_string();
let _ = RoundedRectRadii::new(-0.0f64, f64_v, 1e-310f64, 1.0f64);</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::ellipse::Ellipse&gt; (kurbo/src/shape.rs:68)
  repro_timeout_in_kurbo_ellipse_to_path_fa2400::main (src/main.rs:17)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::RoundedRect as kurbo::Shape&gt;::as_circle(
    &amp;&lt;kurbo::PathSeg as kurbo::Shape&gt;::as_rounded_rect(&amp;BezPath::with_capacity(
    1000000000usize).get_seg(0usize).unwrap()).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_roundedrect_as_circle_4959b2::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = SimplifyBezPath::new(vec![BezPath::with_capacity(
    1000000000usize).pop().unwrap()]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_simplify_simplifybezpath_new_d97c0d::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Line as kurbo::ParamCurveExtrema&gt;::extrema(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_line_extrema_b67c7d::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::RoundedRect as kurbo::Shape&gt;::bounding_box(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_rounded_rect(&amp;BezPath::with_capacity(
    10000000000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 560000000000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_roundedrect_bounding_box_7dabde::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::BezPath as kurbo::Shape&gt;::contains(
    &amp;&lt;kurbo::Arc as kurbo::Shape&gt;::into_path(Arc { center: Point::from((0.0f32,
    0.0f32)), radii: Vec2::from((0.0f64, 0.0f64)),
    start_angle: 2.2250738585072014e-308f64, sweep_angle: f64::INFINITY,
    x_rotation: -0.0f64 }, 0.0f64), Point::default());</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096384 bytes failed
backtrace:
  kurbo::bezpath::{impl#1}::from_iter&lt;core::iter::adapters::chain::Chain&lt;core::iter::sources::once::Once&lt;kurbo::bezpath::PathEl&gt;, kurbo::arc::ArcAppendIter&gt;&gt; (kurbo/src/bezpath.rs:509)
  kurbo::shape::Shape::to_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:68)
  kurbo::shape::Shape::into_path&lt;kurbo::arc::Arc&gt; (kurbo/src/shape.rs:83)
  repro_out_of_memory_in_kurbo_bezpath_contains_b15da1::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::Arc as kurbo::Shape&gt;::contains(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_circle(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap().segment(0.0f64, 0.0f64, 0.0f64).inner_arc(),
    Point::new(0.0f64, 0.0f64));</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_arc_contains_635434::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::CubicBez as kurbo::Shape&gt;::path_segments(
    &amp;BezPath::with_capacity(67108863usize).get_seg(0usize).unwrap().to_cubic(),
    0.0f64);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3758096328 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_cubicbez_path_segments_3334e0::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;kurbo::ConstPoint as kurbo::ParamCurveDeriv&gt;::gauss_arclen(
    &amp;&lt;kurbo::Line as kurbo::ParamCurveDeriv&gt;::deriv(
    &amp;&lt;kurbo::BezPath as kurbo::Shape&gt;::as_line(&amp;BezPath::with_capacity(
    1000000000usize)).unwrap()), &amp;[]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 56000000000 bytes failed
backtrace:
  kurbo::bezpath::BezPath::with_capacity (src/bezpath.rs:192)
  repro_out_of_memory_in_kurbo_constpoint_gauss_arclen_b9140b::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
</table>
