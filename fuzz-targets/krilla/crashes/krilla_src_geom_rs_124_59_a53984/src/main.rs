use krilla::document::Document;
use krilla::geom::Rect;
use krilla::geom::Size;
use krilla::image::Image;
use krilla::page::PageSettings;

fn main() {
    let mut document = Document::default();
    let mut page = document.start_page_with(PageSettings::from_wh(1.0f32, 1.0f32).unwrap());
    let mut surface = page.surface();
    let mut stream_builder = surface.stream_builder();
    let mut surface2 = stream_builder.surface_with_bbox(Rect::from_xywh(0.0f32, f32::MAX, 0.0f32, 0.0f32).unwrap());
    surface2.draw_image(Image::from_rgba8(vec![], 0u32, 0u32), Size::from_wh(1.0f32, 1.0f32).unwrap());
}
