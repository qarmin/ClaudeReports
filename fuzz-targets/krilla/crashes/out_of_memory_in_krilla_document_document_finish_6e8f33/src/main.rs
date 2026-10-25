use krilla::Data;
use krilla::document::Document;
use krilla::image::Image;

fn main() {
    let _ = Image::from_webp(Data::from(include_bytes!("data_0.bin").to_vec()), false);
    let document = Document::default();
    let _ = document.finish();
}
