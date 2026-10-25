use vello_common::paint::ImageSource;
use vello_common::pixmap::Pixmap;

fn main() {
    let _ = ImageSource::Pixmap(std::sync::Arc::new(Pixmap::new(u16::MAX, 10000u16)));
}
