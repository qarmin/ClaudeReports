use krilla::Data;
use krilla::SerializeSettings;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::text::Font;
use krilla::text::GlyphId;
use krilla::text::KrillaGlyph;

fn main() {
    let f32_v = <krilla::text::KrillaGlyph as krilla::text::Glyph>::y_offset(&KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0usize..1usize, None), 0.0f32);
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.draw_glyphs(Point::from_xy(0.0f32, 0.0f32), &[KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, 0.0f32, f32_v, 0usize..1usize, None)], Font::new(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), 0u32).unwrap(), "", f32_v, false);
}
