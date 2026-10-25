# femtovg - crashes

Source: https://github.com/femtovg/femtovg @ 9f5ad2614e922677a467e2d6f66fecba14d3a30a

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Color::hex("e9𮷶?N");</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/color.rs:74:31:
end byte index 4 is not a char boundary; it is inside '𮷶' (bytes 2..6 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;femtovg::color::Color&gt; (femtovg/src/color.rs:74:31)
  repro_femtovg_src_color_rs_74_31_6d7a28::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut atlas = Atlas::new(10000000000000000000usize, 0usize);
let _ = atlas.add_rect(1000000000000000000usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/text/atlas.rs:160:13:
attempt to subtract with overflow
backtrace:
  &lt;femtovg::text::atlas::Atlas&gt;::rect_fits (femtovg/src/text/atlas.rs:160:13)
  &lt;femtovg::text::atlas::Atlas&gt;::add_rect (femtovg/src/text/atlas.rs:62:35)
  repro_femtovg_src_text_atlas_rs_160_13_f0d024::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Color::hex("\u{200b}\u{200d}");</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/color.rs:79:37:
end byte index 2 is not a char boundary; it is inside '\u{200b}' (bytes 0..3 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;femtovg::color::Color&gt; (femtovg/src/color.rs:79:37)
  repro_femtovg_src_color_rs_79_37_ca91f5::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Color::hex("daTI\u{b6c46}");</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/color.rs:75:31:
end byte index 6 is not a char boundary; it is inside '\u{b6c46}' (bytes 4..8 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;femtovg::color::Color&gt; (femtovg/src/color.rs:75:31)
  repro_femtovg_src_color_rs_75_31_8dad5a::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Color::hex(",Z\u{ccbfb}");</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/color.rs:79:60:
end byte index 4 is not a char boundary; it is inside '\u{ccbfb}' (bytes 2..6 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;femtovg::color::Color&gt; (femtovg/src/color.rs:79:60)
  repro_femtovg_src_color_rs_79_60_9cec87::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Color::hex("\u{74c6f}3\"W&amp;");</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/color.rs:73:31:
end byte index 2 is not a char boundary; it is inside '\u{74c6f}' (bytes 0..4 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;femtovg::color::Color&gt; (femtovg/src/color.rs:73:31)
  repro_femtovg_src_color_rs_73_31_2fc122::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut atlas = Atlas::new(10000usize, usize::MAX);
let _ = atlas.add_rect(2usize, 2usize);
let _ = atlas.add_rect(1usize, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/text/atlas.rs:156:16:
attempt to add with overflow
backtrace:
  &lt;femtovg::text::atlas::Atlas&gt;::rect_fits (femtovg/src/text/atlas.rs:156:16)
  &lt;femtovg::text::atlas::Atlas&gt;::add_rect (femtovg/src/text/atlas.rs:62:35)
  repro_femtovg_src_text_atlas_rs_156_16_1c37c4::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut atlas = Atlas::new(1usize, 0usize);
atlas.expand(usize::MAX, 255usize);
let _ = atlas.add_rect(usize::MAX, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/text/atlas.rs:143:12:
attempt to add with overflow
backtrace:
  &lt;femtovg::text::atlas::Atlas&gt;::rect_fits (femtovg/src/text/atlas.rs:143:12)
  &lt;femtovg::text::atlas::Atlas&gt;::add_rect (femtovg/src/text/atlas.rs:62:35)
  repro_femtovg_src_text_atlas_rs_143_12_78fca0::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut atlas = Atlas::new(0usize, 0usize);
atlas.expand(usize::MAX, 10000000000000000000usize);
let _ = atlas.add_rect(17390261748156048065usize, 65535usize);</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/text/atlas.rs:125:17:
attempt to add with overflow
backtrace:
  &lt;femtovg::text::atlas::Atlas&gt;::add_skyline_level (femtovg/src/text/atlas.rs:125:17)
  &lt;femtovg::text::atlas::Atlas&gt;::add_rect (femtovg/src/text/atlas.rs:75:18)
  repro_femtovg_src_text_atlas_rs_125_17_5c6f47::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::new();
let font_metrics = FontMetrics::default();
let f32_v = font_metrics.height();
path.svg_arc_to(1.0f32, f32::MIN, 1.0f32, false, false, f32_v, 1.0f32);
let _ = path.dashed(&amp;[1.0f32], 0.0f32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  femtovg::path::Path::append (src/path.rs:875)
  femtovg::path::Path::move_to (src/path.rs:269)
  femtovg::path::dash_line_segment (src/path.rs:1067)
  femtovg::path::Path::dashed_with_tolerance (src/path.rs:200)
  femtovg::path::Path::dashed (src/path.rs:180)
  repro_out_of_memory_in_femtovg_path_dashed_3fa47c::main (src/main.rs:9)
  (21 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::default();
path.rounded_rect(f32::MIN, 0.0f32, 0.0f32, 0.0f32, 1.0f32);
let path2 = path.dashed(&amp;[1.0f32], 0.0f32);
let _ = path2.verbs();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  femtovg::path::Path::append (src/path.rs:875)
  femtovg::path::Path::move_to (src/path.rs:269)
  femtovg::path::dash_line_segment (src/path.rs:1067)
  femtovg::path::Path::dashed_with_tolerance (src/path.rs:200)
  femtovg::path::Path::dashed (src/path.rs:180)
  repro_out_of_memory_in_femtovg_path_verbs_a69f8a::main (src/main.rs:6)
  (21 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut atlas = Atlas::new(1usize, 1000000000000000000usize);
let _ = atlas.add_rect(1usize, 1usize);
let _ = atlas.add_rect(0usize, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/text/atlas.rs:63:20:
attempt to add with overflow
backtrace:
  &lt;femtovg::text::atlas::Atlas&gt;::add_rect (femtovg/src/text/atlas.rs:63:20)
  repro_femtovg_src_text_atlas_rs_63_20_4fb373::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut atlas = Atlas::new(1000000000000000000usize, 0usize);
atlas.expand(10000000000000000000usize, 1usize);
let _ = atlas.add_rect(10000000000000000000usize, 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/text/atlas.rs:102:33:
attempt to subtract with overflow
backtrace:
  &lt;femtovg::text::atlas::Atlas&gt;::add_skyline_level (femtovg/src/text/atlas.rs:102:33)
  &lt;femtovg::text::atlas::Atlas&gt;::add_rect (femtovg/src/text/atlas.rs:75:18)
  repro_femtovg_src_text_atlas_rs_102_33_409afe::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let f32_v = Paint::FONT_WEIGHT_EXTRA_BOLD;
let mut path = Path::default();
path.arc(0.0f32, f32_v, 10000000000.0f32, 0.0f32, 0.5f32, Solidity::default());
let path2 = path.dashed(&amp;[f32_v], f32_v);
let mut path3 = path2.dashed(&amp;[], 0.0f32);
path3.move_to(f32_v, 0.0f32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1156866432 bytes failed
backtrace:
  femtovg::path::Path::append (src/path.rs:875)
  femtovg::path::Path::move_to (src/path.rs:269)
  repro_out_of_memory_in_femtovg_path_move_to_4d6318::main (src/main.rs:11)
  (21 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Solidity::default();
let _ = Void;
let mut path = Path::new();
path.ellipse(-1.0f32, 0.5f32, 0.0f32, 0.121f32);
let _ = Path::new();
let _ = CompositeOperationState::with_blend_factors(BlendFactor::One,
    BlendFactor::Zero);
let _ = Atlas::new(11081732678320183081usize, 13980417984119302665usize);
let mut canvas = &lt;femtovg::Canvas&lt;femtovg::renderer::Void&gt;&gt;::new(Void).unwrap();
canvas.stroke_path(&amp;path, &amp;Paint::linear_gradient(9.282809e-22f32,
    5.801334e-35f32, 520.0f32, 0.049f32, Color::hex("\r\n\t").premultiplied(),
    Color::rgbf(-1.1754944e-38f32, 956.0f32,
    0.9999999f32)).with_line_dash(&amp;[1.1920929e-7f32]));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  femtovg::path::DashCursor::skip_empty_entries (src/path.rs:934)
  femtovg::path::DashCursor::advance (src/path.rs:925)
  femtovg::path::dash_line_segment (src/path.rs:1059)
  femtovg::path::Path::dashed_with_tolerance (src/path.rs:200)
  femtovg::Canvas&lt;femtovg::renderer::void::Void&gt;::stroke_path_internal&lt;femtovg::renderer::void::Void&gt; (femtovg/src/lib.rs:1250)
  femtovg::Canvas&lt;femtovg::renderer::void::Void&gt;::stroke_path&lt;femtovg::renderer::void::Void&gt; (femtovg/src/lib.rs:1236)
  repro_timeout_in_femtovg_canvas_femtovg_renderer_void_stroke_path_892ef3::main (src/main.rs:19)</pre></td></tr>
<tr><td valign="top"><pre>let paint = Paint::elliptical_gradient_stops(0.0f32, 0.0f32, 0.0f32, 0.0f32,
    0.0f32, 0.0f32, vec![]);
let paint2 = paint.clone().with_font_weight(0.0f32);
let paint3 = paint2.clone().with_font_variation(&amp;[1u8, 0u8, 0u8, 0u8], 0.0f32);
let paint4 = paint3.clone().with_font_italic(false);
let _ = paint4.clone().with_font_variation(&amp;[0u8, 0u8, 0u8, 1u8], 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at femtovg/src/paint.rs:62:9:
FontVariations: too many axes (max 4)
backtrace:
  &lt;femtovg::paint::FontVariations&gt;::set (femtovg/src/paint.rs:62:9)
  &lt;femtovg::paint::Paint&gt;::set_font_variation (femtovg/src/paint.rs:1569:35)
  &lt;femtovg::paint::Paint&gt;::with_font_variation (femtovg/src/paint.rs:1577:14)
  repro_femtovg_src_paint_rs_62_9_971bca::main (src/main.rs:8:28)</pre></td></tr>
<tr><td valign="top"><pre>let mut void = Void;
let _ = &lt;femtovg::renderer::Void as femtovg::Renderer&gt;::screenshot(&amp;mut void);</pre></td><td valign="top"><pre>thread 'main' panicked at imgref/src/lib.rs:679:9:
Invalid ImgRef params. Got stride=0 for 0×0; len=0 (needed 0)
backtrace:
  imgref::imgref_invalid_size (imgref/src/lib.rs:687:5)
  &lt;imgref::Img&lt;alloc::vec::Vec&lt;rgb::formats::rgba::Rgba&lt;u8&gt;&gt;&gt;&gt;::new_stride (imgref/src/lib.rs:654:13)
  &lt;imgref::Img&lt;alloc::vec::Vec&lt;rgb::formats::rgba::Rgba&lt;u8&gt;&gt;&gt;&gt;::new (imgref/src/lib.rs:679:9)
  &lt;femtovg::renderer::void::Void as femtovg::renderer::Renderer&gt;::screenshot (femtovg/src/renderer/void.rs:65:12)
  repro_imgref_src_lib_rs_679_9_6e2bf6::main (src/main.rs:5:13)</pre></td></tr>
</table>
