use krilla::Data;
use krilla::SerializeSettings;
use krilla::document::Document;
use krilla::geom::Size;
use krilla::image::Image;

fn main() {
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.draw_image(Image::from_png(Data::from(include_bytes!("data_0.bin").to_vec()), false).unwrap(), Size::from_wh(f32::MAX, 1.0f32).unwrap());
}
