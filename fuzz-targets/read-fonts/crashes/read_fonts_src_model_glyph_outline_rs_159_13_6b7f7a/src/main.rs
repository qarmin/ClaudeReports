use read_fonts::model::glyph::outline::SvgPen;

fn main() {
    let mut svg_pen = SvgPen::from(SvgPen::with_precision(1000000000000000000usize));
    <read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::outline::OutlinePen>::curve_to(&mut svg_pen, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32);
}
