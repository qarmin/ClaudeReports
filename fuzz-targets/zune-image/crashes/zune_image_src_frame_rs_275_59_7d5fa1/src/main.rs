use zune_core::colorspace::ColorSpace;
use zune_image::frame::Frame;

fn main() {
    let frame = Frame::from_u16(&[0u16, 0u16], ColorSpace::LumaA, 0usize, 0usize);
    let _ = frame.channels_ref(ColorSpace::BGRA, true);
}
