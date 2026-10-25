use krilla::Data;
use krilla::color::cmyk::Color;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::num::NormalizedF32;
use krilla::paint::LineCap;
use krilla::paint::LineJoin;
use krilla::paint::Paint;
use krilla::paint::Stroke;
use krilla::text::Font;
use krilla::text::GlyphId;
use krilla::text::KrillaGlyph;

fn main() {
    let mut document = Document::default();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.set_stroke(Some(Stroke { paint: Paint::from(Color::new(0u8, 0u8, 0u8, 0u8)), width: -68.0f32, miter_limit: 0.0f32, line_cap: LineCap::default(), line_join: LineJoin::default(), opacity: NormalizedF32::ZERO, dash: None }));
    surface.draw_glyphs(Point::default(), &[KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0usize..0usize, None)], Font::new_variable(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32, &[]).unwrap(), "", 0.0f32, false);
}
