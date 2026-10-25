use read_fonts::model::glyph::outline::SvgPen;

fn main() {
    let mut svg_pen = SvgPen::from(SvgPen::with_precision(usize::MAX));
    <read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::outline::OutlinePen>::quad_to(&mut svg_pen, 16777216.0f32, 4.8731973e-34f32, -0.0f32, 1e30f32);
}
