use zune_core::colorspace::ColorSpace;
use zune_image::frame::Frame;

fn main() {
    let frame = Frame::new(vec![]);
    let _ = frame.channels_ref(ColorSpace::RGB, false);
}
