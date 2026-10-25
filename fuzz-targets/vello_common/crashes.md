# vello_common - crashes

Source: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut decimation_sizer = DecimationSizer::default();
let _ = decimation_sizer.upscale();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/filter/gaussian_blur.rs:211:57:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;vello_common::filter::gaussian_blur::DecimationSizer&gt;::upscale (vello_common/src/filter/gaussian_blur.rs:211:57)
  repro_vello_common_src_filter_gaussian_blur_rs_211_57_7a378e::main (src/main.rs:5:30)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Tile::new_clamped(0u16, 0u16, 1000000000u32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:296:13:
Max. number of lines per path exceeded.
backtrace:
  &lt;vello_common::tile::Tile&gt;::new (vello_common/src/tile.rs:296:13)
  &lt;vello_common::tile::Tile&gt;::new_clamped (vello_common/src/tile.rs:277:9)
  repro_vello_common_src_tile_rs_296_13_07081d::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let pixmap = Pixmap::new(0u16, 0u16);
let _ = pixmap.sample_idx(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/pixmap.rs:399:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[color::rgba8::PremulRgba8]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[color::rgba8::PremulRgba8] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;vello_common::pixmap::Pixmap&gt;::sample_idx (vello_common/src/pixmap.rs:399:17)
  repro_vello_common_src_pixmap_rs_399_17_a6c060::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut viewport_state = ViewportState::new(0u16, 0u16, Level::baseline());
viewport_state.pop_clip();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/clip.rs:307:45:
clip stack underflowed
backtrace:
  &lt;vello_common::clip::ClipState&gt;::pop_clip (vello_common/src/clip.rs:307:45)
  &lt;vello_common::viewport::ViewportState&gt;::pop_clip (vello_common/src/viewport.rs:90:25)
  repro_vello_common_src_clip_rs_307_45_bc93a4::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = IndexedPaint::new(100000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/paint.rs:26:35:
exceeded the maximum number of paints: TryFromIntError(PosOverflow)
backtrace:
  &lt;vello_common::paint::IndexedPaint&gt;::new (vello_common/src/paint.rs:26:35)
  repro_vello_common_src_paint_rs_26_35_ba126e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut clip_context = ClipContext::default();
clip_context.pop_clip();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/clip.rs:148:42:
clip stack underflowed
backtrace:
  &lt;vello_common::clip::ClipContext&gt;::pop_clip (vello_common/src/clip.rs:148:42)
  repro_vello_common_src_clip_rs_148_42_3f3aff::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let mut command_recorder = &lt;vello_common::record::CommandRecorder&lt;u8&gt; as ::
    core::default::Default&gt;::default();
let _ = command_recorder.pop_layer();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/record.rs:310:44:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;vello_common::record::CommandRecorder&lt;u8&gt;&gt;::pop_layer (vello_common/src/record.rs:310:44)
  repro_vello_common_src_record_rs_310_44_bdf825::main (src/main.rs:3:30)</pre></td></tr>
<tr><td valign="top"><pre>let root_transforms = RootTransforms::new();
let affine = root_transforms.effective_path_transform(&amp;Transforms::default());
let _ = PreparedFilter::new(&amp;Filter { graph: std::sync::Arc::new(
    FilterGraph::new()) }, &amp;affine);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/filter/mod.rs:44:13:
not implemented: Multi-primitive filter graphs are not yet supported
backtrace:
  &lt;vello_common::filter::PreparedFilter&gt;::new (vello_common/src/filter/mod.rs:44:13)
  repro_vello_common_src_filter_mod_rs_44_13_bc5b05::main (src/main.rs:10:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Filter::from_function(FilterFunction::Contrast { amount: 0.0f32 });</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/filter_effects.rs:84:18:
not implemented: Filter function Contrast { amount: 0.0 } not supported
backtrace:
  &lt;vello_common::filter_effects::Filter&gt;::from_function (vello_common/src/filter_effects.rs:84:18)
  repro_vello_common_src_filter_effects_rs_84_18_3c4f3a::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let size_u16 = SizeU16::from((0u16, 0u16));
let _ = size_u16.clamp(1u16, 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/geometry.rs:57:36:
min &gt; max. min = 1, max = 0
backtrace:
  &lt;u16 as core::cmp::Ord&gt;::clamp::do_panic::runtime (core/src/panic.rs:172:21)
  &lt;u16 as core::cmp::Ord&gt;::clamp::do_panic (core/src/intrinsics/mod.rs:2636:9)
  &lt;u16 as core::cmp::Ord&gt;::clamp (core/src/panic.rs:177:9)
  &lt;vello_common::geometry::SizeU16&gt;::clamp (vello_common/src/geometry.rs:57:36)
  repro_vello_common_src_geometry_rs_57_36_2c0aa2::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let mut culled_windings = CulledWindings::default();
culled_windings.mark_row_active(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:110:20:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u32]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[u32] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;vello_common::tile::CulledWindings&gt;::mark_row_active (vello_common/src/tile.rs:110:20)
  repro_vello_common_src_tile_rs_110_20_97c839::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let strip_fill_segment = StripFillSegment { tile_x0: u16::MAX, tile_x1: 0u16,
    tile_y: 0u16 };
let _ = strip_fill_segment.x0();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/strip.rs:70:9:
attempt to multiply with overflow
backtrace:
  &lt;vello_common::strip::StripFillSegment&gt;::x0 (vello_common/src/strip.rs:70:9)
  repro_vello_common_src_strip_rs_70_9_3c92b6::main (src/main.rs:5:32)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Strip::new(0u16, 0u16, u32::MAX, false);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/strip.rs:205:9:
`alpha_idx` too large
backtrace:
  &lt;vello_common::strip::Strip&gt;::new (vello_common/src/strip.rs:205:9)
  repro_vello_common_src_strip_rs_205_9_665900::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 1usize,
    max_atlases: 0usize, atlas_size: (0u16, 0u16), auto_grow: false,
    allocation_strategy: AllocationStrategy::LeastUsed });</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/multi_atlas.rs:129:18:
Failed to create initial atlas: AtlasLimitReached { max_atlases: 0, diagnostics: Unavailable }
backtrace:
  &lt;vello_common::multi_atlas::MultiAtlasManager&gt;::new (vello_common/src/multi_atlas.rs:129:18)
  &lt;vello_common::image_cache::ImageCache&gt;::new_with_config (vello_common/src/image_cache.rs:72:28)
  repro_vello_common_src_multi_atlas_rs_129_18_f60e04::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let node = Node { draws: 0u32..1u32, layer: None };
let _ = node.draws_in::&lt;u32&gt;(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/record.rs:67:15:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[u32] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;vello_common::record::Node&gt;::draws_in::&lt;u32&gt; (vello_common/src/record.rs:67:15)
  repro_vello_common_src_record_rs_67_15_a5fb44::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let mut culled_windings = CulledWindings::new(0u16);
culled_windings.mark_row_range_active(0usize, 10000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:135:24:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u32]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[u32] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;vello_common::tile::CulledWindings&gt;::mark_row_range_active (vello_common/src/tile.rs:135:24)
  repro_vello_common_src_tile_rs_135_24_ddfd6d::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let strip_fill_segment = StripFillSegment { tile_x0: 0u16, tile_x1: 0u16,
    tile_y: u16::MAX };
let _ = strip_fill_segment.y();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/strip.rs:82:9:
attempt to multiply with overflow
backtrace:
  &lt;vello_common::strip::StripFillSegment&gt;::y (vello_common/src/strip.rs:82:9)
  repro_vello_common_src_strip_rs_82_9_b9fb51::main (src/main.rs:5:32)</pre></td></tr>
<tr><td valign="top"><pre>let culled_windings = CulledWindings::new(1u16);
culled_windings.for_active_rows_in_range::&lt;fn(usize)&gt;(0usize, 1000000000usize,
    |_| ());</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:203:37:
index out of bounds: the len is 1 but the index is 31249999
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u32]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[u32] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;vello_common::tile::CulledWindings&gt;::for_active_rows_in_range::&lt;fn(usize)&gt; (vello_common/src/tile.rs:203:37)
  repro_vello_common_src_tile_rs_203_37_e8cb8c::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vello_common::geometry::SizeU16 as ::core::ops::Add&lt;u16&gt;&gt;::add(
    SizeU16::from_wh(0u16, u16::MAX), 1u16);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/geometry.rs:94:53:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;vello_common::geometry::SizeU16 as core::ops::arith::Add&gt;::add (vello_common/src/geometry.rs:94:53)
  &lt;vello_common::geometry::SizeU16 as core::ops::arith::Add&lt;u16&gt;&gt;::add (vello_common/src/geometry.rs:103:9)
  repro_vello_common_src_geometry_rs_94_53_6f9d5f::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = vello_common::util::strip_bbox(&amp;[Strip::sentinel(1u16, 0u32),
    Strip::new(0u16, 0u16, 0u32, false)]);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/strip.rs:143:9:
assertion `left == right` failed: strip x must be tile-width aligned
  left: 3
 right: 0
backtrace:
  vello_common::strip::visit_strip_fill_segments::&lt;vello_common::geometry::RectU16, vello_common::util::strip_bbox::{closure#0}, vello_common::util::strip_bbox::{closure#1}&gt; (vello_common/src/strip.rs:143:9)
  vello_common::util::strip_bbox (vello_common/src/util.rs:409:5)
  repro_vello_common_src_strip_rs_143_9_b8fa48::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vello_common::geometry::SizeU16 as ::core::ops::Add&lt;vello_common::
    geometry::SizeU16&gt;&gt;::add(SizeU16([u16::MAX, 0u16]), SizeU16::new(1u16));</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/geometry.rs:93:51:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;vello_common::geometry::SizeU16 as core::ops::arith::Add&gt;::add (vello_common/src/geometry.rs:93:51)
  repro_vello_common_src_geometry_rs_93_51_0e7a54::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut viewport_state = ViewportState::new(0u16, 0u16, Level::baseline());
viewport_state.pop_root_viewport();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/viewport.rs:120:14:
root viewport stack underflow
backtrace:
  &lt;vello_common::viewport::ViewportState&gt;::pop_root_viewport (vello_common/src/viewport.rs:120:14)
  repro_vello_common_src_viewport_rs_120_14_a72b21::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PreparedFilter::new(&amp;Filter::from_primitive(
    FilterPrimitive::Image { image_id: 0u32, transform: None }),
    &amp;Affine::scale(0.0f64));</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/filter/mod.rs:102:17:
not implemented: Other filter primitives not yet implemented
backtrace:
  &lt;vello_common::filter::PreparedFilter&gt;::new (vello_common/src/filter/mod.rs:102:17)
  repro_vello_common_src_filter_mod_rs_102_17_babfa1::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut clip_state = ClipState::default();
let mut strip_generator = StripGenerator::new(0u16, 0u16,
    Level::try_detect().unwrap());
clip_state.pop_root_viewport(&amp;mut strip_generator);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/clip.rs:253:39:
filter clip stack underflow
backtrace:
  &lt;vello_common::clip::ClipState&gt;::pop_root_viewport (vello_common/src/clip.rs:253:39)
  repro_vello_common_src_clip_rs_253_39_eb067c::main (src/main.rs:8:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut culled_windings = CulledWindings::new(1u16);
culled_windings.mark_row_range_active(0usize, 100000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:137:24:
range end index 3124999999999999 out of range for slice of length 1
backtrace:
  &lt;[u32] as core::ops::index::IndexMut&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;vello_common::tile::CulledWindings&gt;::mark_row_range_active (vello_common/src/tile.rs:137:24)
  repro_vello_common_src_tile_rs_137_24_d6441b::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let strip_fill_segment = StripFillSegment { tile_x0: 0u16, tile_x1: u16::MAX,
    tile_y: 0u16 };
let _ = strip_fill_segment.x1();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/strip.rs:76:9:
attempt to multiply with overflow
backtrace:
  &lt;vello_common::strip::StripFillSegment&gt;::x1 (vello_common/src/strip.rs:76:9)
  repro_vello_common_src_strip_rs_76_9_231715::main (src/main.rs:5:32)</pre></td></tr>
<tr><td valign="top"><pre>let _ = vello_common::filter::gaussian_blur::compute_gaussian_kernel(1e30f32);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/filter/gaussian_blur.rs:140:28:
attempt to multiply with overflow
backtrace:
  vello_common::filter::gaussian_blur::compute_gaussian_kernel (vello_common/src/filter/gaussian_blur.rs:140:28)
  repro_vello_common_src_filter_gaussian_blur_rs_140_28_8bfd80::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut culled_windings = CulledWindings::default();
culled_windings.mark_row_range_active(0usize, 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:132:24:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u32]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[u32] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;vello_common::tile::CulledWindings&gt;::mark_row_range_active (vello_common/src/tile.rs:132:24)
  repro_vello_common_src_tile_rs_132_24_2aa949::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = FilterData::new(Filter::from_primitive(
    FilterPrimitive::Composite { operator: CompositeOperator::In }),
    Affine::new([0.0f64, 0.0f64, f64::NEG_INFINITY, 0.0f64, 0.0f64, 0.0f64]));</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/filter/mod.rs:242:13:
filter expansion must contain the origin
backtrace:
  &lt;vello_common::filter::FilterData&gt;::new::snapped_padding (vello_common/src/filter/mod.rs:242:13)
  &lt;vello_common::filter::FilterData&gt;::new (vello_common/src/filter/mod.rs:264:30)
  repro_vello_common_src_filter_mod_rs_242_13_ada9ef::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let culled_windings = CulledWindings::new(0u16);
culled_windings.for_active_rows_in_range::&lt;fn(usize)&gt;(0usize, 1usize, |_| ());</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:179:39:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u32]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[u32] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;vello_common::tile::CulledWindings&gt;::for_active_rows_in_range::&lt;fn(usize)&gt; (vello_common/src/tile.rs:179:39)
  repro_vello_common_src_tile_rs_179_39_394b5f::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut strip = Strip::sentinel(0u16, 0u32);
strip.set_alpha_idx(u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/strip.rs:255:9:
`alpha_idx` too large
backtrace:
  &lt;vello_common::strip::Strip&gt;::set_alpha_idx (vello_common/src/strip.rs:255:9)
  repro_vello_common_src_strip_rs_255_9_fbf708::main (src/main.rs:5:11)</pre></td></tr>
<tr><td valign="top"><pre>let pixmap = Pixmap::new(1u16, 0u16);
let _ = pixmap.sample(0u16, 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/pixmap.rs:392:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[color::rgba8::PremulRgba8]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[color::rgba8::PremulRgba8] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;vello_common::pixmap::Pixmap&gt;::sample (vello_common/src/pixmap.rs:392:17)
  repro_vello_common_src_pixmap_rs_392_17_07fcf5::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let paint = Paint::Indexed(IndexedPaint::new(0usize));
let _ = paint.may_have_transparency(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/encode.rs:565:37:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;vello_common::paint::Paint&gt;::may_have_transparency (vello_common/src/encode.rs:565:37)
  repro_vello_common_src_encode_rs_565_37_469822::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut pixmap = Pixmap::new(0u16, 0u16);
pixmap.resize(10000u16, u16::MAX);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2621400000 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::resize (src/pixmap.rs:169)
  repro_out_of_memory_in_vello_common_pixmap_pixmap_resize_ee5efb::main (src/main.rs:5)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = vello_common::filter::gaussian_blur::plan_decimated_blur(f32::INFINITY);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  vello_common::filter::gaussian_blur::plan_decimated_blur (src/filter/gaussian_blur.rs:119)
  repro_timeout_in_vello_common_filter_gaussian_blur_plan_decimated__e6541f::main (src/main.rs:2)</pre></td></tr>
<tr><td valign="top"><pre>let _ = vello_common::util::strip_bbox(&amp;[Strip::new(0u16, 0u16, 0u32, false),
    Strip::new(0u16, 0u16, 100u32, false)]);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/strip.rs:148:9:
assertion `left == right` failed: strip width must be tile-width aligned
  left: 1
 right: 0
backtrace:
  vello_common::strip::visit_strip_fill_segments::&lt;vello_common::geometry::RectU16, vello_common::util::strip_bbox::{closure#0}, vello_common::util::strip_bbox::{closure#1}&gt; (vello_common/src/strip.rs:148:9)
  vello_common::util::strip_bbox (vello_common/src/util.rs:409:5)
  repro_vello_common_src_strip_rs_148_9_307bb5::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut image_cache = ImageCache::new_dummy();
let _ = image_cache.deallocate(Resources::default().register_image(
    std::sync::Arc::new(Pixmap::new(10000u16, u16::MAX))));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2621400000 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::new (src/pixmap.rs:99)
  repro_out_of_memory_in_vello_common_image_cache_imagecache_dealloc_2b9f45::main (src/main.rs:7)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut clip_context = ClipContext::default();
let mut strip_generator = StripGenerator::new(1u16, u16::MAX,
    Level::baseline());
clip_context.push_clip_rect(&amp;Rect { x0: 1.0f64, y0: 960000.0f64, x1: 0.0f64,
    y1: 0.0f64 }, &amp;mut strip_generator);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/util.rs:430:52:
called `Option::unwrap()` on a `None` value
backtrace:
  vello_common::util::strip_bbox (vello_common/src/util.rs:430:52)
  &lt;vello_common::clip::ClipContext&gt;::push_generated_clip::&lt;&lt;vello_common::clip::ClipContext&gt;::push_clip_rect::{closure#0}&gt; (vello_common/src/clip.rs:135:20)
  &lt;vello_common::clip::ClipContext&gt;::push_clip_rect (vello_common/src/clip.rs:116:14)
  repro_vello_common_src_util_rs_430_52_5fba5b::main (src/main.rs:9:18)</pre></td></tr>
<tr><td valign="top"><pre>let mut root_transforms = RootTransforms::new();
root_transforms.pop_root();
let _ = root_transforms.root_transform();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/transforms.rs:105:14:
root transform stack should never be empty
backtrace:
  &lt;vello_common::transforms::RootTransforms&gt;::root_transform (vello_common/src/transforms.rs:105:14)
  repro_vello_common_src_transforms_rs_105_14_ef0678::main (src/main.rs:6:29)</pre></td></tr>
<tr><td valign="top"><pre>let tile = Tile::SENTINEL;
let _ = tile.prev_loc(&amp;Tile { packed_winding_line_idx: 0u32, x: 0u16,
    y: u16::MAX });</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:316:33:
attempt to add with overflow
backtrace:
  &lt;vello_common::tile::Tile&gt;::prev_loc (vello_common/src/tile.rs:316:33)
  repro_vello_common_src_tile_rs_316_33_44a36b::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Pixmap::new(10000u16, u16::MAX);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2621400000 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::new (src/pixmap.rs:99)
  repro_out_of_memory_in_vello_common_pixmap_pixmap_new_bfce70::main (src/main.rs:4)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let multi_atlas_manager = MultiAtlasManager::new(
    AtlasConfig { initial_atlas_count: 1000000000usize,
    max_atlases: 10000000000000000000usize, atlas_size: (56132u16, 80u16),
    auto_grow: false, allocation_strategy: AllocationStrategy::RoundRobin });
let _ = multi_atlas_manager.config();</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  repro_out_of_memory_in_vello_common_multi_atlas_multiatlasmanager__04c35e::main (src/main.rs:6)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = GaussianBlur::new(1e30f32, EdgeMode::None);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  vello_common::filter::gaussian_blur::plan_decimated_blur (src/filter/gaussian_blur.rs:119)
  vello_common::filter::gaussian_blur::GaussianBlur::new (src/filter/gaussian_blur.rs:74)
  repro_timeout_in_vello_common_filter_gaussian_blur_gaussianblur_ne_5ee26d::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let image_cache = ImageCache::new_dummy();
let _ = image_cache.get(ImageCache::new_with_config(
    AtlasConfig { initial_atlas_count: 10000000000000000000usize,
    max_atlases: 11628016252581910041usize, atlas_size: (u16::MAX, 1u16),
    auto_grow: false,
    allocation_strategy: AllocationStrategy::RoundRobin }).allocate(u16::MAX,
    127u16, u16::MAX).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 1 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:432)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  vello_common::image_cache::ImageCache::new_with_config (src/image_cache.rs:72)
  repro_out_of_memory_in_vello_common_image_cache_imagecache_get_dbc923::main (src/main.rs:7)
  (35 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = MultiAtlasManager::new(
    AtlasConfig { initial_atlas_count: 1000000000usize,
    max_atlases: 1000000000usize, atlas_size: (1u16, 1u16), auto_grow: false,
    allocation_strategy: AllocationStrategy::LeastUsed });</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  repro_out_of_memory_in_vello_common_multi_atlas_multiatlasmanager__4dd5ea::main (src/main.rs:6)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Mask::new_luminance(&amp;Pixmap::new(u16::MAX, 10000u16));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2621400000 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::new (src/pixmap.rs:99)
  repro_out_of_memory_in_vello_common_mask_mask_new_luminance_8badd4::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageCache::new_with_config(
    AtlasConfig { initial_atlas_count: 1000000000usize,
    max_atlases: 10000000000000usize, atlas_size: (1u16, 1u16),
    auto_grow: false, allocation_strategy: AllocationStrategy::RoundRobin });</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  vello_common::image_cache::ImageCache::new_with_config (src/image_cache.rs:72)
  repro_out_of_memory_in_vello_common_image_cache_imagecache_new_wit_954628::main (src/main.rs:6)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Mask::new_alpha(&amp;Pixmap::new(u16::MAX, 8368u16));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2193587520 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::new (src/pixmap.rs:99)
  repro_out_of_memory_in_vello_common_mask_mask_new_alpha_a2738a::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Atlas::new(MultiAtlasManager::new(
    AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: usize::MAX,
    atlas_size: (2049u16, 16384u16), auto_grow: false,
    allocation_strategy: AllocationStrategy::
    default() }).create_atlas().unwrap(), u16::MAX, 16384u16);</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  repro_out_of_memory_in_vello_common_multi_atlas_atlas_new_a73744::main (src/main.rs:7)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let atlas_error = AtlasError::AtlasLimitReached { max_atlases: 0usize,
    diagnostics: AtlasSpaceDiagnostics::Allocation { width: 0u16, height: 0u16,
    atlas_width: 0u16, atlas_height: 0u16, max_atlases: 0usize,
    atlases: vec![AtlasLayerDiagnostics { atlas_id: AtlasId::new(0u32),
    total_area: 0u64, free_area: u64::MAX, free_rectangle_count: 255usize,
    largest_free_width: 7u16, largest_free_height: 63490u16 },
    AtlasLayerDiagnostics { atlas_id: AtlasId(1u32), total_area: 76u64,
    free_area: 18446744073709551614u64, free_rectangle_count: 11usize,
    largest_free_width: 32266u16, largest_free_height: 65534u16 }] } };
let _ = atlas_error.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/iter/traits/accum.rs:206:1:
attempt to add with overflow
backtrace:
  &lt;u64 as core::iter::traits::accum::Sum&gt;::sum::&lt;core::iter::adapters::map::Map&lt;core::slice::iter::Iter&lt;vello_common::multi_atlas::AtlasLayerDiagnostics&gt;, &lt;vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display&gt;::fmt::{closure#1}&gt;&gt;::{closure#0} (core/src/iter/traits/accum.rs:55:28)
  &lt;u64 as core::iter::traits::accum::Sum&gt;::sum::&lt;core::iter::adapters::map::Map&lt;core::slice::iter::Iter&lt;vello_common::multi_atlas::AtlasLayerDiagnostics&gt;, &lt;vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display&gt;::fmt::{closure#1}&gt;&gt; (core/src/iter/traits/accum.rs:52:22)
  &lt;vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display&gt;::fmt (vello_common/src/multi_atlas.rs:519:69)
  &lt;&amp;vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display&gt;::fmt (core/src/fmt/mod.rs:2872:62)
  &lt;vello_common::multi_atlas::AtlasError as core::fmt::Display&gt;::fmt (vello_common/src/multi_atlas.rs:429:24)
  &lt;vello_common::multi_atlas::AtlasError as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;vello_common::multi_atlas::AtlasError as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_vello_common_src_multi_atlas_rs_429_24_d2e77a::main (src/main.rs:8:25)</pre></td></tr>
<tr><td valign="top"><pre>let image_cache = ImageCache::new_with_config(
    AtlasConfig { initial_atlas_count: 1000000000usize,
    max_atlases: 1000000000usize, atlas_size: (1u16, 1u16), auto_grow: false,
    allocation_strategy: AllocationStrategy::LeastUsed });
let _ = image_cache.atlas_manager();</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  vello_common::image_cache::ImageCache::new_with_config (src/image_cache.rs:72)
  repro_out_of_memory_in_vello_common_image_cache_imagecache_atlas_m_25b1c7::main (src/main.rs:6)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut clip_context = ClipContext::default();
let mut strip_generator = StripGenerator::new(0u16, 0u16, Level::new());
clip_context.push_clip_path(vec![PathEl::ClosePath], &amp;mut strip_generator,
    Fill::NonZero, Affine::scale_non_uniform(0.0f64, 0.0f64), None);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/flatten_simd.rs:94:9:
Non-empty paths must begin with `PathEl::MoveTo`, got ClosePath
backtrace:
  vello_common::flatten_simd::flatten::&lt;fearless_simd::generated::avx512::Avx512, alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;, vello_common::flatten::FlattenerCallback&gt; (vello_common/src/flatten_simd.rs:94:9)
  vello_common::flatten::fill_impl::&lt;fearless_simd::generated::avx512::Avx512, alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt; (vello_common/src/flatten.rs:209:5)
  vello_common::flatten::fill::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt;::{closure#3} (vello_common/src/flatten.rs:186:30)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::vectorize_avx512::&lt;vello_common::flatten::fill&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt;::{closure#3}, ()&gt; (fearless_simd/src/generated/avx512.rs:396:13)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::&lt;vello_common::flatten::fill&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt;::{closure#3}, ()&gt; (fearless_simd/src/generated/avx512.rs:398:18)
  vello_common::flatten::fill::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt; (vello_common/src/flatten.rs:186:5)
  &lt;vello_common::strip_generator::StripGenerator&gt;::generate_filled_path::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt; (vello_common/src/strip_generator.rs:132:9)
  &lt;vello_common::clip::ClipContext&gt;::push_clip_path::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt;::{closure#0} (vello_common/src/clip.rs:101:23)
  &lt;vello_common::clip::ClipContext&gt;::push_generated_clip::&lt;&lt;vello_common::clip::ClipContext&gt;::push_clip_path&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt;::{closure#0}&gt; (vello_common/src/clip.rs:133:21)
  &lt;vello_common::clip::ClipContext&gt;::push_clip_path::&lt;alloc::vec::Vec&lt;kurbo::bezpath::PathEl&gt;&gt; (vello_common/src/clip.rs:100:14)
  repro_vello_common_src_flatten_simd_rs_94_9_f81a82::main (src/main.rs:11:18)</pre></td></tr>
<tr><td valign="top"><pre>let _ = AtlasLayerDiagnostics { atlas_id: MultiAtlasManager::new(
    AtlasConfig { initial_atlas_count: 1000000000usize,
    max_atlases: 1000000000usize, atlas_size: (1u16, 1u16), auto_grow: false,
    allocation_strategy: AllocationStrategy::
    RoundRobin }).create_atlas().unwrap(), total_area: 0u64, free_area: 0u64,
    free_rectangle_count: 0usize, largest_free_width: 0u16,
    largest_free_height: 0u16 };</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  repro_out_of_memory_in_vello_common_multi_atlas_atlaslayerdiagnost_974339::main (src/main.rs:7)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut clip_context = ClipContext::new();
let mut strip_generator = StripGenerator::new(u16::MAX, 100u16,
    Level::baseline());
clip_context.push_clip_rect(&amp;&lt;kurbo::Circle as kurbo::Shape&gt;::bounding_box(
    &amp;Triangle::ZERO.inscribed_circle()), &amp;mut strip_generator);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/util.rs:429:51:
called `Option::unwrap()` on a `None` value
backtrace:
  vello_common::util::strip_bbox (vello_common/src/util.rs:429:51)
  &lt;vello_common::clip::ClipContext&gt;::push_generated_clip::&lt;&lt;vello_common::clip::ClipContext&gt;::push_clip_rect::{closure#0}&gt; (vello_common/src/clip.rs:135:20)
  &lt;vello_common::clip::ClipContext&gt;::push_clip_rect (vello_common/src/clip.rs:116:14)
  repro_vello_common_src_util_rs_429_51_34a98f::main (src/main.rs:9:18)</pre></td></tr>
<tr><td valign="top"><pre>let _ = LayerProps { blend_mode: RenderContext::new(0u16, 0u16).blend_mode(),
    opacity: 0.0f32, mask: Some(Mask::new_alpha(&amp;Pixmap::new(u16::MAX,
    10000u16))), clip_path: None };</pre></td><td valign="top"><pre>out of memory: memory allocation of 2621400000 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::new (src/pixmap.rs:99)
  repro_out_of_memory_in_vello_common_record_layerprops_literal_12606f::main (src/main.rs:7)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageSource::opaque_id_with_transparency_hint(
    ImageCache::new_with_config(
    AtlasConfig { initial_atlas_count: 1000000000000000000usize,
    max_atlases: 10000000000000000000usize, atlas_size: (1u16, 8330u16),
    auto_grow: false,
    allocation_strategy: AllocationStrategy::BestFit }).allocate(u16::MAX,
    60u16, 34647u16).unwrap(), true);</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  vello_common::image_cache::ImageCache::new_with_config (src/image_cache.rs:72)
  repro_out_of_memory_in_vello_common_paint_imagesource_opaque_id_wi_4ca636::main (src/main.rs:7)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = vello_common::pixmap::unpremultiply_rgba8(&amp;mut vec![102u8, 179u8, 217u8,
    108u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/util.rs:451:13:
attempt to add with overflow
backtrace:
  &lt;u16 as core::ops::arith::Add&gt;::add (core/src/ops/arith.rs:104:45)
  &lt;u16 as vello_common::util::unpremultiply::Div256Ext&gt;::div_256 (vello_common/src/util.rs:451:13)
  vello_common::util::unpremultiply::scalar (vello_common/src/util.rs:495:55)
  vello_common::pixmap::unpremultiply_rgba8_impl::&lt;fearless_simd::generated::avx512::Avx512&gt; (vello_common/src/pixmap.rs:589:26)
  vello_common::pixmap::unpremultiply_rgba8::{closure#3} (vello_common/src/pixmap.rs:565:30)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::vectorize_avx512::&lt;vello_common::pixmap::unpremultiply_rgba8::{closure#3}, bool&gt; (fearless_simd/src/generated/avx512.rs:396:13)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::&lt;vello_common::pixmap::unpremultiply_rgba8::{closure#3}, bool&gt; (fearless_simd/src/generated/avx512.rs:398:18)
  vello_common::pixmap::unpremultiply_rgba8 (vello_common/src/pixmap.rs:565:5)
  repro_vello_common_src_util_rs_451_13_cd178d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageSource::Pixmap(std::sync::Arc::new(Pixmap::new(u16::MAX,
    10000u16)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2621400000 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::new (src/pixmap.rs:99)
  repro_out_of_memory_in_vello_common_paint_imagesource_pixmap_3614bc::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageSource::OpaqueId { id: ImageCache::new_with_config(
    AtlasConfig { initial_atlas_count: 1000000000usize,
    max_atlases: 1000000000usize, atlas_size: (1u16, 1u16), auto_grow: false,
    allocation_strategy: AllocationStrategy::BestFit }).allocate(0u16, 0u16,
    0u16).unwrap(), may_have_transparency: false };</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  vello_common::image_cache::ImageCache::new_with_config (src/image_cache.rs:72)
  repro_out_of_memory_in_vello_common_paint_imagesource_opaqueid_190f3c::main (src/main.rs:7)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let pixmap = Pixmap::new(5570u16, u16::MAX);
let _ = pixmap.clone().take_rgba8(ImageAlphaType::AlphaPremultiplied);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1460119800 bytes failed
backtrace:
  vello_common::pixmap::{impl#6}::clone (vello_common/src/pixmap.rs:26)
  repro_out_of_memory_in_vello_common_pixmap_pixmap_take_rgba8_05e2aa::main (src/main.rs:6)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vello_common::paint::NoOpImageResolver as vello_common::paint::
    ImageResolver&gt;::resolve(&amp;NoOpImageResolver,
    ImageCache::new_with_config(
    AtlasConfig { initial_atlas_count: 10000000000000000000usize,
    max_atlases: 17380519893440117345usize, atlas_size: (1u16, 256u16),
    auto_grow: true,
    allocation_strategy: AllocationStrategy::LeastUsed }).allocate(0u16,
    65518u16, 26046u16).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  vello_common::image_cache::ImageCache::new_with_config (src/image_cache.rs:72)
  repro_out_of_memory_in_vello_common_paint_noopimageresolver_resolv_8e4132::main (src/main.rs:7)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = AtlasUsageStats { allocated_area: 1u32, total_area: 1503970760u32,
    allocated_count: 1u32 };
let _ = Cap::Square;
let _ = ImageFormat::Bgra8;
let _ = SweepKind { start_angle: -0.25f32, inv_angle_delta: f32::NAN };
let atlas = Atlas::new(AtlasId(1u32), 8u16, 13959u16);
let _ = atlas.stats();
let _ = CuspType::Loop;
let _ = PreparedFilter::GaussianBlur(GaussianBlur::new(f32::INFINITY,
    EdgeMode::None));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  vello_common::filter::gaussian_blur::plan_decimated_blur (src/filter/gaussian_blur.rs:119)
  vello_common::filter::gaussian_blur::GaussianBlur::new (src/filter/gaussian_blur.rs:74)
  repro_timeout_in_vello_common_filter_preparedfilter_gaussianblur_13855b::main (src/main.rs:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut decimation_sizer = DecimationSizer::new(u16::MAX, 0u16);
let _ = decimation_sizer.downscale();
let _ = decimation_sizer.upscale();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/filter/gaussian_blur.rs:213:22:
attempt to multiply with overflow
backtrace:
  &lt;vello_common::filter::gaussian_blur::DecimationSizer&gt;::upscale (vello_common/src/filter/gaussian_blur.rs:213:22)
  repro_vello_common_src_filter_gaussian_blur_rs_213_22_a26d19::main (src/main.rs:6:30)</pre></td></tr>
<tr><td valign="top"><pre>let _ = AtlasError::AtlasNotFound(MultiAtlasManager::new(
    AtlasConfig { initial_atlas_count: 100000000000usize,
    max_atlases: 100000000000000000usize, atlas_size: (1u16, 1u16),
    auto_grow: false,
    allocation_strategy: AllocationStrategy::
    LeastUsed }).create_atlas().unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  repro_out_of_memory_in_vello_common_multi_atlas_atlaserror_atlasno_e09e79::main (src/main.rs:7)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageSource::opaque_id(Resources::default().register_image(
    std::sync::Arc::new(Pixmap::new(u16::MAX, 10000u16))));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2621400000 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::new (src/pixmap.rs:99)
  repro_out_of_memory_in_vello_common_paint_imagesource_opaque_id_c6a676::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let simplify_options = SimplifyOptions::default();
let simplify_options2 = simplify_options.angle_thresh(
    -2.9399999999999997e-26f64);
let simplify_options3 = simplify_options2.opt_level(
    SimplifyOptLevel::Subdivide);
let simplify_options4 = simplify_options3.opt_level(
    SimplifyOptLevel::Subdivide);
let _ = simplify_options4.opt_level(SimplifyOptLevel::Subdivide);
let mut image_cache = ImageCache::new_with_config(
    AtlasConfig { initial_atlas_count: 8330046325053843992usize,
    max_atlases: 4294967296usize, atlas_size: (16384u16, 255u16),
    auto_grow: true, allocation_strategy: AllocationStrategy::LeastUsed });
let image_resource = image_cache.deallocate(Resources::default().register_image(
    std::sync::Arc::new(Pixmap::from_parts(include_bytes!(
    "image_0.webp").to_vec(), 16382u16, 2u16,
    PixelMetadata::new(ImageAlphaType::Alpha, true))))).unwrap();
let _ = image_resource.size();
let _ = FocalKind::OnCircle;
let _ = image_resource.offsets();
let _ = Stroke::new(128.0f64);
let _ = StrokeOptLevel::Optimized;
let _ = vello_common::filter_effects::kernels::gaussian_3x3();
let _ = &lt;kurbo::Moments as ::core::ops::Add&lt;kurbo::Moments&gt;&gt;::add(
    Moments { moment_x: 0.9999999999999999f64, moment_y: 1.0f64,
    moment_xx: -74.3f64, moment_xy: 0.9999999999999999f64,
    moment_yy: 1e-310f64 }, Moments::default());
let _ = RenderMode::OptimizeQuality;</pre></td><td valign="top"><pre>timeout: Program received signal SIGABRT, Aborted
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:432)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  vello_common::image_cache::ImageCache::new_with_config (src/image_cache.rs:72)
  repro_timeout_in_vello_common_image_cache_imagecache_new_with_conf_9ce502::main (src/main.rs:22)
  (35 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vello_common::record::CommandRecorder&lt;u8&gt;&gt;::new(1u16, u16::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/util.rs:238:60:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;vello_common::geometry::RectU16 as vello_common::util::RectExt&gt;::snap_to_tile_coordinates (vello_common/src/util.rs:238:60)
  vello_common::record::snapped_scene_size (vello_common/src/record.rs:419:10)
  &lt;vello_common::record::CommandRecorder&lt;u8&gt;&gt;::new (vello_common/src/record.rs:232:25)
  repro_vello_common_src_util_rs_238_60_3fc766::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = AtlasAllocation { atlas_id: MultiAtlasManager::new(
    AtlasConfig { initial_atlas_count: usize::MAX,
    max_atlases: 10392699432343005273usize, atlas_size: (30005u16, 17u16),
    auto_grow: false,
    allocation_strategy: AllocationStrategy::
    default() }).create_atlas().unwrap(),
    allocation: Atlas::new(AtlasId(1073741824u32), 37347u16,
    1u16).allocate(1u16, 1u16).unwrap() };</pre></td><td valign="top"><pre>out of memory: memory allocation of 32 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:423)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  repro_out_of_memory_in_vello_common_multi_atlas_atlasallocation_li_c9ef3c::main (src/main.rs:9)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let atlas_config = AtlasConfig { initial_atlas_count: usize::MAX,
    max_atlases: 1125007570538422820usize, atlas_size: (1u16, 1u16),
    auto_grow: false, allocation_strategy: AllocationStrategy::FirstFit };
let _ = AtlasSpaceDiagnostics::Allocation { width: 1000u16, height: 256u16,
    atlas_width: 96u16, atlas_height: 16106u16, max_atlases: 256usize,
    atlases: vec![AtlasLayerDiagnostics { atlas_id: AtlasId(u32::MAX),
    total_area: 7726114339975327424u64, free_area: 5385087516234100114u64,
    free_rectangle_count: 36028797018963966usize, largest_free_width: 255u16,
    largest_free_height: 39821u16 },
    AtlasLayerDiagnostics { atlas_id: AtlasId(65536u32), total_area: u64::MAX,
    free_area: 18446744073709551614u64, free_rectangle_count: 65535usize,
    largest_free_width: 11278u16, largest_free_height: 68u16 },
    AtlasLayerDiagnostics { atlas_id: AtlasId::new(256u32), total_area: 1u64,
    free_area: 8u64, free_rectangle_count: 256usize,
    largest_free_width: 16385u16, largest_free_height: 9u16 },
    AtlasLayerDiagnostics { atlas_id: AtlasId::new(3491957232u32),
    total_area: 6921050401554868828u64, free_area: 0u64,
    free_rectangle_count: 4usize, largest_free_width: 0u16,
    largest_free_height: 65529u16 },
    AtlasLayerDiagnostics { atlas_id: MultiAtlasManager::new(
    atlas_config).create_atlas().unwrap(), total_area: 6u64, free_area: 13u64,
    free_rectangle_count: 0usize, largest_free_width: 256u16,
    largest_free_height: 31214u16 }] };</pre></td><td valign="top"><pre>out of memory: memory allocation of 16 bytes failed
backtrace:
  guillotiere::allocator::AtlasAllocator::with_options (src/allocator.rs:420)
  guillotiere::allocator::AtlasAllocator::new (src/allocator.rs:403)
  vello_common::multi_atlas::Atlas::new (src/multi_atlas.rs:50)
  vello_common::multi_atlas::MultiAtlasManager::create_atlas (src/multi_atlas.rs:151)
  vello_common::multi_atlas::MultiAtlasManager::new (src/multi_atlas.rs:128)
  repro_out_of_memory_in_vello_common_multi_atlas_atlasspacediagnost_1112e0::main (src/main.rs:10)
  (38 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vello_common::record::CommandRecorder&lt;u8&gt;&gt;::new(u16::MAX, 1u16);</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/util.rs:237:59:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;vello_common::geometry::RectU16 as vello_common::util::RectExt&gt;::snap_to_tile_coordinates (vello_common/src/util.rs:237:59)
  vello_common::record::snapped_scene_size (vello_common/src/record.rs:419:10)
  &lt;vello_common::record::CommandRecorder&lt;u8&gt;&gt;::new (vello_common/src/record.rs:232:25)
  repro_vello_common_src_util_rs_237_59_b645fa::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PreparedFilter::new(&amp;Filter::from_primitive(
    FilterPrimitive::GaussianBlur { std_deviation: 1.0356042e24f32,
    edge_mode: EdgeMode::None }), &amp;Affine::rotate_about(0.0f64,
    Point { x: 0.0f64, y: 1.0f64 }));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  vello_common::filter::gaussian_blur::plan_decimated_blur (src/filter/gaussian_blur.rs:120)
  vello_common::filter::gaussian_blur::GaussianBlur::new (src/filter/gaussian_blur.rs:74)
  vello_common::filter::PreparedFilter::new (src/filter/mod.rs:57)
  repro_timeout_in_vello_common_filter_preparedfilter_new_be78a1::main (src/main.rs:9)</pre></td></tr>
<tr><td valign="top"><pre>let mut decimation_sizer = DecimationSizer::new(1u16, u16::MAX);
let _ = decimation_sizer.downscale();
let _ = decimation_sizer.upscale();</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/filter/gaussian_blur.rs:214:23:
attempt to multiply with overflow
backtrace:
  &lt;vello_common::filter::gaussian_blur::DecimationSizer&gt;::upscale (vello_common/src/filter/gaussian_blur.rs:214:23)
  repro_vello_common_src_filter_gaussian_blur_rs_214_23_a8cd31::main (src/main.rs:6:30)</pre></td></tr>
<tr><td valign="top"><pre>let pixmap = Pixmap::new(u16::MAX, 7357u16);
let _ = pixmap.clone().try_take_rgb8(ImageAlphaType::AlphaPremultiplied);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1928563980 bytes failed
backtrace:
  vello_common::pixmap::{impl#6}::clone (vello_common/src/pixmap.rs:26)
  repro_out_of_memory_in_vello_common_pixmap_pixmap_try_take_rgb8_8b9268::main (src/main.rs:6)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = EncodedImage { source: ImageSource::opaque_id(
    Resources::new().register_image(std::sync::Arc::new(Pixmap::new(32766u16,
    16384u16)))), sampler: ImageSampler::default(),
    may_have_transparency: false, transform: Affine::IDENTITY,
    x_advance: Vec2::splat(0.0f64), y_advance: Vec2::from_angle(0.0f64),
    tint: None };</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147352576 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::new (src/pixmap.rs:99)
  repro_out_of_memory_in_vello_common_encode_encodedimage_literal_7dc008::main (src/main.rs:10)
  (19 std/libc frames omitted)</pre></td></tr>
</table>
