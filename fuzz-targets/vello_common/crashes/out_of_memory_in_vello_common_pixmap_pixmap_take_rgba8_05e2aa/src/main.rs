use peniko::ImageAlphaType;
use vello_common::pixmap::Pixmap;

fn main() {
    let pixmap = Pixmap::new(5570u16, u16::MAX);
    let _ = pixmap.clone().take_rgba8(ImageAlphaType::AlphaPremultiplied);
}
