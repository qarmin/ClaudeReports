use krilla::Data;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::text::Font;
use krilla::text::GlyphId;
use krilla::text::KrillaGlyph;

fn main() {
    let mut document = Document::new();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.push_isolated();
    surface.draw_glyphs(Point { x: 0.0f32, y: 0.0f32 }, &[KrillaGlyph { glyph_id: GlyphId::new(0u32), text_range: 0usize..0usize, x_advance: 0.0f32, x_offset: 0.0f32, y_offset: 0.0f32, y_advance: f32::MIN, location: None }], Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(), "مرحبا", 25.0f32, false);
}
