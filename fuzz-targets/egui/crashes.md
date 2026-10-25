# egui - crashes

Source: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let t_s_transform = TSTransform { scaling: f32::NAN,
    translation: WHITE_UV.to_vec2() };
assert!(t_s_transform == t_s_transform, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:6:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_emath_tstransform_49ca91::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Shape::dashed_line_with_offset(&amp;[], Visuals::default().window_stroke(),
    &amp;[], &amp;[0.0f32], 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/shapes/shape.rs:561:5:
assertion `left == right` failed: Mismatched dash and gap lengths, got dash_lengths: 0, gap_lengths: 1
  left: 0
 right: 1
backtrace:
  epaint::shapes::shape::dashes_from_line (epaint/src/shapes/shape.rs:561:5)
  &lt;epaint::shapes::shape::Shape&gt;::dashed_line_with_offset::&lt;epaint::stroke::Stroke&gt; (epaint/src/shapes/shape.rs:205:9)
  repro_epaint_src_shapes_shape_rs_561_5_99590e::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rgba::from_black_alpha(-1.1754944e-38f32);</pre></td><td valign="top"><pre>thread 'main' panicked at ecolor/src/rgba.rs:106:9:
a should be in the range [0, 1], but was -0.000000000000000000000000000000000000011754944
backtrace:
  &lt;ecolor::rgba::Rgba&gt;::from_black_alpha (ecolor/src/rgba.rs:106:9)
  repro_ecolor_src_rgba_rs_106_9_dcbb53::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::default().region_by_pixels([0usize, 0usize], [1usize,
    0usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:274:9:
x + w should be &lt;= self.width(), but x: 0, w: 1, width: 0
backtrace:
  &lt;epaint::image::ColorImage&gt;::region_by_pixels (epaint/src/image.rs:274:9)
  repro_epaint_src_image_rs_274_9_a824a3::main (src/main.rs:4:35)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Fonts::new(TextOptions { max_texture_side: 0usize,
    color_transfer_function: FontColorTransferFunction::Gamma(0.0f32),
    font_hinting: false, subpixel_binning: false }, FontDefinitions::empty());</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/texture_atlas.rs:83:9:
Tiny texture atlas
backtrace:
  &lt;epaint::texture_atlas::TextureAtlas&gt;::new (epaint/src/texture_atlas.rs:83:9)
  &lt;epaint::text::glyph_atlas::GlyphAtlas&gt;::new (epaint/src/text/glyph_atlas.rs:224:20)
  &lt;epaint::text::fonts::FontsImpl&gt;::new (epaint/src/text/fonts.rs:514:21)
  &lt;epaint::text::fonts::Fonts&gt;::new (epaint/src/text/fonts.rs:96:20)
  repro_epaint_src_texture_atlas_rs_83_9_9ab63b::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let _ = context.run_ui(RawInput::default(), |_| ());</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/textures.rs:337:9:
Dropped TexturesDelta with 1 unapplied deltas. Deltas need to be handled. If you want to drop this intentionally call `clear` before dropping.
backtrace:
  &lt;epaint::textures::TexturesDelta as core::ops::drop::Drop&gt;::drop (epaint/src/textures.rs:337:9)
  repro_epaint_src_textures_rs_337_9_69210d::main (src/main.rs:6:56)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::from_rgba_premultiplied([0usize, 0usize], &amp;vec![118u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:129:9:
assertion `left == right` failed: size: [0, 0], rgba.len(): 1
  left: 0
 right: 1
backtrace:
  &lt;epaint::image::ColorImage&gt;::from_rgba_premultiplied (epaint/src/image.rs:129:9)
  repro_epaint_src_image_rs_129_9_5e5093::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Fonts::new(TextOptions::default(),
    FontDefinitions::empty()).layout_uncached(35184372000000.0f32,
    std::sync::Arc::new(egui_extras::syntax_highlighting::highlight(
    &amp;Context::default(), &amp;Theme::Light.default_style(),
    &amp;CodeTheme::from_memory(&amp;Context::default(), &amp;Style::default()), "Z", "")));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:79:33:
attempt to multiply with overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::filled (epaint/src/image.rs:79:33)
  epaint::text::fonts::synthetic_tofu (epaint/src/text/fonts.rs:870:21)
  &lt;epaint::text::fonts::FontsImpl&gt;::new::{closure#0} (epaint/src/text/fonts.rs:524:61)
  &lt;epaint::text::glyph_atlas::GlyphAtlas&gt;::allocate_raster::&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt;::{closure#1} (epaint/src/text/glyph_atlas.rs:352:36)
  &lt;epaint::text::glyph_atlas::GlyphAtlas&gt;::allocate_raster::&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (epaint/src/text/glyph_atlas.rs:352:14)
  &lt;epaint::text::fonts::FontsImpl&gt;::fontless_cluster (epaint/src/text/fonts.rs:837:14)
  epaint::text::text_layout::segment_into_runs (epaint/src/text/text_layout.rs:1538:32)
  epaint::text::text_layout::layout_section (epaint/src/text/text_layout.rs:572:9)
  epaint::text::text_layout::layout (epaint/src/text/text_layout.rs:116:28)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached_for_viewport (epaint/src/text/fonts.rs:291:9)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached (epaint/src/text/fonts.rs:277:14)
  repro_epaint_src_image_rs_79_33_94536b::main (src/main.rs:10:74)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::from_rgba_premultiplied([usize::MAX, 16usize],
    &amp;include_bytes!("image_0.gif").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:130:13:
attempt to multiply with overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::from_rgba_premultiplied (epaint/src/image.rs:130:13)
  repro_epaint_src_image_rs_130_13_080c07::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;epaint::text::ByteIndex as ::core::ops::Sub&lt;epaint::text::ByteIndex&gt;&gt;::
    sub(ByteIndex(0usize), ByteIndex(1usize));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/index.rs:133:1:
attempt to subtract with overflow
backtrace:
  &lt;epaint::text::index::ByteIndex as core::ops::arith::Sub&gt;::sub (epaint/src/text/index.rs:99:22)
  repro_epaint_src_text_index_rs_133_1_fda37e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Color32::from_rgba_unmultiplied(0u8, 0u8, 0u8,
    0u8).linear_multiply(f32::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at ecolor/src/color32.rs:306:9:
factor should be finite, but was -340282350000000000000000000000000000000
backtrace:
  &lt;ecolor::color32::Color32&gt;::linear_multiply (ecolor/src/color32.rs:306:9)
  repro_ecolor_src_color32_rs_306_9_47f9ba::main (src/main.rs:4:65)</pre></td></tr>
<tr><td valign="top"><pre>let _ = emath::round_to_decimals(0.0f64, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at emath/src/lib.rs:210:5:
Formatting argument out of range
backtrace:
  emath::round_to_decimals (emath/src/lib.rs:210:5)
  repro_emath_src_lib_rs_210_5_d2e921::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut texture_manager = TextureManager::default();
texture_manager.retain(TextureId::default());</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/textures.rs:92:13:
Tried retaining texture Managed(0) which is not allocated
backtrace:
  &lt;epaint::textures::TextureManager&gt;::retain (epaint/src/textures.rs:92:13)
  repro_epaint_src_textures_rs_92_13_7a0022::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut ui = Ui::new(Context::default(), Id::unique(0u8), UiBuilder::default());
let _ = ui.add(Hyperlink::new("".to_string()));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/context.rs:1387:22:
No fonts available until first call to Context::run()
backtrace:
  &lt;egui::context::Context&gt;::fonts_mut::&lt;alloc::sync::Arc&lt;epaint::text::text_layout_types::Galley&gt;, &lt;egui::widgets::label::Label&gt;::layout_in_ui::{closure#6}&gt;::{closure#0} (egui/src/context.rs:1387:22)
  &lt;egui::context::Context&gt;::write::&lt;alloc::sync::Arc&lt;epaint::text::text_layout_types::Galley&gt;, &lt;egui::context::Context&gt;::fonts_mut&lt;alloc::sync::Arc&lt;epaint::text::text_layout_types::Galley&gt;, &lt;egui::widgets::label::Label&gt;::layout_in_ui::{closure#6}&gt;::{closure#0}&gt; (egui/src/context.rs:920:9)
  &lt;egui::context::Context&gt;::fonts_mut::&lt;alloc::sync::Arc&lt;epaint::text::text_layout_types::Galley&gt;, &lt;egui::widgets::label::Label&gt;::layout_in_ui::{closure#6}&gt; (egui/src/context.rs:1380:14)
  &lt;egui::widgets::label::Label&gt;::layout_in_ui (egui/src/widgets/label.rs:257:29)
  &lt;egui::widgets (egui/src/widgets/hyperlink.rs:42:52)
  &lt;egui::ui::Ui&gt;::add::&lt;egui::widgets (egui/src/ui.rs:1647:16)
  &lt;egui::widgets (egui/src/widgets/hyperlink.rs:129:27)
  &lt;egui::ui::Ui&gt;::add::&lt;egui::widgets (egui/src/ui.rs:1647:16)
  repro_egui_src_context_rs_1387_22_030193::main (src/main.rs:9:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut ui = Ui::new(Context::default(), Id::NULL, UiBuilder::new());
ui.set_max_size(Vec2::from([0.0f32, f32::MIN]));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layout.rs:584:9:
Negative size: [0.0 -340282346638528859811704183484516925440.0]
backtrace:
  &lt;egui::layout::Layout&gt;::next_frame_ignore_wrap (egui/src/layout.rs:584:9)
  &lt;egui::layout::Layout&gt;::next_widget_space_ignore_wrap_justify (egui/src/layout.rs:660:26)
  &lt;egui::placer::Placer&gt;::next_widget_space_ignore_wrap_justify (egui/src/placer.rs:228:14)
  &lt;egui::placer::Placer&gt;::set_max_height (egui/src/placer.rs:249:25)
  &lt;egui::ui::Ui&gt;::set_max_height (egui/src/ui.rs:751:21)
  &lt;egui::ui::Ui&gt;::set_max_size (egui/src/ui.rs:739:14)
  repro_egui_src_layout_rs_584_9_70ec91::main (src/main.rs:9:8)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let _ = context.tessellate(vec![], 1.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/context.rs:3378:22:
No fonts loaded
backtrace:
  &lt;egui::context::Context&gt;::tessellate::{closure#0} (egui/src/context.rs:3378:22)
  &lt;egui::context::Context&gt;::write::&lt;alloc::vec::Vec&lt;epaint::ClippedPrimitive&gt;, &lt;egui::context::Context&gt;::tessellate::{closure#0}&gt; (egui/src/context.rs:920:9)
  &lt;egui::context::Context&gt;::tessellate (egui/src/context.rs:3369:14)
  repro_egui_src_context_rs_3378_22_40f619::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rgba::from_luminance_alpha(104.0f32, 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at ecolor/src/rgba.rs:92:9:
l should be in the range [0, 1], but was 104
backtrace:
  &lt;ecolor::rgba::Rgba&gt;::from_luminance_alpha (ecolor/src/rgba.rs:92:9)
  repro_ecolor_src_rgba_rs_92_9_fb9c10::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut ui = Ui::new(Context::default(), Id::new(0u8), UiBuilder::default());
let _ = ui.allocate_response(Vec2::LEFT, Sense::CLICK);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/placer.rs:114:9:
Negative child size: [-1.0 0.0]
backtrace:
  &lt;egui::placer::Placer&gt;::next_space (egui/src/placer.rs:114:9)
  &lt;egui::ui::Ui&gt;::allocate_space_impl (egui/src/ui.rs:1366:38)
  &lt;egui::ui::Ui&gt;::allocate_space (egui/src/ui.rs:1317:25)
  &lt;egui::ui::Ui&gt;::allocate_response (egui/src/ui.rs:1265:31)
  repro_egui_src_placer_rs_114_9_93b9e3::main (src/main.rs:10:16)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rgba::from_white_alpha(1.0000001f32);</pre></td><td valign="top"><pre>thread 'main' panicked at ecolor/src/rgba.rs:116:9:
a: 1.0000001
backtrace:
  &lt;ecolor::rgba::Rgba&gt;::from_white_alpha (ecolor/src/rgba.rs:116:9)
  repro_ecolor_src_rgba_rs_116_9_ca2c56::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::new([1000000000usize, 10000000000000000000usize],
    vec![FontColorTransferFunction::
    TwoCoverageMinusCoverageSq.color_from_coverage(65536.0f32),
    Color32::DARK_RED, Color32::from_rgba_unmultiplied(255u8, 0u8, 8u8,
    162u8)]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:63:13:
attempt to multiply with overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::new (epaint/src/image.rs:63:13)
  repro_epaint_src_image_rs_63_13_8c9861::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::default();
path.add_line_loop(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:434:9:
A path needs at least two points, but got 0
backtrace:
  &lt;epaint::tessellator::Path&gt;::add_line_loop (epaint/src/tessellator.rs:434:9)
  repro_epaint_src_tessellator_rs_434_9_510afd::main (src/main.rs:5:10)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;epaint::text::CharIndex as ::core::ops::Sub&lt;usize&gt;&gt;::sub(
    CharIndex::default(), 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/index.rs:134:1:
attempt to subtract with overflow
backtrace:
  &lt;epaint::text::index::CharIndex as core::ops::arith::Sub&lt;usize&gt;&gt;::sub (epaint/src/text/index.rs:90:22)
  repro_epaint_src_text_index_rs_134_1_407509::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let color32 = Color32::BROWN;
let _ = color32.gamma_multiply(-6.629608e-7f32);</pre></td><td valign="top"><pre>thread 'main' panicked at ecolor/src/color32.rs:270:9:
factor should be finite, but was -0.0000006629608
backtrace:
  &lt;ecolor::color32::Color32&gt;::gamma_multiply (ecolor/src/color32.rs:270:9)
  repro_ecolor_src_color32_rs_270_9_8115a4::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::new([0usize, 0usize], vec![Color32::YELLOW]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:62:9:
size: [0, 0], pixels.len(): 1
backtrace:
  &lt;epaint::image::ColorImage&gt;::new (epaint/src/image.rs:62:9)
  repro_epaint_src_image_rs_62_9_ab9e1a::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let themes = Themes::default();
let _ = themes.get::&lt;egui::widget_style::SeparatorStyle&gt;();</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/theme/themes.rs:51:13:
A style should be set for "egui::widget_style::SeparatorStyle"
backtrace:
  &lt;egui::theme::themes::Themes&gt;::get::&lt;egui::widget_style::SeparatorStyle&gt;::{closure#0} (egui/src/theme/themes.rs:51:13)
  &lt;egui::theme::themes::Themes&gt;::get::&lt;egui::widget_style::SeparatorStyle&gt; (egui/src/theme/themes.rs:50:11)
  repro_egui_src_theme_themes_rs_51_13_ba3640::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = SizedTexture::from_handle(&amp;TextureHandle::new(std::sync::Arc::new(
    &lt;epaint::mutex::RwLock&lt;epaint::TextureManager&gt; as ::core::default::
    Default&gt;::default()), TextureId::default()));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/textures.rs:81:13:
Tried freeing texture Managed(0) which is not allocated
backtrace:
  &lt;epaint::textures::TextureManager&gt;::free (epaint/src/textures.rs:81:13)
  &lt;epaint::texture_handle::TextureHandle as core::ops::drop::Drop&gt;::drop (epaint/src/texture_handle.rs:27:31)
  repro_epaint_src_textures_rs_81_13_ba5b52::main (src/main.rs:6:190)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let _ = context.cumulative_frame_nr_for(ViewportId::from_hash_of(0u8));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/context.rs:2073:21:
cumulative_frame_nr_for failed to find the viewport "B5D0"
backtrace:
  &lt;egui::context::Context&gt;::cumulative_frame_nr_for::{closure#0}::{closure#1} (egui/src/context.rs:2073:21)
  &lt;egui::context::Context&gt;::cumulative_frame_nr_for::{closure#0} (egui/src/context.rs:2072:18)
  &lt;egui::context::Context&gt;::read::&lt;u64, &lt;egui::context::Context&gt;::cumulative_frame_nr_for::{closure#0}&gt; (egui/src/context.rs:915:9)
  &lt;egui::context::Context&gt;::cumulative_frame_nr_for (egui/src/context.rs:2068:14)
  repro_egui_src_context_rs_2073_21_939f3c::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = emath::format_with_decimals_in_range(0.0f64, 1usize..=0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at emath/src/lib.rs:223:5:
min_decimals should be &lt;= max_decimals, but got min_decimals: 1, max_decimals: 0
backtrace:
  emath::format_with_decimals_in_range (emath/src/lib.rs:223:5)
  repro_emath_src_lib_rs_223_5_12ff3c::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let texture_meta = TextureMeta { name: "".to_string(), size: [1000000000usize,
    1000000000usize], bytes_per_pixel: 1000usize, retain_count: 0usize,
    options: TextureOptions::NEAREST_MIRRORED_REPEAT };
let _ = texture_meta.bytes_used();</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/textures.rs:151:9:
attempt to multiply with overflow
backtrace:
  &lt;epaint::textures::TextureMeta&gt;::bytes_used (epaint/src/textures.rs:151:9)
  repro_epaint_src_textures_rs_151_9_822aeb::main (src/main.rs:6:26)</pre></td></tr>
<tr><td valign="top"><pre>let mut variation_coords = VariationCoords::default();
variation_coords.push([0u8, 0u8, 0u8, 0u8], 1.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/text_layout_types.rs:396:45:
Invalid variation axis tag: InvalidByte { pos: 0, byte: 0 }
backtrace:
  &lt;[u8; 4] as epaint::text::text_layout_types::IntoTag&gt;::into_tag (epaint/src/text/text_layout_types.rs:396:45)
  &lt;epaint::text::text_layout_types::VariationCoords&gt;::push::&lt;[u8; 4]&gt; (epaint/src/text/text_layout_types.rs:439:26)
  repro_epaint_src_text_text_layout_types_rs_396_45_a7401c::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Ui::new(Context::default(), Id::unique("".to_string()),
    UiBuilder::new().id(egui::accesskit_root_id()));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/ui.rs:114:9:
Top-level Ui:s should not have an UiBuilder::id_source
backtrace:
  &lt;egui::ui::Ui&gt;::new (egui/src/ui.rs:114:9)
  repro_egui_src_ui_rs_114_9_21860d::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let harness = &lt;egui_kittest::Harness&lt;u32&gt;&gt;::new_ui_state(|_, _| (), 0u32);
let mut ui = Ui::new(Context::default(), harness.ui_id(), UiBuilder::new());
let _ = ui.put(Rect::everything_below(1.0f32), Hyperlink::new("".to_string()));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layout.rs:662:9:
rect is NaN: [[NaN inf] - [NaN inf]]
backtrace:
  &lt;egui::layout::Layout&gt;::next_widget_space_ignore_wrap_justify (egui/src/layout.rs:662:9)
  &lt;egui::layout::Layout&gt;::next_widget_position (egui/src/layout.rs:669:14)
  &lt;egui::layout::Layout&gt;::region_from_max_rect (egui/src/layout.rs:408:25)
  &lt;egui::placer::Placer&gt;::new (egui/src/placer.rs:16:29)
  &lt;egui::ui::Ui&gt;::new_child (egui/src/ui.rs:267:22)
  &lt;egui::ui::Ui&gt;::scope_dyn::&lt;egui::response::Response&gt; (egui/src/ui.rs:2334:33)
  &lt;egui::ui::Ui&gt;::scope_builder::&lt;egui::response::Response, &lt;egui::ui::Ui&gt;::put&lt;egui::widgets (egui/src/ui.rs:2324:14)
  &lt;egui::ui::Ui&gt;::put::&lt;egui::widgets (egui/src/ui.rs:1692:14)
  repro_egui_src_layout_rs_662_9_86d5cf::main (src/main.rs:10:16)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Fonts::new(TextOptions { max_texture_side: 1000000000usize,
    color_transfer_function: FontColorTransferFunction::DARK_MODE_DEFAULT,
    font_hinting: false, subpixel_binning: false },
    FontDefinitions::default()).layout_uncached(0.0f32,
    std::sync::Arc::new(LayoutJob { text: "a".to_string(), sections: vec![],
    wrap: TextWrapping { max_width: 0.0f32, max_rows: 0usize,
    break_anywhere: false, overflow_character: None },
    first_row_min_height: 0.0f32, break_on_newline: false, halign: Align::Min,
    justify: false, round_output_to_gui: false,
    keep_trailing_whitespace: false }));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/text_layout_types.rs:247:17:
LayoutJob has text but no sections: "a"
backtrace:
  &lt;epaint::text::text_layout_types::LayoutJob&gt;::debug_sanity_check (epaint/src/text/text_layout_types.rs:247:17)
  epaint::text::text_layout::layout (epaint/src/text/text_layout.rs:93:9)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached_for_viewport (epaint/src/text/fonts.rs:291:9)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached (epaint/src/text/fonts.rs:277:14)
  repro_epaint_src_text_text_layout_types_rs_247_17_0f1d3a::main (src/main.rs:10:220)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let painter = context.debug_painter();
let f32_v = painter.opacity();
let rot2 = &lt;emath::Rot2 as ::core::ops::Mul&lt;f32&gt;&gt;::mul(
    &lt;emath::Rot2 as ::core::ops::Div&lt;f32&gt;&gt;::div(Rot2::IDENTITY, 0.0f32), f32_v);
let rot22 = rot2.inverse();
let rot23 = rot22.inverse();
let _ = rot23.normalized();</pre></td><td valign="top"><pre>thread 'main' panicked at emath/src/rot2.rs:87:9:
Rot2::normalized produced a non-finite result
backtrace:
  &lt;emath::rot2::Rot2&gt;::normalized (emath/src/rot2.rs:87:9)
  repro_emath_src_rot2_rs_87_9_259a9c::main (src/main.rs:11:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TextureManager::default().alloc("".to_string(),
    ImageData::from(ColorImage::filled([10000000000000000000usize, 1usize],
    Color32::from_rgb_additive(7u8, 192u8, 248u8))), TextureOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:14:21)
capacity overflow
backtrace:
  &lt;ecolor::color32::Color32 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:14:21)
  &lt;epaint::image::ColorImage&gt;::filled (epaint/src/image.rs:79:21)
  repro_epaint_src_image_rs_79_21_e8ce28::main (src/main.rs:8:77)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::default();
path.reserve(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from epaint/src/tessellator.rs:337:16)
capacity overflow
backtrace:
  &lt;epaint::tessellator::Path&gt;::reserve (epaint/src/tessellator.rs:337:16)
  repro_epaint_src_tessellator_rs_337_16_cc7360::main (src/main.rs:5:10)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Fonts::new(TextOptions::default(),
    FontDefinitions::default()).layout_uncached_for_viewport(1.0f32,
    ViewportKey::default(),
    std::sync::Arc::new(LayoutJob::simple_singleline("a".to_string(),
    FontId::monospace(0.0f32), Color32::ORANGE)));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/glyph_atlas.rs:161:9:
Bad px_scale_factor: 0
backtrace:
  &lt;epaint::text::glyph_atlas::OutlineGlyphKey&gt;::new (epaint/src/text/glyph_atlas.rs:161:9)
  &lt;epaint::text::glyph_atlas::GlyphAtlas&gt;::allocate_outline (epaint/src/text/glyph_atlas.rs:297:19)
  &lt;epaint::text::fonts::FontsImpl&gt;::allocate_glyph (epaint/src/text/fonts.rs:753:14)
  epaint::text::text_layout::layout_shaped_run (epaint/src/text/text_layout.rs:367:23)
  epaint::text::text_layout::layout_section (epaint/src/text/text_layout.rs:603:13)
  epaint::text::text_layout::layout (epaint/src/text/text_layout.rs:116:28)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached_for_viewport (epaint/src/text/fonts.rs:291:9)
  repro_epaint_src_text_glyph_atlas_rs_161_9_ed6fc0::main (src/main.rs:10:76)</pre></td></tr>
<tr><td valign="top"><pre>let paint_callback_info = PaintCallbackInfo { viewport: Rect::
    everything_right_of(0.0f32), clip_rect: Rect::ZERO,
    pixels_per_point: 0.0f32, screen_size_px: [u32::MAX, 0u32] };
let _ = paint_callback_info.clip_rect_in_pixels();</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/viewport.rs:38:31:
min &gt; max. min = 0, max = -1
backtrace:
  &lt;i32 as core::cmp::Ord&gt;::clamp::do_panic::runtime (core/src/panic.rs:172:21)
  &lt;i32 as core::cmp::Ord&gt;::clamp::do_panic (core/src/intrinsics/mod.rs:2636:9)
  &lt;i32 as core::cmp::Ord&gt;::clamp (core/src/panic.rs:177:9)
  &lt;epaint::viewport::ViewportInPixels&gt;::from_points (epaint/src/viewport.rs:38:31)
  &lt;epaint::shapes::paint_callback::PaintCallbackInfo&gt;::clip_rect_in_pixels (epaint/src/shapes/paint_callback.rs:52:9)
  repro_epaint_src_viewport_rs_38_31_a9e982::main (src/main.rs:6:33)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Ui::new(Context::default(), Id::new("".to_string()),
    UiBuilder::default().max_rect(Rect::NAN));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layout.rs:402:9:
max_rect is not NaN: [[NaN NaN] - [NaN NaN]]
backtrace:
  &lt;egui::layout::Layout&gt;::region_from_max_rect (egui/src/layout.rs:402:9)
  &lt;egui::placer::Placer&gt;::new (egui/src/placer.rs:16:29)
  &lt;egui::ui::Ui&gt;::new (egui/src/ui.rs:131:22)
  repro_egui_src_layout_rs_402_9_f27035::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::default().region_by_pixels([1usize, 0usize], [usize::MAX,
    0usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:275:13:
attempt to add with overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::region_by_pixels (epaint/src/image.rs:275:13)
  repro_epaint_src_image_rs_275_13_181943::main (src/main.rs:4:35)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let harness = &lt;egui_kittest::Harness&lt;u8&gt;&gt;::new_ui_state(|_, _| (), 0u8);
let mut ui = Ui::new(context.clone(), harness.ui_id(), UiBuilder::new());
let mut ui2 = ui.new_child(UiBuilder::new());
let mut font_id = FontId::new(0.0f32, FontFamily::Monospace);
egui::introspection::font_id_ui(&amp;mut ui2, &amp;mut font_id);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/context.rs:1365:22:
No fonts available until first call to Context::run()
backtrace:
  &lt;egui::context::Context&gt;::fonts::&lt;alloc::vec::Vec&lt;epaint::text::font_id::FontFamily&gt;, egui::introspection::font_id_ui::{closure#0}&gt;::{closure#0} (egui/src/context.rs:1365:22)
  &lt;egui::context::Context&gt;::write::&lt;alloc::vec::Vec&lt;epaint::text::font_id::FontFamily&gt;, &lt;egui::context::Context&gt;::fonts&lt;alloc::vec::Vec&lt;epaint::text::font_id::FontFamily&gt;, egui::introspection::font_id_ui::{closure#0}&gt;::{closure#0}&gt; (egui/src/context.rs:920:9)
  &lt;egui::context::Context&gt;::fonts::&lt;alloc::vec::Vec&lt;epaint::text::font_id::FontFamily&gt;, egui::introspection::font_id_ui::{closure#0}&gt; (egui/src/context.rs:1359:14)
  egui::introspection::font_id_ui (egui/src/introspection.rs:18:23)
  repro_egui_src_context_rs_1365_22_f4c9b0::main (src/main.rs:13:5)</pre></td></tr>
<tr><td valign="top"><pre>let alloc_info = AllocInfo::from_galley(&amp;Fonts::new(
    TextOptions { max_texture_side: 1000000000usize,
    color_transfer_function: FontColorTransferFunction::default(),
    font_hinting: false, subpixel_binning: false },
    FontDefinitions::empty()).layout_uncached(1.0f32,
    std::sync::Arc::new(egui_extras::syntax_highlighting::highlight(
    &amp;Context::default(), &amp;Style::default(),
    &amp;CodeTheme::from_style(&amp;Style::default()), "", ""))));
let _ = alloc_info.num_elements();</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/stats.rs:109:9:
Heterogenous element size
backtrace:
  &lt;epaint::stats::AllocInfo&gt;::num_elements (epaint/src/stats.rs:109:9)
  repro_epaint_src_stats_rs_109_9_b7ebe7::main (src/main.rs:12:24)</pre></td></tr>
<tr><td valign="top"><pre>let mut fonts = Fonts::new(
    TextOptions { max_texture_side: 10000000000000000usize,
    color_transfer_function: FontColorTransferFunction::DARK_MODE_DEFAULT,
    font_hinting: false, subpixel_binning: false }, FontDefinitions::default());
let _ = fonts.layout_uncached(0.0f32,
    std::sync::Arc::new(LayoutJob::single_section("a".to_string(),
    TextFormat::simple(TextStyle::Heading.resolve(&amp;Style::default()),
    Color32::LIGHT_RED))));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/glyph_atlas.rs:157:9:
Bad pixels_per_point 0
backtrace:
  &lt;epaint::text::glyph_atlas::OutlineGlyphKey&gt;::new (epaint/src/text/glyph_atlas.rs:157:9)
  &lt;epaint::text::glyph_atlas::GlyphAtlas&gt;::allocate_outline (epaint/src/text/glyph_atlas.rs:297:19)
  &lt;epaint::text::fonts::FontsImpl&gt;::allocate_glyph (epaint/src/text/fonts.rs:753:14)
  epaint::text::text_layout::layout_shaped_run (epaint/src/text/text_layout.rs:367:23)
  epaint::text::text_layout::layout_section (epaint/src/text/text_layout.rs:603:13)
  epaint::text::text_layout::layout (epaint/src/text/text_layout.rs:116:28)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached_for_viewport (epaint/src/text/fonts.rs:291:9)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached (epaint/src/text/fonts.rs:277:14)
  repro_epaint_src_text_glyph_atlas_rs_157_9_ef0686::main (src/main.rs:13:19)</pre></td></tr>
<tr><td valign="top"><pre>let drag_value = DragValue::from_get_set(|_| 1 as f64);
let _ = drag_value.binary(0usize, false);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/widgets/value_format.rs:151:9:
ValueFormat::binary: `min_width` must be greater than 0
backtrace:
  &lt;egui::widgets::value_format::ValueFormat&gt;::binary (egui/src/widgets/value_format.rs:151:9)
  &lt;egui::widgets::drag_value::DragValue&gt;::binary (egui/src/widgets/drag_value.rs:342:35)
  repro_egui_src_widgets_value_format_rs_151_9_8af667::main (src/main.rs:5:24)</pre></td></tr>
<tr><td valign="top"><pre>let drag_value = DragValue::from_get_set(|_| 0 as f64);
let drag_value2 = drag_value.suffix(());
let _ = drag_value2.octal(0usize, false);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/widgets/value_format.rs:179:9:
ValueFormat::octal: `min_width` must be greater than 0
backtrace:
  &lt;egui::widgets::value_format::ValueFormat&gt;::octal (egui/src/widgets/value_format.rs:179:9)
  &lt;egui::widgets::drag_value::DragValue&gt;::octal (egui/src/widgets/drag_value.rs:356:35)
  repro_egui_src_widgets_value_format_rs_179_9_18d7a5::main (src/main.rs:6:25)</pre></td></tr>
<tr><td valign="top"><pre>let paint_callback_info = PaintCallbackInfo { viewport: Rect::everything_above(
    0.0f32), clip_rect: Context::default().globally_used_rect(),
    pixels_per_point: 0.0f32, screen_size_px: [0u32, u32::MAX] };
let _ = paint_callback_info.clip_rect_in_pixels();</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/viewport.rs:40:29:
min &gt; max. min = 0, max = -1
backtrace:
  &lt;i32 as core::cmp::Ord&gt;::clamp::do_panic::runtime (core/src/panic.rs:172:21)
  &lt;i32 as core::cmp::Ord&gt;::clamp::do_panic (core/src/intrinsics/mod.rs:2636:9)
  &lt;i32 as core::cmp::Ord&gt;::clamp (core/src/panic.rs:177:9)
  &lt;epaint::viewport::ViewportInPixels&gt;::from_points (epaint/src/viewport.rs:40:29)
  &lt;epaint::shapes::paint_callback::PaintCallbackInfo&gt;::clip_rect_in_pixels (epaint/src/shapes/paint_callback.rs:52:9)
  repro_epaint_src_viewport_rs_40_29_a8de5b::main (src/main.rs:7:33)</pre></td></tr>
<tr><td valign="top"><pre>let rangef = &lt;emath::Rangef as ::core::ops::Mul&lt;f32&gt;&gt;::mul(Rangef::new(0.0f32,
    1.0f32), -0.00024414063f32);
let _ = rangef.clamp(0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/num/f32.rs:1605:9:
min &gt; max, or either was NaN. min = -0.0, max = -0.00024414063
backtrace:
  &lt;f32&gt;::clamp::do_panic::runtime (core/src/panic.rs:172:21)
  &lt;f32&gt;::clamp::do_panic (core/src/intrinsics/mod.rs:2636:9)
  &lt;f32&gt;::clamp (core/src/panic.rs:177:9)
  &lt;emath::range::Rangef&gt;::clamp (emath/src/range.rs:70:11)
  repro_emath_src_range_rs_70_11_c850ef::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = emath::format_with_decimals_in_range(0.0f64,
    0usize..=10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at emath/src/lib.rs:227:5:
max_decimals should be &lt; 100, but got 10000000000000000000
backtrace:
  emath::format_with_decimals_in_range (emath/src/lib.rs:227:5)
  repro_emath_src_lib_rs_227_5_ed1dd6::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>epaint::tessellator::path::add_circle_quadrant(&amp;mut vec![], Pos2::NAN,
    1000.0f32, 1000.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:648:48:
range start index 32000 out of range for slice of length 129
backtrace:
  &lt;[emath::vec2::Vec2] as core::ops::index::Index&lt;core::ops::range::RangeInclusive&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;[emath::vec2::Vec2; 129] as core::ops::index::Index&lt;core::ops::range::RangeInclusive&lt;usize&gt;&gt;&gt;::index (core/src/array/mod.rs:391:9)
  epaint::tessellator::path::add_circle_quadrant (epaint/src/tessellator.rs:648:48)
  repro_epaint_src_tessellator_rs_648_48_e50533::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut texture_atlas = TextureAtlas::new([10000usize, 1usize],
    TextOptions::default());
let _ = texture_atlas.allocate((65536usize, 0usize));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/texture_atlas.rs:226:9:
Tried to allocate a 65536 wide glyph in a 10000 wide texture atlas
backtrace:
  &lt;epaint::texture_atlas::TextureAtlas&gt;::allocate (epaint/src/texture_atlas.rs:226:9)
  repro_epaint_src_texture_atlas_rs_226_9_672381::main (src/main.rs:6:27)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImagePoll::Ready { image: std::sync::Arc::new(ColorImage::filled(
    [1usize, 1000000000usize], Color32::from_rgba_unmultiplied_const(0u8, 1u8,
    0u8, 0u8))) };</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_egui_load_imagepoll_ready_524b5a::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut mesh = Mesh::from(Mesh::default());
mesh.reserve_triangles(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/mesh.rs:187:30:
attempt to multiply with overflow
backtrace:
  &lt;epaint::mesh::Mesh&gt;::reserve_triangles (epaint/src/mesh.rs:187:30)
  repro_epaint_src_mesh_rs_187_30_8b893f::main (src/main.rs:5:10)</pre></td></tr>
<tr><td valign="top"><pre>let mut mesh = Mesh::default();
mesh.reserve_triangles(943768170735567030usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from epaint/src/mesh.rs:187:22)
capacity overflow
backtrace:
  &lt;epaint::mesh::Mesh&gt;::reserve_triangles (epaint/src/mesh.rs:187:22)
  repro_epaint_src_mesh_rs_187_22_819674::main (src/main.rs:5:10)</pre></td></tr>
<tr><td valign="top"><pre>let margin_f32 = MarginF32::ZERO;
let rect = Rect::everything_below(0.0f32);
let _ = rect.intersects_ray(Pos2::ZERO, margin_f32.left_top());</pre></td><td valign="top"><pre>thread 'main' panicked at emath/src/rect.rs:710:9:
Debug assert: expected normalized direction, but `d` has length 0
backtrace:
  &lt;emath::rect::Rect&gt;::intersects_ray (emath/src/rect.rs:710:9)
  repro_emath_src_rect_rs_710_9_803cd5::main (src/main.rs:8:18)</pre></td></tr>
<tr><td valign="top"><pre>let rect = Rect::from_min_max(Pos2 { x: 0.0f32, y: 0.0f32 }, Pos2::ZERO);
let _ = rect.intersects_ray_from_center(Vec2::default());</pre></td><td valign="top"><pre>thread 'main' panicked at emath/src/rect.rs:742:9:
expected normalized direction, but `d` has length 0
backtrace:
  &lt;emath::rect::Rect&gt;::intersects_ray_from_center (emath/src/rect.rs:742:9)
  repro_emath_src_rect_rs_742_9_4e6257::main (src/main.rs:7:18)</pre></td></tr>
<tr><td valign="top"><pre>let cubic_bezier_shape = CubicBezierShape::from(
    CubicBezierShape::from_points_stroke([Pos2::from((0.0f32, 0.0f32)),
    Pos2::ZERO, Pos2::from([1.0f32, 1.0f32]), Pos2::default()], false,
    Color32::BROWN, PathStroke::default()));
let _ = cubic_bezier_shape.to_path_shapes(None, None);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/shapes/bezier_shape.rs:178:9:
the tolerance should be positive
backtrace:
  &lt;epaint::shapes::bezier_shape::CubicBezierShape&gt;::num_quadratics (epaint/src/shapes/bezier_shape.rs:178:9)
  epaint::shapes::bezier_shape::flatten_cubic_bezier_with_t::&lt;&lt;epaint::shapes::bezier_shape::CubicBezierShape&gt;::flatten_closed::{closure#3}&gt; (epaint/src/shapes/bezier_shape.rs:530:32)
  &lt;epaint::shapes::bezier_shape::CubicBezierShape&gt;::for_each_flattened_with_t::&lt;&lt;epaint::shapes::bezier_shape::CubicBezierShape&gt;::flatten_closed::{closure#3}&gt; (epaint/src/shapes/bezier_shape.rs:317:9)
  &lt;epaint::shapes::bezier_shape::CubicBezierShape&gt;::flatten_closed (epaint/src/shapes/bezier_shape.rs:302:22)
  &lt;epaint::shapes::bezier_shape::CubicBezierShape&gt;::to_path_shapes (epaint/src/shapes/bezier_shape.rs:63:35)
  repro_epaint_src_shapes_bezier_shape_rs_178_9_e65cbb::main (src/main.rs:8:32)</pre></td></tr>
<tr><td valign="top"><pre>let harness = &lt;egui_kittest::Harness&lt;u32&gt;&gt;::new_ui_state(|_, _| (), 0u32);
let _ = Ui::new(Context::default(), harness.ui_id(),
    UiBuilder::new().max_rect(Mesh::with_texture(TextureId::User(
    0u64)).calc_bounds()));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layout.rs:490:9:
avail is NaN: [[NaN inf] - [NaN inf]]
backtrace:
  &lt;egui::layout::Layout&gt;::available_from_cursor_max_rect (egui/src/layout.rs:490:9)
  &lt;egui::layout::Layout&gt;::available_rect_before_wrap (egui/src/layout.rs:414:14)
  &lt;egui::layout::Layout&gt;::next_frame_ignore_wrap (egui/src/layout.rs:589:35)
  &lt;egui::layout::Layout&gt;::next_widget_space_ignore_wrap_justify (egui/src/layout.rs:660:26)
  &lt;egui::layout::Layout&gt;::next_widget_position (egui/src/layout.rs:669:14)
  &lt;egui::layout::Layout&gt;::region_from_max_rect (egui/src/layout.rs:408:25)
  &lt;egui::placer::Placer&gt;::new (egui/src/placer.rs:16:29)
  &lt;egui::ui::Ui&gt;::new (egui/src/ui.rs:131:22)
  repro_egui_src_layout_rs_490_9_0ef05b::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut ui = Ui::new(Context::default(), Id::unique("".to_string()),
    UiBuilder::new());
ui.set_min_width(-9.67e-31f32);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/ui.rs:766:9:
Negative width makes no sense, but got: -0.000000000000000000000000000000967
backtrace:
  &lt;egui::ui::Ui&gt;::set_min_width (egui/src/ui.rs:766:9)
  repro_egui_src_ui_rs_766_9_00cb86::main (src/main.rs:8:8)</pre></td></tr>
<tr><td valign="top"><pre>let drag_value = DragValue::from_get_set(|_| 0 as f64);
let drag_value2 = drag_value.max_decimals(0usize);
let _ = drag_value2.hexadecimal(0usize, false, false);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/widgets/value_format.rs:207:9:
ValueFormat::hexadecimal: `min_width` must be greater than 0
backtrace:
  &lt;egui::widgets::value_format::ValueFormat&gt; (egui/src/widgets/value_format.rs:207:9)
  &lt;egui::widgets::drag_value::DragValue&gt; (egui/src/widgets/drag_value.rs:370:35)
  repro_egui_src_widgets_value_format_rs_207_9_7901ea::main (src/main.rs:6:25)</pre></td></tr>
<tr><td valign="top"><pre>let align2 = Align2::CENTER_TOP;
let rect = align2.anchor_size(Pos2 { x: 0.0f32, y: 0.0f32 },
    PointerState::default().direction());
let rect2 = rect.with_min_y(0.0f32);
let _ = RectTransform::identity(rect2).transform_rect(rect);</pre></td><td valign="top"><pre>thread 'main' panicked at emath/src/lib.rs:170:5:
from.start() and from.end() should not be equal
backtrace:
  emath::remap::&lt;f32, emath::range::Rangef, emath::range::Rangef&gt; (emath/src/lib.rs:170:5)
  &lt;emath::rect_transform::RectTransform&gt;::transform_pos (emath/src/rect_transform.rs:44:13)
  &lt;emath::rect_transform::RectTransform&gt;::transform_rect (emath/src/rect_transform.rs:52:23)
  repro_emath_src_lib_rs_170_5_fee24b::main (src/main.rs:10:44)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rgba::from_luminance_alpha(0.0f32, f32::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at ecolor/src/rgba.rs:96:9:
a should be in the range [0, 1], but was -340282350000000000000000000000000000000
backtrace:
  &lt;ecolor::rgba::Rgba&gt;::from_luminance_alpha (ecolor/src/rgba.rs:96:9)
  repro_ecolor_src_rgba_rs_96_9_15848e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::default();
path.add_open_points(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:388:9:
A path needs at least two points, but got 0
backtrace:
  &lt;epaint::tessellator::Path&gt;::add_open_points (epaint/src/tessellator.rs:388:9)
  repro_epaint_src_tessellator_rs_388_9_0ea346::main (src/main.rs:5:10)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let f32_v = emath::easing::quadratic_out(0.0f32);
let mut paint_list = PaintList::default();
paint_list.transform_after_rounding_range(ShapeIdx(1usize),
    context.layer_painter(LayerId::new(Order::Background,
    Id::new(0u32))).line(vec![], PathStroke::NONE),
    TSTransform { scaling: f32_v,
    translation: MarginF32::from(Margin::symmetric(0i8, 0i8)).right_bottom() });</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layers.rs:201:41:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[epaint::ClippedShape] as core::ops::index::IndexMut&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;egui::layers::PaintList&gt;::transform_after_rounding_range (egui/src/layers.rs:201:41)
  repro_egui_src_layers_rs_201_41_7bcaca::main (src/main.rs:16:16)</pre></td></tr>
<tr><td valign="top"><pre>let _ = egui::text_selection::text_cursor_state::slice_char_range("",
    CharIndex::default().saturating_add(1usize)..CharIndex::default());</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/text_selection/text_cursor_state.rs:485:5:
Invalid range, start must be less than end, but start = 1, end = 0
backtrace:
  egui::text_selection::text_cursor_state::slice_char_range (egui/src/text_selection/text_cursor_state.rs:485:5)
  repro_egui_src_text_selection_text_cursor_state_rs_485_5_b651a8::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let pos2 = Pos2::default();
let cubic_bezier_shape = CubicBezierShape { points: [pos2, pos2, pos2, pos2],
    closed: false,
    fill: FontColorTransferFunction::LIGHT_MODE_DEFAULT.color_from_coverage(
    0.0f32), stroke: PathStroke { width: 0.0f32, color: ColorMode::TRANSPARENT,
    kind: StrokeKind::Inside } };
let _ = cubic_bezier_shape.split_range(0.0f32..256.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/shapes/bezier_shape.rs:141:9:
range should be in [0.0,1.0]
backtrace:
  &lt;epaint::shapes::bezier_shape::CubicBezierShape&gt;::split_range (epaint/src/shapes/bezier_shape.rs:141:9)
  repro_epaint_src_shapes_bezier_shape_rs_141_9_414d32::main (src/main.rs:11:32)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TextureAtlas::new([10000000000000000000usize, 0usize],
    TextOptions { max_texture_side: 0usize,
    color_transfer_function: FontColorTransferFunction::Off,
    font_hinting: false, subpixel_binning: false });</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  epaint::texture_atlas::resize_to_min_height (src/texture_atlas.rs:268)
  epaint::texture_atlas::TextureAtlas::allocate (src/texture_atlas.rs:250)
  epaint::texture_atlas::TextureAtlas::new (src/texture_atlas.rs:95)
  repro_timeout_in_epaint_textureatlas_new_95d793::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::filled([1000000000usize, 1usize], Color32::from_rgb(0u8,
    0u8, 0u8));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_colorimage_filled_aa3489::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TextureAtlas::new([1000000000usize, 1usize],
    TextOptions { max_texture_side: 1usize,
    color_transfer_function: FontColorTransferFunction::
    TwoCoverageMinusCoverageSq, font_hinting: false, subpixel_binning: false });</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::texture_atlas::TextureAtlas::new (src/texture_atlas.rs:85)
  repro_out_of_memory_in_epaint_textureatlas_new_b9038e::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut paint_list = PaintList::default();
paint_list.reset_shape(Context::default().debug_painter().circle_stroke(
    WHITE_UV, 0.0f32, Stroke::new(0.0f32, Color32::from_rgb(0u8, 0u8, 0u8))));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layers.rs:161:15:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[epaint::ClippedShape]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[epaint::ClippedShape] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;egui::layers::PaintList&gt;::reset_shape (egui/src/layers.rs:161:15)
  repro_egui_src_layers_rs_161_15_08feda::main (src/main.rs:9:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut ui = Ui::new(Context::default(), Id::new(0u32), UiBuilder::default());
let _ = ui.allocate_rect(PopupAnchor::from(Pos2::NAN).rect(
    egui::accesskit_root_id(), &amp;Context::default()).unwrap(),
    Sense::from_bits(0u8).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/ui.rs:1390:9:
rect is nan in advance_cursor_after_rect
backtrace:
  &lt;egui::ui::Ui&gt;::advance_cursor_after_rect (egui/src/ui.rs:1390:9)
  &lt;egui::ui::Ui&gt;::allocate_rect (egui/src/ui.rs:1384:23)
  repro_egui_src_ui_rs_1390_9_301ec2::main (src/main.rs:11:16)</pre></td></tr>
<tr><td valign="top"><pre>epaint::tessellator::path::add_circle_quadrant(&amp;mut vec![], Pos2::NAN, 1.0f32,
    307.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:632:46:
range start index 614 out of range for slice of length 9
backtrace:
  &lt;[emath::vec2::Vec2] as core::ops::index::Index&lt;core::ops::range::RangeInclusive&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;[emath::vec2::Vec2; 9] as core::ops::index::Index&lt;core::ops::range::RangeInclusive&lt;usize&gt;&gt;&gt;::index (core/src/array/mod.rs:391:9)
  epaint::tessellator::path::add_circle_quadrant (epaint/src/tessellator.rs:632:46)
  repro_epaint_src_tessellator_rs_632_46_d63657::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut texture_manager = TextureManager::default();
texture_manager.set(TextureManager::default().alloc("".to_string(),
    ImageData::Color(std::sync::Arc::new(ColorImage::example())),
    TextureOptions::NEAREST_REPEAT),
    ImageDelta { image: ImageData::from(ColorImage::default()),
    options: TextureOptions::NEAREST_MIRRORED_REPEAT, pos: None });</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/textures.rs:67:13:
Tried setting texture Managed(0) which is not allocated
backtrace:
  &lt;epaint::textures::TextureManager&gt;::set (epaint/src/textures.rs:67:13)
  repro_epaint_src_textures_rs_67_13_fa3ee4::main (src/main.rs:9:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::filled([1usize, 0usize],
    Color32::KHAKI).region_by_pixels([1usize, 0usize], [0usize, 1usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:281:9:
y + h should be &lt;= self.height(), but y: 0, h: 1, height: 0
backtrace:
  &lt;epaint::image::ColorImage&gt;::region_by_pixels (epaint/src/image.rs:281:9)
  repro_epaint_src_image_rs_281_9_7e485c::main (src/main.rs:5:66)</pre></td></tr>
<tr><td valign="top"><pre>let vec2 = &lt;emath::Vec2 as emath::GuiRounding&gt;::round_to_pixel_center(
    RectAlign::TOP_START.child().to_sign(), 0.0f32);
let _ = Layout::top_down_justified(
    Align2::CENTER_CENTER.y()).align_size_within_rect(vec2, Rect::ZERO);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layout.rs:375:9:
Negative size: [-inf -inf]
backtrace:
  &lt;egui::layout::Layout&gt;::align_size_within_rect (egui/src/layout.rs:375:9)
  repro_egui_src_layout_rs_375_9_d15d97::main (src/main.rs:8:67)</pre></td></tr>
<tr><td valign="top"><pre>let mut texture_atlas = TextureAtlas::new([10000usize, 1usize],
    TextOptions::default());
let _ = texture_atlas.allocate((0usize, 1000000000usize));
let _ = texture_atlas.allocate((0usize, usize::MAX));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/texture_atlas.rs:241:31:
attempt to add with overflow
backtrace:
  &lt;epaint::texture_atlas::TextureAtlas&gt;::allocate (epaint/src/texture_atlas.rs:241:31)
  repro_epaint_src_texture_atlas_rs_241_31_81fb25::main (src/main.rs:7:27)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::default();
let mut mesh = Mesh::from(Mesh { indices: vec![0u32], vertices: vec![],
    texture_id: TextureId::default() });
path.fill_with_uv(0.0f32, Color32::from_rgba_unmultiplied_const(0u8, 0u8, 0u8,
    1u8), TextureId::Managed(1u64), |a| a, &amp;mut mesh);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:954:9:
assertion `left == right` failed: Mixing different `texture_id` in the same 
  left: Managed(0)
 right: Managed(1)
backtrace:
  epaint::tessellator::fill_closed_path_with_uv::&lt;repro_epaint_src_tessellator_rs_954_9_7e59b8::main::{closure#0}&gt; (epaint/src/tessellator.rs:954:9)
  &lt;epaint::tessellator::Path&gt;::fill_with_uv::&lt;repro_epaint_src_tessellator_rs_954_9_7e59b8::main::{closure#0}&gt; (epaint/src/tessellator.rs:548:9)
  repro_epaint_src_tessellator_rs_954_9_7e59b8::main (src/main.rs:9:10)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
context.set_sublayer(LayerId::new(Order::Tooltip, Id::NULL),
    LayerId::background());</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/memory/mod.rs:1354:9:
assertion `left == right` failed: DEBUG ASSERT: Trying to set sublayers across layers of different order (Tooltip, Background), which is currently undefined behavior in egui
  left: Tooltip
 right: Background
backtrace:
  &lt;egui::memory::Areas&gt;::set_sublayer (egui/src/memory/mod.rs:1354:9)
  &lt;egui::context::Context&gt;::set_sublayer::{closure#0} (egui/src/context.rs:3641:47)
  &lt;egui::context::Context&gt;::memory_mut::&lt;(), &lt;egui::context::Context&gt;::set_sublayer::{closure#0}&gt;::{closure#0} (egui/src/context.rs:1283:31)
  &lt;egui::context::Context&gt;::write::&lt;(), &lt;egui::context::Context&gt;::memory_mut&lt;(), &lt;egui::context::Context&gt;::set_sublayer::{closure#0}&gt;::{closure#0}&gt; (egui/src/context.rs:920:9)
  &lt;egui::context::Context&gt;::memory_mut::&lt;(), &lt;egui::context::Context&gt;::set_sublayer::{closure#0}&gt; (egui/src/context.rs:1283:14)
  &lt;egui::context::Context&gt;::set_sublayer (egui/src/context.rs:3641:14)
  repro_egui_src_memory_mod_rs_1354_9_73b959::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut mesh = Mesh::with_texture(TextureId::User(1u64));
mesh.add_colored_rect(Rect::ZERO, Color32::DARK_BLUE);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/mesh.rs:232:9:
Mesh has an assigned texture
backtrace:
  &lt;epaint::mesh::Mesh&gt;::add_colored_rect (epaint/src/mesh.rs:232:9)
  repro_epaint_src_mesh_rs_232_9_5f93ef::main (core/src/slice/mod.rs:8:10)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::default();
let mut mesh = Mesh::with_texture(TextureId::default());
path.fill_with_uv(1.0f32, Visuals::light().weak_text_color(),
    TextureId::User(0u64), |a| a, &amp;mut mesh);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:982:22:
attempt to subtract with overflow
backtrace:
  epaint::tessellator::fill_closed_path_with_uv::&lt;repro_epaint_src_tessellator_rs_982_22_615074::main::{closure#0}&gt; (epaint/src/tessellator.rs:982:22)
  &lt;epaint::tessellator::Path&gt;::fill_with_uv::&lt;repro_epaint_src_tessellator_rs_982_22_615074::main::{closure#0}&gt; (epaint/src/tessellator.rs:548:9)
  repro_epaint_src_tessellator_rs_982_22_615074::main (src/main.rs:9:10)</pre></td></tr>
<tr><td valign="top"><pre>let texture_atlas = TextureAtlas::new([10000000000000000usize, 1usize],
    TextOptions { max_texture_side: 0usize,
    color_transfer_function: FontColorTransferFunction::Off,
    font_hinting: false, subpixel_binning: false });
let _ = texture_atlas.options();</pre></td><td valign="top"><pre>out of memory: memory allocation of 40000000000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::texture_atlas::TextureAtlas::new (src/texture_atlas.rs:85)
  repro_out_of_memory_in_epaint_textureatlas_options_446f0d::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut fonts = Fonts::new(TextOptions { max_texture_side: 10000usize,
    color_transfer_function: FontColorTransferFunction::Gamma(0.0f32),
    font_hinting: false, subpixel_binning: true }, FontDefinitions::default());
let _ = fonts.layout_uncached(1e30f32,
    std::sync::Arc::new(LayoutJob::simple("\u{301}".to_string(),
    FontId::monospace(1.0f32), Color32::WHITE, 0.0f32)));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/glyph_atlas.rs:120:18:
attempt to add with overflow
backtrace:
  &lt;epaint::text::glyph_atlas::SubpixelBin&gt;::new (epaint/src/text/glyph_atlas.rs:120:18)
  &lt;epaint::text::glyph_atlas::GlyphAtlas&gt;::allocate_outline (epaint/src/text/glyph_atlas.rs:289:13)
  &lt;epaint::text::fonts::FontsImpl&gt;::allocate_glyph (epaint/src/text/fonts.rs:753:14)
  epaint::text::text_layout::layout_shaped_run (epaint/src/text/text_layout.rs:367:23)
  epaint::text::text_layout::layout_section (epaint/src/text/text_layout.rs:603:13)
  epaint::text::text_layout::layout (epaint/src/text/text_layout.rs:116:28)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached_for_viewport (epaint/src/text/fonts.rs:291:9)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached (epaint/src/text/fonts.rs:277:14)
  repro_epaint_src_text_glyph_atlas_rs_120_18_952698::main (src/main.rs:11:19)</pre></td></tr>
<tr><td valign="top"><pre>let galley = Galley::concat(std::sync::Arc::new(
    egui_extras::syntax_highlighting::highlight(&amp;Context::default(),
    &amp;Style::default(), &amp;CodeTheme::default(), "", "")), &amp;[], 0.0f32);
let _ = galley.cursor_end_of_row(&amp;CCursor::end_of_str(""));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/text_layout_types.rs:1423:30:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[epaint::text::text_layout_types::PlacedRow]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[epaint::text::text_layout_types::PlacedRow] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;epaint::text::text_layout_types::Galley&gt;::cursor_end_of_row (epaint/src/text/text_layout_types.rs:1423:30)
  repro_epaint_src_text_text_layout_types_rs_1423_30_1bf3b2::main (src/main.rs:9:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::default();
path.add_line_loop(&amp;[WHITE_UV,
    Align2::LEFT_BOTTOM.pos_in_rect(&amp;Rect::everything_above(1.0f32))]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:444:16:
Comparing NaN vectors ([-0.0 NaN] and [0.0 0.0]). A NaN is not even equal to itself, which leads to very confusing bugs.
backtrace:
  &lt;emath::vec2::Vec2 as core::cmp::PartialEq&gt;::eq (emath/src/vec2.rs:353:9)
  &lt;epaint::tessellator::Path&gt;::add_line_loop (epaint/src/tessellator.rs:444:16)
  repro_epaint_src_tessellator_rs_444_16_a899b9::main (src/main.rs:8:10)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::default();
path.reserve(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 16000000000 bytes failed
backtrace:
  epaint::tessellator::Path::reserve (epaint/src/tessellator.rs:337)
  repro_out_of_memory_in_epaint_tessellator_path_reserve_16839b::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let rgba = Rgba::BLUE;
let f32_v = rgba.intensity();
let fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
let fonts2 = fonts.with_missing_glyph_policy(
    Context::default().missing_glyph_policy());
let mut fonts3 = fonts2.with_missing_glyph_policy(MissingGlyphPolicy::Panic);
let mut fonts_view = fonts3.with_pixels_per_point(f32_v);
let color32 = Color32::LIGHT_BLUE;
let _ = fonts_view.layout("a".to_string(),
    FontSelection::default().resolve(&amp;Style::default()), color32, 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/fonts.rs:806:9:
No glyph for "a" (U+0061) in Proportional. Installed fonts: []. The family has no fonts at all: install some with `FontDefinitions`, or enable the `default_fonts` feature to bundle egui's.
backtrace:
  &lt;epaint::text::fonts::FontsImpl&gt;::on_missing_glyph (epaint/src/text/fonts.rs:806:9)
  &lt;epaint::text::fonts::FontsImpl&gt;::fontless_cluster (epaint/src/text/fonts.rs:833:14)
  epaint::text::text_layout::segment_into_runs (epaint/src/text/text_layout.rs:1538:32)
  epaint::text::text_layout::layout_section (epaint/src/text/text_layout.rs:572:9)
  epaint::text::text_layout::layout (epaint/src/text/text_layout.rs:116:28)
  &lt;epaint::text::galley_cache::GalleyCache&gt;::layout_internal (epaint/src/text/galley_cache.rs:112:34)
  &lt;epaint::text::galley_cache::GalleyCache&gt;::layout (epaint/src/text/galley_cache.rs:134:14)
  &lt;epaint::text::fonts::FontsView&gt;::layout_job (epaint/src/text/fonts.rs:421:27)
  &lt;epaint::text::fonts::FontsView&gt;::layout (epaint/src/text/fonts.rs:453:14)
  repro_epaint_src_text_fonts_rs_806_9_a94fbe::main (src/main.rs:19:24)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Shape::mesh(std::sync::Arc::new(Mesh { indices: vec![1u32],
    vertices: vec![], texture_id: TextureId::Managed(0u64) }));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/shapes/shape.rs:386:9:
Invalid mesh: Mesh {
    indices: [
        1,
    ],
    vertices: [],
    texture_id: Managed(
        0,
    ),
}
backtrace:
  &lt;epaint::shapes::shape::Shape&gt;::mesh::&lt;alloc::sync::Arc&lt;epaint::mesh::Mesh&gt;&gt; (epaint/src/shapes/shape.rs:386:9)
  repro_epaint_src_shapes_shape_rs_386_9_535444::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Layout::top_down_justified(Align::TOP).align_size_within_rect(
    Vec2::from(Vec2b::TRUE), Context::default().globally_used_rect());</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layout.rs:376:9:
Negative outer: [[inf inf] - [-inf -inf]]
backtrace:
  &lt;egui::layout::Layout&gt;::align_size_within_rect (egui/src/layout.rs:376:9)
  repro_egui_src_layout_rs_376_9_2db755::main (src/main.rs:8:52)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageData::Color(std::sync::Arc::new(ColorImage::filled(
    [1000000000usize, 1usize], Color32::BLACK)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_imagedata_color_fb92bb::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut ui = Ui::new(Context::default(),
    ViewportId::from_hash_of(0u8).root_ui_id(), UiBuilder::new());
ui.set_height(-3.5527137e-15f32);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/ui.rs:776:9:
Negative height makes no sense, but got: -0.0000000000000035527137
backtrace:
  &lt;egui::ui::Ui&gt;::set_min_height (egui/src/ui.rs:776:9)
  &lt;egui::ui::Ui&gt;::set_height (egui/src/ui.rs:849:14)
  repro_egui_src_ui_rs_776_9_845da2::main (src/main.rs:8:8)</pre></td></tr>
<tr><td valign="top"><pre>let mut mesh = Mesh::default();
mesh.reserve_vertices(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from epaint/src/mesh.rs:194:23)
capacity overflow
backtrace:
  &lt;epaint::mesh::Mesh&gt;::reserve_vertices (epaint/src/mesh.rs:194:23)
  repro_epaint_src_mesh_rs_194_23_3d0d8b::main (src/main.rs:5:10)</pre></td></tr>
<tr><td valign="top"><pre>let _ = GlyphBitmap { image: ColorImage::filled([1usize, 1000000000usize],
    Color32::GOLD), offset_px: emath::vec2(0.0f32, 0.0f32), is_color: false };</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_text_glyphbitmap_literal_f0ff6c::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let texture_atlas = TextureAtlas::new([10000usize, 0usize],
    TextOptions { max_texture_side: 0usize,
    color_transfer_function: FontColorTransferFunction::LIGHT_MODE_DEFAULT,
    font_hinting: false, subpixel_binning: false });
let _ = texture_atlas.image();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  epaint::texture_atlas::resize_to_min_height (src/texture_atlas.rs:268)
  epaint::texture_atlas::TextureAtlas::allocate (src/texture_atlas.rs:250)
  epaint::texture_atlas::TextureAtlas::new (src/texture_atlas.rs:95)
  repro_timeout_in_epaint_textureatlas_image_423884::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let align2 = Align2::RIGHT_TOP;
let quadratic_bezier_shape = QuadraticBezierShape { points: [Pos2 { x: 0.0f32,
    y: 0.0f32 }, align2.pos_in_rect(&amp;Rect::everything_above(0.0f32)),
    Pos2::new(0.0f32, 0.0f32)], closed: false, fill: Color32::ORANGE,
    stroke: PathStroke::default() };
let _ = quadratic_bezier_shape.sample(10000000000.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/shapes/bezier_shape.rs:454:9:
the sample value should be in [0.0,1.0]
backtrace:
  &lt;epaint::shapes::bezier_shape::QuadraticBezierShape&gt;::sample (epaint/src/shapes/bezier_shape.rs:454:9)
  repro_epaint_src_shapes_bezier_shape_rs_454_9_3306eb::main (src/main.rs:11:36)</pre></td></tr>
<tr><td valign="top"><pre>let _ = RasterizedGlyph { bitmap: GlyphBitmap { image: ColorImage::filled(
    [1usize, 10000000000000000usize], Color32::from_white_alpha(0u8)),
    offset_px: Vec2::UP, is_color: false }, advance_px: 0.0f32 };</pre></td><td valign="top"><pre>out of memory: memory allocation of 40000000000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_text_rasterizedglyph_literal_e8dc05::main (src/main.rs:8)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut texture_atlas = TextureAtlas::new([1000000usize, 1usize],
    TextOptions::default());
let _ = texture_atlas.allocate((0usize, usize::MAX));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/texture_atlas.rs:260:49:
attempt to add with overflow
backtrace:
  &lt;epaint::texture_atlas::TextureAtlas&gt;::allocate (epaint/src/texture_atlas.rs:260:49)
  repro_epaint_src_texture_atlas_rs_260_49_6fd5ed::main (src/main.rs:6:27)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let mut ui = Ui::new(context.clone(), ViewportId::ROOT.root_ui_id(),
    UiBuilder::default());
let _ = ui.place(Rect::everything_above(0.0f32).with_max_x(f32::NAN),
    Hyperlink::new("".to_string()));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/ui.rs:252:9:
max_rect is NaN: [[-inf -inf] - [NaN 0.0]]
backtrace:
  &lt;egui::ui::Ui&gt;::new_child (egui/src/ui.rs:252:9)
  &lt;egui::ui::Ui&gt;::place::&lt;egui::widgets (egui/src/ui.rs:1679:14)
  repro_egui_src_ui_rs_252_9_878c8e::main (src/main.rs:11:16)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageData::from(std::sync::Arc::new(ColorImage::filled([1usize,
    1000000000usize], Color32::DARK_BLUE)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_imagedata_from_std_sync_arc_epaint_c_3bf7f1::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let rect_transform = RectTransform::identity(
    Align2::CENTER_CENTER.align_size_within_rect(
    PointerState::default().velocity(), Context::default().content_rect()));
let _ = rect_transform.transform_pos_clamped(Pos2::NAN);</pre></td><td valign="top"><pre>thread 'main' panicked at emath/src/lib.rs:197:9:
from.start() and from.end() should not be equal
backtrace:
  emath::remap_clamp::&lt;f32, emath::range::Rangef, emath::range::Rangef&gt; (emath/src/lib.rs:197:9)
  &lt;emath::rect_transform::RectTransform&gt;::transform_pos_clamped (emath/src/rect_transform.rs:61:13)
  repro_emath_src_lib_rs_197_9_931679::main (src/main.rs:9:28)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageDelta::partial([1usize, 0usize],
    ImageData::from(std::sync::Arc::new(ColorImage::filled([1usize,
    1000000000usize], Color32::YELLOW))),
    TextureOptions::LINEAR_MIRRORED_REPEAT);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_imagedelta_partial_274e6f::main (src/main.rs:8)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let galley = Galley::concat(std::sync::Arc::new(
    MemoizedEasymarkHighlighter::default().highlight(&amp;Style::default(), "")),
    &amp;[], 0.0f32);
let _ = galley.cursor_end_of_paragraph(&amp;CCursor::end_of_str(""));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/text_layout_types.rs:1455:33:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[epaint::text::text_layout_types::PlacedRow]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[epaint::text::text_layout_types::PlacedRow] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;epaint::text::text_layout_types::Galley&gt;::cursor_end_of_paragraph (epaint/src/text/text_layout_types.rs:1455:33)
  repro_epaint_src_text_text_layout_types_rs_1455_33_036f27::main (src/main.rs:8:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut paint_list = PaintList::default();
paint_list.transform_range(PaintList::default().next_idx(), ShapeIdx(1usize),
    TSTransform::new(Vec2::NAN, 0.0f32));</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layers.rs:177:41:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[epaint::ClippedShape] as core::ops::index::IndexMut&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;egui::layers::PaintList&gt;::transform_range (egui/src/layers.rs:177:41)
  repro_egui_src_layers_rs_177_41_a3502f::main (src/main.rs:8:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::default());
let _ = fonts.layout_uncached_for_viewport(1.0f32, ViewportKey::default(),
    std::sync::Arc::new(LayoutJob::simple_format("Za".to_string(),
    TextFormat { font_id: FontId::monospace(1.0f32),
    extra_letter_spacing: f32::NEG_INFINITY, line_height: None,
    color: Color32::from_black_alpha(0u8), background: Color32::PURPLE,
    expand_bg: 0.0f32, coords: VariationCoords::default(), italics: false,
    underline: Stroke::new(0.0f32, Color32::TRANSPARENT),
    strikethrough: Stroke { width: 0.0f32, color: Color32::TRANSPARENT },
    valign: Align::Min })));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/glyph_atlas.rs:108:18:
attempt to subtract with overflow
backtrace:
  &lt;epaint::text::glyph_atlas::SubpixelBin&gt;::new (epaint/src/text/glyph_atlas.rs:108:18)
  &lt;epaint::text::glyph_atlas::GlyphAtlas&gt;::allocate_outline (epaint/src/text/glyph_atlas.rs:289:13)
  &lt;epaint::text::fonts::FontsImpl&gt;::allocate_glyph (epaint/src/text/fonts.rs:753:14)
  epaint::text::text_layout::layout_shaped_run (epaint/src/text/text_layout.rs:367:23)
  epaint::text::text_layout::layout_section (epaint/src/text/text_layout.rs:603:13)
  epaint::text::text_layout::layout (epaint/src/text/text_layout.rs:116:28)
  &lt;epaint::text::fonts::Fonts&gt;::layout_uncached_for_viewport (epaint/src/text/fonts.rs:291:9)
  repro_epaint_src_text_glyph_atlas_rs_108_18_14e4ef::main (src/main.rs:15:19)</pre></td></tr>
<tr><td valign="top"><pre>let mut texture_manager = TextureManager::default();
texture_manager.set(TextureId::default(), TextureAtlas::new([100000000000usize,
    1usize], TextOptions::default()).take_delta().unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 400000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::texture_atlas::TextureAtlas::new (src/texture_atlas.rs:85)
  repro_out_of_memory_in_epaint_texturemanager_set_9561b5::main (src/main.rs:8)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let ui = Ui::new(context.clone(), Id::new(0u32), UiBuilder::default());
let _ = ui.interact_opt(Rect::NAN,
    Context::default().viewport_id().root_ui_id(), Sense::CLICK,
    InteractOptions { move_to_top: false });</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/context.rs:1523:9:
widget rect is NaN: [[NaN NaN] - [NaN NaN]]
backtrace:
  &lt;egui::context::Context&gt;::create_widget (egui/src/context.rs:1523:9)
  &lt;egui::ui::Ui&gt;::interact_opt (egui/src/ui.rs:958:20)
  repro_egui_src_context_rs_1523_9_070f26::main (src/main.rs:12:16)</pre></td></tr>
<tr><td valign="top"><pre>let fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
let mut fonts2 = fonts.with_missing_glyph_policy(MissingGlyphPolicy::Tofu);
let _ = fonts2.layout_uncached(65536.0f32,
    std::sync::Arc::new(egui_extras::syntax_highlighting::highlight(
    &amp;Context::default(), &amp;Style::default(), &amp;CodeTheme::default(), "C", "")));</pre></td><td valign="top"><pre>out of memory: memory allocation of 1016189943808 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::text::fonts::synthetic_tofu (src/text/fonts.rs:870)
  epaint::text::fonts::{impl#3}::new::{closure#0} (src/text/fonts.rs:524)
  epaint::text::glyph_atlas::{impl#6}::allocate_raster::{closure#1}&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::glyph_atlas::GlyphAtlas::allocate_raster&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::fonts::FontsImpl::fontless_cluster (src/text/fonts.rs:837)
  epaint::text::text_layout::segment_into_runs (src/text/text_layout.rs:1538)
  epaint::text::text_layout::layout_section (src/text/text_layout.rs:572)
  epaint::text::text_layout::layout (src/text/text_layout.rs:116)
  epaint::text::fonts::Fonts::layout_uncached_for_viewport (src/text/fonts.rs:291)
  epaint::text::fonts::Fonts::layout_uncached (src/text/fonts.rs:277)
  repro_out_of_memory_in_epaint_fonts_layout_uncached_881443::main (src/main.rs:12)
  (24 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let texture_atlas = TextureAtlas::new([1000000000usize, 1usize],
    TextOptions::default());
let _ = texture_atlas.image();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::texture_atlas::TextureAtlas::new (src/texture_atlas.rs:85)
  repro_out_of_memory_in_epaint_textureatlas_image_7b2207::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let color_image = ColorImage::example();
let _ = color_image.region_by_pixels([0usize, 1usize], [0usize, usize::MAX]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:282:13:
attempt to add with overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::region_by_pixels (epaint/src/image.rs:282:13)
  repro_epaint_src_image_rs_282_13_138bde::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut path = Path::default();
path.add_open_points(&amp;[AreaState::default().left_top_pos(), Pos2::new(0.0f32,
    1.0f32), Rect::everything_below(0.0f32).right_center()]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:403:27:
Comparing NaN vectors ([NaN NaN] and [0.0 0.0]). A NaN is not even equal to itself, which leads to very confusing bugs.
backtrace:
  &lt;emath::vec2::Vec2 as core::cmp::PartialEq&gt;::eq (emath/src/vec2.rs:353:9)
  &lt;epaint::tessellator::Path&gt;::add_open_points (epaint/src/tessellator.rs:403:27)
  repro_epaint_src_tessellator_rs_403_27_85e63c::main (src/main.rs:8:10)</pre></td></tr>
<tr><td valign="top"><pre>let texture_atlas = TextureAtlas::new([100000000usize, 0usize],
    TextOptions { max_texture_side: 0usize,
    color_transfer_function: FontColorTransferFunction::Off,
    font_hinting: false, subpixel_binning: false });
let _ = texture_atlas.options();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  epaint::image::ColorImage::height (src/image.rs:239)
  epaint::texture_atlas::resize_to_min_height (src/texture_atlas.rs:267)
  epaint::texture_atlas::TextureAtlas::allocate (src/texture_atlas.rs:250)
  epaint::texture_atlas::TextureAtlas::new (src/texture_atlas.rs:95)
  repro_timeout_in_epaint_textureatlas_options_f7125c::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::from(ColorImage::filled([1000000000usize, 1usize],
    Color32::from_rgba_unmultiplied(1u8, 0u8, 0u8, 0u8)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_colorimage_from_epaint_colorimage_0e3a32::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let screenshot_callback = ScreenshotCallback::new(|_| ());
screenshot_callback.clone().complete(std::sync::Arc::new(ColorImage::filled(
    [1usize, 1000000000usize],
    Color32::from(Hsva::from_srgba_premultiplied([0u8, 0u8, 0u8, 0u8])))));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_egui_screenshotcallback_complete_ce4916::main (src/main.rs:8)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mesh = Mesh { indices: vec![0u32], vertices: vec![],
    texture_id: TextureId::User(0u64) };
let _ = mesh.clone().split_to_u16();</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/mesh.rs:244:9:
Mesh is invalid
backtrace:
  &lt;epaint::mesh::Mesh&gt;::split_to_u16 (epaint/src/mesh.rs:244:9)
  repro_epaint_src_mesh_rs_244_9_dcefff::main (src/main.rs:6:26)</pre></td></tr>
<tr><td valign="top"><pre>let text_cursor_state = TextCursorState::from(CCursorRange::default());
let _ = text_cursor_state.range(&amp;Fonts::new(TextOptions::default(),
    FontDefinitions::empty()).layout_uncached(65536.0f32,
    std::sync::Arc::new(MemoizedEasymarkHighlighter::default().highlight(
    &amp;Style::default(), "a"))));</pre></td><td valign="top"><pre>out of memory: memory allocation of 1016189943808 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::text::fonts::synthetic_tofu (src/text/fonts.rs:870)
  epaint::text::fonts::{impl#3}::new::{closure#0} (src/text/fonts.rs:524)
  epaint::text::glyph_atlas::{impl#6}::allocate_raster::{closure#1}&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::glyph_atlas::GlyphAtlas::allocate_raster&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::fonts::FontsImpl::fontless_cluster (src/text/fonts.rs:837)
  epaint::text::text_layout::segment_into_runs (src/text/text_layout.rs:1538)
  epaint::text::text_layout::layout_section (src/text/text_layout.rs:572)
  epaint::text::text_layout::layout (src/text/text_layout.rs:116)
  epaint::text::fonts::Fonts::layout_uncached_for_viewport (src/text/fonts.rs:291)
  epaint::text::fonts::Fonts::layout_uncached (src/text/fonts.rs:277)
  repro_out_of_memory_in_egui_text_selection_textcursorstate_range_01f9f2::main (src/main.rs:11)
  (24 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut mesh = Mesh::default();
mesh.reserve_vertices(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 20000000000 bytes failed
backtrace:
  epaint::mesh::Mesh::reserve_vertices (epaint/src/mesh.rs:194)
  repro_out_of_memory_in_epaint_mesh_reserve_vertices_f54039::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Shape::dashed_line(&amp;[Pos2 { x: 0.0f32, y: 0.0f32 },
    Rect::EVERYTHING.left_top()], Stroke::NONE, 0.0f32, 1.0f32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  epaint::shapes::shape::dashes_from_line (src/shapes/shape.rs:581)
  epaint::shapes::shape::Shape::dashed_line&lt;epaint::stroke::Stroke&gt; (epaint/src/shapes/shape.rs:185)
  repro_out_of_memory_in_epaint_shape_dashed_line_9fb5d8::main (src/main.rs:7)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Event::PasteImage(std::sync::Arc::new(ColorImage::filled([1usize,
    1000000000usize], Color32::GOLD)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_egui_event_pasteimage_09422c::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut texture_manager = TextureManager::default();
let _ = texture_manager.alloc("".to_string(),
    ImageData::from(ColorImage::filled([1000000000usize, 1usize],
    Color32::DEBUG_COLOR)), TextureOptions::LINEAR_REPEAT);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_texturemanager_alloc_e0f3f0::main (src/main.rs:9)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageData::from(ColorImage::filled([1000000000usize, 1usize],
    Color32::PURPLE));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_imagedata_from_epaint_colorimage_e1243f::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>epaint::tessellator::path::add_circle_quadrant(&amp;mut vec![],
    Rect::everything_right_of(0.0f32).left_top(), 65536.0f32, f32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:647:26:
attempt to multiply with overflow
backtrace:
  epaint::tessellator::path::add_circle_quadrant (epaint/src/tessellator.rs:647:26)
  repro_epaint_src_tessellator_rs_647_26_d29de7::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut history = &lt;emath::History&lt;u8&gt;&gt;::new(1usize..1usize, 0.0f32);
history.add(1.0f64, 0u8);
history.add(0.0f64, 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at emath/src/history.rs:129:13:
Time shouldn't move backwards. Last time: 1, new time: 0
backtrace:
  &lt;emath (emath/src/history.rs:129:13)
  repro_emath_src_history_rs_129_13_29c6b3::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Shape::dotted_line(&amp;[Pos2 { x: 0.0f32, y: 1.0f32 }, Pos2::from([0.0f32,
    0.0f32])], Color32::from_black_alpha(0u8), 0.0f32, 0.0f32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  epaint::shapes::shape::points_from_line (src/shapes/shape.rs:545)
  epaint::shapes::shape::Shape::dotted_line&lt;ecolor::color32::Color32&gt; (epaint/src/shapes/shape.rs:173)
  repro_out_of_memory_in_epaint_shape_dotted_line_d005a6::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let pos2 = Pos2::default();
let mut path = Path::default();
path.add_open_points(&amp;[pos2, Rect::everything_left_of(0.0f32).left_top(),
    pos2]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:401:20:
Comparing NaN vectors ([NaN NaN] and [0.0 0.0]). A NaN is not even equal to itself, which leads to very confusing bugs.
backtrace:
  &lt;emath::vec2::Vec2 as core::cmp::PartialEq&gt;::eq (emath/src/vec2.rs:353:9)
  &lt;epaint::tessellator::Path&gt;::add_open_points (epaint/src/tessellator.rs:401:20)
  repro_epaint_src_tessellator_rs_401_20_caea78::main (src/main.rs:8:10)</pre></td></tr>
<tr><td valign="top"><pre>let _ = OutputCommand::CopyImage(ColorImage::filled([1000000000usize, 1usize],
    Color32::GRAY));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_egui_outputcommand_copyimage_1eaa50::main (src/main.rs:6)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let id_type_map = IdTypeMap::default();
let usize_v = id_type_map.count::&lt;u8&gt;();
let color_image = ColorImage { size: [1usize, 1usize],
    source_size: State::default().overscroll(), pixels: vec![] };
let _ = color_image.region_by_pixels([0usize, usize_v], [1usize, 1usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:292:39:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[ecolor::color32::Color32] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;epaint::image::ColorImage&gt;::region_by_pixels (epaint/src/image.rs:292:39)
  repro_epaint_src_image_rs_292_39_b3a5cd::main (src/main.rs:9:25)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Mesh::with_texture(TextureManager::default().alloc("".to_string(),
    ImageData::from(ColorImage::filled([10000usize, 65536usize],
    Color32::CYAN)), TextureOptions::NEAREST_REPEAT));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2621440000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_mesh_with_texture_cf1282::main (src/main.rs:9)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageDelta { image: ImageData::Color(std::sync::Arc::new(
    ColorImage::filled([1usize, 10000000000000usize], Color32::DARK_RED))),
    options: TextureOptions::LINEAR, pos: None };</pre></td><td valign="top"><pre>out of memory: memory allocation of 40000000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_imagedelta_literal_390a85::main (src/main.rs:8)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut mesh = Mesh::with_texture(TextureId::User(0u64));
mesh.reserve_triangles(10000000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 120000000000000 bytes failed
backtrace:
  epaint::mesh::Mesh::reserve_triangles (epaint/src/mesh.rs:187)
  repro_out_of_memory_in_epaint_mesh_reserve_triangles_a6240b::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let pos2 = Pos2::default();
let cubic_bezier_shape = CubicBezierShape { points: [Pos2 { x: 0.0f32,
    y: 0.0f32 }, pos2, pos2, pos2], closed: false,
    fill: Color32::from_black_alpha(0u8), stroke: PathStroke::default() };
let _ = cubic_bezier_shape.sample(10000000000.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/shapes/bezier_shape.rs:229:9:
the sample value should be in [0.0,1.0]
backtrace:
  &lt;epaint::shapes::bezier_shape::CubicBezierShape&gt;::sample (epaint/src/shapes/bezier_shape.rs:229:9)
  repro_epaint_src_shapes_bezier_shape_rs_229_9_8723ac::main (src/main.rs:9:32)</pre></td></tr>
<tr><td valign="top"><pre>let mut ui = Ui::new(Context::default(), Id::new("".to_string()),
    UiBuilder::default());
ui.add_space(f32::NAN);
ui.shrink_height_to_current();</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layout.rs:84:9:
cursor has Nan: [[0.0 NaN] - [10000.0 inf]]
backtrace:
  &lt;egui::layout::Region&gt;::sanity_check (egui/src/layout.rs:84:9)
  &lt;egui::layout::Layout&gt;::next_frame_ignore_wrap (egui/src/layout.rs:583:16)
  &lt;egui::layout::Layout&gt;::next_widget_space_ignore_wrap_justify (egui/src/layout.rs:660:26)
  &lt;egui::placer::Placer&gt;::next_widget_space_ignore_wrap_justify (egui/src/placer.rs:228:14)
  &lt;egui::placer::Placer&gt;::set_max_height (egui/src/placer.rs:249:25)
  &lt;egui::ui::Ui&gt;::set_max_height (egui/src/ui.rs:751:21)
  &lt;egui::ui::Ui&gt;::shrink_height_to_current (egui/src/ui.rs:819:14)
  repro_egui_src_layout_rs_84_9_0c5b65::main (src/main.rs:9:8)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Mesh16 { indices: vec![], vertices: vec![],
    texture_id: TextureManager::default().alloc("".to_string(),
    ImageData::from(ColorImage::filled([100000000000usize, 1usize],
    Color32::PLACEHOLDER)), TextureOptions::NEAREST) };</pre></td><td valign="top"><pre>out of memory: memory allocation of 400000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_mesh16_literal_d22ae5::main (src/main.rs:9)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut interaction = Interaction::default();
let mut ui = Ui::new(Context::default(),
    ViewportId::from_hash_of(0u8).root_ui_id(),
    UiBuilder::default().max_rect(Rect::EVERYTHING));
let mut ui2 = ui.new_child(UiBuilder::new());
interaction.ui(&amp;mut ui2);</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/grid.rs:107:9:
Grid not yet available for right-to-left layouts
backtrace:
  &lt;egui::grid::GridLayout&gt;::new (egui/src/grid.rs:107:9)
  &lt;egui::grid::Grid&gt;::show_dyn::&lt;()&gt;::{closure#3}::{closure#0} (egui/src/grid.rs:485:23)
  &lt;&lt;egui::grid::Grid&gt;::show_dyn&lt;()&gt;::{closure#3}::{closure#0} as core::ops::function::FnOnce&lt;(&amp;mut egui::ui::Ui,)&gt;&gt;::call_once::{shim:vtable#0} (core/src/ops/function.rs:250:5)
  &lt;egui::ui::Ui&gt;::scope_dyn::&lt;()&gt; (egui/src/ui.rs:2336:19)
  &lt;egui::ui::Ui&gt;::allocate_ui_with_layout_dyn::&lt;()&gt; (egui/src/ui.rs:1469:14)
  &lt;egui::ui::Ui&gt; (egui/src/ui.rs:2513:14)
  &lt;egui::ui::Ui&gt; (egui/src/ui.rs:2441:14)
  &lt;egui::grid::Grid&gt;::show_dyn::&lt;()&gt;::{closure#3} (egui/src/grid.rs:476:16)
  &lt;&lt;egui::grid::Grid&gt;::show_dyn&lt;()&gt;::{closure#3} as core::ops::function::FnOnce&lt;(&amp;mut egui::ui::Ui,)&gt;&gt;::call_once::{shim:vtable#0} (core/src/ops/function.rs:250:5)
  &lt;egui::ui::Ui&gt;::scope_dyn::&lt;()&gt; (egui/src/ui.rs:2336:19)
  &lt;egui::ui::Ui&gt;::scope_builder::&lt;(), &lt;egui::grid::Grid&gt;::show_dyn&lt;()&gt;::{closure#3}&gt; (egui/src/ui.rs:2324:14)
  &lt;egui::grid::Grid&gt;::show_dyn::&lt;()&gt; (egui/src/grid.rs:475:12)</pre></td></tr>
<tr><td valign="top"><pre>let usize_v = MAX_GLYPH_SIZE;
let color_image = ColorImage { size: [usize::MAX, usize::MAX],
    source_size: Vec2::UP,
    pixels: vec![FontColorTransferFunction::Gamma(
    -1.1754944e-38f32).color_from_coverage(f32::MIN)] };
let _ = color_image.region_by_pixels([usize_v, 0usize], [1usize,
    18446744073709551614usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from epaint/src/image.rs:289:26)
capacity overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::region_by_pixels (epaint/src/image.rs:289:26)
  repro_epaint_src_image_rs_289_26_57844c::main (src/main.rs:9:25)</pre></td></tr>
<tr><td valign="top"><pre>let layout_job = LayoutJob { text: "".to_string(),
    sections: vec![LayoutSection { leading_space: 0.0f32,
    byte_range: ByteIndex::from(1usize)..ByteIndex::ZERO,
    format: TextFormat::default() }],
    wrap: TextWrapping::truncate_at_width(0.0f32), first_row_min_height: 0.0f32,
    break_on_newline: false, halign: Align::Center, justify: false,
    round_output_to_gui: false, keep_trailing_whitespace: false };
layout_job.debug_sanity_check();</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/text/text_layout_types.rs:255:13:
assertion `left == right` failed: First LayoutSection must start at byte 0
  left: ByteIndex(1)
 right: ByteIndex(0)
backtrace:
  &lt;epaint::text::text_layout_types::LayoutJob&gt;::debug_sanity_check (epaint/src/text/text_layout_types.rs:255:13)
  repro_epaint_src_text_text_layout_types_rs_255_13_9d81c7::main (src/main.rs:10:16)</pre></td></tr>
<tr><td valign="top"><pre>let visuals = Visuals::default();
let color32 = visuals.text_color();
let texture_manager = TextureManager::default();
let _ = texture_manager.meta(TextureManager::default().alloc("".to_string(),
    ImageData::Color(std::sync::Arc::new(ColorImage::filled([1usize,
    1000000000usize], color32))), TextureOptions::default()));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_texturemanager_meta_abe85e::main (src/main.rs:11)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>epaint::tessellator::path::add_circle_quadrant(&amp;mut vec![], Pos2::NAN, 1.0f32,
    f32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:631:26:
attempt to multiply with overflow
backtrace:
  epaint::tessellator::path::add_circle_quadrant (epaint/src/tessellator.rs:631:26)
  repro_epaint_src_tessellator_rs_631_26_e5af1f::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageDelta::full(ImageData::from(std::sync::Arc::new(ColorImage::filled(
    [1usize, 1000000000usize], Color32::MAGENTA))), TextureOptions::NEAREST);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_imagedelta_full_af9ef9::main (src/main.rs:8)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let color_image = ColorImage { size: [usize::MAX, 18446744073709551614usize],
    source_size: Vec2::from((1.0000001f32, -1.0f32)),
    pixels: vec![Color32::BLACK, Color32::GREEN,
    Color32::from_additive_luminance(86u8)] };
let _ = color_image.region_by_pixels([16777214usize, 65536usize], [31usize,
    8191usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:291:26:
attempt to multiply with overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::region_by_pixels (epaint/src/image.rs:291:26)
  repro_epaint_src_image_rs_291_26_447f6d::main (src/main.rs:7:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
let mut fonts_view = fonts.with_pixels_per_point(65536.0f32);
let _ = fonts_view.layout("a".to_string(), FontId::default(),
    Color32::LIGHT_YELLOW, 0.0f32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1178539393024 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::text::fonts::synthetic_tofu (src/text/fonts.rs:870)
  epaint::text::fonts::{impl#3}::new::{closure#0} (src/text/fonts.rs:524)
  epaint::text::glyph_atlas::{impl#6}::allocate_raster::{closure#1}&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::glyph_atlas::GlyphAtlas::allocate_raster&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::fonts::FontsImpl::fontless_cluster (src/text/fonts.rs:837)
  epaint::text::text_layout::segment_into_runs (src/text/text_layout.rs:1538)
  epaint::text::text_layout::layout_section (src/text/text_layout.rs:572)
  epaint::text::text_layout::layout (src/text/text_layout.rs:116)
  epaint::text::galley_cache::GalleyCache::layout_internal (src/text/galley_cache.rs:112)
  epaint::text::galley_cache::GalleyCache::layout (src/text/galley_cache.rs:134)
  epaint::text::fonts::FontsView::layout_job (epaint/src/text/fonts.rs:421)
  epaint::text::fonts::FontsView::layout (epaint/src/text/fonts.rs:453)
  repro_out_of_memory_in_epaint_fontsview_layout_cc6ffa::main (src/main.rs:10)
  (24 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Brush { fill_texture_id: TextureManager::default().alloc("".to_string(),
    ImageData::from(ColorImage::filled([1usize, 1000000000usize],
    Color32::BLACK)), TextureOptions::NEAREST_MIRRORED_REPEAT),
    uv: Rect::everything_below(0.0f32) };</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_epaint_brush_literal_7c6c25::main (src/main.rs:10)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::default());
let _ = fonts.layout_uncached_for_viewport(10000000000.0f32,
    ViewportKey::default(),
    std::sync::Arc::new(LayoutJob::simple_singleline("\"".to_string(),
    FontId::default(), Color32::from_rgb_additive(0u8, 0u8, 0u8))));</pre></td><td valign="top"><pre>out of memory: memory allocation of 12237977088 bytes failed
backtrace:
  vello_common::pixmap::Pixmap::new (src/pixmap.rs:82)
  epaint::text::font_face::FontCell::rasterize_glyph (src/text/font_face.rs:159)
  epaint::text::font_face::FontFace::rasterize_glyph (src/text/font_face.rs:673)
  epaint::text::glyph_atlas::{impl#6}::allocate_outline::{closure#0} (src/text/glyph_atlas.rs:305)
  epaint::text::glyph_atlas::GlyphAtlas::allocate_outline (src/text/glyph_atlas.rs:304)
  epaint::text::fonts::FontsImpl::allocate_glyph (src/text/fonts.rs:753)
  epaint::text::text_layout::layout_shaped_run (src/text/text_layout.rs:367)
  epaint::text::text_layout::layout_section (src/text/text_layout.rs:603)
  epaint::text::text_layout::layout (src/text/text_layout.rs:116)
  epaint::text::fonts::Fonts::layout_uncached_for_viewport (src/text/fonts.rs:291)
  repro_out_of_memory_in_epaint_fonts_layout_uncached_for_viewport_3383d6::main (src/main.rs:11)
  (22 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>epaint::tessellator::path::add_circle_quadrant(&amp;mut vec![],
    AreaState::default().left_top_pos(), 3.0f32, 467.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/tessellator.rs:636:47:
range start index 1868 out of range for slice of length 17
backtrace:
  &lt;[emath::vec2::Vec2] as core::ops::index::Index&lt;core::ops::range::RangeInclusive&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;[emath::vec2::Vec2; 17] as core::ops::index::Index&lt;core::ops::range::RangeInclusive&lt;usize&gt;&gt;&gt;::index (core/src/array/mod.rs:391:9)
  epaint::tessellator::path::add_circle_quadrant (epaint/src/tessellator.rs:636:47)
  repro_epaint_src_tessellator_rs_636_47_108ab1::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TextShape::new(Pos2 { x: 1.0f32, y: 0.0f32 },
    std::sync::Arc::new(Fonts::new(TextOptions::default(),
    FontDefinitions::empty()).layout_uncached(65536.0f32,
    std::sync::Arc::new(LayoutJob::simple("a".to_string(), FontId::new(1.0f32,
    FontFamily::Proportional), Color32::from_rgb(0u8, 0u8, 0u8), 0.0f32)))),
    Color32::PURPLE);</pre></td><td valign="top"><pre>out of memory: memory allocation of 6012928000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::text::fonts::synthetic_tofu (src/text/fonts.rs:870)
  epaint::text::fonts::{impl#3}::new::{closure#0} (src/text/fonts.rs:524)
  epaint::text::glyph_atlas::{impl#6}::allocate_raster::{closure#1}&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::glyph_atlas::GlyphAtlas::allocate_raster&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::fonts::FontsImpl::fontless_cluster (src/text/fonts.rs:837)
  epaint::text::text_layout::segment_into_runs (src/text/text_layout.rs:1538)
  epaint::text::text_layout::layout_section (src/text/text_layout.rs:572)
  epaint::text::text_layout::layout (src/text/text_layout.rs:116)
  epaint::text::fonts::Fonts::layout_uncached_for_viewport (src/text/fonts.rs:291)
  epaint::text::fonts::Fonts::layout_uncached (src/text/fonts.rs:277)
  repro_out_of_memory_in_epaint_textshape_new_a357e7::main (src/main.rs:12)
  (24 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut texture_manager = TextureManager::default();
texture_manager.set(TextureId::default(), TextureAtlas::new([1000000000usize,
    0usize], TextOptions::default()).take_delta().unwrap());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  epaint::texture_atlas::resize_to_min_height (src/texture_atlas.rs:268)
  epaint::texture_atlas::TextureAtlas::allocate (src/texture_atlas.rs:250)
  epaint::texture_atlas::TextureAtlas::new (src/texture_atlas.rs:95)
  repro_timeout_in_epaint_texturemanager_set_d56fcc::main (src/main.rs:8)</pre></td></tr>
<tr><td valign="top"><pre>let color32 = Color32::DARK_GRAY;
let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
let mut fonts_view = fonts.with_pixels_per_point_for_viewport(721.0f32,
    ViewportKey::default());
let _ = fonts_view.layout_no_wrap("a".to_string(), FontId::new(642.0f32,
    FontFamily::Monospace), color32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 299963273988 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::text::fonts::synthetic_tofu (src/text/fonts.rs:870)
  epaint::text::fonts::{impl#3}::new::{closure#0} (src/text/fonts.rs:524)
  epaint::text::glyph_atlas::{impl#6}::allocate_raster::{closure#1}&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::glyph_atlas::GlyphAtlas::allocate_raster&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::fonts::FontsImpl::fontless_cluster (src/text/fonts.rs:837)
  epaint::text::text_layout::segment_into_runs (src/text/text_layout.rs:1538)
  epaint::text::text_layout::layout_section (src/text/text_layout.rs:572)
  epaint::text::text_layout::layout (src/text/text_layout.rs:116)
  epaint::text::galley_cache::GalleyCache::layout_internal (src/text/galley_cache.rs:112)
  epaint::text::galley_cache::GalleyCache::layout (src/text/galley_cache.rs:134)
  epaint::text::fonts::FontsView::layout_job (epaint/src/text/fonts.rs:421)
  epaint::text::fonts::FontsView::layout_no_wrap (epaint/src/text/fonts.rs:462)
  repro_out_of_memory_in_epaint_fontsview_layout_no_wrap_75b4d1::main (src/main.rs:13)
  (24 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let color_image = ColorImage { size: [10000000000000usize,
    100000000000000000usize], source_size: Vec2::RIGHT, pixels: vec![] };
let color_image2 = color_image.clone().with_source_size(Vec2::RIGHT);
let _ = color_image2.region_by_pixels([1usize, 0usize], [1000000usize,
    10000usize]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 40000000000 bytes failed
backtrace:
  epaint::image::ColorImage::region_by_pixels (src/image.rs:289)
  repro_out_of_memory_in_epaint_colorimage_region_by_pixels_1d08c2::main (src/main.rs:7)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
context.copy_image(ColorImage::filled([1usize, 10000000000usize],
    Color32::from_rgba_unmultiplied_const(0u8, 0u8, 0u8, 0u8)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  repro_out_of_memory_in_egui_context_copy_image_8680fd::main (src/main.rs:7)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
let mut fonts2 = fonts.with_missing_glyph_policy(MissingGlyphPolicy::default());
let mut fonts_view = fonts2.with_pixels_per_point(16777216.0f32);
let _ = fonts_view.layout_delayed_color("a".to_string(),
    FontSelection::default().resolve_with_fallback(&amp;Style::default(),
    FontSelection::Style(TextStyle::Body)), 0.0f32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 66596975302148096 bytes failed
backtrace:
  epaint::image::ColorImage::filled (src/image.rs:79)
  epaint::text::fonts::synthetic_tofu (src/text/fonts.rs:870)
  epaint::text::fonts::{impl#3}::new::{closure#0} (src/text/fonts.rs:524)
  epaint::text::glyph_atlas::{impl#6}::allocate_raster::{closure#1}&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::glyph_atlas::GlyphAtlas::allocate_raster&lt;core::iter::sources::once::Once&lt;&amp;epaint::text::glyph_rasterizer::GlyphRasterizer&gt;&gt; (src/text/glyph_atlas.rs:352)
  epaint::text::fonts::FontsImpl::fontless_cluster (src/text/fonts.rs:837)
  epaint::text::text_layout::segment_into_runs (src/text/text_layout.rs:1538)
  epaint::text::text_layout::layout_section (src/text/text_layout.rs:572)
  epaint::text::text_layout::layout (src/text/text_layout.rs:116)
  epaint::text::galley_cache::GalleyCache::layout_internal (src/text/galley_cache.rs:112)
  epaint::text::galley_cache::GalleyCache::layout (src/text/galley_cache.rs:134)
  epaint::text::fonts::FontsView::layout_job (epaint/src/text/fonts.rs:421)
  epaint::text::fonts::FontsView::layout (epaint/src/text/fonts.rs:453)
  epaint::text::fonts::FontsView::layout_delayed_color (epaint/src/text/fonts.rs:475)
  repro_out_of_memory_in_epaint_fontsview_layout_delayed_color_67b46c::main (src/main.rs:13)
  (24 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut texture_atlas = TextureAtlas::new([65536usize, 1usize],
    TextOptions { max_texture_side: 0usize,
    color_transfer_function: FontColorTransferFunction::Gamma(0.0f32),
    font_hinting: false, subpixel_binning: false });
let _ = texture_atlas.allocate((1usize, 10000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  epaint::texture_atlas::resize_to_min_height (src/texture_atlas.rs:274)
  epaint::texture_atlas::TextureAtlas::allocate (src/texture_atlas.rs:250)
  repro_out_of_memory_in_epaint_textureatlas_allocate_86cb61::main (src/main.rs:7)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ImageData::Color(std::sync::Arc::new(ColorImage::from_rgb([usize::MAX,
    10usize], &amp;include_bytes!("image_0.bmp").to_vec())));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:195:13:
attempt to multiply with overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::from_rgb (epaint/src/image.rs:195:13)
  repro_epaint_src_image_rs_195_13_af6da1::main (src/main.rs:5:50)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TextureManager::default().alloc("".to_string(),
    ImageData::from(std::sync::Arc::new(ColorImage::from_rgba_unmultiplied(
    [1usize, usize::MAX], &amp;include_bytes!("image_0.webp").to_vec()))),
    TextureOptions::NEAREST_REPEAT);</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:115:13:
attempt to multiply with overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::from_rgba_unmultiplied (epaint/src/image.rs:115:13)
  repro_epaint_src_image_rs_115_13_dd6695::main (src/main.rs:7:97)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let mut ui = Ui::new(context.clone(), Id::NULL,
    UiBuilder::default().layout(Layout::left_to_right(Align::Max)));
ui.set_height_range(Size::relative(1.0f32).range());</pre></td><td valign="top"><pre>thread 'main' panicked at egui/src/layout.rs:632:9:
frame_rect is NaN: [[NaN NaN] - [NaN NaN]]
backtrace:
  &lt;egui::layout::Layout&gt;::next_frame_ignore_wrap (egui/src/layout.rs:632:9)
  &lt;egui::layout::Layout&gt;::next_widget_space_ignore_wrap_justify (egui/src/layout.rs:660:26)
  &lt;egui::placer::Placer&gt;::next_widget_space_ignore_wrap_justify (egui/src/placer.rs:228:14)
  &lt;egui::placer::Placer&gt;::set_max_height (egui/src/placer.rs:249:25)
  &lt;egui::ui::Ui&gt;::set_max_height (egui/src/ui.rs:751:21)
  &lt;egui::ui::Ui&gt;::set_height_range::&lt;emath::range::Rangef&gt; (egui/src/ui.rs:838:14)
  repro_egui_src_layout_rs_632_9_c74234::main (src/main.rs:12:8)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ColorImage::from_gray([1000000000usize, usize::MAX],
    &amp;vec![]).with_source_size(emath::vec2(f32::INFINITY, f32::NEG_INFINITY));</pre></td><td valign="top"><pre>thread 'main' panicked at epaint/src/image.rs:148:13:
attempt to multiply with overflow
backtrace:
  &lt;epaint::image::ColorImage&gt;::from_gray (epaint/src/image.rs:148:13)
  repro_epaint_src_image_rs_148_13_cbfd61::main (src/main.rs:4:13)</pre></td></tr>
</table>
