use krilla::document::Document;
use krilla::geom::Size;
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;

fn main() {
    let mut document = Document::default();
    let mut page = document.start_page_with(PageSettings::new(Size::from_wh(1.0f32, 1.0f32).unwrap()));
    let mut surface = page.surface();
    surface.push_isolated();
    surface.pop();
    surface.push_opacity(NormalizedF32::ONE);
}
