use krilla::Data;
use krilla::SerializeSettings;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::text::Font;
use krilla::text::GlyphId;
use krilla::text::KrillaGlyph;

fn main() {
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page();
    let f32_v = <krilla::text::KrillaGlyph as krilla::text::Glyph>::y_advance(&KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0usize..0usize, None), 0.0f32);
    let mut surface = page.surface();
    surface.draw_glyphs(Point::default(), &[KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, f32_v, f32::NAN, 0usize..0usize, None), KrillaGlyph { glyph_id: GlyphId::new(10u32), text_range: 0usize..0usize, x_advance: 0.0f32, x_offset: 0.0f32, y_offset: 0.0f32, y_advance: 0.0f32, location: None }], Font::new_variable(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), 0u32, &[]).unwrap(), "", 0.0f32, true);
}
