# read-fonts - crashes

Source: https://github.com/googlefonts/fontations @ 5c480698f43ab4ff5b304de5a9204c1b5e720aec

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let buffer_lengths = BufferLengths { points: 0u32, contours: 0u32,
    unscaled: 0u32, deltas: u32::MAX, iup: 0u32, composite_deltas: 1u32 };
let _ = buffer_lengths.packed_len::&lt;read_fonts::tables::glyf::outline::
    Unscaled&gt;();</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/tables/glyf/outline/buffers.rs:73:15:
attempt to add with overflow
backtrace:
  &lt;read_fonts::tables::glyf::outline::buffers::BufferLengths&gt;::packed_len::&lt;read_fonts::tables::glyf::outline::scale::Unscaled&gt; (read-fonts/src/tables/glyf/outline/buffers.rs:73:15)
  repro_read_fonts_src_tables_glyf_outline_buffers_rs_73_15_164cbf::main (src/main.rs:5:28)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;read_fonts::tables::gvar::GlyphDelta as read_fonts::tables::
    variations::TupleDelta&gt;::new(0u16, 1000000000i32,
    0i32).apply_scalar::&lt;i32&gt;(NOT_COMPUTED);</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:136:16:
attempt to multiply with overflow
backtrace:
  &lt;i32 as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  &lt;font_types::point::Point&lt;i32&gt; as core::ops::arith::Mul&lt;i32&gt;&gt;::mul (font-types/src/point.rs:136:16)
  &lt;read_fonts::tables::gvar::GlyphDelta&gt;::apply_scalar::&lt;i32&gt; (read-fonts/src/tables/gvar.rs:288:9)
  repro_font_types_src_point_rs_136_16_73d0ae::main (src/main.rs:4:130)</pre></td></tr>
<tr><td valign="top"><pre>let buffer_lengths = BufferLengths { points: 1u32, contours: 1u32,
    unscaled: 0u32, deltas: 1u32, iup: u32::MAX, composite_deltas: 0u32 };
let _ = buffer_lengths.packed_len::&lt;read_fonts::tables::glyf::outline::
    ScaleF32&gt;();</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/tables/glyf/outline/buffers.rs:73:16:
attempt to add with overflow
backtrace:
  &lt;read_fonts::tables::glyf::outline::buffers::BufferLengths&gt;::packed_len::&lt;read_fonts::tables::glyf::outline::scale::ScaleF32&gt; (read-fonts/src/tables/glyf/outline/buffers.rs:73:16)
  repro_read_fonts_src_tables_glyf_outline_buffers_rs_73_16_34fb08::main (src/main.rs:5:28)</pre></td></tr>
<tr><td valign="top"><pre>let mut svg_pen = SvgPen::from(SvgPen::with_precision(usize::MAX));
&lt;read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::
    outline::OutlinePen&gt;::quad_to(&amp;mut svg_pen, 16777216.0f32, 4.8731973e-34f32,
    -0.0f32, 1e30f32);</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/model/glyph/outline.rs:150:13:
Formatting argument out of range
backtrace:
  &lt;read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::outline::OutlinePen&gt;::quad_to (read-fonts/src/model/glyph/outline.rs:150:13)
  repro_read_fonts_src_model_glyph_outline_rs_150_13_462ff7::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut svg_pen = SvgPen::with_precision(10000000000000000000usize);
&lt;read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::
    outline::OutlinePen&gt;::line_to(&amp;mut svg_pen, 38.4f32, f32::NEG_INFINITY);</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/model/glyph/outline.rs:141:13:
Formatting argument out of range
backtrace:
  &lt;read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::outline::OutlinePen&gt;::line_to (read-fonts/src/model/glyph/outline.rs:141:13)
  repro_read_fonts_src_model_glyph_outline_rs_141_13_85524c::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut svg_pen = SvgPen::with_precision(1000000000usize);
&lt;read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::
    outline::OutlinePen&gt;::move_to(&amp;mut svg_pen, 0.0f32, 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/model/glyph/outline.rs:132:13:
Formatting argument out of range
backtrace:
  &lt;read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::outline::OutlinePen&gt;::move_to (read-fonts/src/model/glyph/outline.rs:132:13)
  repro_read_fonts_src_model_glyph_outline_rs_132_13_d6f60c::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut svg_pen = SvgPen::from(SvgPen::with_precision(
    1000000000000000000usize));
&lt;read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::
    outline::OutlinePen&gt;::curve_to(&amp;mut svg_pen, 0.0f32, 0.0f32, 0.0f32, 0.0f32,
    1.0f32, 0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/model/glyph/outline.rs:159:13:
Formatting argument out of range
backtrace:
  &lt;read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::outline::OutlinePen&gt;::curve_to (read-fonts/src/model/glyph/outline.rs:159:13)
  repro_read_fonts_src_model_glyph_outline_rs_159_13_6b7f7a::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;read_fonts::tables::gvar::GlyphDelta as read_fonts::tables::
    variations::TupleDelta&gt;::new(0u16, 0i32,
    1000000000i32).apply_scalar::&lt;i32&gt;(
    &lt;font_types::Fixed as font_types::Scalar&gt;::from_raw([0u8, 10u8, 0u8, 0u8]));</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:137:16:
attempt to multiply with overflow
backtrace:
  &lt;i32 as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  &lt;font_types::point::Point&lt;i32&gt; as core::ops::arith::Mul&lt;i32&gt;&gt;::mul (font-types/src/point.rs:137:16)
  &lt;read_fonts::tables::gvar::GlyphDelta&gt;::apply_scalar::&lt;i32&gt; (read-fonts/src/tables/gvar.rs:288:9)
  repro_font_types_src_point_rs_137_16_c2802e::main (src/main.rs:2:130)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![127u8, 127u8, 127u8, 127u8];
let tuple_variation_header = TupleVariationHeader::read(FontData::new(&amp;vec),
    1u16).unwrap();
let tuple_variation_header2 = tuple_variation_header.shape();
let tuple_variation_header3 = tuple_variation_header2.shape();
let _ = tuple_variation_header3.intermediate_start_tuple();</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/tables/variations.rs:177:62:
called `Result::unwrap()` on an `Err` value: OutOfBounds
backtrace:
  &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::intermediate_start_tuple::{closure#0} (read-fonts/src/tables/variations.rs:177:62)
  &lt;bool&gt;::then::&lt;read_fonts::tables::variations::Tuple, &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::intermediate_start_tuple::{closure#0}&gt; (core/src/bool.rs:66:24)
  &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::intermediate_start_tuple (read-fonts/src/tables/variations.rs:174:50)
  repro_read_fonts_src_tables_variations_rs_177_62_1e9688::main (src/main.rs:9:37)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![81u8, 0u8, 93u8, 0u8];
let font_data = FontData::new(&amp;vec);
let tuple_variation_header = TupleVariationHeader::read(font_data,
    1u16).unwrap();
let _ = tuple_variation_header.intermediate_end_tuple();</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/tables/variations.rs:189:62:
called `Result::unwrap()` on an `Err` value: OutOfBounds
backtrace:
  &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::intermediate_end_tuple::{closure#0} (read-fonts/src/tables/variations.rs:189:62)
  &lt;bool&gt;::then::&lt;read_fonts::tables::variations::Tuple, &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::intermediate_end_tuple::{closure#0}&gt; (core/src/bool.rs:66:24)
  &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::intermediate_end_tuple (read-fonts/src/tables/variations.rs:186:50)
  repro_read_fonts_src_tables_variations_rs_189_62_bf8235::main (src/main.rs:8:36)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![178u8, 120u8, 171u8, 116u8];
let font_data = FontData::new(&amp;vec);
let u16_v = TupleIndex::INTERMEDIATE_REGION;
let tuple_variation_header = TupleVariationHeader::read(font_data,
    u16_v).unwrap();
let _ = tuple_variation_header.peak_tuple();</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/tables/variations.rs:165:62:
called `Result::unwrap()` on an `Err` value: OutOfBounds
backtrace:
  &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::peak_tuple::{closure#0} (read-fonts/src/tables/variations.rs:165:62)
  &lt;bool&gt;::then::&lt;read_fonts::tables::variations::Tuple, &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::peak_tuple::{closure#0}&gt; (core/src/bool.rs:66:24)
  &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::peak_tuple (read-fonts/src/tables/variations.rs:162:50)
  repro_read_fonts_src_tables_variations_rs_165_62_7db33e::main (src/main.rs:10:36)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;read_fonts::tables::glyf::outline::Scale26Dot6 as read_fonts::tables::
    glyf::outline::Scale&gt;::component_offset(&amp;Scale26Dot6::new(None, 0u16),
    GlyphDelta { position: 1u16, x_delta: 65536i32,
    y_delta: 0i32 }.apply_scalar::&lt;i32&gt;(Fixed::MAX), &amp;Transform::default(),
    false, Some(GlyphDelta { position: 1u16, x_delta: 1i32,
    y_delta: 0i32 }.apply_scalar::&lt;font_types::Fixed&gt;(Fixed::NEG_ONE)), false);</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/tables/glyf/outline/scale.rs:394:13:
attempt to add with overflow
backtrace:
  &lt;read_fonts::tables::glyf::outline::scale::Scale26Dot6 as read_fonts::tables::glyf::outline::scale::Scale&gt;::component_offset (read-fonts/src/tables/glyf/outline/scale.rs:394:13)
  repro_read_fonts_src_tables_glyf_outline_scale_rs_394_13_fd5769::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![92u8, 17u8, 227u8, 177u8];
let font_data = FontData::new(&amp;vec);
let tuple_variation_header = TupleVariationHeader::read(font_data,
    1u16).unwrap();
let _ = tuple_variation_header.intermediate_tuples();</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/tables/variations.rs:203:72:
called `Result::unwrap()` on an `Err` value: OutOfBounds
backtrace:
  &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::intermediate_tuples::{closure#0} (read-fonts/src/tables/variations.rs:203:72)
  &lt;bool&gt;::then::&lt;(read_fonts::tables::variations::Tuple, read_fonts::tables::variations::Tuple), &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::intermediate_tuples::{closure#0}&gt; (core/src/bool.rs:66:24)
  &lt;read_fonts::tables::variations::TupleVariationHeader&gt;::intermediate_tuples (read-fonts/src/tables/variations.rs:198:50)
  repro_read_fonts_src_tables_variations_rs_203_72_a10999::main (src/main.rs:8:36)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;font_types::Point&lt;u8&gt; as ::core::ops::Div&lt;font_types::Point&lt;u8&gt;&gt;&gt;::div(
    &lt;font_types::Point&lt;u8&gt; as ::core::default::Default&gt;::default(),
    &lt;font_types::Point&lt;u8&gt; as ::core::default::Default&gt;::default());</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:173:16:
attempt to divide by zero
backtrace:
  &lt;u8 as core::ops::arith::Div&gt;::div (core/src/ops/arith.rs:493:45)
  &lt;font_types::point::Point&lt;u8&gt; as core::ops::arith::Div&gt;::div (font-types/src/point.rs:173:16)
  repro_font_types_src_point_rs_173_16_3ee800::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;font_types::Point&lt;u32&gt; as ::core::ops::Mul&lt;font_types::Point&lt;u32&gt;&gt;&gt;::
    mul(&lt;font_types::Point&lt;u32&gt;&gt;::new(1000000000u32, 0u32),
    &lt;font_types::Point&lt;u32&gt;&gt;::broadcast(10000u32));</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:121:16:
attempt to multiply with overflow
backtrace:
  &lt;u32 as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  &lt;font_types::point::Point&lt;u32&gt; as core::ops::arith::Mul&gt;::mul (font-types/src/point.rs:121:16)
  repro_font_types_src_point_rs_121_16_d68054::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;font_types::Point&lt;u8&gt; as ::core::ops::Sub&lt;font_types::Point&lt;u8&gt;&gt;&gt;::sub(
    &lt;font_types::Point&lt;u8&gt; as ::core::default::Default&gt;::default(),
    &lt;font_types::Point&lt;u8&gt;&gt;::broadcast(1u8));</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:95:16:
attempt to subtract with overflow
backtrace:
  &lt;u8 as core::ops::arith::Sub&gt;::sub (core/src/ops/arith.rs:216:45)
  &lt;font_types::point::Point&lt;u8&gt; as core::ops::arith::Sub&gt;::sub (font-types/src/point.rs:95:16)
  repro_font_types_src_point_rs_95_16_d7f137::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let stack = Stack::default();
let _ = stack.fixed_array::&lt;4&gt;(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at read-fonts/src/ps/cff/stack.rs:207:19:
attempt to add with overflow
backtrace:
  &lt;read_fonts::ps::cff::stack::Stack&gt;::fixed_array::&lt;4&gt; (read-fonts/src/ps/cff/stack.rs:207:19)
  repro_read_fonts_src_ps_cff_stack_rs_207_19_b08bcf::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;font_types::Point&lt;u32&gt; as ::core::ops::Sub&lt;font_types::Point&lt;u32&gt;&gt;&gt;::
    sub(&lt;font_types::Point&lt;u32&gt;&gt;::broadcast(0u32),
    &lt;font_types::Point&lt;u32&gt;&gt;::new(0u32, 1u32));</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:96:16:
attempt to subtract with overflow
backtrace:
  &lt;u32 as core::ops::arith::Sub&gt;::sub (core/src/ops/arith.rs:216:45)
  &lt;font_types::point::Point&lt;u32&gt; as core::ops::arith::Sub&gt;::sub (font-types/src/point.rs:96:16)
  repro_font_types_src_point_rs_96_16_0edcf4::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;font_types::Point&lt;u32&gt; as ::core::ops::Add&lt;font_types::Point&lt;u32&gt;&gt;&gt;::
    add(&lt;font_types::Point&lt;u32&gt;&gt;::new(1u32, 0u32),
    &lt;font_types::Point&lt;u32&gt;&gt;::broadcast(u32::MAX));</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:69:16:
attempt to add with overflow
backtrace:
  &lt;u32 as core::ops::arith::Add&gt;::add (core/src/ops/arith.rs:104:45)
  &lt;font_types::point::Point&lt;u32&gt; as core::ops::arith::Add&gt;::add (font-types/src/point.rs:69:16)
  repro_font_types_src_point_rs_69_16_192f39::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;font_types::Point&lt;u8&gt; as ::core::ops::Add&lt;font_types::Point&lt;u8&gt;&gt;&gt;::add(
    &lt;font_types::Point&lt;u8&gt;&gt;::new(1u8, 1u8), &lt;font_types::Point&lt;u8&gt;&gt;::new(0u8,
    255u8));</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:70:16:
attempt to add with overflow
backtrace:
  &lt;u8 as core::ops::arith::Add&gt;::add (core/src/ops/arith.rs:104:45)
  &lt;font_types::point::Point&lt;u8&gt; as core::ops::arith::Add&gt;::add (font-types/src/point.rs:70:16)
  repro_font_types_src_point_rs_70_16_ed1422::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;font_types::Point&lt;u32&gt; as ::core::ops::Mul&lt;font_types::Point&lt;u32&gt;&gt;&gt;::
    mul(&lt;font_types::Point&lt;u32&gt;&gt;::new(0u32, 2u32),
    &lt;font_types::Point&lt;u32&gt;&gt;::new(0u32, u32::MAX));</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:122:16:
attempt to multiply with overflow
backtrace:
  &lt;u32 as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  &lt;font_types::point::Point&lt;u32&gt; as core::ops::arith::Mul&gt;::mul (font-types/src/point.rs:122:16)
  repro_font_types_src_point_rs_122_16_b76267::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;font_types::Point&lt;u32&gt; as ::core::ops::Div&lt;font_types::Point&lt;u32&gt;&gt;&gt;::
    div(&lt;font_types::Point&lt;u32&gt;&gt;::broadcast(1u32),
    &lt;font_types::Point&lt;u32&gt;&gt;::new(1u32, 0u32));</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:174:16:
attempt to divide by zero
backtrace:
  &lt;u32 as core::ops::arith::Div&gt;::div (core/src/ops/arith.rs:493:45)
  &lt;font_types::point::Point&lt;u32&gt; as core::ops::arith::Div&gt;::div (font-types/src/point.rs:174:16)
  repro_font_types_src_point_rs_174_16_0e08a3::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;font_types::Point&lt;u32&gt; as ::core::ops::Div&lt;u32&gt;&gt;::div(
    &lt;font_types::Point&lt;u32&gt;&gt;::broadcast(0u32), 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:188:16:
attempt to divide by zero
backtrace:
  &lt;u32 as core::ops::arith::Div&gt;::div (core/src/ops/arith.rs:493:45)
  &lt;font_types::point::Point&lt;u32&gt; as core::ops::arith::Div&lt;u32&gt;&gt;::div (font-types/src/point.rs:188:16)
  repro_font_types_src_point_rs_188_16_b6546d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;read_fonts::tables::glyf::outline::Unscaled as read_fonts::tables::
    glyf::outline::Scale&gt;::component_offset(&amp;Unscaled,
    &lt;read_fonts::tables::gvar::GlyphDelta as read_fonts::tables::variations::
    TupleDelta&gt;::new(0u16, 0i32, i32::MAX).apply_scalar::&lt;i32&gt;(Fixed::ONE),
    &amp;Transform::default(), false, Some(GlyphDelta { position: 1u16,
    x_delta: 0i32,
    y_delta: 1i32 }.apply_scalar::&lt;font_types::Fixed&gt;(
    &lt;font_types::Fixed as ::core::ops::Neg&gt;::neg(Fixed::NEG_ONE))), false);</pre></td><td valign="top"><pre>thread 'main' panicked at font-types/src/point.rs:82:9:
attempt to add with overflow
backtrace:
  &lt;i32 as core::ops::arith::AddAssign&gt;::add_assign (core/src/ops/arith.rs:790:51)
  &lt;font_types::point::Point&lt;i32&gt; as core::ops::arith::AddAssign&gt;::add_assign (font-types/src/point.rs:82:9)
  &lt;read_fonts::tables::glyf::outline::scale::Unscaled as read_fonts::tables::glyf::outline::scale::Scale&gt;::component_offset (read-fonts/src/tables/glyf/outline/scale.rs:703:13)
  repro_font_types_src_point_rs_82_9_26e6e3::main (src/main.rs:7:13)</pre></td></tr>
</table>
