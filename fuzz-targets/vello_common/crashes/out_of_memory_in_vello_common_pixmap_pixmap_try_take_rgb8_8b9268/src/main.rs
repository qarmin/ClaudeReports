use peniko::ImageAlphaType;
use vello_common::pixmap::Pixmap;

fn main() {
    let pixmap = Pixmap::new(u16::MAX, 7357u16);
    let _ = pixmap.clone().try_take_rgb8(ImageAlphaType::AlphaPremultiplied);
}
