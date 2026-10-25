# krilla - crashes

Source: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let radial_gradient = RadialGradient { fx: 0.0f32, fy: 0.0f32, fr: 0.0f32,
    cx: f32::NAN, cy: 0.0f32, cr: 0.0f32, transform: Transform::identity(),
    spread_method: SpreadMethod::Pad, stops: vec![], anti_alias: false };
assert!(radial_gradient == radial_gradient, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:7:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_krilla_paint_radialgradient_36f008::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;krilla::form::FormField&lt;krilla::form::kind::Checkbox&gt;&gt;::checkbox(
    ".".to_string(), false);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interactive/form.rs:395:9:
field name cannot contain a period
backtrace:
  &lt;krilla::interactive::form::FormField&lt;krilla::interactive::form::kind::Checkbox&gt;&gt;::checkbox (krilla/src/interactive/form.rs:395:9)
  repro_krilla_src_interactive_form_rs_395_9_944e98::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let fill = Fill { paint: Paint::from(SweepGradient { cx: 0.0f32, cy: f32::NAN,
    start_angle: 0.0f32, end_angle: 0.0f32, transform: Transform::identity(),
    spread_method: SpreadMethod::default(), stops: vec![], anti_alias: false }),
    opacity: NormalizedF32::ZERO, rule: FillRule::NonZero };
assert!(fill == fill, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:11:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_krilla_paint_fill_a9cdc2::main (src/main.rs:11:5)</pre></td></tr>
<tr><td valign="top"><pre>let sweep_gradient = SweepGradient { cx: 0.0f32, cy: 0.0f32,
    start_angle: f32::NAN, end_angle: 0.0f32,
    transform: Transform::from_rotate(0.0f32),
    spread_method: SpreadMethod::Repeat, stops: vec![], anti_alias: false };
assert!(sweep_gradient == sweep_gradient, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:7:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_krilla_paint_sweepgradient_f29103::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>let indent = Indent(1000000000000000000usize);
let _ = indent.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interchange/tagging/fmt.rs:65:9:
Formatting argument out of range
backtrace:
  &lt;krilla::interchange::tagging::fmt::Indent as core::fmt::Display&gt;::fmt (krilla/src/interchange/tagging/fmt.rs:65:9)
  &lt;krilla::interchange::tagging::fmt::Indent as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;krilla::interchange::tagging::fmt::Indent as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_krilla_src_interchange_tagging_fmt_rs_65_9_906043::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let stroke_dash = StrokeDash { array: vec![f32::NAN], offset: 0.0f32 };
assert!(stroke_dash == stroke_dash, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:5:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_krilla_paint_strokedash_5b2540::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let stroke = Stroke { paint: Paint::from(Color::new(1u8)), width: 0.0f32,
    miter_limit: 0.0f32, line_cap: LineCap::Round,
    line_join: LineJoin::default(), opacity: NormalizedF32::ZERO,
    dash: Some(StrokeDash { array: vec![f32::NAN], offset: 1.0f32 }) };
assert!(stroke == stroke, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:11:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_krilla_paint_stroke_69ffe4::main (src/main.rs:11:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NaiveRgbColor::new_f32(0.0f32, 1.0f32, 716.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interchange/tagging/tag.rs:383:13:
RGB color components must be in the range [0.0, 1.0]
backtrace:
  &lt;krilla::interchange::tagging::tag::NaiveRgbColor&gt;::new_f32 (krilla/src/interchange/tagging/tag.rs:383:13)
  repro_krilla_src_interchange_tagging_tag_rs_383_13_38fdbb::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;krilla::form::FormField&lt;krilla::form::kind::PushButton&gt;&gt;::push_button(
    ".".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interactive/form.rs:370:9:
field name cannot contain a period
backtrace:
  &lt;krilla::interactive::form::FormField&lt;krilla::interactive::form::kind::PushButton&gt;&gt;::push_button (krilla/src/interactive/form.rs:370:9)
  repro_krilla_src_interactive_form_rs_370_9_c3221d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = LinkAnnotation::new_with_quad_points(vec![],
    Target::Action(Action::ResetForm(ResetFormAction::Include(vec![]))));</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interactive/annotation.rs:200:9:
assertion failed: !quad_points.is_empty()
backtrace:
  &lt;krilla::interactive::annotation::LinkAnnotation&gt;::new_with_quad_points (krilla/src/interactive/annotation.rs:200:9)
  repro_krilla_src_interactive_annotation_rs_200_9_a19dce::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = LinkAnnotation::new_with_quad_points(vec![Quadrilateral([Point::from_xy(
    0.0f32, 0.0f32), Point { x: 0.0f32, y: 0.0f32 }, Point::from_xy(0.0f32,
    0.0f32), Point { x: 0.0f32, y: f32::MIN }])],
    Target::Destination(Destination::Xyz(XyzDestination::new(0usize,
    Point::from_xy(0.0f32, 0.0f32)))));</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interactive/annotation.rs:224:10:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;krilla::interactive::annotation::LinkAnnotation&gt;::new_with_quad_points (krilla/src/interactive/annotation.rs:224:10)
  repro_krilla_src_interactive_annotation_rs_224_10_100312::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;krilla::form::FormField&lt;krilla::form::kind::Radio&gt;&gt;::radio(
    ".".to_string(), None);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interactive/form.rs:444:9:
field name cannot contain a period
backtrace:
  &lt;krilla::interactive::form::FormField&lt;krilla::interactive::form::kind::Radio&gt;&gt;::radio (krilla/src/interactive/form.rs:444:9)
  repro_krilla_src_interactive_form_rs_444_9_057fb8::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let paint = Paint::from(LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: 0.0f32,
    y2: 0.0f32, transform: Transform::from_scale(0.0f32,
    0.0f32).invert().unwrap(), spread_method: SpreadMethod::Pad, stops: vec![],
    anti_alias: false });
assert!(paint == paint, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:8:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_krilla_paint_paint_eada92::main (src/main.rs:8:5)</pre></td></tr>
<tr><td valign="top"><pre>let linear_gradient = LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: 0.0f32,
    y2: 1.0f32, transform: Transform::from_skew(0.0f32, f32::NAN),
    spread_method: SpreadMethod::default(), stops: vec![], anti_alias: false };
assert!(linear_gradient == linear_gradient, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:7:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_krilla_paint_lineargradient_48551e::main (src/main.rs:7:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PageSettings::from_wh(1.0f32, f32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/page.rs:51:84:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;krilla::page::PageSettings&gt;::new (krilla/src/page.rs:51:84)
  &lt;krilla::page::PageSettings&gt;::from_wh (krilla/src/page.rs:61:14)
  repro_krilla_src_page_rs_51_84_a70bc5::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let indent = Indent(usize::MAX);
let _ = indent.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interchange/tagging/fmt.rs:65:46:
attempt to multiply with overflow
backtrace:
  &lt;krilla::interchange::tagging::fmt::Indent as core::fmt::Display&gt;::fmt (krilla/src/interchange/tagging/fmt.rs:65:46)
  &lt;krilla::interchange::tagging::fmt::Indent as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;krilla::interchange::tagging::fmt::Indent as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_krilla_src_interchange_tagging_fmt_rs_65_46_2b04d4::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new_with(SerializeSettings::default());
let mut page = document.start_page();
let mut surface = page.surface();
surface.draw_image(Image::from_png(Data::from(include_bytes!(
    "data_0.bin").to_vec()), false).unwrap(), Size::from_wh(f32::MAX,
    1.0f32).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/graphics/graphics_state.rs:249:48:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;krilla::graphics::graphics_state::GraphicsStates&gt;::transform_bbox (krilla/src/graphics/graphics_state.rs:249:48)
  &lt;krilla::content::ContentBuilder&gt;::expand_bbox (krilla/src/content.rs:205:45)
  &lt;krilla::content::ContentBuilder&gt;::draw_image::{closure#0} (krilla/src/content.rs:972:20)
  &lt;krilla::content::ContentBuilder&gt;::apply_isolated_op::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_image::{closure#0}, &lt;krilla::content::ContentBuilder&gt;::draw_image::{closure#1}&gt; (krilla/src/content.rs:1029:9)
  &lt;krilla::content::ContentBuilder&gt;::draw_image (krilla/src/content.rs:966:14)
  &lt;krilla::surface::Surface&gt;::draw_image (krilla/src/surface.rs:553:14)
  repro_krilla_src_graphics_graphics_state_rs_249_48_b10d94::main (src/main.rs:11:13)</pre></td></tr>
<tr><td valign="top"><pre>let rect = Rect::from(Rect::from_xywh(0.0f32, 0.0f32, 0.0f32, 0.0f32).unwrap());
let action = Action::from(ResetFormAction::All);
let mut document = Document::new();
let mut page = document.start_page();
let mut form_field = &lt;krilla::form::FormField&lt;u32&gt; as ::core::default::
    Default&gt;::default();
let _ = page.add_widget_annotation::&lt;u32&gt;(&amp;mut form_field,
    Annotation::from(LinkAnnotation::new(rect,
    Target::Action(action.clone()))));</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/page.rs:264:13:
Called add_widget_annotation with a non-widget annotation
backtrace:
  &lt;krilla::page::Page&gt;::add_widget_annotation::&lt;u32&gt; (krilla/src/page.rs:264:13)
  repro_krilla_src_page_rs_264_13_adf595::main (src/main.rs:15:18)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new_with(SerializeSettings::default());
let mut page = document.start_page();
let mut surface = page.surface();
surface.draw_glyphs(Point::from_xy(0.0f32, 0.0f32),
    &amp;[KrillaGlyph { glyph_id: GlyphId::new(1u32), text_range: 0usize..1usize,
    x_advance: 0.0f32, x_offset: 0.0f32, y_offset: 0.0f32, y_advance: 0.0f32,
    location: None }, KrillaGlyph { glyph_id: GlyphId::new(0u32),
    text_range: 0usize..0usize, x_advance: 0.0f32, x_offset: 0.0f32,
    y_offset: 0.0f32, y_advance: 0.0f32, location: None }],
    Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(),
    "", 0.0f32, false);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/text/group.rs:115:29:
end byte index 1 is out of bounds for string of length 0
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;krilla::text::group::GlyphSpanner&lt;_&gt; as core::iter::traits::iterator::Iterator&gt;::next::func::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/text/group.rs:115:29)
  &lt;krilla::text::group::GlyphSpanner&lt;krilla::text::glyph::KrillaGlyph&gt; as core::iter::traits::iterator::Iterator&gt;::next (krilla/src/text/group.rs:152:49)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#1} (krilla/src/content.rs:738:37)
  &lt;krilla::content::ContentBuilder&gt;::apply_isolated_op::&lt;&lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#0}, &lt;krilla::content::ContentBuilder&gt;::fill_str... (+187 chars)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:699:14)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:503:22)
  &lt;krilla::surface::Surface&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/surface.rs:373:39)
  repro_krilla_src_text_group_rs_115_29_dda73d::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new();
let mut page = document.start_page();
let mut surface = page.surface();
surface.push_isolated();
surface.draw_glyphs(Point { x: 0.0f32, y: 0.0f32 },
    &amp;[KrillaGlyph { glyph_id: GlyphId::new(0u32), text_range: 0usize..0usize,
    x_advance: 0.0f32, x_offset: 0.0f32, y_offset: 0.0f32, y_advance: f32::MIN,
    location: None }],
    Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(),
    "مرحبا", 25.0f32, false);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/content.rs:1337:37:
called `Option::unwrap()` on a `None` value
backtrace:
  krilla::content::get_glyphs_bbox::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:1337:37)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#1} (krilla/src/content.rs:431:28)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6} (krilla/src/content.rs:510:25)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#1} (krilla/src/content.rs:712:17)
  &lt;krilla::content::ContentBuilder&gt;::apply_isolated_op::&lt;&lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#0}, &lt;krilla::content::ContentBuilder&gt;::fill_str... (+187 chars)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:699:14)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:503:22)
  &lt;krilla::surface::Surface&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/surface.rs:373:39)
  repro_krilla_src_content_rs_1337_37_fe3649::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new_with(SerializeSettings::default());
let _ = document.register_named_destination(NamedDestination::new(
    "".to_string(), XyzDestination::new(1usize, Point::default())));
let _ = document.finish();</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interactive/destination.rs:135:13:
attempted to link to page 2, but document only has 1 pages
backtrace:
  &lt;krilla::interactive::destination::XyzDestination&gt;::serialize::{closure#0} (krilla/src/interactive/destination.rs:135:13)
  &lt;krilla::interactive::destination::XyzDestination&gt;::serialize (krilla/src/interactive/destination.rs:134:64)
  &lt;krilla::serialize::SerializeContext&gt;::serialize_xyz_destinations (krilla/src/serialize.rs:848:18)
  &lt;krilla::serialize::SerializeContext&gt;::finish (krilla/src/serialize.rs:481:14)
  &lt;krilla::document::Document&gt;::finish (krilla/src/document.rs:160:31)
  repro_krilla_src_interactive_destination_rs_135_13_42d342::main (src/main.rs:10:22)</pre></td></tr>
<tr><td valign="top"><pre>let f32_v = &lt;krilla::text::KrillaGlyph as krilla::text::Glyph&gt;::y_offset(
    &amp;KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, 0.0f32, 0.0f32,
    0usize..1usize, None), 0.0f32);
let mut document = Document::new_with(SerializeSettings::default());
let mut page = document.start_page();
let mut surface = page.surface();
surface.draw_glyphs(Point::from_xy(0.0f32, 0.0f32),
    &amp;[KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, 0.0f32, f32_v,
    0usize..1usize, None)],
    Font::new(Data::from(std::sync::Arc::new(include_bytes!(
    "data_0.bin").to_vec())), 0u32).unwrap(), "", f32_v, false);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/text/group.rs:453:21:
end byte index 1 is out of bounds for string of length 0
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  krilla::text::group::check_text_span_prop::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/text/group.rs:453:21)
  krilla::text::group::use_text_spanner::&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#0} (krilla/src/text/group.rs:419:13)
  krilla::text::group::use_text_spanner::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/text/group.rs:434:5)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#1} (krilla/src/content.rs:717:24)
  &lt;krilla::content::ContentBuilder&gt;::apply_isolated_op::&lt;&lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#0}, &lt;krilla::content::ContentBuilder&gt;::fill_str... (+187 chars)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:699:14)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:503:22)
  &lt;krilla::surface::Surface&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/surface.rs:373:39)
  repro_krilla_src_text_group_rs_453_21_558ba9::main (src/main.rs:14:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new();
let mut page = document.start_page_with(PageSettings::from_wh(1.0f32,
    1.0f32).unwrap());
let mut surface = page.surface();
surface.draw_glyphs(Point::default(),
    &amp;[KrillaGlyph { glyph_id: GlyphId::new(1000000000u32),
    text_range: 0usize..0usize, x_advance: 1.0f32, x_offset: 0.0f32,
    y_offset: 0.0f32, y_advance: 0.0f32, location: None }],
    Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(),
    "", 0.0f32, false);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/text/cid.rs:161:53:
called `Result::unwrap()` on an `Err` value: TryFromIntError(PosOverflow)
backtrace:
  &lt;krilla::text::cid::CIDFont&gt;::add_glyph (krilla/src/text/cid.rs:161:53)
  &lt;krilla::text::FontContainer&gt;::add_glyph (krilla/src/text/mod.rs:130:37)
  krilla::text::group::check_text_span_prop::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/text/group.rs:447:24)
  krilla::text::group::use_text_spanner::&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#0} (krilla/src/text/group.rs:419:13)
  krilla::text::group::use_text_spanner::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/text/group.rs:434:5)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#1} (krilla/src/content.rs:717:24)
  &lt;krilla::content::ContentBuilder&gt;::apply_isolated_op::&lt;&lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#0}, &lt;krilla::content::ContentBuilder&gt;::fill_str... (+187 chars)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:699:14)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:503:22)
  &lt;krilla::surface::Surface&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/surface.rs:373:39)
  repro_krilla_src_text_cid_rs_161_53_937b2b::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::default();
document.set_field_tree(FieldTree { fields: vec![Node::from(
    FieldGroup { name: ".".to_string(), fields: vec![] })] });
let _ = document.finish();</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interactive/form.rs:133:9:
field group name cannot contain a period
backtrace:
  &lt;krilla::interactive::form::FieldGroup&gt;::serialize_group (krilla/src/interactive/form.rs:133:9)
  &lt;krilla::interactive::form::Node&gt;::serialize_node (krilla/src/interactive/form.rs:176:29)
  &lt;krilla::interactive::form::AcroForm&gt;::serialize::{closure#0} (krilla/src/interactive/form.rs:59:34)
  &lt;&amp;mut &lt;krilla::interactive::form::AcroForm&gt;::serialize::{closure#0} as core::ops::function::FnOnce&lt;(&amp;krilla::interactive::form::Node,)&gt;&gt;::call_once (core/src/ops/function.rs:310:21)
  &lt;pdf_writer::object::Array&gt;::items::&lt;pdf_writer::object::Ref, core::iter::adapters::map::Map&lt;core::slice::iter::Iter&lt;krilla::interactive::form::Node&gt;, &lt;krilla::interactive::form::AcroForm&gt;::serialize::{closure#0}&gt;&gt; (pdf-writer/src/object.rs:831:22)
  &lt;pdf_writer::forms::Form&gt;::fields::&lt;core::iter::adapters::map::Map&lt;core::slice::iter::Iter&lt;krilla::interactive::form::Node&gt;, &lt;krilla::interactive::form::AcroForm&gt;::serialize::{closure#0}&gt;&gt; (pdf-writer/src/forms.rs:18:46)
  &lt;krilla::interactive::form::AcroForm&gt;::serialize (krilla/src/interactive/form.rs:61:18)
  &lt;krilla::serialize::SerializeContext&gt;::serialize_forms (krilla/src/serialize.rs:838:22)
  &lt;krilla::serialize::SerializeContext&gt;::finish (krilla/src/serialize.rs:475:14)
  &lt;krilla::document::Document&gt;::finish (krilla/src/document.rs:160:31)
  repro_krilla_src_interactive_form_rs_133_9_e7dc01::main (src/main.rs:9:22)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new();
let mut page = document.start_page();
let mut surface = page.surface();
surface.push_transform(&amp;Transform::from_scale(0.0f32,
    0.0f32).invert().unwrap());
let transform = surface.ctm();
let mut stream_builder = surface.stream_builder();
let mut surface2 = stream_builder.surface();
let stream_builder2 = surface2.stream_builder();
let stream = stream_builder2.finish();
let pattern = Pattern { stream: stream.clone(), transform: transform,
    width: 0.0f32, height: 0.0f32 };
assert!(pattern == pattern, "Eq is not reflexive");</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:16:5:
Eq is not reflexive
backtrace:
  repro_contract_eqreflexive_for_krilla_paint_pattern_58e221::main (src/main.rs:16:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::default();
let mut page = document.start_page_with(PageSettings::from_wh(1.0f32,
    1.0f32).unwrap());
let mut surface = page.surface();
let mut stream_builder = surface.stream_builder();
let mut surface2 = stream_builder.surface_with_bbox(Rect::from_xywh(0.0f32,
    f32::MAX, 0.0f32, 0.0f32).unwrap());
surface2.draw_image(Image::from_rgba8(vec![], 0u32, 0u32), Size::from_wh(1.0f32,
    1.0f32).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/geom.rs:124:59:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;krilla::geom::Rect&gt;::expand (krilla/src/geom.rs:124:59)
  &lt;krilla::content::ContentBuilder&gt;::expand_bbox (krilla/src/content.rs:207:18)
  &lt;krilla::content::ContentBuilder&gt;::draw_image::{closure#0} (krilla/src/content.rs:972:20)
  &lt;krilla::content::ContentBuilder&gt;::apply_isolated_op::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_image::{closure#0}, &lt;krilla::content::ContentBuilder&gt;::draw_image::{closure#1}&gt; (krilla/src/content.rs:1029:9)
  &lt;krilla::content::ContentBuilder&gt;::draw_image (krilla/src/content.rs:966:14)
  &lt;krilla::surface::Surface&gt;::draw_image (krilla/src/surface.rs:553:14)
  repro_krilla_src_geom_rs_124_59_a53984::main (src/main.rs:13:14)</pre></td></tr>
<tr><td valign="top"><pre>let mut path_builder = PathBuilder::new();
path_builder.quad_to(0.0f32, 0.0f32, 0.0f32, 0.0f32);
path_builder.close();
let fill_rule = FillRule::NonZero;
let mut document = Document::default();
let mut page = document.start_page();
let mut surface = page.surface();
surface.push_clip_path(&amp;path_builder.finish().unwrap(), &amp;fill_rule);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/content.rs:1282:39:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;krilla::content::ContentBuilder&gt;::content_draw_path::&lt;tiny_skia_path::path::PathSegmentsIter&gt; (krilla/src/content.rs:1282:39)
  &lt;krilla::content::ContentBuilder&gt;::push_clip_path (krilla/src/content.rs:381:14)
  &lt;krilla::surface::Surface&gt;::push_clip_path (krilla/src/surface.rs:449:27)
  repro_krilla_src_content_rs_1282_39_5ba7c3::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new();
let _ = document.register_named_destination(NamedDestination::new(
    "".to_string(), XyzDestination::new(usize::MAX, Point { x: 0.0f32,
    y: 0.0f32 })));
let _ = document.finish();</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/interactive/destination.rs:137:17:
attempt to add with overflow
backtrace:
  &lt;krilla::interactive::destination::XyzDestination&gt;::serialize::{closure#0} (krilla/src/interactive/destination.rs:137:17)
  &lt;krilla::interactive::destination::XyzDestination&gt;::serialize (krilla/src/interactive/destination.rs:134:64)
  &lt;krilla::serialize::SerializeContext&gt;::serialize_xyz_destinations (krilla/src/serialize.rs:848:18)
  &lt;krilla::serialize::SerializeContext&gt;::finish (krilla/src/serialize.rs:481:14)
  &lt;krilla::document::Document&gt;::finish (krilla/src/document.rs:160:31)
  repro_krilla_src_interactive_destination_rs_137_17_4e2eb1::main (src/main.rs:9:22)</pre></td></tr>
<tr><td valign="top"><pre>let spread_method = SpreadMethod::Pad;
let mut document = Document::new_with(SerializeSettings::default());
let mut page = document.start_page_with(PageSettings::new(Size::from_wh(1.0f32,
    1.0f32).unwrap()));
let mut surface = page.surface();
let transform = surface.cur_transform();
surface.set_fill(Some(Fill { paint: Paint::from(SweepGradient { cx: 0.0f32,
    cy: 1.0f32, start_angle: 0.0f32, end_angle: 1.0f32, transform: transform,
    spread_method: spread_method, stops: vec![], anti_alias: false }),
    opacity: NormalizedF32::ZERO, rule: FillRule::default() }));
surface.draw_text(Point::default(),
    Font::new(Data::from(std::sync::Arc::new(include_bytes!(
    "data_0.bin").to_vec())), 0u32).unwrap(), 0.0f32, "Z", false,
    TextDirection::LeftToRight);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/graphics/shading_function.rs:254:54:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[krilla::graphics::paint::Stop]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[krilla::graphics::paint::Stop] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;krilla::graphics::shading_function::ShadingFunction as krilla::serialize::Cacheable&gt;::serialize (krilla/src/graphics/shading_function.rs:254:54)
  &lt;krilla::serialize::SerializeContext&gt;::register_cacheable::&lt;krilla::graphics::shading_function::ShadingFunction&gt;::{closure#0} (krilla/src/serialize.rs:620:20)
  &lt;krilla::serialize::SerializeContext&gt;::register_cached::&lt;krilla::graphics::shading_function::ShadingFunction, &lt;krilla::serialize::SerializeContext&gt;::register_cacheable&lt;krilla::graphics::shading_function::ShadingFunction&gt;::{closure#0}&gt; (krilla/src/serialize.rs:605:13)
  &lt;krilla::serialize::SerializeContext&gt;::register_cacheable::&lt;krilla::graphics::shading_function::ShadingFunction&gt; (krilla/src/serialize.rs:619:14)
  &lt;krilla::graphics::shading_pattern::ShadingPattern as krilla::serialize::Cacheable&gt;::serialize (krilla/src/graphics/shading_pattern.rs:48:30)
  &lt;krilla::serialize::SerializeContext&gt;::register_cacheable::&lt;krilla::graphics::shading_pattern::ShadingPattern&gt;::{closure#0} (krilla/src/serialize.rs:620:20)
  &lt;krilla::serialize::SerializeContext&gt;::register_cached::&lt;krilla::graphics::shading_pattern::ShadingPattern, &lt;krilla::serialize::SerializeContext&gt;::register_cacheable&lt;krilla::graphics::shading_pattern::ShadingPattern&gt;::{closure#0}&gt; (krilla/src/serialize.rs:605:13)
  &lt;krilla::serialize::SerializeContext&gt;::register_cacheable::&lt;krilla::graphics::shading_pattern::ShadingPattern&gt; (krilla/src/serialize.rs:619:14)
  &lt;krilla::serialize::SerializeContext&gt;::register_resourceable::&lt;krilla::graphics::shading_pattern::ShadingPattern&gt; (krilla/src/serialize.rs:632:28)
  &lt;krilla::content::ContentBuilder&gt;::content_set_fill_stroke_properties::&lt;&lt;krilla::content::ContentBuilder&gt;::content_set_fill_properties::set_pattern_fn, krilla::content::set_solid_fill&gt;::{closure#1} (krilla/src/content.rs:1102:32)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new_with(SerializeSettings::default());
let mut page = document.start_page();
let f32_v = &lt;krilla::text::KrillaGlyph as krilla::text::Glyph&gt;::y_advance(
    &amp;KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, 0.0f32, 0.0f32,
    0usize..0usize, None), 0.0f32);
let mut surface = page.surface();
surface.draw_glyphs(Point::default(), &amp;[KrillaGlyph::new(GlyphId::new(0u32),
    0.0f32, 0.0f32, f32_v, f32::NAN, 0usize..0usize, None),
    KrillaGlyph { glyph_id: GlyphId::new(10u32), text_range: 0usize..0usize,
    x_advance: 0.0f32, x_offset: 0.0f32, y_offset: 0.0f32, y_advance: 0.0f32,
    location: None }],
    Font::new_variable(Data::from(std::sync::Arc::new(include_bytes!(
    "data_0.bin").to_vec())), 0u32, &amp;[]).unwrap(), "", 0.0f32, true);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/content.rs:384:18:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;krilla::content::ContentBuilder&gt;::push_clip_path (krilla/src/content.rs:384:18)
  &lt;krilla::surface::Surface&gt;::push_clip_path (krilla/src/surface.rs:449:27)
  krilla::text::glyph::colr::interpret (krilla/src/text/glyph/colr.rs:82:29)
  krilla::text::glyph::colr::draw_glyph (krilla/src/text/glyph/colr.rs:54:5)
  krilla::text::glyph::draw_color_glyph (krilla/src/text/glyph/mod.rs:48:17)
  krilla::text::glyph::draw_glyph (krilla/src/text/glyph/mod.rs:84:5)
  &lt;krilla::surface::Surface&gt;::outline_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/surface.rs:278:13)
  &lt;krilla::surface::Surface&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/surface.rs:307:18)
  repro_krilla_src_content_rs_384_18_41beb6::main (src/main.rs:14:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Image::from_webp(Data::from(include_bytes!("data_0.bin").to_vec()),
    false);
let document = Document::default();
let _ = document.finish();</pre></td><td valign="top"><pre>out of memory: memory allocation of 7078048000 bytes failed
backtrace:
  krilla::graphics::image::decode_webp (src/graphics/image/mod.rs:797)
  krilla::graphics::image::{impl#10}::from_webp::{closure#0} (src/graphics/image/mod.rs:313)
  krilla::util::deferred::Deferred&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;&gt;::new&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;, krilla::graphics::image::{impl#10}::from_webp::{closure_env#0}&gt; (src/util.rs:188)
  krilla::graphics::image::Image::from_webp (src/graphics/image/mod.rs:313)
  repro_out_of_memory_in_krilla_document_document_finish_6e8f33::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let paint = Paint::from(LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: 0.0f32,
    y2: 0.0f32, transform: Transform::identity(),
    spread_method: SpreadMethod::Pad, stops: vec![], anti_alias: false });
let mut document = Document::new();
let mut page = document.start_page();
let mut surface = page.surface();
surface.set_stroke(Some(Stroke { paint: paint.clone(), width: 1.0f32,
    miter_limit: 0.0f32, line_cap: LineCap::Square, line_join: LineJoin::Bevel,
    opacity: NormalizedF32::new(0.0f32).unwrap(), dash: None }));
surface.draw_text(Point::from_xy(0.0f32, 0.0f32),
    Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(),
    0.0f32, "1", false, TextDirection::BottomToTop);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/graphics/shading_function.rs:239:54:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[krilla::graphics::paint::Stop]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[krilla::graphics::paint::Stop] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;krilla::graphics::shading_function::ShadingFunction as krilla::serialize::Cacheable&gt;::serialize (krilla/src/graphics/shading_function.rs:239:54)
  &lt;krilla::serialize::SerializeContext&gt;::register_cacheable::&lt;krilla::graphics::shading_function::ShadingFunction&gt;::{closure#0} (krilla/src/serialize.rs:620:20)
  &lt;krilla::serialize::SerializeContext&gt;::register_cached::&lt;krilla::graphics::shading_function::ShadingFunction, &lt;krilla::serialize::SerializeContext&gt;::register_cacheable&lt;krilla::graphics::shading_function::ShadingFunction&gt;::{closure#0}&gt; (krilla/src/serialize.rs:605:13)
  &lt;krilla::serialize::SerializeContext&gt;::register_cacheable::&lt;krilla::graphics::shading_function::ShadingFunction&gt; (krilla/src/serialize.rs:619:14)
  &lt;krilla::graphics::shading_pattern::ShadingPattern as krilla::serialize::Cacheable&gt;::serialize (krilla/src/graphics/shading_pattern.rs:48:30)
  &lt;krilla::serialize::SerializeContext&gt;::register_cacheable::&lt;krilla::graphics::shading_pattern::ShadingPattern&gt;::{closure#0} (krilla/src/serialize.rs:620:20)
  &lt;krilla::serialize::SerializeContext&gt;::register_cached::&lt;krilla::graphics::shading_pattern::ShadingPattern, &lt;krilla::serialize::SerializeContext&gt;::register_cacheable&lt;krilla::graphics::shading_pattern::ShadingPattern&gt;::{closure#0}&gt; (krilla/src/serialize.rs:605:13)
  &lt;krilla::serialize::SerializeContext&gt;::register_cacheable::&lt;krilla::graphics::shading_pattern::ShadingPattern&gt; (krilla/src/serialize.rs:619:14)
  &lt;krilla::serialize::SerializeContext&gt;::register_resourceable::&lt;krilla::graphics::shading_pattern::ShadingPattern&gt; (krilla/src/serialize.rs:632:28)
  &lt;krilla::content::ContentBuilder&gt;::content_set_fill_stroke_properties::&lt;&lt;krilla::content::ContentBuilder&gt;::content_set_stroke_properties::set_pattern_fn, &lt;krilla::content::ContentBuilder&gt;::content_set_stroke_properties::set_solid_fn&gt;::{closure#1} (krilla/src/content.rs:1102:32)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::default();
let mut page = document.start_page_with(PageSettings::new(Size::from_wh(1.0f32,
    1.0f32).unwrap()));
let mut surface = page.surface();
surface.push_isolated();
surface.pop();
surface.push_opacity(NormalizedF32::ONE);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/surface.rs:644:9:
assertion failed: self.push_instructions.is_empty()
backtrace:
  &lt;krilla::surface::Surface as core::ops::drop::Drop&gt;::drop (krilla/src/surface.rs:644:9)
  repro_krilla_src_surface_rs_644_9_e2e9a5::main (src/main.rs:13:1)</pre></td></tr>
<tr><td valign="top"><pre>let font = Font::new(Data::from(std::sync::Arc::new(include_bytes!(
    "data_0.bin").to_vec())), 0u32).unwrap();
let mut document = Document::default();
let mut page = document.start_page();
let mut surface = page.surface();
let fill = Fill { paint: Paint::from(LinearGradient { x1: 0.0f32,
    y1: f32::INFINITY, x2: 0.0f32, y2: 0.0f32,
    transform: Transform::from_rotate(0.0f32),
    spread_method: SpreadMethod::Reflect, stops: vec![], anti_alias: false }),
    opacity: NormalizedF32::ZERO, rule: FillRule::default() };
surface.set_fill(Some(fill.clone()));
surface.draw_text(Point::from_xy(0.0f32, 0.0f32), font.clone(), 0.0f32, "a",
    false, TextDirection::Auto);</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/graphics/shading_function.rs:114:60:
called `Option::unwrap()` on a `None` value
backtrace:
  krilla::graphics::shading_function::get_expanded_bbox (krilla/src/graphics/shading_function.rs:114:60)
  &lt;krilla::graphics::paint::LinearGradient as krilla::graphics::shading_function::GradientPropertiesExt&gt;::gradient_properties (krilla/src/graphics/shading_function.rs:159:29)
  &lt;krilla::content::ContentBuilder&gt;::content_set_fill_stroke_properties::&lt;&lt;krilla::content::ContentBuilder&gt;::content_set_fill_properties::set_pattern_fn, krilla::content::set_solid_fill&gt; (krilla/src/content.rs:1133:62)
  &lt;krilla::content::ContentBuilder&gt;::content_set_fill_properties (krilla/src/content.rs:1178:14)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#1} (krilla/src/content.rs:438:16)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6} (krilla/src/content.rs:510:25)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#1} (krilla/src/content.rs:712:17)
  &lt;krilla::content::ContentBuilder&gt;::apply_isolated_op::&lt;&lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#0}, &lt;krilla::content::ContentBuilder&gt;::fill_str... (+187 chars)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:699:14)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:503:22)
  &lt;krilla::surface::Surface&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/surface.rs:358:39)
  &lt;krilla::surface::Surface&gt;::draw_text (krilla/src/surface.rs:414:14)</pre></td></tr>
<tr><td valign="top"><pre>let transform = Transform::from_scale(0.0f32, 0.0f32);
let f32_v = transform.tx();
let _ = Image::from_webp(Data::from(include_bytes!("data_0.bin").to_vec()),
    false);
let _ = LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: f32_v, y2: f32::NAN,
    transform: transform, spread_method: SpreadMethod::Pad, stops: vec![],
    anti_alias: false };</pre></td><td valign="top"><pre>out of memory: memory allocation of 10695635200 bytes failed
backtrace:
  krilla::graphics::image::decode_webp (src/graphics/image/mod.rs:797)
  krilla::graphics::image::{impl#10}::from_webp::{closure#0} (src/graphics/image/mod.rs:313)
  krilla::util::deferred::Deferred&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;&gt;::new&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;, krilla::graphics::image::{impl#10}::from_webp::{closure_env#0}&gt; (src/util.rs:188)
  krilla::graphics::image::Image::from_webp (src/graphics/image/mod.rs:313)
  repro_out_of_memory_in_krilla_paint_lineargradient_literal_96b651::main (src/main.rs:10)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let document = Document::default();
let _ = document.finish();
let _ = Image::from_webp(Data::from(std::sync::Arc::new(include_bytes!(
    "data_0.bin").to_vec())), false);
let _ = EmbedError::Existence;</pre></td><td valign="top"><pre>out of memory: memory allocation of 12058784000 bytes failed
backtrace:
  krilla::graphics::image::decode_webp (src/graphics/image/mod.rs:797)
  krilla::graphics::image::{impl#10}::from_webp::{closure#0} (src/graphics/image/mod.rs:313)
  krilla::util::deferred::Deferred&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;&gt;::new&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;, krilla::graphics::image::{impl#10}::from_webp::{closure_env#0}&gt; (src/util.rs:188)
  krilla::graphics::image::Image::from_webp (src/graphics/image/mod.rs:313)
  repro_out_of_memory_in_krilla_embed_embederror_existence_0a69a4::main (src/main.rs:9)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let transform = Transform::default();
let f32_v = transform.tx();
let _ = VariableAppearance { font: FormFont::Standard(StandardFont::TimesRoman),
    font_size: -1e30f32, paint: Some(Paint::from(Color::new(255u8, 255u8,
    19u8))), opacity: NormalizedF32::ZERO };
let _ = Accessibility::UA1;
let _ = AssociationKind::Data;
let _ = Image::from_webp(Data::from(include_bytes!(
    "mutated_0_rgba8.webp").to_vec()), true);
let form_field = &lt;krilla::form::FormField&lt;krilla::form::kind::PushButton&gt; as ::
    core::default::Default&gt;::default();
let _ = form_field.with_location(None);
... (11 lines omitted)
let _ = Stroke::default();
let _ = LinearGradient { x1: 1.1920929e-7f32, y1: 81.9f32, x2: 0.5f32,
    y2: -0.0f32, transform: transform, spread_method: SpreadMethod::Repeat,
    stops: vec![], anti_alias: true };
let _ = Paint::from(krilla::color::luma::Color::default());
let _ = VariableAppearance { font: FormFont::Standard(StandardFont::Courier),
    font_size: f32::MAX, paint: None,
    opacity: NormalizedF32::new(f32::MIN).unwrap() };
let _ = ResetFormAction::Include(vec!["\0".to_string(), "ZK(0\u{575cf}5 \
    6)]F😀o&amp;".to_string(), "Zażółć gęślą jaźń".to_string()]);
let _ = Datetime;
let _ = Stroke::default();
let _ = StrokeDash { array: vec![-15.7f32, 445.0f32, f32_v, f32::NAN,
    1.1754944e-38f32], offset: 10000000000.0f32 };</pre></td><td valign="top"><pre>out of memory: memory allocation of 4980896000 bytes failed
backtrace:
  krilla::graphics::image::decode_webp (src/graphics/image/mod.rs:797)
  krilla::graphics::image::{impl#10}::from_webp::{closure#0} (src/graphics/image/mod.rs:313)
  krilla::util::deferred::Deferred&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;&gt;::new&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;, krilla::graphics::image::{impl#10}::from_webp::{closure_env#0}&gt; (src/util.rs:188)
  krilla::graphics::image::Image::from_webp (src/graphics/image/mod.rs:313)
  repro_out_of_memory_in_krilla_form_variable_text_variableappearanc_38b806::main (src/main.rs:34)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Image::from_webp(Data::from(std::sync::Arc::new(include_bytes!(
    "mutated_0_rgba8.webp").to_vec())), true);
let _ = ResetFormAction::Include(vec!["مرحبا".to_string()]);
let mut path_builder = PathBuilder::default();
path_builder.quad_to(-50.2f32, 0.0f32, 28.6f32, -1.6f32);
let _ = &lt;krilla::icc::ICCProfile&lt;4&gt;&gt;::new(&amp;include_bytes!(
    "sRGB-v4.icc").to_vec());
let _ = GlyphOrientationVertical::Clockwise90;
let _ = Quadrilateral([Point { x: -0.0016725775f32, y: -2.2978705e-26f32 },
    Point::default(), Point::from_xy(1.0f32, 256.0f32), Point::default()]);
let _ = Lbl;
let destination = Destination::from(XyzDestination::new(usize::MAX,
    Point { x: 82.2f32, y: f32::MAX }));
let _ = AnnotationType::Link(LinkAnnotation::new(Rect::from_xywh(0.0f32,
    -0.0f32, 0.5f32, 0.5f32).unwrap(),
    Target::Destination(destination.clone())).with_border(LinkBorder::new(
    f32::MAX, krilla::color::Color::from(Color::default()))));
let _ = Font::new(Data::from(include_bytes!(
    "mutated_2_LatinModernRoman-Regular.otf").to_vec()), 3u32);
let accessibility = Accessibility::from(Accessibility::UA1);
let _ = accessibility.min();
let _ = ConfigurationBuilder::default();
let _ = THead;
let _ = Text::default();
let _ = XyzDestination::new(1usize, Point::from_xy(f32::MIN, 65536.0f32));
let _ = XyzDestination::new(1usize, Point::default());
let _ = LineJoin::Round;
let _ = LineJoin::default();</pre></td><td valign="top"><pre>out of memory: memory allocation of 3722604800 bytes failed
backtrace:
  krilla::graphics::image::decode_webp (src/graphics/image/mod.rs:797)
  krilla::graphics::image::{impl#10}::from_webp::{closure#0} (src/graphics/image/mod.rs:313)
  krilla::util::deferred::Deferred&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;&gt;::new&lt;core::result::Result&lt;krilla::graphics::image::Repr, alloc::string::String&gt;, krilla::graphics::image::{impl#10}::from_webp::{closure_env#0}&gt; (src/util.rs:188)
  krilla::graphics::image::Image::from_webp (src/graphics/image/mod.rs:313)
  repro_out_of_memory_in_krilla_text_font_new_e160c4::main (src/main.rs:25)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::default();
let mut page = document.start_page();
let mut surface = page.surface();
surface.set_stroke(Some(Stroke { paint: Paint::from(Color::new(0u8, 0u8, 0u8,
    0u8)), width: -68.0f32, miter_limit: 0.0f32, line_cap: LineCap::default(),
    line_join: LineJoin::default(), opacity: NormalizedF32::ZERO,
    dash: None }));
surface.draw_glyphs(Point::default(), &amp;[KrillaGlyph::new(GlyphId::new(0u32),
    0.0f32, 0.0f32, 0.0f32, 0.0f32, 0usize..0usize, None)],
    Font::new_variable(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32,
    &amp;[]).unwrap(), "", 0.0f32, false);</pre></td><td valign="top"><pre>thread 'main' panicked at pdf-writer/src/content.rs:170:9:
line width must be positive
backtrace:
  &lt;pdf_writer::content::Content&gt;::set_line_width (pdf-writer/src/content.rs:170:9)
  &lt;krilla::content::ContentBuilder&gt;::content_set_stroke_properties (krilla/src/content.rs:1239:26)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#2} (krilla/src/content.rs:454:16)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#7} (krilla/src/content.rs:529:25)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#7}, krilla::text::glyph::KrillaGlyph&gt;::{closure#1} (krilla/src/content.rs:712:17)
  &lt;krilla::content::ContentBuilder&gt;::apply_isolated_op::&lt;&lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#7}, krilla::text::glyph::KrillaGlyph&gt;::{closure#0}, &lt;krilla::content::ContentBuilder&gt;::fill_str... (+187 chars)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#7}, krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:699:14)
  &lt;krilla::content::ContentBuilder&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:522:22)
  &lt;krilla::surface::Surface&gt;::draw_glyphs::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/surface.rs:344:39)
  repro_pdf_writer_src_content_rs_170_9_047c12::main (src/main.rs:19:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new_with(SerializeSettings::default());
let mut page = document.start_page_with(PageSettings::from_wh(1.0f32,
    1.0f32).unwrap());
let mut surface = page.surface();
surface.draw_text(Point { x: 0.0f32, y: 0.0f32 },
    Font::new(Data::from(std::sync::Arc::new(include_bytes!(
    "data_0.bin").to_vec())), 0u32).unwrap(), 0.0f32, "P", true,
    TextDirection::BottomToTop);</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/tables/glyf.rs:244:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;read_fonts::tables::glyf::SimpleGlyph&gt;::read_points_fast::&lt;i32&gt; (read-fonts/src/tables/glyf.rs:244:17)
  &lt;skrifa::outline::glyf::FreeTypeScaler as skrifa::outline::glyf::Scaler&gt;::load_simple (skrifa/src/outline/glyf/mod.rs:614:15)
  &lt;skrifa::outline::glyf::FreeTypeScaler as skrifa::outline::glyf::Scaler&gt;::load (skrifa/src/outline/glyf/mod.rs:301:49)
  &lt;skrifa::outline::glyf::FreeTypeScaler&gt;::scale (skrifa/src/outline/glyf/mod.rs:504:14)
  &lt;skrifa::outline::OutlineGlyph&gt;::draw_unhinted::&lt;skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder&gt;::{closure#0} (skrifa/src/outline/mod.rs:420:38)
  skrifa::outline::memory::with_temporary_memory::stack_mem::&lt;512, core::result::Result&lt;skrifa::outline::AdjustedMetrics, skrifa::outline::error::DrawError&gt;, &lt;skrifa::outline::OutlineGlyph&gt;::draw_unhinted&lt;skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder&gt;::{closure#0}&gt; (skr... (+31 chars)
  skrifa::outline::memory::with_temporary_memory::&lt;core::result::Result&lt;skrifa::outline::AdjustedMetrics, skrifa::outline::error::DrawError&gt;, &lt;skrifa::outline::OutlineGlyph&gt;::draw_unhinted&lt;skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder&gt;::{closure#0}&gt; (skrifa/src/outline/... (+15 chars)
  skrifa::outline::with_temporary_memory::&lt;core::result::Result&lt;skrifa::outline::AdjustedMetrics, skrifa::outline::error::DrawError&gt;, &lt;skrifa::outline::OutlineGlyph&gt;::draw_unhinted&lt;skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder&gt;::{closure#0}&gt; (skrifa/src/outline/mod.rs:7... (+6 chars)
  &lt;skrifa::outline::OutlineGlyph&gt;::draw_unhinted::&lt;skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder&gt; (skrifa/src/outline/mod.rs:415:17)
  &lt;skrifa::outline::OutlineGlyph&gt;::draw::&lt;skrifa::outline::DrawSettings, krilla::text::glyph::outline::OutlineBuilder&gt; (skrifa/src/outline/mod.rs:361:22)
  krilla::text::glyph::outline::glyph_path (krilla/src/text/glyph/outline.rs:15:14)
  krilla::text::glyph::outline::draw_glyph (krilla/src/text/glyph/outline.rs:32:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new_with(SerializeSettings::default());
let mut page = document.start_page_with(PageSettings::default());
let mut surface = page.surface();
surface.draw_text(Point::from_xy(0.0f32, 0.0f32),
    Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(),
    -2147483600.0f32, "a", false, TextDirection::Auto);</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/num/mod.rs:429:5:
attempt to negate with overflow
backtrace:
  &lt;i32&gt;::abs (core/src/num/int_macros.rs:3635:17)
  &lt;pdf_writer::buf::Limits&gt;::register_int (pdf-writer/src/buf.rs:195:37)
  &lt;pdf_writer::buf::Buf&gt;::push_int (pdf-writer/src/buf.rs:46:21)
  &lt;pdf_writer::buf::Buf&gt;::push_float (pdf-writer/src/buf.rs:55:18)
  &lt;f32 as pdf_writer::object::sealed::Sealed&gt;::write (pdf-writer/src/object.rs:70:13)
  &lt;pdf_writer::object::Obj&gt;::primitive::&lt;f32&gt; (pdf-writer/src/object.rs:701:15)
  &lt;pdf_writer::content::Operation&gt;::operand::&lt;f32&gt; (pdf-writer/src/content.rs:102:20)
  &lt;pdf_writer::content::Content&gt;::set_font (pdf-writer/src/content.rs:546:37)
  &lt;krilla::content::ContentBuilder&gt;::encode_consecutive_glyph_run::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:563:22)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_group::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:857:14)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_span::&lt;krilla::text::glyph::KrillaGlyph&gt; (krilla/src/content.rs:806:18)
  &lt;krilla::content::ContentBuilder&gt;::fill_stroke_glyph_run::&lt;&lt;krilla::content::ContentBuilder&gt;::draw_glyphs&lt;krilla::text::glyph::KrillaGlyph&gt;::{closure#6}, krilla::text::glyph::KrillaGlyph&gt;::{closure#1} (krilla/src/content.rs:755:24)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new();
let mut page = document.start_page();
let mut surface = page.surface();
surface.push_isolated();</pre></td><td valign="top"><pre>thread 'main' panicked at krilla/src/surface.rs:643:9:
assertion failed: self.bd.sub_builders.is_empty()
backtrace:
  &lt;krilla::surface::Surface as core::ops::drop::Drop&gt;::drop (krilla/src/surface.rs:643:9)
  repro_krilla_src_surface_rs_643_9_dddc71::main (src/main.rs:8:1)</pre></td></tr>
</table>
