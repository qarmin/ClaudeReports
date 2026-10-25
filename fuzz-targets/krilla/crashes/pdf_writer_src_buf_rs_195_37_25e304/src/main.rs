use krilla::Data;
use krilla::SerializeSettings;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::page::PageSettings;
use krilla::text::Font;
use krilla::text::TextDirection;

fn main() {
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page_with(PageSettings::default());
    let mut surface = page.surface();
    surface.draw_text(Point::from_xy(0.0f32, 0.0f32), Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(), -2147483600.0f32, "a", false, TextDirection::Auto);
}
