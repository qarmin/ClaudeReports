use vello_common::paint::ImageSource;
use vello_common::pixmap::Pixmap;
use vello_cpu::Resources;

fn main() {
    let _ = ImageSource::opaque_id(Resources::default().register_image(std::sync::Arc::new(Pixmap::new(u16::MAX, 10000u16))));
}
