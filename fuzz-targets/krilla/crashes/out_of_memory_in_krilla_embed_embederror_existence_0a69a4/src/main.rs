use krilla::Data;
use krilla::document::Document;
use krilla::embed::EmbedError;
use krilla::image::Image;

fn main() {
    let document = Document::default();
    let _ = document.finish();
    let _ = Image::from_webp(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), false);
    let _ = EmbedError::Existence;
}
