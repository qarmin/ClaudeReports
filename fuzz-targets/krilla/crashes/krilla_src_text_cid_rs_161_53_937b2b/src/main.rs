use krilla::Data;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::page::PageSettings;
use krilla::text::Font;
use krilla::text::GlyphId;
use krilla::text::KrillaGlyph;

fn main() {
    let mut document = Document::new();
    let mut page = document.start_page_with(PageSettings::from_wh(1.0f32, 1.0f32).unwrap());
    let mut surface = page.surface();
    surface.draw_glyphs(Point::default(), &[KrillaGlyph { glyph_id: GlyphId::new(1000000000u32), text_range: 0usize..0usize, x_advance: 1.0f32, x_offset: 0.0f32, y_offset: 0.0f32, y_advance: 0.0f32, location: None }], Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(), "", 0.0f32, false);
}
