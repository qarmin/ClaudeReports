use zune_core::colorspace::ColorSpace;
use zune_image::frame::Frame;

fn main() {
    let mut frame = Frame::new(vec![]);
    let _ = frame.channels_mut(ColorSpace::RGB, false);
}
