use read_fonts::model::glyph::outline::SvgPen;

fn main() {
    let mut svg_pen = SvgPen::with_precision(10000000000000000000usize);
    <read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::outline::OutlinePen>::line_to(&mut svg_pen, 38.4f32, f32::NEG_INFINITY);
}
