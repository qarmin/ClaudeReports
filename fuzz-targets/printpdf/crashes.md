# printpdf - crashes

Source: https://github.com/fschutt/printpdf @ 5cc7cfd6d37c06862543952fb93aaeb6d1f99bb6

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let px = &lt;printpdf::units::Px as ::core::ops::Sub&lt;printpdf::units::Px&gt;&gt;::sub(Px(
    0usize), Pt::default().into_px(0.0f32));
let pt = px.into_pt(0.0f32);
assert!(pt == pt, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:7:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_printpdf_units_pt_81303c::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;printpdf::units::Px as ::core::ops::Sub&lt;printpdf::units::Px&gt;&gt;::sub(Px(
    0usize), Px(1usize));</pre></td><td valign="top"><pre>thread 'main' panicked at printpdf/src/units.rs:228:1:
attempt to subtract with overflow
backtrace:
  &lt;printpdf::units::Px as core::ops::arith::Sub&gt;::sub (printpdf/src/units.rs:159:24)
  repro_printpdf_src_units_rs_228_1_377846::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = printpdf::font::get_normalized_widths_ttf(&amp;ParsedFont::from_azul(
    BuiltinFont::default().get_parsed_font().unwrap().into_inner()),
    &amp;[(u16::MAX, "".to_string()), (0u16, "".to_string())]);</pre></td><td valign="top"><pre>thread 'main' panicked at printpdf/src/font.rs:1626:27:
attempt to add with overflow
backtrace:
  printpdf::font::get_normalized_widths_ttf (printpdf/src/font.rs:1626:27)
  repro_printpdf_src_font_rs_1626_27_4cddd3::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let mm = Mm(f32::NAN);
assert!(mm == mm, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:5:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_printpdf_units_mm_84ca17::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut raw_image = RawImage::empty(1000000000000000000usize, 10000usize,
    RawImageFormat::R8);
let _ = raw_image.optimize(&amp;ImageOptimizationOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at printpdf/src/image.rs:1091:10:
attempt to multiply with overflow
backtrace:
  &lt;printpdf::image_types::RawImage&gt;::estimate_size_bytes (printpdf/src/image.rs:1091:10)
  &lt;printpdf::image_types::RawImage&gt;::optimize (printpdf/src/image.rs:630:37)
  repro_printpdf_src_image_rs_1091_10_cdad29::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = printpdf::font::get_normalized_widths_codes(
    &amp;BuiltinFont::default().get_parsed_font().unwrap(), &amp;[(u16::MAX, 0u16),
    (0u16, 0u16)]);</pre></td><td valign="top"><pre>thread 'main' panicked at printpdf/src/font.rs:1677:27:
attempt to add with overflow
backtrace:
  printpdf::font::get_normalized_widths_codes (printpdf/src/font.rs:1677:27)
  repro_printpdf_src_font_rs_1677_27_d7ec6d::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;printpdf::units::Px as ::core::ops::Add&lt;printpdf::units::Px&gt;&gt;::add(Px(
    usize::MAX), Px(1usize));</pre></td><td valign="top"><pre>thread 'main' panicked at printpdf/src/units.rs:216:1:
attempt to add with overflow
backtrace:
  &lt;printpdf::units::Px as core::ops::arith::Add&gt;::add (printpdf/src/units.rs:126:24)
  repro_printpdf_src_units_rs_216_1_07ac6b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut raw_image = RawImage::empty(0usize, 1000000000000000000usize,
    RawImageFormat::R16);
let _ = raw_image.optimize(&amp;ImageOptimizationOptions { quality: None,
    max_image_size: None, dither_greyscale: Some(true),
    convert_to_greyscale: None, auto_optimize: None, format: None });</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from printpdf/src/image.rs:941:32)
capacity overflow
backtrace:
  &lt;printpdf::image_types::RawImage&gt;::apply_dithering (printpdf/src/image.rs:941:32)
  &lt;printpdf::image_types::RawImage&gt;::apply_dithering (printpdf/src/image.rs:1057:26)
  &lt;printpdf::image_types::RawImage&gt;::optimize (printpdf/src/image.rs:621:18)
  repro_printpdf_src_image_rs_941_32_0ff0fa::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Svg::parse("&lt;svg xmlns=\"http://www.w3.org/2000/svg\" wiht=\" \
    41\"&gt;&lt;defs&gt;&lt;linearGradient \
    id=\"g\"&gt;&lt;/linearGradient&gt;&lt;fi" /* ... 230 more chars */, &amp;mut vec![]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 72997232400 bytes failed
backtrace:
  tiny_skia::pixmap::Pixmap::new (src/pixmap.rs:51)
  svg2pdf::render::filter::render (src/render/filter.rs:30)
  svg2pdf::render::group::render (src/render/group.rs:28)
  svg2pdf::render::{impl#0}::render (src/render/mod.rs:98)
  svg2pdf::render::group::create_to_stream (src/render/group.rs:140)
  svg2pdf::render::group::render (src/render/group.rs:72)
  svg2pdf::render::tree_to_stream (src/render/mod.rs:39)
  svg2pdf::to_pdf (src/lib.rs:230)
  printpdf::svg::Svg::parse_with_fonts (src/svg.rs:120)
  printpdf::svg::Svg::parse (src/svg.rs:28)
  repro_out_of_memory_in_printpdf_svg_svg_parse_ab4138::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = XObject::External(Svg::parse("&lt;svg xmlns=\"http://www.w3.org/2000/svg\" \
    h=\"1\"&gt;&lt;defs&gt;&lt;filter id=\"f\"&gt;&lt;feGaussianBlur \
    sion=\"2\"/&gt;&lt;/filte" /* ... 113 more chars */, &amp;mut vec![]).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 1943999723520 bytes failed
backtrace:
  tiny_skia::pixmap::Pixmap::new (src/pixmap.rs:51)
  svg2pdf::render::filter::render (src/render/filter.rs:30)
  svg2pdf::render::group::render (src/render/group.rs:28)
  svg2pdf::render::{impl#0}::render (src/render/mod.rs:98)
  svg2pdf::render::group::create_to_stream (src/render/group.rs:140)
  svg2pdf::render::group::render (src/render/group.rs:72)
  svg2pdf::render::tree_to_stream (src/render/mod.rs:39)
  svg2pdf::to_pdf (src/lib.rs:230)
  printpdf::svg::Svg::parse_with_fonts (src/svg.rs:120)
  printpdf::svg::Svg::parse (src/svg.rs:28)
  repro_out_of_memory_in_printpdf_xobject_xobject_external_357324::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = printpdf::html::xml_to_pdf_pages_debug("&lt;svg \
    xmlns=\"http://www.w3.org/2000/svg\" width=\"NaN\" height=\"0\" viewBox=\"0 \
    0 -1 NaN\"&gt;&lt;defs&gt;&lt;linearGra" /* ... 1180 more chars */,
    &amp;XmlRenderOptions::default());</pre></td><td valign="top"><pre>timeout: abnormal termination
backtrace:
  agg_rust_azul::rasterizer_cells_aa::RasterizerCellsAa::add_curr_cell (src/rasterizer_cells_aa.rs:205)
  agg_rust_azul::rasterizer_cells_aa::RasterizerCellsAa::set_curr_cell (src/rasterizer_cells_aa.rs:252)
  agg_rust_azul::rasterizer_cells_aa::RasterizerCellsAa::render_hline (src/rasterizer_cells_aa.rs:314)
  agg_rust_azul::rasterizer_cells_aa::RasterizerCellsAa::line (src/rasterizer_cells_aa.rs:490)
  agg_rust_azul::rasterizer_cells_aa::RasterizerCellsAa::line (src/rasterizer_cells_aa.rs:360)
... (previous line repeated 8 more times)
  agg_rust_azul::rasterizer_sl_clip::RasterizerSlClipInt::line_to (src/rasterizer_sl_clip.rs:181)
  agg_rust_azul::rasterizer_scanline_aa::RasterizerScanlineAa::line_to_d (src/rasterizer_scanline_aa.rs:186)
  agg_rust_azul::rasterizer_scanline_aa::RasterizerScanlineAa::add_vertex (src/rasterizer_scanline_aa.rs:195)
  agg_rust_azul::rasterizer_scanline_aa::RasterizerScanlineAa::add_path (src/rasterizer_scanline_aa.rs:236)
  azul_layout::solver3::display_list::rasterize_svg_clip_to_r8 (src/solver3/display_list.rs:10568)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::push_image_mask_clip&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:5129)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::paint_in_flow_descendants&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:4764)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::paint_in_flow_descendants&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:4775)
... (previous line repeated 3 more times)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::generate_for_stacking_context&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:4552)
  azul_layout::solver3::display_list::generate_display_list_impl&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:3163)
  azul_layout::solver3::display_list::generate_display_list&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:3048)
  azul_layout::solver3::paged_layout::layout_document_paged_impl&lt;azul_css::props::basic::font::FontRef, printpdf::html::xml_to_pdf_pages_debug::{closure_env#0}&gt; (azul-layout/src/solver3/paged_layout.rs:534)
  azul_layout::solver3::paged_layout::layout_document_paged_v2&lt;azul_css::props::basic::font::FontRef, printpdf::html::xml_to_pdf_pages_debug::{closure_env#0}&gt; (azul-layout/src/solver3/paged_layout.rs:227)
  printpdf::html::xml_to_pdf_pages_debug (src/html/mod.rs:877)
  repro_timeout_in_printpdf_html_xml_to_pdf_pages_debug_ce754a::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = RawImageData::U8(include_bytes!("document_0.pdf").to_vec());
let _ = XmpMetadata { rendition_class: None };
let _ = ColorSpace::Cmyk;
let _ = printpdf::html::xml_to_pdf_pages("&lt;svg \
    xmlns=\"http://\\uwww.w3.org/2000/svg\" width=\"33\" height=\"153\" \
    viewBox=\"0 0 71 1e9\"&gt;&lt;defs&gt;&lt;linear" /* ... 1223 more chars */,
    &amp;XmlRenderOptions::default());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:832)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 7 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
... (previous line repeated 2 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 3 more times)
  agg_rust_azul::curves::Curve4Div::bezier (src/curves.rs:790)
  agg_rust_azul::curves::Curve4Div::init (src/curves.rs:751)
  agg_rust_azul::curves::Curve4::init (src/curves.rs:1158)
  agg_rust_azul::conv_curve::{impl#1}::vertex&lt;&amp;mut agg_rust_azul::path_storage::PathStorage&gt; (agg-rust-azul/src/conv_curve.rs:127)
  agg_rust_azul::rasterizer_scanline_aa::RasterizerScanlineAa::add_path (src/rasterizer_scanline_aa.rs:232)
  azul_layout::solver3::display_list::rasterize_svg_clip_to_r8 (src/solver3/display_list.rs:10568)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::push_image_mask_clip&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:5129)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::paint_in_flow_descendants&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:4764)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::paint_in_flow_descendants&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:4775)
... (previous line repeated 4 more times)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::generate_for_stacking_context&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:4552)
  azul_layout::solver3::display_list::generate_display_list_impl&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:3163)
  azul_layout::solver3::display_list::generate_display_list&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:3048)
  azul_layout::solver3::paged_layout::layout_document_paged_impl&lt;azul_css::props::basic::font::FontRef, printpdf::html::xml_to_pdf_pages::{closure_env#0}&gt; (azul-layout/src/solver3/paged_layout.rs:534)
  azul_layout::solver3::paged_layout::layout_document_paged_v2&lt;azul_css::props::basic::font::FontRef, printpdf::html::xml_to_pdf_pages::{closure_env#0}&gt; (azul-layout/src/solver3/paged_layout.rs:227)
  printpdf::html::xml_to_pdf_pages (src/html/mod.rs:589)
  repro_timeout_in_printpdf_html_xml_to_pdf_pages_becf11::main (src/main.rs:10)</pre></td></tr>
<tr><td valign="top"><pre>let mut raw_image = RawImage::empty(0usize, 1000000000usize,
    RawImageFormat::RG8);
let _ = raw_image.optimize(&amp;ImageOptimizationOptions { quality: None,
    max_image_size: None, dither_greyscale: Some(true),
    convert_to_greyscale: None, auto_optimize: None, format: None });</pre></td><td valign="top"><pre>out of memory: memory allocation of 24000000000 bytes failed
backtrace:
  printpdf::image_types::RawImage::apply_dithering (src/image.rs:941)
  printpdf::image_types::RawImage::apply_dithering (src/image.rs:1022)
  printpdf::image_types::RawImage::optimize (src/image.rs:621)
  repro_out_of_memory_in_printpdf_image_types_rawimage_optimize_c601b6::main (src/main.rs:7)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut raw_image = RawImage::empty(0usize, 1000000000usize,
    RawImageFormat::RG8);
let _ = raw_image.apply_dithering();</pre></td><td valign="top"><pre>out of memory: memory allocation of 24000000000 bytes failed
backtrace:
  printpdf::image_types::RawImage::apply_dithering (src/image.rs:941)
  printpdf::image_types::RawImage::apply_dithering (src/image.rs:1022)
  repro_out_of_memory_in_printpdf_image_types_rawimage_apply_ditheri_86bd8c::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut raw_image = RawImage::empty(10000000000000000000usize, 5usize,
    RawImageFormat::RG8);
let _ = raw_image.optimize(&amp;ImageOptimizationOptions { quality: Some(
    1.1920929e-7f32), max_image_size: Some("Zażółć gęślą jaźń".to_string()),
    dither_greyscale: Some(true), convert_to_greyscale: Some(true),
    auto_optimize: Some(true), format: Some(ImageCompression::Flate) });</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from printpdf/src/image.rs:945:35)
capacity overflow
backtrace:
  &lt;printpdf::image_types::RawImage&gt;::apply_dithering (printpdf/src/image.rs:945:35)
  &lt;printpdf::image_types::RawImage&gt;::apply_dithering (printpdf/src/image.rs:1022:26)
  &lt;printpdf::image_types::RawImage&gt;::optimize (printpdf/src/image.rs:621:18)
  repro_printpdf_src_image_rs_1022_26_e6246d::main (src/main.rs:8:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Svg::parse("&lt;svg xmlns=\"http://www.w3.org/2000/svg\" width=\"NaN\" \
    height=\"1e9\" viewBox=\"0 0 84 \
    1e9\"&gt;&lt;defs&gt;&lt;linearG" /* ... 1209 more chars */,
    &amp;mut vec![PdfWarnMsg::info(0usize, 2usize, "M".to_string()),
    PdfWarnMsg::info(1usize, 9usize, "&lt;svg \
    xm&amp;amplns=\"http://www.w3.org/2000/svg\"".to_string()),
    PdfWarnMsg { page: 1usize, op_id: 9usize,
    severity: PdfParseErrorSeverity::Warning,
    msg: "_[;t\n.=\0:\u{200b}aik8r媲q$%V#}\u{50d79}\u{97b6f}".to_string() }]);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  bytemuck::internal::try_cast_slice_mut&lt;u8, tiny_skia::color::PremultipliedColorU8&gt; (bytemuck/src/internal.rs:384)
  bytemuck::internal::cast_slice_mut&lt;u8, tiny_skia::color::PremultipliedColorU8&gt; (bytemuck/src/internal.rs:269)
  bytemuck::cast_slice_mut&lt;u8, tiny_skia::color::PremultipliedColorU8&gt; (bytemuck/src/lib.rs:429)
  tiny_skia::pixmap::SubPixmapMut::pixels_mut (src/pixmap.rs:577)
  tiny_skia::pixmap::SubPixmapMut::slice4_at_xy (src/pipeline/mod.rs:179)
  tiny_skia::pipeline::highp::source_over_rgba (src/pipeline/highp.rs:653)
  tiny_skia::pipeline::highp::Pipeline::next_stage (src/pipeline/highp.rs:54)
  tiny_skia::pipeline::highp::gather (src/pipeline/highp.rs:296)
  tiny_skia::pipeline::highp::Pipeline::next_stage (src/pipeline/highp.rs:54)
  tiny_skia::pipeline::highp::transform (src/pipeline/highp.rs:684)
  tiny_skia::pipeline::highp::Pipeline::next_stage (src/pipeline/highp.rs:54)
  tiny_skia::pipeline::highp::seed_shader (src/pipeline/highp.rs:252)
  tiny_skia::pipeline::highp::Pipeline::next_stage (src/pipeline/highp.rs:54)
  tiny_skia::pipeline::highp::start (src/pipeline/highp.rs:170)
  tiny_skia::pipeline::RasterPipeline::run (src/pipeline/mod.rs:497)
  tiny_skia::pipeline::blitter::{impl#1}::blit_rect (src/pipeline/blitter.rs:336)
  tiny_skia::pipeline::blitter::{impl#1}::blit_h (src/pipeline/blitter.rs:233)
  tiny_skia::scan::path::walk_edges (src/scan/path.rs:235)
  tiny_skia::scan::path::fill_path_impl (src/scan/path.rs:188)
  tiny_skia::scan::path::fill_path (src/scan/path.rs:43)
  tiny_skia::pixmap::PixmapMut::fill_path (src/painter.rs:273)
  tiny_skia::pixmap::PixmapMut::fill_rect (src/painter.rs:209)
  tiny_skia::pixmap::PixmapMut::draw_pixmap (src/painter.rs:500)
  resvg::render::render_group (src/render.rs:128)
  resvg::render::render_node (src/render.rs:29)
  resvg::render::render_nodes (src/render.rs:17)
  resvg::render::render_group (src/render.rs:106)
  resvg::render::render_node (src/render.rs:29)
  resvg::render_node (src/lib.rs:81)
  svg2pdf::render::filter::render (src/render/filter.rs:47)
  svg2pdf::render::group::render (src/render/group.rs:28)
  svg2pdf::render::{impl#0}::render (src/render/mod.rs:98)
  svg2pdf::render::group::create_to_stream (src/render/group.rs:140)
  svg2pdf::render::group::render (src/render/group.rs:72)
  svg2pdf::render::tree_to_stream (src/render/mod.rs:39)
  svg2pdf::to_pdf (src/lib.rs:230)
  printpdf::svg::Svg::parse_with_fonts (src/svg.rs:120)
  printpdf::svg::Svg::parse (src/svg.rs:28)
  repro_timeout_in_printpdf_svg_svg_parse_c44cac::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let mut pdf_document = PdfDocument { metadata: PdfMetadata { info: PdfDocumentInfo::default(),
    xmp: None }, resources: PdfResources { fonts: PdfFontMap::default(),
    xobjects: XObjectMap::default(),
    extgstates: ExtendedGraphicsStateMap::default(),
    shadings: ShadingMap::default(), layers: PdfLayerMap::default() },
    bookmarks: PageAnnotMap::default(), pages: vec![] };
let _ = printpdf::html::add_xml_to_document(&amp;mut pdf_document, "&lt;svg \
    xmlns=\"http://www.w3.org/2000/svg\" width=\"23\" height=\"191\" \
    viewBox=\"0 0 105 0\"&gt;&lt;defs&gt;&lt;linearGra" /* ... 265 more chars */,
    &amp;XmlRenderOptions::default());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:820)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
... (previous line repeated 4 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 2 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
... (previous line repeated 2 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 1 more times)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:978)
  agg_rust_azul::curves::Curve4Div::recursive_bezier (src/curves.rs:977)
... (previous line repeated 3 more times)
  agg_rust_azul::curves::Curve4Div::bezier (src/curves.rs:790)
  agg_rust_azul::curves::Curve4Div::init (src/curves.rs:751)
  agg_rust_azul::curves::Curve4::init (src/curves.rs:1158)
  agg_rust_azul::conv_curve::{impl#1}::vertex&lt;&amp;mut agg_rust_azul::path_storage::PathStorage&gt; (agg-rust-azul/src/conv_curve.rs:127)
  agg_rust_azul::rasterizer_scanline_aa::RasterizerScanlineAa::add_path (src/rasterizer_scanline_aa.rs:232)
  azul_layout::solver3::display_list::rasterize_svg_clip_to_r8 (src/solver3/display_list.rs:10568)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::push_image_mask_clip&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:5129)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::paint_in_flow_descendants&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:4764)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::paint_in_flow_descendants&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:4775)
... (previous line repeated 1 more times)
  azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;::generate_for_stacking_context&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:4552)
  azul_layout::solver3::display_list::generate_display_list_impl&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:3163)
  azul_layout::solver3::display_list::generate_display_list&lt;azul_css::props::basic::font::FontRef&gt; (src/solver3/display_list.rs:3048)
  azul_layout::solver3::paged_layout::layout_document_paged_impl&lt;azul_css::props::basic::font::FontRef, printpdf::html::xml_to_pdf_pages::{closure_env#0}&gt; (azul-layout/src/solver3/paged_layout.rs:534)
  azul_layout::solver3::paged_layout::layout_document_paged_v2&lt;azul_css::props::basic::font::FontRef, printpdf::html::xml_to_pdf_pages::{closure_env#0}&gt; (azul-layout/src/solver3/paged_layout.rs:227)
  printpdf::html::xml_to_pdf_pages (src/html/mod.rs:589)
  printpdf::html::add_xml_to_document (src/html/mod.rs:1086)
  repro_timeout_in_printpdf_html_add_xml_to_document_9c3016::main (src/main.rs:15)</pre></td></tr>
<tr><td valign="top"><pre>let mut pdf_document = PdfDocument::new("");
let _ = pdf_document.add_xobject(&amp;Svg::parse("&lt;svg \
    xmlns=\"http://www.w3.org/2000/svg\" x=\"0\"&gt;&lt;defs&gt;&gt;&lt;filter \
    id=\"f\"&gt;&lt;feGaussianBlur /&gt;&lt;/filter&gt;&lt;/def" /* ... 106 more chars */,
    &amp;mut vec![]).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 633600000000 bytes failed
backtrace:
  tiny_skia::pixmap::Pixmap::new (src/pixmap.rs:51)
  svg2pdf::render::filter::render (src/render/filter.rs:30)
  svg2pdf::render::group::render (src/render/group.rs:28)
  svg2pdf::render::{impl#0}::render (src/render/mod.rs:98)
  svg2pdf::render::group::create_to_stream (src/render/group.rs:140)
  svg2pdf::render::group::render (src/render/group.rs:72)
  svg2pdf::render::tree_to_stream (src/render/mod.rs:39)
  svg2pdf::to_pdf (src/lib.rs:230)
  printpdf::svg::Svg::parse_with_fonts (src/svg.rs:120)
  printpdf::svg::Svg::parse (src/svg.rs:28)
  repro_out_of_memory_in_printpdf_pdfdocument_add_xobject_106f2b::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut raw_image = RawImage::empty(1usize, 4294967296usize,
    RawImageFormat::R8);
let _ = raw_image.resize_to_fit_size(2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3037000500 bytes failed
backtrace:
  printpdf::image_types::RawImage::resize_to_fit_size (src/image.rs:1216)
  repro_out_of_memory_in_printpdf_image_types_rawimage_resize_to_fit_88343e::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut raw_image = RawImage::empty(usize::MAX, 15usize, RawImageFormat::R8);
let _ = raw_image.optimize(&amp;ImageOptimizationOptions { quality: Some(
    0.9999999f32), max_image_size: Some("😀👍🏽".to_string()),
    dither_greyscale: Some(true), convert_to_greyscale: None,
    auto_optimize: Some(false), format: Some(ImageCompression::Auto) });</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from printpdf/src/image.rs:945:35)
capacity overflow
backtrace:
  &lt;printpdf::image_types::RawImage&gt;::apply_dithering (printpdf/src/image.rs:945:35)
  &lt;printpdf::image_types::RawImage&gt;::optimize (printpdf/src/image.rs:621:18)
  repro_printpdf_src_image_rs_621_18_314089::main (src/main.rs:8:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut raw_image = RawImage::empty(10000000000000000000usize, 65536usize,
    RawImageFormat::R16);
let _ = raw_image.apply_dithering();</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from printpdf/src/image.rs:945:35)
capacity overflow
backtrace:
  &lt;printpdf::image_types::RawImage&gt;::apply_dithering (printpdf/src/image.rs:945:35)
  &lt;printpdf::image_types::RawImage&gt;::apply_dithering (printpdf/src/image.rs:1057:26)
  repro_printpdf_src_image_rs_1057_26_3c1ddf::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut pdf_document = PdfDocument::default();
let _ = pdf_document.add_bookmark("", 0usize);
pdf_document.save_writer::&lt;Vec&lt;u8&gt;&gt;(&amp;mut vec![],
    &amp;PdfSaveOptions { optimize: false, subset_fonts: false, secure: false,
    image_optimization: None }, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at printpdf/src/serialize.rs:370:55:
called `Option::unwrap()` on a `None` value
backtrace:
  printpdf::serialize::to_lopdf_doc (printpdf/src/serialize.rs:370:55)
  printpdf::serialize::serialize_pdf::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (printpdf/src/serialize.rs:96:19)
  &lt;printpdf::PdfDocument&gt;::save_writer::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (printpdf/src/lib.rs:758:9)
  repro_printpdf_src_serialize_rs_370_55_e58ce5::main (src/main.rs:7:18)</pre></td></tr>
<tr><td valign="top"><pre>let mut pdf_document = PdfDocument::parse(&amp;include_bytes!(
    "data_0.bin").to_vec(), &amp;PdfParseOptions::default(), &amp;mut vec![]).unwrap();
let _ = printpdf::html::add_xml_to_document(&amp;mut pdf_document, "&lt;svg xs=\"vg\" \
    width=\"1e9\" height=\"55\" vx=\"0N\"&gt;&lt;circle cx=\"1\" r=\"0.6\"/&gt;&lt;/svg&gt;",
    &amp;XmlRenderOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at azul-layout/src/solver3/display_list.rs:10553:35:
attempt to multiply with overflow
backtrace:
  azul_layout::solver3::display_list::rasterize_svg_clip_to_r8 (azul-layout/src/solver3/display_list.rs:10553:35)
  &lt;azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;&gt;::push_image_mask_clip (azul-layout/src/solver3/display_list.rs:5129:17)
  &lt;azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;&gt;::paint_in_flow_descendants (azul-layout/src/solver3/display_list.rs:4764:50)
  &lt;azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;&gt;::paint_in_flow_descendants (azul-layout/src/solver3/display_list.rs:4775:18)
... (previous line repeated 1 more times)
  &lt;azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;&gt;::generate_for_stacking_context (azul-layout/src/solver3/display_list.rs:4552:14)
  azul_layout::solver3::display_list::generate_display_list_impl::&lt;azul_css::props::basic::font::FontRef&gt; (azul-layout/src/solver3/display_list.rs:3163:15)
  azul_layout::solver3::display_list::generate_display_list::&lt;azul_css::props::basic::font::FontRef&gt; (azul-layout/src/solver3/display_list.rs:3048:5)
  azul_layout::solver3::paged_layout::layout_document_paged_impl::&lt;azul_css::props::basic::font::FontRef, printpdf (azul-layout/src/solver3/paged_layout.rs:534:29)
  azul_layout::solver3::paged_layout::layout_document_paged_v2::&lt;azul_css::props::basic::font::FontRef, printpdf (azul-layout/src/solver3/paged_layout.rs:227:5)
  printpdf (printpdf/src/html/mod.rs:589:31)
  printpdf (printpdf/src/html/mod.rs:1086:11)</pre></td></tr>
<tr><td valign="top"><pre>let _ = printpdf::html::xml_to_pdf_pages("&lt;svg xs=\"vg\" width=\"1e9\" \
    height=\"1\" x=\"0 \"&gt;&lt;path d=\"M1 7Z\" s=\"d\"/&gt;&lt;/svg&gt;",
    &amp;XmlRenderOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at azul-layout/src/solver3/display_list.rs:10553:34:
attempt to multiply with overflow
backtrace:
  azul_layout::solver3::display_list::rasterize_svg_clip_to_r8 (azul-layout/src/solver3/display_list.rs:10553:34)
  &lt;azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;&gt;::push_image_mask_clip (azul-layout/src/solver3/display_list.rs:5129:17)
  &lt;azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;&gt;::paint_in_flow_descendants (azul-layout/src/solver3/display_list.rs:4764:50)
  &lt;azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;&gt;::paint_in_flow_descendants (azul-layout/src/solver3/display_list.rs:4775:18)
... (previous line repeated 1 more times)
  &lt;azul_layout::solver3::display_list::DisplayListGenerator&lt;azul_css::props::basic::font::FontRef&gt;&gt;::generate_for_stacking_context (azul-layout/src/solver3/display_list.rs:4552:14)
  azul_layout::solver3::display_list::generate_display_list_impl::&lt;azul_css::props::basic::font::FontRef&gt; (azul-layout/src/solver3/display_list.rs:3163:15)
  azul_layout::solver3::display_list::generate_display_list::&lt;azul_css::props::basic::font::FontRef&gt; (azul-layout/src/solver3/display_list.rs:3048:5)
  azul_layout::solver3::paged_layout::layout_document_paged_impl::&lt;azul_css::props::basic::font::FontRef, printpdf (azul-layout/src/solver3/paged_layout.rs:534:29)
  azul_layout::solver3::paged_layout::layout_document_paged_v2::&lt;azul_css::props::basic::font::FontRef, printpdf (azul-layout/src/solver3/paged_layout.rs:227:5)
  printpdf (printpdf/src/html/mod.rs:589:31)
  repro_azul_layout_src_solver3_display_list_rs_10553_34_cd006e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let raw_image = RawImage { pixels: RawImageData::U8(vec![119u8]), width: 0usize,
    height: 0usize, data_format: RawImageFormat::RG8, tag: vec![] };
let _ = raw_image.encode_to_bytes(&amp;[OutputImageFormat::Pnm]);</pre></td><td valign="top"><pre>thread 'main' panicked at image/src/io/encoder.rs:115:17:
assertion `left == right` failed: Invalid buffer length: expected 0 got 1 for 0x0 image
  left: 0
 right: 1
backtrace:
  &lt;image::codecs::pnm::encoder::PnmEncoder&lt;&amp;mut &amp;mut core::io::cursor::Cursor&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt;&gt; as image::io::encoder::ImageEncoder&gt;::write_image (image/src/codecs/pnm/encoder.rs:347:9)
  &lt;image::codecs::pnm::encoder::PnmEncoder&lt;&amp;mut &amp;mut core::io::cursor::Cursor&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt;&gt; as image::io::encoder::ImageEncoderBoxed&gt;::write_image (image/src/io/encoder.rs:115:17)
  &lt;image::images::dynimage::DynamicImage&gt;::write_with_encoder_impl (image/src/images/dynimage.rs:1353:17)
  &lt;image::images::dynimage::DynamicImage&gt;::write_to::&lt;&amp;mut core::io::cursor::Cursor&lt;&amp;mut alloc::vec::Vec&lt;u8&gt;&gt;&gt; (image/src/images/dynimage.rs:1373:14)
  &lt;printpdf::image_types::RawImage&gt;::encode_to_bytes (printpdf/src/image.rs:584:18)
  repro_image_src_io_encoder_rs_115_17_a81cb6::main (src/main.rs:8:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ParsedFont::from_bytes(&amp;include_bytes!("data_0.bin").to_vec(), 1usize,
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts-azul/src/font.rs:1213:25:
range start index 4194324 out of range for slice of length 60
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;allsorts::font::Font&lt;azul_layout::font::parsed::ManualTableProvider&gt;&gt;::cmap_subtable_data (allsorts-azul/src/font.rs:1213:25)
  &lt;azul_layout::font::parsed::ParsedFont&gt;::from_provider::&lt;azul_layout::font::parsed::ManualTableProvider&gt; (azul-layout/src/font.rs:1319:44)
  &lt;azul_layout::font::parsed::ParsedFont&gt;::from_bytes_internal (azul-layout/src/font.rs:974:25)
  &lt;azul_layout::font::parsed::ParsedFont&gt;::from_bytes (azul-layout/src/font.rs:910:28)
  &lt;printpdf::font::parsed_font::ParsedFont&gt;::from_bytes (printpdf/src/font.rs:96:25)
  repro_allsorts_azul_src_font_rs_1213_25_2bd0ab::main (src/main.rs:4:13)</pre></td></tr>
</table>
